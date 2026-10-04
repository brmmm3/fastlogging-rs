use std::borrow::Cow;
use std::cmp;
use std::collections::HashMap;
use std::path::PathBuf;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use fastlogging::{
    CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, LoggingConfig, NOTSET, SUCCESS, TRACE, WARNING,
};

use crate::LoggingError;
use crate::def::{EncryptionMethod, LevelSyms, WriterConfigEnum, WriterTypeEnum};
use crate::logger::Logger;
use crate::writer::{ExtConfig, ServerConfig};

#[pyclass]
#[derive(Debug)]
pub struct Logging {
    instance: fastlogging::Logging,
    indent: Option<(usize, usize, usize, String)>,
    getframe: Py<PyAny>,
    format_exc: Py<PyAny>,
}

impl Logging {
    fn do_indent<'a>(&self, msg: &'a str) -> PyResult<Cow<'a, str>> {
        // Fast path: without indentation the message is passed straight
        // through, avoiding an extra allocation and any GIL interaction.
        let Some((offset, inc, max, s)) = &self.indent else {
            return Ok(Cow::Borrowed(msg));
        };
        Python::attach(|py| -> PyResult<Cow<'a, str>> {
            let mut message = String::with_capacity(msg.len() + s.len());
            message.push_str(msg);
            if let Ok(mut frame) = self.getframe.call1(py, (*offset,)) {
                let mut depth = 0;
                loop {
                    frame = match frame.getattr(py, "f_back") {
                        Ok(f) => f.extract(py)?,
                        Err(_) => {
                            break;
                        }
                    };
                    depth += inc;
                    if depth >= *max {
                        break;
                    }
                }
                message.insert_str(0, &s[..depth.min(s.len())]);
            }
            Ok(Cow::Owned(message))
        })
    }
}

#[pymethods]
impl Logging {
    #[new]
    #[pyo3(signature=(level=None, domain=None, configs=None, ext_config=None, config_path=None, indent=None))]
    pub fn new(
        level: Option<u8>,                         // Global log level
        domain: Option<String>,                    // Optional log domain
        configs: Option<Vec<Py<PyAny>>>,           // List of writer configurations
        ext_config: Option<&Bound<'_, ExtConfig>>, // Extended formatting configuration
        config_path: Option<PathBuf>,              // Optional configuration file
        indent: Option<(usize, usize, usize)>,     // If defined indent text by call depth
        py: Python,
    ) -> Result<Self, LoggingError> {
        let (getframe, format_exc) = (
            crate::root::get_getframe(py)?.clone_ref(py),
            crate::root::get_format_exc(py)?.clone_ref(py),
        );
        let indent = indent.map(|(offset, inc, max)| {
            let inc = cmp::min(inc, 8);
            let max = cmp::min(max, 256);
            // Pre-build the padding string once with exactly the right length.
            let s = " ".repeat(max.saturating_sub(offset) * inc);
            (offset, inc, max, s)
        });
        let writer_configs = if let Some(configs) = configs {
            let mut writer_configs: Vec<fastlogging::WriterConfigEnum> =
                Vec::with_capacity(configs.len());
            for config in configs {
                writer_configs.push(crate::root::extract_writer_config_enum(config, py)?);
            }
            Some(writer_configs)
        } else {
            None
        };
        Ok(Self {
            instance: fastlogging::Logging::new(
                level.unwrap_or(NOTSET),
                domain.unwrap_or_else(|| "root".to_string()),
                writer_configs,
                ext_config.map(|v| v.borrow().0.clone()),
                config_path,
            )
            .map_err(|e| PyException::new_err(e.to_string()))?,
            indent,
            getframe,
            format_exc,
        })
    }

    #[pyo3(signature=(now=None,))]
    pub fn shutdown(&mut self, now: Option<bool>, py: Python) -> Result<(), LoggingError> {
        py.detach(|| -> Result<(), LoggingError> {
            Ok(self.instance.shutdown(now.unwrap_or_default())?)
        })
    }

    pub fn set_level(&mut self, wid: usize, level: u8) -> Result<(), LoggingError> {
        Ok(self.instance.set_level(wid, level)?)
    }

    pub fn set_domain(&mut self, domain: String) {
        self.instance.set_domain(&domain)
    }

