#[macro_use]
extern crate serde_derive;

mod def;
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
pub use logging::{Logging, DEFAULT_LOGGER};
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
