use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass]
pub enum Level2Sym {
    NotSet = 0,
    Debug = 10,
    Info = 20,
    Warning = 30,
    Error = 40,
    Critical = 50,
    Exception = 60,
    NoLog = 70,
}

#[pymethods]
impl Level2Sym {
    #[new]
    fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Level2Sym::NotSet),
            10 => Ok(Level2Sym::Debug),
            20 => Ok(Level2Sym::Info),
            30 => Ok(Level2Sym::Warning),
            40 => Ok(Level2Sym::Error),
            50 => Ok(Level2Sym::Critical),
            60 => Ok(Level2Sym::Exception),
            70 => Ok(Level2Sym::NoLog),
            _ => Err(PyValueError::new_err(format!("Invalid value {value}"))),
        }
    }

    #[getter]
    fn value(&self) -> u8 {
        match self {
            Self::NotSet => 0,
            Self::Debug => 10,
            Self::Info => 20,
            Self::Warning => 30,
            Self::Error => 40,
            Self::Critical => 50,
            Self::Exception => 60,
            Self::NoLog => 70,
        }
    }

    #[getter]
    fn name(&self) -> &'static str {
        match self {
            Self::NotSet => "NOTSET",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warning => "WARNING",
            Self::Error => "ERROR",
            Self::Critical => "CRITICAL",
            Self::Exception => "EXCEPTION",
            Self::NoLog => "NOLOG",
        }
    }
}

#[pyclass]
pub struct LevelSyms(pub fastlogging::LevelSyms);

#[pymethods]
impl LevelSyms {
    #[new]
    fn new() -> Self {
        Self(fastlogging::LevelSyms::Sym)
    }

    #[getter]
    pub fn value(&self) -> u8 {
        self.0.clone() as u8
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.to_string()
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub enum MessageStructEnum {
    String,
    Json,
    Xml,
}

impl Into<fastlogging::MessageStructEnum> for MessageStructEnum {
    fn into(self) -> fastlogging::MessageStructEnum {
        match self {
            Self::String => fastlogging::MessageStructEnum::String,
            Self::Json => fastlogging::MessageStructEnum::Json,
            Self::Xml => fastlogging::MessageStructEnum::Xml,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub enum CompressionMethodEnum {
    Store,
    Deflate,
    Zstd,
    Lzma,
}

impl Into<fastlogging::CompressionMethodEnum> for CompressionMethodEnum {
    fn into(self) -> fastlogging::CompressionMethodEnum {
        match self {
            Self::Store => fastlogging::CompressionMethodEnum::Store,
            Self::Deflate => fastlogging::CompressionMethodEnum::Deflate,
            Self::Zstd => fastlogging::CompressionMethodEnum::Zstd,
            Self::Lzma => fastlogging::CompressionMethodEnum::Lzma,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub enum EncryptionMethod {
    NONE {},
    AuthKey { key: Vec<u8> },
    AES { key: Vec<u8> },
}

impl Into<fastlogging::EncryptionMethod> for EncryptionMethod {
    fn into(self) -> fastlogging::EncryptionMethod {
        match self {
            Self::NONE {} => fastlogging::EncryptionMethod::NONE,
            Self::AuthKey { key } => fastlogging::EncryptionMethod::AuthKey(key),
            Self::AES { key } => fastlogging::EncryptionMethod::AES(key),
        }
    }
}

#[pyclass]
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
}

#[pyclass]
pub struct ConsoleWriterConfig(pub fastlogging::ConsoleWriterConfig);

#[pymethods]
impl ConsoleWriterConfig {
    #[new]
    pub fn new(level: u8, colors: bool) -> Self {
        Self(fastlogging::ConsoleWriterConfig::new(level, colors))
    }
}

#[pyclass]
pub struct FileWriterConfig(pub fastlogging::FileWriterConfig);

#[pymethods]
impl FileWriterConfig {
    #[new]
    pub fn new(
        level: u8,
        path: PathBuf,
        size: usize,
        backlog: usize,
        timeout: Option<Duration>,
        time: Option<SystemTime>,
        compression: Option<CompressionMethodEnum>,
    ) -> PyResult<Self> {
        Ok(Self(fastlogging::FileWriterConfig::new(
            level,
            path,
            size,
            backlog,
            timeout,
            time,
            compression.map(|x| x.into()),
        )?))
    }
}

#[pyclass]
pub struct ServerConfig(pub fastlogging::ServerConfig);

#[pymethods]
impl ServerConfig {
    #[new]
    pub fn new(level: u8, address: String, key: EncryptionMethod) -> Self {
        Self(fastlogging::ServerConfig::new(level, address, key.into()))
    }
}

#[pyclass]
pub struct ClientWriterConfig(pub fastlogging::ClientWriterConfig);

#[pymethods]
impl ClientWriterConfig {
    #[new]
    pub fn new(level: u8, address: String, key: EncryptionMethod) -> Self {
        Self(fastlogging::ClientWriterConfig::new(
            level,
            address,
            key.into(),
        ))
    }
}
