use std::{fmt, path::PathBuf};

use crate::{
    callback::CallbackWriterConfig, config::LoggingInstance, CallbackWriter, ClientWriter,
    ClientWriterConfig, ConsoleWriter, ConsoleWriterConfig, FileWriter, FileWriterConfig,
    LoggingError, LoggingServer, ServerConfig, SyslogWriter, SyslogWriterConfig,
};

// Log-Levels
pub const NOLOG: u8 = 70;
pub const EXCEPTION: u8 = 60;
pub const CRITICAL: u8 = 50;
pub const FATAL: u8 = CRITICAL;
pub const ERROR: u8 = 40;
pub const WARNING: u8 = 30;
pub const WARN: u8 = WARNING;
pub const SUCCESS: u8 = 25;
pub const INFO: u8 = 20;
pub const DEBUG: u8 = 10;
pub const TRACE: u8 = 5;
pub const NOTSET: u8 = 0;

pub fn level2str(level: u8) -> &'static str {
    match level {
        NOTSET..TRACE => "NOTSET",
        TRACE..DEBUG => "TRACE",
        DEBUG..INFO => "DEBUG",
        INFO..SUCCESS => "INFO",
        SUCCESS..WARNING => "SUCCESS",
        WARNING..ERROR => "WARNING",
        ERROR..FATAL => "ERROR",
        FATAL..EXCEPTION => "FATAL",
        EXCEPTION..NOLOG => "EXCEPTION",
        _ => "NOLOG",
    }
}

pub fn level2short(level: u8) -> &'static str {
    match level {
        NOTSET..TRACE => "NOT",
        TRACE..DEBUG => "TRC",
        DEBUG..INFO => "DBG",
        INFO..SUCCESS => "INF",
        SUCCESS..WARNING => "SCS",
        WARNING..ERROR => "WRN",
        ERROR..FATAL => "ERR",
        FATAL..EXCEPTION => "FTL",
        EXCEPTION..NOLOG => "EXC",
        _ => "NOL",
    }
}

pub fn level2sym(level: u8) -> &'static str {
    match level {
        NOTSET..TRACE => "N",
        TRACE..DEBUG => "T",
        DEBUG..INFO => "D",
        INFO..SUCCESS => "I",
        SUCCESS..WARNING => "S",
        WARNING..ERROR => "W",
        ERROR..FATAL => "E",
        FATAL..EXCEPTION => "F",
        EXCEPTION..NOLOG => "!",
        _ => "-",
    }
}

