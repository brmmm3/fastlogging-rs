use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use pyo3::{exceptions::PyValueError, prelude::*};

use crate::LoggingError;

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone)]
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

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageStructEnum {
    String,
    Json,
    Xml,
}

impl From<MessageStructEnum> for fastlogging::MessageStructEnum {
    fn from(val: MessageStructEnum) -> Self {
        use MessageStructEnum::*;
        match val {
            String => fastlogging::MessageStructEnum::String,
            Json => fastlogging::MessageStructEnum::Json,
            Xml => fastlogging::MessageStructEnum::Xml,
        }
    }
}

#[pymethods]
impl MessageStructEnum {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompressionMethodEnum {
    Store,
    Deflate,
    Zstd,
    Lzma,
}

impl From<CompressionMethodEnum> for fastlogging::CompressionMethodEnum {
    fn from(val: CompressionMethodEnum) -> Self {
        use CompressionMethodEnum::*;
        match val {
            Store => fastlogging::CompressionMethodEnum::Store,
            Deflate => fastlogging::CompressionMethodEnum::Deflate,
            Zstd => fastlogging::CompressionMethodEnum::Zstd,
            Lzma => fastlogging::CompressionMethodEnum::Lzma,
        }
    }
}

#[pymethods]
impl CompressionMethodEnum {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

#[pyclass(eq)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncryptionMethod {
    NONE {},
    AuthKey { key: Vec<u8> },
    AES { key: Vec<u8> },
}

impl From<EncryptionMethod> for fastlogging::EncryptionMethod {
    fn from(val: EncryptionMethod) -> Self {
        use EncryptionMethod::*;
        match val {
            NONE {} => fastlogging::EncryptionMethod::NONE,
            AuthKey { key } => fastlogging::EncryptionMethod::AuthKey(key),
            AES { key } => fastlogging::EncryptionMethod::AES(key),
        }
    }
}

impl From<fastlogging::EncryptionMethod> for EncryptionMethod {
    fn from(val: fastlogging::EncryptionMethod) -> Self {
        use fastlogging::EncryptionMethod::*;
        match val {
            NONE => EncryptionMethod::NONE {},
            AuthKey(key) => EncryptionMethod::AuthKey { key },
            AES(key) => EncryptionMethod::AES { key },
        }
    }
}

#[pymethods]
impl EncryptionMethod {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

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

#[pyclass]
#[derive(Debug, Clone)]
pub enum WriterConfigEnum {
    Root { config: RootConfig },
    Console { config: ConsoleWriterConfig },
    File { config: FileWriterConfig },
    Client { config: ClientWriterConfig },
    Server { config: ServerConfig },
    Syslog { config: SyslogWriterConfig },
}

impl From<WriterConfigEnum> for fastlogging::WriterConfigEnum {
    fn from(val: WriterConfigEnum) -> Self {
        use WriterConfigEnum::*;
        match val {
            Root { config } => fastlogging::WriterConfigEnum::Root(config.into()),
            Console { config } => fastlogging::WriterConfigEnum::Console(config.into()),
            File { config } => fastlogging::WriterConfigEnum::File(config.into()),
            Client { config } => fastlogging::WriterConfigEnum::Client(config.into()),
            Server { config } => fastlogging::WriterConfigEnum::Server(config.into()),
            Syslog { config } => fastlogging::WriterConfigEnum::Syslog(config.into()),
        }
    }
}

impl From<fastlogging::WriterConfigEnum> for WriterConfigEnum {
    fn from(val: fastlogging::WriterConfigEnum) -> WriterConfigEnum {
        use fastlogging::WriterConfigEnum::*;
        match val {
            Root(config) => WriterConfigEnum::Root {
                config: config.into(),
            },
            Console(config) => WriterConfigEnum::Console {
                config: config.into(),
            },
            File(config) => WriterConfigEnum::File {
                config: config.into(),
            },
            Client(config) => WriterConfigEnum::Client {
                config: config.into(),
            },
            Server(config) => WriterConfigEnum::Server {
                config: config.into(),
            },
            Syslog(config) => WriterConfigEnum::Syslog {
                config: config.into(),
            },
        }
    }
}

impl From<&WriterConfigEnum> for fastlogging::WriterConfigEnum {
    fn from(val: &WriterConfigEnum) -> Self {
        use WriterConfigEnum::*;
        match val {
            Root { config } => fastlogging::WriterConfigEnum::Root(config.into()),
            Console { config } => fastlogging::WriterConfigEnum::Console(config.into()),
            File { config } => fastlogging::WriterConfigEnum::File(config.into()),
            Client { config } => fastlogging::WriterConfigEnum::Client(config.into()),
            Server { config } => fastlogging::WriterConfigEnum::Server(config.into()),
            Syslog { config } => fastlogging::WriterConfigEnum::Syslog(config.into()),
        }
    }
}

impl<'a> From<&'a fastlogging::WriterConfigEnum> for WriterConfigEnum {
    fn from(val: &'a fastlogging::WriterConfigEnum) -> WriterConfigEnum {
        use fastlogging::WriterConfigEnum::*;
        match val {
            Root(config) => WriterConfigEnum::Root {
                config: config.into(),
            },
            Console(config) => WriterConfigEnum::Console {
                config: config.into(),
            },
            File(config) => WriterConfigEnum::File {
                config: config.into(),
            },
            Client(config) => WriterConfigEnum::Client {
                config: config.into(),
            },
            Server(config) => WriterConfigEnum::Server {
                config: config.into(),
            },
            Syslog(config) => WriterConfigEnum::Syslog {
                config: config.into(),
            },
        }
    }
}

#[pymethods]
impl WriterConfigEnum {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub enum WriterTypeEnum {
    Root {},
    Console {},
    File { path: PathBuf },
    Client { address: String },
    Server { address: String },
    Syslog {},
}

impl From<WriterTypeEnum> for fastlogging::WriterTypeEnum {
    fn from(val: WriterTypeEnum) -> Self {
        use WriterTypeEnum::*;
        match val {
            Root {} => fastlogging::WriterTypeEnum::Root,
            Console {} => fastlogging::WriterTypeEnum::Console,
            File { path } => fastlogging::WriterTypeEnum::File(path),
            Client { address } => fastlogging::WriterTypeEnum::Client(address),
            Server { address } => fastlogging::WriterTypeEnum::Server(address),
            Syslog {} => fastlogging::WriterTypeEnum::Syslog,
        }
    }
}

impl From<fastlogging::WriterTypeEnum> for WriterTypeEnum {
    fn from(val: fastlogging::WriterTypeEnum) -> Self {
        use fastlogging::WriterTypeEnum::*;
        match val {
            Root => WriterTypeEnum::Root {},
            Console => WriterTypeEnum::Console {},
            File(path) => WriterTypeEnum::File { path },
            Client(address) => WriterTypeEnum::Client { address },
            Server(address) => WriterTypeEnum::Server { address },
            Syslog => WriterTypeEnum::Syslog {},
        }
    }
}

#[pymethods]
impl WriterTypeEnum {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}
