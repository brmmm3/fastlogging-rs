use std::collections::HashMap;
use std::ffi::{c_char, c_double, c_uchar, c_uint, c_ushort, c_void, CStr, CString};
use std::path::PathBuf;
use std::ptr::null;
use std::slice;

use fastlogging::{
    EncryptionMethod, ExtConfig, LevelSyms, Logger, Logging, MessageStructEnum, WriterEnum,
};

use crate::util::char2string;
use crate::{EncryptionMethodEnum, KeyStruct};

#[repr(C)]
pub struct CusizeVec {
    cnt: c_uint,
    values: *const c_uint,
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

#[repr(C)]
pub struct ServerConfig {
    level: u8,
    address: *const c_char,
    port: u16,
    key: *mut KeyStruct,
}

#[repr(C)]
pub struct ServerConfigs {
    cnt: c_uint,
    keys: *const u32,
    values: *const ServerConfig,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WriterEnumTyp {
    Root,
    Console,
    File,
    Client,
    Server,
    Callback,
    Syslog,
}

pub fn writer_config_as_void(config: fastlogging::WriterConfigEnum) -> *const c_void {
    match config {
        fastlogging::WriterConfigEnum::Root(root_config) => {
            Box::into_raw(Box::new(root_config)) as *const c_void
        }
        fastlogging::WriterConfigEnum::Console(console_writer_config) => {
            Box::into_raw(Box::new(console_writer_config)) as *const c_void
        }
        fastlogging::WriterConfigEnum::File(file_writer_config) => {
            Box::into_raw(Box::new(file_writer_config)) as *const c_void
        }
        fastlogging::WriterConfigEnum::Client(client_writer_config) => {
            Box::into_raw(Box::new(client_writer_config)) as *const c_void
        }
        fastlogging::WriterConfigEnum::Server(server_config) => {
            Box::into_raw(Box::new(server_config)) as *const c_void
        }
        fastlogging::WriterConfigEnum::Callback(callback_writer_config) => {
            Box::into_raw(Box::new(callback_writer_config)) as *const c_void
        }
        fastlogging::WriterConfigEnum::Syslog(syslog_writer_config) => {
            Box::into_raw(Box::new(syslog_writer_config)) as *const c_void
        }
    }
}

impl From<fastlogging::WriterConfigEnum> for WriterEnumTyp {
    fn from(config: fastlogging::WriterConfigEnum) -> Self {
        match config {
            fastlogging::WriterConfigEnum::Root(_) => Self::Root,
            fastlogging::WriterConfigEnum::Console(_) => Self::Console,
            fastlogging::WriterConfigEnum::File(_) => Self::File,
            fastlogging::WriterConfigEnum::Client(_) => Self::Client,
            fastlogging::WriterConfigEnum::Server(_) => Self::Server,
            fastlogging::WriterConfigEnum::Callback(_) => Self::Callback,
            fastlogging::WriterConfigEnum::Syslog(_) => Self::Syslog,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WriterTypeEnumTyp {
    Root,
    Console,
    File,
    Files,
    Client,
    Clients,
    Server,
    Servers,
    Callback,
    Syslog,
}

#[repr(C)]
pub struct WriterTypeEnum {
    typ: WriterTypeEnumTyp,
    value: *const c_char,
}

impl WriterTypeEnum {
    pub fn c_into_rust(&self) -> fastlogging::WriterTypeEnum {
        match self.typ {
            WriterTypeEnumTyp::Root => fastlogging::WriterTypeEnum::Root,
            WriterTypeEnumTyp::Console => fastlogging::WriterTypeEnum::Console,
            WriterTypeEnumTyp::File => fastlogging::WriterTypeEnum::File(char2string(self.value)),
            WriterTypeEnumTyp::Files => fastlogging::WriterTypeEnum::Files,
            WriterTypeEnumTyp::Client => {
                fastlogging::WriterTypeEnum::Client(char2string(self.value))
            }
            WriterTypeEnumTyp::Clients => fastlogging::WriterTypeEnum::Clients,
            WriterTypeEnumTyp::Server => {
                fastlogging::WriterTypeEnum::Server(char2string(self.value))
            }
            WriterTypeEnumTyp::Servers => fastlogging::WriterTypeEnum::Servers,
            WriterTypeEnumTyp::Callback => fastlogging::WriterTypeEnum::Callback,
            WriterTypeEnumTyp::Syslog => fastlogging::WriterTypeEnum::Syslog,
        }
    }
}

impl From<fastlogging::WriterTypeEnum> for WriterTypeEnum {
    fn from(config: fastlogging::WriterTypeEnum) -> Self {
        match config {
            fastlogging::WriterTypeEnum::Root => Self {
                typ: WriterTypeEnumTyp::Root,
                value: null(),
            },
            fastlogging::WriterTypeEnum::Console => Self {
                typ: WriterTypeEnumTyp::Console,
                value: null(),
            },
            fastlogging::WriterTypeEnum::File(value) => Self {
                typ: WriterTypeEnumTyp::File,
                value: CString::new(value)
                    .expect("Error: CString::new()")
                    .into_raw(),
            },
            fastlogging::WriterTypeEnum::Files => Self {
                typ: WriterTypeEnumTyp::Files,
                value: null(),
            },
            fastlogging::WriterTypeEnum::Client(value) => Self {
                typ: WriterTypeEnumTyp::Client,
                value: CString::new(value)
                    .expect("Error: CString::new()")
                    .into_raw(),
            },
            fastlogging::WriterTypeEnum::Clients => Self {
                typ: WriterTypeEnumTyp::Clients,
                value: null(),
            },
            fastlogging::WriterTypeEnum::Server(value) => Self {
                typ: WriterTypeEnumTyp::Server,
                value: CString::new(value)
                    .expect("Error: CString::new()")
                    .into_raw(),
            },
            fastlogging::WriterTypeEnum::Servers => Self {
                typ: WriterTypeEnumTyp::Servers,
                value: null(),
            },
            fastlogging::WriterTypeEnum::Callback => Self {
                typ: WriterTypeEnumTyp::Callback,
                value: null(),
            },
            fastlogging::WriterTypeEnum::Syslog => Self {
                typ: WriterTypeEnumTyp::Syslog,
                value: null(),
            },
        }
    }
}

#[repr(C)]
pub struct WriterTypeEnums {
    cnt: c_uint,
    types: *const WriterTypeEnum,
}

/*#[repr(C)]
pub struct WriterEnum {
    typ: WriterEnumTyp,
    writer: *const c_void,
}

#[repr(C)]
pub struct WriterEnums {
    cnt: c_uint,
    values: *const WriterEnum,
}*/

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WriterConfigEnum {
    typ: WriterEnumTyp,
    config: *const c_void,
}

impl WriterConfigEnum {
    pub fn c_into_rust(&self) -> fastlogging::WriterConfigEnum {
        match self.typ {
            WriterEnumTyp::Root => fastlogging::WriterConfigEnum::Root(unsafe {
                *Box::from_raw(self.config as *mut _)
            }),
            WriterEnumTyp::Console => fastlogging::WriterConfigEnum::Console(unsafe {
                *Box::from_raw(self.config as *mut _)
            }),
            WriterEnumTyp::File => fastlogging::WriterConfigEnum::File(unsafe {
                *Box::from_raw(self.config as *mut _)
            }),
            WriterEnumTyp::Client => fastlogging::WriterConfigEnum::Client(unsafe {
                *Box::from_raw(self.config as *mut _)
            }),
            WriterEnumTyp::Server => fastlogging::WriterConfigEnum::Server(unsafe {
                *Box::from_raw(self.config as *mut _)
            }),
            WriterEnumTyp::Callback => fastlogging::WriterConfigEnum::Callback(unsafe {
                *Box::from_raw(self.config as *mut _)
            }),
            WriterEnumTyp::Syslog => fastlogging::WriterConfigEnum::Syslog(unsafe {
                *Box::from_raw(self.config as *mut _)
            }),
        }
    }
}

impl From<fastlogging::WriterConfigEnum> for WriterConfigEnum {
    fn from(config: fastlogging::WriterConfigEnum) -> Self {
        match config {
            fastlogging::WriterConfigEnum::Root(root_config) => Self {
                typ: WriterEnumTyp::Root,
                config: Box::into_raw(Box::new(root_config)) as *const c_void,
            },
            fastlogging::WriterConfigEnum::Console(console_writer_config) => Self {
                typ: WriterEnumTyp::Console,
                config: Box::into_raw(Box::new(console_writer_config)) as *const c_void,
            },
            fastlogging::WriterConfigEnum::File(file_writer_config) => Self {
                typ: WriterEnumTyp::File,
                config: Box::into_raw(Box::new(file_writer_config)) as *const c_void,
            },
            fastlogging::WriterConfigEnum::Client(client_writer_config) => Self {
                typ: WriterEnumTyp::Client,
                config: Box::into_raw(Box::new(client_writer_config)) as *const c_void,
            },
            fastlogging::WriterConfigEnum::Server(server_config) => Self {
                typ: WriterEnumTyp::Server,
                config: Box::into_raw(Box::new(server_config)) as *const c_void,
            },
            fastlogging::WriterConfigEnum::Callback(callback_writer_config) => Self {
                typ: WriterEnumTyp::Callback,
                config: Box::into_raw(Box::new(callback_writer_config)) as *const c_void,
            },
            fastlogging::WriterConfigEnum::Syslog(syslog_writer_config) => Self {
                typ: WriterEnumTyp::Syslog,
                config: Box::into_raw(Box::new(syslog_writer_config)) as *const c_void,
            },
        }
    }
}

#[repr(C)]
pub struct WriterConfigEnums {
    cnt: c_uint,
    wids: *const c_uint,
    configs: *const WriterConfigEnum,
}

#[repr(C)]
pub struct WriterEnums {
    cnt: c_uint,
    writers: *const WriterEnum,
}

#[inline]
fn cchar2vec(s: *const c_char) -> Vec<u8> {
    (unsafe { CStr::from_ptr(s) })
        .to_str()
        .unwrap()
        .as_bytes()
        .to_vec()
}

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

/// For further reading ...
/// [](https://internals.rust-lang.org/t/precise-semantics-of-no-mangle/4098)

/// # Safety
///
/// Create new logging instance.
#[no_mangle]
pub unsafe extern "C" fn logging_init_root() {
    fastlogging::logging_init_root();
}

/// # Safety
///
/// Create new logging instance.
#[no_mangle]
pub unsafe extern "C" fn logging_new_default() -> *mut Logging {
    Box::into_raw(Box::new(fastlogging::logging_new_default().unwrap()))
}

/// # Safety
///
/// Create new logging instance.
#[no_mangle]
pub unsafe extern "C" fn logging_new(
    level: c_char, // Global log level
    domain: *const c_char,
    configs: *mut WriterConfigEnums, // This is a Vec<WriterConfigEnum>
    ext_config: *mut ExtConfig,
    config_path: *const c_char, // Optional path to config file
) -> *mut Logging {
    let domain = if domain.is_null() {
        "root".to_string()
    } else {
        char2string(domain)
    };
    let configs = if configs.is_null() {
        Vec::new()
    } else {
        let configs = &mut *configs;
        let configs_configs: &[WriterConfigEnum] =
            slice::from_raw_parts(configs.configs as *mut _, configs.cnt as usize);
        println!("*****");
        println!("configs.cnt={}", configs.cnt);
        println!("configs.configs={:p}", configs.configs);
        configs_configs.to_vec()
    };
    let ext_config = if ext_config.is_null() {
        None
    } else {
        Some(*Box::from_raw(ext_config))
    };
    let config_path = if config_path.is_null() {
        None
    } else {
        Some(PathBuf::from(char2string(config_path)))
    };
    let logging = Logging::new(level as u8, domain, configs, ext_config, config_path).unwrap();
    Box::into_raw(Box::new(logging))
}

/// # Safety
///
/// Shutdown logging.
#[no_mangle]
pub unsafe extern "C" fn logging_apply_config(logging: &mut Logging, path: *const c_char) -> isize {
    let path = PathBuf::from(char2string(path));
    let result = if let Err(err) = logging.apply_config(&path) {
        eprintln!("logging_apply_config failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    };
    if logging.drop {
        drop(Box::from_raw(logging));
    }
    result
}

/// # Safety
///
/// Shutdown logging.
#[no_mangle]
pub unsafe extern "C" fn logging_shutdown(logging: &mut Logging, now: i8) -> isize {
    let result = if let Err(err) = logging.shutdown(now != 0) {
        eprintln!("logging_shutdown failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    };
    if logging.drop {
        drop(Box::from_raw(logging));
    }
    result
}

/// # Safety
///
/// Set logging level.
#[no_mangle]
pub unsafe extern "C" fn logging_set_level(logging: &mut Logging, wid: c_uint, level: u8) -> isize {
    if let Err(err) = logging.set_level(wid as usize, level) {
        eprintln!("logging_set_level failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// Set logging domain.
#[no_mangle]
pub unsafe extern "C" fn logging_set_domain(logging: &mut Logging, domain: *const c_char) {
    logging.set_domain(&char2string(domain));
}

/// # Safety
///
/// Set log level symbols.
#[no_mangle]
pub unsafe extern "C" fn logging_set_level2sym(logging: &mut Logging, level2sym: u8) {
    logging.set_level2sym(if level2sym == 0 {
        &LevelSyms::Sym
    } else if level2sym == 1 {
        &LevelSyms::Short
    } else {
        &LevelSyms::Str
    });
}

/// # Safety
///
/// Set extended configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_set_ext_config(logging: &mut Logging, ext_config: &ExtConfig) {
    logging.set_ext_config(ext_config);
}

/// # Safety
///
/// Add logger.
#[no_mangle]
pub unsafe extern "C" fn logging_add_logger(logging: &mut Logging, logger: &mut Logger) {
    logging.add_logger(logger);
}

/// # Safety
///
/// Remove logger.
#[no_mangle]
pub unsafe extern "C" fn logging_remove_logger(logging: &mut Logging, logger: &mut Logger) {
    logging.remove_logger(logger);
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn logging_set_root_writer_config(
    logging: &mut Logging,
    config: *mut WriterConfigEnum,
) -> isize {
    let config = *Box::from_raw(config);
    match logging.set_root_writer_config(&config.c_into_rust()) {
        Ok(_r) => 0,
        Err(err) => {
            eprintln!("logging_set_root_writer_config failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn logging_set_root_writer(
    logging: &mut Logging,
    writer: *mut WriterEnum,
) -> isize {
    match logging.set_root_writer(*Box::from_raw(writer)) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("logging_set_root_writer failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn logging_add_writer_config(
    logging: &mut Logging,
    config: *mut WriterConfigEnum,
) -> isize {
    let config = *Box::from_raw(config);
    match logging.add_writer_config(&config) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("logging_add_writer_config failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn logging_add_writer(
    logging: &mut Logging,
    writer: *mut WriterEnum,
) -> usize {
    logging.add_writer(*Box::from_raw(writer))
}

/// # Safety
///
/// Remove writer.
#[no_mangle]
pub unsafe extern "C" fn logging_remove_writer(
    logging: &mut Logging,
    wid: usize,
) -> *const WriterEnum {
    match logging.remove_writer(wid) {
        Some(w) => Box::into_raw(Box::new(w)),
        None => null(),
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn logging_add_writer_configs(
    logging: &mut Logging,
    configs: *mut WriterConfigEnums,
) -> isize {
    let configs = *Box::from_raw(configs);
    let configs: Vec<WriterConfigEnum> = Vec::from_raw_parts(
        configs.configs as *mut _,
        configs.cnt as usize,
        configs.cnt as usize,
    );
    let configs: Vec<fastlogging::WriterConfigEnum> = configs
        .into_iter()
        .map(|w| w.c_into_rust())
        .collect::<Vec<_>>();

    match logging.add_writer_configs(configs.as_slice()) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("logging_add_writer_configs failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn logging_add_writers(
    logging: &mut Logging,
    writers: *mut WriterEnums,
) -> *mut CusizeVec {
    let writers = *Box::from_raw(writers);
    let wids = logging.add_writers(Vec::from_raw_parts(
        writers.writers as *mut _,
        writers.cnt as usize,
        writers.cnt as usize,
    ));
    Box::into_raw(Box::new(CusizeVec {
        cnt: wids.len() as u32,
        values: wids.as_ptr() as *const u32,
    }))
}

/// # Safety
///
/// Remove writers.
#[no_mangle]
pub unsafe extern "C" fn logging_remove_writers(
    logging: &mut Logging,
    wids: *mut c_uint,
    wid_cnt: c_uint,
) -> *mut WriterConfigEnums {
    let wids = Vec::from_raw_parts(wids as *mut usize, wid_cnt as usize, wid_cnt as usize);
    let writers = logging.remove_writers(Some(wids.clone()));
    let configs = writers
        .into_iter()
        .map(|w| WriterConfigEnum {
            typ: WriterEnumTyp::from(w.config().clone()),
            config: writer_config_as_void(w.config()),
        })
        .collect::<Vec<_>>();
    let result = Box::into_raw(Box::new(WriterConfigEnums {
        cnt: configs.len() as u32,
        wids: wids.as_ptr() as *const u32,
        configs: configs.as_ptr(),
    }));
    std::mem::forget(wids);
    std::mem::forget(configs);
    result
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn logging_enable(logging: &mut Logging, wid: usize) -> isize {
    match logging.enable(wid) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("logging_enable failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn logging_disable(logging: &mut Logging, wid: usize) -> isize {
    match logging.disable(wid) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("logging_disable failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn logging_enable_type(
    logging: &mut Logging,
    typ: *mut WriterTypeEnum,
) -> isize {
    let typ = *Box::from_raw(typ);
    match logging.enable_type(typ.c_into_rust()) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("logging_enable failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn logging_disable_type(
    logging: &mut Logging,
    typ: *mut WriterTypeEnum,
) -> isize {
    let typ = *Box::from_raw(typ);
    match logging.disable_type(typ.c_into_rust()) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("logging_disable_type failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Sync specific writers.
#[no_mangle]
pub unsafe extern "C" fn logging_sync(
    logging: &Logging,
    types: *mut WriterTypeEnums,
    timeout: c_double,
) -> isize {
    let types = *Box::from_raw(types);
    let types = Vec::from_raw_parts(
        types.types as *mut _,
        types.cnt as usize,
        types.cnt as usize,
    );
    if let Err(err) = logging.sync(types, timeout) {
        eprintln!("logging_sync failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// Sync all writers.
#[no_mangle]
pub unsafe extern "C" fn logging_sync_all(logging: &Logging, timeout: c_double) -> isize {
    if let Err(err) = logging.sync_all(timeout) {
        eprintln!("logging_sync_all failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

// File writer

/// # Safety
///
/// Rotate file.
#[no_mangle]
pub unsafe extern "C" fn logging_rotate(logging: &Logging, path: *mut PathBuf) -> isize {
    let path = if path.is_null() {
        None
    } else {
        Some(*Box::from_raw(path))
    };
    if let Err(err) = logging.rotate(path) {
        eprintln!("logging_rotate failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

// Network

/// # Safety
///
/// Set encryption.
#[no_mangle]
pub unsafe extern "C" fn logging_set_encryption(
    logging: &mut Logging,
    wid: c_uint,
    encryption: c_uchar,
    key: *const c_char,
) -> isize {
    let key = if encryption == 0 || key.is_null() {
        EncryptionMethod::NONE
    } else if encryption == 1 {
        EncryptionMethod::AuthKey(cchar2vec(key))
    } else {
        EncryptionMethod::AES(cchar2vec(key))
    };
    if let Err(err) = logging.set_encryption(wid as usize, key) {
        eprintln!("logging_set_encryption failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

// Config

/// # Safety
///
/// Set debug level.
#[no_mangle]
pub unsafe extern "C" fn logging_set_debug(logging: &mut Logging, debug: u8) {
    logging.set_debug(debug);
}

/// # Safety
///
/// Get configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_writer_config(
    logging: &Logging,
    wid: c_uint,
) -> *const WriterConfigEnum {
    match logging.get_writer_config(wid as usize) {
        Some(config) => Box::into_raw(Box::new(WriterConfigEnum {
            typ: WriterEnumTyp::from(config.clone()),
            config: writer_config_as_void(config),
        })),
        None => null(),
    }
}

/// # Safety
///
/// Get configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_writer_configs(logging: &Logging) -> *const WriterConfigEnums {
    let mut wids = Vec::new();
    let mut configs = Vec::new();
    for (k, v) in logging.get_writer_configs().into_iter() {
        wids.push(k);
        configs.push(WriterConfigEnum {
            typ: WriterEnumTyp::from(v.clone()),
            config: writer_config_as_void(v),
        });
    }
    let results = WriterConfigEnums {
        cnt: wids.len() as u32,
        wids: wids.as_ptr() as *const u32,
        configs: configs.as_ptr(),
    };
    std::mem::forget(wids);
    std::mem::forget(configs);
    Box::into_raw(Box::new(results))
}

/// # Safety
///
/// Get server configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_server_config(
    logging: &Logging,
    wid: usize,
) -> *mut ServerConfig {
    match logging.get_server_config(wid) {
        Ok(c) => {
            let key = KeyStruct {
                typ: EncryptionMethodEnum::from(&c.key),
                len: c.key.len() as c_uint,
                key: Box::into_raw(Box::new(c.key.key_cloned())) as *const u8,
            };
            Box::into_raw(Box::new(ServerConfig {
                level: c.level,
                address: CString::new(c.address)
                    .expect("Error: CString::new()")
                    .into_raw(),
                port: c.port,
                key: Box::into_raw(Box::new(key)),
            }))
        }
        Err(_err) => null::<ServerConfig>() as *mut _,
    }
}

/// # Safety
///
/// Get configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_server_configs(logging: &Logging) -> *const ServerConfigs {
    let mut keys = Vec::new();
    let mut values = Vec::new();
    for (k, v) in logging.get_server_configs().into_iter() {
        println!("k={k}");
        println!("v={v:?}");
        keys.push(k as u32);
        let key = KeyStruct {
            typ: EncryptionMethodEnum::from(&v.key),
            len: v.key.len() as c_uint,
            key: Box::into_raw(Box::new(v.key.key_cloned())) as *const u8,
        };
        let config = ServerConfig {
            level: v.level,
            address: CString::new(v.address)
                .expect("Error: CString::new()")
                .into_raw() as *const c_char,
            port: v.port,
            key: Box::into_raw(Box::new(key)),
        };
        values.push(config);
    }
    let configs = ServerConfigs {
        cnt: keys.len() as u32,
        keys: keys.as_ptr(),
        values: values.as_ptr(),
    };
    std::mem::forget(keys);
    std::mem::forget(values);
    Box::into_raw(Box::new(configs))
}

/// # Safety
///
/// Get server configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_root_server_address_port(logging: &Logging) -> *const c_char {
    match logging.get_root_server_address_port() {
        Some(s) => CString::new(s).expect("Error: CString::new()").into_raw() as *const c_char,
        None => null(),
    }
}

/// # Safety
///
/// Get server configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_server_addresses_ports(
    logging: &Logging,
) -> *const Cu32StringVec {
    Box::into_raw(Box::new(Cu32StringVec::from(
        logging.get_server_addresses_ports(),
    )))
}

/// # Safety
///
/// Get server configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_server_addresses(logging: &Logging) -> *const Cu32StringVec {
    Box::into_raw(Box::new(Cu32StringVec::from(
        logging.get_server_addresses(),
    )))
}

/// # Safety
///
/// Get server configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_server_ports(logging: &Logging) -> *const Cu32u16Vec {
    Box::into_raw(Box::new(Cu32u16Vec::from(logging.get_server_ports())))
}

/// # Safety
///
/// Get server authentification key.
#[no_mangle]
pub unsafe extern "C" fn logging_get_server_auth_key(logging: &Logging) -> *mut KeyStruct {
    let mut key = logging.get_server_auth_key().key_cloned().unwrap();
    key.shrink_to_fit();
    let c_key = KeyStruct {
        typ: EncryptionMethodEnum::AuthKey,
        len: key.len() as c_uint,
        key: key.as_ptr(),
    };
    std::mem::forget(key);
    Box::into_raw(Box::new(c_key))
}

/// # Safety
///
/// Get configuration as string.
#[no_mangle]
pub unsafe extern "C" fn logging_get_config_string(logging: &Logging) -> *const c_char {
    logging.get_config_string().as_ptr() as *const c_char
}

/// # Safety
///
/// Save configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_save_config(logging: &mut Logging, path: *const c_char) -> isize {
    let path = if path.is_null() {
        None
    } else {
        Some(PathBuf::from(char2string(path)))
    };
    if let Err(err) = logging.save_config(path.as_deref()) {
        eprintln!("logging_get_server_config failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

// Logging calls

/// # Safety
///
/// trace message.
#[no_mangle]
pub unsafe extern "C" fn logging_trace(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.trace(char2string(message)) {
        eprintln!("logging_trace failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// debug message.
#[no_mangle]
pub unsafe extern "C" fn logging_debug(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.debug(char2string(message)) {
        eprintln!("logging_debug failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// info message.
#[no_mangle]
pub unsafe extern "C" fn logging_info(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.info(char2string(message)) {
        eprintln!("logging_info failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// success message.
#[no_mangle]
pub unsafe extern "C" fn logging_success(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.success(char2string(message)) {
        eprintln!("logging_success failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// warning message.
#[no_mangle]
pub unsafe extern "C" fn logging_warning(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.warning(char2string(message)) {
        eprintln!("logging_warning failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// error message.
#[no_mangle]
pub unsafe extern "C" fn logging_error(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.error(char2string(message)) {
        eprintln!("logging_error failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// critical message.
#[no_mangle]
pub unsafe extern "C" fn logging_critical(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.critical(char2string(message)) {
        eprintln!("logging_critical failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// fatal message.
#[no_mangle]
pub unsafe extern "C" fn logging_fatal(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.fatal(char2string(message)) {
        eprintln!("logging_fatal failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// exception message.
#[no_mangle]
pub unsafe extern "C" fn logging_exception(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.exception(char2string(message)) {
        eprintln!("logging_exception failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}
