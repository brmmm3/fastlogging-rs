use std::ops::Add;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use once_cell::sync::Lazy;
use parking_lot::RwLock;

use fastlogging::{
    CallbackWriterConfig, ClientWriterConfig, CompressionMethodEnum, ConsoleWriterConfig,
    EncryptionMethod, FileWriterConfig, ServerConfig, SyslogWriterConfig, WriterConfigEnum,
};

use crate::get_option_str;

/// # Safety
///
/// Create a ConsoleWriterConfig and wrap it in a WriterConfigEnum (FFM).
/// The returned pointer is heap-allocated and must eventually be consumed by
/// loggingNew / loggingAddWriterConfig, which take ownership.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn consoleWriterConfigNew(level: u8, colors: bool) -> *mut WriterConfigEnum {
    Box::into_raw(Box::new(WriterConfigEnum::Console(
        ConsoleWriterConfig::new(level, colors),
    )))
}

/// # Safety
///
/// Create a FileWriterConfig and wrap it in a WriterConfigEnum (FFM).
/// `timeout_secs` = 0 means no timeout; `time_secs` = 0 means no scheduled time.
/// Returns null on invalid arguments or construction failure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fileWriterConfigNew(
    level: u8,
    path_ptr: *const u8,
    path_len: usize,
    size: usize,
    backlog: usize,
    timeout_secs: u64,
    time_secs: u64,
    compression: i8,
) -> *mut WriterConfigEnum {
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
    let config = match FileWriterConfig::new(
        level,
        PathBuf::from(path),
        size,
        backlog,
        timeout,
        time,
        compression,
    ) {
        Ok(c) => c,
        Err(_) => return std::ptr::null_mut(),
    };
    Box::into_raw(Box::new(WriterConfigEnum::File(config)))
}

/// # Safety
///
/// Create a ClientWriterConfig and wrap it in a WriterConfigEnum (FFM).
/// `encryption`: 0 = NONE, 1 = AuthKey, 2 = AES.
/// Returns null on invalid arguments.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn clientWriterConfigNew(
    level: u8,
    address_ptr: *const u8,
    address_len: usize,
    encryption: i8,
    key_ptr: *const u8,
    key_len: usize,
) -> *mut WriterConfigEnum {
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
    Box::into_raw(Box::new(WriterConfigEnum::Client(ClientWriterConfig::new(
        level, address, key,
    ))))
}

/// # Safety
///
/// Create a ServerConfig and wrap it in a WriterConfigEnum (FFM).
/// `encryption`: 0 = NONE, 1 = AuthKey, 2 = AES.
/// Returns null on invalid arguments.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn serverConfigNew(
    level: u8,
    address_ptr: *const u8,
    address_len: usize,
    encryption: i8,
    key_ptr: *const u8,
    key_len: usize,
) -> *mut WriterConfigEnum {
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
    Box::into_raw(Box::new(WriterConfigEnum::Server(ServerConfig::new(
        level, address, key,
    ))))
}

/// # Safety
///
/// Create a SyslogWriterConfig and wrap it in a WriterConfigEnum (FFM).
/// `hostname_ptr` may be null (means "no hostname").
/// Returns null on invalid arguments.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn syslogWriterConfigNew(
    level: u8,
    hostname_ptr: *const u8,
    hostname_len: usize,
    pname_ptr: *const u8,
    pname_len: usize,
    pid: u32,
) -> *mut WriterConfigEnum {
    let hostname = if !hostname_ptr.is_null() && hostname_len > 0 {
        match get_option_str(hostname_ptr, hostname_len) {
            Some(s) => Some(s.to_string()),
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
    Box::into_raw(Box::new(WriterConfigEnum::Syslog(SyslogWriterConfig::new(
        level, hostname, pname, pid,
    ))))
}

pub static CALLBACK_JAVA_FUNC: Lazy<
    RwLock<Option<extern "C" fn(i32, *const u8, usize, *const u8, usize)>>,
> = Lazy::new(|| RwLock::new(None));

fn rust_cb_func(
    level: u8,
    domain: String,
    message: String,
) -> Result<(), fastlogging::LoggingError> {
    if let Some(cb) = *CALLBACK_JAVA_FUNC.read() {
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
/// Register a Java callback and create a CallbackWriterConfig wrapped in a
/// WriterConfigEnum (FFM).  The callback is stored globally; only one
/// callback is active at a time.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn callbackWriterConfigNew(
    java_cb_func: extern "C" fn(i32, *const u8, usize, *const u8, usize),
    level: u8,
) -> *mut WriterConfigEnum {
    CALLBACK_JAVA_FUNC.write().replace(java_cb_func);
    Box::into_raw(Box::new(WriterConfigEnum::Callback(
        CallbackWriterConfig::new(level, Some(Box::new(rust_cb_func))),
    )))
}
