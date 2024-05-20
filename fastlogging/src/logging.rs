use std::collections::HashMap;
use std::io::{ Error, ErrorKind };
use std::sync::{ Arc, Mutex };
use std::thread::{ self, JoinHandle };

use flume::{ bounded, Receiver, Sender };
use chrono::Local;

use crate::console::{ ConsoleWriter, ConsoleWriterConfig };
use crate::def::{
    MessageTypeEnum,
    NOTSET,
    CRITICAL,
    DEBUG,
    ERROR,
    EXCEPTION,
    FATAL,
    INFO,
    WARNING,
};
use crate::file::{ FileWriter, FileWriterConfig };
use crate::net::{ ClientWriter, ClientWriterConfig, LoggingServer, ServerConfig };
use crate::{ level2str, level2sym, level2short, LevelSyms };
use crate::logger::Logger;

fn logging_thread_worker(
    rx: Receiver<MessageTypeEnum>,
    config: Arc<Mutex<LoggingConfig>>,
    stop: Arc<Mutex<bool>>
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        if *stop.lock().unwrap() {
            break;
        }
        if let Ok(mut config) = config.lock() {
            match rx.recv()? {
                MessageTypeEnum::Message((level, message)) => {
                    let date = Local::now();
                    let message = format!(
                        "{} {} {} {message}",
                        date.format("%Y.%m.%d %H:%M:%S"),
                        config.domain,
                        match config.level2sym {
                            LevelSyms::Sym => level2sym(level),
                            LevelSyms::Short => level2short(level),
                            LevelSyms::Str => level2str(level),
                        }
                    );
                    if let Some(ref mut console) = config.console {
                        console.send(level, message.clone())?;
                    }
                    if let Some(ref mut file) = config.file {
                        file.send(level, message.clone())?;
                    }
                    for client in config.clients.values() {
                        client.send(level, message.clone())?;
                    }
                }
                MessageTypeEnum::Rotate => {
                    if let Some(ref mut file) = config.file {
                        file.rotate()?;
                    }
                }
                MessageTypeEnum::Sync(timeout) => {
                    if let Some(ref mut console) = config.console {
                        console.sync(timeout)?;
                    }
                    if let Some(ref mut file) = config.file {
                        file.sync(timeout)?;
                    }
                    for client in config.clients.values() {
                        client.sync(timeout)?;
                    }
                }
                MessageTypeEnum::Stop => {
                    break;
                }
            }
        } else {
            break;
        }
    }
    Ok(())
}

