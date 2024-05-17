use std::io::{ Error, ErrorKind };

use flume::Sender;

use crate::def::{ MessageType, CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, WARNING };

#[derive(Debug)]
pub struct Logger {
    pub level: u8,
    pub domain: String,
    tx: Option<Sender<MessageType>>,
}

impl Logger {
    pub fn new(level: u8, domain: String) -> Self {
        Self { level, domain, tx: None }
    }

    pub fn set_tx(&mut self, tx: Option<Sender<MessageType>>) {
        self.tx = tx;
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
    }

    pub fn set_domain(&mut self, domain: String) {
        self.domain = domain;
    }

    // Logging calls

    #[inline]
    fn log<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if let Some(ref tx) = self.tx {
            let message = format!("{}: {}", self.domain, message.into());
            tx
                .send(Some((DEBUG, message)))
                .map_err(|e| Error::new(ErrorKind::NotConnected, e.to_string()))?;
        }
        Err(
            Error::new(
                ErrorKind::NotConnected,
                "Logger not registered at Logging instance. Call add_logger first."
            )
        )
    }

    pub fn debug<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= DEBUG { self.log(message) } else { Ok(()) }
    }

    pub fn info<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= INFO { self.log(message) } else { Ok(()) }
    }

    pub fn warning<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= WARNING { self.log(message) } else { Ok(()) }
    }

    pub fn error<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= ERROR { self.log(message) } else { Ok(()) }
    }

    pub fn critical<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= CRITICAL { self.log(message) } else { Ok(()) }
    }

    pub fn fatal<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= FATAL { self.log(message) } else { Ok(()) }
    }

    pub fn exception<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= EXCEPTION { self.log(message) } else { Ok(()) }
    }

    pub fn __repr__(&self) -> String {
        format!("Logger(level={} domain={})", self.level, self.domain)
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}
