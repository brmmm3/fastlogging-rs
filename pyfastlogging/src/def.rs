use pyo3::{ exceptions::PyValueError, prelude::* };

#[pyclass]
pub enum Level2Sym {
    NotSet = 0,
    Debug = 10,
    Info = 20,
    Warning = 30,
    Error = 40,
    Critical = 50,
    Exception = 60,
    NoLog = 70,
}

#[pymethods]
impl Level2Sym {
    #[new]
    fn new(value: u8) -> PyResult<Self> {
        match value {
            0 => Ok(Level2Sym::NotSet),
            10 => Ok(Level2Sym::Debug),
            20 => Ok(Level2Sym::Info),
            30 => Ok(Level2Sym::Warning),
            40 => Ok(Level2Sym::Error),
            50 => Ok(Level2Sym::Critical),
            60 => Ok(Level2Sym::Exception),
            70 => Ok(Level2Sym::NoLog),
            _ => Err(PyValueError::new_err(format!("Invalid value {value}"))),
        }
    }

    #[getter]
    fn value(&self) -> u8 {
        match self {
            Self::NotSet => 0,
            Self::Debug => 10,
            Self::Info => 20,
            Self::Warning => 30,
            Self::Error => 40,
            Self::Critical => 50,
            Self::Exception => 60,
            Self::NoLog => 70,
        }
    }

    #[getter]
    fn name(&self) -> &'static str {
        match self {
            Self::NotSet => "NOTSET",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warning => "WARNING",
            Self::Error => "ERROR",
            Self::Critical => "CRITICAL",
            Self::Exception => "EXCEPTION",
            Self::NoLog => "NOLOG",
        }
    }
}

#[pyclass]
pub struct LevelSyms(pub fastlogging::LevelSyms);

#[pymethods]
impl LevelSyms {
    #[new]
    fn new() -> Self {
        Self(fastlogging::LevelSyms::Sym)
    }

    #[getter]
    pub fn value(&self) -> u8 {
        self.0.clone() as u8
    }

    #[getter]
    pub fn name(&self) -> String {
        self.0.to_string()
    }
}
