use std::sync::Mutex;

use fastlogging::NOTSET;
use once_cell::sync::Lazy;
use pyo3::prelude::*;

mod def;
pub use def::{
    ClientWriterConfig, ConsoleWriterConfig, EncryptionMethod, FileWriterConfig, ServerConfig,
};
mod logger;
mod logging;

static LOGGING: Lazy<Mutex<logging::Logging>> = Lazy::new(|| {
    Python::with_gil(|py| -> Mutex<logging::Logging> {
        let console = Bound::new(py, ConsoleWriterConfig::new(NOTSET, false)).unwrap();
        Mutex::new(
            logging::Logging::new(
                None,
                None,
                None,
                None,
                Some(&console),
                None,
                None,
                None,
                None,
                None,
            )
            .unwrap(),
        )
    })
});

#[pyfunction]
fn shutdown(now: Option<bool>) -> PyResult<()> {
    LOGGING.lock().unwrap().shutdown(now)
}

#[pyfunction]
fn add_logger(obj: Py<logger::Logger>, py: Python) {
    LOGGING.lock().unwrap().add_logger(obj, py)
}

#[pyfunction]
fn remove_logger(obj: Py<logger::Logger>, py: Python) {
    LOGGING.lock().unwrap().remove_logger(obj, py)
}

#[pyfunction]
fn set_level(level: u8) {
    LOGGING.lock().unwrap().set_level(level)
}

#[pyfunction]
fn set_domain(domain: String) {
    LOGGING.lock().unwrap().set_domain(domain)
}

#[pyfunction]
fn set_level2sym(level2sym: &Bound<'_, def::LevelSyms>) {
    LOGGING.lock().unwrap().set_level2sym(level2sym)
}

#[pyfunction]
fn set_console_writer(config: Option<&Bound<'_, ConsoleWriterConfig>>) -> PyResult<()> {
    LOGGING.lock().unwrap().set_console_writer(config)
}

#[pyfunction]
fn set_console_colors(colors: bool) {
    LOGGING.lock().unwrap().set_console_colors(colors);
}

#[pyfunction]
fn set_file_writer(config: Option<&Bound<'_, FileWriterConfig>>) -> PyResult<()> {
    LOGGING.lock().unwrap().set_file_writer(config)
}

#[pyfunction]
fn rotate() -> PyResult<()> {
    LOGGING.lock().unwrap().rotate()
}

#[pyfunction]
fn sync(
    console: Option<bool>,
    file: Option<bool>,
    client: Option<bool>,
    timeout: Option<f64>,
) -> PyResult<()> {
    LOGGING.lock().unwrap().sync(console, file, client, timeout)
}

#[pyfunction]
fn sync_all(timeout: Option<f64>) -> PyResult<()> {
    LOGGING.lock().unwrap().sync_all(timeout)
}

// Network client

#[pyfunction]
fn connect(config: &Bound<'_, ClientWriterConfig>) -> PyResult<()> {
    LOGGING.lock().unwrap().connect(config)
}

#[pyfunction]
fn disconnect(address: &str) -> PyResult<()> {
    LOGGING.lock().unwrap().disconnect(address)
}

#[pyfunction]
fn set_client_level(address: &str, level: u8) -> PyResult<()> {
    LOGGING.lock().unwrap().set_client_level(address, level)
}

#[pyfunction]
fn set_client_encryption(address: &str, key: EncryptionMethod) -> PyResult<()> {
    LOGGING
        .lock()
        .unwrap()
        .set_client_encryption(address, key.into())
}

// Network server

#[pyfunction]
fn server_start(address: String, level: u8, key: EncryptionMethod) -> PyResult<()> {
    LOGGING
        .lock()
        .unwrap()
        .server_start(address, level, key.into())
}

#[pyfunction]
fn server_shutdown() -> PyResult<()> {
    LOGGING.lock().unwrap().server_shutdown()
}

#[pyfunction]
fn set_server_level(level: u8) -> PyResult<()> {
    LOGGING.lock().unwrap().set_server_level(level)
}

#[pyfunction]
fn set_server_encryption(key: EncryptionMethod) -> PyResult<()> {
    LOGGING.lock().unwrap().set_server_encryption(key.into())
}

