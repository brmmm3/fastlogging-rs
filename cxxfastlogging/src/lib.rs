//! C++ bindings for the [`fastlogging`] crate, generated with the [`cxx`] crate.
//!
//! This crate exposes most of the `fastlogging` Rust API to C++ as a set of
//! opaque Rust types (`Logging`, `Logger`, `WriterConfig`) together with
//! associated/free functions that create and manipulate them. The actual
//! Rust API (this module's public items) can also be used directly from
//! Rust without going through the `cxx` bridge at all.
//!
//! See `h/fastlogging.h` for a stable C++ include path, and `doc/API.md` for
//! a written overview and usage examples.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use fastlogging::LoggingError;

/// Opaque wrapper around a [`fastlogging::WriterConfigEnum`].
///
/// Instances are created with one of the `WriterConfig::new_*` factory
/// functions and consumed by [`Logging::new`], [`Logging::add_writer_config`],
/// [`Logging::add_writer_configs`] or [`Logging::set_root_writer_config`].
pub struct WriterConfig(fastlogging::WriterConfigEnum);

impl WriterConfig {
    /// Create configuration for a console writer.
    pub fn new_console(level: u8, colors: bool) -> Box<WriterConfig> {
        Box::new(WriterConfig(fastlogging::WriterConfigEnum::Console(
            fastlogging::ConsoleWriterConfig::new(level, colors),
        )))
    }

    /// Create configuration for a file writer.
    ///
    /// `timeout_secs` and `time_secs` less than zero mean "not set".
    pub fn new_file(
        level: u8,
        path: &str,
        size: u64,
        backlog: u64,
        timeout_secs: i64,
        time_secs: i64,
        compression: ffi::CompressionMethodEnum,
    ) -> Result<Box<WriterConfig>, LoggingError> {
        let timeout = if timeout_secs < 0 {
            None
        } else {
            Some(Duration::from_secs(timeout_secs as u64))
        };
        let time = if time_secs < 0 {
            None
        } else {
            Some(SystemTime::now() + Duration::from_secs(time_secs as u64))
        };
        let config = fastlogging::FileWriterConfig::new(
            level,
            PathBuf::from(path),
            size as usize,
            backlog as usize,
            timeout,
            time,
            Some(compression_from_ffi(compression)),
        )?;
        Ok(Box::new(WriterConfig(fastlogging::WriterConfigEnum::File(
            config,
        ))))
    }

    /// Create configuration for a network client writer.
    pub fn new_client(
        level: u8,
        address: &str,
        key_type: ffi::EncryptionMethodEnum,
        key: &[u8],
    ) -> Box<WriterConfig> {
        let key = encryption_from_ffi(key_type, key);
        Box::new(WriterConfig(fastlogging::WriterConfigEnum::Client(
            fastlogging::ClientWriterConfig::new(level, address, key),
        )))
    }

    /// Create configuration for a network server (listener) writer.
    pub fn new_server(
        level: u8,
        address: &str,
        key_type: ffi::EncryptionMethodEnum,
        key: &[u8],
    ) -> Box<WriterConfig> {
        let key = encryption_from_ffi(key_type, key);
        Box::new(WriterConfig(fastlogging::WriterConfigEnum::Server(
            fastlogging::ServerConfig::new(level, address, key),
        )))
    }

    /// Create configuration for a syslog / eventlog writer.
    ///
    /// An empty `hostname` means "not set".
    pub fn new_syslog(level: u8, hostname: &str, pname: &str, pid: u32) -> Box<WriterConfig> {
        let hostname = if hostname.is_empty() {
            None
        } else {
            Some(hostname.to_string())
        };
        Box::new(WriterConfig(fastlogging::WriterConfigEnum::Syslog(
            fastlogging::SyslogWriterConfig::new(level, hostname, pname, pid),
        )))
    }
}

/// Opaque wrapper around a [`fastlogging::Logging`] instance.
pub struct Logging(fastlogging::Logging);

impl Logging {
    /// Create a new `Logging` instance with a default console writer.
    pub fn new_default() -> Result<Box<Logging>, LoggingError> {
        Ok(Box::new(Logging(fastlogging::logging_new_default()?)))
    }

