//! Integration tests for the `fastlogging` crate.
//!
//! These tests exercise the public API end-to-end: creating a `Logging`
//! instance with different writers, logging messages, managing writers and
//! loggers, syncing, rotating files, config save/apply and level handling.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use fastlogging::{
    CallbackWriterConfig, ClientWriterConfig, ConsoleWriterConfig, EncryptionMethod, ExtConfig,
    FileWriterConfig, LevelSyms, Logging, LoggingError, MessageStructEnum, RootConfig,
    ServerConfig, WriterConfigEnum, WriterTypeEnum,
};

/// Helper: read a file, returning an empty string if it does not exist yet.
fn read_file_opt(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

/// Helper: build a callback writer config that counts invocations.
fn counting_callback(count: Arc<AtomicUsize>) -> CallbackWriterConfig {
    CallbackWriterConfig::new(
        fastlogging::DEBUG,
        Some(Box::new(
            move |_level: u8, _domain: String, _message: String| {
                count.fetch_add(1, Ordering::SeqCst);
                Ok(())
            },
        )),
    )
}

/// Helper: first writer id of the given logging instance.
fn first_wid(logging: &Logging) -> usize {
    *logging.get_writer_configs().keys().next().unwrap()
}

// ---------------------------------------------------------------------------
// Level helpers
// ---------------------------------------------------------------------------

#[test]
fn level_conversion() {
    assert_eq!(fastlogging::level2str(fastlogging::NOTSET), "NOTSET");
    assert_eq!(fastlogging::level2str(fastlogging::TRACE), "TRACE");
    assert_eq!(fastlogging::level2str(fastlogging::DEBUG), "DEBUG");
    assert_eq!(fastlogging::level2str(fastlogging::INFO), "INFO");
    assert_eq!(fastlogging::level2str(fastlogging::SUCCESS), "SUCCESS");
    assert_eq!(fastlogging::level2str(fastlogging::WARNING), "WARNING");
    assert_eq!(fastlogging::level2str(fastlogging::ERROR), "ERROR");
    assert_eq!(fastlogging::level2str(fastlogging::FATAL), "FATAL");
    assert_eq!(fastlogging::level2str(fastlogging::CRITICAL), "FATAL");
    assert_eq!(fastlogging::level2str(fastlogging::EXCEPTION), "EXCEPTION");
    assert_eq!(fastlogging::level2str(200), "NOLOG");
    // Aliases.
    assert_eq!(fastlogging::WARN, fastlogging::WARNING);
    assert_eq!(fastlogging::CRITICAL, fastlogging::FATAL);
    // Short forms.
    assert_eq!(fastlogging::level2short(fastlogging::DEBUG), "DBG");
    assert_eq!(fastlogging::level2short(fastlogging::ERROR), "ERR");
    // Symbols.
    assert_eq!(fastlogging::level2sym(fastlogging::DEBUG), "D");
    assert_eq!(fastlogging::level2sym(fastlogging::ERROR), "E");
    // level2string.
    assert_eq!(
        fastlogging::level2string(&LevelSyms::Str, fastlogging::INFO),
        "INFO"
    );
    assert_eq!(
        fastlogging::level2string(&LevelSyms::Short, fastlogging::INFO),
        "INF"
    );
    assert_eq!(
        fastlogging::level2string(&LevelSyms::Sym, fastlogging::INFO),
        "I"
    );
}

// ---------------------------------------------------------------------------
// Basic logging
// ---------------------------------------------------------------------------

#[test]
fn default_logging() {
    let mut logging = Logging::default();
    logging.trace("trace").unwrap();
    logging.debug("debug").unwrap();
    logging.info("info").unwrap();
    logging.warning("warning").unwrap();
    logging.error("error").unwrap();
    logging.critical("critical").unwrap();
    logging.shutdown(false).unwrap();
}

#[test]
fn init_logging() {
    // Logging::init() is a convenience constructor with a console writer.
    let mut logging = Logging::init().unwrap();
    logging.info("hello").unwrap();
    logging.shutdown(false).unwrap();
}

#[test]
fn logging_new_default() {
    let mut logging = fastlogging::logging_new_default().unwrap();
    logging.info("hello").unwrap();
    logging.shutdown(false).unwrap();
}

#[test]
fn logging_new_with_console_writer() {
    let mut logging = Logging::new(
        fastlogging::DEBUG,
        "test",
        Some(vec![
            ConsoleWriterConfig::new(fastlogging::DEBUG, false).into(),
        ]),
        None,
        None,
    )
    .unwrap();
    logging.trace("trace msg").unwrap();
    logging.debug("debug msg").unwrap();
    logging.info("info msg").unwrap();
    logging.success("success msg").unwrap();
    logging.warning("warning msg").unwrap();
    logging.error("error msg").unwrap();
    logging.critical("critical msg").unwrap();
    logging.fatal("fatal msg").unwrap();
    logging.shutdown(false).unwrap();
}

#[test]
fn console_writer_config_defaults() {
    let config = ConsoleWriterConfig::new(fastlogging::DEBUG, false);
    assert!(matches!(
        config.target,
        fastlogging::console::ConsoleTargetEnum::StdOut
    ));
    assert!(config.enabled);
    assert_eq!(config.level, fastlogging::DEBUG);
}

// ---------------------------------------------------------------------------
// Callback writer
// ---------------------------------------------------------------------------

#[test]
fn callback_writer_receives_messages() {
    let received: Arc<Mutex<Vec<(u8, String, String)>>> = Arc::new(Mutex::new(Vec::new()));
    let received_clone = received.clone();
    let callback = move |level: u8, domain: String, message: String| {
        received_clone
            .lock()
            .unwrap()
            .push((level, domain, message));
        Ok(())
    };
    let mut logging = Logging::new(
        fastlogging::DEBUG,
        "cb-domain",
        Some(vec![
            CallbackWriterConfig::new(fastlogging::DEBUG, Some(Box::new(callback))).into(),
        ]),
        None,
        None,
    )
    .unwrap();
    logging.info("callback msg").unwrap();
    logging.sync_all(2.0).unwrap();
    logging.shutdown(false).unwrap();
    let received = received.lock().unwrap();
    assert_eq!(received.len(), 1, "callback must have been invoked once");
    assert_eq!(received[0].0, fastlogging::INFO);
    assert_eq!(received[0].1, "cb-domain");
    assert!(
        received[0].2.contains("callback msg"),
        "formatted message must contain the raw message, got: {}",
        received[0].2
    );
}

#[test]
fn callback_writer_respects_level() {
    // Writer level is INFO: TRACE/DEBUG messages must be filtered out.
    let count = Arc::new(AtomicUsize::new(0));
    let mut logging = Logging::new(
        fastlogging::DEBUG, // global level: everything passes
        "root",
        Some(vec![
            CallbackWriterConfig::new(
                fastlogging::INFO,
                Some(Box::new({
                    let count = count.clone();
                    move |_level: u8, _domain: String, _message: String| {
                        count.fetch_add(1, Ordering::SeqCst);
                        Ok(())
                    }
                })),
            )
            .into(),
        ]),
        None,
        None,
    )
    .unwrap();
    logging.debug("filtered").unwrap();
    logging.info("passes").unwrap();
    logging.warning("passes too").unwrap();
    logging.sync_all(2.0).unwrap();
    logging.shutdown(false).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 2);
}

