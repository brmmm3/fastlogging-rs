use core::slice;
use std::ffi::{CString, c_char, c_int, c_longlong, c_uchar, c_uint, c_ulong};
use std::ops::Add;
use std::path::PathBuf;
use std::ptr;
use std::sync::RwLock;
use std::time::{Duration, SystemTime};

use once_cell::sync::Lazy;

use crate::util::{char2string, option_char2string};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EncryptionMethodEnum {
    NONE,
    AuthKey,
    AES,
}

impl From<&fastlogging::EncryptionMethod> for EncryptionMethodEnum {
    fn from(key: &fastlogging::EncryptionMethod) -> Self {
        match key {
            fastlogging::EncryptionMethod::NONE => EncryptionMethodEnum::NONE,
            fastlogging::EncryptionMethod::AuthKey(_key) => EncryptionMethodEnum::AuthKey,
            fastlogging::EncryptionMethod::AES(_key) => EncryptionMethodEnum::AES,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct CKeyStruct {
    pub typ: EncryptionMethodEnum,
    pub len: c_uint,
    pub key: *const u8,
}

/// # Safety
///
/// Create and return new config for console writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn console_writer_config_new(
    level: c_uchar,
    colors: c_char,
) -> *mut fastlogging::WriterConfigEnum {
    Box::into_raw(Box::new(fastlogging::WriterConfigEnum::Console(
        fastlogging::ConsoleWriterConfig::new(level, colors != 0),
    )))
}

/// # Safety
///
/// Create and return new config for file writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn file_writer_config_new(
    level: c_uchar,
    path: *const c_char,
    size: c_uint,
    backlog: c_uint,
    timeout: c_int,
    time: c_longlong,
    compression: *mut fastlogging::CompressionMethodEnum,
) -> *mut fastlogging::WriterConfigEnum {
    let timeout = if timeout < 0 {
        None
    } else {
        Some(Duration::from_secs(timeout as u64))
    };
    let time = if time < 0 {
        None
    } else {
        Some(SystemTime::now().add(Duration::from_secs(time as u64)))
    };
    let compression = if compression.is_null() {
        None
    } else {
        Some(unsafe { *Box::from_raw(compression) })
    };
    Box::into_raw(Box::new(fastlogging::WriterConfigEnum::File(
        fastlogging::FileWriterConfig::new(
            level,
            PathBuf::from(char2string(path)),
            size as usize,
            backlog as usize,
            timeout,
            time,
            compression,
        )
        .unwrap(),
    )))
}

/// # Safety
///
/// Create and return new config for client writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn client_writer_config_new(
    level: c_uchar,
    address: *const c_char,
    key: *mut CKeyStruct,
) -> *mut fastlogging::WriterConfigEnum {
    let key = if key.is_null() {
        fastlogging::EncryptionMethod::NONE
    } else {
        let c_key = unsafe { *Box::from_raw(key) };
        let key = unsafe { slice::from_raw_parts(c_key.key, c_key.len as usize) }.to_vec();
        if c_key.typ == EncryptionMethodEnum::AuthKey {
            fastlogging::EncryptionMethod::AuthKey(key)
        } else {
            fastlogging::EncryptionMethod::AES(key)
        }
    };
    Box::into_raw(Box::new(fastlogging::WriterConfigEnum::Client(
        fastlogging::ClientWriterConfig::new(level, char2string(address), key),
    )))
}

/// # Safety
///
/// Create and return new config for server.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn server_config_new(
    level: c_uchar,
    address: *const c_char,
    key: *mut CKeyStruct,
) -> *mut fastlogging::WriterConfigEnum {
    let key = if key.is_null() {
        fastlogging::EncryptionMethod::NONE
    } else {
        let c_key = unsafe { *Box::from_raw(key) };
        if c_key.typ == EncryptionMethodEnum::NONE {
            fastlogging::EncryptionMethod::NONE
        } else {
            let key = unsafe { slice::from_raw_parts(c_key.key, c_key.len as usize) }.to_vec();
            if c_key.typ == EncryptionMethodEnum::AuthKey {
                fastlogging::EncryptionMethod::AuthKey(key)
            } else {
                fastlogging::EncryptionMethod::AES(key)
            }
        }
    };
    Box::into_raw(Box::new(fastlogging::WriterConfigEnum::Server(
        fastlogging::ServerConfig::new(level, char2string(address), key),
    )))
}

/// # Safety
///
/// Create and return new config for syslog writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syslog_writer_config_new(
    level: c_uchar,
    hostname: *const c_char,
    pname: *const c_char,
    pid: c_uint,
) -> *mut fastlogging::WriterConfigEnum {
    Box::into_raw(Box::new(fastlogging::WriterConfigEnum::Syslog(
        fastlogging::SyslogWriterConfig::new(
            level,
            option_char2string(hostname),
            char2string(pname),
            pid,
        ),
    )))
}

#[unsafe(no_mangle)]
pub static CALLBACK_C_FUNC: Lazy<
    RwLock<Option<extern "C" fn(c_uchar, *const c_char, *const c_char)>>,
> = Lazy::new(|| RwLock::new(None));

pub fn callback_func(
    level: u8,
    domain: String,
    message: String,
) -> Result<(), fastlogging::LoggingError> {
    if let Some(callback) = *CALLBACK_C_FUNC.read().unwrap() {
        let c_domain = CString::new(domain).unwrap();
        let c_message = CString::new(message).unwrap();
        callback(level, c_domain.as_ptr(), c_message.as_ptr());
    } else {
        println!("DUMMY-CB: {level} {domain}: {message}");
    }
    Ok(())
}

/// # Safety
///
/// Create and return new config for callback writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn callback_writer_config_new(
    level: c_uchar,
    callback: extern "C" fn(c_uchar, *const c_char, *const c_char),
) -> *mut fastlogging::WriterConfigEnum {
    if callback as *mut c_ulong != ptr::null_mut() {
        *CALLBACK_C_FUNC.write().unwrap() = Some(callback);
    } else {
        *CALLBACK_C_FUNC.write().unwrap() = None;
    }
    Box::into_raw(Box::new(fastlogging::WriterConfigEnum::Callback(
        fastlogging::CallbackWriterConfig::new(level, Some(Box::new(callback_func))),
    )))
}
