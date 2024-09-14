use std::ffi::{c_char, c_int, c_longlong, c_uchar, c_uint, c_ulong, CStr, CString};
use std::ops::Add;
use std::path::PathBuf;
use std::ptr;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use fastlogging::{
    CallbackWriterConfig, ClientWriterConfig, CompressionMethodEnum, ConsoleWriterConfig,
    EncryptionMethod, FileWriterConfig, ServerConfig, SyslogWriterConfig, WriterConfigEnum,
};
use once_cell::sync::Lazy;

use crate::util::{char2string, option_char2string};

#[no_mangle]
pub unsafe extern "C" fn console_writer_config_new(
    level: c_uchar,
    colors: c_char,
) -> *mut ConsoleWriterConfig {
    Box::into_raw(Box::new(ConsoleWriterConfig::new(level, colors != 0)))
}

#[no_mangle]
pub unsafe extern "C" fn console_writer_config_enum_new(
    level: c_uchar,
    colors: c_char,
) -> *mut WriterConfigEnum {
    Box::into_raw(Box::new(WriterConfigEnum::Console(
        ConsoleWriterConfig::new(level, colors != 0),
    )))
}

#[no_mangle]
pub unsafe extern "C" fn file_writer_config_new(
    level: c_uchar,
    path: *const c_char,
    size: c_uint,
    backlog: c_uint,
    timeout: c_int,
    time: c_longlong,
    compression: *mut CompressionMethodEnum,
) -> *mut FileWriterConfig {
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
        Some(*Box::from_raw(compression))
    };
    Box::into_raw(Box::new(
        FileWriterConfig::new(
            level,
            PathBuf::from(char2string(path)),
            size as usize,
            backlog as usize,
            timeout,
            time,
            compression,
        )
        .unwrap(),
    ))
}

