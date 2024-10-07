use std::{
    fmt,
    io::Write,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use flume::{bounded, Receiver, SendError, Sender};
use regex::Regex;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use crate::{
    LoggingError, CRITICAL, DEBUG, ERROR, EXCEPTION, INFO, NOTSET, SUCCESS, TRACE, WARNING,
};

#[derive(Debug)]
pub enum ConsoleTypeEnum {
    Message((u8, String, String)), // level, domain, message
    Sync,                          // timeout
    Stop,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsoleTargetEnum {
    StdOut,
    StdErr,
    Both,
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleWriterConfig {
    pub(crate) enabled: bool,
    pub(crate) level: u8, // Log level
    pub(crate) domain_filter: Option<String>,
    pub(crate) message_filter: Option<String>,
    pub(crate) colors: bool,
    pub(crate) target: ConsoleTargetEnum,
    pub(crate) debug: u8,
}

impl ConsoleWriterConfig {
    pub fn new(level: u8, colors: bool) -> Self {
        Self {
            enabled: true,
            level,
            domain_filter: None,
            message_filter: None,
            colors,
            target: ConsoleTargetEnum::StdOut,
            debug: 0,
        }
    }
}

impl Default for ConsoleWriterConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: NOTSET,
            domain_filter: None,
            message_filter: None,
            colors: false,
            target: ConsoleTargetEnum::StdOut,
            debug: 0,
        }
    }
}

impl fmt::Display for ConsoleWriterConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

fn console_writer_thread(
    config: Arc<Mutex<ConsoleWriterConfig>>,
    rx: Receiver<ConsoleTypeEnum>,
    sync_tx: Sender<u8>,
    stop: Arc<AtomicBool>,
) -> Result<(), LoggingError> {
    let stdout_bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut stdout_buffer = stdout_bufwtr.buffer();
    let stderr_bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut stderr_buffer = stderr_bufwtr.buffer();
    loop {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        match rx.recv()? {
            ConsoleTypeEnum::Message((level, domain, message)) => {
                if let Ok(ref config) = config.lock() {
                    if !config.enabled {
                        continue;
                    }
                    if let Some(ref domain_filter) = config.domain_filter {
                        let re = Regex::new(domain_filter).unwrap();
                        if !re.is_match(&domain) {
                            continue;
                        }
                    }
                    if let Some(ref message_filter) = config.message_filter {
                        let re = Regex::new(message_filter).unwrap();
                        if !re.is_match(&domain) {
                            continue;
                        }
                    }
                    if config.colors {
                        let (bufwtr, buffer) = if config.target == ConsoleTargetEnum::StdOut {
                            (&stdout_bufwtr, &mut stdout_buffer)
                        } else if config.target == ConsoleTargetEnum::StdErr {
                            (&stderr_bufwtr, &mut stderr_buffer)
                        } else if level < ERROR {
                            (&stdout_bufwtr, &mut stdout_buffer)
                        } else {
                            (&stderr_bufwtr, &mut stderr_buffer)
                        };
                        buffer.clear();
                        buffer.set_color(ColorSpec::new().set_fg(Some(match level {
                            TRACE => Color::White,
                            DEBUG => Color::Blue,
                            INFO => Color::Green,
                            SUCCESS => Color::Cyan,
                            WARNING => Color::Yellow,
                            ERROR => Color::Magenta,
                            CRITICAL => Color::Red,
                            EXCEPTION => Color::Red,
                            _ => Color::White,
                        })))?;
                        writeln!(buffer, "{message}")?;
                        buffer.reset()?;
                        bufwtr.print(buffer)?;
                    } else if config.target == ConsoleTargetEnum::StdOut {
                        println!("{message}");
                    } else if config.target == ConsoleTargetEnum::StdErr {
                        eprintln!("{message}");
                    } else if level < ERROR {
                        println!("{message}");
                    } else {
                        eprintln!("{message}");
                    }
                } else {
                    break;
                }
            }
            ConsoleTypeEnum::Sync => {
                sync_tx.send(1)?;
            }
            ConsoleTypeEnum::Stop => {
                break;
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct ConsoleWriter {
    pub(crate) config: Arc<Mutex<ConsoleWriterConfig>>,
    tx: Sender<ConsoleTypeEnum>,
    sync_rx: Receiver<u8>,
    thr: Option<JoinHandle<()>>,
    pub(crate) debug: u8,
}

impl ConsoleWriter {
    pub fn new(config: ConsoleWriterConfig, stop: Arc<AtomicBool>) -> Result<Self, LoggingError> {
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
                        if let Err(err) = console_writer_thread(config.clone(), rx, sync_tx, stop) {
                            eprintln!("console_writer_thread failed: {err:?}");
                        }
                    })?,
            ),
            debug: 0,
        })
    }

    pub fn shutdown(&mut self) -> Result<(), LoggingError> {
        if let Some(thr) = self.thr.take() {
            self.tx.send(ConsoleTypeEnum::Stop).map_err(|e| {
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
        self.tx.send(ConsoleTypeEnum::Sync).map_err(|e| {
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

    pub fn set_colors(&self, colors: bool) {
        self.config.lock().unwrap().colors = colors;
    }

    pub fn set_target(&self, target: ConsoleTargetEnum) {
        self.config.lock().unwrap().target = target;
    }

    #[inline]
    pub fn send(
        &self,
        level: u8,
        domain: String,
        message: String,
    ) -> Result<(), SendError<ConsoleTypeEnum>> {
        self.tx
            .send(ConsoleTypeEnum::Message((level, domain, message)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ConsoleWriterConfig, Logging, DEBUG, NOTSET};

    #[test]
    fn console() {
        let mut logging = Logging::new(
            NOTSET,
            "root",
            vec![ConsoleWriterConfig::new(DEBUG, true).into()],
            None,
            None,
        )
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
