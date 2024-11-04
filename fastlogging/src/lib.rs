#[macro_use]
extern crate serde_derive;

mod def;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub use def::*;
mod config;
mod error;
pub use config::{ExtConfig, LoggingConfig};
pub use error::LoggingError;
mod file;
pub use file::{CompressionMethodEnum, FileWriter, FileWriterConfig};
mod net;
pub use net::{
    ClientTypeEnum, ClientWriter, ClientWriterConfig, EncryptionMethod, LoggingServer, ServerConfig,
};
mod console;
pub use console::{ConsoleWriter, ConsoleWriterConfig};
mod callback;
pub use callback::{CallbackWriter, CallbackWriterConfig};
mod root;
use root::PARENT_LOGGER_ADDRESS;
pub use root::ROOT_LOGGER;
mod logging;
pub use logging::Logging;
mod logger;
pub use logger::Logger;
#[cfg(target_family = "unix")]
mod syslog;
#[cfg(target_family = "unix")]
pub use syslog::{SyslogTypeEnum, SyslogWriter, SyslogWriterConfig};
#[cfg(target_family = "unix")]
mod unix;
#[cfg(target_family = "unix")]
pub use unix::getppid;
#[cfg(target_family = "windows")]
mod eventlog;
#[cfg(target_family = "windows")]
pub use eventlog::{SyslogTypeEnum, SyslogWriter, SyslogWriterConfig};
#[cfg(target_family = "windows")]
mod windows;
#[cfg(target_family = "windows")]
pub use windows::getppid;

/// Initialize fastlogging with default console writer.
pub fn logging_new_default() -> Result<Logging, LoggingError> {
    Logging::new(
        NOTSET,
        "root",
        vec![ConsoleWriterConfig::new(NOTSET, false).into()],
        None,
        None,
    )
}

/// Shutdown fastlogging.
pub fn logging_init_root() {
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
