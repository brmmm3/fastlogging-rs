use std::cmp;
use std::collections::HashMap;
use std::path::PathBuf;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use fastlogging::{
    CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, LoggingConfig, NOTSET, SUCCESS, TRACE, WARNING,
};
use pyo3::types::PyBytes;

use crate::def::{EncryptionMethod, LevelSyms, WriterConfigEnum, WriterTypeEnum};
use crate::logger::Logger;
use crate::writer::{
    CallbackWriterConfig, ExtConfig, RootConfig, ServerConfig, SyslogWriterConfig,
};
use crate::{ClientWriterConfig, ConsoleWriterConfig, FileWriterConfig, LoggingError};

#[pyclass]
#[derive(Debug)]
pub struct Logging {
    instance: fastlogging::Logging,
    indent: Option<(usize, usize, usize, String)>,
    getframe: Py<PyAny>,
    format_exc: Py<PyAny>,
}

impl Logging {
    fn do_indent(&self, obj: Py<PyAny>) -> PyResult<String> {
        Python::attach(|py| {
            let mut message: String = obj.extract(py)?;
            if let Some((offset, inc, max, s)) = &self.indent
                && let Ok(mut frame) = self.getframe.call1(py, (*offset,))
            {
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
                message.insert_str(0, &s[..depth]);
            }
            Ok(message)
        })
    }
}