// ---------------------------------------------------------------------------
// File writer
// ---------------------------------------------------------------------------

#[test]
fn file_writer_writes_messages() {
    let temp_dir = tempfile::TempDir::with_prefix("fastlogging_test").unwrap();
    let log_file = temp_dir.path().join("test.log");
    let mut logging = Logging::new(
        fastlogging::DEBUG,
        "test",
        Some(vec![
            FileWriterConfig::new(fastlogging::DEBUG, log_file.clone(), 0, 0, None, None, None)
                .unwrap()
                .into(),
        ]),
        None,
        None,
    )
    .unwrap();
    // Use long level names for assertions.
    logging.set_level2sym(&LevelSyms::Str);
    logging.info("file info msg").unwrap();
    logging.error("file error msg").unwrap();
    logging.sync_all(2.0).unwrap();
    logging.shutdown(false).unwrap();
    let content = std::fs::read_to_string(&log_file).unwrap();
    assert!(
        content.contains("file info msg"),
        "log file must contain info msg"
    );
    assert!(
        content.contains("file error msg"),
        "log file must contain error msg"
    );
    assert!(
        content.contains("INFO"),
        "log file must contain level name INFO"
    );
    assert!(
        content.contains("ERROR"),
        "log file must contain level name ERROR"
    );
}

