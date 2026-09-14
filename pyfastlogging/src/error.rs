#![allow(non_snake_case)]

use pyo3::PyTypeInfo;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

#[pyclass(extends = PyException)]
#[derive(Debug)]
pub struct LoggingError(pub fastlogging::LoggingError);

#[pymethods]
impl LoggingError {
    #[new]
    fn new(_message: String) -> Self {
        Self(fastlogging::LoggingError::InvalidValue(_message))
    }
}

impl From<LoggingError> for PyErr {
    fn from(error: LoggingError) -> Self {
        let message = match error.0 {
            fastlogging::LoggingError::Io { kind, message } => format!("{kind}: {message}"),
            fastlogging::LoggingError::Utf8Error(e) => e.to_string(),
            fastlogging::LoggingError::SyslogError(e)
            | fastlogging::LoggingError::RecvError(e)
            | fastlogging::LoggingError::SendError(e)
            | fastlogging::LoggingError::InvalidValue(e)
            | fastlogging::LoggingError::InvalidFile(e)
            | fastlogging::LoggingError::ConfigError(e)
            | fastlogging::LoggingError::ArchiveError(e) => e,
            fastlogging::LoggingError::SendCmdError(m, c, e) => {
                format!("{m}: Failed to send {c} command: {e}")
            }
            fastlogging::LoggingError::RecvAswError(m, c, e) => {
                format!("{m}: Failed to receive {c} answer: {e}")
            }
            fastlogging::LoggingError::InvalidEncryption(m, k, e) => {
                format!("{m}: Invalid encryption {k:?}: {e}")
            }
            fastlogging::LoggingError::JoinError(m, e) => format!("{m}: {e}"),
        };
        Python::attach(|py| PyErr::from_type(LoggingError::type_object(py), message))
    }
}

impl From<fastlogging::LoggingError> for LoggingError {
    fn from(error: fastlogging::LoggingError) -> Self {
        LoggingError(error)
    }
}

impl From<LoggingError> for fastlogging::LoggingError {
    fn from(val: LoggingError) -> Self {
        val.0
    }
}

impl From<pyo3::PyErr> for LoggingError {
    fn from(error: pyo3::PyErr) -> Self {
        LoggingError(fastlogging::LoggingError::InvalidValue(error.to_string()))
    }
}