    /// Create a new `Logging` instance with the given global log `level`,
    /// `domain` and writer `configs`.
    pub fn create(
        level: u8,
        domain: &str,
        configs: Vec<Box<WriterConfig>>,
    ) -> Result<Box<Logging>, LoggingError> {
        let configs = configs
            .into_iter()
            .map(|c| {
                let WriterConfig(cfg) = *c;
                cfg
            })
            .collect::<Vec<_>>();
        let logging = fastlogging::Logging::new(level, domain, Some(configs), None, None)?;
        Ok(Box::new(Logging(logging)))
    }

    /// Load and apply a configuration file.
    pub fn apply_config(&mut self, path: &str) -> Result<(), LoggingError> {
        self.0.apply_config(Path::new(path))
    }

    /// Shut down the logging instance and all its writers.
    pub fn shutdown(&mut self, now: bool) -> Result<(), LoggingError> {
        self.0.shutdown(now)
    }

    /// Get the global log level.
    pub fn level(&self) -> u8 {
        self.0.level
    }

    /// Set the log level of writer `wid`. Use `wid = 0` for the root writer.
    pub fn set_level(&mut self, wid: u64, level: u8) -> Result<(), LoggingError> {
        self.0.set_level(wid as usize, level)
    }

    /// Set the logging domain.
    pub fn set_domain(&mut self, domain: &str) {
        self.0.set_domain(domain)
    }

    /// Select how log levels are rendered in log messages.
    pub fn set_level2sym(&mut self, level2sym: ffi::LevelSymsEnum) {
        self.0.set_level2sym(&level_syms_from_ffi(level2sym))
    }

    /// Set the extended logging configuration.
    pub fn set_ext_config(&mut self, ext_config: ffi::ExtConfigFfi) {
        self.0.set_ext_config(&ext_config_from_ffi(ext_config))
    }

    /// Register `logger` at this `Logging` instance.
    pub fn add_logger(&mut self, logger: &mut Logger) {
        self.0.add_logger(&mut logger.0)
    }

    /// Unregister `logger` from this `Logging` instance.
    pub fn remove_logger(&mut self, logger: &mut Logger) {
        self.0.remove_logger(&mut logger.0)
    }

    /// Set the root writer. Only `Client` or `Server` writer configs are allowed.
    pub fn set_root_writer_config(
        &mut self,
        config: Box<WriterConfig>,
    ) -> Result<(), LoggingError> {
        let WriterConfig(cfg) = *config;
        self.0.set_root_writer_config(&cfg)
    }

    /// Add a new writer and return its writer id.
    pub fn add_writer_config(&mut self, config: Box<WriterConfig>) -> Result<u64, LoggingError> {
        let WriterConfig(cfg) = *config;
        Ok(self.0.add_writer_config(&cfg)? as u64)
    }

    /// Add multiple writers and return their writer ids.
    pub fn add_writer_configs(
        &mut self,
        configs: Vec<Box<WriterConfig>>,
    ) -> Result<Vec<u64>, LoggingError> {
        let configs = configs
            .into_iter()
            .map(|c| {
                let WriterConfig(cfg) = *c;
                cfg
            })
            .collect::<Vec<_>>();
        Ok(self
            .0
            .add_writer_configs(configs)?
            .into_iter()
            .map(|x| x as u64)
            .collect())
    }

    /// Remove writer `wid`. Returns `true` if a writer was removed.
    pub fn remove_writer(&mut self, wid: u64) -> bool {
        self.0.remove_writer(wid as usize).is_some()
    }

    /// Enable writer `wid`.
    pub fn enable(&self, wid: u64) -> Result<(), LoggingError> {
        self.0.enable(wid as usize)
    }

    /// Disable writer `wid`.
    pub fn disable(&self, wid: u64) -> Result<(), LoggingError> {
        self.0.disable(wid as usize)
    }

    /// Enable all writers of the given type. `data` is only used for the
    /// `File`, `Client` and `Server` variants (path resp. address); pass an
    /// empty string for the other variants.
    pub fn enable_type(&self, tag: ffi::WriterTypeTag, data: &str) -> Result<(), LoggingError> {
        self.0.enable_type(writer_type_from_tag(tag, data))
    }

