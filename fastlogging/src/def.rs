use std::fmt;

// Log-Levels
pub const NOLOG: u8 = 70;
pub const EXCEPTION: u8 = 60;
pub const CRITICAL: u8 = 50;
pub const FATAL: u8 = CRITICAL;
pub const ERROR: u8 = 40;
pub const WARNING: u8 = 30;
pub const WARN: u8 = WARNING;
pub const INFO: u8 = 20;
pub const DEBUG: u8 = 10;
pub const NOTSET: u8 = 0;

pub fn level2str(level: u8) -> &'static str {
    match level {
        0..=9 => "NOTSET",
        10..=19 => "DEBUG",
        20..=29 => "INFO",
        30..=39 => "WARNING",
        40..=49 => "ERROR",
        50..=59 => "FATAL",
        60..=69 => "EXCEPTION",
        _ => "NOLOG",
    }
}

pub fn level2short(level: u8) -> &'static str {
    match level {
        0..=9 => "NOT",
        10..=19 => "DBG",
        20..=29 => "INF",
        30..=39 => "WRN",
        40..=49 => "ERR",
        50..=59 => "FTL",
        60..=69 => "EXC",
        _ => "NOL",
    }
}

pub fn level2sym(level: u8) -> &'static str {
    match level {
        0..=9 => "N",
        10..=19 => "D",
        20..=29 => "I",
        30..=39 => "W",
        40..=49 => "E",
        50..=59 => "F",
        60..=69 => "!",
        _ => "-",
    }
}

#[derive(Debug, Clone)]
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
pub enum MessageTypeEnum {
    Message((u8, String)),
    Sync(f64),
    Rotate,
    Stop,
}
