use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::path::{Path, PathBuf};
use std::process;
use std::{env, fs};

use once_cell::sync::Lazy;
use parking_lot::RwLock;

use crate::config::{ConfigFile, FileMerge, default_config_file};
use crate::console::ConsoleWriterConfig;
use crate::net::{AUTH_KEY, ClientWriterConfig, EncryptionMethod, ServerConfig};
use crate::{
    ExtConfig, LevelSyms, Logger, Logging, LoggingError, NOTSET, WriterConfigEnum, WriterEnum,
    WriterTypeEnum, getppid,
};

pub static PARENT_LOGGER_ADDRESS: Lazy<RwLock<Option<(u32, ClientWriterConfig)>>> =
    Lazy::new(|| RwLock::new(None));

pub static ROOT_LOGGER: Lazy<RwLock<Logging>> = Lazy::new(|| {
    fn create_default_logger(config_file: Option<PathBuf>) -> Logging {
        let mut logging = Logging::new(NOTSET, "root", None, None, config_file).unwrap();
        if let Err(err) =
            logging.set_root_writer_config(&WriterConfigEnum::Server(ServerConfig::new(
                NOTSET,
                "127.0.0.1",
                EncryptionMethod::AuthKey(AUTH_KEY.to_vec()),
            )))
        {
            eprintln!("Failed to create Root ServerLogger: {err:?}");
        }
        logging.drop = false;
        logging
    }

    fn get_port_file(pid: u32) -> PathBuf {
        let mut temp_dir = env::temp_dir();
        temp_dir.push(format!("fastlogging_rs_server_port.{pid}"));
        temp_dir
    }

    fn get_parent_server_address() -> Result<Option<(String, EncryptionMethod)>, LoggingError> {
        let port_file = get_port_file(getppid());
        if port_file.exists() {
            // Parent process exists. Check if logging server is reachable.
            let mut buffer = Vec::new();
            if fs::File::open(port_file)?.read_to_end(&mut buffer)? >= 4 {
                let port = u16::from_le_bytes(buffer[..2].try_into().unwrap());
                let address = format!("127.0.0.1:{port}");
                let encryption = match buffer[2] {
                    0 => EncryptionMethod::NONE,
                    1 => EncryptionMethod::AuthKey(buffer[3..].to_vec()),
                    2 => EncryptionMethod::AES(buffer[3..].to_vec()),
                    _ => {
                        return Err(LoggingError::InvalidValue(format!(
                            "Invalid encryption type {}",
                            buffer[2]
                        )));
                    }
                };
                if let Ok(mut stream) = TcpStream::connect(&address) {
                    let buffer = vec![0xfeu8, 0xffu8, 0xffu8, 0xffu8];
                    stream.write_all(&buffer)?;
                    stream.flush()?;
                    stream.shutdown(Shutdown::Both)?;
                    return Ok(Some((address, encryption)));
                }
            }
        }
        Ok(None)
    }

    fn setup_logging() -> Result<Logging, LoggingError> {
        // Check if parent process with fastlogging instance exists.
        let mut logging = create_default_logger(None);
        if let Ok(server) = logging.get_server_config(0) {
            let port_file = get_port_file(process::id());
            // Server config above is just a copy. So we need to access the original directly.
            logging
                .instance
                .read()
                .get_server_config(0)
                .unwrap()
                .port_file = Some(port_file.clone());
            let mut file = fs::File::create(port_file)?;
            file.write_all(&u16::to_le_bytes(server.port))?;
            file.write_all(&logging.get_server_auth_key().to_bytes())?;
        }
        if let Some((server_address, encryption)) = get_parent_server_address()? {
            // Connect to parent server port
            let mut client = ClientWriterConfig::new(NOTSET, server_address, encryption);
            client.debug = logging.instance.read().debug;
            *PARENT_LOGGER_ADDRESS.write() = Some((getppid(), client.clone()));
            logging.add_writer_config(&WriterConfigEnum::Client(client))?;
        } else {
            // If default config file exists, then use this configuration. Else create default console logger.
            let default_file_config = default_config_file();
            if default_file_config.1.is_empty() {
                logging.add_writer_config(&WriterConfigEnum::Console(ConsoleWriterConfig::new(
                    NOTSET, false,
                )))?;
            } else {
                let mut config_file = ConfigFile::new();
                config_file.load(&default_file_config.0)?;
                let mut instance = logging.instance.write();
                config_file.merge(&mut instance, FileMerge::MergeReplace)?;
            }
        }
        Ok(logging)
    }

    let logging = match setup_logging() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to setup default logger: {e}");
            create_default_logger(None)
        }
    };
    RwLock::new(logging)
});

