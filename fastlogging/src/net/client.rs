use std::{
    fmt,
    io::{ BufWriter, Error, ErrorKind, Write },
    net::TcpStream,
    sync::{ Arc, Mutex },
    thread::{ self, JoinHandle },
    time::Duration,
};

use flume::{ bounded, Receiver, SendError, Sender };
use ring::aead;

use super::def::NetConfig;

#[derive(Debug)]
pub enum ClientTypeEnum {
    Message((u8, String)), // level, message
    Sync, // timeout
    Stop,
}

#[derive(Debug, Clone)]
pub struct ClientWriterConfig {
    pub(crate) level: u8,
    pub(crate) address: String,
    pub(crate) key: Option<Vec<u8>>,
}

impl ClientWriterConfig {
    pub fn new(level: u8, address: String, key: Option<Vec<u8>>) -> Self {
        Self { level, address, key }
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
    stop: Arc<Mutex<bool>>
) -> Result<(), Box<dyn std::error::Error>> {
    let address = config.lock().unwrap().address.clone();
    let mut stream = BufWriter::new(TcpStream::connect(address)?);
    let mut buffer = [0u8; 3];
    loop {
        if *stop.lock().unwrap() {
            break;
        }
        match rx.recv()? {
            ClientTypeEnum::Message((level, message)) => {
                let size = message.len();
                buffer[0] = size as u8;
                buffer[1] = (size >> 8) as u8;
                buffer[2] = level;
                if let Ok(ref mut config) = config.lock() {
                    let _ = stream.write_all(&buffer);
                    let seal = config.seal.clone();
                    let seal = aead::Aad::from(&seal);
                    if let Some(ref mut sk) = config.sk {
                        let mut encrypted = message.as_bytes().to_vec();
                        sk
                            .seal_in_place_append_tag(seal, &mut encrypted)
                            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
                        let _ = stream.write_all(&encrypted);
                    } else {
                        let _ = stream.write_all(message.as_bytes());
                    }
                }
            }
            ClientTypeEnum::Sync => {
                sync_tx.send(1)?;
            }
            ClientTypeEnum::Stop => {
                break;
            }
        }
    }
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
    pub fn new(config: ClientWriterConfig, stop: Arc<Mutex<bool>>) -> Result<Self, Error> {
        let config = Arc::new(
            Mutex::new(NetConfig::new(config.level, config.address, config.key)?)
        );
        let (tx, rx) = bounded(1000);
        let (sync_tx, sync_rx) = bounded(1);
        Ok(Self {
            config: config.clone(),
            tx,
            sync_rx,
            thr: Some(
                thread::Builder
                    ::new()
                    .name("ClientLogging".to_string())
                    .spawn(move || {
                        if let Err(err) = client_writer_thread(config, rx, sync_tx, stop) {
                            eprintln!("{err:?}");
                        }
                    })?
            ),
        })
    }

    pub fn shutdown(&mut self) -> Result<(), Error> {
        if let Some(thr) = self.thr.take() {
            self.tx
                .send(ClientTypeEnum::Stop)
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
            thr.join().map_err(|e|
                Error::new(ErrorKind::Other, e.downcast_ref::<&str>().unwrap().to_string())
            )
        } else {
            Ok(())
        }
    }

    pub fn set_level(&mut self, level: u8) {
        self.config.lock().unwrap().level = level;
    }

    pub fn set_encryption(&mut self, key: Option<Vec<u8>>) -> Result<(), Error> {
        self.config
            .lock()
            .unwrap()
            .set_encryption(key)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
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

    #[inline]
    pub fn send(&self, level: u8, message: String) -> Result<(), SendError<ClientTypeEnum>> {
        self.tx.send(ClientTypeEnum::Message((level, message)))
    }
}
