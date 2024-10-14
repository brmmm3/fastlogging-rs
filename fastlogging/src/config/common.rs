use crate::MessageStructEnum;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExtConfig {
    pub structured: MessageStructEnum,
    pub hostname: bool, // Log hostname
    pub pname: bool,    // Log process name
    pub pid: bool,      // Log process ID
    pub tname: bool,    // Log thread name
    pub tid: bool,      // Log thread ID
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
