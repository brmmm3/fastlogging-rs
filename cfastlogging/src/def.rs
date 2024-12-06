use std::collections::HashMap;
use std::ffi::{c_char, c_uchar, c_uint, c_ushort, CString};
use std::ptr::null;

use fastlogging::{
    EncryptionMethod, ExtConfig, MessageStructEnum, ServerConfig, WriterConfigEnum, WriterEnum,
};

use crate::CEncryptionMethodEnum;

#[repr(C)]
pub struct CusizeVec {
    pub cnt: c_uint,
    pub values: Vec<usize>,
}

#[repr(C)]
pub enum CWriterEnum {
    Root,
    Console,
    File,
    Client,
    Server,
    Callback,
    Syslog,
}

impl From<WriterEnum> for CWriterEnum {
    fn from(value: WriterEnum) -> Self {
        match value {
            WriterEnum::Root => CWriterEnum::Root,
            WriterEnum::Console(_console_writer) => CWriterEnum::Console,
            WriterEnum::File(_file_writer) => CWriterEnum::File,
            WriterEnum::Client(_client_writer) => CWriterEnum::Client,
            WriterEnum::Server(_logging_server) => CWriterEnum::Server,
            WriterEnum::Callback(_callback_writer) => CWriterEnum::Callback,
            WriterEnum::Syslog(_syslog_writer) => CWriterEnum::Syslog,
        }
    }
}

#[repr(C)]
pub struct CWriterEnums {
    pub cnt: c_uint,
    pub values: *const CWriterEnum,
}

#[repr(C)]
pub struct CWriterConfigEnums {
    pub cnt: c_uint,
    pub keys: Vec<usize>,
    pub values: Vec<WriterConfigEnum>,
}

#[repr(C)]
pub struct CEncryptionMethod {
    typ: CEncryptionMethodEnum,
    len: u32,
    key: *const u8,
}

impl From<EncryptionMethod> for CEncryptionMethod {
    fn from(value: EncryptionMethod) -> Self {
        match value {
            EncryptionMethod::NONE => CEncryptionMethod {
                typ: CEncryptionMethodEnum::NONE,
                len: 0,
                key: null(),
            },
            EncryptionMethod::AuthKey(key) => CEncryptionMethod {
                typ: CEncryptionMethodEnum::AuthKey,
                len: key.len() as u32,
                key: Box::into_raw(Box::new(key)) as *const u8,
            },
            EncryptionMethod::AES(key) => CEncryptionMethod {
                typ: CEncryptionMethodEnum::AES,
                len: key.len() as u32,
                key: Box::into_raw(Box::new(key)) as *const u8,
            },
        }
    }
}

#[repr(C)]
pub struct CServerConfig {
    level: u8,
    address: *const char,
    port: u16,
    key: *const CEncryptionMethod,
    port_file: *const char,
}

impl From<ServerConfig> for CServerConfig {
    fn from(config: ServerConfig) -> Self {
        CServerConfig {
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
pub struct CServerConfigs {
    pub cnt: c_uint,
    pub keys: *const u32,
    pub values: *const CServerConfig,
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
        let ports = items.values().map(|v| *v as u16).collect::<Vec<_>>();
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
#[no_mangle]
pub unsafe extern "C" fn ext_config_new(
    structured: c_uchar,
    hostname: c_char,
    pname: c_char,
    pid: c_char,
    tname: c_char,
    tid: c_char,
) -> *const ExtConfig {
    let structured = match structured {
        0 => MessageStructEnum::String,
        1 => MessageStructEnum::Json,
        2 => MessageStructEnum::Xml,
        _ => MessageStructEnum::String,
    };
    Box::into_raw(Box::new(ExtConfig::new(
        structured,
        hostname != 0,
        pname != 0,
        pid != 0,
        tname != 0,
        tid != 0,
    )))
}
