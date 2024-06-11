#[macro_use]
extern crate serde_derive;

mod def;
pub use def::*;
mod config;
pub use config::ExtConfig;
mod file;
pub use file::{CompressionMethodEnum, FileWriter, FileWriterConfig};
mod net;
pub use net::{ClientWriter, ClientWriterConfig, EncryptionMethod, LoggingServer, ServerConfig};
mod console;
pub use console::{ConsoleWriter, ConsoleWriterConfig};
mod syslog;
pub use syslog::{SyslogWriter, SyslogWriterConfig};
mod logging;
pub use logging::{logging_init, Logging, LOGGING};
mod logger;
pub use logger::Logger;