#[test]
fn file_writer_domain_filter() {
    // The domain filter is applied to the domain of each log message.
    // We use two loggers with different domains to exercise the filter.
    let temp_dir = tempfile::TempDir::with_prefix("fastlogging_test").unwrap();
    let log_file = temp_dir.path().join("filtered.log");
    let config =
        FileWriterConfig::new(fastlogging::DEBUG, log_file.clone(), 0, 0, None, None, None)
            .unwrap();
    // domain_filter is private; set it via a serde round-trip.
    let json = serde_json::to_string(&config).unwrap();
    let mut value: serde_json::Value = serde_json::from_str(&json).unwrap();
    value["domain_filter"] = serde_json::Value::String("keep".to_string());
    let config: FileWriterConfig = serde_json::from_value(value).unwrap();

    let mut logging = Logging::new(
        fastlogging::DEBUG,
        "root",
        Some(vec![WriterConfigEnum::File(config)]),
        None,
        None,
    )
    .unwrap();
    let mut keep_logger = fastlogging::Logger::new(fastlogging::DEBUG, "keep");
    let mut other_logger = fastlogging::Logger::new(fastlogging::DEBUG, "other");
    logging.add_logger(&mut keep_logger);
    logging.add_logger(&mut other_logger);
    keep_logger.info("kept message").unwrap();
    other_logger.info("dropped message").unwrap();
    keep_logger.flush(1.0);
    other_logger.flush(1.0);
    logging.sync_all(2.0).unwrap();
    logging.remove_logger(&mut keep_logger);
    logging.remove_logger(&mut other_logger);
    logging.shutdown(false).unwrap();
    let content = std::fs::read_to_string(&log_file).unwrap();
    assert!(content.contains("kept message"));
    assert!(!content.contains("dropped message"));
}

#[test]
fn file_writer_level_filter() {
    let temp_dir = tempfile::TempDir::with_prefix("fastlogging_test").unwrap();
    let log_file = temp_dir.path().join("level.log");
    let mut logging = Logging::new(
        fastlogging::DEBUG,
        "test",
        Some(vec![
            FileWriterConfig::new(
                fastlogging::WARNING, // only >= WARNING reaches the file
                log_file.clone(),
                0,
                0,
                None,
                None,
                None,
            )
            .unwrap()
            .into(),
        ]),
        None,
        None,
    )
    .unwrap();
    logging.debug("debug message").unwrap();
    logging.info("info message").unwrap();
    logging.error("error message").unwrap();
    logging.sync_all(2.0).unwrap();
    logging.shutdown(false).unwrap();
    let content = std::fs::read_to_string(&log_file).unwrap();
    assert!(!content.contains("debug message"));
    assert!(!content.contains("info message"));
    assert!(content.contains("error message"));
}

#[test]
fn file_writer_rotation() {
    let temp_dir = tempfile::TempDir::with_prefix("fastlogging_test").unwrap();
    let log_file = temp_dir.path().join("rotate.log");
    // backlog=3 enables rotation.
    let mut logging = Logging::new(
        fastlogging::DEBUG,
        "test",
        Some(vec![
            FileWriterConfig::new(fastlogging::DEBUG, log_file.clone(), 0, 3, None, None, None)
                .unwrap()
                .into(),
        ]),
        None,
        None,
    )
    .unwrap();
    for i in 0..5 {
        logging.info(format!("message {i}")).unwrap();
    }
    logging.sync_all(2.0).unwrap();
    logging.rotate(None).unwrap();
    logging.info("after rotate").unwrap();
    logging.sync_all(2.0).unwrap();
    logging.shutdown(false).unwrap();
    // Current file contains the new message but not the old ones.
    let current = std::fs::read_to_string(&log_file).unwrap();
    assert!(current.contains("after rotate"));
    assert!(!current.contains("message 0"));
    // The rotated content is stored (uncompressed) in the backlog zip file.
    let backlog = log_file.with_extension("log.1");
    assert!(backlog.exists(), "backlog file must exist after rotation");
    let zip_file = std::fs::File::open(&backlog).unwrap();
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();
    assert_eq!(archive.len(), 1);
    let mut content = String::new();
    std::io::Read::read_to_string(&mut archive.by_index(0).unwrap(), &mut content).unwrap();
    assert!(content.contains("message 0"));
}

