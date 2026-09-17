#[macro_use]
extern crate serde_derive;

pub mod def;

pub use def::*;
mod config;
pub mod error;
pub use config::{ExtConfig, LoggingConfig};
pub use error::LoggingError;
pub mod file;
pub use file::{CompressionMethodEnum, FileWriter, FileWriterConfig};
pub mod net;
pub use net::{
    ClientTypeEnum, ClientWriter, ClientWriterConfig, EncryptionMethod, LoggingServer, ServerConfig,
};
pub mod console;
pub use console::{ConsoleWriter, ConsoleWriterConfig};
pub mod callback;
pub use callback::{CallbackWriter, CallbackWriterConfig};
pub mod otel;
pub use otel::{
    OpenTelemetryWriter, OpenTelemetryWriterConfig, DEFAULT_OTEL_BATCH_SIZE,
    DEFAULT_OTEL_ENDPOINT, DEFAULT_OTEL_FLUSH_INTERVAL, DEFAULT_OTEL_SERVICE_NAME,
    level2severity_number, level2severity_text,
};
pub mod logging;
pub mod root;
pub use logging::Logging;
pub use root::ROOT_LOGGER;
pub mod logger;
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
