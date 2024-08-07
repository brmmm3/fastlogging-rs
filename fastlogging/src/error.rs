use serde::{Deserialize, Serialize};

use crate::{console::ConsoleTypeEnum, ClientTypeEnum, LoggingTypeEnum, SyslogTypeEnum};

#[derive(Debug, Clone, thiserror::Error, Deserialize, Serialize)]
pub enum LoggingError {
    #[error("I/O error ({kind}): {message}")]
    Io { kind: String, message: String },

    // Represents a failure to convert to UTF8 string.
    #[error("{0}")]
    Utf8Error(String),

    #[error("{0}")]
    SyslogError(String),

    #[error("{0}")]
    RecvError(String),

    #[error("{0}")]
    SendError(String),

    #[error("{0}")]
    InvalidValue(String),
}

impl From<std::io::Error> for LoggingError {
    fn from(error: std::io::Error) -> Self {
        LoggingError::Io {
            kind: error.kind().to_string(),
            message: error.to_string(),
        }
    }
}

impl From<syslog::Error> for LoggingError {
    fn from(error: syslog::Error) -> Self {
        LoggingError::SyslogError(error.to_string())
    }
}

impl From<flume::RecvError> for LoggingError {
    fn from(error: flume::RecvError) -> Self {
        LoggingError::RecvError(error.to_string())
    }
}

impl From<flume::SendError<u8>> for LoggingError {
    fn from(error: flume::SendError<u8>) -> Self {
        LoggingError::SendError(error.to_string())
    }
}

impl From<flume::SendError<LoggingTypeEnum>> for LoggingError {
    fn from(error: flume::SendError<LoggingTypeEnum>) -> Self {
        LoggingError::SendError(error.to_string())
    }
}

impl From<flume::SendError<ConsoleTypeEnum>> for LoggingError {
    fn from(error: flume::SendError<ConsoleTypeEnum>) -> Self {
        LoggingError::SendError(error.to_string())
    }
}

impl From<flume::SendError<SyslogTypeEnum>> for LoggingError {
    fn from(error: flume::SendError<SyslogTypeEnum>) -> Self {
        LoggingError::SendError(error.to_string())
    }
}

impl From<flume::SendError<ClientTypeEnum>> for LoggingError {
    fn from(error: flume::SendError<ClientTypeEnum>) -> Self {
        LoggingError::SendError(error.to_string())
    }
}
