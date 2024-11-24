use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::Mutex;
use std::{env, fs};

use once_cell::sync::Lazy;

use crate::config::{default_config_file, ConfigFile, FileMerge};
use crate::console::ConsoleWriterConfig;
use crate::net::{ClientWriterConfig, EncryptionMethod, ServerConfig, AUTH_KEY};
use crate::{
    getppid, ExtConfig, LevelSyms, Logger, Logging, LoggingError, WriterConfigEnum, WriterEnum,
    WriterTypeEnum, NOTSET,
};

pub static PARENT_LOGGER_ADDRESS: Lazy<Mutex<Option<(u32, ClientWriterConfig)>>> =
    Lazy::new(|| Mutex::new(None));

pub static ROOT_LOGGER: Lazy<Mutex<Logging>> = Lazy::new(|| {
    fn create_default_logger(config_file: Option<PathBuf>) -> Logging {
        let mut logging = Logging::new(NOTSET, "root", vec![], None, config_file).unwrap();
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
                        )))
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
                .lock()
                .unwrap()
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
            client.debug = logging.instance.lock().unwrap().debug;
            *PARENT_LOGGER_ADDRESS.lock().unwrap() = Some((getppid(), client.clone()));
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
                let mut instance = logging.instance.lock().unwrap();
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
    Mutex::new(logging)
});

/// Initialize ROOT logger.
pub fn root_init() {
    drop(ROOT_LOGGER.lock().unwrap());
}

/// Shutdown fastlogging.
pub fn shutdown(now: bool) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().shutdown(now)
}

/// Set log level for writer with ID `wid` to `level`.
pub fn set_level(wid: usize, level: u8) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().set_level(wid, level)
}

/// Set logging domain.
pub fn set_domain<S: Into<String>>(domain: S) {
    ROOT_LOGGER.lock().unwrap().set_domain(&domain.into())
}

pub fn set_level2sym(level2sym: &LevelSyms) {
    ROOT_LOGGER.lock().unwrap().set_level2sym(level2sym)
}

/// Set extended configuration.
pub fn set_ext_config(ext_config: &ExtConfig) {
    ROOT_LOGGER.lock().unwrap().set_ext_config(ext_config)
}

/// Add fastlogging logger.
pub fn add_logger(logger: &mut Logger) {
    ROOT_LOGGER.lock().unwrap().add_logger(logger)
}

/// Remnove fastlogging logger.
pub fn remove_logger(logger: &mut Logger) {
    ROOT_LOGGER.lock().unwrap().remove_logger(logger)
}

pub fn set_root_writer_config(config: &WriterConfigEnum) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().set_root_writer_config(config)
}

pub fn set_root_writer(writer: WriterEnum) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().set_root_writer(writer)
}

pub fn add_writer_config(config: &WriterConfigEnum) -> Result<usize, LoggingError> {
    ROOT_LOGGER.lock().unwrap().add_writer_config(config)
}

pub fn add_writer(writer: WriterEnum) -> usize {
    ROOT_LOGGER.lock().unwrap().add_writer(writer)
}

pub fn remove_writer(wid: usize) -> Option<WriterEnum> {
    ROOT_LOGGER.lock().unwrap().remove_writer(wid)
}

pub fn add_writer_configs(configs: &[WriterConfigEnum]) -> Result<Vec<usize>, LoggingError> {
    ROOT_LOGGER.lock().unwrap().add_writer_configs(configs)
}

/// Add list of writers. `writers` contains list of writers to add.
pub fn add_writers(writers: Vec<WriterEnum>) -> Vec<usize> {
    ROOT_LOGGER.lock().unwrap().add_writers(writers)
}

/// Remove list of writer. `wids` contains list of writer IDs. The return value is a list of removed writers.
pub fn remove_writers(wids: Option<Vec<usize>>) -> Vec<WriterEnum> {
    ROOT_LOGGER.lock().unwrap().remove_writers(wids)
}

/// Enable writer with ID `wid`.
pub fn enable(wid: usize) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().enable(wid)
}

/// Disable writer with ID `wid`.
pub fn disable(wid: usize) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().disable(wid)
}

/// Enable all writers with type `typ`. See [WriterTypeEnum]
pub fn enable_type(typ: WriterTypeEnum) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().enable_type(typ)
}