/// Initialize ROOT logger.
pub fn root_init() {
    drop(ROOT_LOGGER.write());
}

/// Shutdown fastlogging.
pub fn shutdown(now: bool) -> Result<(), LoggingError> {
    ROOT_LOGGER.write().shutdown(now)
}

/// Set log level for writer with ID `wid` to `level`.
pub fn set_level(wid: usize, level: u8) -> Result<(), LoggingError> {
    ROOT_LOGGER.write().set_level(wid, level)
}

pub fn set_root_level(level: u8) {
    ROOT_LOGGER.write().instance.write().level = level;
    ROOT_LOGGER.write().level = level;
}

pub fn get_root_level() -> u8 {
    ROOT_LOGGER.read().level
}

/// Set logging domain.
pub fn set_domain<S: Into<String>>(domain: S) {
    ROOT_LOGGER.write().set_domain(&domain.into())
}

pub fn set_level2sym(level2sym: &LevelSyms) {
    ROOT_LOGGER.write().set_level2sym(level2sym)
}

/// Set extended configuration.
pub fn set_ext_config(ext_config: &ExtConfig) {
    ROOT_LOGGER.write().set_ext_config(ext_config)
}

/// Add fastlogging logger.
pub fn add_logger(logger: &mut Logger) {
    ROOT_LOGGER.write().add_logger(logger)
}

/// Remove fastlogging logger.
pub fn remove_logger(logger: &mut Logger) {
    ROOT_LOGGER.write().remove_logger(logger)
}

pub fn set_root_writer_config(config: &WriterConfigEnum) -> Result<(), LoggingError> {
    ROOT_LOGGER.write().set_root_writer_config(config)
}

pub fn set_root_writer(writer: WriterEnum) -> Result<(), LoggingError> {
    ROOT_LOGGER.write().set_root_writer(writer)
}

pub fn add_writer_config(config: &WriterConfigEnum) -> Result<usize, LoggingError> {
    ROOT_LOGGER.write().add_writer_config(config)
}

pub fn add_writer(writer: WriterEnum) -> usize {
    ROOT_LOGGER.write().add_writer(writer)
}

pub fn remove_writer(wid: usize) -> Option<WriterEnum> {
    ROOT_LOGGER.write().remove_writer(wid)
}

pub fn add_writer_configs(configs: Vec<WriterConfigEnum>) -> Result<Vec<usize>, LoggingError> {
    ROOT_LOGGER.write().add_writer_configs(configs)
}

/// Add list of writers. `writers` contains list of writers to add.
pub fn add_writers(writers: Vec<WriterEnum>) -> Vec<usize> {
    ROOT_LOGGER.write().add_writers(writers)
}

/// Remove list of writer. `wids` contains list of writer IDs. The return value is a list of removed writers.
pub fn remove_writers(wids: Option<Vec<usize>>) -> Vec<WriterEnum> {
    ROOT_LOGGER.write().remove_writers(wids)
}

/// Enable writer with ID `wid`.
pub fn enable(wid: usize) -> Result<(), LoggingError> {
    ROOT_LOGGER.write().enable(wid)
}

/// Disable writer with ID `wid`.
pub fn disable(wid: usize) -> Result<(), LoggingError> {
    ROOT_LOGGER.write().disable(wid)
}

/// Enable all writers with type `typ`. See [WriterTypeEnum]
pub fn enable_type(typ: WriterTypeEnum) -> Result<(), LoggingError> {
    ROOT_LOGGER.write().enable_type(typ)
}

/// Disable all writers with type `typ`. See [WriterTypeEnum]
pub fn disable_type(typ: WriterTypeEnum) -> Result<(), LoggingError> {
    ROOT_LOGGER.write().disable_type(typ)
}

/// Syncronize all writers with types contained in `types`. Wait for maximum time `timeout` seconds.
pub fn sync(types: Vec<WriterTypeEnum>, timeout: f64) -> Result<(), LoggingError> {
    ROOT_LOGGER.read().sync(types, timeout)
}

/// Syncronize all writers. Wait for maximum time `timeout` seconds.
pub fn sync_all(timeout: f64) -> Result<(), LoggingError> {
    ROOT_LOGGER.read().sync_all(timeout)
}