    /// Disable all writers of the given type. See [`Logging::enable_type`].
    pub fn disable_type(&self, tag: ffi::WriterTypeTag, data: &str) -> Result<(), LoggingError> {
        self.0.disable_type(writer_type_from_tag(tag, data))
    }

    /// Synchronize all writers of the given type. See [`Logging::enable_type`].
    pub fn sync_type(
        &self,
        tag: ffi::WriterTypeTag,
        data: &str,
        timeout: f64,
    ) -> Result<(), LoggingError> {
        self.0.sync(vec![writer_type_from_tag(tag, data)], timeout)
    }

    /// Synchronize all writers.
    pub fn sync_all(&self, timeout: f64) -> Result<(), LoggingError> {
        self.0.sync_all(timeout)
    }

    /// Rotate a single log file, or all log files if `path` is empty.
    pub fn rotate(&self, path: &str) -> Result<(), LoggingError> {
        let path = if path.is_empty() {
            None
        } else {
            Some(PathBuf::from(path))
        };
        self.0.rotate(path)
    }

    /// Configure encryption for writer `wid`.
    pub fn set_encryption(
        &mut self,
        wid: u64,
        key_type: ffi::EncryptionMethodEnum,
        key: &[u8],
    ) -> Result<(), LoggingError> {
        self.0
            .set_encryption(wid as usize, encryption_from_ffi(key_type, key))
    }

    /// Set the debug level. Only useful for developers of `fastlogging` itself.
    pub fn set_debug(&mut self, debug: u8) {
        self.0.set_debug(debug)
    }

    /// Get the server configuration of writer `wid`.
    pub fn get_server_config(&self, wid: u64) -> Result<ffi::ServerConfigInfo, LoggingError> {
        let config = self.0.get_server_config(wid as usize)?;
        Ok(server_config_to_ffi(wid, config))
    }

    /// Get the server configuration of all server writers.
    pub fn get_server_configs(&self) -> Vec<ffi::ServerConfigInfo> {
        self.0
            .get_server_configs()
            .into_iter()
            .map(|(id, config)| server_config_to_ffi(id as u64, config))
            .collect()
    }

    /// Get `address:port` of the root server writer, or an empty string.
    pub fn get_root_server_address_port(&self) -> String {
        self.0.get_root_server_address_port().unwrap_or_default()
    }

    /// Get `address:port` of all server writers.
    pub fn get_server_addresses_ports(&self) -> Vec<ffi::IdString> {
        self.0
            .get_server_addresses_ports()
            .into_iter()
            .map(|(id, value)| ffi::IdString {
                id: id as u64,
                value,
            })
            .collect()
    }

    /// Get the address of all server writers.
    pub fn get_server_addresses(&self) -> Vec<ffi::IdString> {
        self.0
            .get_server_addresses()
            .into_iter()
            .map(|(id, value)| ffi::IdString {
                id: id as u64,
                value,
            })
            .collect()
    }

    /// Get the port of all server writers.
    pub fn get_server_ports(&self) -> Vec<ffi::IdU16> {
        self.0
            .get_server_ports()
            .into_iter()
            .map(|(id, value)| ffi::IdU16 {
                id: id as u64,
                value,
            })
            .collect()
    }

    /// Get the authentication key used by the internal logging server.
    pub fn get_server_auth_key(&self) -> Vec<u8> {
        self.0
            .get_server_auth_key()
            .key_cloned()
            .unwrap_or_default()
    }

    /// Get a human-readable dump of the current configuration.
    pub fn get_config_string(&self) -> String {
        self.0.get_config_string()
    }

    /// Save the current configuration to `path`, or to the default path if empty.
    pub fn save_config(&mut self, path: &str) -> Result<(), LoggingError> {
        let path = if path.is_empty() {
            None
        } else {
            Some(Path::new(path))
        };
        self.0.save_config(path)
    }

    /// Log a TRACE level message.
    pub fn trace(&self, message: &str) -> Result<(), LoggingError> {
        self.0.trace(message)
    }

    /// Log a DEBUG level message.
    pub fn debug(&self, message: &str) -> Result<(), LoggingError> {
        self.0.debug(message)
    }

    /// Log an INFO level message.
    pub fn info(&self, message: &str) -> Result<(), LoggingError> {
        self.0.info(message)
    }

