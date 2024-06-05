use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};

use chrono::Local;
use flume::{Receiver, Sender};

use crate::config::{ConfigFile, ExtConfig, LoggingConfig};
use crate::console::{ConsoleWriter, ConsoleWriterConfig};
use crate::def::{LoggingTypeEnum, CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, WARNING};
use crate::file::{FileWriter, FileWriterConfig};
use crate::logger::Logger;
use crate::net::{
    ClientWriter, ClientWriterConfig, EncryptionMethod, LoggingServer, ServerConfig, AUTH_KEY,
};
use crate::{
    level2short, level2str, level2string, level2sym, LevelSyms, MessageStructEnum, RootConfig,
    SyslogWriter, WriterConfigEnum, WriterTypeEnum, SUCCESS, TRACE,
};

#[inline]
fn build_string_message(
    buffer: &mut String,
    config: &MutexGuard<LoggingConfig>,
    level: u8,
    tname: Option<String>,
    tid: u32,
    message: String,
) {
    buffer.push_str(&Local::now().format("%Y.%m.%d %H:%M:%S").to_string());
    if let Some(ref hostname) = config.hostname {
        buffer.push(' ');
        buffer.push_str(hostname);
    }
    if !config.pname.is_empty() {
        buffer.push(' ');
        buffer.push_str(&config.pname);
    }
    if config.pid > 0 {
        if config.pname.is_empty() {
            buffer.push(' ');
        }
        buffer.push('[');
        buffer.push_str(&config.pid.to_string());
        buffer.push(']');
    }
    if let Some(ref tname) = tname {
        buffer.push('>');
        buffer.push_str(tname);
    }
    if tid > 0 {
        if tname.is_none() {
            buffer.push('>');
        }
        buffer.push('[');
        buffer.push_str(&tid.to_string());
        buffer.push(']');
    }
    buffer.push(' ');
    buffer.push_str(&config.domain);
    buffer.push(':');
    buffer.push(' ');
    buffer.push_str(match config.level2sym {
        LevelSyms::Sym => level2sym(level),
        LevelSyms::Short => level2short(level),
        LevelSyms::Str => level2str(level),
    });
    buffer.push(' ');
    buffer.push_str(&message);
}

#[inline]
fn build_json_message(
    buffer: &mut String,
    config: &MutexGuard<LoggingConfig>,
    level: u8,
    tname: Option<String>,
    tid: u32,
    message: String,
) {
    buffer.push('{');
    buffer.push_str("\"date\":");
    buffer.push_str(&Local::now().format("\"%Y.%m.%d %H:%M:%S\"").to_string());
    if let Some(ref hostname) = config.hostname {
        buffer.push_str(",\"host\":\"");
        buffer.push_str(hostname);
        buffer.push('"');
    }
    if !config.pname.is_empty() {
        buffer.push_str(",\"pname\":\"");
        buffer.push_str(&config.pname);
        buffer.push('"');
    }
    if config.pid > 0 {
        buffer.push_str(",\"pid\":");
        buffer.push_str(&config.pid.to_string());
    }
    if let Some(ref tname) = tname {
        buffer.push_str(",\"tname\":\"");
        buffer.push_str(tname);
        buffer.push('"');
    }
    if tid > 0 {
        buffer.push_str(",\"tid\":");
        buffer.push_str(&tid.to_string());
    }
    buffer.push_str(",\"domain\":\"");
    buffer.push_str(&config.domain);
    buffer.push_str("\",\"level\":\"");
    buffer.push_str(match config.level2sym {
        LevelSyms::Sym => level2sym(level),
        LevelSyms::Short => level2short(level),
        LevelSyms::Str => level2str(level),
    });
    buffer.push_str("\",\"message\":\"");
    buffer.push_str(&message);
    buffer.push_str("\"}");
}

