use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use once_cell::sync::Lazy;
use pyo3::types::PyTuple;
use pyo3::{prelude::*, IntoPyObjectExt};

use crate::def::{CompressionMethodEnum, MessageStructEnum};
use crate::{EncryptionMethod, LoggingError};

#[pyclass]
#[derive(Debug, Clone)]
pub struct ExtConfig(pub fastlogging::ExtConfig);

#[pymethods]
impl ExtConfig {
    #[new]
    pub fn new(
        structured: MessageStructEnum,
        hostname: bool,
        pname: bool,
        pid: bool,
        tname: bool,
        tid: bool,
    ) -> Self {
        Self(fastlogging::ExtConfig::new(
            structured.into(),
            hostname,
            pname,
            pid,
            tname,
            tid,
        ))
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct RootConfig(pub fastlogging::RootConfig);

impl From<RootConfig> for fastlogging::RootConfig {
    fn from(val: RootConfig) -> Self {
        val.0
    }
}

impl From<fastlogging::RootConfig> for RootConfig {
    fn from(val: fastlogging::RootConfig) -> RootConfig {
        RootConfig(val)
    }
}

impl From<&RootConfig> for fastlogging::RootConfig {
    fn from(val: &RootConfig) -> Self {
        val.0.clone()
    }
}

impl From<&fastlogging::RootConfig> for RootConfig {
    fn from(val: &fastlogging::RootConfig) -> RootConfig {
        RootConfig(val.clone())
    }
}

#[pymethods]
impl RootConfig {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ConsoleWriterConfig(pub fastlogging::ConsoleWriterConfig);

#[pymethods]
impl ConsoleWriterConfig {
    #[new]
    pub fn new(level: u8, colors: bool) -> Self {
        Self(fastlogging::ConsoleWriterConfig::new(level, colors))
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

impl From<ConsoleWriterConfig> for fastlogging::ConsoleWriterConfig {
    fn from(val: ConsoleWriterConfig) -> Self {
        val.0
    }
}

impl From<fastlogging::ConsoleWriterConfig> for ConsoleWriterConfig {
    fn from(val: fastlogging::ConsoleWriterConfig) -> ConsoleWriterConfig {
        ConsoleWriterConfig(val)
    }
}

impl From<&ConsoleWriterConfig> for fastlogging::ConsoleWriterConfig {
    fn from(val: &ConsoleWriterConfig) -> Self {
        val.0.clone()
    }
}

impl From<&fastlogging::ConsoleWriterConfig> for ConsoleWriterConfig {
    fn from(val: &fastlogging::ConsoleWriterConfig) -> ConsoleWriterConfig {
        ConsoleWriterConfig(val.clone())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct FileWriterConfig(pub fastlogging::FileWriterConfig);

#[pymethods]
impl FileWriterConfig {
    #[new]
    #[pyo3(signature=(level, path, size=None, backlog=None, timeout=None, time=None, compression=None))]
    pub fn new(
        level: u8,
        path: PathBuf,
        size: Option<usize>,
        backlog: Option<usize>,
        timeout: Option<Duration>,
        time: Option<SystemTime>,
        compression: Option<CompressionMethodEnum>,
    ) -> Result<Self, LoggingError> {
        Ok(Self(fastlogging::FileWriterConfig::new(
            level,
            path,
            size.unwrap_or_default(),
            backlog.unwrap_or_default(),
            timeout,
            time,
            compression.map(|x| x.into()),
        )?))
    }
}

impl From<FileWriterConfig> for fastlogging::FileWriterConfig {
    fn from(val: FileWriterConfig) -> Self {
        val.0
    }
}

impl From<fastlogging::FileWriterConfig> for FileWriterConfig {
    fn from(val: fastlogging::FileWriterConfig) -> FileWriterConfig {
        FileWriterConfig(val)
    }
}

impl From<&FileWriterConfig> for fastlogging::FileWriterConfig {
    fn from(val: &FileWriterConfig) -> Self {
        val.0.clone()
    }
}

impl From<&fastlogging::FileWriterConfig> for FileWriterConfig {
    fn from(val: &fastlogging::FileWriterConfig) -> FileWriterConfig {
        FileWriterConfig(val.clone())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ServerConfig(pub fastlogging::ServerConfig);

#[pymethods]
impl ServerConfig {
    #[new]
    #[pyo3(signature=(level, address, key=None))]
    pub fn new(level: u8, address: String, key: Option<EncryptionMethod>) -> Self {
        let key = key.unwrap_or(EncryptionMethod::NONE {});
        Self(fastlogging::ServerConfig::new(level, address, key.into()))
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

impl From<ServerConfig> for fastlogging::ServerConfig {
    fn from(val: ServerConfig) -> Self {
        val.0
    }
}

impl From<fastlogging::ServerConfig> for ServerConfig {
    fn from(val: fastlogging::ServerConfig) -> ServerConfig {
        ServerConfig(val)
    }
}

impl From<&ServerConfig> for fastlogging::ServerConfig {
    fn from(val: &ServerConfig) -> Self {
        val.0.clone()
    }
}

impl From<&fastlogging::ServerConfig> for ServerConfig {
    fn from(val: &fastlogging::ServerConfig) -> ServerConfig {
        ServerConfig(val.clone())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ClientWriterConfig(pub fastlogging::ClientWriterConfig);

#[pymethods]
impl ClientWriterConfig {
    #[new]
    #[pyo3(signature=(level, address, key=None))]
    pub fn new(level: u8, address: String, key: Option<EncryptionMethod>) -> Self {
        let key = key.unwrap_or(EncryptionMethod::NONE {});
        Self(fastlogging::ClientWriterConfig::new(
            level,
            address,
            key.into(),
        ))
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

impl From<ClientWriterConfig> for fastlogging::ClientWriterConfig {
    fn from(val: ClientWriterConfig) -> Self {
        val.0
    }
}

impl From<fastlogging::ClientWriterConfig> for ClientWriterConfig {
    fn from(val: fastlogging::ClientWriterConfig) -> ClientWriterConfig {
        ClientWriterConfig(val)
    }
}

impl From<&ClientWriterConfig> for fastlogging::ClientWriterConfig {
    fn from(val: &ClientWriterConfig) -> Self {
        val.0.clone()
    }
}

impl From<&fastlogging::ClientWriterConfig> for ClientWriterConfig {
    fn from(val: &fastlogging::ClientWriterConfig) -> ClientWriterConfig {
        ClientWriterConfig(val.clone())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct SyslogWriterConfig(pub fastlogging::SyslogWriterConfig);

#[pymethods]
impl SyslogWriterConfig {
    #[new]
    #[pyo3(signature=(level, hostname=None, pname=None, pid=None))]
    pub fn new(
        level: u8,
        hostname: Option<String>,
        pname: Option<String>,
        pid: Option<u32>,
    ) -> Self {
        Self(fastlogging::SyslogWriterConfig::new(
            level,
            hostname,
            pname.unwrap_or_default(),
            pid.unwrap_or_default(),
        ))
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

impl From<SyslogWriterConfig> for fastlogging::SyslogWriterConfig {
    fn from(val: SyslogWriterConfig) -> Self {
        val.0
    }
}

impl From<fastlogging::SyslogWriterConfig> for SyslogWriterConfig {
    fn from(val: fastlogging::SyslogWriterConfig) -> SyslogWriterConfig {
        SyslogWriterConfig(val)
    }
}

impl From<&SyslogWriterConfig> for fastlogging::SyslogWriterConfig {
    fn from(val: &SyslogWriterConfig) -> Self {
        val.0.clone()
    }
}

impl From<&fastlogging::SyslogWriterConfig> for SyslogWriterConfig {
    fn from(val: &fastlogging::SyslogWriterConfig) -> SyslogWriterConfig {
        SyslogWriterConfig(val.clone())
    }
}

pub static CALLBACK_PY_FUNC: Lazy<Mutex<Option<PyObject>>> = Lazy::new(|| Mutex::new(None));

pub fn callback_func(
    level: u8,
    domain: String,
    message: String,
) -> Result<(), fastlogging::LoggingError> {
    if let Some(callable) = CALLBACK_PY_FUNC.lock().unwrap().as_ref() {
        Python::with_gil(|py| -> Result<(), LoggingError> {
            let args = PyTuple::new(
                py,
                &[
                    level.into_py_any(py)?,
                    domain.into_py_any(py)?,
                    message.into_py_any(py)?,
                ],
            )?;
            callable.call(py, args, None)?;
            Ok(())
        })
        .map_err(|e| {
            let err: fastlogging::LoggingError = e.into();
            err
        })?;
    }
    Ok(())
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct CallbackWriterConfig(pub fastlogging::CallbackWriterConfig);

#[pymethods]
impl CallbackWriterConfig {
    #[new]
    #[pyo3(signature=(level, callback=None))]
    pub fn new(level: u8, callback: Option<PyObject>) -> Self {
        *CALLBACK_PY_FUNC.lock().unwrap() = callback;
        Self(fastlogging::CallbackWriterConfig::new(
            level,
            Some(Box::new(callback_func)),
        ))
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}
