use std::ffi::{c_char, c_int, c_longlong, c_uchar, c_uint, CStr};
use std::ops::Add;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use fastlogging::{
    ClientWriterConfig, CompressionMethodEnum, ConsoleWriterConfig, EncryptionMethod,
    FileWriterConfig, ServerConfig, SyslogWriterConfig,
};

use crate::util::{char2string, option_char2string};

#[no_mangle]
pub unsafe extern "C" fn console_writer_config_new(
    level: c_uchar,
    colors: c_char,
) -> Box<ConsoleWriterConfig> {
    Box::new(ConsoleWriterConfig::new(level, colors != 0))
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
) -> Box<FileWriterConfig> {
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
    Box::new(
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
    )
}

#[no_mangle]
pub unsafe extern "C" fn client_writer_config_new(
    level: c_uchar,
    address: *const c_char,
    encryption: c_uchar,
    key: *const c_char,
) -> Box<ClientWriterConfig> {
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
    Box::new(ClientWriterConfig::new(level, char2string(address), key))
}

#[no_mangle]
pub unsafe extern "C" fn server_config_new(
    level: c_uchar,
    address: *const c_char,
    encryption: c_uchar,
    key: *const c_char,
) -> Box<ServerConfig> {
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
    Box::new(ServerConfig::new(level, char2string(address), key))
}

#[no_mangle]
pub unsafe extern "C" fn syslog_writer_config_new(
    level: c_uchar,
    hostname: *const c_char,
    pname: *const c_char,
    pid: c_uint,
) -> Box<SyslogWriterConfig> {
    Box::new(SyslogWriterConfig::new(
        level,
        option_char2string(hostname),
        char2string(pname),
        pid,
    ))
}