#[test]
fn file_writer_config_validation() {
    // backlog=0 with rotation parameters must be rejected.
    let err = FileWriterConfig::new(
        fastlogging::DEBUG,
        PathBuf::from("x.log"),
        1000, // size > 0 => rotation requested
        0,    // but backlog == 0
        None,
        None,
        None,
    )
    .unwrap_err();
    assert!(matches!(err, LoggingError::InvalidValue(_)));
    // backlog > BACKLOG_MAX must be rejected.
    let err = FileWriterConfig::new(
        fastlogging::DEBUG,
        PathBuf::from("x.log"),
        1000,
        1001,
        None,
        None,
        None,
    )
    .unwrap_err();
    assert!(matches!(err, LoggingError::InvalidValue(_)));
}

// ---------------------------------------------------------------------------
// Writer management
// ---------------------------------------------------------------------------

#[test]
fn add_remove_writer() {
    let mut logging = Logging::new(fastlogging::DEBUG, "test", None, None, None).unwrap();
    // No writers yet.
    assert!(logging.get_writer_configs().is_empty());
    let wid = logging
        .add_writer_config(&ConsoleWriterConfig::new(fastlogging::DEBUG, false).into())
        .unwrap();
    assert!(wid > 0);
    let cfg = logging.get_writer_config(wid).expect("writer exists");
    assert!(matches!(cfg, WriterConfigEnum::Console { .. }));
    // Adding the same config again yields a different writer id.
    let wid2 = logging
        .add_writer_config(&ConsoleWriterConfig::new(fastlogging::DEBUG, false).into())
        .unwrap();
    assert_ne!(wid, wid2);
    let removed = logging.remove_writer(wid);
    assert!(removed.is_some());
    assert!(logging.get_writer_config(wid).is_none());
    assert!(logging.get_writer_config(wid2).is_some());
    logging.shutdown(false).unwrap();
}

#[test]
fn remove_writer_invalid_id() {
    let mut logging = Logging::new(fastlogging::DEBUG, "test", None, None, None).unwrap();
    assert!(logging.remove_writer(12345).is_none());
    assert!(logging.get_writer_config(12345).is_none());
    logging.shutdown(false).unwrap();
}

#[test]
fn add_writers_batch() {
    let mut logging = Logging::new(fastlogging::DEBUG, "test", None, None, None).unwrap();
    let wids = logging
        .add_writer_configs(vec![
            ConsoleWriterConfig::new(fastlogging::DEBUG, false).into(),
            ConsoleWriterConfig::new(fastlogging::INFO, false).into(),
        ])
        .unwrap();
    assert_eq!(wids.len(), 2);
    assert_eq!(logging.get_writer_configs().len(), 2);
    let removed = logging.remove_writers(Some(wids));
    assert_eq!(removed.len(), 2);
    assert!(logging.get_writer_configs().is_empty());
    logging.shutdown(false).unwrap();
}

#[test]
fn remove_all_writers() {
    let mut logging = Logging::new(
        fastlogging::DEBUG,
        "test",
        Some(vec![
            ConsoleWriterConfig::new(fastlogging::DEBUG, false).into(),
            ConsoleWriterConfig::new(fastlogging::INFO, false).into(),
        ]),
        None,
        None,
    )
    .unwrap();
    assert_eq!(logging.get_writer_configs().len(), 2);
    // remove_writers(None) removes all writers.
    let removed = logging.remove_writers(None);
    assert_eq!(removed.len(), 2);
    assert!(logging.get_writer_configs().is_empty());
    logging.shutdown(false).unwrap();
}

#[test]
fn disable_enable_writer_file() {
    // disable()/enable() take effect for file writers.
    let temp_dir = tempfile::TempDir::with_prefix("fastlogging_test").unwrap();
    let log_file = temp_dir.path().join("toggle.log");
    let mut logging = Logging::new(
        fastlogging::DEBUG,
        "test",
        Some(vec![
            FileWriterConfig::new(fastlogging::DEBUG, log_file.clone(), 0, 0, None, None, None)
                .unwrap()
                .into(),
        ]),
        None,
        None,
    )
    .unwrap();
    let wid = first_wid(&logging);
    logging.disable(wid).unwrap();
    logging.info("disabled").unwrap();
    logging.sync_all(2.0).unwrap();
    assert!(!read_file_opt(&log_file).contains("disabled"));
    logging.enable(wid).unwrap();
    logging.info("enabled").unwrap();
    logging.sync_all(2.0).unwrap();
    logging.shutdown(false).unwrap();
    assert!(read_file_opt(&log_file).contains("enabled"));
}

