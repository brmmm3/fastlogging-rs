use std::{
    collections::HashMap,
    fmt,
    io::{ Error, ErrorKind, Read, Write },
    net::{ TcpListener, TcpStream },
    sync::{ Arc, Mutex },
    thread::{ self, JoinHandle },
    time::Duration,
};

use flume::Sender;
use ring::aead::{ self, BoundKey };

use crate::def::LoggingTypeEnum;

use super::{ def::{ NetConfig, AUTH_KEY }, NonceGenerator };

#[derive(Debug, Clone)]
pub struct ServerConfig {
    level: u8,
    address: String,
    key: Option<Vec<u8>>,
}

impl ServerConfig {
    pub fn new(level: u8, address: String, key: Option<Vec<u8>>) -> Self {
        Self { level, address, key }
    }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn handle_client(
    config: Arc<Mutex<NetConfig>>,
    mut stream: TcpStream,
    tx: Sender<LoggingTypeEnum>,
    stop: Arc<Mutex<bool>>
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut buffer = [0u8; 4096];
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    // If channel is unencrypted then an AUTH_KEY is required first.
    // Wait up to 5 seconds for auth key.
    stream.read_exact(&mut buffer[0..AUTH_KEY.len()])?;
    if !AUTH_KEY.starts_with(&buffer[0..AUTH_KEY.len()]) {
        Err("Invalid auth key".to_string())?;
    }
    loop {
        if *stop.lock().unwrap() {
            break;
        }
        if let Err(err) = stream.read_exact(&mut buffer[0..2]) {
            if err.kind() == ErrorKind::TimedOut {
                continue;
            }
        }
        let size = (buffer[0] as usize) | ((buffer[1] as usize) << 8);
        if size > buffer.len() {
            //
            return Ok(true);
        }
        let msg_level = buffer[2];
        let data = &mut buffer[..size];
        stream.read_exact(data)?;
        let message = std::str::from_utf8(data).unwrap().to_string();
        if let Ok(ref mut config) = config.lock() {
            if msg_level >= config.level {
                tx.send(LoggingTypeEnum::Message((msg_level, message)))?;
            }
        }
    }
    Ok(false)
}

fn handle_encrypted_client(
    config: Arc<Mutex<NetConfig>>,
    mut stream: TcpStream,
    tx: Sender<LoggingTypeEnum>,
    stop: Arc<Mutex<bool>>
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut buffer = [0u8; 4096];
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut key = aead::OpeningKey::new(
        aead::UnboundKey
            ::new(&aead::AES_256_GCM, config.lock().unwrap().key.as_deref().unwrap())
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?,
        NonceGenerator::new()
    );
    let seal = aead::Aad::from(config.lock().unwrap().seal.clone());
    loop {
        if *stop.lock().unwrap() {
            break;
        }
        if let Err(err) = stream.read_exact(&mut buffer[0..2]) {
            if err.kind() == ErrorKind::TimedOut {
                continue;
            }
        }
        let size = (buffer[0] as usize) | ((buffer[1] as usize) << 8);
        if size > buffer.len() {
            //
            return Ok(true);
        }
        let msg_level = buffer[2];
        let data = &mut buffer[..size];
        stream.read_exact(data)?;
        if msg_level >= config.lock().unwrap().level {
            let _ = key
                .open_in_place(seal.clone(), data)
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
            let message = std::str::from_utf8(data).unwrap().to_string();
            tx.send(LoggingTypeEnum::Message((msg_level, message)))?;
        }
    }
    Ok(false)
}

fn server_thread(
    config: Arc<Mutex<NetConfig>>,
    tx: Sender<LoggingTypeEnum>,
    stop: Arc<Mutex<bool>>
) -> Result<(), Box<dyn std::error::Error>> {
    let address = config.lock().unwrap().address.clone();
    let listener = TcpListener::bind(address).unwrap();
    let pool = threadpool::ThreadPool::new(num_cpus::get());
    let buggy_clients: Arc<Mutex<HashMap<std::net::SocketAddr, usize>>> = Arc::new(
        Mutex::new(HashMap::new())
    );

    for stream in listener.incoming() {
        if *stop.lock().unwrap() {
            break;
        }
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
                eprint!("server_thread: {e:?}");
                continue;
            }
        };
        // Clients have are allowed to produce 3 errors. In case of more errors they will be ignored.
        if *buggy_clients.lock().unwrap().get(&addr).unwrap_or(&0) > 3 {
            continue;
        }
        let config = config.clone();
        let tx = tx.clone();
        let buggy_clients = buggy_clients.clone();
        let stop = stop.clone();
        pool.execute(move || {
            let is_encrypted = config.lock().unwrap().key.is_some();
            if
                let Err(err) = (match is_encrypted {
                    false => handle_client(config, stream, tx, stop),
                    true => handle_encrypted_client(config, stream, tx, stop),
                })
            {
                eprint!("server_thread: Error with client {addr}: {err:?}");
                if let Ok(mut buggy_client) = buggy_clients.lock() {
                    if let Some(c) = buggy_client.get_mut(&addr) {
                        *c += 1;
                    } else {
                        buggy_client.insert(addr, 1);
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
    pub(crate) config: Arc<Mutex<NetConfig>>,
    thr: Option<JoinHandle<()>>,
}

impl LoggingServer {
    pub fn new(
        config: ServerConfig,
        tx: Sender<LoggingTypeEnum>,
        stop: Arc<Mutex<bool>>
    ) -> Result<Self, Error> {
        let config = Arc::new(
            Mutex::new(NetConfig::new(config.level, config.address, config.key)?)
        );
        Ok(Self {
            config: config.clone(),
            thr: Some(
                thread::Builder
                    ::new()
                    .name("LoggingServer".to_string())
                    .spawn(move || {
                        if let Err(err) = server_thread(config, tx, stop) {
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
            let mut stream = TcpStream::connect(address)?;
            stream.write_all(&[255u8, 255u8, 255u8])?;
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
}
