use std::{ io::{ Error, ErrorKind }, thread };

use flume::Sender;

use crate::{
    def::{ LoggingTypeEnum, CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, WARNING },
    SUCCESS,
    TRACE,
};

#[derive(Debug)]
pub struct Logger {
    pub(crate) level: u8,
    pub(crate) domain: String,
    pub(crate) tname: bool,
    pub(crate) tid: bool,
    tx: Option<Sender<LoggingTypeEnum>>,
}

impl Logger {
    pub fn new(level: u8, domain: String) -> Self {
        Self { level, domain, tname: false, tid: false, tx: None }
    }

    pub fn new_ext(level: u8, domain: String, tname: bool, tid: bool) -> Self {
        Self { level, domain, tname, tid, tx: None }
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
    fn log<S: Into<String>>(&self, level: u8, message: S) -> Result<(), Error> {
        if let Some(ref tx) = self.tx {
            let message = format!("{}: {}", self.domain, message.into());
            return (
                if self.tname || self.tid {
                    let tname = if self.tname {
                        thread::current().name().unwrap_or_default().to_string()
                    } else {
                        "".to_string()
                    };
                    let tid = if self.tid { thread_id::get() as u32 } else { 0 };
                    tx.send(LoggingTypeEnum::MessageExt((level, message, tid, tname)))
                } else {
                    tx.send(LoggingTypeEnum::Message((level, message)))
                }
            ).map_err(|e| Error::new(ErrorKind::Other, e.to_string()));
        }
        Err(
            Error::new(
                ErrorKind::NotConnected,
                "Logger not registered at Logging instance. Call add_logger first."
            )
        )
    }

    pub fn trace<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= TRACE { self.log(TRACE, message) } else { Ok(()) }
    }

    pub fn debug<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= DEBUG { self.log(DEBUG, message) } else { Ok(()) }
    }

    pub fn info<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= INFO { self.log(INFO, message) } else { Ok(()) }
    }

    pub fn success<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= SUCCESS { self.log(SUCCESS, message) } else { Ok(()) }
    }

    pub fn warning<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= WARNING { self.log(WARNING, message) } else { Ok(()) }
    }

    pub fn error<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= ERROR { self.log(ERROR, message) } else { Ok(()) }
    }

    pub fn critical<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= CRITICAL { self.log(CRITICAL, message) } else { Ok(()) }
    }

    pub fn fatal<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= FATAL { self.log(FATAL, message) } else { Ok(()) }
    }

    pub fn exception<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= EXCEPTION { self.log(EXCEPTION, message) } else { Ok(()) }
    }

    pub fn __repr__(&self) -> String {
        format!("Logger(level={} domain={})", self.level, self.domain)
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}
