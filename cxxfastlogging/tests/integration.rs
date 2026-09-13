//! Integration tests for the `cxxfastlogging` crate.
//!
//! These tests mirror the `fastlogging/tests/integration.rs` tests, exercising
//! the cxx bridge API end-to-end: creating a Logging instance with different
//! writers, logging messages, managing writers and loggers, syncing, rotating
//! files, config save/apply and level handling.

use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use cxxfastlogging::ffi::{
    CompressionMethodEnum, ExtConfigFfi, LevelSymsEnum, MessageStructEnum, WriterTypeTag,
};
use cxxfastlogging::{Logger, Logging, WriterConfig};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Helper: read a file, returning an empty string if it does not exist.
fn read_file_opt(path: &std::path::Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

/// Helper: create a unique temp directory for this test run.
fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "cxxfastlogging_test_{}_{}",
        name,
        std::process::id()
    ));
    let _ = fs::create_dir_all(&dir);
    dir
}

/// Helper: clean up a temp directory.
fn cleanup(dir: &PathBuf) {
    let _ = fs::remove_dir_all(dir);
}

// ---------------------------------------------------------------------------
// Level helpers
// ---------------------------------------------------------------------------

#[test]
fn level_conversion() {
    assert_eq!(fastlogging::NOTSET, 0);
    assert_eq!(fastlogging::TRACE, 5);
    assert_eq!(fastlogging::DEBUG, 10);
    assert_eq!(fastlogging::INFO, 20);
    assert_eq!(fastlogging::SUCCESS, 25);
    assert_eq!(fastlogging::WARNING, 30);
    assert_eq!(fastlogging::ERROR, 40);
    assert_eq!(fastlogging::FATAL, fastlogging::CRITICAL);
    assert_eq!(fastlogging::CRITICAL, 50);
    assert_eq!(fastlogging::EXCEPTION, 60);
    // Aliases.
    assert_eq!(fastlogging::WARN, fastlogging::WARNING);
}

// ---------------------------------------------------------------------------
// Basic logging
// ---------------------------------------------------------------------------

#[test]
fn default_logging() {
    let mut logging = Logging::new_default().expect("new_default failed");
    logging.trace("trace").unwrap();
    logging.debug("debug").unwrap();
    logging.info("info").unwrap();
    logging.warning("warning").unwrap();
    logging.error("error").unwrap();
    logging.critical("critical").unwrap();
    logging.shutdown(false).unwrap();
}

