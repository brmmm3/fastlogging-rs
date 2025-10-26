#[macro_use]
extern crate serde_derive;

mod def;

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
mod logging;
pub mod root;
pub use logging::Logging;
pub use root::ROOT_LOGGER;
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
        Some(vec![ConsoleWriterConfig::new(NOTSET, false).into()]),
        None,
        None,
    )
}
