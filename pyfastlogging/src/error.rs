#![allow(non_snake_case)]

use pyo3::exceptions;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct LoggingError(pub fastlogging::LoggingError);

impl From<LoggingError> for PyErr {
    fn from(error: LoggingError) -> Self {
        match error.0 {
            fastlogging::LoggingError::Io { kind, message } => match kind.as_str() {
                "WouldBlock" => PyErr::new::<exceptions::PyBlockingIOError, _>(message),
                "NotFound" => PyErr::new::<exceptions::PyFileNotFoundError, _>(message),
                "InvalidData" => PyErr::new::<exceptions::PyValueError, _>(message),
                "UnexpectedEof" => PyErr::new::<exceptions::PyEOFError, _>(message),
                _ => PyErr::new::<exceptions::PyException, _>(message),
            },
            fastlogging::LoggingError::Utf8Error(e) => {
                PyErr::new::<exceptions::PyUnicodeError, _>(e)
            }
            fastlogging::LoggingError::SyslogError(e) => {
                PyErr::new::<exceptions::PyValueError, _>(e)
            }
            fastlogging::LoggingError::RecvError(e) => PyErr::new::<exceptions::PyValueError, _>(e),
            fastlogging::LoggingError::SendError(e) => PyErr::new::<exceptions::PyValueError, _>(e),
            fastlogging::LoggingError::SendCmdError(m, c, e) => {
                PyErr::new::<exceptions::PyValueError, _>(format!(
                    "{m}: Failed to send {c} command: {e}"
                ))
            }
            fastlogging::LoggingError::RecvAswError(m, c, e) => {
                PyErr::new::<exceptions::PyValueError, _>(format!(
                    "{m}: Failed to receive {c} answer: {e}"
                ))
            }
            fastlogging::LoggingError::InvalidValue(e) => {
                PyErr::new::<exceptions::PyValueError, _>(e)
            }
            fastlogging::LoggingError::InvalidEncryption(m, k, e) => {
                PyErr::new::<exceptions::PyValueError, _>(format!(
                    "{m}: Invalid encryption {k:?}: {e}"
                ))
            }
            fastlogging::LoggingError::JoinError(m, e) => {
                PyErr::new::<exceptions::PyRuntimeError, _>(format!("{m}: {e}"))
            }
            fastlogging::LoggingError::ConfigError(e) => {
                PyErr::new::<exceptions::PyValueError, _>(e)
            }
            fastlogging::LoggingError::ArchiveError(e) => {
                PyErr::new::<exceptions::PyValueError, _>(e)
            }
        }
    }
}

impl From<fastlogging::LoggingError> for LoggingError {
    fn from(error: fastlogging::LoggingError) -> Self {
        LoggingError(error)
    }
}
