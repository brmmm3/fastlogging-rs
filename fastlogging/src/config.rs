use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::str;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use flume::{bounded, Receiver, Sender};
use gethostname::gethostname;

const CONFIG_FILE_SIZE_MAX: u64 = 4096;

use crate::level2string;
use crate::LoggingError;
use crate::{
    ClientWriter, ClientWriterConfig, ConsoleWriter, ConsoleWriterConfig, FileWriter,
    FileWriterConfig, LevelSyms, LoggingServer, LoggingTypeEnum, MessageStructEnum, ServerConfig,
    SyslogWriter, SyslogWriterConfig, NOTSET,
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExtConfig {
    pub(crate) structured: MessageStructEnum,
    pub(crate) hostname: bool, // Log hostname
    pub(crate) pname: bool,    // Log process name
    pub(crate) pid: bool,      // Log process ID
    pub(crate) tname: bool,    // Log thread name
    pub(crate) tid: bool,      // Log thread ID
}

impl ExtConfig {
    pub fn new(
        structured: MessageStructEnum,
        hostname: bool,
        pname: bool,
        pid: bool,
        tname: bool,
        tid: bool,
    ) -> Self {
        Self {
            structured,
            hostname,
            pname,
            pid,
            tname,
            tid,
        }
    }
}

impl Default for ExtConfig {
    fn default() -> Self {
        Self {
            hostname: false,
            pname: false,
            pid: false,
            tname: false,
            tid: false,
            structured: MessageStructEnum::String,
        }
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FileConfig {
    pub(crate) level: u8,
    pub(crate) domain: String,
    pub(crate) hostname: Option<String>,
    pub(crate) pname: String,
    pub(crate) pid: u32,
    pub(crate) tname: bool,
    pub(crate) tid: bool,
    pub(crate) structured: MessageStructEnum,
    pub(crate) console: Option<ConsoleWriterConfig>,
    pub(crate) file: Option<FileWriterConfig>,
    pub(crate) server: Option<ServerConfig>,
    pub(crate) connect: Option<ClientWriterConfig>,
    pub(crate) syslog: Option<SyslogWriterConfig>,
    pub(crate) level2sym: LevelSyms,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            level: NOTSET,
            domain: "root".to_string(),
            hostname: None,
            pname: "".to_string(),
            pid: 0,
            tname: false,
            tid: false,
            structured: MessageStructEnum::String,
            console: None,
            file: None,
            server: None,
            connect: None,
            syslog: None,
            level2sym: LevelSyms::Sym,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: u8,
    pub domain: String,
    pub hostname: Option<String>,
    pub pname: String,
    pub pid: u32,
    pub tname: bool,
    pub tid: bool,
    pub structured: MessageStructEnum,
    pub level2sym: LevelSyms,
    pub console: Option<ConsoleWriterConfig>,
    pub files: HashMap<PathBuf, FileWriterConfig>,
    pub servers: HashMap<String, ServerConfig>,
    pub clients: HashMap<String, ClientWriterConfig>,
    pub syslog: Option<SyslogWriterConfig>,
    pub debug: u8,
}

impl LoggingConfig {
    pub fn from_json_vec(data: &[u8]) -> Self {
        serde_json::from_slice(data).unwrap()
    }

    pub fn to_json_vec(&self) -> Result<Vec<u8>, LoggingError> {
        Ok(serde_json::to_vec(&self).unwrap())
    }
}

#[derive(Debug)]
pub struct LoggingInstance {
    pub(crate) level: u8,
    pub(crate) domain: String,
    pub(crate) hostname: Option<String>,
    pub(crate) pname: String,
    pub(crate) pid: u32,
    pub(crate) tname: bool,
    pub(crate) tid: bool,
    pub(crate) structured: MessageStructEnum,
    pub(crate) level2sym: LevelSyms,
    pub(crate) console: Option<ConsoleWriter>,
    pub(crate) files: HashMap<PathBuf, FileWriter>,
    pub(crate) servers: HashMap<String, LoggingServer>,
    pub(crate) clients: HashMap<String, ClientWriter>,
    pub(crate) syslog: Option<SyslogWriter>,
    pub(crate) debug: u8,
}

impl LoggingInstance {
    pub fn set_ext_config(&mut self, ext_config: ExtConfig) {
        self.structured = ext_config.structured;
        let hostname = if ext_config.hostname {
            Some(gethostname().into_string().unwrap())
        } else {
            None
        };
        self.hostname.clone_from(&hostname);
        let pname = (if ext_config.pname {
            std::env::current_exe()
                .ok()
                .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
                .and_then(|s| s.into_string().ok())
        } else {
            None
        })
        .unwrap_or_default();
        self.pname.clone_from(&pname);
        self.pid = if ext_config.pid { process::id() } else { 0 };
        self.tname = ext_config.tname;
        self.tid = ext_config.tid;
    }

    pub fn get_config(&self) -> LoggingConfig {
        LoggingConfig {
            level: self.level,
            domain: self.domain.clone(),
            hostname: self.hostname.clone(),
            pname: self.pname.clone(),
            pid: self.pid,
            tname: self.tname,
            tid: self.tid,
            structured: self.structured.clone(),
            level2sym: self.level2sym.clone(),
            console: self
                .console
                .as_ref()
                .map(|c| c.config.lock().unwrap().clone()),
            files: self
                .files
                .iter()
                .map(|(k, v)| (k.clone(), v.config.lock().unwrap().clone()))
                .collect(),
            servers: self
                .servers
                .iter()
                .map(|(k, v)| {
                    (k.clone(), {
                        let config = v.config.lock().unwrap();
                        ServerConfig {
                            level: config.level,
                            address: config.address.clone(),
                            port: config.port,
                            key: config.key.clone(),
                            port_file: config.port_file.clone(),
                        }
                    })
                })
                .collect(),
            clients: self
                .clients
                .iter()
                .map(|(k, v)| {
                    (k.clone(), {
                        let config = v.config.lock().unwrap();
                        ClientWriterConfig {
                            level: config.level,
                            address: config.address.clone(),
                            port: config.port,
                            key: config.key.clone(),
                            debug: config.debug,
                        }
                    })
                })
                .collect(),
            syslog: self
                .syslog
                .as_ref()
                .map(|c| c.config.lock().unwrap().clone()),
            debug: self.debug,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ConfigFile {
    pub(crate) path: PathBuf,
    pub(crate) config: FileConfig,
}

pub fn default_config_file() -> (PathBuf, Vec<u8>) {
    #[cfg(feature = "config_json")]
    if Path::new("fastlogging.json").exists() {
        return (PathBuf::from("fastlogging.json"), b"json".to_vec());
    }
    #[cfg(feature = "config_xml")]
    if Path::new("fastlogging.xml").exists() {
        return (PathBuf::from("fastlogging.xml"), b"xml".to_vec());
    }
    #[cfg(feature = "config_yaml")]
    if Path::new("fastlogging.yaml").exists() {
        return (PathBuf::from("fastlogging.yaml"), b"yaml".to_vec());
    }
    (PathBuf::new(), Vec::new())
}

impl ConfigFile {
    pub fn new(path: Option<PathBuf>) -> Result<Self, LoggingError> {
        // Initialize settings with default settings from optional config file.
        let (path, lextension) = if let Some(ref path) = path {
            if let Some(extension) = path.extension() {
                (
                    path.to_owned(),
                    extension.as_encoded_bytes().to_ascii_lowercase(),
                )
            } else {
                return Err(LoggingError::InvalidValue(
                    "Config file has no extension.".to_string(),
                ));
            }
        } else {
            default_config_file()
        };
        let data = if path.is_file() {
            match fs::metadata(&path) {
                Ok(m) => {
                    if m.len() > CONFIG_FILE_SIZE_MAX {
                        return Err(LoggingError::InvalidValue(
                            "Config file is too big!".to_string(),
                        ));
                    }
                }
                Err(err) => {
                    return Err(LoggingError::InvalidValue(format!(
                        "Failed to read config file metadata: {err:?}"
                    )));
                }
            }
            match fs::read_to_string(&path) {
                Ok(d) => Some(d),
                Err(err) => {
                    return Err(LoggingError::InvalidValue(format!(
                        "Failed to read config file: {err:?}"
                    )));
                }
            }
        } else {
            None
        };
        let config = if let Some(data) = data {
            let file_config: FileConfig = match if lextension == b"json" {
                ConfigFile::from_json(&path, &data)
            } else if lextension == b"xml" {
                ConfigFile::from_xml(&path, &data)
            } else if lextension == b"yaml" {
                ConfigFile::from_yaml(&path, &data)
            } else {
                return Err(LoggingError::InvalidValue(format!(
                    "Unsupported config file type {}",
                    str::from_utf8(&lextension).unwrap()
                )));
            } {
                Ok(d) => d,
                Err(err) => {
                    return Err(LoggingError::InvalidValue(format!(
                        "Failed to read config file {path:?}: {err:?}"
                    )));
                }
            };
            file_config
        } else {
            FileConfig::default()
        };
        Ok(Self { path, config })
    }

    pub fn init(
        &mut self,
        level: Option<u8>, // Global log level
        domain: Option<String>,
        ext_config: Option<ExtConfig>, // Extended logging configuration
        console: Option<ConsoleWriterConfig>, // If config is defined start ConsoleLogging
        file: Option<FileWriterConfig>, // If config is defined start FileLogging
        server: Option<ServerConfig>,  // If config is defined start LoggingServer
        connect: Option<ClientWriterConfig>, // If config is defined start ClientLogging
        syslog: Option<u8>,            // If log level is defined start SyslogLogging
    ) -> Result<
        (
            LoggingInstance,
            Sender<LoggingTypeEnum>,
            Receiver<LoggingTypeEnum>,
            Arc<AtomicBool>,
        ),
        LoggingError,
    > {
        // Use settings from optional config file as default and overwrite them if provided here as arguments.
        let (tx, rx) = bounded(1000);
        let stop = Arc::new(AtomicBool::new(false));
        let level = level.unwrap_or(self.config.level);
        let domain = domain.unwrap_or(self.config.domain.clone());
        // Console writer
        let console = if let Some(config) = console {
            self.config.console = Some(config.clone());
            Some(ConsoleWriter::new(config, stop.clone())?)
        } else if let Some(ref config) = self.config.console {
            Some(ConsoleWriter::new(config.to_owned(), stop.clone())?)
        } else {
            None
        };
        // File writer
        let mut files = HashMap::new();
        if let Some(config) = file {
            self.config.file = Some(config.clone());
            files.insert(config.path.clone(), FileWriter::new(config, stop.clone())?);
        } else if let Some(ref config) = self.config.file {
            files.insert(
                config.path.clone(),
                FileWriter::new(config.to_owned(), stop.clone())?,
            );
        };
        // Network writer
        let mut clients = HashMap::new();
        if let Some(config) = connect {
            self.config.connect = Some(config.clone());
            clients.insert(
                config.address.clone(),
                ClientWriter::new(config, stop.clone())?,
            );
        } else if let Some(ref config) = self.config.connect {
            clients.insert(
                config.address.clone(),
                ClientWriter::new(config.to_owned(), stop.clone())?,
            );
        }
        // Logging server
        let mut servers = HashMap::new();
        if let Some(config) = server {
            self.config.server = Some(config.clone());
            let server = LoggingServer::new(config, tx.clone(), stop.clone())?;
            let address = server.config.lock().unwrap().get_address();
            servers.insert(address, server);
        } else if let Some(ref config) = self.config.server {
            let server = LoggingServer::new(config.to_owned(), tx.clone(), stop.clone())?;
            let address = server.config.lock().unwrap().get_address();
            servers.insert(address, server);
        };
        let mut config = LoggingInstance {
            level,
            domain,
            hostname: self.config.hostname.clone(),
            pname: self.config.pname.clone(),
            pid: self.config.pid,
            tname: false,
            tid: false,
            structured: self.config.structured.clone(),
            console,
            files,
            servers,
            clients,
            syslog: None,
            level2sym: LevelSyms::Sym,
            debug: 0,
        };
        if let Some(ext_config) = ext_config {
            config.set_ext_config(ext_config);
        }
        // Syslog
        config.syslog = if let Some(level) = syslog {
            self.config.structured = config.structured.clone();
            Some(SyslogWriter::new(
                SyslogWriterConfig::new(
                    level,
                    config.hostname.clone(),
                    config.pname.clone(),
                    config.pid,
                ),
                stop.clone(),
            )?)
        } else {
            None
        };
        Ok((config, tx, rx, stop))
    }

    pub fn from_json(path: &Path, data: &str) -> Result<FileConfig, LoggingError> {
        #[cfg(feature = "config_json")]
        let file_data = serde_json::from_str(data).map_err(|e| {
            LoggingError::InvalidValue(format!("Failed to read config file {path:?}: {e:?}"))
        });
        #[cfg(not(feature = "config_json"))]
        let file_data = Err(LoggingError::InvalidValue(
            "Support for JSON type config files is not enabled".to_string(),
        ));
        file_data
    }

    pub fn to_json(&self) -> Result<String, LoggingError> {
        #[cfg(feature = "config_json")]
        let data = serde_json::to_string_pretty(&self.config).map_err(|e| {
            LoggingError::InvalidValue(format!("Failed to serialize file configuration: {e:?}"))
        });
        #[cfg(not(feature = "config_json"))]
        let data = Err(LoggingError::InvalidValue(
            "Support for JSON type config files is not enabled".to_string(),
        ));
        data
    }

    pub fn from_xml(path: &Path, data: &str) -> Result<FileConfig, LoggingError> {
        #[cfg(feature = "config_xml")]
        let file_data = quick_xml::de::from_str(data).map_err(|e| {
            LoggingError::InvalidValue(format!("Failed to read config file {path:?}: {e:?}"))
        });
        #[cfg(not(feature = "config_xml"))]
        let file_data = Err(LoggingError::InvalidValue(
            "Support for XML type config files is not enabled".to_string(),
        ));
        file_data
    }

    pub fn to_xml(&self) -> Result<String, LoggingError> {
        #[cfg(feature = "config_xml")]
        let data = quick_xml::se::to_string(&self.config).map_err(|e| {
            LoggingError::InvalidValue(format!("Failed to serialize file configuration: {e:?}"))
        });
        #[cfg(not(feature = "config_xml"))]
        let data = Err(LoggingError::InvalidValue(
            "Support for XML type config files is not enabled".to_string(),
        ));
        data
    }

    pub fn from_yaml(path: &Path, data: &str) -> Result<FileConfig, LoggingError> {
        #[cfg(feature = "config_yaml")]
        let file_data = serde_yaml::from_str(data).map_err(|e| {
            LoggingError::InvalidValue(format!("Failed to read config file {path:?}: {e:?}"))
        });
        #[cfg(not(feature = "config_yaml"))]
        let file_data = Err(LoggingError::InvalidValue(
            "Support for YAML type config files is not enabled".to_string(),
        ));
        file_data
    }

    pub fn to_yaml(&self) -> Result<String, LoggingError> {
        #[cfg(feature = "config_yaml")]
        let data = serde_yaml::to_string(&self.config).map_err(|e| {
            LoggingError::InvalidValue(format!("Failed to serialize file configuration: {e:?}"))
        });
        #[cfg(not(feature = "config_yaml"))]
        let data = Err(LoggingError::InvalidValue(
            "Support for YAML type config files is not enabled".to_string(),
        ));
        data
    }

    pub fn save(&self, path: &Path) -> Result<(), LoggingError> {
        let (path, lextension) = {
            if let Some(extension) = path.extension() {
                (
                    path.to_owned(),
                    extension.as_encoded_bytes().to_ascii_lowercase(),
                )
            } else {
                return Err(LoggingError::InvalidValue(
                    "Config file has no extension.".to_string(),
                ));
            }
        };
        let data = (if lextension == b"json" {
            self.to_json()
        } else if lextension == b"xml" {
            self.to_xml()
        } else if lextension == b"yaml" {
            self.to_yaml()
        } else {
            return Err(LoggingError::InvalidValue(format!(
                "Unsupported config file type {}",
                str::from_utf8(&lextension).unwrap()
            )));
        })?;
        fs::write(path, data)?;
        Ok(())
    }
}

impl fmt::Display for ConfigFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let config = &self.config;
        write!(
            f,
            "path={:?}\n\
                   level={:?}\n\
                   domain={:?}\n\
                   hostname={:?}\n\
                   pname={:?}\n\
                   pid={}\n\
                   tname={:?}\n\
                   tid={}\n\
                   structured={:?}\n\
                   console={:?}\n\
                   file={:?}\n\
                   syslog={:?}\n\
                   server={:?}\n\
                   clients={:?}",
            self.path,
            level2string(&config.level2sym, config.level),
            config.domain,
            config.hostname,
            config.pname,
            config.pid,
            config.tname,
            config.tid,
            config.structured,
            config.console,
            config.file,
            config.syslog,
            config.server,
            config.connect
        )
    }
}
