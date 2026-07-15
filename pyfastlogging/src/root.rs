use std::{
    collections::HashMap,
    path::PathBuf,
    sync::atomic::{AtomicU8, Ordering},
};

use fastlogging::{
    CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, NOTSET, SUCCESS, TRACE, WARNING,
};
use once_cell::sync::OnceCell;
use pyo3::prelude::*;

use crate::{
    ClientWriterConfig, ConsoleWriterConfig, EncryptionMethod, FileWriterConfig, LevelSyms,
    LoggingError, ServerConfig, WriterConfigEnum, WriterTypeEnum,
    logger::Logger,
    writer::{CallbackWriterConfig, ExtConfig, RootConfig, SyslogWriterConfig},
};

static LEVEL: AtomicU8 = AtomicU8::new(NOTSET);

static FORMAT_EXC: OnceCell<Py<PyAny>> = OnceCell::new();

pub fn get_format_exc(py: Python) -> PyResult<&'static Py<PyAny>> {
    FORMAT_EXC
        .get_or_try_init(|| {
            let traceback = py.import("traceback")?;
            traceback.getattr("format_exc").map(|f| f.into())
        })
        .map(|f| f.as_ref())
}

/// Python layer for fastlogging.

fn extract_writer_config_enum(
    config: Py<PyAny>,
    py: Python,
) -> Result<fastlogging::WriterConfigEnum, LoggingError> {
    Ok(if let Ok(config) = config.extract::<RootConfig>(py) {
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
        return Err(fastlogging::LoggingError::InvalidValue(
            "writer has invalid argument type".to_string(),
        )
        .into());
    })
}

/// Initialize root logger.
#[pyfunction]
pub fn root_init() {
    fastlogging::root::root_init();
    LEVEL.store(NOTSET, Ordering::Relaxed);
}

/// Shutdown fastlogging module.
#[pyfunction]
#[pyo3(signature=(now=None, /))]
pub fn shutdown(now: Option<bool>) -> Result<(), LoggingError> {
    Ok(fastlogging::root::shutdown(now.unwrap_or_default())?)
}

/// Set log level for writer with ID `wid` to `level`.
#[pyfunction]
#[pyo3(signature = (wid, level, /))]
pub fn set_level(wid: usize, level: u8) -> Result<(), LoggingError> {
    fastlogging::root::set_level(wid, level)?;
    LEVEL.store(fastlogging::root::get_root_level(), Ordering::Relaxed);
    Ok(())
}

/// Set log level for writer with ID `wid` to `level`.
#[pyfunction]
#[pyo3(signature = (level, /))]
pub fn set_root_level(level: u8) -> Result<(), LoggingError> {
    fastlogging::root::set_root_level(level);
    LEVEL.store(level, Ordering::Relaxed);
    Ok(())
}

/// Set logging domain.
#[pyfunction]
#[pyo3(signature = (domain, /))]
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
#[pyo3(signature = (logger, /))]
pub fn remove_logger(logger: Py<Logger>, py: Python) {
    fastlogging::root::remove_logger(&mut logger.borrow_mut(py).instance)
}

#[pyfunction]
#[pyo3(signature = (config, /))]
pub fn set_root_writer(config: Py<PyAny>, py: Python) -> Result<(), LoggingError> {
    fastlogging::root::set_root_writer_config(&extract_writer_config_enum(config, py)?)
        .map_err(|e| e.into())
}

#[pyfunction]
#[pyo3(signature = (config, /))]
pub fn add_writer(config: Py<PyAny>, py: Python) -> Result<usize, LoggingError> {
    fastlogging::root::add_writer_config(&extract_writer_config_enum(config, py)?)
        .map_err(|e| e.into())
}

#[pyfunction]
#[pyo3(signature = (wid, /))]
pub fn remove_writer(wid: usize) -> Option<WriterConfigEnum> {
    fastlogging::root::remove_writer(wid).map(|w| w.config().into())
}

#[pyfunction]
#[pyo3(signature = (configs, /))]
pub fn add_writers(configs: Vec<Py<PyAny>>, py: Python) -> Result<Vec<usize>, LoggingError> {
    configs
        .into_iter()
        .map(|config| add_writer(config, py))
        .collect::<Result<Vec<_>, LoggingError>>()
}

#[pyfunction]
#[pyo3(signature = (wids=None, /))]
pub fn remove_writers(wids: Option<Vec<usize>>) -> Vec<WriterConfigEnum> {
    fastlogging::root::remove_writers(wids)
        .into_iter()
        .map(|w| w.config().into())
        .collect()
}

#[pyfunction]
#[pyo3(signature = (wid, /))]
pub fn enable(wid: usize) -> Result<(), LoggingError> {
    Ok(fastlogging::root::enable(wid)?)
}

#[pyfunction]
#[pyo3(signature = (wid, /))]
pub fn disable(wid: usize) -> Result<(), LoggingError> {
    Ok(fastlogging::root::disable(wid)?)
}

#[pyfunction]
#[pyo3(signature = (typ, /))]
pub fn enable_type(typ: WriterTypeEnum) -> Result<(), LoggingError> {
    Ok(fastlogging::root::enable_type(typ.into())?)
}

#[pyfunction]
pub fn disable_type(typ: WriterTypeEnum) -> Result<(), LoggingError> {
    Ok(fastlogging::root::disable_type(typ.into())?)
}