#[pymethods]
impl Logging {
    #[new]
    #[pyo3(signature=(level, domain=None, configs=None, ext_config=None, config_path=None, indent=None))]
    pub fn new(
        level: Option<u8>,                         // Global log level
        domain: Option<String>,                    // Optional log domain
        configs: Option<Vec<Py<PyAny>>>,           // List of writer configurations
        ext_config: Option<&Bound<'_, ExtConfig>>, // Extended formatting configuration
        config_path: Option<PathBuf>,              // Optional configuration file
        indent: Option<(usize, usize, usize)>,     // If defined indent text by call depth
        py: Python,
    ) -> Result<Self, LoggingError> {
        let (getframe, format_exc) = Python::attach(|py| -> PyResult<(Py<PyAny>, Py<PyAny>)> {
            let sys = py.import("sys")?;
            let getframe = sys.getattr("_getframe")?;
            let traceback = py.import("traceback")?;
            let format_exc = traceback.getattr("format_exc")?;
            Ok((getframe.into(), format_exc.into()))
        })?;
        let indent = match indent {
            Some((offset, mut inc, mut max)) => {
                inc = cmp::min(inc, 8);
                max = cmp::min(max, 256);
                let mut s = String::with_capacity(max);
                let _ = (0..(max - offset) * inc)
                    .map(|_| s.push(' '))
                    .collect::<Vec<_>>();
                Some((offset, inc, max, s))
            }
            None => None,
        };
        let writer_configs = if let Some(configs) = configs {
            let mut writer_configs: Vec<fastlogging::WriterConfigEnum> = Vec::new();
            for config in configs {
                if let Ok(v) = config.extract::<WriterConfigEnum>(py) {
                    writer_configs.push(v.into());
                } else if let Ok(v) = config.extract::<RootConfig>(py) {
                    writer_configs.push(WriterConfigEnum::Root { config: v }.into());
                } else if let Ok(v) = config.extract::<ConsoleWriterConfig>(py) {
                    writer_configs.push(WriterConfigEnum::Console { config: v }.into());
                } else if let Ok(v) = config.extract::<FileWriterConfig>(py) {
                    writer_configs.push(WriterConfigEnum::File { config: v }.into());
                } else if let Ok(v) = config.extract::<ClientWriterConfig>(py) {
                    writer_configs.push(WriterConfigEnum::Client { config: v }.into());
                } else if let Ok(v) = config.extract::<ServerConfig>(py) {
                    writer_configs.push(WriterConfigEnum::Server { config: v }.into());
                } else if let Ok(v) = config.extract::<SyslogWriterConfig>(py) {
                    writer_configs.push(WriterConfigEnum::Syslog { config: v }.into());
                } else if let Ok(v) = config.extract::<CallbackWriterConfig>(py) {
                    writer_configs.push(WriterConfigEnum::Callback { config: v }.into());
                } else {
                    return Err(LoggingError(fastlogging::LoggingError::InvalidValue(
                        format!("Writer configuration {config:?} has invalid type"),
                    )));
                }
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

    pub fn add_writer(&mut self, config: WriterConfigEnum) -> Result<usize, LoggingError> {
        Ok(self.instance.add_writer_config(&config.into())?)
    }

    pub fn remove_writer(&mut self, wid: usize) -> Option<WriterConfigEnum> {
        self.instance.remove_writer(wid).map(|c| c.config().into())
    }

    pub fn add_writers(
        &mut self,
        configs: Vec<WriterConfigEnum>,
    ) -> Result<Vec<usize>, LoggingError> {
        Ok(self
            .instance
            .add_writer_configs(configs.into_iter().map(|c| c.into()).collect::<Vec<_>>())?)
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

    #[pyo3(signature=(types=None, timeout=None))]
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

    #[pyo3(signature=(timeout=None))]
    pub fn sync_all(&self, timeout: Option<f64>) -> Result<(), LoggingError> {
        Ok(self.instance.sync_all(timeout.unwrap_or(1.0))?)
    }

    // File logger

    #[pyo3(signature=(path=None))]
    pub fn rotate(&self, path: Option<PathBuf>) -> Result<(), LoggingError> {
        Ok(self.instance.rotate(path)?)
    }

    // Network

    pub fn set_encryption(
        &mut self,
        wid: usize,
        key: EncryptionMethod,
    ) -> Result<(), LoggingError> {
        Ok(self.instance.set_encryption(wid, key.into())?)
    }

    // Config

    pub fn get_writer_config(&self, wid: usize) -> Option<WriterConfigEnum> {
        self.instance.get_writer_config(wid).map(|c| c.into())
    }

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

    #[pyo3(signature=(path=None,))]
    pub fn save_config(&mut self, path: Option<PathBuf>) -> Result<(), LoggingError> {
        Ok(self.instance.save_config(path.as_deref())?)
    }

    // Logging methods

    pub fn trace(&self, obj: Py<PyAny>) -> PyResult<()> {
        if self.instance.level <= TRACE {
            self.instance
                .trace(self.do_indent(obj)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    pub fn debug(&self, obj: Py<PyAny>) -> PyResult<()> {
        if self.instance.level <= DEBUG {
            self.instance
                .debug(self.do_indent(obj)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    pub fn info(&self, obj: Py<PyAny>) -> PyResult<()> {
        if self.instance.level <= INFO {
            self.instance
                .info(self.do_indent(obj)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    pub fn success(&self, obj: Py<PyAny>) -> PyResult<()> {
        if self.instance.level <= SUCCESS {
            self.instance
                .success(self.do_indent(obj)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    pub fn warning(&self, obj: Py<PyAny>) -> PyResult<()> {
        if self.instance.level <= WARNING {
            self.instance
                .warning(self.do_indent(obj)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    pub fn error(&self, obj: Py<PyAny>) -> PyResult<()> {
        if self.instance.level <= ERROR {
            self.instance
                .error(self.do_indent(obj)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    pub fn critical(&self, obj: Py<PyAny>) -> PyResult<()> {
        if self.instance.level <= CRITICAL {
            self.instance
                .critical(self.do_indent(obj)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    pub fn fatal(&self, obj: Py<PyAny>) -> PyResult<()> {
        if self.instance.level <= FATAL {
            self.instance
                .fatal(self.do_indent(obj)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    pub fn exception(&self, obj: Py<PyAny>) -> PyResult<()> {
        if self.instance.level <= EXCEPTION {
            Python::attach(|py| {
                let message: String = obj.extract(py)?;
                let tb: String = self.format_exc.call0(py)?.extract(py)?;
                self.instance
                    .exception(format!("{message}\n{tb}"))
                    .map_err(|e| PyException::new_err(e.to_string()))
            })
        } else {
            Ok(())
        }
    }

    pub fn set_debug(&mut self, debug: u8) {
        self.instance.set_debug(debug);
    }

    pub fn __setstate__(&mut self, state: Bound<'_, PyBytes>) -> Result<(), LoggingError> {
        println!("__setstate__");
        let data: &[u8] = state.as_bytes();
        let config = LoggingConfig::from_json_vec(data);
        println!("config={config:?}");
        Ok(())
    }

    pub fn __getstate__<'py>(&self, py: Python<'py>) -> Result<Bound<'py, PyBytes>, LoggingError> {
        println!("__getstate__");
        let config = self
            .instance
            .instance
            .lock()
            .unwrap()
            .get_logging_config()
            .to_json_vec()?;
        Ok(PyBytes::new(py, &config))
    }

    pub fn __getnewargs__<'py>(
        &self,
        py: Python<'py>,
    ) -> Result<(Bound<'py, PyBytes>,), LoggingError> {
        println!("__getnewargs__");
        let config = self
            .instance
            .instance
            .lock()
            .unwrap()
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