    /// Log a SUCCESS level message.
    pub fn success(&self, message: &str) -> Result<(), LoggingError> {
        self.0.success(message)
    }

    /// Log a WARNING level message.
    pub fn warning(&self, message: &str) -> Result<(), LoggingError> {
        self.0.warning(message)
    }

    /// Log an ERROR level message.
    pub fn error(&self, message: &str) -> Result<(), LoggingError> {
        self.0.error(message)
    }

    /// Log a CRITICAL level message.
    pub fn critical(&self, message: &str) -> Result<(), LoggingError> {
        self.0.critical(message)
    }

    /// Log a FATAL level message.
    pub fn fatal(&self, message: &str) -> Result<(), LoggingError> {
        self.0.fatal(message)
    }

    /// Log an EXCEPTION level message.
    pub fn exception(&self, message: &str) -> Result<(), LoggingError> {
        self.0.exception(message)
    }
}

/// Opaque wrapper around a [`fastlogging::Logger`].
pub struct Logger(fastlogging::Logger);

impl Logger {
    /// Create a new logger with the given log `level` and `domain`.
    pub fn create(level: u8, domain: &str) -> Box<Logger> {
        Box::new(Logger(fastlogging::Logger::new(level, domain)))
    }

    /// Create a new logger which also logs the thread name and/or id.
    pub fn new_ext(level: u8, domain: &str, tname: bool, tid: bool) -> Box<Logger> {
        Box::new(Logger(fastlogging::Logger::new_ext(
            level, domain, tname, tid,
        )))
    }

    /// Set the log level.
    pub fn set_level(&mut self, level: u8) {
        self.0.set_level(level)
    }

    /// Get the log level.
    pub fn level(&self) -> u8 {
        self.0.level()
    }

    /// Set the logging domain.
    pub fn set_domain(&mut self, domain: &str) {
        self.0.set_domain(domain)
    }

    /// Log a TRACE level message.
    pub fn trace(&self, message: &str) -> Result<(), LoggingError> {
        self.0.trace(message)
    }

    /// Log a DEBUG level message.
    pub fn debug(&self, message: &str) -> Result<(), LoggingError> {
        self.0.debug(message)
    }

    /// Log an INFO level message.
    pub fn info(&self, message: &str) -> Result<(), LoggingError> {
        self.0.info(message)
    }

    /// Log a SUCCESS level message.
    pub fn success(&self, message: &str) -> Result<(), LoggingError> {
        self.0.success(message)
    }

    /// Log a WARNING level message.
    pub fn warning(&self, message: &str) -> Result<(), LoggingError> {
        self.0.warning(message)
    }

    /// Log an ERROR level message.
    pub fn error(&self, message: &str) -> Result<(), LoggingError> {
        self.0.error(message)
    }

    /// Log a CRITICAL level message.
    pub fn critical(&self, message: &str) -> Result<(), LoggingError> {
        self.0.critical(message)
    }

    /// Log a FATAL level message.
    pub fn fatal(&self, message: &str) -> Result<(), LoggingError> {
        self.0.fatal(message)
    }

    /// Log an EXCEPTION level message.
    pub fn exception(&self, message: &str) -> Result<(), LoggingError> {
        self.0.exception(message)
    }
}

// ---------------------------------------------------------------------------
// Root logger (process-wide singleton), mirrors `fastlogging::root`.
// ---------------------------------------------------------------------------

fn root_init() {
    fastlogging::root::root_init();
}

fn root_shutdown(now: bool) -> Result<(), LoggingError> {
    fastlogging::root::shutdown(now)
}

fn root_set_level(wid: u64, level: u8) -> Result<(), LoggingError> {
    fastlogging::root::set_level(wid as usize, level)
}

fn root_set_domain(domain: &str) {
    fastlogging::root::set_domain(domain)
}

fn root_set_level2sym(level2sym: ffi::LevelSymsEnum) {
    fastlogging::root::set_level2sym(&level_syms_from_ffi(level2sym))
}

fn root_set_ext_config(ext_config: ffi::ExtConfigFfi) {
    fastlogging::root::set_ext_config(&ext_config_from_ffi(ext_config))
}

