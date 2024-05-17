use std::{
    io::{ BufWriter, Error, ErrorKind, Write },
    net::TcpStream,
    sync::{ Arc, Mutex },
    thread::{ self, JoinHandle },
};

use flume::{ bounded, Receiver, SendError, Sender };
use ring::aead;

use crate::def::MessageType;

use super::def::Config;

fn logger_thread(
    config: Arc<Mutex<Config>>,
    rx: Receiver<Option<(u8, String)>>
) -> Result<(), Box<dyn std::error::Error>> {
    let address = config.lock().unwrap().address.clone();
    let mut stream = BufWriter::new(TcpStream::connect(address)?);
    let mut buffer = [0u8; 3];
    while let Some((level, message)) = rx.recv()? {
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
    Ok(())
}

#[derive(Debug)]
pub struct ClientLogging {
    level: u8,
    config: Arc<Mutex<Config>>,
    tx: Sender<MessageType>,
    thr: Option<JoinHandle<()>>,
}

impl ClientLogging {
    pub fn new(level: u8, address: String) -> Result<Self, Error> {
        let (tx, rx) = bounded(1000);
        let config = Arc::new(Mutex::new(Config::new(level, address)));
        Ok(Self {
            level,
            config: config.clone(),
            tx,
            thr: Some(
                thread::Builder
                    ::new()
                    .name("ClientLogging".to_string())
                    .spawn(move || {
                        if let Err(err) = logger_thread(config, rx) {
                            eprintln!("{err:?}");
                        }
                    })?
            ),
        })
    }

    pub fn shutdown(&mut self) -> Result<(), Error> {
        if let Some(thr) = self.thr.take() {
            self.tx.send(None).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
            thr.join().map_err(|e|
                Error::new(ErrorKind::Other, e.downcast_ref::<&str>().unwrap().to_string())
            )
        } else {
            Ok(())
        }
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
        if let Ok(ref mut config) = self.config.lock() {
            config.level = level;
        }
    }

    pub fn set_encryption(&mut self, key: Option<Vec<u8>>) -> Result<(), Error> {
        if let Ok(ref mut config) = self.config.lock() {
            config.set_encryption(key).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
        }
        Ok(())
    }

    #[inline]
    pub fn send(&self, level: u8, message: String) -> Result<(), SendError<MessageType>> {
        if level >= self.level { self.tx.send(Some((level, message))) } else { Ok(()) }
    }
}