    pub fn set_level2sym(&mut self, level2sym: &Bound<'_, LevelSyms>) {
        self.instance.set_level2sym(&level2sym.borrow().0)
    }

    pub fn set_ext_config(&mut self, ext_config: &Bound<'_, ExtConfig>) {
        self.instance.set_ext_config(&ext_config.borrow().0)
    }

    pub fn add_logger(&mut self, logger: Py<Logger>, py: Python) {
        self.instance
            .add_logger(&mut logger.borrow_mut(py).instance)
    }

    pub fn remove_logger(&mut self, logger: Py<Logger>, py: Python) {
        self.instance
            .remove_logger(&mut logger.borrow_mut(py).instance)
    }

    pub fn set_root_writer(&mut self, config: WriterConfigEnum) -> Result<(), LoggingError> {
        Ok(self.instance.set_root_writer_config(&config.into())?)
    }

    pub fn add_writer(&mut self, config: Py<PyAny>, py: Python) -> Result<usize, LoggingError> {
        let config = crate::root::extract_writer_config_enum(config, py)?;
        Ok(self.instance.add_writer_config(&config)?)
    }

    pub fn remove_writer(&mut self, wid: usize) -> Option<WriterConfigEnum> {
        self.instance.remove_writer(wid).map(|c| c.config().into())
    }

    pub fn add_writers(
        &mut self,
        configs: Vec<Py<PyAny>>,
        py: Python,
    ) -> Result<Vec<usize>, LoggingError> {
        configs
            .into_iter()
            .map(|config| self.add_writer(config, py))
            .collect::<Result<Vec<_>, LoggingError>>()
    }

    #[pyo3(signature=(wids=None,))]
    pub fn remove_writers(&mut self, wids: Option<Vec<usize>>) -> Vec<WriterConfigEnum> {
        self.instance
            .remove_writers(wids)
            .into_iter()
            .map(|c| c.config().into())
            .collect::<Vec<_>>()
    }

    pub fn enable(&self, wid: usize) -> Result<(), LoggingError> {
        Ok(self.instance.enable(wid)?)
    }

    pub fn disable(&self, wid: usize) -> Result<(), LoggingError> {
        Ok(self.instance.disable(wid)?)
    }

    pub fn enable_type(&self, typ: WriterTypeEnum) -> Result<(), LoggingError> {
        Ok(self.instance.enable_type(typ.into())?)
    }

    pub fn disable_type(&self, typ: WriterTypeEnum) -> Result<(), LoggingError> {
        Ok(self.instance.disable_type(typ.into())?)
    }

    #[pyo3(signature=(types=None, timeout=None, /))]
    pub fn sync(
        &self,
        types: Option<Vec<WriterTypeEnum>>,
        timeout: Option<f64>,
    ) -> Result<(), LoggingError> {
        if let Some(types) = types {
            Ok(self.instance.sync(
                types.into_iter().map(|t| t.into()).collect::<Vec<_>>(),
                timeout.unwrap_or(1.0),
            )?)
        } else {
            self.sync_all(timeout)
        }
    }

    #[pyo3(signature=(timeout=None, /))]
    pub fn sync_all(&self, timeout: Option<f64>) -> Result<(), LoggingError> {
        Ok(self.instance.sync_all(timeout.unwrap_or(1.0))?)
    }

    // File logger

    #[pyo3(signature=(path=None, /))]
    pub fn rotate(&self, path: Option<PathBuf>) -> Result<(), LoggingError> {
        Ok(self.instance.rotate(path)?)
    }

    // Network

    #[pyo3(signature=(wid, key, /))]
    pub fn set_encryption(
        &mut self,
        wid: usize,
        key: EncryptionMethod,
    ) -> Result<(), LoggingError> {
        Ok(self.instance.set_encryption(wid, key.into())?)
    }

    // Config

    #[pyo3(signature=(wid, /))]
    pub fn get_writer_config(&self, wid: usize) -> Option<WriterConfigEnum> {
        self.instance.get_writer_config(wid).map(|c| c.into())
    }

    #[pyo3(signature=(wid, /))]
    pub fn get_server_config(&self, wid: usize) -> Result<ServerConfig, LoggingError> {
        Ok(self.instance.get_server_config(wid)?.into())
    }