fn root_add_logger(logger: &mut Logger) {
    fastlogging::root::add_logger(&mut logger.0)
}

fn root_remove_logger(logger: &mut Logger) {
    fastlogging::root::remove_logger(&mut logger.0)
}

fn root_add_writer_config(config: Box<WriterConfig>) -> Result<u64, LoggingError> {
    let WriterConfig(cfg) = *config;
    Ok(fastlogging::root::add_writer_config(&cfg)? as u64)
}

fn root_remove_writer(wid: u64) -> bool {
    fastlogging::root::remove_writer(wid as usize).is_some()
}

fn root_enable(wid: u64) -> Result<(), LoggingError> {
    fastlogging::root::enable(wid as usize)
}

fn root_disable(wid: u64) -> Result<(), LoggingError> {
    fastlogging::root::disable(wid as usize)
}

fn root_enable_type(tag: ffi::WriterTypeTag, data: &str) -> Result<(), LoggingError> {
    fastlogging::root::enable_type(writer_type_from_tag(tag, data))
}

fn root_disable_type(tag: ffi::WriterTypeTag, data: &str) -> Result<(), LoggingError> {
    fastlogging::root::disable_type(writer_type_from_tag(tag, data))
}

fn root_sync_type(tag: ffi::WriterTypeTag, data: &str, timeout: f64) -> Result<(), LoggingError> {
    fastlogging::root::sync(vec![writer_type_from_tag(tag, data)], timeout)
}

fn root_sync_all(timeout: f64) -> Result<(), LoggingError> {
    fastlogging::root::sync_all(timeout)
}

fn root_rotate(path: &str) -> Result<(), LoggingError> {
    let path = if path.is_empty() {
        None
    } else {
        Some(PathBuf::from(path))
    };
    fastlogging::root::rotate(path)
}

fn root_set_debug(debug: u8) {
    fastlogging::root::set_debug(debug)
}

fn root_get_server_auth_key() -> Vec<u8> {
    fastlogging::root::get_server_auth_key()
        .key_cloned()
        .unwrap_or_default()
}

fn root_get_config_string() -> String {
    fastlogging::root::get_config_string()
}

fn root_save_config(path: &str) -> Result<(), LoggingError> {
    let path = if path.is_empty() {
        None
    } else {
        Some(Path::new(path))
    };
    fastlogging::root::save_config(path)
}

fn root_trace(message: &str) -> Result<(), LoggingError> {
    fastlogging::root::trace(message)
}

fn root_debug(message: &str) -> Result<(), LoggingError> {
    fastlogging::root::debug(message)
}

fn root_info(message: &str) -> Result<(), LoggingError> {
    fastlogging::root::info(message)
}

fn root_success(message: &str) -> Result<(), LoggingError> {
    fastlogging::root::success(message)
}

fn root_warning(message: &str) -> Result<(), LoggingError> {
    fastlogging::root::warning(message)
}

fn root_error(message: &str) -> Result<(), LoggingError> {
    fastlogging::root::error(message)
}

fn root_critical(message: &str) -> Result<(), LoggingError> {
    fastlogging::root::critical(message)
}

fn root_fatal(message: &str) -> Result<(), LoggingError> {
    fastlogging::root::fatal(message)
}

fn root_exception(message: &str) -> Result<(), LoggingError> {
    fastlogging::root::exception(message)
}

// ---------------------------------------------------------------------------
// Conversions between the shared `ffi` types and the real `fastlogging` types.
// ---------------------------------------------------------------------------

fn encryption_from_ffi(
    key_type: ffi::EncryptionMethodEnum,
    key: &[u8],
) -> fastlogging::EncryptionMethod {
    match key_type {
        ffi::EncryptionMethodEnum::AuthKey => fastlogging::EncryptionMethod::AuthKey(key.to_vec()),
        ffi::EncryptionMethodEnum::AES => fastlogging::EncryptionMethod::AES(key.to_vec()),
        _ => fastlogging::EncryptionMethod::NONE,
    }
}

