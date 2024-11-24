use std::{collections::HashMap, path::PathBuf};

use pyo3::{exceptions::PyException, prelude::*};

use crate::{
    logger::Logger,
    writer::{CallbackWriterConfig, ExtConfig, RootConfig, SyslogWriterConfig},
    ClientWriterConfig, ConsoleWriterConfig, EncryptionMethod, FileWriterConfig, LevelSyms,
    LoggingError, ServerConfig, WriterConfigEnum, WriterTypeEnum,
};

/// Python layer for fastlogging.

/// Shutdown fastlogging module.
#[pyfunction]
pub fn root_init() {
    fastlogging::root::root_init();
}

/// Shutdown fastlogging module.
#[pyfunction]
#[pyo3(signature=(now=None,))]
pub fn shutdown(now: Option<bool>) -> Result<(), LoggingError> {
    Ok(fastlogging::root::shutdown(now.unwrap_or_default())?)
}

/// Set log level for writer with ID `wid` to `level`.
#[pyfunction]
pub fn set_level(wid: usize, level: u8) -> Result<(), LoggingError> {
    Ok(fastlogging::root::set_level(wid, level)?)
}

/// Set logging domain.
#[pyfunction]
pub fn set_domain(domain: String) {
    fastlogging::root::set_domain(domain)
}

/// Configure log level symbols. For valid values see [LevelSyms].
#[pyfunction]
pub fn set_level2sym(level2sym: &Bound<'_, LevelSyms>) {
    fastlogging::root::set_level2sym(&level2sym.borrow().0)
}

/// Set extended configuration. For details see [ExtConfig].
#[pyfunction]
pub fn set_ext_config(ext_config: &Bound<'_, ExtConfig>) {
    fastlogging::root::set_ext_config(&ext_config.borrow().0)
}

#[pyfunction]
pub fn add_logger(logger: Py<Logger>, py: Python) {
    fastlogging::root::add_logger(&mut logger.borrow_mut(py).instance)
}

#[pyfunction]
pub fn remove_logger(logger: Py<Logger>, py: Python) {
    fastlogging::root::remove_logger(&mut logger.borrow_mut(py).instance)
}

#[pyfunction]
pub fn add_writer(config: PyObject, py: Python) -> PyResult<usize> {
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
    fastlogging::root::add_writer_config(&config).map_err(|e| PyException::new_err(e.to_string()))
}

#[pyfunction]
pub fn remove_writer(wid: usize) -> Option<WriterConfigEnum> {
    fastlogging::root::remove_writer(wid).map(|w| w.config().into())
}

#[pyfunction]
pub fn enable(wid: usize) -> Result<(), LoggingError> {
    Ok(fastlogging::root::enable(wid)?)
}

#[pyfunction]
pub fn disable(wid: usize) -> Result<(), LoggingError> {
    Ok(fastlogging::root::disable(wid)?)
}

#[pyfunction]
pub fn enable_type(typ: WriterTypeEnum) -> Result<(), LoggingError> {
    Ok(fastlogging::root::enable_type(typ.into())?)
}

#[pyfunction]
pub fn disable_type(typ: WriterTypeEnum) -> Result<(), LoggingError> {
    Ok(fastlogging::root::disable_type(typ.into())?)
}

#[pyfunction]
#[pyo3(signature=(types, timeout=None))]
pub fn sync(types: Vec<WriterTypeEnum>, timeout: Option<f64>) -> Result<(), LoggingError> {
    Ok(fastlogging::root::sync(
        types.into_iter().map(|t| t.into()).collect::<Vec<_>>(),
        timeout.unwrap_or(1.0),
    )?)
}

#[pyfunction]
#[pyo3(signature=(timeout=None))]
pub fn sync_all(timeout: Option<f64>) -> Result<(), LoggingError> {
    Ok(fastlogging::root::sync_all(timeout.unwrap_or_default())?)
}

#[pyfunction]
#[pyo3(signature=(path=None))]
pub fn rotate(path: Option<PathBuf>) -> Result<(), LoggingError> {
    Ok(fastlogging::root::rotate(path)?)
}

// Network

#[pyfunction]
pub fn set_encryption(wid: usize, key: EncryptionMethod) -> Result<(), LoggingError> {
    Ok(fastlogging::root::set_encryption(wid, key.into())?)
}

// Config

/// Set debug mode.
#[pyfunction]
pub fn set_debug(debug: u8) {
    fastlogging::root::set_debug(debug);
}

/// Get configuration for writer with ID `wid`.
#[pyfunction]
pub fn get_writer_config(wid: usize) -> Option<WriterConfigEnum> {
    fastlogging::root::get_writer_config(wid).map(|w| w.into())
}

#[pyfunction]
pub fn get_server_config(wid: usize) -> Result<ServerConfig, LoggingError> {
    Ok(fastlogging::root::get_server_config(wid)?.into())
}

#[pyfunction]
pub fn get_server_configs() -> HashMap<usize, ServerConfig> {
    fastlogging::root::get_server_configs()
        .into_iter()
        .map(|(k, v)| (k, v.into()))
        .collect::<HashMap<_, _>>()
}

#[pyfunction]
pub fn get_server_addresses_ports() -> HashMap<usize, String> {
    fastlogging::root::get_server_addresses_ports()
}

#[pyfunction]
pub fn get_server_addresses() -> HashMap<usize, String> {
    fastlogging::root::get_server_addresses()
}

#[pyfunction]
pub fn get_server_ports() -> HashMap<usize, u16> {
    fastlogging::root::get_server_ports()
}

#[pyfunction]
pub fn get_server_auth_key() -> EncryptionMethod {
    fastlogging::root::get_server_auth_key().into()
}

#[pyfunction]
pub fn get_config_string() -> String {
    fastlogging::root::get_config_string()
}

#[pyfunction]
#[pyo3(signature=(path=None,))]
pub fn save_config(path: Option<PathBuf>) -> Result<(), LoggingError> {
    Ok(fastlogging::root::save_config(path.as_deref())?)
}

/// Get process ID of parent process.
#[pyfunction]
pub fn get_parent_pid() -> Option<u32> {
    fastlogging::root::get_parent_pid()
}

#[pyfunction]
pub fn get_parent_server_address() -> Option<ClientWriterConfig> {
    fastlogging::root::get_parent_server_address().map(|v| v.into())
}

#[pyfunction]
pub fn get_parent_pid_server_address() -> Option<(u32, ClientWriterConfig)> {
    fastlogging::root::get_parent_pid_server_address().map(|(ppid, config)| (ppid, config.into()))
}

// Logging methods

#[pyfunction]
pub fn trace(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::root::trace(obj.to_string())?)
}

#[pyfunction]
pub fn debug(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::root::debug(obj.to_string())?)
}

#[pyfunction]
pub fn info(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::root::info(obj.to_string())?)
}

#[pyfunction]
pub fn success(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::root::success(obj.to_string())?)
}

#[pyfunction]
pub fn warning(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::root::warning(obj.to_string())?)
}

#[pyfunction]
#[pyo3(name = "error")]
pub fn error_func(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::root::error(obj.to_string())?)
}

#[pyfunction]
pub fn critical(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::root::critical(obj.to_string())?)
}

#[pyfunction]
pub fn fatal(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::root::fatal(obj.to_string())?)
}

#[pyfunction]
pub fn exception(obj: PyObject) -> Result<(), LoggingError> {
    Ok(fastlogging::root::exception(obj.to_string())?)
}
