//! # fastlogging
//!
//! A lightning-fast, thread-safe structured logging library for Rust and other
//! languages. It routes log messages through a non-blocking background thread to
//! one or more independent *writers*, keeping your hot path free of I/O.
//!
//! This crate is the Rust core of the multi-platform
//! [fastlogging](https://github.com/brmmm3/fastlogging-rs) project. Bespoke
//! wrappers are provided for C, C++, C#, Java and Go.
//!
//! ## Quick start
//!
//! Add `fastlogging` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! fastlogging = "0.9"
//! ```
//!
//! Create a logger with a default console writer and start logging:
//!
//! ```no_run
//! use fastlogging::{logging_new_default, LoggingError};
//!
//! fn main() -> Result<(), LoggingError> {
//!     let mut log = logging_new_default()?;
//!     log.info("Hello, fastlogging!")?;
//!     log.shutdown(false)?;
//!     Ok(())
//! }
//! ```
//!
//! Or configure a logger explicitly:
//!
//! ```no_run
//! use fastlogging::{DEBUG, ConsoleWriterConfig, Logging, LoggingError};
//!
//! fn main() -> Result<(), LoggingError> {
//!     let mut log = Logging::new(
//!         DEBUG,
//!         "myapp",
//!         Some(vec![ConsoleWriterConfig::new(DEBUG, true).into()]),
//!         None,
//!         None,
//!     )?;
//!     log.debug("starting up")?;
//!     log.info("ready")?;
//!     log.shutdown(false)?;
//!     Ok(())
//! }
//! ```
//!
//! ## How it works
//!
//! Every log call performs a cheap level check on the caller's thread. If the
//! message passes, it is handed to a flume-backed channel and a single background
//! `LoggingThread` dispatches it to each writer's own thread. This means logging
//! never blocks your code on disk, network or terminal I/O.
//!
//! ```text
//! Your code --level check--> channel --> LoggingThread --> ConsoleWriter (thread)
//!                                          |-------------> FileWriter    (thread)
//!                                          |-------------> ClientWriter  (thread)
//!                                          |-------------> LoggingServer (thread)
//!                                          |-------------> CallbackWriter(thread)
//!                                          `-------------> SyslogWriter  (thread)
//! ```
//!
//! ## Log levels
//!
//! Levels are `u8` constants exported by this crate:
//!
//! | Level     | Value |
//! |-----------|-------|
//! | `NOLOG`   | 100   |
//! | `EXCEPTION` | 60  |
//! | `CRITICAL`/`FATAL` | 50 |
//! | `ERROR`   | 40    |
//! | `WARNING`/`WARN` | 30 |
//! | `SUCCESS` | 25    |
//! | `INFO`    | 20    |
//! | `DEBUG`   | 10    |
//! | `TRACE`   | 5     |
//! | `NOTSET`  | 0     |
//!
//! See the [`def`] module and
//! [doc/LEVELS.md](https://github.com/brmmm3/fastlogging-rs/blob/main/fastlogging/doc/LEVELS.md)
//! for details.
//!
//! ## Writers
//!
//! Messages are delivered to any number of writers:
//!
//! - [`ConsoleWriter`] — colored output to the terminal.
//! - [`FileWriter`] — plain, rolling and compressed log files.
//! - [`ClientWriter`] / [`LoggingServer`] — send and receive log records over
//!   the network, optionally encrypted.
//! - [`CallbackWriter`] — invoke a Rust closure for every message.
//! - [`SyslogWriter`] — syslog (Unix) or event log (Windows).
//! - [`OpenTelemetryWriter`] — export to an OpenTelemetry Collector via
//!   OTLP/HTTP.
//!
//! ## Primary API
//!
//! - [`Logging`] — the main logging entry point; create one, add writers, log
//!   messages and shut it down.
//! - [`Logger`] — a lightweight, per-domain handle that logs to an existing
//!   [`Logging`] instance from any thread.
//! - [`ROOT_LOGGER`] — a process-wide, lazily initialised logger, available the
//!   moment it is first accessed, together with helper functions in the
//!   [`root`] module.
//!
//! ## Extended configuration
//!
//! Use [`ExtConfig`] to build a complete configuration in code, or persist it to
//! JSON, YAML or XML files with the `config_json`, `config_yaml` and `config_xml`
//! cargo features (all enabled by default).
//!
//! ## Feature flags
//!
//! | Feature       | Description                               | Default |
//! |---------------|-------------------------------------------|---------|
//! | `config_json` | JSON configuration support                | yes     |
//! | `config_xml`  | XML configuration support                 | yes     |
//! | `config_yaml` | YAML configuration support                | yes     |
//!
//! ## Further reading
//!
//! Extensive guides live in the [`fastlogging/doc`](https://github.com/brmmm3/fastlogging-rs/tree/main/fastlogging/doc)
//! directory: quick start, architecture, platform notes, writers, networking and
//! runnable examples.

#[macro_use]
extern crate serde_derive;

/// Log levels, converters and shared configuration enums.
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
    DEFAULT_OTEL_BATCH_SIZE, DEFAULT_OTEL_ENDPOINT, DEFAULT_OTEL_FLUSH_INTERVAL,
    DEFAULT_OTEL_SERVICE_NAME, OpenTelemetryWriter, OpenTelemetryWriterConfig,
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
