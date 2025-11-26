use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use chrono::Local;
use flume::{Receiver, Sender, bounded};

use crate::callback::CallbackWriter;
use crate::config::{ConfigFile, ExtConfig, FileMerge, LoggingInstance};
use crate::console::{ConsoleWriter, ConsoleWriterConfig};
use crate::def::{CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, LoggingTypeEnum, WARNING};
use crate::file::FileWriter;
use crate::logger::Logger;
use crate::net::{AUTH_KEY, ClientWriter, EncryptionMethod, LoggingServer, ServerConfig};
use crate::{
    LevelSyms, LoggingError, MessageStructEnum, NOTSET, SUCCESS, SyslogWriter, TRACE,
    WriterConfigEnum, WriterEnum, WriterTypeEnum, level2short, level2str, level2string, level2sym,
};

#[inline]
fn build_string_message(
    buffer: &mut String,
    config: &MutexGuard<LoggingInstance>,
    level: u8,
    domain: &str,
    message: String,
    tname: Option<String>,
    tid: u32,
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
    buffer.push_str(domain);
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
    domain: &str,
    message: String,
    tname: Option<String>,
    tid: u32,
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
    buffer.push_str(domain);
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
    domain: &str,
    message: String,
    tname: Option<String>,
    tid: u32,
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
    buffer.push_str(domain);
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
    instance: Arc<Mutex<LoggingInstance>>,
    stop: Arc<AtomicBool>,
) -> Result<(), LoggingError> {
    let mut buffer = String::with_capacity(4096);
    loop {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        let mut remote = false;
        let (level, domain, message, tname, tid) = match rx.recv()? {
            LoggingTypeEnum::Message((level, domain, message)) => (level, domain, message, None, 0),
            LoggingTypeEnum::MessageRemote((level, domain, message)) => {
                remote = true;
                (level, domain, message, None, 0)
            }
            LoggingTypeEnum::MessageExt((level, domain, message, tid, tname)) => {
                (level, domain, message, Some(tname), tid)
            }
            LoggingTypeEnum::Sync((types, timeout)) => {
                let instance = instance.lock().unwrap();
                let pid = process::id();
                let debug = instance.debug;
                if debug > 0 {
                    println!("{pid} logging_thread_worker: SYNC");
                }
                for typ in types {
                    if let Some(wids) = instance.typ2wids.get(&typ) {
                        for wid in wids {
                            if debug > 0 {
                                println!("{pid} logging_thread_worker: SYNC(wid={wid})");
                            }
                            instance.writers.get(wid).unwrap().sync(timeout)?;
                        }
                    }
                }
                sync_tx.send(1)?;
                continue;
            }
            LoggingTypeEnum::Stop => {
                if instance.lock().unwrap().debug > 0 {
                    println!("{} logging_thread_worker: STOP", process::id());
                }
                break;
            }
        };
        // Build message
        // {date} {hostname} {pname}[{pid}]>{tname}[{tid}] {domain}: {level} {message}
        let mut instance = instance.lock().unwrap();
        buffer.clear();
        if remote {
            buffer.push_str(&message);
        } else {
            match instance.structured {
                MessageStructEnum::String => {
                    build_string_message(
                        &mut buffer,
                        &instance,
                        level,
                        &domain,
                        message,
                        tname,
                        tid,
                    );
                }
                MessageStructEnum::Json => {
                    build_json_message(&mut buffer, &instance, level, &domain, message, tname, tid);
                }
                MessageStructEnum::Xml => {
                    build_xml_message(&mut buffer, &instance, level, &domain, message, tname, tid);
                }
            }
        }
        // Send message to writers
        if instance.debug > 2 {
            println!(
                "{} logging_thread_worker: MESSAGE {buffer:?}",
                process::id()
            );
        }
        for writer in instance.writers.values_mut() {
            match writer {
                WriterEnum::Root => {}
                WriterEnum::Console(console_writer) => {
                    if console_writer.config.lock().unwrap().level <= level {
                        console_writer.send(level, domain.clone(), buffer.clone())?;
                    }
                }
                WriterEnum::File(file_writer) => {
                    if file_writer.config.lock().unwrap().level <= level {
                        file_writer.send(level, domain.clone(), buffer.clone())?;
                    }
                }
                WriterEnum::Client(client_writer) => {
                    if client_writer.config.lock().unwrap().level <= level {
                        client_writer.send(level, domain.clone(), buffer.clone())?;
                    }
                }
                WriterEnum::Server(_logging_server) => {}
                WriterEnum::Callback(callback_writer) => {
                    if callback_writer.config.lock().unwrap().level <= level {
                        callback_writer.send(level, domain.clone(), buffer.clone())?;
                    }
                }
                WriterEnum::Syslog(syslog_writer) => {
                    if syslog_writer.config.lock().unwrap().level <= level {
                        syslog_writer.send(level, domain.clone(), buffer.clone())?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn logging_thread(
    rx: Receiver<LoggingTypeEnum>,
    sync_tx: Sender<u8>,
    config: Arc<Mutex<LoggingInstance>>,
    stop: Arc<AtomicBool>,
) -> Result<(), LoggingError> {
    let mut some_err = None;
    if let Err(err) = logging_thread_worker(rx, sync_tx, config.clone(), stop) {
        eprintln!(
            "{} Logging broker thread crashed with error: {err:?}",
            process::id()
        );
        some_err = Some(err);
    }
    if let Ok(mut config) = config.lock() {
        for writer in config.writers.values_mut() {
            match writer {
                WriterEnum::Root => {}
                WriterEnum::Console(console_writer) => {
                    if let Err(err) = console_writer.shutdown() {
                        eprintln!("Failed to stop console logger: {err:?}");
                    }
                }
                WriterEnum::File(file_writer) => {
                    if let Err(err) = file_writer.shutdown() {
                        eprintln!("Failed to stop file logger: {err:?}");
                    }
                }
                WriterEnum::Client(client_writer) => {
                    if let Err(err) = client_writer.shutdown() {
                        eprintln!(
                            "{} Failed to stop client writer {}: {err:?}",
                            process::id(),
                            client_writer.config.lock().unwrap().address
                        );
                    }
                }
                WriterEnum::Server(logging_server) => {
                    if let Err(err) = logging_server.shutdown() {
                        eprintln!(
                            "{} Failed to stop logging server {}: {err:?}",
                            process::id(),
                            logging_server.config.lock().unwrap().address
                        );
                    }
                }
                WriterEnum::Callback(callback_writer) => {
                    if let Err(err) = callback_writer.shutdown() {
                        eprintln!("Failed to stop callback logger: {err:?}");
                    }
                }
                WriterEnum::Syslog(syslog_writer) => {
                    if let Err(err) = syslog_writer.shutdown() {
                        eprintln!("Failed to stop syslog logger: {err:?}");
                    }
                }
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
    pub level: u8,      // Global log level
    pub domain: String, // Log domain
    pub instance: Arc<Mutex<LoggingInstance>>,
    pub(crate) server_tx: Sender<LoggingTypeEnum>,
    pub drop: bool,
    pub(crate) config_file: ConfigFile,
    pub(crate) tname: bool,
    pub(crate) tid: bool,
    loggers: HashSet<String>,
    sync_rx: Receiver<u8>,
    stop: Arc<AtomicBool>,
    thr: Option<JoinHandle<()>>,
}

impl Logging {
    pub fn new<S: Into<String>>(
        level: u8, // Global log level
        domain: S,
        configs: Option<Vec<WriterConfigEnum>>, // List of writer configs
        ext_config: Option<ExtConfig>,          // Extended logging configuration
        config_path: Option<PathBuf>,           // Optional configuration file
    ) -> Result<Self, LoggingError> {
        // Initialize config from optional config file.
        let mut config_file = ConfigFile::new();
        let mut instance = LoggingInstance::new(level, domain.into(), configs.unwrap_or_default())?;
        if let Some(ext_config) = ext_config {
            instance.set_ext_config(ext_config);
        }
        if let Some(ref config_path) = config_path {
            config_file.load(config_path)?;
            config_file.merge(&mut instance, FileMerge::MergeReplace)?;
        }
        // Overwrite settings with arguments, if provided.
        let level = instance.level;
        let domain = instance.domain.clone();
        let tname = instance.tname;
        let tid = instance.tid;
        let server_tx = instance.server_tx.clone();
        let server_rx = instance.server_rx.clone();
        let stop = instance.stop.clone();
        let (sync_tx, sync_rx) = bounded(1);
        let instance = Arc::new(Mutex::new(instance));
        let logging = Self {
            level,
            domain,
            instance: instance.clone(),
            server_tx,
            drop: true,
            config_file,
            tname,
            tid,
            loggers: HashSet::new(),
            sync_rx,
            stop: stop.clone(),
            thr: Some(
                thread::Builder::new()
                    .name("LoggingThread".to_string())
                    .spawn(move || {
                        if let Err(err) = logging_thread(server_rx, sync_tx, instance, stop) {
                            eprintln!(
                                "{} logging_thread returned with error: {err:?}",
                                process::id()
                            );
                        }
                    })?,
            ),
        };
        Ok(logging)
    }

    pub fn init() -> Result<Self, LoggingError> {
        let writer = WriterConfigEnum::Console(ConsoleWriterConfig::new(NOTSET, false));
        Logging::new(NOTSET, "root", Some(vec![writer]), None, None)
    }

    pub fn apply_config(&mut self, path: &Path) -> Result<(), LoggingError> {
        self.config_file = ConfigFile::new();
        self.config_file.load(path)?;
        let file_config = &self.config_file.config;
        let mut instance = self.instance.lock().unwrap();
        instance.level = file_config.level;
        instance.domain = file_config.domain.clone();
        // Console writer
        for file_config in file_config.configs.iter() {
            match file_config {
                WriterConfigEnum::Root(root_config) => {
                    instance.level = root_config.level;
                    instance.domain = root_config.domain.clone();
                }
                WriterConfigEnum::Console(console_writer_config) => {
                    instance.add_writer(WriterEnum::Console(ConsoleWriter::new(
                        console_writer_config.clone(),
                        self.stop.clone(),
                    )?));
                }
                WriterConfigEnum::File(file_writer_config) => {
                    instance.add_writer(WriterEnum::File(FileWriter::new(
                        file_writer_config.clone(),
                        self.stop.clone(),
                    )?));
                }
                WriterConfigEnum::Client(client_writer_config) => {
                    instance.add_writer(WriterEnum::Client(ClientWriter::new(
                        client_writer_config.clone(),
                        self.stop.clone(),
                    )?));
                }
                WriterConfigEnum::Server(server_config) => {
                    instance.add_writer(WriterEnum::Server(LoggingServer::new(
                        server_config.clone(),
                        self.server_tx.clone(),
                        self.stop.clone(),
                    )?));
                }
                WriterConfigEnum::Callback(callback_writer_config) => {
                    instance.add_writer(WriterEnum::Callback(CallbackWriter::new(
                        callback_writer_config.clone(),
                        self.stop.clone(),
                    )?));
                }
                WriterConfigEnum::Syslog(syslog_writer_config) => {
                    instance.add_writer(WriterEnum::Syslog(SyslogWriter::new(
                        syslog_writer_config.clone(),
                        self.stop.clone(),
                    )?));
                }
            }
        }
        Ok(())
    }

    pub fn shutdown(&mut self, now: bool) -> Result<(), LoggingError> {
        if self.thr.is_none() {
            return Ok(());
        }
        if now {
            self.stop.store(true, Ordering::Relaxed);
        }
        if let Err(err) = self.server_tx.send(LoggingTypeEnum::Stop) {
            eprintln!("Failed to send STOP signal to broker thread: {err:?}");
        }
        if let Some(thr) = self.thr.take() {
            thr.join().map_err(|e| {
                LoggingError::JoinError(
                    "Logging".to_string(),
                    e.downcast_ref::<&str>().unwrap().to_string(),
                )
            })
        } else {
            Ok(())
        }
    }

    pub fn set_level(&mut self, wid: usize, level: u8) -> Result<(), LoggingError> {
        let mut instance = self.instance.lock().unwrap();
        let writer = match instance.writers.get_mut(&wid) {
            Some(w) => w,
            None => {
                return Err(LoggingError::InvalidValue(format!(
                    "Writer {wid} does not exist"
                )));
            }
        };
        match writer {
            WriterEnum::Root => {
                instance.level = level;
                self.level = level;
            }
            WriterEnum::Console(console_config) => {
                console_config.set_level(level);
            }
            WriterEnum::File(file_writer) => file_writer.set_level(level),
            WriterEnum::Client(client_writer) => client_writer.set_level(level),
            WriterEnum::Server(logging_server) => logging_server.set_level(level),
            WriterEnum::Callback(callback_writer) => callback_writer.set_level(level),
            WriterEnum::Syslog(syslog_writer) => syslog_writer.set_level(level),
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
        logger.set_tx(Some(self.server_tx.clone()));
        self.loggers.insert(logger.domain.clone());
    }

    pub fn remove_logger(&mut self, logger: &'_ mut Logger) {
        logger.set_tx(None);
        self.loggers.remove(&logger.domain);
    }

    pub fn set_root_writer_config(
        &mut self,
        config: &WriterConfigEnum,
    ) -> Result<(), LoggingError> {
        match config {
            WriterConfigEnum::Client(_client_config) => {}
            WriterConfigEnum::Server(_server_config) => {}
            _ => {
                return Err(LoggingError::InvalidValue(
                    "Only Server or Client type is allowed for Root logger".to_string(),
                ));
            }
        }
        let mut instance = self.instance.lock().unwrap();
        instance.set_root_writer_config(config)
    }

    pub fn set_root_writer(&mut self, writer: WriterEnum) -> Result<(), LoggingError> {
        match &writer {
            WriterEnum::Client(_client_writer) => {}
            WriterEnum::Server(_server_writer) => {}
            _ => {
                return Err(LoggingError::InvalidValue(
                    "Only Server or Client type is allowed for Root logger".to_string(),
                ));
            }
        }
        let mut instance = self.instance.lock().unwrap();
        instance.set_root_writer(writer);
        Ok(())
    }

    pub fn add_writer_config(&mut self, config: &WriterConfigEnum) -> Result<usize, LoggingError> {
        self.instance.lock().unwrap().add_writer_config(config)
    }

    pub fn add_writer(&mut self, writer: WriterEnum) -> usize {
        self.instance.lock().unwrap().add_writer(writer)
    }

    pub fn remove_writer(&mut self, wid: usize) -> Option<WriterEnum> {
        self.instance.lock().unwrap().remove_writer(wid)
    }

    pub fn add_writer_configs(
        &mut self,
        configs: Vec<WriterConfigEnum>,
    ) -> Result<Vec<usize>, LoggingError> {
        self.instance.lock().unwrap().add_writer_configs(configs)
    }

    pub fn add_writers(&mut self, writers: Vec<WriterEnum>) -> Vec<usize> {
        self.instance.lock().unwrap().add_writers(writers)
    }

    pub fn remove_writers(&mut self, wids: Option<Vec<usize>>) -> Vec<WriterEnum> {
        self.instance.lock().unwrap().remove_writers(wids)
    }

    pub fn enable(&self, wid: usize) -> Result<(), LoggingError> {
        let mut instance = self.instance.lock().unwrap();
        let writer = match instance.writers.get_mut(&wid) {
            Some(w) => w,
            None => {
                return Err(LoggingError::InvalidValue(format!(
                    "Writer {wid} does not exist"
                )));
            }
        };
        match writer {
            WriterEnum::Root => {}
            WriterEnum::Console(console_config) => {
                console_config.enable();
            }
            WriterEnum::File(file_writer) => file_writer.enable(),
            WriterEnum::Client(client_writer) => client_writer.enable(),
            WriterEnum::Server(logging_server) => logging_server.enable(),
            WriterEnum::Callback(callback_writer) => callback_writer.enable(),
            WriterEnum::Syslog(syslog_writer) => syslog_writer.enable(),
        }
        Ok(())
    }

    pub fn disable(&self, wid: usize) -> Result<(), LoggingError> {
        let mut instance = self.instance.lock().unwrap();
        let writer = match instance.writers.get_mut(&wid) {
            Some(w) => w,
            None => {
                return Err(LoggingError::InvalidValue(format!(
                    "Writer {wid} does not exist"
                )));
            }
        };
        match writer {
            WriterEnum::Root => {}
            WriterEnum::Console(console_config) => {
                console_config.disable();
            }
            WriterEnum::File(file_writer) => file_writer.disable(),
            WriterEnum::Client(client_writer) => client_writer.disable(),
            WriterEnum::Server(logging_server) => logging_server.disable(),
            WriterEnum::Callback(callback_writer) => callback_writer.disable(),
            WriterEnum::Syslog(syslog_writer) => syslog_writer.disable(),
        }
        Ok(())
    }

    pub fn enable_type(&self, typ: WriterTypeEnum) -> Result<(), LoggingError> {
        let instance = self.instance.lock().unwrap();
        let wids = match instance.typ2wids.get(&typ) {
            Some(w) => w,
            None => {
                return Err(LoggingError::InvalidValue(format!(
                    "Writer type {typ:?} does not exist"
                )));
            }
        };
        for wid in wids {
            self.enable(*wid)?;
        }
        Ok(())
    }

    pub fn disable_type(&self, typ: WriterTypeEnum) -> Result<(), LoggingError> {
        let instance = self.instance.lock().unwrap();
        let wids = match instance.typ2wids.get(&typ) {
            Some(w) => w,
            None => {
                return Err(LoggingError::InvalidValue(format!(
                    "Writer type {typ:?} does not exist"
                )));
            }
        };
        for wid in wids {
            self.disable(*wid)?;
        }
        Ok(())
    }

    pub fn sync(&self, types: Vec<WriterTypeEnum>, timeout: f64) -> Result<(), LoggingError> {
        self.server_tx
            .send(LoggingTypeEnum::Sync((types, timeout)))
            .map_err(|e| LoggingError::SendError(format!("Failed to send SYNC command: {e}")))?;
        self.sync_rx
            .recv_timeout(Duration::from_secs_f64(timeout))
            .map_err(|e| LoggingError::RecvError(format!("Failed to receive SYNC answer: {e}")))?;
        Ok(())
    }

    pub fn sync_all(&self, timeout: f64) -> Result<(), LoggingError> {
        self.sync(
            vec![
                WriterTypeEnum::Console,
                WriterTypeEnum::Files,
                WriterTypeEnum::Clients,
                WriterTypeEnum::Servers,
                WriterTypeEnum::Callback,
                WriterTypeEnum::Syslog,
            ],
            timeout,
        )?;
        Ok(())
    }

    // File logger

    pub fn rotate(&self, path: Option<PathBuf>) -> Result<(), LoggingError> {
        for writer in self.instance.lock().unwrap().writers.values() {
            if let WriterEnum::File(writer) = writer
                && (path.is_none() || path.as_ref().unwrap() == &writer.config.lock().unwrap().path)
            {
                writer.rotate()?;
            }
        }
        Ok(())
    }

    // Network

    pub fn set_encryption(
        &mut self,
        wid: usize,
        key: EncryptionMethod,
    ) -> Result<(), LoggingError> {
        match self.instance.lock().unwrap().writers.get_mut(&wid) {
            Some(w) => match w {
                WriterEnum::Client(client_writer) => {
                    client_writer.set_encryption(key)?;
                }
                WriterEnum::Server(logging_server) => {
                    logging_server.set_encryption(key)?;
                }
                _ => {
                    return Err(LoggingError::InvalidValue(format!(
                        "Unable to configure encryption for Writer {w:?}"
                    )));
                }
            },
            None => {
                return Err(LoggingError::InvalidValue(format!(
                    "Writer {wid} does not exist"
                )));
            }
        }
        Ok(())
    }

    // Config

    pub fn set_debug(&mut self, debug: u8) {
        let mut config = self.instance.lock().unwrap();
        config.debug = debug;
        for writer in config.writers.values_mut() {
            match writer {
                WriterEnum::Root => {}
                WriterEnum::Console(console_writer) => console_writer.debug = debug,
                WriterEnum::File(file_writer) => file_writer.debug = debug,
                WriterEnum::Client(client_writer) => client_writer.debug = debug,
                WriterEnum::Server(logging_server) => logging_server.debug = debug,
                WriterEnum::Callback(callback_writer) => callback_writer.debug = debug,
                WriterEnum::Syslog(syslog_writer) => syslog_writer.debug = debug,
            }
        }
    }

    pub fn get_writer_config(&self, wid: usize) -> Option<WriterConfigEnum> {
        self.instance.lock().unwrap().get_writer_config(wid)
    }

    pub fn get_writer_configs(&self) -> HashMap<usize, WriterConfigEnum> {
        self.instance.lock().unwrap().get_writer_configs()
    }

    pub fn get_server_config(&self, wid: usize) -> Result<ServerConfig, LoggingError> {
        let writer = match self.get_writer_config(wid) {
            Some(w) => w,
            None => {
                return Err(LoggingError::InvalidValue(format!(
                    "Writer wid={wid} does not exist"
                )));
            }
        };
        match writer {
            WriterConfigEnum::Server(config) => Ok(config),
            _ => Err(LoggingError::InvalidValue(format!(
                "Writer wid={wid} has invalid type {writer:?}"
            ))),
        }
    }

    pub fn get_server_configs(&self) -> HashMap<usize, ServerConfig> {
        self.instance.lock().unwrap().get_server_configs()
    }

    pub fn get_root_server_address_port(&self) -> Option<String> {
        self.instance.lock().unwrap().get_root_server_address_port()
    }

    pub fn get_server_addresses_ports(&self) -> HashMap<usize, String> {
        self.instance.lock().unwrap().get_server_addresses_ports()
    }

    pub fn get_server_addresses(&self) -> HashMap<usize, String> {
        self.instance.lock().unwrap().get_server_addresses()
    }

    pub fn get_server_ports(&self) -> HashMap<usize, u16> {
        self.instance.lock().unwrap().get_server_ports()
    }

    pub fn get_server_auth_key(&self) -> EncryptionMethod {
        EncryptionMethod::AuthKey(AUTH_KEY.to_vec())
    }

    pub fn get_config_string(&self) -> String {
        let instance = self.instance.lock().unwrap();
        format!(
            "level={:?}\n\
            domain={:?}\n\
            hostname={:?}\n\
            pname={:?}\n\
            pid={}\n\
            tname={:?}\n\
            tid={}\n\
            structured={:?}\n\
            writers={:?}\n\
            loggers={:?}",
            level2string(&instance.level2sym, instance.level),
            instance.domain,
            instance.hostname,
            instance.pname,
            instance.pid,
            instance.tname,
            instance.tid,
            instance.structured,
            self.get_writer_configs(),
            self.loggers
        )
    }

    pub fn save_config(&mut self, path: Option<&Path>) -> Result<(), LoggingError> {
        self.config_file =
            ConfigFile::from_instance(&self.config_file.path, &self.instance.lock().unwrap());
        self.config_file.save(path)
    }

    // Logging methods

    #[inline]
    fn log<S: Into<String>>(&self, level: u8, message: S) -> Result<(), LoggingError> {
        (if self.tname || self.tid {
            let tname = if self.tname {
                thread::current().name().unwrap_or_default().to_string()
            } else {
                "".to_string()
            };
            let tid = if self.tid { thread_id::get() as u32 } else { 0 };
            self.server_tx.send(LoggingTypeEnum::MessageExt((
                level,
                self.domain.clone(),
                message.into(),
                tid,
                tname,
            )))
        } else {
            self.server_tx.send(LoggingTypeEnum::Message((
                level,
                self.domain.clone(),
                message.into(),
            )))
        })
        .map_err(|e| {
            LoggingError::SendError(format!(
                "Failed to send {} message: {e:?}",
                level2str(level)
            ))
        })
    }

    pub fn trace<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= TRACE {
            self.log(TRACE, message)?;
        }
        Ok(())
    }

    pub fn debug<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= DEBUG {
            self.log(DEBUG, message)?;
        }
        Ok(())
    }

    pub fn info<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= INFO {
            self.log(INFO, message)?;
        }
        Ok(())
    }

    pub fn success<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= SUCCESS {
            self.log(SUCCESS, message)?;
        }
        Ok(())
    }

    pub fn warning<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= WARNING {
            self.log(WARNING, message)?;
        }
        Ok(())
    }

    pub fn error<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= ERROR {
            self.log(ERROR, message)?;
        }
        Ok(())
    }

    pub fn critical<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= CRITICAL {
            self.log(CRITICAL, message)?;
        }
        Ok(())
    }

    pub fn fatal<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
        if self.level <= FATAL {
            self.log(FATAL, message)?;
        }
        Ok(())
    }

    pub fn exception<S: Into<String>>(&self, message: S) -> Result<(), LoggingError> {
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
        Self::new(NOTSET, "root", None, None, None).unwrap()
    }
}

impl Drop for Logging {
    fn drop(&mut self) {
        self.shutdown(false).unwrap();
    }
}
