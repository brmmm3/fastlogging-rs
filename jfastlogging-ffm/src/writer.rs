use std::ops::Add;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use fastlogging::{
    CallbackWriterConfig, ClientWriterConfig, CompressionMethodEnum, ConsoleWriterConfig,
    EncryptionMethod, FileWriterConfig, ServerConfig, SyslogWriterConfig,
};
use once_cell::sync::Lazy;

use crate::get_option_str;

/// # Safety
///
/// Create a new ConsoleWriterConfig (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_consoleWriterConfigNew(
    level: u8,
    colors: bool,
) -> *mut ConsoleWriterConfig {
    let console = ConsoleWriterConfig::new(level, colors);
    Box::into_raw(Box::new(console))
}

/// # Safety
///
/// Create a new FileWriterConfig (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_fileWriterConfigNew(
    level: u8,
    path_ptr: *const u8,
    path_len: usize,
    size: usize,
    backlog: usize,
    timeout_secs: u64,
    time_secs: u64,
    compression: i8,
) -> *mut FileWriterConfig {
    if path_ptr.is_null() || path_len == 0 {
        return std::ptr::null_mut();
    }
    let path = match get_option_str(path_ptr, path_len) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    let timeout = if timeout_secs > 0 {
        Some(Duration::from_secs(timeout_secs))
    } else {
        None
    };
    let time = if time_secs > 0 {
        Some(SystemTime::now().add(Duration::from_secs(time_secs)))
    } else {
        None
    };
    let compression = Some(match compression {
        0 => CompressionMethodEnum::Store,
        1 => CompressionMethodEnum::Deflate,
        2 => CompressionMethodEnum::Zstd,
        3 => CompressionMethodEnum::Lzma,
        _ => return std::ptr::null_mut(),
    });
    let writer = match FileWriterConfig::new(
        level,
        PathBuf::from(path),
        size,
        backlog,
        timeout,
        time,
        compression,
    ) {
        Ok(w) => w,
        Err(_) => return std::ptr::null_mut(),
    };
    Box::into_raw(Box::new(writer))
}

/// # Safety
///
/// Create a new ClientWriterConfig (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_clientWriterConfigNew(
    level: u8,
    address_ptr: *const u8,
    address_len: usize,
    encryption: i8,
    key_ptr: *const u8,
    key_len: usize,
) -> *mut ClientWriterConfig {
    if address_ptr.is_null() || address_len == 0 {
        return std::ptr::null_mut();
    }
    let address = match get_option_str(address_ptr, address_len) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    let key = if encryption == 0 || key_ptr.is_null() || key_len == 0 {
        EncryptionMethod::NONE
    } else {
        let key_bytes = unsafe { std::slice::from_raw_parts(key_ptr, key_len).to_vec() };
        if encryption == 1 {
            EncryptionMethod::AuthKey(key_bytes)
        } else {
            EncryptionMethod::AES(key_bytes)
        }
    };
    Box::into_raw(Box::new(ClientWriterConfig::new(level, address, key)))
}

/// # Safety
///
/// Create a new ServerConfig (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_serverConfigNew(
    level: u8,
    address_ptr: *const u8,
    address_len: usize,
    encryption: i8,
    key_ptr: *const u8,
    key_len: usize,
) -> *mut ServerConfig {
    if address_ptr.is_null() || address_len == 0 {
        return std::ptr::null_mut();
    }
    let address = match get_option_str(address_ptr, address_len) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    let key = if encryption == 0 || key_ptr.is_null() || key_len == 0 {
        EncryptionMethod::NONE
    } else {
        let key_bytes = unsafe { std::slice::from_raw_parts(key_ptr, key_len).to_vec() };
        if encryption == 1 {
            EncryptionMethod::AuthKey(key_bytes)
        } else {
            EncryptionMethod::AES(key_bytes)
        }
    };
    Box::into_raw(Box::new(ServerConfig::new(level, address, key)))
}

/// # Safety
///
/// Create a new SyslogWriterConfig (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_syslogWriterConfigNew(
    level: u8,
    hostname_ptr: *const u8,
    hostname_len: usize,
    pname_ptr: *const u8,
    pname_len: usize,
    pid: u32,
) -> *mut SyslogWriterConfig {
    let hostname = if !hostname_ptr.is_null() && hostname_len > 0 {
        match get_option_str(hostname_ptr, hostname_len) {
            Some(s) => Some(s),
            None => return std::ptr::null_mut(),
        }
    } else {
        None
    };
    if pname_ptr.is_null() || pname_len == 0 {
        return std::ptr::null_mut();
    }
    let pname = match get_option_str(pname_ptr, pname_len) {
        Some(s) => s,
        None => return std::ptr::null_mut(),
    };
    Box::into_raw(Box::new(SyslogWriterConfig::new(
        level,
        hostname.map(|v| v.to_string()),
        pname,
        pid,
    )))
}

pub static CALLBACK_JAVA_FUNC: Lazy<
    Mutex<Option<extern "C" fn(i32, *const u8, usize, *const u8, usize)>>,
> = Lazy::new(|| Mutex::new(None));

fn rust_cb_func(
    level: u8,
    domain: String,
    message: String,
) -> Result<(), fastlogging::LoggingError> {
    if let Some(cb) = *CALLBACK_JAVA_FUNC.lock().unwrap() {
        cb(
            level as i32,
            domain.as_ptr(),
            domain.len(),
            message.as_ptr(),
            message.len(),
        );
    }
    Ok(())
}

/// # Safety
///
/// Create a new CallbackWriterConfig (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_callbackWriterConfigNew(
    java_cb_func: extern "C" fn(i32, *const u8, usize, *const u8, usize),
    level: u8,
) -> *mut CallbackWriterConfig {
    CALLBACK_JAVA_FUNC.lock().unwrap().replace(java_cb_func);
    Box::into_raw(Box::new(CallbackWriterConfig::new(
        level,
        Some(Box::new(rust_cb_func)),
    )))
}
