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

use crate::LoggingError;

fn level2evt_level(level: u8) -> log::Level {
    match level {
        70 | 60 | 50 | 40 => log::Level::Error,
        30 => log::Level::Warn,
        25 => log::Level::Info,
        20 => log::Level::Info,
        10 => log::Level::Debug,
        _ => log::Level::Trace,
    }
}

#[derive(Debug)]
pub enum SyslogTypeEnum {
    Message((u8, String)), // level, message
    Sync(f64),             // timeout
    Stop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyslogWriterConfig {
    pub level: u8, // Log level
    formatter: String,
}

impl SyslogWriterConfig {
    pub fn new<S: Into<String>>(level: u8, hostname: Option<String>, pname: S, pid: u32) -> Self {
        Self {
            level,
            formatter: format!(
                "{}: {}[{pid}]",
                hostname.map(|v| format!("{v}: ")).unwrap_or_default(),
                pname.into()
            ),
        }
    }
}

impl fmt::Display for SyslogWriterConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn syslog_writer_thread(
    config: Arc<Mutex<SyslogWriterConfig>>,
    rx: Receiver<SyslogTypeEnum>,
    sync_tx: Sender<u8>,
    stop: Arc<AtomicBool>,
) -> Result<(), LoggingError> {
    eventlog::init("fastlogging", level2evt_level(config.lock().unwrap().level))
        .map_err(|e| LoggingError::InvalidValue(e.to_string()))?;
    loop {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        match rx.recv()? {
            SyslogTypeEnum::Message((level, message)) => {
                log::log!(level2evt_level(level), "{}", message);
            }
            SyslogTypeEnum::Sync(_) => {
                sync_tx.send(1)?;
            }
            SyslogTypeEnum::Stop => {
                break;
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct SyslogWriter {
    pub(crate) config: Arc<Mutex<SyslogWriterConfig>>,
    tx: Sender<SyslogTypeEnum>,
    sync_rx: Receiver<u8>,
    thr: Option<JoinHandle<()>>,
}

impl SyslogWriter {
    pub fn new(config: SyslogWriterConfig, stop: Arc<AtomicBool>) -> Result<Self, LoggingError> {
        let config = Arc::new(Mutex::new(config));
        let (tx, rx) = bounded(1000);
        let (sync_tx, sync_rx) = bounded(1);
        Ok(Self {
            config: config.clone(),
            tx,
            sync_rx,
            thr: Some(
                thread::Builder::new()
                    .name("SyslogWriter".to_string())
                    .spawn(move || {
                        if let Err(err) = syslog_writer_thread(config, rx, sync_tx, stop) {
                            eprintln!("syslog_writer_thread failed: {err:?}");
                        }
                    })?,
            ),
        })
    }

    pub fn shutdown(&mut self) -> Result<(), LoggingError> {
        if let Some(thr) = self.thr.take() {
            self.tx.send(SyslogTypeEnum::Stop).map_err(|e| {
                LoggingError::SendCmdError(
                    "SyslogWriter".to_string(),
                    "STOP".to_string(),
                    e.to_string(),
                )
            })?;
            thr.join().map_err(|e| {
                LoggingError::JoinError(
                    "SyslogWriter".to_string(),
                    e.downcast_ref::<&str>().unwrap().to_string(),
                )
            })
        } else {
            Ok(())
        }
    }

    pub fn sync(&self, timeout: f64) -> Result<(), LoggingError> {
        self.tx.send(SyslogTypeEnum::Sync(timeout)).map_err(|e| {
            LoggingError::SendCmdError(
                "SyslogWriter".to_string(),
                "SYNC".to_string(),
                e.to_string(),
            )
        })?;
        self.sync_rx
            .recv_timeout(Duration::from_secs_f64(timeout))
            .map_err(|e| {
                LoggingError::RecvAswError(
                    "SyslogWriter".to_string(),
                    "SYNC".to_string(),
                    e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn set_level(&self, level: u8) {
        self.config.lock().unwrap().level = level;
    }

    #[inline]
    pub fn send(&self, level: u8, message: String) -> Result<(), SendError<SyslogTypeEnum>> {
        self.tx.send(SyslogTypeEnum::Message((level, message)))
    }
}