#[inline]
fn build_xml_message(
    buffer: &mut String,
    config: &MutexGuard<LoggingConfig>,
    level: u8,
    tname: Option<String>,
    tid: u32,
    message: String,
) {
    buffer.push_str("<log>");
    buffer.push_str("<date>");
    buffer.push_str(&Local::now().format("%Y.%m.%d %H:%M:%S").to_string());
    buffer.push_str("</date>");
    if let Some(ref hostname) = config.hostname {
        buffer.push_str("<host>");
        buffer.push_str(hostname);
        buffer.push_str("</host>");
    }
    if !config.pname.is_empty() {
        buffer.push_str("<pname>");
        buffer.push_str(&config.pname);
        buffer.push_str("</pname>");
    }
    if config.pid > 0 {
        buffer.push_str("<pid>");
        buffer.push_str(&config.pid.to_string());
        buffer.push_str("</pid>");
    }
    if let Some(ref tname) = tname {
        buffer.push_str("<tname>");
        buffer.push_str(tname);
        buffer.push_str("</tname>");
    }
    if tid > 0 {
        buffer.push_str("<tid>");
        buffer.push_str(&tid.to_string());
        buffer.push_str("</tid>");
    }
    buffer.push_str("<domain>");
    buffer.push_str(&config.domain);
    buffer.push_str("</domain><level>");
    buffer.push_str(match config.level2sym {
        LevelSyms::Sym => level2sym(level),
        LevelSyms::Short => level2short(level),
        LevelSyms::Str => level2str(level),
    });
    buffer.push_str("</level><message>");
    buffer.push_str(&message);
    buffer.push_str("</message></log>");
}

fn logging_thread_worker(
    rx: Receiver<LoggingTypeEnum>,
    config: Arc<Mutex<LoggingConfig>>,
    stop: Arc<Mutex<bool>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::with_capacity(4096);
    loop {
        if *stop.lock().unwrap() {
            break;
        }
        let mut remote = false;
        let (level, tname, tid, message) = match rx.recv()? {
            LoggingTypeEnum::Message((level, message)) => (level, None, 0, message),
            LoggingTypeEnum::MessageRemote((level, message)) => {
                remote = true;
                (level, None, 0, message)
            }
            LoggingTypeEnum::MessageExt((level, message, tid, tname)) => {
                (level, Some(tname), tid, message)
            }
            LoggingTypeEnum::Rotate => {
                for file in config.lock().unwrap().files.values() {
                    file.rotate()?;
                }
                continue;
            }
            LoggingTypeEnum::Sync(timeout) => {
                let mut config = config.lock().unwrap();
                if let Some(ref mut console) = config.console {
                    console.sync(timeout)?;
                }
                for file in config.files.values() {
                    file.sync(timeout)?;
                }
                for client in config.clients.values() {
                    client.sync(timeout)?;
                }
                continue;
            }
            LoggingTypeEnum::Stop => {
                break;
            }
        };
        // Build message
        // {date} {hostname} {pname}[{pid}]>{tname}[{tid}] {domain}: {level} {message}
        let mut config = config.lock().unwrap();
        buffer.clear();
        if remote {
            buffer.push_str(&message);
        } else {
            match config.structured {
                MessageStructEnum::String => {
                    build_string_message(&mut buffer, &config, level, tname, tid, message);
                }
                MessageStructEnum::Json => {
                    build_json_message(&mut buffer, &config, level, tname, tid, message);
                }
                MessageStructEnum::Xml => {
                    build_xml_message(&mut buffer, &config, level, tname, tid, message);
                }
            }
        }
        // Send message to writers
        if let Some(ref mut console) = config.console {
            console.send(level, buffer.clone())?;
        }
        for file in config.files.values() {
            file.send(level, buffer.clone())?;
        }
        for client in config.clients.values() {
            client.send(level, buffer.clone())?;
        }
        if let Some(ref mut syslog) = config.syslog {
            syslog.send(level, buffer.clone())?;
        }
    }
    Ok(())
}