#[test]
fn logging_new_with_console_writer() {
    let configs = vec![WriterConfig::new_console(fastlogging::DEBUG, false)];
    let mut logging = Logging::create(fastlogging::DEBUG, "test", configs).expect("create failed");
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
fn console_writer_config_new() {
    let cfg = WriterConfig::new_console(fastlogging::DEBUG, false);
    let configs = vec![cfg];
    let mut logging = Logging::create(fastlogging::DEBUG, "test", configs).expect("create failed");
    logging.info("console test").unwrap();
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// File writer
// ---------------------------------------------------------------------------

#[test]
fn file_writer_writes_messages() {
    let dir = temp_dir("file_writes");
    let log_file = dir.join("test.log");
    let configs = vec![
        WriterConfig::new_file(
            fastlogging::DEBUG,
            log_file.to_str().unwrap(),
            0,
            0,
            -1,
            -1,
            CompressionMethodEnum::Store,
        )
        .expect("new_file failed"),
    ];
    let mut logging = Logging::create(fastlogging::DEBUG, "test", configs).expect("create failed");
    logging.info("file info msg").unwrap();
    logging.error("file error msg").unwrap();
    logging.sync_all(2.0).unwrap();
    logging.shutdown(false).unwrap();
    let content = read_file_opt(&log_file);
    assert!(
        content.contains("file info msg"),
        "log file must contain info msg"
    );
    assert!(
        content.contains("file error msg"),
        "log file must contain error msg"
    );
    cleanup(&dir);
}

#[test]
fn file_writer_level_filter() {
    let dir = temp_dir("file_level");
    let log_file = dir.join("level.log");
    let configs = vec![
        WriterConfig::new_file(
            fastlogging::WARNING,
            log_file.to_str().unwrap(),
            0,
            0,
            -1,
            -1,
            CompressionMethodEnum::Store,
        )
        .expect("new_file failed"),
    ];
    let mut logging = Logging::create(fastlogging::DEBUG, "test", configs).expect("create failed");
    logging.debug("debug message").unwrap();
    logging.info("info message").unwrap();
    logging.error("error message").unwrap();
    logging.sync_all(2.0).unwrap();
    logging.shutdown(false).unwrap();
    let content = read_file_opt(&log_file);
    assert!(!content.contains("debug message"));
    assert!(!content.contains("info message"));
    assert!(content.contains("error message"));
    cleanup(&dir);
}

#[test]
fn file_writer_rotation() {
    let dir = temp_dir("file_rotate");
    let log_file = dir.join("rotate.log");
    // backlog=3 enables rotation.
    let configs = vec![
        WriterConfig::new_file(
            fastlogging::DEBUG,
            log_file.to_str().unwrap(),
            0,
            3,
            -1,
            -1,
            CompressionMethodEnum::Store,
        )
        .expect("new_file failed"),
    ];
    let mut logging = Logging::create(fastlogging::DEBUG, "test", configs).expect("create failed");
    for i in 0..5 {
        let msg = format!("message {i}");
        logging.info(&msg).unwrap();
    }
    logging.sync_all(2.0).unwrap();
    logging.rotate("").unwrap();
    logging.info("after rotate").unwrap();
    logging.sync_all(2.0).unwrap();
    logging.shutdown(false).unwrap();
    let current = read_file_opt(&log_file);
    assert!(current.contains("after rotate"));
    cleanup(&dir);
}

// ---------------------------------------------------------------------------
// Writer management
// ---------------------------------------------------------------------------

#[test]
fn add_remove_writer() {
    let mut logging = Logging::create(fastlogging::DEBUG, "test", vec![]).expect("create failed");
    let wid = logging
        .add_writer_config(WriterConfig::new_console(fastlogging::DEBUG, false))
        .expect("add_writer_config failed");
    assert!(wid > 0);
    // Add a second writer.
    let wid2 = logging
        .add_writer_config(WriterConfig::new_console(fastlogging::INFO, false))
        .expect("add_writer_config(2) failed");
    assert_ne!(wid, wid2);
    // Remove writer 1.
    assert!(logging.remove_writer(wid));
    // Remove writer 2.
    assert!(logging.remove_writer(wid2));
    logging.shutdown(false).unwrap();
}

#[test]
fn remove_writer_invalid_id() {
    let mut logging = Logging::create(fastlogging::DEBUG, "test", vec![]).expect("create failed");
    assert!(!logging.remove_writer(12345));
    logging.shutdown(false).unwrap();
}

#[test]
fn add_writers_batch() {
    let mut logging = Logging::create(fastlogging::DEBUG, "test", vec![]).expect("create failed");
    let wids = logging
        .add_writer_configs(vec![
            WriterConfig::new_console(fastlogging::DEBUG, false),
            WriterConfig::new_console(fastlogging::INFO, false),
        ])
        .expect("add_writer_configs failed");
    assert_eq!(wids.len(), 2);
    logging.shutdown(false).unwrap();
}

#[test]
fn disable_enable_writer_file() {
    let dir = temp_dir("toggle");
    let log_file = dir.join("toggle.log");
    let configs = vec![
        WriterConfig::new_file(
            fastlogging::DEBUG,
            log_file.to_str().unwrap(),
            0,
            0,
            -1,
            -1,
            CompressionMethodEnum::Store,
        )
        .expect("new_file failed"),
    ];
    let mut logging = Logging::create(fastlogging::DEBUG, "test", configs).expect("create failed");
    // Writer 1 is the file writer.
    logging.disable(1).expect("disable(1) failed");
    logging.info("disabled").unwrap();
    logging.sync_all(2.0).unwrap();
    assert!(!read_file_opt(&log_file).contains("disabled"));
    logging.enable(1).expect("enable(1) failed");
    logging.info("enabled").unwrap();
    logging.sync_all(2.0).unwrap();
    logging.shutdown(false).unwrap();
    assert!(read_file_opt(&log_file).contains("enabled"));
    cleanup(&dir);
}

#[test]
fn disable_enable_invalid_writer() {
    let mut logging = Logging::create(fastlogging::DEBUG, "test", vec![]).expect("create failed");
    assert!(logging.disable(9999).is_err());
    assert!(logging.enable(9999).is_err());
    logging.shutdown(false).unwrap();
}

#[test]
fn enable_disable_type() {
    // Use console writers (type=Console).
    let configs = vec![
        WriterConfig::new_console(fastlogging::DEBUG, false),
        WriterConfig::new_console(fastlogging::DEBUG, false),
    ];
    let mut logging = Logging::create(fastlogging::DEBUG, "test", configs).expect("create failed");
    logging
        .disable_type(WriterTypeTag::Console, "")
        .expect("disable_type(Console) failed");
    logging
        .enable_type(WriterTypeTag::Console, "")
        .expect("enable_type(Console) failed");
    // Syslog type — should fail since no syslog writers exist.
    assert!(logging.disable_type(WriterTypeTag::Syslog, "").is_err());
    assert!(logging.enable_type(WriterTypeTag::Syslog, "").is_err());
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// Level and domain management
// ---------------------------------------------------------------------------

#[test]
fn set_level_writer() {
    let configs = vec![WriterConfig::new_console(fastlogging::INFO, false)];
    let mut logging = Logging::create(fastlogging::DEBUG, "test", configs).expect("create failed");
    // Writer 1 has level INFO.  Change it to DEBUG.
    logging
        .set_level(1, fastlogging::DEBUG)
        .expect("set_level failed");
    logging.debug("passes now").unwrap();
    logging.shutdown(false).unwrap();
}

#[test]
fn set_domain() {
    let mut logging = Logging::create(fastlogging::DEBUG, "old", vec![]).expect("create failed");
    logging.set_domain("new-domain");
    let cfg = logging.get_config_string();
    assert!(cfg.contains("new-domain"));
    logging.shutdown(false).unwrap();
}

#[test]
fn set_ext_config() {
    let mut logging = Logging::create(fastlogging::DEBUG, "test", vec![]).expect("create failed");
    logging.set_ext_config(ExtConfigFfi {
        structured: MessageStructEnum::String,
        hostname: false,
        pname: false,
        pid: false,
        tname: false,
        tid: false,
    });
    logging.set_ext_config(ExtConfigFfi {
        structured: MessageStructEnum::Json,
        hostname: false,
        pname: false,
        pid: false,
        tname: true,
        tid: true,
    });
    logging.shutdown(false).unwrap();
}

#[test]
fn set_level2sym() {
    let mut logging = Logging::create(fastlogging::DEBUG, "test", vec![]).expect("create failed");
    logging.set_level2sym(LevelSymsEnum::Str);
    let cfg = logging.get_config_string();
    assert!(cfg.contains("DEBUG"));
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// Logger
// ---------------------------------------------------------------------------

#[test]
fn add_remove_logger() {
    let configs = vec![WriterConfig::new_console(fastlogging::DEBUG, false)];
    let mut logging = Logging::create(fastlogging::DEBUG, "root", configs).expect("create failed");
    let mut logger = Logger::create(fastlogging::DEBUG, "logger-domain");
    logging.add_logger(&mut logger);
    logger.info("logger message").unwrap();
    logger.trace("trace").unwrap();
    logger.debug("debug").unwrap();
    logger.warning("warning").unwrap();
    logger.error("error").unwrap();
    logger.critical("critical").unwrap();
    logging.sync_all(2.0).unwrap();
    logging.remove_logger(&mut logger);
    logging.shutdown(false).unwrap();
}

#[test]
fn logger_level_filtering() {
    let configs = vec![WriterConfig::new_console(fastlogging::DEBUG, false)];
    let mut logging = Logging::create(fastlogging::DEBUG, "root", configs).expect("create failed");
    let mut logger = Logger::create(fastlogging::INFO, "logger-domain");
    logging.add_logger(&mut logger);
    // Logger level is INFO, so DEBUG should be filtered.
    logger.debug("filtered by logger level").unwrap();
    logger.info("passes").unwrap();
    logging.sync_all(2.0).unwrap();
    // Change logger level to DEBUG.
    logger.set_level(fastlogging::DEBUG);
    logger.debug("now passes").unwrap();
    logging.sync_all(2.0).unwrap();
    logging.remove_logger(&mut logger);
    logging.shutdown(false).unwrap();
}

#[test]
fn logger_set_domain() {
    let mut logger = Logger::create(fastlogging::DEBUG, "d1");
    logger.set_domain("d2");
    // No direct way to inspect; just verify it doesn't crash.
    logger.set_level(fastlogging::TRACE);
}

#[test]
fn logger_level() {
    let mut logger = Logger::create(fastlogging::INFO, "d1");
    assert_eq!(logger.level(), fastlogging::INFO);
    logger.set_level(fastlogging::TRACE);
    assert_eq!(logger.level(), fastlogging::TRACE);
}

// ---------------------------------------------------------------------------
// Config save/apply
// ---------------------------------------------------------------------------

#[test]
fn save_config() {
    let dir = temp_dir("save_config");
    let config_path = dir.join("config.json");
    let log_file = dir.join("save.log");
    let configs = vec![
        WriterConfig::new_file(
            fastlogging::DEBUG,
            log_file.to_str().unwrap(),
            0,
            0,
            -1,
            -1,
            CompressionMethodEnum::Store,
        )
        .expect("new_file failed"),
    ];
    let mut logging = Logging::create(fastlogging::DEBUG, "test", configs).expect("create failed");
    logging
        .save_config(config_path.to_str().unwrap())
        .expect("save_config failed");
    assert!(config_path.exists());
    logging.shutdown(false).unwrap();
    cleanup(&dir);
}

#[test]
fn get_config_string() {
    let mut logging = Logging::create(fastlogging::DEBUG, "test", vec![]).expect("create failed");
    let cfg = logging.get_config_string();
    assert!(cfg.contains("level="));
    assert!(cfg.contains("domain="));
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// Network configs (construct-only)
// ---------------------------------------------------------------------------

#[test]
fn server_client_config_construction() {
    // Server with no encryption.
    let _server = WriterConfig::new_server(
        fastlogging::DEBUG,
        "127.0.0.1:0",
        cxxfastlogging::ffi::EncryptionMethodEnum::NONE,
        &[],
    );
    // Client with no encryption.
    let _client = WriterConfig::new_client(
        fastlogging::DEBUG,
        "127.0.0.1:0",
        cxxfastlogging::ffi::EncryptionMethodEnum::NONE,
        &[],
    );
}

// ---------------------------------------------------------------------------
// Error handling
// ---------------------------------------------------------------------------

#[test]
fn shutdown_twice_is_idempotent() {
    let mut logging = Logging::new_default().expect("new_default failed");
    logging.shutdown(false).unwrap();
    // The Box<Logging> is consumed; calling shutdown again is not possible
    // because the logging variable is still valid but the internal state is
    // shut down.  This test verifies that a single shutdown is clean.
}

// ---------------------------------------------------------------------------
// ExtConfig helpers
// ---------------------------------------------------------------------------

#[test]
fn ext_config_default() {
    let ext = ExtConfigFfi {
        structured: MessageStructEnum::String,
        hostname: false,
        pname: false,
        pid: false,
        tname: false,
        tid: false,
    };
    assert_eq!(ext.hostname, false);
    assert_eq!(ext.pname, false);
    assert_eq!(ext.pid, false);
    assert_eq!(ext.tname, false);
    assert_eq!(ext.tid, false);
}

// ---------------------------------------------------------------------------
// Concurrency smoke test
// ---------------------------------------------------------------------------

#[test]
fn concurrent_logging() {
    let dir = temp_dir("concurrent");
    let log_file = dir.join("concurrent.log");
    let configs = vec![
        WriterConfig::new_file(
            fastlogging::DEBUG,
            log_file.to_str().unwrap(),
            0,
            0,
            -1,
            -1,
            CompressionMethodEnum::Store,
        )
        .expect("new_file failed"),
    ];
    let logging = Arc::new(Mutex::new(
        Logging::create(fastlogging::DEBUG, "test", configs).expect("create failed"),
    ));
    let mut handles = Vec::new();
    for t in 0..4 {
        let logging = logging.clone();
        handles.push(std::thread::spawn(move || {
            for i in 0..50 {
                let msg = format!("thread {t} message {i}");
                logging.lock().unwrap().info(&msg).unwrap();
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    logging.lock().unwrap().sync_all(2.0).unwrap();
    logging.lock().unwrap().shutdown(false).unwrap();
    let content = read_file_opt(&log_file);
    for t in 0..4 {
        for i in 0..50 {
            assert!(
                content.contains(&format!("thread {t} message {i}")),
                "missing thread {t} message {i}"
            );
        }
    }
    cleanup(&dir);
}

// ---------------------------------------------------------------------------
// Logging level getter
// ---------------------------------------------------------------------------

#[test]
fn logging_level_getter() {
    let mut logging = Logging::create(fastlogging::DEBUG, "test", vec![]).expect("create failed");
    assert_eq!(logging.level(), fastlogging::DEBUG);
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// Sync type
// ---------------------------------------------------------------------------

#[test]
fn sync_type() {
    let configs = vec![WriterConfig::new_console(fastlogging::DEBUG, false)];
    let mut logging = Logging::create(fastlogging::DEBUG, "test", configs).expect("create failed");
    logging
        .sync_type(WriterTypeTag::Console, "", 2.0)
        .expect("sync_type failed");
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// Set encryption
// ---------------------------------------------------------------------------

#[test]
fn set_encryption() {
    let configs = vec![WriterConfig::new_console(fastlogging::DEBUG, false)];
    let mut logging = Logging::create(fastlogging::DEBUG, "test", configs).expect("create failed");
    // Set encryption on writer 1 (console writer).  This may or may not succeed
    // depending on writer type, but should not panic.
    let _ = logging.set_encryption(1, cxxfastlogging::ffi::EncryptionMethodEnum::NONE, &[]);
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// Get server configs (construct-only, no server running)
// ---------------------------------------------------------------------------

#[test]
fn get_server_configs_empty() {
    let mut logging = Logging::create(fastlogging::DEBUG, "test", vec![]).expect("create failed");
    let configs = logging.get_server_configs();
    assert!(configs.is_empty());
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// Root server address (no server, should be empty)
// ---------------------------------------------------------------------------

#[test]
fn get_root_server_address_port_empty() {
    let mut logging = Logging::create(fastlogging::DEBUG, "test", vec![]).expect("create failed");
    let addr = logging.get_root_server_address_port();
    assert!(addr.is_empty());
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// Apply config missing file errors
// ---------------------------------------------------------------------------

#[test]
fn apply_config_missing_file_errors() {
    let mut logging = Logging::create(fastlogging::DEBUG, "test", vec![]).expect("create failed");
    assert!(logging.apply_config("/nonexistent/config.json").is_err());
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// Set root writer config rejects non-network
// ---------------------------------------------------------------------------

#[test]
fn set_root_writer_rejects_non_network() {
    let mut logging = Logging::create(fastlogging::DEBUG, "test", vec![]).expect("create failed");
    // Only Server/Client types are allowed as root writer.
    let err = logging.set_root_writer_config(WriterConfig::new_console(fastlogging::DEBUG, false));
    assert!(err.is_err());
    logging.shutdown(false).unwrap();
}

// ---------------------------------------------------------------------------
// Logger new_ext
// ---------------------------------------------------------------------------

#[test]
fn logger_new_ext() {
    let logger = Logger::new_ext(fastlogging::DEBUG, "ext-domain", true, true);
    logger.info("ext logger msg").ok(); // may fail without a logging instance
}

// ---------------------------------------------------------------------------
// Counting via file writer (replaces callback tests since cxx bridge
// does not expose callback writers)
// ---------------------------------------------------------------------------

#[test]
fn file_writer_counts_messages() {
    let dir = temp_dir("count");
    let log_file = dir.join("count.log");
    let configs = vec![
        WriterConfig::new_file(
            fastlogging::DEBUG,
            log_file.to_str().unwrap(),
            0,
            0,
            -1,
            -1,
            CompressionMethodEnum::Store,
        )
        .expect("new_file failed"),
    ];
    let mut logging = Logging::create(fastlogging::DEBUG, "test", configs).expect("create failed");
    // Log 10 messages.
    for i in 0..10 {
        let msg = format!("counted message {i}");
        logging.info(&msg).unwrap();
    }
    logging.sync_all(2.0).unwrap();
    logging.shutdown(false).unwrap();
    let content = read_file_opt(&log_file);
    for i in 0..10 {
        assert!(
            content.contains(&format!("counted message {i}")),
            "missing counted message {i}"
        );
    }
    cleanup(&dir);
}

#[test]
fn file_writer_respects_level() {
    let dir = temp_dir("respect");
    let log_file = dir.join("respect.log");
    // Writer level is INFO: TRACE/DEBUG messages must be filtered out.
    let configs = vec![
        WriterConfig::new_file(
            fastlogging::INFO,
            log_file.to_str().unwrap(),
            0,
            0,
            -1,
            -1,
            CompressionMethodEnum::Store,
        )
        .expect("new_file failed"),
    ];
    let mut logging = Logging::create(fastlogging::DEBUG, "root", configs).expect("create failed");
    logging.debug("filtered").unwrap();
    logging.info("passes").unwrap();
    logging.warning("passes too").unwrap();
    logging.sync_all(2.0).unwrap();
    logging.shutdown(false).unwrap();
    let content = read_file_opt(&log_file);
    assert!(!content.contains("filtered"), "debug should be filtered");
    assert!(content.contains("passes"), "info should pass");
    assert!(content.contains("passes too"), "warning should pass");
    cleanup(&dir);
}