#[no_mangle]
pub unsafe extern "C" fn file_writer_config_enum_new(
    level: c_uchar,
    path: *const c_char,
    size: c_uint,
    backlog: c_uint,
    timeout: c_int,
    time: c_longlong,
    compression: *mut CompressionMethodEnum,
) -> *mut WriterConfigEnum {
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
        Some(*Box::from_raw(compression))
    };
    Box::into_raw(Box::new(WriterConfigEnum::File(
        FileWriterConfig::new(
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

#[no_mangle]
pub unsafe extern "C" fn client_writer_config_new(
    level: c_uchar,
    address: *const c_char,
    encryption: c_uchar,
    key: *const c_char,
) -> *mut ClientWriterConfig {
    let key = if encryption == 0 || key.is_null() {
        EncryptionMethod::NONE
    } else {
        let key = (unsafe { CStr::from_ptr(key) })
            .to_str()
            .unwrap()
            .as_bytes()
            .to_vec();
        if encryption == 1 {
            EncryptionMethod::AuthKey(key)
        } else {
            EncryptionMethod::AES(key)
        }
    };
    Box::into_raw(Box::new(ClientWriterConfig::new(
        level,
        char2string(address),
        key,
    )))
}

#[no_mangle]
pub unsafe extern "C" fn client_writer_config_enum_new(
    level: c_uchar,
    address: *const c_char,
    encryption: c_uchar,
    key: *const c_char,
) -> *mut WriterConfigEnum {
    let key = if encryption == 0 || key.is_null() {
        EncryptionMethod::NONE
    } else {
        let key = (unsafe { CStr::from_ptr(key) })
            .to_str()
            .unwrap()
            .as_bytes()
            .to_vec();
        if encryption == 1 {
            EncryptionMethod::AuthKey(key)
        } else {
            EncryptionMethod::AES(key)
        }
    };
    Box::into_raw(Box::new(WriterConfigEnum::Client(ClientWriterConfig::new(
        level,
        char2string(address),
        key,
    ))))
}

#[no_mangle]
pub unsafe extern "C" fn server_config_new(
    level: c_uchar,
    address: *const c_char,
    encryption: c_uchar,
    key: *const c_char,
) -> *mut ServerConfig {
    let key = if encryption == 0 || key.is_null() {
        EncryptionMethod::NONE
    } else {
        let key = (unsafe { CStr::from_ptr(key) })
            .to_str()
            .unwrap()
            .as_bytes()
            .to_vec();
        if encryption == 1 {
            EncryptionMethod::AuthKey(key)
        } else {
            EncryptionMethod::AES(key)
        }
    };
    Box::into_raw(Box::new(ServerConfig::new(
        level,
        char2string(address),
        key,
    )))
}

#[no_mangle]
pub unsafe extern "C" fn server_config_enum_new(
    level: c_uchar,
    address: *const c_char,
    encryption: c_uchar,
    key: *const c_char,
) -> *mut WriterConfigEnum {
    let key = if encryption == 0 || key.is_null() {
        EncryptionMethod::NONE
    } else {
        let key = (unsafe { CStr::from_ptr(key) })
            .to_str()
            .unwrap()
            .as_bytes()
            .to_vec();
        if encryption == 1 {
            EncryptionMethod::AuthKey(key)
        } else {
            EncryptionMethod::AES(key)
        }
    };
    Box::into_raw(Box::new(WriterConfigEnum::Server(ServerConfig::new(
        level,
        char2string(address),
        key,
    ))))
}

#[no_mangle]
pub unsafe extern "C" fn syslog_writer_config_new(
    level: c_uchar,
    hostname: *const c_char,
    pname: *const c_char,
    pid: c_uint,
) -> *mut SyslogWriterConfig {
    Box::into_raw(Box::new(SyslogWriterConfig::new(
        level,
        option_char2string(hostname),
        char2string(pname),
        pid,
    )))
}

#[no_mangle]
pub unsafe extern "C" fn syslog_writer_config_enum_new(
    level: c_uchar,
    hostname: *const c_char,
    pname: *const c_char,
    pid: c_uint,
) -> *mut WriterConfigEnum {
    Box::into_raw(Box::new(WriterConfigEnum::Syslog(SyslogWriterConfig::new(
        level,
        option_char2string(hostname),
        char2string(pname),
        pid,
    ))))
}

pub static CALLBACK_C_FUNC: Lazy<
    Mutex<Option<extern "C" fn(c_uchar, *const c_char, *const c_char)>>,
> = Lazy::new(|| Mutex::new(None));

pub fn callback_func(
    level: u8,
    domain: String,
    message: String,
) -> Result<(), fastlogging::LoggingError> {
    if let Some(callback) = *CALLBACK_C_FUNC.lock().unwrap() {
        let c_domain = CString::new(domain).unwrap();
        let c_message = CString::new(message).unwrap();
        callback(level, c_domain.as_ptr(), c_message.as_ptr());
    } else {
        println!("DUMMY-CB: {level} {domain}: {message}");
    }
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn callback_writer_config_new(
    level: c_uchar,
    callback: extern "C" fn(c_uchar, *const c_char, *const c_char),
) -> *mut CallbackWriterConfig {
    if callback as *mut c_ulong != ptr::null_mut() {
        *CALLBACK_C_FUNC.lock().unwrap() = Some(callback);
    } else {
        *CALLBACK_C_FUNC.lock().unwrap() = None;
    }
    Box::into_raw(Box::new(CallbackWriterConfig::new(
        level,
        Some(Box::new(callback_func)),
    )))
}

#[no_mangle]
pub unsafe extern "C" fn callback_writer_config_enum_new(
    level: c_uchar,
    callback: extern "C" fn(c_uchar, *const c_char, *const c_char),
) -> *mut WriterConfigEnum {
    if callback as *mut c_ulong != ptr::null_mut() {
        *CALLBACK_C_FUNC.lock().unwrap() = Some(callback);
    } else {
        *CALLBACK_C_FUNC.lock().unwrap() = None;
    }
    Box::into_raw(Box::new(WriterConfigEnum::Callback(
        CallbackWriterConfig::new(level, Some(Box::new(callback_func))),
    )))
}
