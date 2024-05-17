use std::collections::HashMap;
use std::io::{ Error, ErrorKind };
use std::path::PathBuf;
use std::sync::{ Arc, Mutex };
use std::thread::{ self, JoinHandle };
use std::time::{ Duration, Instant };

use flume::{ bounded, Receiver, Sender };
use chrono::Local;

use crate::console::ConsoleLogging;
use crate::def::{ MessageType, NOTSET, CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, WARNING };
use crate::file::FileLogging;
use crate::net::{ ClientLogging, LoggingServer };
use crate::{ level2str, level2sym, level2short, LevelSyms };
use crate::logger::Logger;

fn logging_thread_worker(
    rx: Receiver<MessageType>,
    config: Arc<Mutex<LoggingConfig>>
) -> Result<(), Box<dyn std::error::Error>> {
    while let Some((level, message)) = rx.recv()? {
        if let Ok(mut config) = config.lock() {
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
        } else {
            break;
        }
    }
    Ok(())
}

fn logging_thread(
    rx: Receiver<MessageType>,
    config: Arc<Mutex<LoggingConfig>>
) -> Result<(), Box<dyn std::error::Error>> {
    let mut some_err = None;
    if let Err(err) = logging_thread_worker(rx, config.clone()) {
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
    console: Option<ConsoleLogging>,
    file: Option<FileLogging>,
    server: Option<LoggingServer>,
    clients: HashMap<String, ClientLogging>,
    level2sym: LevelSyms,
}

#[derive(Debug)]
pub struct Logging {
    pub level: u8,
    config: Arc<Mutex<LoggingConfig>>,
    pub tx: Sender<MessageType>,
    sync_rx: Receiver<u8>,
    stop: Arc<Mutex<bool>>,
    thr: Option<JoinHandle<()>>,
}

impl Logging {
    pub fn new(
        level: Option<u8>, // Global log level
        domain: Option<String>,
        console: Option<bool>, // If true start ConsoleLogging
        file: Option<PathBuf>, // If path is defined start FileLogging
        server: Option<String>, // If address is defined start LoggingServer
        connect: Option<String>, // If address is defined start ClientLogging
        max_size: Option<usize>, // Maximum size of log files
        backlog: Option<usize> // Maximum number of backup log files
    ) -> Result<Self, Error> {
        let level = level.unwrap_or(NOTSET);
        let domain = domain.unwrap_or("root".to_string());
        let (tx, rx) = bounded(1000);
        let stop = Arc::new(Mutex::new(false));
        let (sync_tx, sync_rx) = bounded(1);
        let console = if console.unwrap_or_default() {
            Some(ConsoleLogging::new(level, stop.clone())?)
        } else {
            None
        };
        let file = if let Some(path) = file {
            Some(
                FileLogging::new(
                    level,
                    path,
                    max_size.unwrap_or_default(),
                    backlog.unwrap_or_default(),
                    stop.clone(),
                    sync_tx
                )?
            )
        } else {
            None
        };
        let mut clients = HashMap::new();
        if let Some(address) = connect {
            clients.insert(address.clone(), ClientLogging::new(level, address)?);
        }
        let server = if let Some(address) = server {
            Some(LoggingServer::new(level, address, tx.clone())?)
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
            sync_rx,
            stop,
            thr: Some(
                thread::Builder
                    ::new()
                    .name("FileLogging".to_string())
                    .spawn(move || {
                        if let Err(err) = logging_thread(rx, config) {
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
        if let Err(err) = self.tx.send(None) {
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

    pub fn set_console_writer(&mut self, level: Option<u8>) -> Result<(), Error> {
        self.config.lock().unwrap().console = if let Some(level) = level {
            Some(ConsoleLogging::new(level, self.stop.clone())?)
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

    pub fn set_file_writer(
        &mut self,
        level: Option<u8>,
        path: Option<PathBuf>,
        max_size: Option<usize>, // Maximum size of log files
        backlog: Option<usize> // Maximum number of backup log files
    ) -> Result<(), Error> {
        let level = level.unwrap_or(NOTSET);
        self.config.lock().unwrap().file = if let Some(path) = path {
            let (sync_tx, sync_rx) = bounded(1);
            self.sync_rx = sync_rx;
            Some(
                FileLogging::new(
                    level,
                    path,
                    max_size.unwrap_or_default(),
                    backlog.unwrap_or_default(),
                    self.stop.clone(),
                    sync_tx
                )?
            )
        } else {
            None
        };
        Ok(())
    }

    pub fn rotate(&self) -> Result<(), Error> {
        if let Some(ref file) = self.config.lock().unwrap().file { file.rotate() } else { Ok(()) }
    }

    pub fn sync(&self, timeout: f64) -> Result<(), Error> {
        if let Some(ref file) = self.config.lock().unwrap().file {
            file.send(255, "".to_string())?;
            self.sync_rx
                .recv_deadline(
                    Instant::now().checked_add(Duration::from_secs_f64(timeout)).unwrap()
                )
                .map_err(|e| Error::new(ErrorKind::BrokenPipe, e.to_string()))?;
        }
        Ok(())
    }

    // Network client

    pub fn connect<S: Into<String>>(
        &mut self,
        address: S,
        level: u8,
        key: Option<Vec<u8>>
    ) -> Result<(), Error> {
        if let Ok(mut config) = self.config.lock() {
            let address: String = address.into();
            let mut client = ClientLogging::new(level, address.clone())?;
            client.set_encryption(key)?;
            config.clients.insert(address, client);
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
        if let Ok(mut config) = self.config.lock() {
            let address: String = address.into();
            let mut server = LoggingServer::new(level, address, self.tx.clone())?;
            server.set_encryption(key)?;
            config.server = Some(server);
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
            .send(Some((level, message.into())))
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
