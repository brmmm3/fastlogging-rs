use std::thread;

use flume::Sender;

use crate::{
    def::{LoggingTypeEnum, CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, WARNING},
    LoggingError, SUCCESS, TRACE,
};

#[repr(C)]
#[derive(Debug)]
pub struct Logger {
    pub(crate) level: u8,
    pub(crate) domain: String,
    pub(crate) tname: bool,
    pub(crate) tid: bool,
    tx: Option<Sender<LoggingTypeEnum>>,
}

impl Logger {
    pub fn new<S: Into<String>>(level: u8, domain: S) -> Self {
        Self {
            level,
            domain: domain.into(),
            tname: false,
            tid: false,
            tx: None,
        }
    }

    pub fn new_ext<S: Into<String>>(level: u8, domain: S, tname: bool, tid: bool) -> Self {
        Self {
            level,
            domain: domain.into(),
            tname,
            tid,
            tx: None,
        }
    }

    pub fn set_tx(&mut self, tx: Option<Sender<LoggingTypeEnum>>) {
        self.tx = tx;
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn set_domain(&mut self, domain: String) {
        self.domain = domain;
    }

    // Logging calls

    #[inline]
    fn log<S: Into<String>>(&self, level: u8, message: S) -> Result<(), LoggingError> {
        if let Some(ref tx) = self.tx {
            let message = format!("{}: {}", self.domain, message.into());
            return (if self.tname || self.tid {
                let tname = if self.tname {
                    thread::current().name().unwrap_or_default().to_string()
                } else {
                    "".to_string()
                };
                let tid = if self.tid { thread_id::get() as u32 } else { 0 };
                tx.send(LoggingTypeEnum::MessageExt((
                    level,
                    self.domain.clone(),
                    message,
                    tid,
                    tname,
                )))
            } else {
                tx.send(LoggingTypeEnum::Message((
                    level,
                    self.domain.clone(),
                    message,
                )))
            })
            .map_err(|e| LoggingError::SendError(format!("Failed to send message: {e}")));
        }
        Err(LoggingError::ConfigError(
            "Logger not registered at Logging instance. Call add_logger first.".to_string(),
        ))
    }

    pub fn trace<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= TRACE {
            self.log(TRACE, message)
        } else {
            Ok(())
        }
    }

    pub fn debug<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= DEBUG {
            self.log(DEBUG, message)
        } else {
            Ok(())
        }
    }

    pub fn info<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= INFO {
            self.log(INFO, message)
        } else {
            Ok(())
        }
    }

    pub fn success<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= SUCCESS {
            self.log(SUCCESS, message)
        } else {
            Ok(())
        }
    }

    pub fn warning<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= WARNING {
            self.log(WARNING, message)
        } else {
            Ok(())
        }
    }

    pub fn error<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= ERROR {
            self.log(ERROR, message)
        } else {
            Ok(())
        }
    }

    pub fn critical<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= CRITICAL {
            self.log(CRITICAL, message)
        } else {
            Ok(())
        }
    }

    pub fn fatal<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= FATAL {
            self.log(FATAL, message)
        } else {
            Ok(())
        }
    }

    pub fn exception<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= EXCEPTION {
            self.log(EXCEPTION, message)
        } else {
            Ok(())
        }
    }

    pub fn __repr__(&self) -> String {
        format!("Logger(level={} domain={})", self.level, self.domain)
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}
