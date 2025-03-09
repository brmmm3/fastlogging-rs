use crate::MessageStructEnum;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExtConfig {
    /// Set log message structuring.
    pub structured: MessageStructEnum,
    /// Include hostname in log messages.
    pub hostname: bool,
    /// Include process name in log messages.
    pub pname: bool,
    /// Include process id in log messages.
    pub pid: bool,
    /// Include thread name in log messages.
    pub tname: bool,
    /// Include thread id in log messages.
    pub tid: bool,
}

impl ExtConfig {
    pub fn new(
        structured: MessageStructEnum,
        hostname: bool,
        pname: bool,
        pid: bool,
        tname: bool,
        tid: bool,
    ) -> Self {
        Self {
            structured,
            hostname,
            pname,
            pid,
            tname,
            tid,
        }
    }
}

impl Default for ExtConfig {
    fn default() -> Self {
        Self {
            hostname: false,
            pname: false,
            pid: false,
            tname: false,
            tid: false,
            structured: MessageStructEnum::String,
        }
    }
}
