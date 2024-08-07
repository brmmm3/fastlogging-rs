use std::{
    fmt,
    io::{BufWriter, Error, ErrorKind, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use flume::{bounded, Receiver, SendError, Sender};
use ring::aead;

use crate::LoggingError;

use super::{def::NetConfig, EncryptionMethod};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientTypeEnum {
    Message((u8, String)), // level, message
    Sync,                  // timeout
    Stop,
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientWriterConfig {
    pub(crate) level: u8,
    pub(crate) address: String,
    pub(crate) port: u16,
    pub(crate) key: EncryptionMethod,
    pub(crate) debug: u8,
}

impl ClientWriterConfig {
    pub fn new<S: Into<String>>(level: u8, address: S, key: EncryptionMethod) -> Self {
        let address: String = address.into();
        let port = if address.contains(':') {
            address.split(':').last().unwrap().parse::<u16>().unwrap()
        } else {
            0
        };
        Self {
            level,
            address,
            port,
            key,
            debug: 0,
        }
    }
}

impl fmt::Display for ClientWriterConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn client_writer_thread(
    config: Arc<Mutex<NetConfig>>,
    rx: Receiver<ClientTypeEnum>,
    sync_tx: Sender<u8>,
    stop: Arc<Mutex<bool>>,
) -> Result<(), LoggingError> {
    let (address, debug) = {
        let config = config.lock().unwrap();
        (config.address.clone(), config.debug)
    };
    if debug > 0 {
        println!("client_writer_thread CONNECTING to {address}");
    }
    /*println!(
        "++client_writer_thread CONNECTING to {address} {}",
        std::process::id()
    );*/
    let mut stream = BufWriter::new(TcpStream::connect(&address)?);
    if debug > 0 {
        println!("client_writer_thread CONNECTED to {address}");
    }
    /*println!(
        "++client_writer_thread CONNECTED to {address} {}",
        std::process::id()
    );*/
    let mut buffer = [0u8; 3];
    {
        let config = config.lock().unwrap();
        if !config.key.is_encrypted() {
            //println!("client_writer_thread SEND KEY");
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
        if *stop.lock().unwrap() {
            if debug > 0 {
                println!("client_writer_thread STOP signal");
            }
            //println!("++client_writer_thread STOP signal {}", std::process::id());
            break;
        }
        match rx.recv()? {
            ClientTypeEnum::Message((level, message)) => {
                /*println!(
                    "++client_writer_thread SEND MESSAGE {} {level} {message}",
                    std::process::id()
                );*/
                if let Ok(ref mut config) = config.lock() {
                    let size;
                    let seal = config.seal.clone();
                    let seal = aead::Aad::from(&seal);
                    if let Some(ref mut sk) = config.sk {
                        let mut encrypted = message.as_bytes().to_vec();
                        sk.seal_in_place_append_tag(seal, &mut encrypted)
                            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
                        size = encrypted.len();
                        buffer[0] = size as u8;
                        buffer[1] = (size >> 8) as u8;
                        buffer[2] = level;
                        let _ = stream.write_all(&buffer);
                        let _ = stream.write_all(&encrypted);
                    } else {
                        size = message.len();
                        buffer[0] = size as u8;
                        buffer[1] = (size >> 8) as u8;
                        buffer[2] = level;
                        let _ = stream.write_all(&buffer);
                        let _ = stream.write_all(message.as_bytes());
                    }
                    stream.flush()?;
                }
            }
            ClientTypeEnum::Sync => {
                if debug > 0 {
                    println!("client_writer_thread SYNC");
                }
                //println!("++client_writer_thread SYNC {}", std::process::id());
                sync_tx.send(1)?;
            }
            ClientTypeEnum::Stop => {
                if debug > 0 {
                    println!("client_writer_thread STOP received");
                }
                /*println!(
                    "++client_writer_thread STOP received {}",
                    std::process::id()
                );*/
                break;
            }
        }
    }
    //stream.into_inner()?.shutdown(Shutdown::Both)?;
    //println!("++client_writer_thread FIN {}", std::process::id());
    Ok(())
}

#[derive(Debug)]
pub struct ClientWriter {
    pub(crate) config: Arc<Mutex<NetConfig>>,
    tx: Sender<ClientTypeEnum>,
    sync_rx: Receiver<u8>,
    thr: Option<JoinHandle<()>>,
}

impl ClientWriter {
    pub fn new(writer: ClientWriterConfig, stop: Arc<Mutex<bool>>) -> Result<Self, Error> {
        let config = Arc::new(Mutex::new(NetConfig::new(
            writer.level,
            writer.address,
            writer.port,
            writer.key,
        )?));
        config.lock().unwrap().debug = writer.debug;
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
                        "client_writer_thread: Finished with error: {} {err:?}",
                        std::process::id()
                    );
                }
                //println!("++client_writer_thread FINISHED {}", std::process::id());
            })?;
        rx_started
            .recv_timeout(Duration::from_millis(100))
            .map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to start logging server: {e}"),
                )
            })?;
        Ok(Self {
            config,
            tx,
            sync_rx,
            thr: Some(thr),
        })
    }

    pub fn shutdown(&mut self) -> Result<(), Error> {
        if let Some(thr) = self.thr.take() {
            self.tx
                .send(ClientTypeEnum::Stop)
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
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

    pub fn sync(&self, timeout: f64) -> Result<(), Error> {
        self.tx
            .send(ClientTypeEnum::Sync)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        self.sync_rx
            .recv_timeout(Duration::from_secs_f64(timeout))
            .map_err(|e| Error::new(ErrorKind::BrokenPipe, e.to_string()))?;
        Ok(())
    }

    pub fn set_level(&mut self, level: u8) {
        self.config.lock().unwrap().level = level;
    }

    pub fn set_encryption(&mut self, key: EncryptionMethod) -> Result<(), Error> {
        self.config
            .lock()
            .unwrap()
            .set_encryption(key)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
    }

    #[inline]
    pub fn send(&self, level: u8, message: String) -> Result<(), SendError<ClientTypeEnum>> {
        self.tx.send(ClientTypeEnum::Message((level, message)))
    }
}
