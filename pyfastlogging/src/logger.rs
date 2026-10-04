use std::borrow::Cow;
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
