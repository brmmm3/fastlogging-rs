use std::io;

use serde::{Deserialize, Serialize};
use zip::result::ZipError;

use crate::{
    console::ConsoleTypeEnum, ClientTypeEnum, EncryptionMethod, LoggingTypeEnum, SyslogTypeEnum,
};

pub const EIO: i32 = 5;
pub const EINVAL: i32 = 22;
pub const EFAIL: i32 = 100;

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

    #[error("{0}: Failed to send {1} command: {2}")]
    SendCmdError(String, String, String),

    #[error("{0}: Failed to receive {1} answer: {2}")]
    RecvAswError(String, String, String),

    #[error("{0}")]
    InvalidValue(String),

    #[error("{0}: Invalid encryption {1:?}: {2}")]
    InvalidEncryption(String, EncryptionMethod, String),

    #[error("{0}: {1}")]
    JoinError(String, String),

    #[error("{0}")]
    ConfigError(String),

    #[error("{0}")]
    ArchiveError(String),
}

impl LoggingError {
    pub fn as_int(&self) -> i32 {
        match self {
            LoggingError::Io {
                kind: _,
                message: _,
            } => EIO,
            LoggingError::Utf8Error(_) => EINVAL,
            LoggingError::SyslogError(_) => EFAIL,
            LoggingError::RecvError(_) => EFAIL,
            LoggingError::SendError(_) => EFAIL,
            LoggingError::SendCmdError(_, _, _) => EFAIL,
            LoggingError::RecvAswError(_, _, _) => EFAIL,
            LoggingError::InvalidValue(_) => EINVAL,
            LoggingError::InvalidEncryption(_, _, _) => EINVAL,
            LoggingError::JoinError(_, _) => EFAIL,
            LoggingError::ConfigError(_) => EINVAL,
            LoggingError::ArchiveError(_) => EFAIL,
        }
    }
}

impl From<io::Error> for LoggingError {
    fn from(error: io::Error) -> Self {
        let kind = match error.kind() {
            io::ErrorKind::NotFound => "NotFound".to_string(),
            _ => error.kind().to_string(),
        };
        LoggingError::Io {
            kind,
            message: error.to_string(),
        }
    }
}

impl From<ZipError> for LoggingError {
    fn from(error: ZipError) -> Self {
        LoggingError::ArchiveError(error.to_string())
    }
}

#[cfg(target_family = "unix")]
impl From<syslog::Error> for LoggingError {
    fn from(error: syslog::Error) -> Self {
        LoggingError::SyslogError(error.to_string())
    }
}

#[cfg(target_family = "windows")]
impl From<eventlog::Error> for LoggingError {
    fn from(error: eventlog::Error) -> Self {
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
