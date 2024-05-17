use std::cmp;
use std::io::Error;
use std::path::PathBuf;
use std::time::Duration;

use fastlogging::{ CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, WARNING };
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use crate::logger::Logger;
use crate::def::LevelSyms;

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
        console: Option<bool>, // If true start ConsoleLogging
        file: Option<PathBuf>, // If path is defined start FileLogging
        server: Option<String>, // If address is defined start LoggingServer
        connect: Option<String>, // If address is defined start ClientLogging
        max_size: Option<usize>, // Maximum size of log files
        backlog: Option<usize> // Maximum number of backup log files
    ) -> Result<Self, Error> {
        let (getframe, format_exc) = Python::with_gil(
            |py| -> Result<(Py<PyAny>, Py<PyAny>), Error> {
                let sys = py.import_bound("sys")?;
                let getframe = sys.getattr("_getframe")?;
                let traceback = py.import_bound("traceback")?;
                let format_exc = traceback.getattr("format_exc")?;
                Ok((getframe.into(), format_exc.into()))
            }
        )?;
        let indent = match indent {
            Some((offset, mut inc, mut max)) => {
                inc = cmp::min(inc, 8);
                max = cmp::min(max, 256);
                let mut s = String::with_capacity(max);
                let _ = (0..(max - offset) * inc)
                    .into_iter()
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
                console,
                file,
                server,
                connect,
                max_size,
                backlog
            )?,
            indent,
            getframe: getframe.into(),
            format_exc: format_exc.into(),
        })
    }

    pub fn shutdown(&mut self, now: Option<bool>) -> PyResult<()> {
        self.instance.shutdown(now).map_err(|e| PyException::new_err(e))
    }

    pub fn add_logger(&mut self, logger: Py<Logger>, py: Python) {
        self.instance.add_logger(&mut logger.borrow_mut(py).instance)
    }

    pub fn remove_logger(&mut self, logger: Py<Logger>, py: Python) {
        self.instance.remove_logger(&mut logger.borrow_mut(py).instance)
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

    // Console logger

    pub fn set_console_writer(&mut self, level: Option<u8>) -> PyResult<()> {
        self.instance.set_console_writer(level).map_err(|e| PyException::new_err(e))
    }

    pub fn set_console_colors(&mut self, colors: bool) {
        self.instance.set_console_colors(colors);
    }

    // File logger

    pub fn set_file_writer(
        &mut self,
        level: Option<u8>,
        path: Option<PathBuf>,
        max_size: Option<usize>, // Maximum size of log files
        backlog: Option<usize> // Maximum number of backup log files
    ) -> PyResult<()> {
        self.instance
            .set_file_writer(level, path, max_size, backlog)
            .map_err(|e| PyException::new_err(e))
    }

    pub fn rotate(&self) -> PyResult<()> {
        self.instance.rotate().map_err(|e| PyException::new_err(e))
    }

    pub fn sync(&self, timeout: Option<f64>) -> PyResult<()> {
        self.instance.sync(timeout.unwrap_or(1.0)).map_err(|e| PyException::new_err(e))
    }

    // Network client

    pub fn connect(&mut self, address: String, level: u8, key: Option<Vec<u8>>) -> PyResult<()> {
        self.instance.connect(address, level, key).map_err(|e| PyException::new_err(e))
    }

    pub fn disconnect(&mut self, address: &str) -> PyResult<()> {
        self.instance.disconnect(address).map_err(|e| PyException::new_err(e))
    }

    pub fn set_client_level(&mut self, address: &str, level: u8) -> PyResult<()> {
        self.instance.set_client_level(address, level).map_err(|e| PyException::new_err(e))
    }

    pub fn set_client_encryption(&mut self, address: &str, key: Option<String>) -> PyResult<()> {
        self.instance
            .set_client_encryption(
                address,
                key.map(|k| k.into_bytes())
            )
            .map_err(|e| PyException::new_err(e))
    }

    // Network server

    pub fn server_start(
        &mut self,
        address: String,
        level: u8,
        key: Option<Vec<u8>>
    ) -> PyResult<()> {
        self.instance.server_start(address, level, key).map_err(|e| PyException::new_err(e))
    }

    pub fn server_shutdown(&mut self) -> PyResult<()> {
        self.instance.server_shutdown().map_err(|e| PyException::new_err(e))
    }

    pub fn set_server_level(&mut self, level: u8) -> PyResult<()> {
        self.instance.set_server_level(level).map_err(|e| PyException::new_err(e))
    }

    pub fn set_server_encryption(&mut self, key: Option<String>) -> PyResult<()> {
        self.instance
            .set_server_encryption(key.map(|k| k.into_bytes()))
            .map_err(|e| PyException::new_err(e))
    }

    pub fn debug(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= DEBUG {
            self.instance.debug(self.do_indent(obj)?).map_err(|e| PyException::new_err(e))
        } else {
            Ok(())
        }
    }

    pub fn info(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= INFO {
            self.instance.info(self.do_indent(obj)?).map_err(|e| PyException::new_err(e))
        } else {
            Ok(())
        }
    }

    pub fn warning(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= WARNING {
            self.instance.warning(self.do_indent(obj)?).map_err(|e| PyException::new_err(e))
        } else {
            Ok(())
        }
    }

    pub fn error(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= ERROR {
            self.instance.error(self.do_indent(obj)?).map_err(|e| PyException::new_err(e))
        } else {
            Ok(())
        }
    }

    pub fn critical(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= CRITICAL {
            self.instance.critical(self.do_indent(obj)?).map_err(|e| PyException::new_err(e))
        } else {
            Ok(())
        }
    }

    pub fn fatal(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level <= FATAL {
            self.instance.fatal(self.do_indent(obj)?).map_err(|e| PyException::new_err(e))
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
                    .map_err(|e| PyException::new_err(e))
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
        self.instance.shutdown(None).unwrap();
    }
}