pub fn level2string(levelsym: &LevelSyms, level: u8) -> &'static str {
    match levelsym {
        LevelSyms::Sym => level2sym(level),
        LevelSyms::Short => level2short(level),
        LevelSyms::Str => level2str(level),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LevelSyms {
    Sym,
    Short,
    Str,
}

impl fmt::Display for LevelSyms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootConfig {
    pub level: u8,
    pub domain: String,
    pub hostname: Option<String>,
    pub pname: String,
    pub pid: u32,
    pub tname: bool,
    pub tid: bool,
    pub structured: MessageStructEnum,
    pub level2sym: LevelSyms,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WriterTypeEnum {
    Root,
    Console,
    File(PathBuf),
    Client(String),
    Server(String),
    Callback,
    Syslog,
}

impl fmt::Display for WriterTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WriterConfigEnum {
    Root(RootConfig),
    Console(ConsoleWriterConfig),
    File(FileWriterConfig),
    Client(ClientWriterConfig),
    Server(ServerConfig),
    Callback(CallbackWriterConfig),
    Syslog(SyslogWriterConfig),
}

impl WriterConfigEnum {
    pub fn new(instance: &LoggingInstance, writer: &WriterEnum) -> Self {
        match writer {
            WriterEnum::Root => WriterConfigEnum::Root(RootConfig {
                level: instance.level,
                domain: instance.domain.clone(),
                hostname: instance.hostname.clone(),
                pname: instance.pname.clone(),
                pid: instance.pid,
                tname: instance.tname,
                tid: instance.tid,
                structured: instance.structured.clone(),
                level2sym: instance.level2sym.clone(),
            }),
            WriterEnum::Console(console_writer) => {
                WriterConfigEnum::Console(console_writer.config.lock().unwrap().clone())
            }

            WriterEnum::File(file_writer) => {
                WriterConfigEnum::File(file_writer.config.lock().unwrap().clone())
            }

            WriterEnum::Client(client_writer) => {
                WriterConfigEnum::Client(client_writer.config.lock().unwrap().get_client_config())
            }

            WriterEnum::Server(logging_server) => {
                WriterConfigEnum::Server(logging_server.config.lock().unwrap().get_server_config())
            }

            WriterEnum::Callback(callback_writer) => {
                WriterConfigEnum::Callback(callback_writer.config.lock().unwrap().clone())
            }

            WriterEnum::Syslog(syslog_writer) => {
                WriterConfigEnum::Syslog(syslog_writer.config.lock().unwrap().clone())
            }
        }
    }
}

impl fmt::Display for WriterConfigEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<ConsoleWriterConfig> for WriterConfigEnum {
    fn from(config: ConsoleWriterConfig) -> Self {
        Self::Console(config)
    }
}

impl From<FileWriterConfig> for WriterConfigEnum {
    fn from(config: FileWriterConfig) -> Self {
        Self::File(config)
    }
}

impl From<ClientWriterConfig> for WriterConfigEnum {
    fn from(config: ClientWriterConfig) -> Self {
        Self::Client(config)
    }
}

impl From<ServerConfig> for WriterConfigEnum {
    fn from(config: ServerConfig) -> Self {
        Self::Server(config)
    }
}

impl From<CallbackWriterConfig> for WriterConfigEnum {
    fn from(config: CallbackWriterConfig) -> Self {
        Self::Callback(config)
    }
}

impl From<SyslogWriterConfig> for WriterConfigEnum {
    fn from(config: SyslogWriterConfig) -> Self {
        Self::Syslog(config)
    }
}

#[derive(Debug)]
pub enum WriterEnum {
    Root,
    Console(ConsoleWriter),
    File(FileWriter),
    Client(ClientWriter),
    Server(LoggingServer),
    Callback(CallbackWriter),
    Syslog(SyslogWriter),
}

impl WriterEnum {
    pub fn new(
        instance: &mut LoggingInstance,
        config: &WriterConfigEnum,
    ) -> Result<Self, LoggingError> {
        match config {
            WriterConfigEnum::Root(root_config) => {
                instance.level = root_config.level;
                instance.domain = root_config.domain.clone();
                instance.hostname = root_config.hostname.clone();
                instance.pname = root_config.pname.clone();
                instance.pid = root_config.pid;
                instance.tname = root_config.tname;
                instance.tid = root_config.tid;
                instance.structured = root_config.structured.clone();
                instance.level2sym = root_config.level2sym.clone();
                Ok(WriterEnum::Root)
            }
            WriterConfigEnum::Console(console_writer_config) => Ok(WriterEnum::Console(
                ConsoleWriter::new(console_writer_config.clone(), instance.stop.clone())?,
            )),
            WriterConfigEnum::File(file_writer_config) => Ok(WriterEnum::File(FileWriter::new(
                file_writer_config.clone(),
                instance.stop.clone(),
            )?)),
            WriterConfigEnum::Client(client_writer_config) => Ok(WriterEnum::Client(
                ClientWriter::new(client_writer_config.clone(), instance.stop.clone())?,
            )),
            WriterConfigEnum::Server(server_config) => Ok(WriterEnum::Server(LoggingServer::new(
                server_config.clone(),
                instance.server_tx.clone(),
                instance.stop.clone(),
            )?)),
            WriterConfigEnum::Callback(callback_writer_config) => Ok(WriterEnum::Callback(
                CallbackWriter::new(callback_writer_config.clone(), instance.stop.clone())?,
            )),
            WriterConfigEnum::Syslog(syslog_writer_config) => Ok(WriterEnum::Syslog(
                SyslogWriter::new(syslog_writer_config.clone(), instance.stop.clone())?,
            )),
        }
    }
}

#[derive(Debug)]
pub enum LoggingTypeEnum {
    Message((u8, String, String)),                 // level, domain, message
    MessageRemote((u8, String, String)),           // level, domain, message
    MessageExt((u8, String, String, u32, String)), // level, domain, message, tname, tid
    Sync((bool, bool, bool, bool, bool, f64)),     // console, file, client, syslog, timeout
    Rotate,
    Stop,
}

impl fmt::Display for LoggingTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageStructEnum {
    String,
    Json,
    Xml,
}

impl fmt::Display for MessageStructEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
