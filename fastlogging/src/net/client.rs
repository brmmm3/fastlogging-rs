use std::{
    fmt,
    io::{BufWriter, Error, Write},
    net::TcpStream,
    process,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use flume::{Receiver, SendError, Sender, bounded};
use parking_lot::RwLock;
use regex::Regex;
use ring::aead;

use crate::LoggingError;

use super::{EncryptionMethod, def::NetConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientTypeEnum {
    Message((u8, String, String)), // level, domain, message
    Sync,                          // timeout
    Stop,
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientWriterConfig {
    /// Only send log messages if enabled is true
    pub enabled: bool,
    /// Log level for filtering log messages
    pub level: u8,
    /// Optional filter log messages by domain
    pub domain_filter: Option<String>,
    /// Optional filter log messages by their contents
    pub message_filter: Option<String>,
    /// IP address to connect and send log messages
    pub address: String,
    /// IP port
    pub port: u16,
    /// Optional key for authentication and message encryption
    pub key: EncryptionMethod,
    /// Debug level. Only for developers.
    pub debug: u8,
}

impl ClientWriterConfig {
    pub fn new<S: Into<String>>(level: u8, address: S, key: EncryptionMethod) -> Self {
        let address: String = address.into();
        let port = if address.contains(':') {
            address
                .split(':')
                .next_back()
                .unwrap()
                .parse::<u16>()
                .unwrap()
        } else {
            0
        };
        Self {
            enabled: true,
            level,
            domain_filter: None,
            message_filter: None,
            address,
            port,
            key,
            debug: 0,
        }
    }

    pub fn get_address_port(&self) -> String {
        if self.port > 0 {
            format!("{}:{}", self.address, self.port)
        } else {
            self.address.clone()
        }
    }
}

impl fmt::Display for ClientWriterConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

fn client_writer_thread(
    config: Arc<RwLock<NetConfig>>,
    rx: Receiver<ClientTypeEnum>,
    sync_tx: Sender<u8>,
    stop: Arc<AtomicBool>,
) -> Result<(), LoggingError> {
    let (address, debug) = {
        let config = config.read();
        (config.address.clone(), config.debug)
    };
    if debug > 0 {
        println!(
            "{} client_writer_thread CONNECTING to {address}",
            process::id()
        );
    }
    let mut stream = BufWriter::new(TcpStream::connect(&address)?);
    if debug > 0 {
        println!(
            "{} client_writer_thread CONNECTED to {address}",
            process::id()
        );
    }
    let mut buffer = [0u8; 4];
    {
        let config = config.read();
        if !config.key.is_encrypted() {
            if debug > 1 {
                println!("{} client_writer_thread SEND KEY", process::id());
            }
            let key = config.key.key().unwrap();
            let size = key.len();
            buffer[0] = size as u8;
            buffer[1] = (size >> 8) as u8;
            stream.write_all(&buffer)?;
            stream.write_all(key)?;
            stream.flush()?;
        }
    }
    loop {
        if stop.load(Ordering::Relaxed) {
            if debug > 0 {
                println!("{} client_writer_thread STOP signal", process::id());
            }
            break;
        }
        match rx.recv()? {
            ClientTypeEnum::Message((level, domain, message)) => {
                if debug > 1 {
                    println!(
                        "{} client_writer_thread SEND MESSAGE {level} {message}",
                        process::id()
                    );
                }
                let mut config_write = config.write();
                let size;
                let seal = config_write.seal.clone();
                let seal = aead::Aad::from(&seal);
                if let Some(ref mut sk) = config_write.sk {
                    let mut domain = domain.as_bytes().to_vec();
                    sk.seal_in_place_append_tag(seal, &mut domain)
                        .map_err(|e| Error::other(e.to_string()))?;
                    let mut message = message.as_bytes().to_vec();
                    sk.seal_in_place_append_tag(seal, &mut message)
                        .map_err(|e| Error::other(e.to_string()))?;
                    size = message.len();
                    buffer[0] = size as u8;
                    buffer[1] = (size >> 8) as u8;
                    buffer[2] = level;
                    buffer[3] = domain.len() as u8;
                    let _ = stream.write_all(&buffer);
                    let _ = stream.write_all(&domain);
                    let _ = stream.write_all(&message);
                } else {
                    size = message.len();
                    buffer[0] = size as u8;
                    buffer[1] = (size >> 8) as u8;
                    buffer[2] = level;
                    buffer[3] = domain.len() as u8;
                    //println!("SEND {buffer:?}");
                    let _ = stream.write_all(&buffer);
                    let _ = stream.write_all(domain.as_bytes());
                    //println!("SEND1 {:?}", domain.as_bytes());
                    let _ = stream.write_all(message.as_bytes());
                    //println!("SEND2 {:?}", message.as_bytes());
                }
                stream.flush()?;
            }
            ClientTypeEnum::Sync => {
                if debug > 0 {
                    println!("{} client_writer_thread SYNC", process::id());
                }
                sync_tx.send(1)?;
            }
            ClientTypeEnum::Stop => {
                if debug > 0 {
                    println!("{} client_writer_thread STOP received", process::id());
                }
                break;
            }
        }
    }
    //stream.into_inner()?.shutdown(Shutdown::Both)?;
    //println!("{} client_writer_thread FIN", process::id());
    Ok(())
}

#[derive(Debug)]
pub struct ClientWriter {
    pub config: Arc<RwLock<NetConfig>>,
    tx: Sender<ClientTypeEnum>,
    sync_rx: Receiver<u8>,
    thr: Option<JoinHandle<()>>,
    pub debug: u8,
}

impl ClientWriter {
    pub fn new(
        writer_config: ClientWriterConfig,
        stop: Arc<AtomicBool>,
    ) -> Result<Self, LoggingError> {
        let config = Arc::new(RwLock::new(NetConfig::new(
            writer_config.level,
            writer_config.address,
            writer_config.port,
            writer_config.key,
        )?));
        let (tx, rx) = bounded(1000);
        let (sync_tx, sync_rx) = bounded(1);
        let (tx_started, rx_started) = bounded(1);
        // Wait for thread started
        let config_cloned = config.clone();
        let thr = thread::Builder::new()
            .name("ClientLogging".to_string())
            .spawn(move || {
                tx_started.send(1).expect("Failed to send started signal");
                if let Err(err) = client_writer_thread(config_cloned, rx, sync_tx, stop) {
                    eprintln!(
                        "{} client_writer_thread: Finished with error: {err:?}",
                        process::id()
                    );
                }
                //println!("{} client_writer_thread FINISHED", process::id());
            })?;
        rx_started
            .recv_timeout(Duration::from_millis(100))
            .map_err(|e| LoggingError::RecvError(format!("Failed to start logging server: {e}")))?;
        Ok(Self {
            config,
            tx,
            sync_rx,
            thr: Some(thr),
            debug: 0,
        })
    }

    pub fn shutdown(&mut self) -> Result<(), LoggingError> {
        if let Some(thr) = self.thr.take() {
            self.tx.send(ClientTypeEnum::Stop).map_err(|e| {
                LoggingError::SendCmdError(
                    "ClientWriter".to_string(),
                    "STOP".to_string(),
                    e.to_string(),
                )
            })?;
            thr.join().map_err(|e| {
                LoggingError::JoinError(
                    "ClientWriter".to_string(),
                    e.downcast_ref::<&str>().unwrap().to_string(),
                )
            })
        } else {
            Ok(())
        }
    }

    pub fn sync(&self, timeout: f64) -> Result<(), LoggingError> {
        self.tx.send(ClientTypeEnum::Sync).map_err(|e| {
            LoggingError::SendCmdError(
                "ClientWriter".to_string(),
                "SYNC".to_string(),
                e.to_string(),
            )
        })?;
        self.sync_rx
            .recv_timeout(Duration::from_secs_f64(timeout))
            .map_err(|e| {
                LoggingError::RecvAswError(
                    "ClientWriter".to_string(),
                    "SYNC".to_string(),
                    e.to_string(),
                )
            })?;
        Ok(())
    }

    pub fn enable(&self) {
        self.config.write().enabled = true;
    }

    pub fn disable(&self) {
        self.config.write().enabled = false;
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.config.write().enabled = enabled;
    }

    pub fn set_level(&mut self, level: u8) {
        self.config.write().level = level;
    }

    pub fn set_domain_filter(&self, domain_filter: Option<String>) -> Result<(), regex::Error> {
        if let Some(ref message) = domain_filter {
            Regex::new(message)?;
        }
        self.config.write().domain_filter = domain_filter;
        Ok(())
    }

    pub fn set_message_filter(&self, message_filter: Option<String>) -> Result<(), regex::Error> {
        if let Some(ref message) = message_filter {
            Regex::new(message)?;
        }
        self.config.write().message_filter = message_filter;
        Ok(())
    }

    pub fn set_encryption(&mut self, method: EncryptionMethod) -> Result<(), LoggingError> {
        self.config
            .write()
            .set_encryption(method.clone())
            .map_err(|e| {
                LoggingError::InvalidEncryption("ClientWriter".to_string(), method, e.to_string())
            })
    }

    #[inline]
    pub fn send(
        &self,
        level: u8,
        domain: String,
        message: String,
    ) -> Result<(), SendError<ClientTypeEnum>> {
        self.tx
            .send(ClientTypeEnum::Message((level, domain, message)))
    }
}