fn logging_thread(
    rx: Receiver<LoggingTypeEnum>,
    config: Arc<Mutex<LoggingConfig>>,
    stop: Arc<Mutex<bool>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut some_err = None;
    if let Err(err) = logging_thread_worker(rx, config.clone(), stop) {
        eprintln!("Logging broker thread crashed with error: {err:?}");
        some_err = Some(err);
    }
    if let Ok(mut config) = config.lock() {
        for (address, writer) in config.clients.iter_mut() {
            if let Err(err) = writer.shutdown() {
                eprintln!("Failed to stop client {address}: {err:?}");
            }
        }
        if let Some(ref mut writer) = config.server {
            if let Err(err) = writer.shutdown() {
                eprintln!("Failed to stop logging server: {err:?}");
            }
        }
        if let Some(ref mut writer) = config.console {
            if let Err(err) = writer.shutdown() {
                eprintln!("Failed to stop console logger: {err:?}");
            }
        }
        for writer in config.files.values_mut() {
            if let Err(err) = writer.shutdown() {
                eprintln!("Failed to stop file logger: {err:?}");
            }
        }
        if let Some(ref mut writer) = config.syslog {
            if let Err(err) = writer.shutdown() {
                eprintln!("Failed to stop syslog logger: {err:?}");
            }
        }
    }
    if let Some(err) = some_err {
        Err(err)
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Logging {
    pub level: u8,
    config_file: ConfigFile,
    config: Arc<Mutex<LoggingConfig>>,
    tname: bool,
    tid: bool,
    pub tx: Sender<LoggingTypeEnum>,
    stop: Arc<Mutex<bool>>,
    thr: Option<JoinHandle<()>>,
}

impl Logging {
    pub fn new(
        level: Option<u8>, // Global log level
        domain: Option<String>,
        ext_config: Option<ExtConfig>, // Extended logging configuration
        console: Option<ConsoleWriterConfig>, // If config is defined start ConsoleLogging
        file: Option<FileWriterConfig>, // If config is defined start FileLogging
        server: Option<ServerConfig>,  // If config is defined start LoggingServer
        connect: Option<ClientWriterConfig>, // If config is defined start ClientLogging
        syslog: Option<u8>,            // If log level is defined start SyslogLogging
        config: Option<PathBuf>,       // Optional configuration file
    ) -> Result<Self, Error> {
        // Initialize config from optional config file.
        let mut config_file = ConfigFile::new(config)?;
        // Overwrite settings with arguments, if provided.
        let (config, tx, rx, stop) = config_file.init(
            level, domain, ext_config, console, file, server, connect, syslog,
        )?;
        let level = config.level;
        let tname = config.tname;
        let tid = config.tid;
        let config = Arc::new(Mutex::new(config));
        Ok(Self {
            level,
            config: config.clone(),
            config_file,
            tname,
            tid,
            tx,
            stop: stop.clone(),
            thr: Some(
                thread::Builder::new()
                    .name("FileLogging".to_string())
                    .spawn(move || {
                        if let Err(err) = logging_thread(rx, config, stop) {
                            eprintln!("logging_thread returned with error: {err:?}");
                        }
                    })?,
            ),
        })
    }

    pub fn shutdown(&mut self, now: bool) -> Result<(), Error> {
        if self.thr.is_none() {
            return Ok(());
        }
        if now {
            *self.stop.lock().unwrap() = true;
        }
        if let Err(err) = self.tx.send(LoggingTypeEnum::Stop) {
            eprintln!("Failed to send STOP signal to broker thread: {err:?}");
        }
        if let Some(thr) = self.thr.take() {
            thr.join().map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    e.downcast_ref::<&str>().unwrap().to_string(),
                )
            })
        } else {
            Ok(())
        }
    }

    pub fn add_logger(&mut self, logger: &'_ mut Logger) {
        logger.set_tx(Some(self.tx.clone()));
    }

    pub fn remove_logger(&mut self, logger: &'_ mut Logger) {
        logger.set_tx(None);
    }

    pub fn set_level(&mut self, writer: WriterTypeEnum, level: u8) -> Result<(), Error> {
        let mut config = self.config.lock().unwrap();
        match writer {
            WriterTypeEnum::Root => {
                config.level = level;
                self.level = level;
            }
            WriterTypeEnum::Console => {
                if let Some(ref writer) = config.console {
                    writer.set_level(level);
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Console writer not configured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::File(path) => {
                if let Some(writer) = config.files.get_mut(&path) {
                    writer.set_level(level);
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "File writer not configured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::Client(address) => {
                if let Some(writer) = config.clients.get_mut(&address) {
                    writer.set_level(level);
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Client writer not configured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::Server => {
                if let Some(ref mut writer) = config.server {
                    writer.set_level(level);
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Server not running".to_string(),
                    ));
                }
            }
            WriterTypeEnum::Syslog => {
                if let Some(ref writer) = config.syslog {
                    writer.set_level(level);
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Syslog writer not configured".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn set_domain(&mut self, domain: String) {
        if let Ok(mut config) = self.config.lock() {
            config.domain = domain;
        }
    }

    pub fn set_level2sym(&mut self, level2sym: LevelSyms) {
        if let Ok(mut config) = self.config.lock() {
            config.level2sym = level2sym;
        }
    }

    pub fn set_ext_config(&mut self, ext_config: ExtConfig) {
        if let Ok(mut config) = self.config.lock() {
            config.set_ext_config(ext_config);
            self.tname = config.tname;
            self.tid = config.tid;
        }
    }

    pub fn add_writer(&mut self, writer: WriterConfigEnum) -> Result<(), Error> {
        let mut config = self.config.lock().unwrap();
        match writer {
            WriterConfigEnum::Root(_) => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "Root logger can't be added".to_string(),
                ));
            }
            WriterConfigEnum::Console(cfg) => {
                config.console = Some(ConsoleWriter::new(cfg, self.stop.clone())?);
            }
            WriterConfigEnum::File(cfg) => {
                config
                    .files
                    .insert(cfg.path.clone(), FileWriter::new(cfg, self.stop.clone())?);
            }
            WriterConfigEnum::Client(cfg) => {
                let address: String = cfg.address.clone();
                let client: ClientWriter = ClientWriter::new(cfg, self.stop.clone())?;
                config.clients.insert(address, client);
            }
            WriterConfigEnum::Server(cfg) => {
                config.server = Some(LoggingServer::new(cfg, self.tx.clone(), self.stop.clone())?);
            }
            WriterConfigEnum::Syslog(cfg) => {
                config.syslog = Some(SyslogWriter::new(cfg, self.stop.clone())?);
            }
        }
        Ok(())
    }

    pub fn remove_writer(&mut self, writer: WriterTypeEnum) -> Result<(), Error> {
        let mut config = self.config.lock().unwrap();
        match writer {
            WriterTypeEnum::Root => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "Root logger can't be removed".to_string(),
                ));
            }
            WriterTypeEnum::Console => {
                if let Some(mut writer) = config.console.take() {
                    writer.shutdown()?;
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Console writer not configured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::File(path) => {
                if let Some(mut writer) = config.files.remove(&path) {
                    writer.shutdown()?;
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "File writer not configured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::Client(address) => {
                if let Some(mut writer) = config.clients.remove(&address) {
                    writer.shutdown()?;
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Client writer not configured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::Server => {
                if let Some(mut server) = config.server.take() {
                    server.shutdown()?;
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Server not conigured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::Syslog => {
                if let Some(mut server) = config.syslog.take() {
                    server.shutdown()?;
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Syslog writer not conigured".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn sync(
        &self,
        console: bool,
        file: bool,
        client: bool,
        syslog: bool,
        timeout: f64,
    ) -> Result<(), Error> {
        let config: MutexGuard<LoggingConfig> = self.config.lock().unwrap();
        if console {
            if let Some(ref writer) = config.console {
                writer.sync(timeout)?;
            }
        }
        if file {
            for writer in config.files.values() {
                writer.sync(timeout)?;
            }
        }
        if client {
            for writer in config.clients.values() {
                writer.sync(timeout)?;
            }
        }
        if syslog {
            if let Some(ref writer) = config.syslog {
                writer.sync(timeout)?;
            }
        }
        Ok(())
    }

    pub fn sync_all(&self, timeout: f64) -> Result<(), Error> {
        self.sync(true, true, true, true, timeout)?;
        Ok(())
    }

    // File logger

    pub fn rotate(&self, path: Option<PathBuf>) -> Result<(), Error> {
        for (key, file) in self.config.lock().unwrap().files.iter() {
            if path.is_none() || path.as_ref().unwrap() == key {
                file.rotate()?;
            }
        }
        Ok(())
    }

    // Network

    pub fn set_encryption(
        &mut self,
        writer: WriterTypeEnum,
        key: EncryptionMethod,
    ) -> Result<(), Error> {
        let mut config = self.config.lock().unwrap();
        match writer {
            WriterTypeEnum::File(path) => {
                if let Some(mut writer) = config.files.remove(&path) {
                    writer.shutdown()?;
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "File writer not configured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::Client(address) => {
                if let Some(mut writer) = config.clients.remove(&address) {
                    writer.set_encryption(key)?;
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Client writer not configured".to_string(),
                    ));
                }
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "Unable to configure encryption for this writer type".to_string(),
                ));
            }
        }
        Ok(())
    }

    // Config

    pub fn get_config(&self, writer: WriterTypeEnum) -> Result<WriterConfigEnum, Error> {
        let mut config = self.config.lock().unwrap();
        Ok(match writer {
            WriterTypeEnum::Root => WriterConfigEnum::Root(RootConfig {
                level: self.level,
                domain: config.domain.clone(),
                hostname: config.hostname.clone(),
                pname: config.pname.clone(),
                pid: config.pid,
                tname: config.tname,
                tid: config.tid,
                structured: config.structured.clone(),
                level2sym: config.level2sym.clone(),
            }),
            WriterTypeEnum::Console => {
                if let Some(ref writer) = config.console {
                    WriterConfigEnum::Console(writer.config.lock().unwrap().clone())
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Console writer not configured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::File(path) => {
                if let Some(writer) = config.files.get_mut(&path) {
                    WriterConfigEnum::File(writer.config.lock().unwrap().clone())
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "File writer not configured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::Client(address) => {
                if let Some(writer) = config.clients.get_mut(&address) {
                    let cfg = writer.config.lock().unwrap();
                    WriterConfigEnum::Client(ClientWriterConfig::new(
                        cfg.level,
                        format!("{}:{}", cfg.address, cfg.port),
                        cfg.key.clone(),
                    ))
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Client writer not configured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::Server => {
                if let Some(ref mut writer) = config.server {
                    let cfg = writer.config.lock().unwrap();
                    WriterConfigEnum::Server(ServerConfig::new(
                        cfg.level,
                        format!("{}:{}", cfg.address, cfg.port),
                        cfg.key.clone(),
                    ))
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Server not running".to_string(),
                    ));
                }
            }
            WriterTypeEnum::Syslog => {
                if let Some(ref writer) = config.syslog {
                    WriterConfigEnum::Syslog(writer.config.lock().unwrap().clone())
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "Syslog writer not configured".to_string(),
                    ));
                }
            }
        })
    }

    pub fn get_server_config(&self) -> Option<ServerConfig> {
        if let Ok(WriterConfigEnum::Server(config)) = self.get_config(WriterTypeEnum::Server) {
            Some(config)
        } else {
            None
        }
    }

    pub fn get_server_auth_key(&self) -> Vec<u8> {
        AUTH_KEY.to_vec()
    }

    pub fn get_config_string(&self) -> String {
        let c = self.config.lock().unwrap();
        format!(
            "level={:?}\n\
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
            level2string(&c.level2sym, c.level),
            c.domain,
            c.hostname,
            c.pname,
            c.pid,
            c.tname,
            c.tid,
            c.structured,
            c.console
                .as_ref()
                .map(|c| c.config.lock().unwrap().to_string()),
            c.files
                .iter()
                .map(|(path, c)| format!("{path:?}: {}", c.config.lock().unwrap())),
            c.syslog
                .as_ref()
                .map(|c| c.config.lock().unwrap().to_string()),
            c.server
                .as_ref()
                .map(|c| c.config.lock().unwrap().to_string()),
            c.clients
                .iter()
                .map(|(ip, c)| format!("{ip}: {}", c.config.lock().unwrap()))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    pub fn save_config(&self, path: &Path) -> Result<(), Error> {
        self.config_file.save(path)
    }

    // Logging methods

    #[inline]
    fn log<S: Into<String>>(&self, level: u8, message: S) -> Result<(), Error> {
        (if self.tname || self.tid {
            let tname = if self.tname {
                thread::current().name().unwrap_or_default().to_string()
            } else {
                "".to_string()
            };
            let tid = if self.tid { thread_id::get() as u32 } else { 0 };
            self.tx.send(LoggingTypeEnum::MessageExt((
                level,
                message.into(),
                tid,
                tname,
            )))
        } else {
            self.tx
                .send(LoggingTypeEnum::Message((level, message.into())))
        })
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

    pub fn trace<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= TRACE {
            self.log(TRACE, message)?;
        }
        Ok(())
    }

    pub fn debug<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= DEBUG {
            self.log(DEBUG, message)?;
        }
        Ok(())
    }

    pub fn info<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= INFO {
            self.log(INFO, message)?;
        }
        Ok(())
    }

    pub fn success<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= SUCCESS {
            self.log(SUCCESS, message)?;
        }
        Ok(())
    }

    pub fn warning<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= WARNING {
            self.log(WARNING, message)?;
        }
        Ok(())
    }

    pub fn error<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= ERROR {
            self.log(ERROR, message)?;
        }
        Ok(())
    }

    pub fn critical<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= CRITICAL {
            self.log(CRITICAL, message)?;
        }
        Ok(())
    }

    pub fn fatal<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= FATAL {
            self.log(FATAL, message)?;
        }
        Ok(())
    }

    pub fn exception<S: Into<String>>(&self, message: S) -> Result<(), Error> {
        if self.level <= EXCEPTION {
            self.log(EXCEPTION, message)?;
        }
        Ok(())
    }

    pub fn __repr__(&self) -> String {
        if let Ok(config) = self.config.lock() {
            format!("Logging(level={} domain={})", self.level, config.domain)
        } else {
            format!("Logging(level={})", self.level)
        }
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl Default for Logging {
    fn default() -> Self {
        Self::new(None, None, None, None, None, None, None, None, None).unwrap()
    }
}
