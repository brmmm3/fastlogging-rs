use std::{
    collections::HashMap,
    fmt,
    io::{Error, ErrorKind, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    path::PathBuf,
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use flume::{bounded, Sender};
use regex::Regex;
use ring::aead::{self, BoundKey};

use crate::{def::LoggingTypeEnum, LoggingError};

use super::{def::NetConfig, EncryptionMethod, NonceGenerator};

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub(crate) level: u8,
    pub(crate) address: String,
    pub(crate) port: u16,
    pub(crate) key: EncryptionMethod,
    pub(crate) port_file: Option<PathBuf>,
}

impl ServerConfig {
    pub fn new<S: Into<String>>(level: u8, address: S, key: EncryptionMethod) -> Self {
        let address: String = address.into();
        // If port is missing (default value is 0) or 0 then the used port will be chosen by the OS.
        let (address, port) = if address.contains(':') {
            let mut address_split = address.split(':');
            (
                address_split.next().unwrap().to_string(),
                address_split.last().unwrap().parse::<u16>().unwrap(),
            )
        } else {
            (address, 0)
        };
        Self {
            level,
            address,
            port,
            key,
            port_file: None,
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

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn read(
    stream: &mut TcpStream,
    buffer: &mut [u8],
    read_max: usize,
) -> Result<usize, std::io::Error> {
    let mut bytes_read = 0;
    loop {
        let cnt = match stream.read(&mut buffer[bytes_read..read_max]) {
            Ok(s) => s,
            Err(e) => {
                if e.kind() != ErrorKind::WouldBlock && e.kind() != ErrorKind::TimedOut {
                    eprintln!("read ERROR: {e:?}");
                }
                return Err(e);
            }
        };
        bytes_read += cnt;
        if cnt == 0 {
            stream.peer_addr()?;
            thread::sleep(Duration::from_millis(10));
        }
        if bytes_read >= read_max {
            break;
        }
    }
    Ok(bytes_read)
}

fn handle_client(
    config: Arc<Mutex<NetConfig>>,
    stream: &mut TcpStream,
    tx: Sender<LoggingTypeEnum>,
    stop: Arc<AtomicBool>,
    stop_server: Arc<AtomicBool>,
) -> Result<bool, LoggingError> {
    let perr_addr = stream.peer_addr().unwrap().to_string();
    let mut domain_buffer = [0u8; 256];
    let mut buffer = [0u8; 4352];
    let mut authenticated = false;
    let mut config_level = config.lock().unwrap().level;
    let mut debug = config.lock().unwrap().debug;
    if debug > 0 {
        println!("{} handle_client BEGIN", process::id());
    }
    loop {
        if stop.load(Ordering::Relaxed) || stop_server.load(Ordering::Relaxed) {
            break;
        }
        if let Ok(ref config) = config.lock() {
            config_level = config.level;
            debug = config.debug;
        }
        if debug > 1 {
            println!(
                "{} handle_client: WAIT {:?}",
                process::id(),
                stream.peer_addr()
            );
        }
        if let Err(err) = read(stream, &mut buffer, 4) {
            if err.kind() == ErrorKind::WouldBlock || err.kind() != ErrorKind::TimedOut {
                continue;
            }
            eprintln!(
                "{} handle_client: ERROR {err:?} {:?}",
                process::id(),
                &buffer[..4]
            );
            break;
        }
        let message_size = (buffer[0] as usize) | ((buffer[1] as usize) << 8);
        if message_size > buffer.len() {
            // Exit if received data is too big
            if message_size < 0xfffe {
                Err(LoggingError::RecvError(format!(
                    "Receive size {message_size} is too big"
                )))?;
            } else if message_size == 0xffff {
                // Exit server
                return Ok(true);
            }
            // Exit just this client
            return Ok(false);
        }
        if !authenticated {
            //println!("handle_client: AUTHENTICATE");
            // If channel is unencrypted then an AUTH_KEY is required first.
            // Wait up to 5 seconds for auth key.
            read(stream, &mut buffer, message_size)?;
            let key: Vec<u8> = config.lock().unwrap().key.key_cloned().unwrap();
            /*println!(
                "AUTH_KEY: {} {message_size}\n{:?}\n{:?}",
                key.len(),
                key,
                &buffer[..message_size]
            );*/
            if key.len() != message_size || !key.starts_with(&buffer[..message_size]) {
                Err(LoggingError::RecvError("Invalid auth key".to_string()))?;
            }
            if debug > 1 {
                println!("{} handle_client: AUTHENTICATED", process::id());
            }
            authenticated = true;
            continue;
        }
        let msg_level = buffer[2];
        let domain_size = buffer[3] as usize;
        let domain_data = &mut domain_buffer[..domain_size];
        stream.read_exact(domain_data)?;
        let message_data = &mut buffer[..message_size];
        stream.read_exact(message_data)?;
        if msg_level >= config_level {
            let domain = std::str::from_utf8(domain_data).unwrap().to_string();
            let message = format!(
                "{perr_addr}: {}",
                std::str::from_utf8(message_data).unwrap()
            );
            if debug > 2 {
                println!(
                    "{} handle_client: MESSAGE {domain}: {message:?}",
                    process::id()
                );
            }
            tx.send(LoggingTypeEnum::MessageRemote((msg_level, domain, message)))?;
        }
    }
    if debug > 0 {
        println!(
            "{} handle_client: FINISHED {:?}",
            process::id(),
            stream.peer_addr()
        );
    }
    Ok(false)
}

fn handle_encrypted_client(
    config: Arc<Mutex<NetConfig>>,
    stream: &mut TcpStream,
    tx: Sender<LoggingTypeEnum>,
    stop: Arc<AtomicBool>,
    stop_server: Arc<AtomicBool>,
) -> Result<bool, LoggingError> {
    //println!("handle_encrypted_client");
    let perr_addr = stream.peer_addr().unwrap().to_string();
    let mut domain_buffer = [0u8; 512];
    let mut buffer = [0u8; 4352];
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut key = aead::OpeningKey::new(
        aead::UnboundKey::new(
            &aead::AES_256_GCM,
            config.lock().unwrap().key.key().unwrap(),
        )
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?,
        NonceGenerator::new(),
    );
    let seal = aead::Aad::from(config.lock().unwrap().seal.clone());
    let mut config_level = config.lock().unwrap().level;
    let mut debug = config.lock().unwrap().debug;
    loop {
        if stop.load(Ordering::Relaxed) || stop_server.load(Ordering::Relaxed) {
            break;
        }
        if let Ok(ref config) = config.lock() {
            config_level = config.level;
            debug = config.debug;
        }
        if debug > 1 {
            println!("handle_encrypted_client: WAIT");
        }
        if let Err(err) = stream.read_exact(&mut buffer[..3]) {
            if err.kind() == ErrorKind::WouldBlock {
                continue;
            }
        }
        let size = (buffer[0] as usize) | ((buffer[1] as usize) << 8);
        if size > buffer.len() {
            // Exit if received data is too big
            if size < 0xffff {
                Err(LoggingError::RecvError(format!(
                    "Receive size {size} is too big"
                )))?;
            }
            return Ok(true);
        }
        let msg_level = buffer[2];
        let domain_size = buffer[3] as usize;
        let domain_data = &mut domain_buffer[..domain_size];
        stream.read_exact(domain_data)?;
        let message_data = &mut buffer[..size];
        stream.read_exact(message_data)?;
        if msg_level >= config_level {
            let _ = key
                .open_in_place(seal.clone(), domain_data)
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
            let _ = key
                .open_in_place(seal.clone(), message_data)
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
            let domain = std::str::from_utf8(domain_data).unwrap().to_string();
            let message = format!(
                "{perr_addr}: {}",
                std::str::from_utf8(message_data).unwrap()
            );
            if debug > 2 {
                println!("handle_encrypted_client: MESSAGE {domain}: {message:?}");
            }
            tx.send(LoggingTypeEnum::MessageRemote((msg_level, domain, message)))?;
        }
    }
    Ok(false)
}

fn server_thread(
    config: Arc<Mutex<NetConfig>>,
    listener: TcpListener,
    tx: Sender<LoggingTypeEnum>,
    stop: Arc<AtomicBool>,
) -> Result<(), LoggingError> {
    let mut debug = config.lock().unwrap().debug;
    let pool = threadpool::ThreadPool::new(num_cpus::get());
    let clients: Arc<Mutex<HashMap<std::net::SocketAddr, TcpStream>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let buggy_clients: Arc<Mutex<HashMap<std::net::SocketAddr, usize>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let stop_server = Arc::new(AtomicBool::new(false));
    if debug > 0 {
        println!("{} server_thread STARTED", process::id());
    }
    for stream in listener.incoming() {
        debug = config.lock().unwrap().debug;
        if stop.load(Ordering::Relaxed) || stop_server.load(Ordering::Relaxed) {
            if debug > 0 {
                println!("{} server_thread: EXIT FOR LOOP", process::id());
            }
            break;
        }
        // Message format: [size:u16, level:u8, data]
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("server_thread: TcpStream: {e:?}");
                continue;
            }
        };
        //stream.set_read_timeout(Some(Duration::from_millis(100)))?;
        stream.set_nodelay(true)?;
        let addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                eprintln!("server_thread: SocketAddr: {e:?}");
                continue;
            }
        };
        if debug > 0 {
            println!(
                "{} server_thread: CLIENT {} CONNECTED {addr:?}",
                process::id(),
                clients.lock().unwrap().len() + 1
            );
        }
        // Clients have are allowed to produce 3 errors. In case of more errors they will be ignored.
        if *buggy_clients.lock().unwrap().get(&addr).unwrap_or(&0) > 3 {
            continue;
        }
        let config = config.clone();
        let tx = tx.clone();
        let buggy_clients = buggy_clients.clone();
        let stop = stop.clone();
        let stop_server = stop_server.clone();
        clients.lock().unwrap().insert(addr, stream.try_clone()?);
        let clients = clients.clone();
        pool.execute(move || {
            let is_encrypted = config.lock().unwrap().key.is_encrypted();
            if debug > 0 {
                println!(
                    "{} server_thread: CLIENT {addr:?} ENCRYPTED {is_encrypted}",
                    process::id()
                );
            }
            let result = match is_encrypted {
                false => handle_client(config, &mut stream, tx, stop, stop_server.clone()),
                true => handle_encrypted_client(config, &mut stream, tx, stop, stop_server.clone()),
            };
            if debug > 0 {
                println!(
                    "{} server_thread: CLIENT {} DISCONNECTED {addr:?}",
                    process::id(),
                    clients.lock().unwrap().len()
                );
            }
            clients.lock().unwrap().remove(&addr);
            match result {
                Ok(stop) => {
                    if stop {
                        stop_server.store(true, Ordering::Relaxed)
                    }
                }
                Err(err) => {
                    eprintln!("server_thread: Error with client {stream:?}: {err:?}");
                    if let Ok(mut buggy_client) = buggy_clients.lock() {
                        if let Some(c) = buggy_client.get_mut(&addr) {
                            *c += 1;
                        } else {
                            buggy_client.insert(addr, 1);
                        }
                    }
                }
            }
        });
    }
    if debug > 0 {
        println!(
            "{} server_thread: JOIN CLIENTS={}",
            process::id(),
            clients.lock().unwrap().len()
        );
    }
    stop_server.store(true, Ordering::Relaxed);
    for (addr, stream) in clients.lock().unwrap().drain() {
        if debug > 0 {
            println!("{} server_thread: SHUTDOWN CLIENT {addr:?}", process::id());
        }
        if let Err(err) = stream.shutdown(Shutdown::Both) {
            eprintln!(
                "{} server_thread: SHUTDOWN CLIENT {addr:?} FAILED: {err:?}",
                process::id()
            );
        }
    }
    pool.join();
    if debug > 0 {
        println!("{} server_thread: FINISHED", process::id());
    }
    Ok(())
}

