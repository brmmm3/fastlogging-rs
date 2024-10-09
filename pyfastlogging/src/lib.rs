use std::path::PathBuf;

use pyo3::{exceptions::PyException, prelude::*};
use writer::{CallbackWriterConfig, ExtConfig, RootConfig, SyslogWriterConfig};

mod def;
pub use def::{EncryptionMethod, LevelSyms, WriterConfigEnum, WriterTypeEnum};
mod writer;
pub use writer::{ClientWriterConfig, ConsoleWriterConfig, FileWriterConfig, ServerConfig};
mod error;
pub use error::LoggingError;
mod logger;
mod logging;

#[pyfunction]
#[pyo3(signature=(now=None,))]
fn shutdown(now: Option<bool>) -> Result<(), LoggingError> {
    Ok(fastlogging::shutdown(now.unwrap_or_default())?)
}

#[pyfunction]
fn set_level(wid: usize, level: u8) -> Result<(), LoggingError> {
    Ok(fastlogging::set_level(wid, level)?)
}

#[pyfunction]
fn set_domain(domain: String) {
    fastlogging::set_domain(domain)
}

#[pyfunction]
fn set_level2sym(level2sym: &Bound<'_, LevelSyms>) {
    fastlogging::set_level2sym(&level2sym.borrow().0)
}

#[pyfunction]
fn set_ext_config(ext_config: &Bound<'_, ExtConfig>) {
    fastlogging::set_ext_config(&ext_config.borrow().0)
}

#[pyfunction]
fn add_logger(logger: Py<logger::Logger>, py: Python) {
    fastlogging::add_logger(&mut logger.borrow_mut(py).instance)
}

#[pyfunction]
fn remove_logger(logger: Py<logger::Logger>, py: Python) {
    fastlogging::remove_logger(&mut logger.borrow_mut(py).instance)
}

#[pyfunction]
fn add_writer(config: PyObject, py: Python) -> PyResult<usize> {
    let config = if let Ok(config) = config.extract::<RootConfig>(py) {
        fastlogging::WriterConfigEnum::Root(config.0)
    } else if let Ok(config) = config.extract::<ConsoleWriterConfig>(py) {
        fastlogging::WriterConfigEnum::Console(config.0)
    } else if let Ok(config) = config.extract::<FileWriterConfig>(py) {
        fastlogging::WriterConfigEnum::File(config.0)
    } else if let Ok(config) = config.extract::<ClientWriterConfig>(py) {
        fastlogging::WriterConfigEnum::Client(config.0)
    } else if let Ok(config) = config.extract::<ServerConfig>(py) {
        fastlogging::WriterConfigEnum::Server(config.0)
    } else if let Ok(config) = config.extract::<SyslogWriterConfig>(py) {
        fastlogging::WriterConfigEnum::Syslog(config.0)
    } else if let Ok(config) = config.extract::<CallbackWriterConfig>(py) {
        fastlogging::WriterConfigEnum::Callback(config.0)
    } else {
        return Err(PyException::new_err(
            "writer has invalid argument type".to_string(),
        ));
    };
    Ok(fastlogging::add_writer_config(&config)
        .map_err(|e| PyException::new_err(e.to_string()))?
        .into())
}

#[pyfunction]
fn remove_writer(wid: usize) -> Option<WriterConfigEnum> {
    fastlogging::remove_writer(wid).map(|w| w.config().into())
}

#[pyfunction]
#[pyo3(signature=(types, timeout=None))]
fn sync(types: Vec<WriterTypeEnum>, timeout: Option<f64>) -> Result<(), LoggingError> {
    Ok(fastlogging::sync(
        types.into_iter().map(|t| t.into()).collect::<Vec<_>>(),
        timeout.unwrap_or(1.0),
    )?)
}

#[pyfunction]
#[pyo3(signature=(timeout=None))]
fn sync_all(timeout: Option<f64>) -> Result<(), LoggingError> {
    Ok(fastlogging::sync_all(timeout.unwrap_or_default())?)
}

#[pyfunction]
#[pyo3(signature=(path=None))]
fn rotate(path: Option<PathBuf>) -> Result<(), LoggingError> {
    Ok(fastlogging::rotate(path)?)
}

// Network