#[test]
fn disable_enable_invalid_writer() {
    let mut logging = Logging::new(fastlogging::DEBUG, "test", None, None, None).unwrap();
    assert!(logging.disable(9999).is_err());
    assert!(logging.enable(9999).is_err());
    logging.shutdown(false).unwrap();
}

#[test]
fn enable_disable_type() {
    // disable_type()/enable_type() operate on all writers of a type.
    let mut logging = Logging::new(
        fastlogging::DEBUG,
        "test",
        Some(vec![
            CallbackWriterConfig::new(fastlogging::DEBUG, None).into(),
            CallbackWriterConfig::new(fastlogging::DEBUG, None).into(),
        ]),
        None,
        None,
    )
    .unwrap();
    assert_eq!(logging.get_writer_configs().len(), 2);
    logging.disable_type(WriterTypeEnum::Callback).unwrap();
    logging.enable_type(WriterTypeEnum::Callback).unwrap();
    // Unknown types yield an error.
    assert!(logging.disable_type(WriterTypeEnum::Syslog).is_err());
    assert!(logging.enable_type(WriterTypeEnum::Syslog).is_err());
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// Level and domain management
// ---------------------------------------------------------------------------

#[test]
fn set_level_global() {
    let count = Arc::new(AtomicUsize::new(0));
    let mut logging = Logging::new(
        fastlogging::INFO, // only INFO+ passes
        "test",
        Some(vec![counting_callback(count.clone()).into()]),
        None,
        None,
    )
    .unwrap();
    logging.debug("filtered by global level").unwrap();
    logging.info("passes").unwrap();
    logging.sync_all(2.0).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    // Add a Root writer so the global level can be changed.
    let root_wid = logging
        .add_writer_config(&WriterConfigEnum::Root(RootConfig::default()))
        .unwrap();
    logging.set_level(root_wid, fastlogging::DEBUG).unwrap();
    assert_eq!(logging.level, fastlogging::DEBUG);
    logging.debug("now passes").unwrap();
    logging.sync_all(2.0).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 2);
    logging.shutdown(false).unwrap();
}

#[test]
fn set_level_writer() {
    let count = Arc::new(AtomicUsize::new(0));
    let mut logging = Logging::new(
        fastlogging::DEBUG,
        "test",
        Some(vec![
            CallbackWriterConfig::new(
                fastlogging::INFO,
                Some(Box::new({
                    let count = count.clone();
                    move |_level: u8, _domain: String, _message: String| {
                        count.fetch_add(1, Ordering::SeqCst);
                        Ok(())
                    }
                })),
            )
            .into(),
        ]),
        None,
        None,
    )
    .unwrap();
    let wid = first_wid(&logging);
    logging.debug("filtered").unwrap();
    logging.sync_all(2.0).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 0);
    // Lower the writer's level to DEBUG.
    logging.set_level(wid, fastlogging::DEBUG).unwrap();
    logging.debug("passes now").unwrap();
    logging.sync_all(2.0).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    // Unknown writer id.
    assert!(logging.set_level(9999, fastlogging::DEBUG).is_err());
    logging.shutdown(false).unwrap();
}

#[test]
fn set_domain_updates_instance_config() {
    let mut logging = Logging::new(fastlogging::DEBUG, "old", None, None, None).unwrap();
    logging.set_domain("new-domain");
    let cfg = logging.get_config_string();
    assert!(cfg.contains("new-domain"));
    logging.shutdown(false).unwrap();
}

#[test]
fn set_ext_config() {
    let mut logging = Logging::new(fastlogging::DEBUG, "test", None, None, None).unwrap();
    logging.set_ext_config(&ExtConfig::new(
        MessageStructEnum::String,
        false,
        false,
        false,
        false,
        false,
    ));
    logging.set_ext_config(&ExtConfig::new(
        MessageStructEnum::Json,
        false,
        false,
        false,
        true,
        true,
    ));
    logging.shutdown(false).unwrap();
}

