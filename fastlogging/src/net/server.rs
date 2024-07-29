use std::{
    collections::HashMap,
    fmt,
    io::{Error, ErrorKind, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    process,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use flume::{bounded, Sender};
use ring::aead::{self, BoundKey};

use crate::def::LoggingTypeEnum;

use super::{def::NetConfig, EncryptionMethod, NonceGenerator};

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub level: u8,
    pub address: String,
    pub port: u16,
    pub key: EncryptionMethod,
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
                eprintln!("read ERROR: {e:?}");
                return Err(e);
            }
        };
        bytes_read += cnt;
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
    stop: Arc<Mutex<bool>>,
    stop_server: Arc<AtomicBool>,
) -> Result<bool, Box<dyn std::error::Error>> {
    //println!("handle_client");
    let perr_addr = stream.peer_addr().unwrap().to_string();
    let mut buffer = [0u8; 4352];
    let mut authenticated = false;
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut config_level = config.lock().unwrap().level;
    let mut debug = config.lock().unwrap().debug;
    loop {
        if *stop.lock().unwrap() || stop_server.load(Ordering::Relaxed) {
            break;
        }
        if let Ok(ref config) = config.lock() {
            config_level = config.level;
            debug = config.debug;
        }
        if debug > 1 {
            println!("handle_client: WAIT");
        }
        //println!("handle_client: WAIT");
        if let Err(err) = read(stream, &mut buffer, 3) {
            if err.kind() == ErrorKind::WouldBlock {
                continue;
            }
            println!("handle_client: ERROR {err:?} {:?}", &buffer[..3]);
            break;
        }
        let size = (buffer[0] as usize) | ((buffer[1] as usize) << 8);
        if size > buffer.len() {
            // Exit if received data is too big
            if size < 0xffff {
                Err(format!("Receive size {size} is too big"))?;
            }
            return Ok(true);
        }
        if !authenticated {
            //println!("handle_client: AUTHENTICATE");
            // If channel is unencrypted then an AUTH_KEY is required first.
            // Wait up to 5 seconds for auth key.
            read(stream, &mut buffer, size)?;
            let key: Vec<u8> = config.lock().unwrap().key.key_cloned().unwrap();
            /*println!(
                "AUTH_KEY: {} {size}\n{:?}\n{:?}",
                key.len(),
                key,
                &buffer[..size]
            );*/
            if key.len() != size || !key.starts_with(&buffer[..size]) {
                Err("Invalid auth key".to_string())?;
            }
            if debug > 1 {
                println!("handle_client: AUTHENTICATED");
            }
            authenticated = true;
            continue;
        }
        let msg_level = buffer[2];
        read(stream, &mut buffer, size)?;
        if msg_level >= config_level {
            let message = format!(
                "{perr_addr}: {}",
                std::str::from_utf8(&buffer[..size]).unwrap()
            );
            if debug > 2 {
                println!("handle_client: MESSAGE {message:?}");
            }
            tx.send(LoggingTypeEnum::MessageRemote((msg_level, message)))?;
        }
    }
    //println!("handle_client: FINISHED");
    Ok(false)
}

fn handle_encrypted_client(
    config: Arc<Mutex<NetConfig>>,
    stream: &mut TcpStream,
    tx: Sender<LoggingTypeEnum>,
    stop: Arc<Mutex<bool>>,
    stop_server: Arc<AtomicBool>,
) -> Result<bool, Box<dyn std::error::Error>> {
    //println!("handle_encrypted_client");
    let perr_addr = stream.peer_addr().unwrap().to_string();
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
        if *stop.lock().unwrap() || stop_server.load(Ordering::Relaxed) {
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
                Err(format!("Receive size {size} is too big"))?;
            }
            return Ok(true);
        }
        let msg_level = buffer[2];
        let data = &mut buffer[..size];
        stream.read_exact(data)?;
        if msg_level >= config_level {
            let _ = key
                .open_in_place(seal.clone(), data)
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
            let message = format!(
                "{perr_addr}: {}",
                std::str::from_utf8(&buffer[..size]).unwrap()
            );
            if debug > 2 {
                println!("handle_encrypted_client: MESSAGE {message:?}");
            }
            tx.send(LoggingTypeEnum::MessageRemote((msg_level, message)))?;
        }
    }
    Ok(false)
}

fn server_thread(
    config: Arc<Mutex<NetConfig>>,
    listener: TcpListener,
    tx: Sender<LoggingTypeEnum>,
    stop: Arc<Mutex<bool>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut debug = config.lock().unwrap().debug;
    let pool = threadpool::ThreadPool::new(num_cpus::get());
    let clients: Arc<Mutex<HashMap<std::net::SocketAddr, TcpStream>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let buggy_clients: Arc<Mutex<HashMap<std::net::SocketAddr, usize>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let stop_server = Arc::new(AtomicBool::new(false));
    if debug > 0 {
        println!("server_thread STARTED {}", process::id());
    }
    for stream in listener.incoming() {
        debug = config.lock().unwrap().debug;
        if *stop.lock().unwrap() || stop_server.load(Ordering::Relaxed) {
            //println!("server_thread STOPPING. Inform Clients");
            let stop_cmd = [255, 255, 255];
            for (_addr, mut stream) in clients.lock().unwrap().drain() {
                stream.write_all(&stop_cmd)?;
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
        let addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                eprintln!("server_thread: SocketAddr: {e:?}");
                continue;
            }
        };
        if debug > 0 {
            println!("server_thread: CLIENT CONNECTED {} {addr:?}", process::id());
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
                    "server_thread: CLIENT {} {addr:?} ENCRYPTED {is_encrypted}",
                    process::id()
                );
            }
            let result = match is_encrypted {
                false => handle_client(config, &mut stream, tx, stop, stop_server.clone()),
                true => handle_encrypted_client(config, &mut stream, tx, stop, stop_server.clone()),
            };
            if debug > 0 {
                println!(
                    "server_thread: CLIENT DISCONNECTED {} {addr:?}",
                    process::id()
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
        println!("server_thread: JOIN {}", process::id());
    }
    pool.join();
    if debug > 0 {
        println!("server_thread: FINISHED {}", process::id());
    }
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
        stop: Arc<Mutex<bool>>,
    ) -> Result<Self, Error> {
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
                if let Err(err) = server_thread(config_clone, listener, tx, stop) {
                    eprintln!("LOGSRV: server_thread: {err:?}");
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
        })
    }

    pub fn shutdown(&mut self) -> Result<(), Error> {
        if let Some(thr) = self.thr.take() {
            // Send SHUTDOWN (255) to server socket
            let stop_cmd = [255, 255, 255];
            loop {
                let mut stream = TcpStream::connect(self.config.lock().unwrap().get_address())?;
                stream.write_all(&stop_cmd)?;
                stream.shutdown(Shutdown::Both)?;
                thread::sleep(Duration::from_millis(10));
                if thr.is_finished() {
                    break;
                }
            }
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
}
