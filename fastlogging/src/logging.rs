use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{env, fs};

use chrono::Local;
use flume::{bounded, Receiver, Sender};
use once_cell::sync::Lazy;

use crate::config::{default_config_file, ConfigFile, ExtConfig, LoggingInstance};
use crate::console::{ConsoleWriter, ConsoleWriterConfig};
use crate::def::{LoggingTypeEnum, CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, WARNING};
use crate::file::{FileWriter, FileWriterConfig};
use crate::logger::Logger;
use crate::net::{
    ClientWriter, ClientWriterConfig, EncryptionMethod, LoggingServer, ServerConfig, AUTH_KEY,
};
use crate::{
    getppid, level2short, level2str, level2string, level2sym, LevelSyms, MessageStructEnum,
    RootConfig, SyslogWriter, WriterConfigEnum, WriterTypeEnum, NOTSET, SUCCESS, TRACE,
};

pub static ROOT_LOGGER: Lazy<Mutex<Logging>> = Lazy::new(|| {
    fn create_default_logger(config_file: Option<PathBuf>) -> Logging {
        //println!("create_default_logger with config_file={config_file:?}");
        let server = ServerConfig::new(
            NOTSET,
            "127.0.0.1",
            EncryptionMethod::AuthKey(AUTH_KEY.to_vec()),
        );
        let mut logging = Logging::new(
            None,
            None,
            None,
            None,
            None,
            Some(server),
            None,
            None,
            config_file,
        )
        .unwrap();
        logging.drop = false;
        logging
    }

    fn get_port_file(pid: u32) -> PathBuf {
        //println!("get_port_file for pid {pid}");
        let mut temp_dir = env::temp_dir();
        temp_dir.push(format!("fastlogging_rs_server_port.{pid}"));
        temp_dir
    }

    fn get_parent_server_address() -> Result<Option<(String, EncryptionMethod)>, Error> {
        /*println!("get_parent_server_address");
        println!(
            "## {} {}",
            process::id(),
            std::os::unix::process::parent_id()
        );*/
        let port_file = get_port_file(getppid());
        //println!("CHECK port_file={port_file:?} {}", port_file.exists());
        if port_file.exists() {
            // Parent process exists. Check if logging server is reachable.
            let mut buffer = Vec::new();
            if fs::File::open(port_file)?.read_to_end(&mut buffer)? >= 4 {
                let port = u16::from_le_bytes(buffer[..2].try_into().unwrap());
                let address = format!("127.0.0.1:{port}");
                //println!("Connecting to address={address}");
                let encryption = match buffer[2] {
                    0 => EncryptionMethod::NONE,
                    1 => EncryptionMethod::AuthKey(buffer[3..].to_vec()),
                    2 => EncryptionMethod::AES(buffer[3..].to_vec()),
                    _ => {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Invalid encryption type {}", buffer[2]),
                        ))
                    }
                };
                if TcpStream::connect(&address).is_ok() {
                    //println!("CONNECTED to {address}");
                    return Ok(Some((address, encryption)));
                }
            }
        }
        Ok(None)
    }

    fn setup_logging() -> Result<Logging, Error> {
        //println!("---setup_logging");
        // Check if parent process with fastlogging instance exists.
        let mut logging = create_default_logger(None);
        if let Some(server_port) = logging.get_server_ports().get(0) {
            let port_file = get_port_file(process::id());
            //println!("Create port_file {port_file:?} for port {server_port}");
            //println!("SERVER_AUTH_KEY {:?}", logging.get_server_auth_key());
            let mut file = fs::File::create(port_file)?;
            file.write_all(&u16::to_le_bytes(*server_port))?;
            file.write_all(&logging.get_server_auth_key().to_bytes())?;
        }
        if let Some((server_address, encryption)) = get_parent_server_address()? {
            // Connect to parent server port
            let client = ClientWriterConfig::new(NOTSET, server_address, encryption);
            //println!("ADD_WRITER {client:?}");
            logging.add_writer(&WriterConfigEnum::Client(client))?;
        } else {
            //println!("ROOT");
            // If default config file exists, then use this configuration. Else create default console logger.
            let config_file = default_config_file();
            //println!("config_file={config_file:?}");
            if config_file.1.is_empty() {
                //println!("add_writer CONSOLE");
                logging.add_writer(&WriterConfigEnum::Console(ConsoleWriterConfig::new(
                    NOTSET, false,
                )))?;
            } else {
                logging.apply_config(&config_file.0)?;
            }
        }
        Ok(logging)
    }

    //println!("Setup ROOT_LOGGER");
    let logging = match setup_logging() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to setup default logger: {e}");
            create_default_logger(None)
        }
    };
    //println!("logging={logging:?}");
    Mutex::new(logging)
});

