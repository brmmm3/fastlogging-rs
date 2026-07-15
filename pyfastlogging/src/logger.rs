use std::cmp;

use pyo3::{exceptions::PyException, prelude::*};

use fastlogging::{CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, SUCCESS, TRACE, WARNING};

#[pyclass]
#[derive(Debug)]
pub struct Logger {
    pub instance: fastlogging::Logger,
    indent: Option<(usize, usize, usize, String)>,
    getframe: Py<PyAny>,
    format_exc: Py<PyAny>,
}

impl Logger {
    fn do_indent(&self, msg: &str) -> PyResult<String> {
        Python::attach(|py| -> PyResult<String> {
            let mut message: String = msg.to_string();
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
impl Logger {
    #[new]
    #[pyo3(signature=(level, domain, indent=None, tname=None, tid=None))]
    pub fn new(
        level: u8,
        domain: String,
        indent: Option<(usize, usize, usize)>,
        tname: Option<bool>,
        tid: Option<bool>,
        py: Python,
    ) -> PyResult<Self> {
        let (getframe, format_exc) = {
            let sys = py.import("sys")?;
            let getframe = sys.getattr("_getframe")?;
            let traceback = py.import("traceback")?;
            let format_exc = traceback.getattr("format_exc")?;
            (getframe.into(), format_exc.into())
        };
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
            instance: fastlogging::Logger::new_ext(
                level,
                domain,
                tname.unwrap_or_default(),
                tid.unwrap_or_default(),
            ),
            indent,
            getframe,
            format_exc,
        })
    }

    pub fn set_level(&mut self, level: u8) {
        self.instance.set_level(level);
    }

    pub fn level(&self) -> u8 {
        self.instance.level()
    }

    pub fn set_domain(&mut self, domain: String) {
        self.instance.set_domain(&domain);
    }

    // Logging calls

    #[pyo3(signature = (msg, /))]
    pub fn trace(&self, msg: &str) -> PyResult<()> {
        if self.instance.level() <= TRACE {
            self.instance
                .trace(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature = (msg, /))]
    pub fn debug(&self, msg: &str) -> PyResult<()> {
        if self.instance.level() <= DEBUG {
            self.instance
                .debug(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature = (msg, /))]
    pub fn info(&self, msg: &str) -> PyResult<()> {
        if self.instance.level() <= INFO {
            self.instance
                .info(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature = (msg, /))]
    pub fn success(&self, msg: &str) -> PyResult<()> {
        if self.instance.level() <= SUCCESS {
            self.instance
                .success(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature = (msg, /))]
    pub fn warning(&self, msg: &str) -> PyResult<()> {
        if self.instance.level() <= WARNING {
            self.instance
                .warning(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature = (msg, /))]
    pub fn error(&self, msg: &str) -> PyResult<()> {
        if self.instance.level() <= ERROR {
            self.instance
                .error(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature = (msg, /))]
    pub fn critical(&self, msg: &str) -> PyResult<()> {
        if self.instance.level() <= CRITICAL {
            self.instance
                .critical(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature = (msg, /))]
    pub fn fatal(&self, msg: &str) -> PyResult<()> {
        if self.instance.level() <= FATAL {
            self.instance
                .fatal(self.do_indent(msg)?)
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    #[pyo3(signature = (msg, /))]
    pub fn exception(&self, msg: &str, py: Python) -> PyResult<()> {
        if self.instance.level() <= EXCEPTION {
            let tb: String = self.format_exc.call0(py)?.extract(py)?;
            self.instance
                .exception(format!("{msg}\n{tb}"))
                .map_err(|e| PyException::new_err(e.to_string()))
        } else {
            Ok(())
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}