/// Rotate a single log file `path` or all log files with `path` is `None`.
pub fn rotate(path: Option<PathBuf>) -> Result<(), LoggingError> {
    ROOT_LOGGER.read().rotate(path)
}

// Network

pub fn set_encryption(wid: usize, key: EncryptionMethod) -> Result<(), LoggingError> {
    ROOT_LOGGER.write().set_encryption(wid, key)
}

// Config

/// Set debug mode.
pub fn set_debug(debug: u8) {
    let logger = ROOT_LOGGER.read();
    let mut config = logger.instance.write();
    config.debug = debug;
    for writer in config.writers.values_mut() {
        match writer {
            WriterEnum::Console(console_writer) => console_writer.debug = debug,
            WriterEnum::File(file_writer) => file_writer.debug = debug,
            WriterEnum::Client(client_writer) => client_writer.debug = debug,
            WriterEnum::Server(server_writer) => server_writer.debug = debug,
            WriterEnum::Callback(callback_writer) => callback_writer.debug = debug,
            WriterEnum::Syslog(syslog_writer) => syslog_writer.debug = debug,
            _ => {}
        }
    }
}

pub fn get_writer_config(wid: usize) -> Option<WriterConfigEnum> {
    ROOT_LOGGER.read().get_writer_config(wid)
}

pub fn get_writer_configs() -> HashMap<usize, WriterConfigEnum> {
    ROOT_LOGGER.read().get_writer_configs()
}

pub fn get_server_config(wid: usize) -> Result<ServerConfig, LoggingError> {
    ROOT_LOGGER.read().get_server_config(wid)
}

pub fn get_server_configs() -> HashMap<usize, ServerConfig> {
    ROOT_LOGGER.read().get_server_configs()
}

pub fn get_root_server_address_port() -> Option<String> {
    ROOT_LOGGER.read().get_root_server_address_port()
}

pub fn get_server_addresses_ports() -> HashMap<usize, String> {
    ROOT_LOGGER.read().get_server_addresses_ports()
}

pub fn get_server_addresses() -> HashMap<usize, String> {
    ROOT_LOGGER.read().get_server_addresses()
}

pub fn get_server_ports() -> HashMap<usize, u16> {
    ROOT_LOGGER.read().get_server_ports()
}

pub fn get_server_auth_key() -> EncryptionMethod {
    ROOT_LOGGER.read().get_server_auth_key()
}

/// Get fastlogging configuration as string.
pub fn get_config_string() -> String {
    ROOT_LOGGER.read().get_config_string()
}

/// Save fastlogging configuration to file `path`.
pub fn save_config(path: Option<&Path>) -> Result<(), LoggingError> {
    ROOT_LOGGER.write().save_config(path)
}

/// Get process ID of parent process.
pub fn get_parent_pid() -> Option<u32> {
    // Initialize root logger is not already done.
    let _logger = ROOT_LOGGER.read();
    PARENT_LOGGER_ADDRESS.read().as_ref().map(|v| v.0)
}

/// Get IP address of parent process LoggingServer.
pub fn get_parent_client_writer_config() -> Option<ClientWriterConfig> {
    // Initialize root logger is not already done.
    let _logger = ROOT_LOGGER.read();
    PARENT_LOGGER_ADDRESS.read().as_ref().map(|v| v.1.clone())
}

/// Get process ID of parent process and IP address of parent process LoggingServer.
pub fn get_parent_pid_client_writer_config() -> Option<(u32, ClientWriterConfig)> {
    // Initialize root logger is not already done.
    let _logger = ROOT_LOGGER.read();
    PARENT_LOGGER_ADDRESS.read().clone()
}

// Logging methods

/// Log TRACE level message.
pub fn trace<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.read().trace(message)
}

/// Log DEBUG level message.
pub fn debug<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.read().debug(message)
}

/// Log INFO level message.
pub fn info<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.read().info(message)
}

/// Log SUCCESS level message.
pub fn success<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.read().success(message)
}

/// Log WARNING level message.
pub fn warning<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.read().warning(message)
}

/// Log ERROR level message.
pub fn error<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.read().error(message)
}

/// Log CRITICAL level message.
pub fn critical<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.read().critical(message)
}

/// Log FATAL level message.
pub fn fatal<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.read().fatal(message)
}

/// Log EXCEPTION level message.
pub fn exception<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.read().exception(message)
}