#[inline]
fn build_string_message(
    buffer: &mut String,
    config: &MutexGuard<LoggingInstance>,
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
    config: &MutexGuard<LoggingInstance>,
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
    config: &MutexGuard<LoggingInstance>,
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
    sync_tx: Sender<u8>,
    config: Arc<Mutex<LoggingInstance>>,
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
                let debug = config.lock().unwrap().debug;
                for file in config.lock().unwrap().files.values() {
                    if debug > 0 {
                        println!(
                            "logging_thread_worker: ROTATE {:?}",
                            file.config.lock().unwrap().path
                        );
                    }
                    file.rotate()?;
                }
                continue;
            }
            LoggingTypeEnum::Sync((console, file, client, syslog, timeout)) => {
                let mut config = config.lock().unwrap();
                let debug = config.debug;
                if debug > 0 {
                    println!("logging_thread_worker: SYNC");
                }
                if console {
                    if let Some(ref mut console) = config.console {
                        if debug > 0 {
                            println!("logging_thread_worker: SYNC->CONSOLE");
                        }
                        console.sync(timeout)?;
                    }
                }
                if file {
                    for file in config.files.values() {
                        if debug > 0 {
                            println!(
                                "logging_thread_worker: SYNC->FILE {:?}",
                                file.config.lock().unwrap().path
                            );
                        }
                        file.sync(timeout)?;
                    }
                }
                if client {
                    for client in config.clients.values() {
                        if debug > 0 {
                            let client_config = client.config.lock().unwrap();
                            println!(
                                "logging_thread_worker: SYNC->CLIENT {}:{}",
                                client_config.address, client_config.port
                            );
                        }
                        client.sync(timeout)?;
                    }
                }
                if syslog {
                    if let Some(ref mut syslog) = config.syslog {
                        if debug > 0 {
                            println!("logging_thread_worker: SYNC->SYSLOG");
                        }
                        syslog.sync(timeout)?;
                    }
                }
                sync_tx.send(1)?;
                continue;
            }
            LoggingTypeEnum::Stop => {
                if config.lock().unwrap().debug > 0 {
                    println!("logging_thread_worker: STOP");
                }
                break;
            }
        };
        // Build message
        // {date} {hostname} {pname}[{pid}]>{tname}[{tid}] {domain}: {level} {message}
        let config = config.lock().unwrap();
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
        if config.debug > 2 {
            println!("logging_thread_worker: MESSAGE {buffer:?}");
        }
        if let Some(ref writer) = config.console {
            if writer.config.lock().unwrap().level <= level {
                writer.send(level, buffer.clone())?;
            }
        }
        for writer in config.files.values() {
            if writer.config.lock().unwrap().level <= level {
                writer.send(level, buffer.clone())?;
            }
        }
        for writer in config.clients.values() {
            if writer.config.lock().unwrap().level <= level {
                writer.send(level, buffer.clone())?;
            }
        }
        if let Some(ref writer) = config.syslog {
            if writer.config.lock().unwrap().level <= level {
                writer.send(level, buffer.clone())?;
            }
        }
    }
    Ok(())
}