#[pyfunction]
fn set_encryption(wid: usize, key: EncryptionMethod) -> Result<(), LoggingError> {
    Ok(fastlogging::set_encryption(wid, key.into())?)
}

// Config

#[pyfunction]
pub fn set_debug(debug: u8) {
    fastlogging::set_debug(debug);
}

#[pyfunction]
fn get_config(wid: usize) -> Option<WriterConfigEnum> {
    fastlogging::get_config(wid).map(|w| w.into())
}

#[pyfunction]
fn get_server_config(wid: usize) -> Result<ServerConfig, LoggingError> {
    Ok(fastlogging::get_server_config(wid)?.into())
}

#[pyfunction]
fn get_server_auth_key() -> EncryptionMethod {
    fastlogging::get_server_auth_key().into()
}

#[pyfunction]
fn get_config_string() -> String {
    fastlogging::get_config_string()
}

#[pyfunction]
#[pyo3(signature=(path=None,))]
fn save_config(path: Option<PathBuf>) -> Result<(), LoggingError> {
    Ok(fastlogging::save_config(path.as_deref())?)
}

#[pyfunction]
pub fn get_parent_pid() -> Option<u32> {
    fastlogging::get_parent_pid()
}

#[pyfunction]
pub fn get_parent_server_address() -> Option<ClientWriterConfig> {
    fastlogging::get_parent_server_address().map(|v| v.into())
}

#[pyfunction]
pub fn get_parent_pid_server_address() -> Option<(u32, ClientWriterConfig)> {
    fastlogging::get_parent_pid_server_address().map(|(ppid, config)| (ppid, config.into()))
}

// Logging methods

#[pyfunction]
fn trace(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::trace(obj.to_string())?)
}

#[pyfunction]
fn debug(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::debug(obj.to_string())?)
}

#[pyfunction]
fn info(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::info(obj.to_string())?)
}

#[pyfunction]
fn success(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::success(obj.to_string())?)
}

#[pyfunction]
fn warning(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::warning(obj.to_string())?)
}

#[pyfunction]
#[pyo3(name = "error")]
fn error_func(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::error(obj.to_string())?)
}

#[pyfunction]
fn critical(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::critical(obj.to_string())?)
}

#[pyfunction]
fn fatal(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::fatal(obj.to_string())?)
}

#[pyfunction]
fn exception(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::exception(obj.to_string())?)
}

#[pyfunction]
fn shutdown_at_exit() -> Result<(), LoggingError> {
    Ok(fastlogging::shutdown(false)?)
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
    m.add_class::<def::MessageStructEnum>()?;
    m.add_class::<def::CompressionMethodEnum>()?;
    m.add_class::<def::EncryptionMethod>()?;
    m.add_class::<def::WriterTypeEnum>()?;
    m.add_class::<ExtConfig>()?;
    m.add_class::<ConsoleWriterConfig>()?;
    m.add_class::<FileWriterConfig>()?;
    m.add_class::<ServerConfig>()?;
    m.add_class::<ClientWriterConfig>()?;
    m.add_class::<CallbackWriterConfig>()?;
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
    m.add_function(wrap_pyfunction!(set_debug, m)?)?;
    m.add_function(wrap_pyfunction!(get_config, m)?)?;
    m.add_function(wrap_pyfunction!(get_server_config, m)?)?;
    m.add_function(wrap_pyfunction!(get_server_auth_key, m)?)?;
    m.add_function(wrap_pyfunction!(get_config_string, m)?)?;
    m.add_function(wrap_pyfunction!(save_config, m)?)?;
    m.add_function(wrap_pyfunction!(get_parent_pid, m)?)?;
    m.add_function(wrap_pyfunction!(get_parent_server_address, m)?)?;
    m.add_function(wrap_pyfunction!(get_parent_pid_server_address, m)?)?;
    m.add_function(wrap_pyfunction!(trace, m)?)?;
    m.add_function(wrap_pyfunction!(debug, m)?)?;
    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(success, m)?)?;
    m.add_function(wrap_pyfunction!(warning, m)?)?;
    m.add_function(wrap_pyfunction!(error_func, m)?)?;
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
        let mut logging = Logging::new(None, None, vec![], None, None, None).unwrap();
        Python::with_gil(|py| {
            logging.shutdown(Some(true), py).unwrap();
        });
    }
}