#[pyfunction]
#[pyo3(signature=(types=None, timeout=None, /))]
pub fn sync(types: Option<Vec<WriterTypeEnum>>, timeout: Option<f64>) -> Result<(), LoggingError> {
    if let Some(types) = types {
        Ok(fastlogging::root::sync(
            types.into_iter().map(|t| t.into()).collect::<Vec<_>>(),
            timeout.unwrap_or(1.0),
        )?)
    } else {
        Ok(fastlogging::root::sync_all(timeout.unwrap_or_default())?)
    }
}

#[pyfunction]
#[pyo3(signature=(timeout=None, /))]
pub fn sync_all(timeout: Option<f64>) -> Result<(), LoggingError> {
    Ok(fastlogging::root::sync_all(timeout.unwrap_or_default())?)
}

#[pyfunction]
#[pyo3(signature=(path=None, /))]
pub fn rotate(path: Option<PathBuf>) -> Result<(), LoggingError> {
    Ok(fastlogging::root::rotate(path)?)
}

// Network

#[pyfunction]
#[pyo3(signature = (wid, key, /))]
pub fn set_encryption(wid: usize, key: EncryptionMethod) -> Result<(), LoggingError> {
    Ok(fastlogging::root::set_encryption(wid, key.into())?)
}

// Config

/// Get configuration for writer with ID `wid`.
#[pyfunction]
#[pyo3(signature = (wid, /))]
pub fn get_writer_config(wid: usize) -> Option<WriterConfigEnum> {
    fastlogging::root::get_writer_config(wid).map(|w| w.into())
}

#[pyfunction]
#[pyo3(signature = (wid, /))]
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
#[pyo3(signature=(path=None, /))]
pub fn save_config(path: Option<PathBuf>) -> Result<(), LoggingError> {
    Ok(fastlogging::root::save_config(path.as_deref())?)
}

/// Get process id of parent process.
#[pyfunction]
pub fn get_parent_pid() -> Option<u32> {
    fastlogging::root::get_parent_pid()
}

#[pyfunction]
pub fn get_parent_client_writer_config() -> Option<ClientWriterConfig> {
    fastlogging::root::get_parent_client_writer_config().map(|v| v.into())
}

#[pyfunction]
pub fn get_parent_pid_client_writer_config() -> Option<(u32, ClientWriterConfig)> {
    fastlogging::root::get_parent_pid_client_writer_config()
        .map(|(ppid, config)| (ppid, config.into()))
}

// Logging methods

#[pyfunction]
#[pyo3(signature = (msg, /))] // Enforce positional-only FASTCALL path with '/'
pub fn trace(msg: &str) -> Result<(), LoggingError> {
    if LEVEL.load(Ordering::Relaxed) <= TRACE {
        Ok(fastlogging::root::trace(msg.to_string())?)
    } else {
        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (msg, /))]
pub fn debug(msg: &str) -> Result<(), LoggingError> {
    if LEVEL.load(Ordering::Relaxed) <= DEBUG {
        Ok(fastlogging::root::debug(msg.to_string())?)
    } else {
        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (msg, /))]
pub fn info(msg: &str) -> Result<(), LoggingError> {
    if LEVEL.load(Ordering::Relaxed) <= INFO {
        Ok(fastlogging::root::info(msg.to_string())?)
    } else {
        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (msg, /))]
pub fn success(msg: &str) -> Result<(), LoggingError> {
    if LEVEL.load(Ordering::Relaxed) <= SUCCESS {
        Ok(fastlogging::root::success(msg.to_string())?)
    } else {
        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (msg, /))]
pub fn warning(msg: &str) -> Result<(), LoggingError> {
    if LEVEL.load(Ordering::Relaxed) <= WARNING {
        Ok(fastlogging::root::warning(msg.to_string())?)
    } else {
        Ok(())
    }
}

#[pyfunction]
#[pyo3(name = "error")]
#[pyo3(signature = (msg, /))]
pub fn error_func(msg: &str) -> Result<(), LoggingError> {
    if LEVEL.load(Ordering::Relaxed) <= ERROR {
        Ok(fastlogging::root::error(msg.to_string())?)
    } else {
        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (msg, /))]
pub fn critical(msg: &str) -> Result<(), LoggingError> {
    if LEVEL.load(Ordering::Relaxed) <= CRITICAL {
        Ok(fastlogging::root::critical(msg.to_string())?)
    } else {
        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (msg, /))]
pub fn fatal(msg: &str) -> Result<(), LoggingError> {
    if LEVEL.load(Ordering::Relaxed) <= FATAL {
        Ok(fastlogging::root::fatal(msg.to_string())?)
    } else {
        Ok(())
    }
}

#[pyfunction]
#[pyo3(signature = (msg, /))]
pub fn exception(msg: &str, py: Python) -> Result<(), LoggingError> {
    if LEVEL.load(Ordering::Relaxed) <= EXCEPTION {
        let tb: String = get_format_exc(py)?.call0(py)?.extract(py)?;
        Ok(fastlogging::root::exception(format!("{msg}\n{tb}"))?)
    } else {
        Ok(())
    }
}

/// Set debug mode.
#[pyfunction]
#[pyo3(signature = (debug, /))]
pub fn set_debug(debug: u8) {
    fastlogging::root::set_debug(debug);
}
