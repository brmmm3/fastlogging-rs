use std::path::PathBuf;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use pyo3::prelude::*;

mod def;
pub use def::{
    ClientWriterConfig, ConsoleWriterConfig, EncryptionMethod, FileWriterConfig, ServerConfig,
    WriterConfigEnum, WriterTypeEnum,
};
mod logger;
mod logging;

static LOGGING: Lazy<Mutex<logging::Logging>> = Lazy::new(|| {
    Python::with_gil(|_py| -> Mutex<logging::Logging> {
        Mutex::new(
            logging::Logging::new(None, None, None, None, None, None, None, None, None, None)
                .unwrap(),
        )
    })
});

#[pyfunction]
#[pyo3(signature=(now=None,))]
fn shutdown(now: Option<bool>) -> PyResult<()> {
    LOGGING.lock().unwrap().shutdown(now)
}

#[pyfunction]
fn set_level(writer: WriterTypeEnum, level: u8) -> PyResult<()> {
    LOGGING.lock().unwrap().set_level(writer, level)
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
fn set_ext_config(ext_config: &Bound<'_, def::ExtConfig>) {
    LOGGING.lock().unwrap().set_ext_config(ext_config)
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
fn add_writer(writer: PyObject, py: Python) -> PyResult<WriterTypeEnum> {
    LOGGING.lock().unwrap().add_writer(writer, py)
}

#[pyfunction]
fn remove_writer(writer: WriterTypeEnum) -> PyResult<()> {
    LOGGING.lock().unwrap().remove_writer(writer)
}

#[pyfunction]
#[pyo3(signature=(console=None, file=None, client=None, syslog=None, timeout=None))]
fn sync(
    console: Option<bool>,
    file: Option<bool>,
    client: Option<bool>,
    syslog: Option<bool>,
    timeout: Option<f64>,
) -> PyResult<()> {
    LOGGING
        .lock()
        .unwrap()
        .sync(console, file, client, syslog, timeout)
}

#[pyfunction]
#[pyo3(signature=(timeout=None))]
fn sync_all(timeout: Option<f64>) -> PyResult<()> {
    LOGGING.lock().unwrap().sync_all(timeout)
}

#[pyfunction]
#[pyo3(signature=(path=None))]
fn rotate(path: Option<PathBuf>) -> PyResult<()> {
    LOGGING.lock().unwrap().rotate(path)
}

// Network

#[pyfunction]
fn set_encryption(writer: WriterTypeEnum, key: EncryptionMethod) -> PyResult<()> {
    LOGGING.lock().unwrap().set_encryption(writer, key)
}

// Config

#[pyfunction]
fn get_config(writer: WriterTypeEnum) -> PyResult<WriterConfigEnum> {
    LOGGING.lock().unwrap().get_config(writer)
}

#[pyfunction]
fn get_server_config(address: String) -> Option<ServerConfig> {
    LOGGING.lock().unwrap().get_server_config(address)
}

#[pyfunction]
fn get_server_auth_key() -> EncryptionMethod {
    LOGGING.lock().unwrap().get_server_auth_key()
}

#[pyfunction]
fn get_config_string() -> String {
    LOGGING.lock().unwrap().get_config_string()
}

#[pyfunction]
fn save_config(path: PathBuf) -> PyResult<()> {
    LOGGING.lock().unwrap().save_config(path)
}

// Logging methods

#[pyfunction]
fn trace(obj: PyObject) -> PyResult<()> {
    LOGGING.lock().unwrap().trace(obj)
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
fn success(obj: PyObject) -> PyResult<()> {
    LOGGING.lock().unwrap().success(obj)
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
    println!("#fastlogging_rs_init#BEGIN");
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
    m.add_class::<def::ExtConfig>()?;
    m.add_class::<def::ConsoleWriterConfig>()?;
    m.add_class::<def::FileWriterConfig>()?;
    m.add_class::<def::ServerConfig>()?;
    m.add_class::<def::ClientWriterConfig>()?;
    m.add_class::<logging::Logging>()?;
    m.add_class::<logger::Logger>()?;
    m.add_function(wrap_pyfunction!(shutdown, m)?)?;
    m.add_function(wrap_pyfunction!(set_level, m)?)?;
    m.add_function(wrap_pyfunction!(set_domain, m)?)?;
    m.add_function(wrap_pyfunction!(set_level2sym, m)?)?;
    m.add_function(wrap_pyfunction!(set_ext_config, m)?)?;
    m.add_function(wrap_pyfunction!(add_logger, m)?)?;
    m.add_function(wrap_pyfunction!(remove_logger, m)?)?;
    m.add_function(wrap_pyfunction!(add_writer, m)?)?;
    m.add_function(wrap_pyfunction!(remove_writer, m)?)?;
    m.add_function(wrap_pyfunction!(sync, m)?)?;
    m.add_function(wrap_pyfunction!(sync_all, m)?)?;
    m.add_function(wrap_pyfunction!(rotate, m)?)?;
    m.add_function(wrap_pyfunction!(set_encryption, m)?)?;
    m.add_function(wrap_pyfunction!(get_config, m)?)?;
    m.add_function(wrap_pyfunction!(get_server_config, m)?)?;
    m.add_function(wrap_pyfunction!(get_server_auth_key, m)?)?;
    m.add_function(wrap_pyfunction!(get_config_string, m)?)?;
    m.add_function(wrap_pyfunction!(save_config, m)?)?;
    m.add_function(wrap_pyfunction!(trace, m)?)?;
    m.add_function(wrap_pyfunction!(debug, m)?)?;
    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(success, m)?)?;
    m.add_function(wrap_pyfunction!(warning, m)?)?;
    m.add_function(wrap_pyfunction!(error, m)?)?;
    m.add_function(wrap_pyfunction!(critical, m)?)?;
    m.add_function(wrap_pyfunction!(fatal, m)?)?;
    m.add_function(wrap_pyfunction!(exception, m)?)?;
    let fun: Py<PyAny> = PyModule::import_bound(py, "atexit")?
        .getattr("register")?
        .into();
    let _ = fun.call1(py, (wrap_pyfunction!(shutdown_at_exit, m)?,))?;
    println!("#name# {:?}", m.name());
    println!("#filename# {:?}", m.filename());
    println!("#dict# {:#?}", m.dict());
    println!("#index# {:?}", m.index());
    println!("#fastlogging_rs_init#END");
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
