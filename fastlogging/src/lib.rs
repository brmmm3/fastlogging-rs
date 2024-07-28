#[macro_use]
extern crate serde_derive;

mod def;
use std::{
    io::Error,
    path::{Path, PathBuf},
};

pub use def::*;
mod config;
pub use config::{ExtConfig, LoggingConfig};
mod file;
pub use file::{CompressionMethodEnum, FileWriter, FileWriterConfig};
mod net;
pub use net::{ClientWriter, ClientWriterConfig, EncryptionMethod, LoggingServer, ServerConfig};
mod console;
pub use console::{ConsoleWriter, ConsoleWriterConfig};
mod syslog;
pub use syslog::{SyslogWriter, SyslogWriterConfig};
mod logging;
pub use logging::{Logging, ROOT_LOGGER};
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