#[derive(Debug)]
pub struct LoggingServer {
    pub(crate) config: Arc<Mutex<NetConfig>>,
    thr: Option<JoinHandle<()>>,
    pub(crate) debug: u8,
}

impl LoggingServer {
    pub fn new(
        config: ServerConfig,
        tx: Sender<LoggingTypeEnum>,
        stop: Arc<AtomicBool>,
    ) -> Result<Self, LoggingError> {
        let config = Arc::new(Mutex::new(NetConfig::new(
            config.level,
            config.address,
            config.port,
            config.key,
        )?));
        let config_clone = config.clone();
        let (tx_started, rx_started) = bounded(1);
        let thr = thread::Builder::new()
            .name("LoggingServer".to_string())
            .spawn(move || {
                let listener = {
                    let mut config_clone = config_clone.lock().unwrap();
                    let listener = match TcpListener::bind(format!(
                        "{}:{}",
                        config_clone.address, config_clone.port
                    )) {
                        Ok(v) => v,
                        Err(err) => {
                            eprintln!(
                                "LOGSRV: server_thread: Failed to bind {}:{}: {err:?}",
                                config_clone.address, config_clone.port
                            );
                            return;
                        }
                    };
                    if config_clone.port == 0 {
                        config_clone.port = match listener.local_addr() {
                            Ok(v) => v,
                            Err(err) => {
                                eprintln!(
                                    "LOGSRV: server_thread: Failed to get local address: {err:?}"
                                );
                                return;
                            }
                        }
                        .port();
                    }
                    listener
                };
                tx_started.send(1).expect("Failed to send started signal");
                if let Err(err) = server_thread(config_clone.clone(), listener, tx, stop) {
                    eprintln!("LOGSRV: server_thread: {err:?}");
                }
                //println!("SERVER FIN {}", process::id());
                if let Some(ref port_file) = config_clone.lock().unwrap().port_file {
                    //println!("Remove PORT FILE {port_file:?}");
                    if let Err(err) = std::fs::remove_file(port_file) {
                        eprintln!("LOGSRV: Failed to remove port file {port_file:?}: {err:?}");
                    }
                }
            })?;
        // Wait for thread started
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
            thr: Some(thr),
            debug: 0,
        })
    }

    pub fn shutdown(&mut self) -> Result<(), LoggingError> {
        if let Some(thr) = self.thr.take() {
            // Send SHUTDOWN (255) to server socket
            let stop_cmd = [255, 255, 255, 255];
            loop {
                let mut stream = TcpStream::connect(self.config.lock().unwrap().get_address())?;
                stream.write_all(&stop_cmd)?;
                stream.flush()?;
                //stream.shutdown(Shutdown::Both)?;
                thread::sleep(Duration::from_millis(10));
                if thr.is_finished() {
                    break;
                }
            }
            thr.join().map_err(|e| {
                LoggingError::JoinError(
                    "ServerWriter".to_string(),
                    e.downcast_ref::<&str>().unwrap().to_string(),
                )
            })
        } else {
            Ok(())
        }
    }

    pub fn enable(&self) {
        self.config.lock().unwrap().enabled = true;
    }

    pub fn disable(&self) {
        self.config.lock().unwrap().enabled = false;
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.config.lock().unwrap().enabled = enabled;
    }

    pub fn set_level(&mut self, level: u8) {
        self.config.lock().unwrap().level = level;
    }

    pub fn set_domain_filter(&self, domain_filter: Option<String>) -> Result<(), regex::Error> {
        if let Some(ref message) = domain_filter {
            Regex::new(message)?;
        }
        self.config.lock().unwrap().domain_filter = domain_filter;
        Ok(())
    }

    pub fn set_message_filter(&self, message_filter: Option<String>) -> Result<(), regex::Error> {
        if let Some(ref message) = message_filter {
            Regex::new(message)?;
        }
        self.config.lock().unwrap().message_filter = message_filter;
        Ok(())
    }

    pub fn set_encryption(&mut self, key: EncryptionMethod) -> Result<(), LoggingError> {
        self.config
            .lock()
            .unwrap()
            .set_encryption(key.clone())
            .map_err(|e| {
                LoggingError::InvalidEncryption("LoggingServer".to_string(), key, e.to_string())
            })
    }
}
