use std::{ cmp, io::Error };

use fastlogging::{ CRITICAL, DEBUG, ERROR, EXCEPTION, FATAL, INFO, WARNING };
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Logger {
    pub instance: fastlogging::Logger,
    indent: Option<(usize, usize, usize, String)>,
    getframe: Py<PyAny>,
    format_exc: Py<PyAny>,
}

impl Logger {
    fn do_indent(&self, mut message: String) -> Result<String, Error> {
        if let Some((offset, inc, max, s)) = &self.indent {
            Python::with_gil(
                |py| -> Result<String, Error> {
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
                    Ok(message)
                }
            )
        } else {
            Ok(message)
        }
    }
}

#[pymethods]
impl Logger {
    #[new]
    pub fn new(
        level: u8,
        domain: String,
        indent: Option<(usize, usize, usize)>
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
            instance: fastlogging::Logger::new(level, domain),
            indent,
            getframe: getframe.into(),
            format_exc: format_exc.into(),
        })
    }

    pub fn debug(&self, message: String) -> Result<(), Error> {
        if self.instance.level <= DEBUG {
            self.instance.debug(self.do_indent(message)?)
        } else {
            Ok(())
        }
    }

    pub fn info(&self, message: String) -> Result<(), Error> {
        if self.instance.level <= INFO {
            self.instance.info(self.do_indent(message)?)
        } else {
            Ok(())
        }
    }

    pub fn warning(&self, message: String) -> Result<(), Error> {
        if self.instance.level <= WARNING {
            self.instance.warning(self.do_indent(message)?)
        } else {
            Ok(())
        }
    }

    pub fn error(&self, message: String) -> Result<(), Error> {
        if self.instance.level <= ERROR {
            self.instance.error(self.do_indent(message)?)
        } else {
            Ok(())
        }
    }

    pub fn critical(&self, message: String) -> Result<(), Error> {
        if self.instance.level <= CRITICAL {
            self.instance.critical(self.do_indent(message)?)
        } else {
            Ok(())
        }
    }

    pub fn fatal(&self, message: String) -> Result<(), Error> {
        if self.instance.level <= FATAL {
            self.instance.fatal(self.do_indent(message)?)
        } else {
            Ok(())
        }
    }

    pub fn exception(&self, message: String, py: Python) -> Result<(), Error> {
        if self.instance.level <= EXCEPTION {
            let tb: String = self.format_exc.call0(py)?.extract(py)?;
            self.instance.exception(format!("{message}\n{tb}"))
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