fn compression_from_ffi(value: ffi::CompressionMethodEnum) -> fastlogging::CompressionMethodEnum {
    match value {
        ffi::CompressionMethodEnum::Deflate => fastlogging::CompressionMethodEnum::Deflate,
        ffi::CompressionMethodEnum::Zstd => fastlogging::CompressionMethodEnum::Zstd,
        ffi::CompressionMethodEnum::Lzma => fastlogging::CompressionMethodEnum::Lzma,
        _ => fastlogging::CompressionMethodEnum::Store,
    }
}

fn level_syms_from_ffi(value: ffi::LevelSymsEnum) -> fastlogging::LevelSyms {
    match value {
        ffi::LevelSymsEnum::Short => fastlogging::LevelSyms::Short,
        ffi::LevelSymsEnum::Str => fastlogging::LevelSyms::Str,
        _ => fastlogging::LevelSyms::Sym,
    }
}

fn message_struct_from_ffi(value: ffi::MessageStructEnum) -> fastlogging::MessageStructEnum {
    match value {
        ffi::MessageStructEnum::Json => fastlogging::MessageStructEnum::Json,
        ffi::MessageStructEnum::Xml => fastlogging::MessageStructEnum::Xml,
        _ => fastlogging::MessageStructEnum::String,
    }
}

fn ext_config_from_ffi(value: ffi::ExtConfigFfi) -> fastlogging::ExtConfig {
    fastlogging::ExtConfig::new(
        message_struct_from_ffi(value.structured),
        value.hostname,
        value.pname,
        value.pid,
        value.tname,
        value.tid,
    )
}

fn writer_type_from_tag(tag: ffi::WriterTypeTag, data: &str) -> fastlogging::WriterTypeEnum {
    match tag {
        ffi::WriterTypeTag::Root => fastlogging::WriterTypeEnum::Root,
        ffi::WriterTypeTag::Console => fastlogging::WriterTypeEnum::Console,
        ffi::WriterTypeTag::File => fastlogging::WriterTypeEnum::File(data.to_string()),
        ffi::WriterTypeTag::Files => fastlogging::WriterTypeEnum::Files,
        ffi::WriterTypeTag::Client => fastlogging::WriterTypeEnum::Client(data.to_string()),
        ffi::WriterTypeTag::Clients => fastlogging::WriterTypeEnum::Clients,
        ffi::WriterTypeTag::Server => fastlogging::WriterTypeEnum::Server(data.to_string()),
        ffi::WriterTypeTag::Servers => fastlogging::WriterTypeEnum::Servers,
        ffi::WriterTypeTag::Callback => fastlogging::WriterTypeEnum::Callback,
        ffi::WriterTypeTag::Syslog => fastlogging::WriterTypeEnum::Syslog,
        _ => fastlogging::WriterTypeEnum::Root,
    }
}

fn server_config_to_ffi(id: u64, config: fastlogging::ServerConfig) -> ffi::ServerConfigInfo {
    let (key_type, key) = match config.key {
        fastlogging::EncryptionMethod::NONE => (ffi::EncryptionMethodEnum::NONE, Vec::new()),
        fastlogging::EncryptionMethod::AuthKey(key) => (ffi::EncryptionMethodEnum::AuthKey, key),
        fastlogging::EncryptionMethod::AES(key) => (ffi::EncryptionMethodEnum::AES, key),
    };
    ffi::ServerConfigInfo {
        id,
        level: config.level,
        address: config.address,
        port: config.port,
        key_type,
        key,
        port_file: config
            .port_file
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default(),
    }
}

// ---------------------------------------------------------------------------
// The cxx bridge itself.
// ---------------------------------------------------------------------------

#[cxx::bridge]
pub mod ffi {
    /// Encryption method for network writers.
    enum EncryptionMethodEnum {
        NONE,
        AuthKey,
        AES,
    }

    /// Compression method for rotated log files.
    enum CompressionMethodEnum {
        Store,
        Deflate,
        Zstd,
        Lzma,
    }

    /// How log messages are structured.
    enum MessageStructEnum {
        String,
        Json,
        Xml,
    }

    /// How log levels are rendered in log messages.
    enum LevelSymsEnum {
        Sym,
        Short,
        Str,
    }

    /// Category of writer, used with `enable_type` / `disable_type` / `sync_type`.
    ///
    /// `data` passed alongside a `File`, `Client` or `Server` tag selects a
    /// specific file path resp. network address; it is ignored for the other
    /// variants.
    enum WriterTypeTag {
        Root,
        Console,
        File,
        Files,
        Client,
        Clients,
        Server,
        Servers,
        Callback,
        Syslog,
    }

