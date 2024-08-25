use std::{
    fmt,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use flume::{bounded, Receiver, SendError, Sender};
use regex::Regex;

use crate::{LoggingError, NOTSET};

pub trait CallbackFnT: Fn(u8, String, String) -> Result<(), LoggingError> {}

impl<F> CallbackFnT for F where F: Fn(u8, String, String) -> Result<(), LoggingError> {}

impl std::fmt::Debug for dyn CallbackFnT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CallbackFnT")
    }
}

pub type CallbackFn = Box<dyn CallbackFnT + Send + Sync>;

#[derive(Debug)]
pub enum CallbackTypeEnum {
    Message((u8, String, String)), // level, domain, message
    Sync,                          // timeout
    Stop,
}

#[repr(C)]
#[derive(Clone, Serialize, Deserialize)]
pub struct CallbackWriterConfig {
    pub(crate) enabled: bool,
    pub(crate) level: u8,
    pub(crate) domain_filter: Option<String>,
    pub(crate) message_filter: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub(crate) callback: Option<Arc<Mutex<CallbackFn>>>,
}

impl std::fmt::Debug for CallbackWriterConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "level={}", self.level)
    }
}

impl CallbackWriterConfig {
    pub fn new(level: u8, callback: Option<CallbackFn>) -> Self {
        Self {
            enabled: true,
            level,
            domain_filter: None,
            message_filter: None,
            callback: callback.map(|f| Arc::new(Mutex::new(f))),
        }
    }
}

impl Default for CallbackWriterConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: NOTSET,
            domain_filter: None,
            message_filter: None,
            callback: None,
        }
    }
}

impl fmt::Display for CallbackWriterConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

fn callback_writer_thread(
    config: Arc<Mutex<CallbackWriterConfig>>,
    rx: Receiver<CallbackTypeEnum>,
    sync_tx: Sender<u8>,
    stop: Arc<AtomicBool>,
) -> Result<(), LoggingError> {
    loop {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        match rx.recv()? {
            CallbackTypeEnum::Message((level, domain, message)) => {
                if let Ok(ref config) = config.lock() {
                    if let Some(ref callback) = config.callback {
                        if let Err(err) = (callback.lock().unwrap())(level, domain, message) {
                            eprintln!("CallbackWriter: Error: {err:?}");
                        }
                    }
                } else {
                    break;
                }
            }
            CallbackTypeEnum::Sync => {
                sync_tx.send(1)?;
            }
            CallbackTypeEnum::Stop => {
                break;
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct CallbackWriter {
    pub(crate) config: Arc<Mutex<CallbackWriterConfig>>,
    tx: Sender<CallbackTypeEnum>,
    sync_rx: Receiver<u8>,
    thr: Option<JoinHandle<()>>,
}

impl CallbackWriter {
    pub fn new(config: CallbackWriterConfig, stop: Arc<AtomicBool>) -> Result<Self, LoggingError> {
        let config = Arc::new(Mutex::new(config));
        let (tx, rx) = bounded(1000);
        let (sync_tx, sync_rx) = bounded(1);
        Ok(Self {
            config: config.clone(),
            tx,
            sync_rx,
            thr: Some(
                thread::Builder::new()
                    .name("ConsoleWriter".to_string())
                    .spawn(move || {
                        if let Err(err) = callback_writer_thread(config.clone(), rx, sync_tx, stop)
                        {
                            eprintln!("console_writer_thread failed: {err:?}");
                        }
                    })?,
            ),
        })
    }

    pub fn shutdown(&mut self) -> Result<(), LoggingError> {
        if let Some(thr) = self.thr.take() {
            self.tx.send(CallbackTypeEnum::Stop).map_err(|e| {
                LoggingError::SendCmdError(
                    "ConsoleWriter".to_string(),
                    "STOP".to_string(),
                    e.to_string(),
                )
            })?;
            thr.join().map_err(|e| {
                LoggingError::JoinError(
                    "ConsoleWriter".to_string(),
                    e.downcast_ref::<&str>().unwrap().to_string(),
                )
            })
        } else {
            Ok(())
        }
    }

    pub fn sync(&self, timeout: f64) -> Result<(), LoggingError> {
        self.tx.send(CallbackTypeEnum::Sync).map_err(|e| {
            LoggingError::SendCmdError(
                "ConsoleWriter".to_string(),
                "SYNC".to_string(),
                e.to_string(),
            )
        })?;
        self.sync_rx
            .recv_timeout(Duration::from_secs_f64(timeout))
            .map_err(|e| {
                LoggingError::RecvAswError(
                    "ConsoleWriter".to_string(),
                    "SYNC".to_string(),
                    e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn enable(&self) {
        self.config.lock().unwrap().enabled = true;
    }

    pub fn disable(&self) {
        self.config.lock().unwrap().enabled = false;
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.config.lock().unwrap().enabled = enabled;
    }

    pub fn set_level(&self, level: u8) {
        self.config.lock().unwrap().level = level;
    }

    pub fn set_domain_filter(&self, domain_filter: Option<String>) -> Result<(), regex::Error> {
        if let Some(ref message) = domain_filter {
            Regex::new(message)?;
        }
        self.config.lock().unwrap().domain_filter = domain_filter;
        Ok(())
    }

    pub fn set_message_filter(&self, message_filter: Option<String>) -> Result<(), regex::Error> {
        if let Some(ref message) = message_filter {
            Regex::new(message)?;
        }
        self.config.lock().unwrap().message_filter = message_filter;
        Ok(())
    }

    pub fn set_callback(&self, callback: Option<CallbackFn>) {
        self.config.lock().unwrap().callback = callback.map(|f| Arc::new(Mutex::new(f)));
    }

    #[inline]
    pub fn send(
        &self,
        level: u8,
        domain: String,
        message: String,
    ) -> Result<(), SendError<CallbackTypeEnum>> {
        self.tx
            .send(CallbackTypeEnum::Message((level, domain, message)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Logging, LoggingError, WriterConfigEnum, DEBUG};

    use super::CallbackWriterConfig;

    fn writer_callback(level: u8, domain: String, message: String) -> Result<(), LoggingError> {
        println!("CB: {level} {domain} {message}");
        Ok(())
    }

    #[test]
    fn callback() {
        let callback_writer = CallbackWriterConfig::new(DEBUG, Some(Box::new(writer_callback)));
        let mut logging =
            Logging::new(None, None, None, None, None, None, None, None, None).unwrap();
        logging
            .add_writer(&WriterConfigEnum::Callback(callback_writer))
            .unwrap();
        logging.trace("Trace Message".to_string()).unwrap();
        logging.debug("Debug Message".to_string()).unwrap();
        logging.info("Info Message".to_string()).unwrap();
        logging.success("Success Message".to_string()).unwrap();
        logging.warning("Warning Message".to_string()).unwrap();
        logging.error("Error Message".to_string()).unwrap();
        logging.fatal("Fatal Message".to_string()).unwrap();
        logging.shutdown(false).unwrap();
    }
}
