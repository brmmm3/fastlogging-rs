use std::cmp;
use std::io::Error;
use std::path::PathBuf;

use pyo3::exceptions::{PyException, PyTypeError};
use pyo3::prelude::*;

use fastlogging::{
    LoggingConfig, CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, SUCCESS, TRACE, WARNING,
};
use pyo3::types::PyBytes;

use crate::def::{
    ClientWriterConfig, ConsoleWriterConfig, EncryptionMethod, ExtConfig, FileWriterConfig,
    LevelSyms, RootConfig, ServerConfig, SyslogWriterConfig, WriterConfigEnum, WriterTypeEnum,
};
use crate::logger::Logger;

#[pyclass]
#[derive(Debug)]
pub struct Logging {
    instance: fastlogging::Logging,
    indent: Option<(usize, usize, usize, String)>,
    getframe: Py<PyAny>,
    format_exc: Py<PyAny>,
}

impl Logging {
    fn do_indent(&self, obj: PyObject) -> PyResult<String> {
        Python::with_gil(|py| {
            let mut message: String = obj.extract(py)?;
            if let Some((offset, inc, max, s)) = &self.indent {
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
                    message.insert_str(0, &s[..depth]);
                }
            }
            Ok(message)
        })
    }
}

#[pymethods]
impl Logging {
    #[new]
    #[pyo3(signature=(level, domain=None, indent=None, ext_config=None, console=None, file=None, server=None, connect=None, syslog=None, config=None))]
    pub fn new(
        level: Option<u8>, // Global log level
        domain: Option<String>,
        indent: Option<(usize, usize, usize)>, // If defined indent text by call depth
        ext_config: Option<&Bound<'_, ExtConfig>>, // Extended configuration
        console: Option<&Bound<'_, ConsoleWriterConfig>>, // If config is defined start ConsoleWriter
        file: Option<&Bound<'_, FileWriterConfig>>,       // If config is defined start FileWriter
        server: Option<&Bound<'_, ServerConfig>>, // If config is defined start LoggingServer
        connect: Option<&Bound<'_, ClientWriterConfig>>, // If config is defined start ClientWriter
        syslog: Option<u8>,                       // If log level is defined start SyslogLogging
        config: Option<PathBuf>,                  // Optional configuration file
    ) -> Result<Self, Error> {
        let (getframe, format_exc) =
            Python::with_gil(|py| -> Result<(Py<PyAny>, Py<PyAny>), Error> {
                let sys = py.import_bound("sys")?;
                let getframe = sys.getattr("_getframe")?;
                let traceback = py.import_bound("traceback")?;
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
        Ok(Self {
            instance: fastlogging::Logging::new(
                level,
                domain,
                ext_config.map(|v| v.borrow().0.clone()),
                console.map(|v| v.borrow().0.clone()),
                file.map(|v| v.borrow().0.clone()),
                server.map(|v| v.borrow().0.clone()),
                connect.map(|v| v.borrow().0.clone()),
                syslog,
                config,
            )?,
            indent,
            getframe,
            format_exc,
        })
    }

    #[pyo3(signature=(now=None,))]
    pub fn shutdown(&mut self, now: Option<bool>) -> PyResult<()> {
        self.instance
            .shutdown(now.unwrap_or_default())
            .map_err(PyException::new_err)
    }

    pub fn set_level(&mut self, writer: WriterTypeEnum, level: u8) -> PyResult<()> {
        let writer: fastlogging::WriterTypeEnum = writer.into();
        self.instance
            .set_level(&writer, level)
            .map_err(PyException::new_err)
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

    pub fn add_writer(&mut self, writer: PyObject, py: Python) -> PyResult<WriterTypeEnum> {
        let writer = if let Ok(writer) = writer.extract::<RootConfig>(py) {
            fastlogging::WriterConfigEnum::Root(writer.0)
        } else if let Ok(writer) = writer.extract::<ConsoleWriterConfig>(py) {
            fastlogging::WriterConfigEnum::Console(writer.0)
        } else if let Ok(writer) = writer.extract::<FileWriterConfig>(py) {
            fastlogging::WriterConfigEnum::File(writer.0)
        } else if let Ok(writer) = writer.extract::<ClientWriterConfig>(py) {
            fastlogging::WriterConfigEnum::Client(writer.0)
        } else if let Ok(writer) = writer.extract::<ServerConfig>(py) {
            fastlogging::WriterConfigEnum::Server(writer.0)
        } else if let Ok(writer) = writer.extract::<SyslogWriterConfig>(py) {
            fastlogging::WriterConfigEnum::Syslog(writer.0)
        } else {
            return Err(PyTypeError::new_err("writer has invalid argument type"));
        };
        Ok(self
            .instance
            .add_writer(&writer)
            .map_err(PyException::new_err)?
            .into())
    }

    pub fn remove_writer(&mut self, writer: WriterTypeEnum) -> PyResult<()> {
        self.instance
            .remove_writer(&(writer.into()))
            .map_err(PyException::new_err)
    }

    pub fn add_logger(&mut self, logger: Py<Logger>, py: Python) {
        self.instance
            .add_logger(&mut logger.borrow_mut(py).instance)
    }

    pub fn remove_logger(&mut self, logger: Py<Logger>, py: Python) {
        self.instance
            .remove_logger(&mut logger.borrow_mut(py).instance)
    }

    #[pyo3(signature=(console=None, file=None, client=None, syslog=None, timeout=None))]
    pub fn sync(
        &self,
        console: Option<bool>,
        file: Option<bool>,
        client: Option<bool>,
        syslog: Option<bool>,
        timeout: Option<f64>,
    ) -> PyResult<()> {
        self.instance
            .sync(
                console.unwrap_or_default(),
                file.unwrap_or_default(),
                client.unwrap_or_default(),
                syslog.unwrap_or_default(),
                timeout.unwrap_or(1.0),
            )
            .map_err(PyException::new_err)
    }

    #[pyo3(signature=(timeout=None))]
    pub fn sync_all(&self, timeout: Option<f64>) -> PyResult<()> {
        self.instance
            .sync(true, true, true, true, timeout.unwrap_or(1.0))
            .map_err(PyException::new_err)
    }

    // File logger

    #[pyo3(signature=(path=None))]
    pub fn rotate(&self, path: Option<PathBuf>) -> PyResult<()> {
        self.instance.rotate(path).map_err(PyException::new_err)
    }

    // Network

    pub fn set_encryption(
        &mut self,
        writer: WriterTypeEnum,
        key: EncryptionMethod,
    ) -> PyResult<()> {
        self.instance
            .set_encryption(writer.into(), key.into())
            .map_err(PyException::new_err)
    }

    // Config

    pub fn set_debug(&mut self, debug: u8) {
        self.instance.set_debug(debug);
    }

    pub fn get_config(&self, writer: WriterTypeEnum) -> PyResult<WriterConfigEnum> {
        self.instance
            .get_config(&(writer.into()))
            .map(|c| c.into())
            .map_err(PyException::new_err)
    }

    pub fn get_server_config(&self, address: String) -> Option<ServerConfig> {
        self.instance.get_server_config(&address).map(ServerConfig)
    }

    pub fn get_server_configs(&self) -> Vec<ServerConfig> {
        self.instance
            .get_server_configs()
            .into_iter()
            .map(ServerConfig)
            .collect()
    }

    pub fn get_server_addresses(&self) -> Vec<String> {
        self.instance.get_server_addresses()
    }

    pub fn get_server_ports(&self) -> Vec<u16> {
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

    pub fn save_config(&self, path: PathBuf) -> PyResult<()> {
        self.instance
            .save_config(&path)
            .map_err(PyException::new_err)
    }

    // Logging methods

    pub fn trace(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= TRACE {
            self.instance
                .trace(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn debug(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= DEBUG {
            self.instance
                .debug(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn info(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= INFO {
            self.instance
                .info(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn success(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= SUCCESS {
            self.instance
                .success(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn warning(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= WARNING {
            self.instance
                .warning(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn error(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= ERROR {
            self.instance
                .error(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn critical(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= CRITICAL {
            self.instance
                .critical(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn fatal(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= FATAL {
            self.instance
                .fatal(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn exception(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= EXCEPTION {
            Python::with_gil(|py| {
                let message: String = obj.extract(py)?;
                let tb: String = self.format_exc.call0(py)?.extract(py)?;
                self.instance
                    .exception(format!("{message}\n{tb}"))
                    .map_err(PyException::new_err)
            })
        } else {
            Ok(())
        }
    }

    pub fn __setstate__(&mut self, state: Bound<'_, PyBytes>) -> PyResult<()> {
        println!("__setstate__");
        let data: &[u8] = state.as_bytes();
        let config = LoggingConfig::from_json_vec(data);
        println!("config={:?}", config);
        Ok(())
    }

    pub fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        println!("__getstate__");
        let config = self
            .instance
            .instance
            .lock()
            .unwrap()
            .get_config()
            .to_json_vec()
            .map_err(PyException::new_err)?;
        Ok(PyBytes::new_bound(py, &config))
    }

    pub fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyBytes>,)> {
        println!("__getnewargs__");
        let config = self
            .instance
            .instance
            .lock()
            .unwrap()
            .get_config()
            .to_json_vec()
            .map_err(PyException::new_err)?;
        Ok((PyBytes::new_bound(py, &config),))
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
