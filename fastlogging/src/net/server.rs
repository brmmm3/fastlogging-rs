use std::{
    collections::HashMap,
    io::{ Error, ErrorKind, Read, Write },
    net::{ TcpListener, TcpStream },
    sync::{ Arc, Mutex },
    thread::{ self, JoinHandle },
};

use flume::Sender;
use ring::aead::{ self, BoundKey };

use crate::def::MessageType;

use super::{ def::Config, NonceGenerator };

fn handle_client(
    config: Arc<Mutex<Config>>,
    mut stream: TcpStream,
    tx: Sender<MessageType>
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut buffer = [0u8; 4096];
    let n = stream.read(&mut buffer[0..2])?;
    if n == 3 {
        let size = (buffer[0] as usize) | ((buffer[1] as usize) << 8);
        if size == 0xffff {
            return Ok(true);
        }
        let msg_level = buffer[2];
        let data = &mut buffer[..size];
        stream.read_exact(data)?;
        if let Ok(ref mut config) = config.lock() {
            let seal = config.seal.clone();
            let seal = aead::Aad::from(&seal);
            if config.sk.is_some() {
                let mut ok = aead::OpeningKey::new(
                    aead::UnboundKey
                        ::new(&aead::AES_256_GCM, config.key.as_deref().unwrap())
                        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?,
                    NonceGenerator::new()
                );
                let _ = ok
                    .open_in_place(seal, data)
                    .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
            }
        }
        let message = std::str::from_utf8(data).unwrap().to_string();
        if let Ok(ref mut config) = config.lock() {
            if msg_level >= config.level {
                tx.send(Some((msg_level, message)))?;
            }
        }
    }
    Ok(false)
}

fn logger_thread(
    config: Arc<Mutex<Config>>,
    tx: Sender<MessageType>
) -> Result<(), Box<dyn std::error::Error>> {
    let address = config.lock().unwrap().address.clone();
    let listener = TcpListener::bind(address).unwrap();
    let pool = threadpool::ThreadPool::new(num_cpus::get());
    let buggy_clients: Arc<Mutex<HashMap<std::net::SocketAddr, usize>>> = Arc::new(
        Mutex::new(HashMap::new())
    );

    for stream in listener.incoming() {
        // Message format: [size:u16, level:u8, data]
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprint!("{e:?}");
                continue;
            }
        };
        let addr = match stream.peer_addr() {
            Ok(a) => a,
            Err(e) => {
                eprint!("{e:?}");
                continue;
            }
        };
        // Clients have are allowed to produce 3 errors. In case of more errors they will be ignored.
        if *buggy_clients.lock().unwrap().get(&addr).unwrap_or(&0) > 3 {
            continue;
        }
        let addr_clone = addr.clone();
        let config_clone = config.clone();
        let tx_clone = tx.clone();
        let buggy_clients_clone = buggy_clients.clone();
        pool.execute(move || {
            if let Err(err) = handle_client(config_clone, stream, tx_clone) {
                eprint!("{err:?}");
                if let Ok(mut buggy_client) = buggy_clients_clone.lock() {
                    if let Some(c) = buggy_client.get_mut(&addr_clone) {
                        *c += 1;
                    } else {
                        buggy_client.insert(addr_clone, 1);
                    }
                }
            }
        });
    }
    pool.join();
    Ok(())
}

#[derive(Debug)]
pub struct LoggingServer {
    config: Arc<Mutex<Config>>,
    thr: Option<JoinHandle<()>>,
}

impl LoggingServer {
    pub fn new(level: u8, address: String, tx: Sender<MessageType>) -> Result<Self, Error> {
        let config = Arc::new(Mutex::new(Config::new(level, address)));
        Ok(Self {
            config: config.clone(),
            thr: Some(
                thread::Builder
                    ::new()
                    .name("LoggingServer".to_string())
                    .spawn(move || {
                        if let Err(err) = logger_thread(config, tx) {
                            eprintln!("{err:?}");
                        }
                    })?
            ),
        })
    }

    pub fn shutdown(&mut self) -> Result<(), Error> {
        if let Some(thr) = self.thr.take() {
            // Send SHUTDOWN (255) to server socket
            let address = self.config.lock().unwrap().address.clone();
            let mut stream = TcpStream::connect(&address)?;
            stream.write_all(&[255u8, 255u8, 255u8])?;
            thr.join().map_err(|e|
                Error::new(ErrorKind::Other, e.downcast_ref::<&str>().unwrap().to_string())
            )
        } else {
            Ok(())
        }
    }

    pub fn set_level(&mut self, level: u8) {
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
}
