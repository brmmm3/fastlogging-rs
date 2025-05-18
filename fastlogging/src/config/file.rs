use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;

const CONFIG_FILE_SIZE_MAX: u64 = 4096;

use crate::callback::CallbackWriter;
use crate::level2string;
use crate::LoggingError;
use crate::WriterConfigEnum;
use crate::WriterEnum;
use crate::WriterTypeEnum;
use crate::{
    ClientWriter, ConsoleWriter, FileWriter, LevelSyms, LoggingServer, MessageStructEnum,
    SyslogWriter, NOTSET,
};

use super::LoggingInstance;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileMerge {
    Replace,      // Replace complete configuration by config file contents
    Merge,        // Add only new writers
    MergeReplace, // Add new writers and replace existing writers
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
    pub(crate) level2sym: LevelSyms,
    pub(crate) configs: Vec<WriterConfigEnum>,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            level: NOTSET,
            domain: "".to_string(),
            hostname: None,
            pname: "".to_string(),
            pid: 0,
            tname: false,
            tid: false,
            structured: MessageStructEnum::String,
            level2sym: LevelSyms::Sym,
            configs: Vec::new(),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ConfigFile {
    pub(crate) path: PathBuf,
    pub(crate) config: FileConfig, // Configuration from file
}

pub fn default_config_file() -> (PathBuf, Vec<u8>) {
    if let Ok(path) = std::env::var("FASTLOGGING_CONFIG_FILE") {
        let path = PathBuf::from(path);
        if path.exists() {
            if let Some(ext) = path.extension() {
                let ext = ext.as_encoded_bytes().to_ascii_lowercase();
                if ext == b"json" || ext == b"yaml" || ext == b"xml" {
                    return (PathBuf::from(path), ext);
                }
            }
        }
    }
    #[cfg(feature = "config_json")]
    if Path::new("fastlogging.json").exists() {
        return (PathBuf::from("fastlogging.json"), b"json".to_vec());
    }
    #[cfg(feature = "config_yaml")]
    if Path::new("fastlogging.yaml").exists() {
        return (PathBuf::from("fastlogging.yaml"), b"yaml".to_vec());
    }
    #[cfg(feature = "config_xml")]
    if Path::new("fastlogging.xml").exists() {
        return (PathBuf::from("fastlogging.xml"), b"xml".to_vec());
    }
    (PathBuf::new(), Vec::new())
}

impl ConfigFile {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
            config: FileConfig::default(),
        }
    }

    pub fn load(&mut self, path: &Path) -> Result<(), LoggingError> {
        // Initialize settings with default settings from optional config file.
        let (path, lextension) = if let Some(extension) = path.extension() {
            (
                path.to_owned(),
                extension.as_encoded_bytes().to_ascii_lowercase(),
            )
        } else {
            return Err(LoggingError::InvalidValue(
                "Config file has no extension".to_string(),
            ));
        };
        if !path.is_file() {
            return Err(LoggingError::InvalidFile(format!(
                "File {path:?} not found or not accessible"
            )));
        }
        match fs::metadata(&path) {
            Ok(m) => {
                if m.len() > CONFIG_FILE_SIZE_MAX {
                    return Err(LoggingError::InvalidValue(
                        "Config file is too big".to_string(),
                    ));
                }
            }
            Err(err) => {
                return Err(LoggingError::InvalidValue(format!(
                    "Failed to read config file metadata: {err:?}"
                )));
            }
        }
        let data = match fs::read_to_string(&path) {
            Ok(d) => d,
            Err(err) => {
                return Err(LoggingError::InvalidValue(format!(
                    "Failed to read config file: {err:?}"
                )));
            }
        };
        self.config = match if lextension == b"json" {
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
        self.path = path;
        Ok(())
    }

    pub fn save(&mut self, path: Option<&Path>) -> Result<(), LoggingError> {
        let path = path.unwrap_or(&self.path);
        let (path, lextension) = {
            if let Some(extension) = path.extension() {
                (
                    path.to_owned(),
                    extension.as_encoded_bytes().to_ascii_lowercase(),
                )
            } else {
                return Err(LoggingError::InvalidValue(
                    "Config file has no extension".to_string(),
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
        fs::write(&path, data)?;
        self.path = path.to_path_buf();
        Ok(())
    }

    pub fn merge(
        &self,
        instance: &mut LoggingInstance,
        merge: FileMerge,
    ) -> Result<(), LoggingError> {
        if !self.path.is_file() {
            return Ok(());
        }
        if merge == FileMerge::Replace {
            instance.level = self.config.level;
            instance.domain = self.config.domain.clone();
            instance.hostname = self.config.hostname.clone();
            instance.tname = self.config.tname;
            instance.tid = self.config.tid;
            instance.structured = self.config.structured.clone();
            instance.level2sym = self.config.level2sym.clone();
            for (_wid, writer) in instance.writers.drain() {
                match writer {
                    WriterEnum::Root => {}
                    WriterEnum::Console(mut console_writer) => console_writer.shutdown()?,
                    WriterEnum::File(mut file_writer) => file_writer.shutdown()?,
                    WriterEnum::Client(mut client_writer) => client_writer.shutdown()?,
                    WriterEnum::Server(mut logging_server) => logging_server.shutdown()?,
                    WriterEnum::Callback(mut callback_writer) => callback_writer.shutdown()?,
                    WriterEnum::Syslog(mut syslog_writer) => syslog_writer.shutdown()?,
                }
            }
        } else {
            if self.config.level != NOTSET {
                instance.level = self.config.level;
            }
            if !self.config.domain.is_empty() {
                instance.domain = self.config.domain.clone();
            }
            if let Some(ref hostname) = self.config.hostname {
                instance.hostname = Some(hostname.clone());
            }
        }
        for config in self.config.configs.iter() {
            match config {
                WriterConfigEnum::Root(root_config) => {
                    instance.level = root_config.level;
                    instance.domain = root_config.domain.clone();
                    instance.hostname = root_config.hostname.clone();
                    instance.tname = root_config.tname;
                    instance.tid = root_config.tid;
                    instance.structured = root_config.structured.clone();
                    instance.level2sym = root_config.level2sym.clone();
                }
                WriterConfigEnum::Console(console_config) => {
                    let configs = instance.get_filtered_writer_configs(WriterTypeEnum::Console);
                    if merge == FileMerge::MergeReplace {
                        instance.remove_writers(Some(configs.into_keys().collect::<Vec<_>>()));
                    } else if merge == FileMerge::Merge && configs.is_empty() {
                        instance.add_writer(WriterEnum::Console(ConsoleWriter::new(
                            console_config.clone(),
                            instance.stop.clone(),
                        )?));
                    }
                }
                WriterConfigEnum::File(file_config) => {
                    let configs = instance.get_filtered_writer_configs(WriterTypeEnum::File(
                        file_config.path.to_str().unwrap().to_string(),
                    ));
                    if merge == FileMerge::MergeReplace {
                        instance.remove_writers(Some(configs.into_keys().collect::<Vec<_>>()));
                    } else if merge == FileMerge::Merge && configs.is_empty() {
                        instance.add_writer(WriterEnum::File(FileWriter::new(
                            file_config.clone(),
                            instance.stop.clone(),
                        )?));
                    }
                }
                WriterConfigEnum::Client(client_config) => {
                    let configs = instance.get_filtered_writer_configs(WriterTypeEnum::Client(
                        client_config.get_address_port(),
                    ));
                    if merge == FileMerge::MergeReplace {
                        instance.remove_writers(Some(configs.into_keys().collect::<Vec<_>>()));
                    } else if merge == FileMerge::Merge && configs.is_empty() {
                        instance.add_writer(WriterEnum::Client(ClientWriter::new(
                            client_config.clone(),
                            instance.stop.clone(),
                        )?));
                    }
                }
                WriterConfigEnum::Server(server_config) => {
                    let configs = instance.get_filtered_writer_configs(WriterTypeEnum::Server(
                        server_config.get_address_port(),
                    ));
                    if merge == FileMerge::MergeReplace {
                        instance.remove_writers(Some(configs.into_keys().collect::<Vec<_>>()));
                    } else if merge == FileMerge::Merge && configs.is_empty() {
                        instance.add_writer(WriterEnum::Server(LoggingServer::new(
                            server_config.clone(),
                            instance.server_tx.clone(),
                            instance.stop.clone(),
                        )?));
                    }
                }
                WriterConfigEnum::Callback(callback_config) => {
                    let configs = instance.get_filtered_writer_configs(WriterTypeEnum::Callback);
                    if merge == FileMerge::MergeReplace {
                        instance.remove_writers(Some(configs.into_keys().collect::<Vec<_>>()));
                    } else if merge == FileMerge::Merge && configs.is_empty() {
                        instance.add_writer(WriterEnum::Callback(CallbackWriter::new(
                            callback_config.clone(),
                            instance.stop.clone(),
                        )?));
                    }
                }
                WriterConfigEnum::Syslog(syslog_config) => {
                    let configs = instance.get_filtered_writer_configs(WriterTypeEnum::Syslog);
                    if merge == FileMerge::MergeReplace {
                        instance.remove_writers(Some(configs.into_keys().collect::<Vec<_>>()));
                    } else if merge == FileMerge::Merge && configs.is_empty() {
                        instance.add_writer(WriterEnum::Syslog(SyslogWriter::new(
                            syslog_config.clone(),
                            instance.stop.clone(),
                        )?));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn from_instance(path: &Path, instance: &LoggingInstance) -> Self {
        Self {
            path: path.to_path_buf(),
            config: FileConfig {
                level: instance.level,
                domain: instance.domain.clone(),
                hostname: instance.hostname.clone(),
                pname: instance.pname.clone(),
                pid: instance.pid,
                tname: instance.tname,
                tid: instance.tid,
                structured: instance.structured.clone(),
                level2sym: instance.level2sym.clone(),
                configs: instance
                    .get_writer_configs()
                    .into_values()
                    .collect::<Vec<_>>(),
            },
        }
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
                   configs={:?}",
            self.path,
            level2string(&config.level2sym, config.level),
            config.domain,
            config.hostname,
            config.pname,
            config.pid,
            config.tname,
            config.tid,
            config.structured,
            config.configs,
        )
    }
}
