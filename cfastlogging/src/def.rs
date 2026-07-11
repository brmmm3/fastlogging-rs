use std::collections::HashMap;
use std::ffi::{CString, c_char, c_uchar, c_uint, c_ushort};
use std::ptr::null;

use crate::EncryptionMethodEnum;

#[repr(C)]
pub struct CusizeVec {
    pub cnt: c_uint,
    pub values: Vec<usize>,
}

#[repr(C)]
pub enum WriterEnum {
    Root,
    Console,
    File,
    Client,
    Server,
    Callback,
    Syslog,
}

impl From<fastlogging::WriterEnum> for WriterEnum {
    fn from(value: fastlogging::WriterEnum) -> Self {
        match value {
            fastlogging::WriterEnum::Root => WriterEnum::Root,
            fastlogging::WriterEnum::Console(_console_writer) => WriterEnum::Console,
            fastlogging::WriterEnum::File(_file_writer) => WriterEnum::File,
            fastlogging::WriterEnum::Client(_client_writer) => WriterEnum::Client,
            fastlogging::WriterEnum::Server(_logging_server) => WriterEnum::Server,
            fastlogging::WriterEnum::Callback(_callback_writer) => WriterEnum::Callback,
            fastlogging::WriterEnum::Syslog(_syslog_writer) => WriterEnum::Syslog,
        }
    }
}

#[repr(C)]
pub struct WriterEnums {
    pub cnt: c_uint,
    pub values: *const WriterEnum,
}

#[repr(C)]
pub struct WriterConfigEnums {
    pub cnt: c_uint,
    pub keys: Vec<usize>,
    pub values: Vec<fastlogging::WriterConfigEnum>,
}

#[repr(C)]
pub struct EncryptionMethod {
    typ: EncryptionMethodEnum,
    len: u32,
    key: *const u8,
}

impl From<fastlogging::EncryptionMethod> for EncryptionMethod {
    fn from(value: fastlogging::EncryptionMethod) -> Self {
        match value {
            fastlogging::EncryptionMethod::NONE => EncryptionMethod {
                typ: EncryptionMethodEnum::NONE,
                len: 0,
                key: null(),
            },
            fastlogging::EncryptionMethod::AuthKey(key) => EncryptionMethod {
                typ: EncryptionMethodEnum::AuthKey,
                len: key.len() as u32,
                key: Box::into_raw(Box::new(key)) as *const u8,
            },
            fastlogging::EncryptionMethod::AES(key) => EncryptionMethod {
                typ: EncryptionMethodEnum::AES,
                len: key.len() as u32,
                key: Box::into_raw(Box::new(key)) as *const u8,
            },
        }
    }
}

#[repr(C)]
pub struct ServerConfig {
    level: u8,
    address: *const char,
    port: u16,
    key: *const EncryptionMethod,
    port_file: *const char,
}

impl From<fastlogging::ServerConfig> for ServerConfig {
    fn from(config: fastlogging::ServerConfig) -> Self {
        ServerConfig {
            level: config.level,
            address: CString::new(config.address)
                .expect("Error: CString::new()")
                .into_raw() as *const char,
            port: config.port,
            key: Box::into_raw(Box::new(config.key.into())),
            port_file: match config.port_file {
                Some(v) => CString::new(v.to_str().unwrap())
                    .expect("Error: CString::new()")
                    .into_raw() as *const char,
                None => null(),
            },
        }
    }
}

#[repr(C)]
pub struct ServerConfigs {
    pub cnt: c_uint,
    pub keys: *const u32,
    pub values: *const ServerConfig,
}

#[repr(C)]
pub struct Cu32StringVec {
    cnt: c_uint,
    keys: *const c_uint,
    values: *const *const c_char, // List of C-Strings
}

impl From<HashMap<usize, String>> for Cu32StringVec {
    fn from(items: HashMap<usize, String>) -> Self {
        let wids = items.keys().map(|v| *v as u32).collect::<Vec<_>>();
        let strings = items
            .values()
            .filter_map(|v| CString::new(v.clone()).ok())
            .collect::<Vec<_>>();
        let c_wids = wids.as_ptr() as *const c_uint;
        let c_strings = strings.as_ptr() as *const *const c_char;
        std::mem::forget(wids);
        std::mem::forget(strings);
        Cu32StringVec {
            cnt: items.len() as c_uint,
            keys: c_wids,
            values: c_strings,
        }
    }
}

#[repr(C)]
pub struct Cu32u16Vec {
    cnt: c_uint,
    keys: *const c_uint,
    values: *const c_ushort,
}

impl From<HashMap<usize, u16>> for Cu32u16Vec {
    fn from(items: HashMap<usize, u16>) -> Self {
        let wids = items.keys().map(|v| *v as u32).collect::<Vec<_>>();
        let ports = items.values().copied().collect::<Vec<_>>();
        let c_wids = wids.as_ptr() as *const c_uint;
        let c_ports = ports.as_ptr() as *const c_ushort;
        std::mem::forget(wids);
        std::mem::forget(ports);
        Cu32u16Vec {
            cnt: items.len() as c_uint,
            keys: c_wids,
            values: c_ports,
        }
    }
}

/*#[inline]
fn cchar2vec(s: *const c_char) -> Vec<u8> {
    (unsafe { CStr::from_ptr(s) })
        .to_str()
        .unwrap()
        .as_bytes()
        .to_vec()
}*/

/// # Safety
///
/// Create extended configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ext_config_new(
    structured: c_uchar,
    hostname: c_char,
    pname: c_char,
    pid: c_char,
    tname: c_char,
    tid: c_char,
) -> *const fastlogging::ExtConfig {
    let structured = match structured {
        0 => fastlogging::MessageStructEnum::String,
        1 => fastlogging::MessageStructEnum::Json,
        2 => fastlogging::MessageStructEnum::Xml,
        _ => fastlogging::MessageStructEnum::String,
    };
    Box::into_raw(Box::new(fastlogging::ExtConfig::new(
        structured,
        hostname != 0,
        pname != 0,
        pid != 0,
        tname != 0,
        tid != 0,
    )))
}