fn logging_thread(
    rx: Receiver<LoggingTypeEnum>,
    sync_tx: Sender<u8>,
    config: Arc<Mutex<LoggingInstance>>,
    stop: Arc<Mutex<bool>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut some_err = None;
    if let Err(err) = logging_thread_worker(rx, sync_tx, config.clone(), stop) {
        eprintln!("Logging broker thread crashed with error: {err:?}");
        some_err = Some(err);
    }
    if let Ok(mut config) = config.lock() {
        for (address, writer) in config.clients.iter_mut() {
            if let Err(err) = writer.shutdown() {
                eprintln!("Failed to stop client writer {address}: {err:?}");
            }
        }
        for (address, writer) in config.servers.iter_mut() {
            if let Err(err) = writer.shutdown() {
                eprintln!("Failed to stop logging server {address}: {err:?}");
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

#[repr(C)]
#[derive(Debug)]
pub struct Logging {
    pub level: u8,
    config_file: ConfigFile,
    pub instance: Arc<Mutex<LoggingInstance>>,
    tname: bool,
    tid: bool,
    pub tx: Sender<LoggingTypeEnum>,
    sync_rx: Receiver<u8>,
    stop: Arc<Mutex<bool>>,
    thr: Option<JoinHandle<()>>,
    pub drop: bool,
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
        let (sync_tx, sync_rx) = bounded(1);
        let config = Arc::new(Mutex::new(config));
        Ok(Self {
            level,
            instance: config.clone(),
            config_file,
            tname,
            tid,
            tx,
            sync_rx,
            stop: stop.clone(),
            thr: Some(
                thread::Builder::new()
                    .name("LoggingThread".to_string())
                    .spawn(move || {
                        if let Err(err) = logging_thread(rx, sync_tx, config, stop) {
                            eprintln!("logging_thread returned with error: {err:?}");
                        }
                    })?,
            ),
            drop: true,
        })
    }

    pub fn init() -> Result<Self, Error> {
        let console_writer = ConsoleWriterConfig::new(NOTSET, false);
        Logging::new(
            None,
            None,
            None,
            Some(console_writer),
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn apply_config(&mut self, path: &Path) -> Result<(), Error> {
        self.config_file = ConfigFile::new(Some(path.to_path_buf()))?;
        let config = &self.config_file.config;
        let mut instance = self.instance.lock().unwrap();
        instance.level = config.level;
        instance.domain = config.domain.clone();
        // Console writer
        if let Some(ref writer_config) = config.console {
            instance.console = Some(ConsoleWriter::new(
                writer_config.clone(),
                self.stop.clone(),
            )?);
        }
        // File writer
        if let Some(ref writer_config) = config.file {
            instance.files.insert(
                writer_config.path.clone(),
                FileWriter::new(writer_config.clone(), self.stop.clone())?,
            );
        }
        // Network writer
        if let Some(ref writer_config) = config.connect {
            instance.clients.insert(
                writer_config.address.clone(),
                ClientWriter::new(writer_config.clone(), self.stop.clone())?,
            );
        }
        // Logging server
        if let Some(ref writer_config) = config.server {
            instance.servers.insert(
                writer_config.address.clone(),
                LoggingServer::new(writer_config.clone(), self.tx.clone(), self.stop.clone())?,
            );
        }
        // Syslog
        if let Some(ref writer_config) = config.syslog {
            instance.syslog = Some(SyslogWriter::new(writer_config.clone(), self.stop.clone())?);
        }
        Ok(())
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

    pub fn set_level(&mut self, writer: &WriterTypeEnum, level: u8) -> Result<(), Error> {
        let mut config = self.instance.lock().unwrap();
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
                if let Some(writer) = config.files.get_mut(path) {
                    writer.set_level(level);
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "File writer not configured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::Client(address) => {
                if let Some(writer) = config.clients.get_mut(address) {
                    writer.set_level(level);
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("Client {address} not configured"),
                    ));
                }
            }
            WriterTypeEnum::Server(address) => {
                if let Some(writer) = config.servers.get_mut(address) {
                    writer.set_level(level);
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("Server {address} not running"),
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

    pub fn set_domain(&mut self, domain: &str) {
        if let Ok(mut config) = self.instance.lock() {
            config.domain = domain.to_string();
        }
    }

    pub fn set_level2sym(&mut self, level2sym: &LevelSyms) {
        if let Ok(mut config) = self.instance.lock() {
            config.level2sym = level2sym.to_owned();
        }
    }

    pub fn set_ext_config(&mut self, ext_config: &ExtConfig) {
        if let Ok(mut config) = self.instance.lock() {
            config.set_ext_config(ext_config.to_owned());
            self.tname = config.tname;
            self.tid = config.tid;
        }
    }

    pub fn add_logger(&mut self, logger: &'_ mut Logger) {
        logger.set_tx(Some(self.tx.clone()));
    }

    pub fn remove_logger(&mut self, logger: &'_ mut Logger) {
        logger.set_tx(None);
    }

    pub fn add_writer(&mut self, writer: &WriterConfigEnum) -> Result<WriterTypeEnum, Error> {
        let mut config = self.instance.lock().unwrap();
        Ok(match writer {
            WriterConfigEnum::Root(_) => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "Root logger can't be added".to_string(),
                ));
            }
            WriterConfigEnum::Console(cfg) => {
                config.console = Some(ConsoleWriter::new(cfg.to_owned(), self.stop.clone())?);
                WriterTypeEnum::Console
            }
            WriterConfigEnum::File(cfg) => {
                config.files.insert(
                    cfg.path.clone(),
                    FileWriter::new(cfg.to_owned(), self.stop.clone())?,
                );
                WriterTypeEnum::File(cfg.path.clone())
            }
            WriterConfigEnum::Client(cfg) => {
                let address: String = cfg.address.clone();
                let client = ClientWriter::new(cfg.to_owned(), self.stop.clone())?;
                config.clients.insert(address.clone(), client);
                WriterTypeEnum::Client(address)
            }
            WriterConfigEnum::Server(cfg) => {
                let address: String = cfg.address.clone();
                let server =
                    LoggingServer::new(cfg.to_owned(), self.tx.clone(), self.stop.clone())?;
                config.servers.insert(address.clone(), server);
                WriterTypeEnum::Server(address)
            }
            WriterConfigEnum::Syslog(cfg) => {
                config.syslog = Some(SyslogWriter::new(cfg.to_owned(), self.stop.clone())?);
                WriterTypeEnum::Syslog
            }
        })
    }

    pub fn remove_writer(&mut self, writer: &WriterTypeEnum) -> Result<(), Error> {
        let mut config = self.instance.lock().unwrap();
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
                if let Some(mut writer) = config.files.remove(path) {
                    writer.shutdown()?;
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "File writer not configured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::Client(address) => {
                if let Some(mut writer) = config.clients.remove(address) {
                    writer.shutdown()?;
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("Client {address} not configured"),
                    ));
                }
            }
            WriterTypeEnum::Server(address) => {
                if let Some(mut server) = config.servers.remove(address) {
                    server.shutdown()?;
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("Server {address} not conigured"),
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
        self.tx
            .send(LoggingTypeEnum::Sync((
                console, file, client, syslog, timeout,
            )))
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        self.sync_rx
            .recv_timeout(Duration::from_secs_f64(timeout))
            .map_err(|e| Error::new(ErrorKind::BrokenPipe, e.to_string()))?;
        Ok(())
    }

    pub fn sync_all(&self, timeout: f64) -> Result<(), Error> {
        self.sync(true, true, true, true, timeout)?;
        Ok(())
    }

    // File logger

    pub fn rotate(&self, path: Option<PathBuf>) -> Result<(), Error> {
        for (key, file) in self.instance.lock().unwrap().files.iter() {
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
        let mut config = self.instance.lock().unwrap();
        match writer {
            WriterTypeEnum::Server(address) => {
                if let Some(writer) = config.servers.get_mut(&address) {
                    writer.set_encryption(key)?;
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("Server {address} not configured"),
                    ));
                }
            }
            WriterTypeEnum::Client(address) => {
                if let Some(writer) = config.clients.get_mut(&address) {
                    writer.set_encryption(key)?;
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("Client {address} not configured"),
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

    pub fn set_debug(&mut self, debug: u8) {
        let mut config = self.instance.lock().unwrap();
        config.debug = debug;
        for server in config.servers.values_mut() {
            server.config.lock().unwrap().debug = debug;
        }
    }

    pub fn get_config(&self, writer: &WriterTypeEnum) -> Result<WriterConfigEnum, Error> {
        let mut config = self.instance.lock().unwrap();
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
                if let Some(writer) = config.files.get_mut(path) {
                    WriterConfigEnum::File(writer.config.lock().unwrap().clone())
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "File writer not configured".to_string(),
                    ));
                }
            }
            WriterTypeEnum::Client(address) => {
                if let Some(writer) = config.clients.get_mut(address) {
                    WriterConfigEnum::Client(writer.config.lock().unwrap().get_client_config())
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("Client {address} not configured"),
                    ));
                }
            }
            WriterTypeEnum::Server(address) => {
                if let Some(writer) = config.servers.get_mut(address) {
                    WriterConfigEnum::Server(writer.config.lock().unwrap().get_server_config())
                } else {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        format!("Server {address} not running"),
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

    pub fn get_server_config(&self, address: &str) -> Option<ServerConfig> {
        if let Ok(WriterConfigEnum::Server(config)) =
            self.get_config(&WriterTypeEnum::Server(address.to_owned()))
        {
            Some(config)
        } else {
            None
        }
    }

    pub fn get_server_configs(&self) -> Vec<ServerConfig> {
        let config = self.instance.lock().unwrap();
        config
            .servers
            .values()
            .map(|v| v.config.lock().unwrap().get_server_config())
            .collect()
    }

    pub fn get_server_addresses(&self) -> Vec<String> {
        let config = self.instance.lock().unwrap();
        let addresses = config.servers.keys().map(|k| k.to_owned()).collect();
        //println!("get_server_addresses: addresses={addresses:?}");
        addresses
    }

    pub fn get_server_ports(&self) -> Vec<u16> {
        let config = self.instance.lock().unwrap();
        let ports = config
            .servers
            .values()
            .map(|v| v.config.lock().unwrap().port)
            .collect();
        //println!("get_server_ports: ports={ports:?}");
        ports
    }

    pub fn get_server_auth_key(&self) -> EncryptionMethod {
        let key = EncryptionMethod::AuthKey(AUTH_KEY.to_vec());
        //println!("get_server_auth_key: key={key:?}");
        key
    }

    pub fn get_config_string(&self) -> String {
        let c = self.instance.lock().unwrap();
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
            servers={:?}\n\
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
            c.servers
                .iter()
                .map(|(ip, c)| format!("{ip}: {}", c.config.lock().unwrap()))
                .collect::<Vec<_>>()
                .join("\n"),
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
        if let Ok(config) = self.instance.lock() {
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

impl Drop for Logging {
    fn drop(&mut self) {
        self.shutdown(false).unwrap();
    }
}
