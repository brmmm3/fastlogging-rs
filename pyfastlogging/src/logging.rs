use std::cmp;
use std::io::Error;
use std::path::PathBuf;

use fastlogging::{CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, SUCCESS, TRACE, WARNING};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use crate::def::{
    ClientWriterConfig, ConsoleWriterConfig, EncryptionMethod, ExtConfig, FileWriterConfig,
    LevelSyms, ServerConfig,
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
            getframe: getframe.into(),
            format_exc: format_exc.into(),
        })
    }

    pub fn shutdown(&mut self, now: Option<bool>) -> PyResult<()> {
        self.instance
            .shutdown(now.unwrap_or_default())
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

    pub fn set_level(&mut self, level: u8) {
        self.instance.set_level(level)
    }

    pub fn set_domain(&mut self, domain: String) {
        self.instance.set_domain(domain)
    }

    pub fn set_level2sym(&mut self, level2sym: &Bound<'_, LevelSyms>) {
        self.instance.set_level2sym(level2sym.borrow().0.clone())
    }

    pub fn set_ext_config(&mut self, ext_config: &Bound<'_, ExtConfig>) {
        self.instance.set_ext_config(ext_config.borrow().0.clone())
    }

    // Console logger

    pub fn set_console_writer(
        &mut self,
        config: Option<&Bound<'_, ConsoleWriterConfig>>,
    ) -> PyResult<()> {
        self.instance
            .set_console_writer(config.map(|c| c.borrow().0.clone()))
            .map_err(PyException::new_err)
    }

    pub fn set_console_colors(&mut self, colors: bool) {
        self.instance.set_console_colors(colors);
    }

    pub fn sync(
        &self,
        console: Option<bool>,
        file: Option<bool>,
        client: Option<bool>,
        timeout: Option<f64>,
    ) -> PyResult<()> {
        self.instance
            .sync(
                console.unwrap_or_default(),
                file.unwrap_or_default(),
                client.unwrap_or_default(),
                timeout.unwrap_or(1.0),
            )
            .map_err(PyException::new_err)
    }

    pub fn sync_all(&self, timeout: Option<f64>) -> PyResult<()> {
        self.instance
            .sync(true, true, true, timeout.unwrap_or(1.0))
            .map_err(PyException::new_err)
    }

    // File logger

    pub fn set_file_writer(
        &mut self,
        config: Option<&Bound<'_, FileWriterConfig>>,
    ) -> PyResult<()> {
        self.instance
            .set_file_writer(config.map(|c| c.borrow().0.clone()))
            .map_err(PyException::new_err)
    }

    pub fn rotate(&self) -> PyResult<()> {
        self.instance.rotate().map_err(PyException::new_err)
    }

    // Network client

    pub fn connect(&mut self, config: &Bound<'_, ClientWriterConfig>) -> PyResult<()> {
        self.instance
            .connect(config.borrow().0.clone())
            .map_err(PyException::new_err)
    }

    pub fn disconnect(&mut self, address: &str) -> PyResult<()> {
        self.instance
            .disconnect(address)
            .map_err(PyException::new_err)
    }

    pub fn set_client_level(&mut self, address: &str, level: u8) -> PyResult<()> {
        self.instance
            .set_client_level(address, level)
            .map_err(PyException::new_err)
    }

    pub fn set_client_encryption(&mut self, address: &str, key: EncryptionMethod) -> PyResult<()> {
        self.instance
            .set_client_encryption(address, key.into())
            .map_err(PyException::new_err)
    }

    // Network server

    pub fn server_start(
        &mut self,
        address: String,
        level: u8,
        key: EncryptionMethod,
    ) -> PyResult<()> {
        self.instance
            .server_start(address, level, key.into())
            .map_err(PyException::new_err)
    }

    pub fn server_shutdown(&mut self) -> PyResult<()> {
        self.instance
            .server_shutdown()
            .map_err(PyException::new_err)
    }

    pub fn set_server_level(&mut self, level: u8) -> PyResult<()> {
        self.instance
            .set_server_level(level)
            .map_err(PyException::new_err)
    }

    pub fn set_server_encryption(&mut self, key: EncryptionMethod) -> PyResult<()> {
        self.instance
            .set_server_encryption(key.into())
            .map_err(PyException::new_err)
    }

    pub fn get_server_config(&self) -> Option<ServerConfig> {
        self.instance.get_server_config().map(|x| ServerConfig(x))
    }

    pub fn get_server_auth_key(&self) -> Vec<u8> {
        self.instance.get_server_auth_key()
    }

    // Config

    pub fn get_config(&self) -> String {
        self.instance.get_config()
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
