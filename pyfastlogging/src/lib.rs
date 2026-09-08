use pyo3::prelude::*;

mod def;
pub use def::{EncryptionMethod, LevelSyms, WriterConfigEnum, WriterTypeEnum};
mod writer;
use writer::{CallbackWriterConfig, ExtConfig};
pub use writer::{ClientWriterConfig, ConsoleWriterConfig, FileWriterConfig, ServerConfig};
mod error;
pub use error::LoggingError;
mod logger;
mod logging;
pub mod root;

// Python layer for fastlogging.

/// This function is called when Python interpreter exits. The fastlogging module is shutdown.
#[pyfunction]
fn shutdown_at_exit() -> Result<(), LoggingError> {
    Ok(fastlogging::root::shutdown(false)?)
}

/// Python API
#[pymodule]
#[pyo3(name = "pyfastlogging")]
fn init(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("EXCEPTION", fastlogging::EXCEPTION)?;
    m.add("CRITICAL", fastlogging::CRITICAL)?;
    m.add("FATAL", fastlogging::FATAL)?;
    m.add("ERROR", fastlogging::ERROR)?;
    m.add("WARNING", fastlogging::WARNING)?;
    m.add("WARN", fastlogging::WARN)?;
    m.add("SUCCESS", fastlogging::SUCCESS)?;
    m.add("INFO", fastlogging::INFO)?;
    m.add("DEBUG", fastlogging::DEBUG)?;
    m.add("TRACE", fastlogging::TRACE)?;
    m.add("NOTSET", fastlogging::NOTSET)?;
    m.add_class::<def::Level2Sym>()?;
    m.add_class::<def::MessageStructEnum>()?;
    m.add_class::<def::CompressionMethodEnum>()?;
    m.add_class::<def::EncryptionMethod>()?;
    m.add_class::<def::WriterTypeEnum>()?;
    m.add_class::<def::WriterConfigEnum>()?;
    m.add_class::<ExtConfig>()?;
    m.add_class::<ConsoleWriterConfig>()?;
    m.add_class::<FileWriterConfig>()?;
    m.add_class::<ServerConfig>()?;
    m.add_class::<ClientWriterConfig>()?;
    m.add_class::<CallbackWriterConfig>()?;
    m.add_class::<logging::Logging>()?;
    m.add_class::<logger::Logger>()?;
    m.add_function(wrap_pyfunction!(root::root_init, m)?)?;
    m.add_function(wrap_pyfunction!(root::shutdown, m)?)?;
    m.add_function(wrap_pyfunction!(root::set_level, m)?)?;
    m.add_function(wrap_pyfunction!(root::set_root_level, m)?)?;
    m.add_function(wrap_pyfunction!(root::set_domain, m)?)?;
    m.add_function(wrap_pyfunction!(root::set_level2sym, m)?)?;
    m.add_function(wrap_pyfunction!(root::set_ext_config, m)?)?;
    m.add_function(wrap_pyfunction!(root::add_logger, m)?)?;
    m.add_function(wrap_pyfunction!(root::remove_logger, m)?)?;
    m.add_function(wrap_pyfunction!(root::set_root_writer, m)?)?;
    m.add_function(wrap_pyfunction!(root::add_writer, m)?)?;
    m.add_function(wrap_pyfunction!(root::remove_writer, m)?)?;
    m.add_function(wrap_pyfunction!(root::add_writers, m)?)?;
    m.add_function(wrap_pyfunction!(root::remove_writers, m)?)?;
    m.add_function(wrap_pyfunction!(root::enable, m)?)?;
    m.add_function(wrap_pyfunction!(root::disable, m)?)?;
    m.add_function(wrap_pyfunction!(root::enable_type, m)?)?;
    m.add_function(wrap_pyfunction!(root::disable_type, m)?)?;
    m.add_function(wrap_pyfunction!(root::sync, m)?)?;
    m.add_function(wrap_pyfunction!(root::sync_all, m)?)?;
    m.add_function(wrap_pyfunction!(root::rotate, m)?)?;
    m.add_function(wrap_pyfunction!(root::set_encryption, m)?)?;
    m.add_function(wrap_pyfunction!(root::set_debug, m)?)?;
    m.add_function(wrap_pyfunction!(root::get_writer_config, m)?)?;
    m.add_function(wrap_pyfunction!(root::get_server_config, m)?)?;
    m.add_function(wrap_pyfunction!(root::get_server_configs, m)?)?;
    m.add_function(wrap_pyfunction!(root::get_server_addresses_ports, m)?)?;
    m.add_function(wrap_pyfunction!(root::get_server_addresses, m)?)?;
    m.add_function(wrap_pyfunction!(root::get_server_ports, m)?)?;
    m.add_function(wrap_pyfunction!(root::get_server_auth_key, m)?)?;
    m.add_function(wrap_pyfunction!(root::get_config_string, m)?)?;
    m.add_function(wrap_pyfunction!(root::save_config, m)?)?;
    m.add_function(wrap_pyfunction!(root::get_parent_pid, m)?)?;
    m.add_function(wrap_pyfunction!(root::get_parent_client_writer_config, m)?)?;
    m.add_function(wrap_pyfunction!(
        root::get_parent_pid_client_writer_config,
        m
    )?)?;
    m.add_function(wrap_pyfunction!(root::trace, m)?)?;
    m.add_function(wrap_pyfunction!(root::debug, m)?)?;
    m.add_function(wrap_pyfunction!(root::info, m)?)?;
    m.add_function(wrap_pyfunction!(root::success, m)?)?;
    m.add_function(wrap_pyfunction!(root::warning, m)?)?;
    m.add_function(wrap_pyfunction!(root::error_func, m)?)?;
    m.add_function(wrap_pyfunction!(root::critical, m)?)?;
    m.add_function(wrap_pyfunction!(root::fatal, m)?)?;
    m.add_function(wrap_pyfunction!(root::exception, m)?)?;
    let fun: Py<PyAny> = PyModule::import(py, "atexit")?.getattr("register")?.into();
    let _ = fun.call1(py, (wrap_pyfunction!(shutdown_at_exit, m)?,))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use self::logging::Logging;

    use super::*;
    use crate::def;
    use crate::logger;
    use pyo3::IntoPyObjectExt;
    use std::fs;
    use std::path::PathBuf;

    /// Run a test function with the GIL held, ensuring the Python
    /// interpreter is initialized once for all tests.
    fn with_python<F, R>(f: F) -> R
    where
        F: for<'py> FnOnce(pyo3::Python<'py>) -> R,
    {
        // `Python::attach` attaches the current thread to the Python
        // interpreter. With the `auto-initialize` feature (dev-dependency)
        // the interpreter is automatically initialized on the first call.
        pyo3::Python::attach(f)
    }

    fn tmp_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "pyfastlogging_test_{}_{}",
            std::process::id(),
            name
        ));
        path
    }

    fn cleanup(name: &str) {
        let path = tmp_path(name);
        let path_str = path.to_string_lossy().to_string();
        let _ = fs::remove_file(&path);
        let _ = fs::remove_file(format!("{path_str}.1"));
        let _ = fs::remove_file(format!("{path_str}.2"));
        let _ = fs::remove_file(format!("{path_str}.gz"));
    }

    /// In unit tests there is no importable `pyfastlogging` module on
    /// `sys.path`, so we construct the module in-process via `init()`.
    fn import_module(py: Python<'_>) -> Bound<'_, PyModule> {
        let module = PyModule::new(py, "pyfastlogging").unwrap();
        init(py, &module).unwrap();
        module
    }

    /// Wait (with a short retry loop) until the given file contains `needle`.
    fn wait_for_file_content(path: &PathBuf, needle: &str) -> bool {
        for _ in 0..50 {
            if let Ok(content) = fs::read_to_string(path) {
                if content.contains(needle) {
                    return true;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        false
    }

    #[test]
    fn it_works() {
        with_python(|py| {
            let mut logging = Logging::new(None, None, None, None, None, None, py).unwrap();
            logging.shutdown(Some(true), py).unwrap();
        });
    }

    #[test]
    fn test_level_constants() {
        with_python(|py| {
            let module = import_module(py);
            assert_eq!(
                module.getattr("NOTSET").unwrap().extract::<u8>().unwrap(),
                fastlogging::NOTSET
            );
            assert_eq!(
                module.getattr("TRACE").unwrap().extract::<u8>().unwrap(),
                fastlogging::TRACE
            );
            assert_eq!(
                module.getattr("DEBUG").unwrap().extract::<u8>().unwrap(),
                fastlogging::DEBUG
            );
            assert_eq!(
                module.getattr("INFO").unwrap().extract::<u8>().unwrap(),
                fastlogging::INFO
            );
            assert_eq!(
                module.getattr("SUCCESS").unwrap().extract::<u8>().unwrap(),
                fastlogging::SUCCESS
            );
            assert_eq!(
                module.getattr("WARNING").unwrap().extract::<u8>().unwrap(),
                fastlogging::WARNING
            );
            assert_eq!(
                module.getattr("ERROR").unwrap().extract::<u8>().unwrap(),
                fastlogging::ERROR
            );
            assert_eq!(
                module.getattr("CRITICAL").unwrap().extract::<u8>().unwrap(),
                fastlogging::CRITICAL
            );
            assert_eq!(
                module.getattr("FATAL").unwrap().extract::<u8>().unwrap(),
                fastlogging::FATAL
            );
            assert_eq!(
                module
                    .getattr("EXCEPTION")
                    .unwrap()
                    .extract::<u8>()
                    .unwrap(),
                fastlogging::EXCEPTION
            );
            let version: String = module.getattr("__version__").unwrap().extract().unwrap();
            assert_eq!(version, env!("CARGO_PKG_VERSION"));
        });
    }

    #[test]
    fn test_logging_new_with_console_writer() {
        with_python(|py| {
            let config = ConsoleWriterConfig::new(fastlogging::DEBUG, false);
            let mut logging = Logging::new(
                Some(fastlogging::DEBUG),
                Some("test".to_string()),
                Some(vec![config.into_py_any(py).unwrap()]),
                None,
                None,
                None,
                py,
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
            logging.shutdown(Some(true), py).unwrap();
        });
    }

    #[test]
    fn test_logging_new_with_file_writer() {
        with_python(|py| {
            let path = tmp_path("file_writer.log");
            let config = FileWriterConfig::new(
                fastlogging::DEBUG,
                path.clone(),
                None,
                None,
                None,
                None,
                None,
            )
            .unwrap();
            let mut logging = Logging::new(
                Some(fastlogging::DEBUG),
                Some("test".to_string()),
                Some(vec![config.into_py_any(py).unwrap()]),
                None,
                None,
                None,
                py,
            )
            .unwrap();
            logging.info("file info msg").unwrap();
            logging.sync_all(Some(2.0)).unwrap();
            logging.shutdown(Some(false), py).unwrap();
            assert!(
                wait_for_file_content(&path, "file info msg"),
                "log file should contain the message"
            );
            cleanup("file_writer.log");
        });
    }

    #[test]
    fn test_logging_new_invalid_config_rejected() {
        with_python(|py| {
            let invalid = 42i32.into_py_any(py).unwrap();
            let err = Logging::new(
                Some(fastlogging::DEBUG),
                None,
                Some(vec![invalid]),
                None,
                None,
                None,
                py,
            );
            assert!(err.is_err(), "invalid writer config must be rejected");
        });
    }

    #[test]
    fn test_logger_class() {
        with_python(|py| {
            let mut logging = Logging::new(
                Some(fastlogging::DEBUG),
                Some("test".to_string()),
                Some(vec![
                    ConsoleWriterConfig::new(fastlogging::DEBUG, false)
                        .into_py_any(py)
                        .unwrap(),
                ]),
                None,
                None,
                None,
                py,
            )
            .unwrap();
            let logger = logger::Logger::new(
                fastlogging::INFO,
                "logger-domain".to_string(),
                None,
                None,
                None,
                py,
            )
            .unwrap();
            let logger_py = Py::new(py, logger).unwrap();
            logging.add_logger(logger_py.clone_ref(py), py);
            {
                let l = logger_py.bind(py).borrow();
                assert_eq!(l.level(), fastlogging::INFO);
                l.trace("t").unwrap();
                l.debug("d").unwrap();
                l.info("i").unwrap();
                l.success("s").unwrap();
                l.warning("w").unwrap();
                l.error("e").unwrap();
                l.critical("c").unwrap();
                l.fatal("f").unwrap();
            }
            {
                let mut l = logger_py.bind(py).borrow_mut();
                l.set_level(fastlogging::DEBUG);
                assert_eq!(l.level(), fastlogging::DEBUG);
                l.set_domain("new-domain".to_string());
            }
            logging.shutdown(Some(true), py).unwrap();
        });
    }

    #[test]
    fn test_add_remove_writer() {
        with_python(|py| {
            let mut logging = Logging::new(
                Some(fastlogging::DEBUG),
                Some("test".to_string()),
                None,
                None,
                None,
                None,
                py,
            )
            .unwrap();
            let config = ConsoleWriterConfig::new(fastlogging::DEBUG, false);
            let wid = logging
                .add_writer(config.into_py_any(py).unwrap(), py)
                .expect("add_writer succeeds");
            assert!(wid > 0, "writer id must be > 0");
            let cfg = logging
                .get_writer_config(wid)
                .expect("writer config exists");
            assert!(matches!(cfg, WriterConfigEnum::Console { .. }));
            let removed = logging.remove_writer(wid);
            assert!(removed.is_some());
            assert!(logging.get_writer_config(wid).is_none());
            logging.shutdown(Some(true), py).unwrap();
        });
    }

    #[test]
    fn test_disable_enable_writer() {
        with_python(|py| {
            let mut logging = Logging::new(
                Some(fastlogging::DEBUG),
                Some("test".to_string()),
                None,
                None,
                None,
                None,
                py,
            )
            .unwrap();
            let config = ConsoleWriterConfig::new(fastlogging::DEBUG, false);
            let wid = logging
                .add_writer(config.into_py_any(py).unwrap(), py)
                .expect("add_writer succeeds");
            assert!(wid > 0, "writer id must be > 0");
            println!("#1");
            logging.disable(wid).unwrap();
            println!("#1");
            logging.enable(wid).unwrap();
            println!("#1");
            logging
                .disable_type(def::WriterTypeEnum::Console {})
                .unwrap();
            println!("#1");
            logging
                .enable_type(def::WriterTypeEnum::Console {})
                .unwrap();
            println!("#1");
            logging.shutdown(Some(true), py).unwrap();
        });
    }
}
