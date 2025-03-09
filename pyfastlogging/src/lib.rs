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

/// Python layer for fastlogging.

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
    m.add_function(wrap_pyfunction!(root::set_domain, m)?)?;
    m.add_function(wrap_pyfunction!(root::set_level2sym, m)?)?;
    m.add_function(wrap_pyfunction!(root::set_ext_config, m)?)?;
    m.add_function(wrap_pyfunction!(root::add_logger, m)?)?;
    m.add_function(wrap_pyfunction!(root::remove_logger, m)?)?;
    m.add_function(wrap_pyfunction!(root::add_writer, m)?)?;
    m.add_function(wrap_pyfunction!(root::remove_writer, m)?)?;
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

    #[test]
    fn it_works() {
        let mut logging = Logging::new(None, None, vec![], None, None, None).unwrap();
        Python::with_gil(|py| {
            logging.shutdown(Some(true), py).unwrap();
        });
    }
}
