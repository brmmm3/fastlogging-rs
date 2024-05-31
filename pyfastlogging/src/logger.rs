use std::{cmp, io::Error};

use fastlogging::{CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, SUCCESS, TRACE, WARNING};
use pyo3::{exceptions::PyException, prelude::*};

#[pyclass]
#[derive(Debug)]
pub struct Logger {
    pub instance: fastlogging::Logger,
    indent: Option<(usize, usize, usize, String)>,
    getframe: Py<PyAny>,
    format_exc: Py<PyAny>,
}

impl Logger {
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
impl Logger {
    #[new]
    pub fn new(
        level: u8,
        domain: String,
        indent: Option<(usize, usize, usize)>,
        tname: Option<bool>,
        tid: Option<bool>,
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
            instance: fastlogging::Logger::new_ext(
                level,
                domain,
                tname.unwrap_or_default(),
                tid.unwrap_or_default(),
            ),
            indent,
            getframe: getframe.into(),
            format_exc: format_exc.into(),
        })
    }

    pub fn set_level(&mut self, level: u8) {
        self.instance.set_level(level);
    }

    pub fn level(&self) -> u8 {
        self.instance.level()
    }

    pub fn set_domain(&mut self, domain: String) {
        self.instance.set_domain(domain);
    }

    // Logging calls

    pub fn trace(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level() <= TRACE {
            self.instance
                .trace(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn debug(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level() <= DEBUG {
            self.instance
                .debug(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn info(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level() <= INFO {
            self.instance
                .info(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn success(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level() <= SUCCESS {
            self.instance
                .success(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn warning(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level() <= WARNING {
            self.instance
                .warning(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn error(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level() <= ERROR {
            self.instance
                .error(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn critical(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level() <= CRITICAL {
            self.instance
                .critical(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn fatal(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level() <= FATAL {
            self.instance
                .fatal(self.do_indent(obj)?)
                .map_err(PyException::new_err)
        } else {
            Ok(())
        }
    }

    pub fn exception(&self, obj: PyObject) -> PyResult<()> {
        if self.instance.level() <= EXCEPTION {
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
        format!("{self:?}")
    }

    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}