/// Disable all writers with type `typ`. See [WriterTypeEnum]
pub fn disable_type(typ: WriterTypeEnum) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().disable_type(typ)
}

/// Syncronize all writers with types contained in `types`. Wait for maximum time `timeout` seconds.
pub fn sync(types: Vec<WriterTypeEnum>, timeout: f64) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().sync(types, timeout)
}

/// Syncronize all writers. Wait for maximum time `timeout` seconds.
pub fn sync_all(timeout: f64) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().sync_all(timeout)
}

/// Rotate a single log file `path` or all log files with `path` is `None`.
pub fn rotate(path: Option<PathBuf>) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().rotate(path)
}

// Network

pub fn set_encryption(wid: usize, key: EncryptionMethod) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().set_encryption(wid, key)
}

// Config

/// Set debug mode.

pub fn set_debug(debug: u8) {
    let logger = ROOT_LOGGER.lock().unwrap();
    let mut config = logger.instance.lock().unwrap();
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
    ROOT_LOGGER.lock().unwrap().get_writer_config(wid)
}

pub fn get_writer_configs() -> HashMap<usize, WriterConfigEnum> {
    ROOT_LOGGER.lock().unwrap().get_writer_configs()
}

pub fn get_server_config(wid: usize) -> Result<ServerConfig, LoggingError> {
    ROOT_LOGGER.lock().unwrap().get_server_config(wid)
}

pub fn get_server_configs() -> HashMap<usize, ServerConfig> {
    ROOT_LOGGER.lock().unwrap().get_server_configs()
}

pub fn get_root_server_address_port() -> Option<String> {
    ROOT_LOGGER.lock().unwrap().get_root_server_address_port()
}

pub fn get_server_addresses_ports() -> HashMap<usize, String> {
    ROOT_LOGGER.lock().unwrap().get_server_addresses_ports()
}

pub fn get_server_addresses() -> HashMap<usize, String> {
    ROOT_LOGGER.lock().unwrap().get_server_addresses()
}

pub fn get_server_ports() -> HashMap<usize, u16> {
    ROOT_LOGGER.lock().unwrap().get_server_ports()
}

pub fn get_server_auth_key() -> EncryptionMethod {
    ROOT_LOGGER.lock().unwrap().get_server_auth_key()
}

/// Get fastlogging configuration as string.
pub fn get_config_string() -> String {
    ROOT_LOGGER.lock().unwrap().get_config_string()
}

/// Save fastlogging configuration to file `path`.
pub fn save_config(path: Option<&Path>) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().save_config(path)
}

/// Get process ID of parent process.
pub fn get_parent_pid() -> Option<u32> {
    // Initialize root logger is not already done.
    let _logger = ROOT_LOGGER.lock().unwrap();
    PARENT_LOGGER_ADDRESS.lock().unwrap().as_ref().map(|v| v.0)
}

/// Get IP address of parent process LoggingServer.
pub fn get_parent_server_address() -> Option<ClientWriterConfig> {
    // Initialize root logger is not already done.
    let _logger = ROOT_LOGGER.lock().unwrap();
    PARENT_LOGGER_ADDRESS
        .lock()
        .unwrap()
        .as_ref()
        .map(|v| v.1.clone())
}

/// Get process ID of parent process and IP address of parent process LoggingServer.
pub fn get_parent_pid_server_address() -> Option<(u32, ClientWriterConfig)> {
    // Initialize root logger is not already done.
    let _logger = ROOT_LOGGER.lock().unwrap();
    PARENT_LOGGER_ADDRESS.lock().unwrap().clone()
}

// Logging methods

/// Log TRACE level message.
pub fn trace<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().trace(message)
}

/// Log DEBUG level message.
pub fn debug<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().debug(message)
}

/// Log INFO level message.
pub fn info<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().info(message)
}

/// Log SUCCESS level message.
pub fn success<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().success(message)
}

/// Log WARNING level message.
pub fn warning<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().warning(message)
}

/// Log ERROR level message.
pub fn error<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().error(message)
}

/// Log CRITICAL level message.
pub fn critical<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().critical(message)
}

/// Log FATAL level message.
pub fn fatal<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().fatal(message)
}

/// Log EXCEPTION level message.
pub fn exception<S: Into<String>>(message: S) -> Result<(), LoggingError> {
    ROOT_LOGGER.lock().unwrap().exception(message)
}
