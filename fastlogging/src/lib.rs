#[macro_use]
extern crate serde_derive;

mod def;
use std::{
    io::Error,
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
mod syslog;
use root::PARENT_LOGGER_ADDRESS;
pub use syslog::{SyslogTypeEnum, SyslogWriter, SyslogWriterConfig};
mod root;
pub use root::ROOT_LOGGER;
mod logging;
pub use logging::Logging;
mod logger;
pub use logger::Logger;
#[cfg(target_family = "unix")]
mod unix;
#[cfg(target_family = "unix")]
pub use unix::getppid;
#[cfg(target_family = "windows")]
mod windows;
#[cfg(target_family = "windows")]
pub use windows::getppid;

pub fn shutdown(now: bool) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().shutdown(now)
}

pub fn set_level(writer: &WriterTypeEnum, level: u8) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().set_level(writer, level)
}

pub fn set_domain<S: Into<String>>(domain: S) {
    ROOT_LOGGER.lock().unwrap().set_domain(&domain.into())
}

pub fn set_level2sym(level2sym: &LevelSyms) {
    ROOT_LOGGER.lock().unwrap().set_level2sym(level2sym)
}

pub fn set_ext_config(ext_config: &ExtConfig) {
    ROOT_LOGGER.lock().unwrap().set_ext_config(ext_config)
}

pub fn add_logger(logger: &mut Logger) {
    ROOT_LOGGER.lock().unwrap().add_logger(logger)
}

pub fn remove_logger(logger: &mut Logger) {
    ROOT_LOGGER.lock().unwrap().remove_logger(logger)
}

pub fn add_writer(writer: &WriterConfigEnum) -> Result<WriterTypeEnum, Error> {
    ROOT_LOGGER.lock().unwrap().add_writer(writer)
}

pub fn remove_writer(writer: &WriterTypeEnum) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().remove_writer(writer)
}

pub fn sync(
    console: bool,
    file: bool,
    client: bool,
    syslog: bool,
    timeout: f64,
) -> Result<(), Error> {
    ROOT_LOGGER
        .lock()
        .unwrap()
        .sync(console, file, client, syslog, timeout)
}

pub fn sync_all(timeout: f64) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().sync_all(timeout)
}

pub fn rotate(path: Option<PathBuf>) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().rotate(path)
}

// Network

pub fn set_encryption(writer: WriterTypeEnum, key: EncryptionMethod) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().set_encryption(writer, key)
}

// Config

pub fn set_debug(debug: u8) {
    let logger = ROOT_LOGGER.lock().unwrap();
    let mut config = logger.instance.lock().unwrap();
    config.debug = debug;
    for writer in config.clients.values_mut() {
        writer.config.lock().unwrap().debug = debug;
    }
    for server in config.servers.values_mut() {
        server.config.lock().unwrap().debug = debug;
    }
}

pub fn get_config(writer: &WriterTypeEnum) -> Result<WriterConfigEnum, Error> {
    ROOT_LOGGER.lock().unwrap().get_config(writer)
}

pub fn get_server_config(address: &str) -> Option<ServerConfig> {
    ROOT_LOGGER.lock().unwrap().get_server_config(address)
}

pub fn get_server_auth_key() -> EncryptionMethod {
    ROOT_LOGGER.lock().unwrap().get_server_auth_key()
}

pub fn get_config_string() -> String {
    ROOT_LOGGER.lock().unwrap().get_config_string()
}

pub fn save_config(path: &Path) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().save_config(path)
}

pub fn get_parent_pid() -> Option<u32> {
    // Initialize root logger is not already done.
    let _logger = ROOT_LOGGER.lock().unwrap();
    PARENT_LOGGER_ADDRESS.lock().unwrap().as_ref().map(|v| v.0)
}

pub fn get_parent_server_address() -> Option<ClientWriterConfig> {
    // Initialize root logger is not already done.
    let _logger = ROOT_LOGGER.lock().unwrap();
    PARENT_LOGGER_ADDRESS
        .lock()
        .unwrap()
        .as_ref()
        .map(|v| v.1.clone())
}

pub fn get_parent_pid_server_address() -> Option<(u32, ClientWriterConfig)> {
    // Initialize root logger is not already done.
    let _logger = ROOT_LOGGER.lock().unwrap();
    PARENT_LOGGER_ADDRESS.lock().unwrap().clone()
}

// Logging methods

pub fn trace<S: Into<String>>(message: S) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().trace(message)
}

pub fn debug<S: Into<String>>(message: S) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().debug(message)
}

pub fn info<S: Into<String>>(message: S) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().info(message)
}

pub fn success<S: Into<String>>(message: S) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().success(message)
}

pub fn warning<S: Into<String>>(message: S) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().warning(message)
}

pub fn error<S: Into<String>>(message: S) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().error(message)
}

pub fn critical<S: Into<String>>(message: S) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().critical(message)
}

pub fn fatal<S: Into<String>>(message: S) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().fatal(message)
}

pub fn exception<S: Into<String>>(message: S) -> Result<(), Error> {
    ROOT_LOGGER.lock().unwrap().exception(message)
}
