use std::collections::HashMap;
use std::process;
use std::str;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use flume::bounded;
use flume::Receiver;
use flume::Sender;
use gethostname::gethostname;

use crate::LoggingError;
use crate::LoggingTypeEnum;
use crate::ServerConfig;
use crate::WriterConfigEnum;
use crate::WriterEnum;
use crate::WriterTypeEnum;
use crate::{LevelSyms, MessageStructEnum};

use super::ExtConfig;

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
    pub configs: HashMap<usize, WriterConfigEnum>,
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
    pub(crate) server_tx: Sender<LoggingTypeEnum>,
    pub(crate) server_rx: Receiver<LoggingTypeEnum>,
    pub(crate) writers: HashMap<usize, WriterEnum>,
    pub(crate) wid: usize, // Next writer ID
    pub(crate) debug: u8,
    pub(crate) stop: Arc<AtomicBool>,
}

impl LoggingInstance {
    pub fn new(
        level: u8,
        domain: String,
        configs: Vec<WriterConfigEnum>, // List of writer configs
    ) -> Result<Self, LoggingError> {
        let (server_tx, server_rx) = bounded(1000);
        let mut instance = Self {
            level,
            domain,
            hostname: None,
            pname: "".to_string(),
            pid: 0,
            tname: false,
            tid: false,
            structured: MessageStructEnum::String,
            level2sym: LevelSyms::Sym,
            server_tx,
            server_rx,
            writers: HashMap::new(),
            wid: 1,
            debug: 0,
            stop: Arc::new(AtomicBool::new(false)),
        };
        instance.add_writer_configs(&configs)?;
        Ok(instance)
    }

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

    pub fn get_writer_configs(&self) -> HashMap<usize, WriterConfigEnum> {
        self.writers
            .iter()
            .filter_map(|(k, w)| match w {
                WriterEnum::Root => None,
                _ => Some((*k, WriterConfigEnum::new(self, w))),
            })
            .collect::<HashMap<usize, _>>()
    }

    pub fn get_writer_config(&self, wid: usize) -> Option<WriterConfigEnum> {
        self.get_writer_configs().remove(&wid)
    }

    pub fn get_filtered_writer_configs(
        &self,
        wtype: WriterTypeEnum,
    ) -> HashMap<usize, WriterConfigEnum> {
        self.get_writer_configs()
            .into_iter()
            .filter(|w| match &w.1 {
                WriterConfigEnum::Root(_root_config) => wtype == WriterTypeEnum::Root,
                WriterConfigEnum::Console(_console_writer_config) => {
                    wtype == WriterTypeEnum::Console
                }
                WriterConfigEnum::File(file_writer_config) => {
                    if let WriterTypeEnum::File(ref path) = wtype {
                        &file_writer_config.path == path || path.to_string_lossy().is_empty()
                    } else {
                        false
                    }
                }
                WriterConfigEnum::Client(client_writer_config) => {
                    if let WriterTypeEnum::Client(ref address) = wtype {
                        if address.contains(':') {
                            vec![
                                &client_writer_config.address,
                                &client_writer_config.port.to_string(),
                            ] == address.split(':').collect::<Vec<_>>()
                        } else {
                            &client_writer_config.address == address || address.is_empty()
                        }
                    } else {
                        false
                    }
                }
                WriterConfigEnum::Server(server_config) => {
                    if let WriterTypeEnum::Server(ref address) = wtype {
                        if address.contains(':') {
                            vec![&server_config.address, &server_config.port.to_string()]
                                == address.split(':').collect::<Vec<_>>()
                        } else {
                            &server_config.address == address || address.is_empty()
                        }
                    } else {
                        false
                    }
                }
                WriterConfigEnum::Callback(_callback_writer_config) => {
                    wtype == WriterTypeEnum::Callback
                }
                WriterConfigEnum::Syslog(_syslog_writer_config) => wtype == WriterTypeEnum::Syslog,
            })
            .collect()
    }

    pub fn get_logging_config(&self) -> LoggingConfig {
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
            configs: self.get_writer_configs(),
            debug: self.debug,
        }
    }

    pub fn get_server_config(&self, wid: usize) -> Result<ServerConfig, LoggingError> {
        let writer = match self.writers.get(&wid) {
            Some(w) => w,
            None => {
                return Err(LoggingError::InvalidValue(format!(
                    "Writer {wid} does not exist"
                )));
            }
        };
        match writer {
            WriterEnum::Server(server) => Ok(server.config.lock().unwrap().get_server_config()),
            _ => Err(LoggingError::InvalidValue(format!(
                "Writer wid={wid} has invalid type {writer:?}"
            ))),
        }
    }

    pub fn get_server_configs(&self) -> HashMap<usize, ServerConfig> {
        self.writers
            .iter()
            .filter_map(|(k, w)| match w {
                WriterEnum::Server(c) => Some((*k, c.config.lock().unwrap().get_server_config())),
                _ => None,
            })
            .collect()
    }

    pub fn get_server_addresses(&self) -> HashMap<usize, String> {
        self.get_server_configs()
            .iter()
            .map(|(k, c)| (*k, c.address.clone()))
            .collect::<HashMap<usize, _>>()
    }

    pub fn get_server_ports(&self) -> HashMap<usize, u16> {
        self.get_server_configs()
            .iter()
            .map(|(k, c)| (*k, c.port))
            .collect::<HashMap<usize, _>>()
    }

    pub fn set_root_writer_config(
        &mut self,
        config: &WriterConfigEnum,
    ) -> Result<(), LoggingError> {
        let writer = WriterEnum::new(self, config)?;
        self.writers.insert(0, writer);
        Ok(())
    }

    pub fn set_root_writer(&mut self, writer: WriterEnum) {
        self.writers.insert(0, writer);
    }

    pub fn add_writer_config(&mut self, config: &WriterConfigEnum) -> Result<usize, LoggingError> {
        let writer = WriterEnum::new(self, config)?;
        Ok(self.add_writer(writer))
    }

    pub fn add_writer(&mut self, writer: WriterEnum) -> usize {
        self.writers.insert(self.wid, writer);
        self.wid += 1;
        self.wid
    }

    pub fn remove_writer(&mut self, wid: usize) -> Option<WriterEnum> {
        self.writers.remove(&wid)
    }

    pub fn add_writer_configs(
        &mut self,
        configs: &[WriterConfigEnum],
    ) -> Result<Vec<usize>, LoggingError> {
        let mut wids = Vec::new();
        for config in configs.into_iter() {
            let writer = WriterEnum::new(self, config)?;
            self.writers.insert(self.wid, writer);
            wids.push(self.wid);
            self.wid += 1;
        }
        Ok(wids)
    }

    pub fn add_writers(&mut self, writers: Vec<WriterEnum>) -> Vec<usize> {
        let mut wids = Vec::new();
        for writer in writers.into_iter() {
            self.writers.insert(self.wid, writer);
            wids.push(self.wid);
            self.wid += 1;
        }
        wids
    }

    pub fn remove_writers(&mut self, wids: Vec<usize>) -> Vec<WriterEnum> {
        wids.iter()
            .filter_map(|wid| self.writers.remove(wid))
            .collect::<Vec<_>>()
    }
}