#[test]
fn set_level2sym() {
    let mut logging = Logging::new(fastlogging::DEBUG, "test", None, None, None).unwrap();
    logging.set_level2sym(&LevelSyms::Str);
    let cfg = logging.get_config_string();
    assert!(cfg.contains("DEBUG"));
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// Logger
// ---------------------------------------------------------------------------

#[test]
fn add_remove_logger() {
    let count = Arc::new(AtomicUsize::new(0));
    let mut logging = Logging::new(
        fastlogging::DEBUG,
        "root",
        Some(vec![counting_callback(count.clone()).into()]),
        None,
        None,
    )
    .unwrap();
    let mut logger = fastlogging::Logger::new(fastlogging::DEBUG, "logger-domain");
    // Before registration the logger has no channel and reports an error.
    assert!(logger.info("not registered").is_err());
    logging.add_logger(&mut logger);
    logger.info("logger message").unwrap();
    logger.trace("trace").unwrap();
    logger.debug("debug").unwrap();
    logger.warning("warning").unwrap();
    logger.error("error").unwrap();
    logger.critical("critical").unwrap();
    logger.flush(1.0);
    logging.sync_all(2.0).unwrap();
    logging.remove_logger(&mut logger);
    assert!(logger.info("removed").is_err());
    logging.shutdown(false).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 5);
}

#[test]
fn logger_level_filtering() {
    let count = Arc::new(AtomicUsize::new(0));
    let mut logging = Logging::new(
        fastlogging::DEBUG,
        "root",
        Some(vec![counting_callback(count.clone()).into()]),
        None,
        None,
    )
    .unwrap();
    let mut logger = fastlogging::Logger::new(fastlogging::INFO, "logger-domain");
    logging.add_logger(&mut logger);
    logger.debug("filtered by logger level").unwrap();
    logger.info("passes").unwrap();
    logger.flush(1.0);
    logging.sync_all(2.0).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    // Change logger level.
    logger.set_level(fastlogging::DEBUG);
    logger.debug("now passes").unwrap();
    logger.flush(1.0);
    logging.sync_all(2.0).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 2);
    logging.remove_logger(&mut logger);
    logging.shutdown(false).unwrap();
}

#[test]
fn logger_set_domain() {
    let mut logger = fastlogging::Logger::new(fastlogging::DEBUG, "d1");
    logger.set_domain("d2");
    let repr = logger.__repr__();
    assert!(repr.contains("d2"));
}

#[test]
fn logger_level() {
    let mut logger = fastlogging::Logger::new(fastlogging::INFO, "d1");
    assert_eq!(logger.level(), fastlogging::INFO);
    logger.set_level(fastlogging::TRACE);
    assert_eq!(logger.level(), fastlogging::TRACE);
}

// ---------------------------------------------------------------------------
// Config save/apply/roundtrip
// ---------------------------------------------------------------------------

#[test]
fn save_apply_config_roundtrip() {
    let temp_dir = tempfile::TempDir::with_prefix("fastlogging_test").unwrap();
    let config_path = temp_dir.path().join("config.json");
    let log_file = temp_dir.path().join("roundtrip.log");
    let mut logging = Logging::new(
        fastlogging::DEBUG,
        "test",
        Some(vec![
            FileWriterConfig::new(fastlogging::DEBUG, log_file.clone(), 0, 0, None, None, None)
                .unwrap()
                .into(),
        ]),
        None,
        None,
    )
    .unwrap();
    logging.save_config(Some(&config_path)).unwrap();
    assert!(config_path.exists());
    // Create a second instance and apply the saved config.
    let mut restored = Logging::new(fastlogging::NOTSET, "ignored", None, None, None).unwrap();
    restored.apply_config(&config_path).unwrap();
    restored.info("after restore").unwrap();
    restored.info("roundtrip").unwrap();
    restored.sync_all(2.0).unwrap();
    restored.shutdown(false).unwrap();
    logging.shutdown(false).unwrap();
    let content = std::fs::read_to_string(&log_file).unwrap();
    assert!(content.contains("after restore"));
    assert!(content.contains("roundtrip"));
}

#[test]
fn get_config_string() {
    let mut logging = Logging::new(fastlogging::DEBUG, "test", None, None, None).unwrap();
    let cfg = logging.get_config_string();
    assert!(cfg.contains("level="));
    assert!(cfg.contains("domain="));
    logging.shutdown(false).unwrap();
}