    /// Extended logging configuration.
    struct ExtConfigFfi {
        structured: MessageStructEnum,
        hostname: bool,
        pname: bool,
        pid: bool,
        tname: bool,
        tid: bool,
    }

    /// Configuration of a network server writer, as returned by
    /// `Logging::get_server_config` / `Logging::get_server_configs`.
    struct ServerConfigInfo {
        id: u64,
        level: u8,
        address: String,
        port: u16,
        key_type: EncryptionMethodEnum,
        key: Vec<u8>,
        port_file: String,
    }

    /// A writer id paired with a `String` value.
    struct IdString {
        id: u64,
        value: String,
    }

    /// A writer id paired with a `u16` value.
    struct IdU16 {
        id: u64,
        value: u16,
    }

    extern "Rust" {
        type WriterConfig;

        #[Self = "WriterConfig"]
        fn new_console(level: u8, colors: bool) -> Box<WriterConfig>;
        #[Self = "WriterConfig"]
        fn new_file(
            level: u8,
            path: &str,
            size: u64,
            backlog: u64,
            timeout_secs: i64,
            time_secs: i64,
            compression: CompressionMethodEnum,
        ) -> Result<Box<WriterConfig>>;
        #[Self = "WriterConfig"]
        fn new_client(
            level: u8,
            address: &str,
            key_type: EncryptionMethodEnum,
            key: &[u8],
        ) -> Box<WriterConfig>;
        #[Self = "WriterConfig"]
        fn new_server(
            level: u8,
            address: &str,
            key_type: EncryptionMethodEnum,
            key: &[u8],
        ) -> Box<WriterConfig>;
        #[Self = "WriterConfig"]
        fn new_syslog(level: u8, hostname: &str, pname: &str, pid: u32) -> Box<WriterConfig>;
    }

    extern "Rust" {
        type Logging;

        #[Self = "Logging"]
        fn new_default() -> Result<Box<Logging>>;
        #[Self = "Logging"]
        fn create(level: u8, domain: &str, configs: Vec<Box<WriterConfig>>)
        -> Result<Box<Logging>>;

        fn apply_config(self: &mut Logging, path: &str) -> Result<()>;
        fn shutdown(self: &mut Logging, now: bool) -> Result<()>;
        fn level(self: &Logging) -> u8;
        fn set_level(self: &mut Logging, wid: u64, level: u8) -> Result<()>;
        fn set_domain(self: &mut Logging, domain: &str);
        fn set_level2sym(self: &mut Logging, level2sym: LevelSymsEnum);
        fn set_ext_config(self: &mut Logging, ext_config: ExtConfigFfi);
        fn add_logger(self: &mut Logging, logger: &mut Logger);
        fn remove_logger(self: &mut Logging, logger: &mut Logger);
        fn set_root_writer_config(self: &mut Logging, config: Box<WriterConfig>) -> Result<()>;
        fn add_writer_config(self: &mut Logging, config: Box<WriterConfig>) -> Result<u64>;
        fn add_writer_configs(
            self: &mut Logging,
            configs: Vec<Box<WriterConfig>>,
        ) -> Result<Vec<u64>>;
        fn remove_writer(self: &mut Logging, wid: u64) -> bool;
        fn enable(self: &Logging, wid: u64) -> Result<()>;
        fn disable(self: &Logging, wid: u64) -> Result<()>;
        fn enable_type(self: &Logging, tag: WriterTypeTag, data: &str) -> Result<()>;
        fn disable_type(self: &Logging, tag: WriterTypeTag, data: &str) -> Result<()>;
        fn sync_type(self: &Logging, tag: WriterTypeTag, data: &str, timeout: f64) -> Result<()>;
        fn sync_all(self: &Logging, timeout: f64) -> Result<()>;
        fn rotate(self: &Logging, path: &str) -> Result<()>;
        fn set_encryption(
            self: &mut Logging,
            wid: u64,
            key_type: EncryptionMethodEnum,
            key: &[u8],
        ) -> Result<()>;
        fn set_debug(self: &mut Logging, debug: u8);
        fn get_server_config(self: &Logging, wid: u64) -> Result<ServerConfigInfo>;
        fn get_server_configs(self: &Logging) -> Vec<ServerConfigInfo>;
        fn get_root_server_address_port(self: &Logging) -> String;
        fn get_server_addresses_ports(self: &Logging) -> Vec<IdString>;
        fn get_server_addresses(self: &Logging) -> Vec<IdString>;
        fn get_server_ports(self: &Logging) -> Vec<IdU16>;
        fn get_server_auth_key(self: &Logging) -> Vec<u8>;
        fn get_config_string(self: &Logging) -> String;
        fn save_config(self: &mut Logging, path: &str) -> Result<()>;
        fn trace(self: &Logging, message: &str) -> Result<()>;
        fn debug(self: &Logging, message: &str) -> Result<()>;
        fn info(self: &Logging, message: &str) -> Result<()>;
        fn success(self: &Logging, message: &str) -> Result<()>;
        fn warning(self: &Logging, message: &str) -> Result<()>;
        fn error(self: &Logging, message: &str) -> Result<()>;
        fn critical(self: &Logging, message: &str) -> Result<()>;
        fn fatal(self: &Logging, message: &str) -> Result<()>;
        fn exception(self: &Logging, message: &str) -> Result<()>;
    }