#[pyfunction]
fn debug(obj: PyObject) -> PyResult<()> {
    LOGGING.lock().unwrap().debug(obj)
}

#[pyfunction]
fn info(obj: PyObject) -> PyResult<()> {
    LOGGING.lock().unwrap().info(obj)
}

#[pyfunction]
fn warning(obj: PyObject) -> PyResult<()> {
    LOGGING.lock().unwrap().warning(obj)
}

#[pyfunction]
fn error(obj: PyObject) -> PyResult<()> {
    LOGGING.lock().unwrap().error(obj)
}

#[pyfunction]
fn critical(obj: PyObject) -> PyResult<()> {
    LOGGING.lock().unwrap().critical(obj)
}

#[pyfunction]
fn fatal(obj: PyObject) -> PyResult<()> {
    LOGGING.lock().unwrap().fatal(obj)
}

#[pyfunction]
fn exception(obj: PyObject) -> PyResult<()> {
    LOGGING.lock().unwrap().exception(obj)
}

#[pyfunction]
fn shutdown_at_exit() -> PyResult<()> {
    LOGGING.lock().unwrap().shutdown(None)
}

/// fastlogging_rs is a simple example for using Rust to create Python extension modules.
#[pymodule]
#[pyo3(name = "fastlogging_rs")]
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
    m.add_class::<def::CompressionMethodEnum>()?;
    m.add_class::<def::EncryptionMethod>()?;
    m.add_class::<def::ExtConfig>()?;
    m.add_class::<def::ConsoleWriterConfig>()?;
    m.add_class::<def::FileWriterConfig>()?;
    m.add_class::<def::ServerConfig>()?;
    m.add_class::<def::ClientWriterConfig>()?;
    m.add_class::<logging::Logging>()?;
    m.add_class::<logger::Logger>()?;
    m.add_function(wrap_pyfunction!(shutdown, m)?)?;
    m.add_function(wrap_pyfunction!(add_logger, m)?)?;
    m.add_function(wrap_pyfunction!(remove_logger, m)?)?;
    m.add_function(wrap_pyfunction!(set_level, m)?)?;
    m.add_function(wrap_pyfunction!(set_domain, m)?)?;
    m.add_function(wrap_pyfunction!(set_level2sym, m)?)?;
    m.add_function(wrap_pyfunction!(set_console_writer, m)?)?;
    m.add_function(wrap_pyfunction!(set_console_colors, m)?)?;
    m.add_function(wrap_pyfunction!(set_file_writer, m)?)?;
    m.add_function(wrap_pyfunction!(rotate, m)?)?;
    m.add_function(wrap_pyfunction!(sync, m)?)?;
    m.add_function(wrap_pyfunction!(sync_all, m)?)?;
    m.add_function(wrap_pyfunction!(connect, m)?)?;
    m.add_function(wrap_pyfunction!(disconnect, m)?)?;
    m.add_function(wrap_pyfunction!(set_client_level, m)?)?;
    m.add_function(wrap_pyfunction!(set_client_encryption, m)?)?;
    m.add_function(wrap_pyfunction!(server_start, m)?)?;
    m.add_function(wrap_pyfunction!(server_shutdown, m)?)?;
    m.add_function(wrap_pyfunction!(set_server_level, m)?)?;
    m.add_function(wrap_pyfunction!(set_server_encryption, m)?)?;
    m.add_function(wrap_pyfunction!(debug, m)?)?;
    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(warning, m)?)?;
    m.add_function(wrap_pyfunction!(error, m)?)?;
    m.add_function(wrap_pyfunction!(critical, m)?)?;
    m.add_function(wrap_pyfunction!(fatal, m)?)?;
    m.add_function(wrap_pyfunction!(exception, m)?)?;
    let fun: Py<PyAny> = PyModule::import_bound(py, "atexit")?
        .getattr("register")?
        .into();
    let _ = fun.call1(py, (wrap_pyfunction!(shutdown_at_exit, m)?,))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use self::logging::Logging;

    use super::*;

    #[test]
    fn it_works() {
        let mut logging =
            Logging::new(None, None, None, None, None, None, None, None, None, None).unwrap();
        //logging.info("Hello".to_string()).unwrap();
        logging.shutdown(Some(true)).unwrap();
    }
}