    pub fn get_server_configs(&self) -> HashMap<usize, ServerConfig> {
        self.instance
            .get_server_configs()
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect()
    }

    pub fn get_root_server_address_port(&self) -> Option<String> {
        self.instance.get_root_server_address_port()
    }

    pub fn get_server_addresses_ports(&self) -> HashMap<usize, String> {
        self.instance.get_server_addresses_ports()
    }

    pub fn get_server_addresses(&self) -> HashMap<usize, String> {
        self.instance.get_server_addresses()
    }

    pub fn get_server_ports(&self) -> HashMap<usize, u16> {
        self.instance.get_server_ports()
    }

    pub fn get_server_auth_key(&self) -> EncryptionMethod {
        EncryptionMethod::AuthKey {
            key: self.instance.get_server_auth_key().key().unwrap().to_vec(),
        }
    }

    pub fn get_config_string(&self) -> String {
        self.instance.get_config_string()
    }

    #[pyo3(signature=(path=None, /))]
    pub fn save_config(&mut self, path: Option<PathBuf>) -> Result<(), LoggingError> {
        Ok(self.instance.save_config(path.as_deref())?)
    }

    // Logging methods

    #[pyo3(signature=(msg, /))]
    pub fn trace(&self, msg: &str) -> PyResult<()> {
        if self.instance.level <= TRACE {
            self.instance
                .trace(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature=(msg, /))]
    pub fn debug(&self, msg: &str) -> PyResult<()> {
        if self.instance.level <= DEBUG {
            self.instance
                .debug(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature=(msg, /))]
    pub fn info(&self, msg: &str) -> PyResult<()> {
        if self.instance.level <= INFO {
            self.instance
                .info(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature=(msg, /))]
    pub fn success(&self, msg: &str) -> PyResult<()> {
        if self.instance.level <= SUCCESS {
            self.instance
                .success(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature=(msg, /))]
    pub fn warning(&self, msg: &str) -> PyResult<()> {
        if self.instance.level <= WARNING {
            self.instance
                .warning(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature=(msg, /))]
    pub fn error(&self, msg: &str) -> PyResult<()> {
        if self.instance.level <= ERROR {
            self.instance
                .error(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature=(msg, /))]
    pub fn critical(&self, msg: &str) -> PyResult<()> {
        if self.instance.level <= CRITICAL {
            self.instance
                .critical(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature=(msg, /))]
    pub fn fatal(&self, msg: &str) -> PyResult<()> {
        if self.instance.level <= FATAL {
            self.instance
                .fatal(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature=(msg, /))]
    pub fn exception(&self, msg: &str, py: Python) -> PyResult<()> {
        if self.instance.level <= EXCEPTION {
            let tb: String = self.format_exc.call0(py)?.extract(py)?;
            self.instance
                .exception(format!("{msg}\n{tb}"))
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    pub fn set_debug(&mut self, debug: u8) {
        self.instance.set_debug(debug);
    }

    pub fn __setstate__(&mut self, state: Bound<'_, PyBytes>) -> Result<(), LoggingError> {
        let data: &[u8] = state.as_bytes();
        let _config = LoggingConfig::from_json_vec(data);
        Ok(())
    }

    pub fn __getstate__<'py>(&self, py: Python<'py>) -> Result<Bound<'py, PyBytes>, LoggingError> {
        let config = self
            .instance
            .instance
            .read()
            .get_logging_config()
            .to_json_vec()?;
        Ok(PyBytes::new(py, &config))
    }

    pub fn __getnewargs__<'py>(
        &self,
        py: Python<'py>,
    ) -> Result<(Bound<'py, PyBytes>,), LoggingError> {
        let config = self
            .instance
            .instance
            .read()
            .get_logging_config()
            .to_json_vec()?;
        Ok((PyBytes::new(py, &config),))
    }

    fn __repr__(&self) -> String {
        self.instance.__repr__()
    }

    fn __str__(&self) -> String {
        self.instance.__str__()
    }
}

impl Drop for Logging {
    fn drop(&mut self) {
        self.instance.shutdown(false).unwrap();
    }
}
