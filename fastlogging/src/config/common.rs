use crate::MessageStructEnum;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExtConfig {
    pub(crate) structured: MessageStructEnum,
    pub(crate) hostname: bool, // Log hostname
    pub(crate) pname: bool,    // Log process name
    pub(crate) pid: bool,      // Log process ID
    pub(crate) tname: bool,    // Log thread name
    pub(crate) tid: bool,      // Log thread ID
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