    extern "Rust" {
        type Logger;

        #[Self = "Logger"]
        fn create(level: u8, domain: &str) -> Box<Logger>;
        #[Self = "Logger"]
        fn new_ext(level: u8, domain: &str, tname: bool, tid: bool) -> Box<Logger>;

        fn set_level(self: &mut Logger, level: u8);
        fn level(self: &Logger) -> u8;
        fn set_domain(self: &mut Logger, domain: &str);
        fn trace(self: &Logger, message: &str) -> Result<()>;
        fn debug(self: &Logger, message: &str) -> Result<()>;
        fn info(self: &Logger, message: &str) -> Result<()>;
        fn success(self: &Logger, message: &str) -> Result<()>;
        fn warning(self: &Logger, message: &str) -> Result<()>;
        fn error(self: &Logger, message: &str) -> Result<()>;
        fn critical(self: &Logger, message: &str) -> Result<()>;
        fn fatal(self: &Logger, message: &str) -> Result<()>;
        fn exception(self: &Logger, message: &str) -> Result<()>;
    }

    extern "Rust" {
        fn root_init();
        fn root_shutdown(now: bool) -> Result<()>;
        fn root_set_level(wid: u64, level: u8) -> Result<()>;
        fn root_set_domain(domain: &str);
        fn root_set_level2sym(level2sym: LevelSymsEnum);
        fn root_set_ext_config(ext_config: ExtConfigFfi);
        fn root_add_logger(logger: &mut Logger);
        fn root_remove_logger(logger: &mut Logger);
        fn root_add_writer_config(config: Box<WriterConfig>) -> Result<u64>;
        fn root_remove_writer(wid: u64) -> bool;
        fn root_enable(wid: u64) -> Result<()>;
        fn root_disable(wid: u64) -> Result<()>;
        fn root_enable_type(tag: WriterTypeTag, data: &str) -> Result<()>;
        fn root_disable_type(tag: WriterTypeTag, data: &str) -> Result<()>;
        fn root_sync_type(tag: WriterTypeTag, data: &str, timeout: f64) -> Result<()>;
        fn root_sync_all(timeout: f64) -> Result<()>;
        fn root_rotate(path: &str) -> Result<()>;
        fn root_set_debug(debug: u8);
        fn root_get_server_auth_key() -> Vec<u8>;
        fn root_get_config_string() -> String;
        fn root_save_config(path: &str) -> Result<()>;
        fn root_trace(message: &str) -> Result<()>;
        fn root_debug(message: &str) -> Result<()>;
        fn root_info(message: &str) -> Result<()>;
        fn root_success(message: &str) -> Result<()>;
        fn root_warning(message: &str) -> Result<()>;
        fn root_error(message: &str) -> Result<()>;
        fn root_critical(message: &str) -> Result<()>;
        fn root_fatal(message: &str) -> Result<()>;
        fn root_exception(message: &str) -> Result<()>;
    }
}