#[test]
fn apply_config_missing_file_errors() {
    let mut logging = Logging::new(fastlogging::DEBUG, "test", None, None, None).unwrap();
    let err = logging.apply_config(std::path::Path::new("/nonexistent/config.json"));
    assert!(err.is_err());
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// Network configs (construct-only)
// ---------------------------------------------------------------------------

#[test]
fn server_client_config_construction() {
    let server = ServerConfig::new(fastlogging::DEBUG, "127.0.0.1", EncryptionMethod::NONE);
    let cfg = WriterConfigEnum::Server(server.clone());
    assert!(matches!(cfg, WriterConfigEnum::Server(_)));
    let client = ClientWriterConfig::new(
        fastlogging::DEBUG,
        "127.0.0.1",
        EncryptionMethod::AuthKey(vec![1, 2, 3]),
    );
    assert_eq!(client.port, 0);
    let cfg = WriterConfigEnum::Client(client);
    assert!(matches!(cfg, WriterConfigEnum::Client(_)));
    // Encryption methods exist.
    match EncryptionMethod::AuthKey(vec![1, 2, 3]) {
        EncryptionMethod::AuthKey(key) => assert_eq!(key, vec![1, 2, 3]),
        _ => panic!("wrong encryption variant"),
    }
    let _ = server;
}

// ---------------------------------------------------------------------------
// Error handling
// ---------------------------------------------------------------------------

#[test]
fn set_root_writer_rejects_non_network() {
    let mut logging = Logging::new(fastlogging::DEBUG, "test", None, None, None).unwrap();
    // Only Server/Client types are allowed as root writer.
    let err = logging.set_root_writer_config(&WriterConfigEnum::Console(ConsoleWriterConfig::new(
        fastlogging::DEBUG,
        false,
    )));
    assert!(err.is_err());
    logging.shutdown(false).unwrap();
}

#[test]
fn get_server_config_invalid_writer() {
    let mut logging = Logging::new(fastlogging::DEBUG, "test", None, None, None).unwrap();
    assert!(logging.get_server_config(9999).is_err());
    logging.shutdown(false).unwrap();
}

#[test]
fn shutdown_twice_is_idempotent() {
    let mut logging = Logging::init().unwrap();
    logging.shutdown(false).unwrap();
    logging.shutdown(false).unwrap();
}

#[test]
fn logging_is_debug_repr() {
    let mut logging = Logging::init().unwrap();
    let repr = format!("{logging:?}");
    assert!(repr.contains("Logging"));
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// ExtConfig helpers
// ---------------------------------------------------------------------------

#[test]
fn ext_config_default() {
    let ext = ExtConfig::default();
    assert!(!ext.hostname);
    assert!(!ext.pname);
    assert!(!ext.pid);
    assert!(!ext.tname);
    assert!(!ext.tid);
    assert!(matches!(ext.structured, MessageStructEnum::String));
}

#[test]
fn writer_config_from_conversions() {
    let console = ConsoleWriterConfig::new(fastlogging::DEBUG, false);
    let _: WriterConfigEnum = console.into();
    let file = FileWriterConfig::new(
        fastlogging::DEBUG,
        PathBuf::from("x.log"),
        0,
        0,
        None,
        None,
        None,
    )
    .unwrap();
    let _: WriterConfigEnum = file.into();
    let callback = CallbackWriterConfig::new(fastlogging::DEBUG, None);
    let _: WriterConfigEnum = callback.into();
}

// ---------------------------------------------------------------------------
// Concurrency smoke test
// ---------------------------------------------------------------------------

#[test]
fn concurrent_logging() {
    let temp_dir = tempfile::TempDir::with_prefix("fastlogging_test").unwrap();
    let log_file = temp_dir.path().join("concurrent.log");
    let logging = Logging::new(
        fastlogging::DEBUG,
        "test",
        Some(vec![
            FileWriterConfig::new(fastlogging::DEBUG, log_file.clone(), 0, 0, None, None, None)
                .unwrap()
                .into(),
        ]),
        None,
        None,
    )
    .unwrap();
    let logging = Arc::new(Mutex::new(logging));
    let mut handles = Vec::new();
    for t in 0..4 {
        let logging = logging.clone();
        handles.push(std::thread::spawn(move || {
            for i in 0..50 {
                logging
                    .lock()
                    .unwrap()
                    .info(format!("thread {t} message {i}"))
                    .unwrap();
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    logging.lock().unwrap().sync_all(2.0).unwrap();
    logging.lock().unwrap().shutdown(false).unwrap();
    let content = std::fs::read_to_string(&log_file).unwrap();
    for t in 0..4 {
        for i in 0..50 {
            assert!(
                content.contains(&format!("thread {t} message {i}")),
                "missing thread {t} message {i}"
            );
        }
    }
}
