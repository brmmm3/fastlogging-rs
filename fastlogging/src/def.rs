use std::fmt;

// Log-Levels
pub const NOLOG: u8 = 70;
pub const EXCEPTION: u8 = 60;
pub const CRITICAL: u8 = 50;
pub const FATAL: u8 = CRITICAL;
pub const ERROR: u8 = 40;
pub const WARNING: u8 = 30;
pub const WARN: u8 = WARNING;
pub const SUCCESS: u8 = 25;
pub const INFO: u8 = 20;
pub const DEBUG: u8 = 10;
pub const TRACE: u8 = 5;
pub const NOTSET: u8 = 0;

pub fn level2str(level: u8) -> &'static str {
    match level {
        NOTSET..TRACE => "NOTSET",
        TRACE..DEBUG => "TRACE",
        DEBUG..INFO => "DEBUG",
        INFO..SUCCESS => "INFO",
        SUCCESS..WARNING => "SUCCESS",
        WARNING..ERROR => "WARNING",
        ERROR..FATAL => "ERROR",
        FATAL..EXCEPTION => "FATAL",
        EXCEPTION..NOLOG => "EXCEPTION",
        _ => "NOLOG",
    }
}

pub fn level2short(level: u8) -> &'static str {
    match level {
        NOTSET..TRACE => "NOT",
        TRACE..DEBUG => "TRC",
        DEBUG..INFO => "DBG",
        INFO..SUCCESS => "INF",
        SUCCESS..WARNING => "SCS",
        WARNING..ERROR => "WRN",
        ERROR..FATAL => "ERR",
        FATAL..EXCEPTION => "FTL",
        EXCEPTION..NOLOG => "EXC",
        _ => "NOL",
    }
}

pub fn level2sym(level: u8) -> &'static str {
    match level {
        NOTSET..TRACE => "N",
        TRACE..DEBUG => "T",
        DEBUG..INFO => "D",
        INFO..SUCCESS => "I",
        SUCCESS..WARNING => "S",
        WARNING..ERROR => "W",
        ERROR..FATAL => "E",
        FATAL..EXCEPTION => "F",
        EXCEPTION..NOLOG => "!",
        _ => "-",
    }
}

pub fn level2string(levelsym: &LevelSyms, level: u8) -> &'static str {
    match levelsym {
        LevelSyms::Sym => level2sym(level),
        LevelSyms::Short => level2short(level),
        LevelSyms::Str => level2str(level),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LevelSyms {
    Sym,
    Short,
    Str,
}

impl fmt::Display for LevelSyms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug)]
pub enum LoggingTypeEnum {
    Message((u8, String)),                 // level, message
    MessageRemote((u8, String)),           // level, message
    MessageExt((u8, String, u32, String)), // level, tname, tid, message
    Sync(f64),                             // timeout
    Rotate,
    Stop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageStructEnum {
    String,
    Json,
    Xml,
}

impl fmt::Display for MessageStructEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