fn logging_thread(
    rx: Receiver<MessageTypeEnum>,
    config: Arc<Mutex<LoggingConfig>>,
    stop: Arc<Mutex<bool>>
) -> Result<(), Box<dyn std::error::Error>> {
    let mut some_err = None;
    if let Err(err) = logging_thread_worker(rx, config.clone(), stop) {
        eprintln!("Logging broker thread crashed with error: {err:?}");
        some_err = Some(err);
    }
    if let Ok(mut config) = config.lock() {
        for (address, client) in config.clients.iter_mut() {
            if let Err(err) = client.shutdown() {
                eprintln!("Failed to stop client {address}: {err:?}");
            }
        }
        if let Some(ref mut server) = config.server {
            if let Err(err) = server.shutdown() {
                eprintln!("Failed to stop server: {err:?}");
            }
        }
        if let Some(ref mut console) = config.console {
            if let Err(err) = console.shutdown() {
                eprintln!("Failed to stop console logger: {err:?}");
            }
        }
        if let Some(ref mut file) = config.file {
            if let Err(err) = file.shutdown() {
                eprintln!("Failed to stop file logger: {err:?}");
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
pub struct LoggingConfig {
    level: u8,
    domain: String,
    console: Option<ConsoleWriter>,
    file: Option<FileWriter>,
    server: Option<LoggingServer>,
    clients: HashMap<String, ClientWriter>,
    level2sym: LevelSyms,
}

#[derive(Debug)]
pub struct Logging {
    pub level: u8,
    config: Arc<Mutex<LoggingConfig>>,
    pub tx: Sender<MessageTypeEnum>,
    stop: Arc<Mutex<bool>>,
    thr: Option<JoinHandle<()>>,
}

impl Logging {
    pub fn new(
        level: Option<u8>, // Global log level
        domain: Option<String>,
        console: Option<ConsoleWriterConfig>, // If true start ConsoleLogging
        file: Option<FileWriterConfig>, // If path is defined start FileLogging
        server: Option<ServerConfig>, // If address is defined start LoggingServer
        connect: Option<ClientWriterConfig> // If address is defined start ClientLogging
    ) -> Result<Self, Error> {
        let level = level.unwrap_or(NOTSET);
        let domain = domain.unwrap_or("root".to_string());
        let (tx, rx) = bounded(1000);
        let stop = Arc::new(Mutex::new(false));
        let console = if let Some(config) = console {
            Some(ConsoleWriter::new(config, stop.clone())?)
        } else {
            None
        };
        let file = if let Some(config) = file {
            Some(FileWriter::new(config, stop.clone())?)
        } else {
            None
        };
        let mut clients = HashMap::new();
        if let Some(config) = connect {
            clients.insert(config.address.clone(), ClientWriter::new(config, stop.clone())?);
        }
        let server = if let Some(config) = server {
            Some(LoggingServer::new(config, tx.clone(), stop.clone())?)
        } else {
            None
        };
        let config = Arc::new(
            Mutex::new(LoggingConfig {
                level,
                domain,
                console,
                file,
                server,
                clients,
                level2sym: LevelSyms::Sym,
            })
        );
        Ok(Self {
            level,
            config: config.clone(),
            tx,
            stop: stop.clone(),
            thr: Some(
                thread::Builder
                    ::new()
                    .name("FileLogging".to_string())
                    .spawn(move || {
                        if let Err(err) = logging_thread(rx, config, stop) {
                            eprintln!("logging_thread returned with error: {err:?}");
                        }
                    })?
            ),
        })
    }

    pub fn shutdown(&mut self, now: Option<bool>) -> Result<(), Error> {
        if self.thr.is_none() {
            return Ok(());
        }
        if now.unwrap_or_default() {
            *self.stop.lock().unwrap() = true;
        }
        if let Err(err) = self.tx.send(MessageTypeEnum::Stop) {
            eprintln!("Failed to send STOP signal to broker thread: {err:?}");
        }
        if let Some(thr) = self.thr.take() {
            thr.join().map_err(|e|
                Error::new(ErrorKind::Other, e.downcast_ref::<&str>().unwrap().to_string())
            )
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
        if let Some(ref file) = self.config.lock().unwrap().file { file.rotate() } else { Ok(()) }
    }

    pub fn sync(&self, console: bool, file: bool, client: bool, timeout: f64) -> Result<(), Error> {
        if console {
            if let Some(ref console) = self.config.lock().unwrap().console {
                console.sync(timeout)?;
            }
        }
        if file {
            if let Some(ref file) = self.config.lock().unwrap().file {
                file.sync(timeout)?;
            }
        }
        if client {
            for client in self.config.lock().unwrap().clients.values() {
                client.sync(timeout)?;
            }
        }
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
            return Err(Error::new(ErrorKind::NotFound, "Client not found".to_string()));
        }
        Ok(())
    }

    pub fn set_client_level(&mut self, address: &str, level: u8) -> Result<(), Error> {
        if let Ok(mut config) = self.config.lock() {
            if let Some(client) = config.clients.get_mut(address) {
                client.set_level(level);
                return Ok(());
            }
            return Err(Error::new(ErrorKind::NotFound, "Client not found".to_string()));
        }
        Ok(())
    }

    pub fn set_client_encryption(
        &mut self,
        address: &str,
        key: Option<Vec<u8>>
    ) -> Result<(), Error> {
        if let Ok(mut config) = self.config.lock() {
            if let Some(client) = config.clients.get_mut(address) {
                client.set_encryption(key)?;
                return Ok(());
            }
            return Err(Error::new(ErrorKind::NotFound, "Client not found".to_string()));
        }
        Ok(())
    }

    // Network server

    pub fn server_start<S: Into<String>>(
        &mut self,
        address: S,
        level: u8,
        key: Option<Vec<u8>>
    ) -> Result<(), Error> {
        if let Ok(mut logging_config) = self.config.lock() {
            let config = ServerConfig::new(level, address.into(), key);
            logging_config.server = Some(
                LoggingServer::new(config, self.tx.clone(), self.stop.clone())?
            );
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
            return Err(Error::new(ErrorKind::NotFound, "Server not running".to_string()));
        }
        Ok(())
    }

    pub fn set_server_encryption(&mut self, key: Option<Vec<u8>>) -> Result<(), Error> {
        if let Ok(mut config) = self.config.lock() {
            if let Some(ref mut server) = config.server {
                server.set_encryption(key)?;
                return Ok(());
            }
            return Err(Error::new(ErrorKind::NotFound, "Server not running".to_string()));
        }
        Ok(())
    }

    // Logging calls

    #[inline]
    fn log<S: Into<String>>(&self, level: u8, message: S) -> Result<(), Error> {
        self.tx
            .send(MessageTypeEnum::Message((level, message.into())))
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
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
