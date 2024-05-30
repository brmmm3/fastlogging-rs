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
    level2short, level2str, level2string, level2sym, LevelSyms, MessageStructEnum, SUCCESS, TRACE,
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
            LoggingTypeEnum::MessageExt((level, tname, tid, message)) => {
                (level, Some(tname), tid, message)
            }
            LoggingTypeEnum::Rotate => {
                if let Some(ref mut file) = config.lock().unwrap().file {
                    file.rotate()?;
                }
                continue;
            }
            LoggingTypeEnum::Sync(timeout) => {
                let mut config = config.lock().unwrap();
                if let Some(ref mut console) = config.console {
                    console.sync(timeout)?;
                }
                if let Some(ref mut file) = config.file {
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
        if let Some(ref mut file) = config.file {
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
        if let Some(ref mut writer) = config.file {
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
        let (config, level, tname, tid, tx, rx, stop) = config_file.init(
            level, domain, ext_config, console, file, server, connect, syslog,
        )?;
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

    pub fn set_level(&mut self, level: u8) {
        if let Ok(mut config) = self.config.lock() {
            config.level = level;
            self.level = level;
        }
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

    // Console logger

    pub fn set_console_writer(&mut self, config: Option<ConsoleWriterConfig>) -> Result<(), Error> {
        self.config.lock().unwrap().console = if let Some(config) = config {
            Some(ConsoleWriter::new(config, self.stop.clone())?)
        } else {
            None
        };
        Ok(())
    }

    pub fn set_console_colors(&mut self, colors: bool) {
        if let Some(ref mut console) = self.config.lock().unwrap().console {
            console.set_colors(colors);
        }
    }

    // File logger

    pub fn set_file_writer(&mut self, config: Option<FileWriterConfig>) -> Result<(), Error> {
        self.config.lock().unwrap().file = if let Some(config) = config {
            Some(FileWriter::new(config, self.stop.clone())?)
        } else {
            None
        };
        Ok(())
    }

    pub fn rotate(&self) -> Result<(), Error> {
        if let Some(ref file) = self.config.lock().unwrap().file {
            file.rotate()
        } else {
            Ok(())
        }
    }

    pub fn sync(&self, console: bool, file: bool, client: bool, timeout: f64) -> Result<(), Error> {
        let config: MutexGuard<LoggingConfig> = self.config.lock().unwrap();
        if console {
            if let Some(ref console) = config.console {
                console.sync(timeout)?;
            }
        }
        if file {
            if let Some(ref file) = config.file {
                file.sync(timeout)?;
            }
        }
        if client {
            for client in config.clients.values() {
                client.sync(timeout)?;
            }
        }
        Ok(())
    }

    pub fn sync_all(&self, timeout: f64) -> Result<(), Error> {
        self.sync(true, true, true, timeout)?;
        Ok(())
    }

    // Network client

    pub fn connect(&mut self, config: ClientWriterConfig) -> Result<(), Error> {
        if let Ok(mut logging_config) = self.config.lock() {
            let address: String = config.address.clone();
            let client: ClientWriter = ClientWriter::new(config, self.stop.clone())?;
            logging_config.clients.insert(address, client);
        }
        Ok(())
    }

    pub fn disconnect(&mut self, address: &str) -> Result<(), Error> {
        if let Ok(mut config) = self.config.lock() {
            if config.clients.remove(address).is_some() {
                return Ok(());
            }
            return Err(Error::new(
                ErrorKind::NotFound,
                "Client not found".to_string(),
            ));
        }
        Ok(())
    }

    pub fn set_client_level(&mut self, address: &str, level: u8) -> Result<(), Error> {
        if let Ok(mut config) = self.config.lock() {
            if let Some(client) = config.clients.get_mut(address) {
                client.set_level(level);
                return Ok(());
            }
            return Err(Error::new(
                ErrorKind::NotFound,
                "Client not found".to_string(),
            ));
        }
        Ok(())
    }

    pub fn set_client_encryption(
        &mut self,
        address: &str,
        key: EncryptionMethod,
    ) -> Result<(), Error> {
        if let Ok(mut config) = self.config.lock() {
            if let Some(client) = config.clients.get_mut(address) {
                client.set_encryption(key)?;
                return Ok(());
            }
            return Err(Error::new(
                ErrorKind::NotFound,
                "Client not found".to_string(),
            ));
        }
        Ok(())
    }

    // Network server

    pub fn server_start<S: Into<String>>(
        &mut self,
        address: S,
        level: u8,
        key: EncryptionMethod,
    ) -> Result<(), Error> {
        if let Ok(mut logging_config) = self.config.lock() {
            let config = ServerConfig::new(level, address.into(), key);
            logging_config.server = Some(LoggingServer::new(
                config,
                self.tx.clone(),
                self.stop.clone(),
            )?);
        }
        Ok(())
    }

    pub fn server_shutdown(&mut self) -> Result<(), Error> {
        if let Ok(mut config) = self.config.lock() {
            if let Some(mut server) = config.server.take() {
                server.shutdown()?;
            }
        }
        Ok(())
    }

    pub fn set_server_level(&mut self, level: u8) -> Result<(), Error> {
        if let Ok(mut config) = self.config.lock() {
            if let Some(ref mut server) = config.server {
                server.set_level(level);
                return Ok(());
            }
            return Err(Error::new(
                ErrorKind::NotFound,
                "Server not running".to_string(),
            ));
        }
        Ok(())
    }

    pub fn set_server_encryption(&mut self, key: EncryptionMethod) -> Result<(), Error> {
        if let Ok(mut config) = self.config.lock() {
            if let Some(ref mut server) = config.server {
                server.set_encryption(key)?;
                return Ok(());
            }
            return Err(Error::new(
                ErrorKind::NotFound,
                "Server not running".to_string(),
            ));
        }
        Ok(())
    }

    pub fn get_server_config(&self) -> Option<ServerConfig> {
        self.config.lock().unwrap().server.as_ref().map(|s| {
            let c = s.config.lock().unwrap();
            ServerConfig {
                level: c.level,
                address: c.address.clone(),
                port: c.port,
                key: c.key.clone(),
            }
        })
    }

    pub fn get_server_auth_key(&self) -> Vec<u8> {
        AUTH_KEY.to_vec()
    }

    // Config

    pub fn get_config(&self) -> String {
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
            c.file
                .as_ref()
                .map(|c| c.config.lock().unwrap().to_string()),
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
