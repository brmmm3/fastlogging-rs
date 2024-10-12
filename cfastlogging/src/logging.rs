use core::slice;
use std::ffi::{c_char, c_double, c_uchar, c_uint, CStr, CString};
use std::path::PathBuf;
use std::ptr::null;

use fastlogging::{
    EncryptionMethod, ExtConfig, LevelSyms, Logger, Logging, MessageStructEnum, ServerConfig,
    WriterConfigEnum, WriterEnum, WriterTypeEnum,
};

use crate::util::{char2string, option_char2string};

#[repr(C)]
pub struct CServerConfig {
    level: u8,
    address: *const c_char,
    port: u16,
    key: EncryptionMethod,
}

#[repr(C)]
pub struct CusizeVec {
    cnt: c_uint,
    values: Vec<usize>,
}

#[repr(C)]
pub struct CWriterEnumVec {
    cnt: c_uint,
    values: Vec<WriterEnum>,
}

#[repr(C)]
pub struct CWriterConfigEnumHashMap {
    cnt: c_uint,
    keys: Vec<usize>,
    values: Vec<WriterConfigEnum>,
}

#[repr(C)]
pub struct CServerConfigHashMap {
    cnt: c_uint,
    keys: Vec<usize>,
    values: Vec<ServerConfig>,
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
) -> *mut ExtConfig {
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
pub unsafe extern "C" fn logging_init() -> *mut Logging {
    Box::into_raw(Box::new(fastlogging::logging_init().unwrap()))
}

/// # Safety
///
/// Create new logging instance.
#[no_mangle]
pub unsafe extern "C" fn logging_new(
    level: c_char, // Global log level
    domain: *const c_char,
    writers_ptr: *mut *mut WriterConfigEnum, // This is a Vec<WriterConfigEnum>
    writers_cnt: c_uint,
    ext_config: *mut ExtConfig,
    config_path: *const c_char, // Optional path to config file
) -> *mut Logging {
    println!("logging_new level={level} domain={domain:p} writers_ptr={writers_ptr:p} writers_cnt={writers_cnt} ext_config={ext_config:p} config={config_path:p}");
    let domain = if domain.is_null() {
        "root".to_string()
    } else {
        char2string(domain)
    };
    println!("domain={domain}");
    let writers = if writers_ptr.is_null() {
        Vec::new()
    } else {
        let writers = slice::from_raw_parts(writers_ptr, writers_cnt as usize);
        writers
            .into_iter()
            .map(|w| *Box::from_raw(*w))
            .collect::<Vec<_>>()
    };
    println!("writers={:p}", &writers);
    println!("writers={writers:?}");
    let ext_config = if ext_config.is_null() {
        None
    } else {
        Some(*Box::from_raw(ext_config))
    };
    println!("ext_config={:p}", &ext_config);
    println!("ext_config={ext_config:?}");
    Box::into_raw(Box::new(
        Logging::new(
            level as u8,
            domain,
            writers,
            ext_config,
            option_char2string(config_path).map(PathBuf::from),
        )
        .unwrap(),
    ))
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
pub unsafe extern "C" fn logging_shutdown(logging: &mut Logging, now: u8) -> isize {
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
    println!("logging_set_root_writer_config {config:p}");
    let config = *Box::from_raw(config);
    println!("logging_set_root_writer_config #");
    println!("logging_set_root_writer_config {:p}", &config);
    match logging.set_root_writer_config(&config) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
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
    println!("logging_set_root_writer {writer:p}");
    let writer = *Box::from_raw(writer);
    println!("logging_set_root_writer #");
    println!("logging_set_root_writer {:p}", &writer);
    match logging.set_root_writer(writer) {
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
    println!("logging_add_writer_config {config:p}");
    let config = *Box::from_raw(config);
    println!("logging_add_writer_config #");
    println!("logging_add_writer_config {:p}", &config);
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
    println!("logging_add_writer {writer:p}");
    let writer = *Box::from_raw(writer);
    println!("logging_add_writer #");
    println!("logging_add_writer {:p}", &writer);
    logging.add_writer(writer)
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
    configs: *mut WriterConfigEnum,
    config_cnt: usize,
) -> isize {
    println!("logging_add_writer_configs {configs:p}");
    let configs = slice::from_raw_parts(configs, config_cnt);
    println!("logging_add_writer_configs #");
    println!("logging_add_writer_configs {:p}", configs);
    match logging.add_writer_configs(configs) {
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
    writers: *mut WriterEnum,
    writer_cnt: usize,
) -> *mut CusizeVec {
    println!("logging_add_writers {writers:p}");
    let writers = Vec::from_raw_parts(writers, writer_cnt, writer_cnt);
    println!("logging_add_writers #");
    println!("logging_add_writers {:p}", &writers);
    let wids = logging.add_writers(writers);
    Box::into_raw(Box::new(CusizeVec {
        cnt: wids.len() as u32,
        values: wids,
    }))
}

/// # Safety
///
/// Remove writers.
#[no_mangle]
pub unsafe extern "C" fn logging_remove_writers(
    logging: &mut Logging,
    wids: *mut usize,
    wid_cnt: usize,
) -> *mut CWriterEnumVec {
    let v = Vec::from_raw_parts(wids, wid_cnt, wid_cnt);
    println!("logging_remove_writers {wids:p}");
    println!("logging_remove_writers #");
    let writers = logging.remove_writers(v);
    Box::into_raw(Box::new(CWriterEnumVec {
        cnt: writers.len() as u32,
        values: writers,
    }))
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn logging_enable(logging: &mut Logging, wid: usize) -> isize {
    println!("logging_enable {wid}");
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
    println!("logging_disable {wid}");
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
    println!("logging_enable_type {:p}", &typ);
    match logging.enable_type(*Box::from_raw(typ)) {
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
    println!("logging_disable_type {:p}", &typ);
    match logging.disable_type(*Box::from_raw(typ)) {
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
    types: *mut WriterTypeEnum,
    type_cnt: c_uint,
    timeout: c_double,
) -> isize {
    let types = Vec::from_raw_parts(types, type_cnt as usize, type_cnt as usize);
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
        Some(config) => &config,
        None => null(),
    }
}

/// # Safety
///
/// Get configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_writer_configs(
    logging: &Logging,
) -> *const CWriterConfigEnumHashMap {
    let mut configs = CWriterConfigEnumHashMap {
        cnt: 0,
        keys: Vec::new(),
        values: Vec::new(),
    };
    for (k, v) in logging.get_writer_configs().into_iter() {
        configs.keys.push(k);
        configs.values.push(v);
    }
    configs.cnt = configs.keys.len() as u32;
    Box::into_raw(Box::new(configs))
}

/// # Safety
///
/// Get server configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_server_config(
    logging: &Logging,
    wid: usize,
) -> *mut CServerConfig {
    match logging.get_server_config(wid) {
        Ok(c) => Box::into_raw(Box::new(CServerConfig {
            level: c.level,
            address: CString::new(c.address)
                .expect("Error: CString::new()")
                .into_raw(),
            port: c.port,
            key: c.key,
        })),
        Err(_err) => null::<CServerConfig>() as *mut _,
    }
}

/// # Safety
///
/// Get configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_server_configs(
    logging: &Logging,
) -> *const CServerConfigHashMap {
    let mut configs = CServerConfigHashMap {
        cnt: 0,
        keys: Vec::new(),
        values: Vec::new(),
    };
    for (k, v) in logging.get_server_configs().into_iter() {
        configs.keys.push(k);
        configs.values.push(v);
    }
    configs.cnt = configs.keys.len() as u32;
    Box::into_raw(Box::new(configs))
}

/// # Safety
///
/// Get server configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_root_server_address_port(logging: &Logging) -> *const char {
    match logging.get_root_server_address_port() {
        Some(s) => CString::new(s).expect("Error: CString::new()").into_raw() as *const char,
        None => null(),
    }
}

// TODO: get_server_addresses_ports, get_server_addresses, get_server_ports

/// # Safety
///
/// Get server authentification key.
#[no_mangle]
pub unsafe extern "C" fn logging_get_server_auth_key(logging: &Logging) -> *const c_char {
    logging.get_server_auth_key().key_cloned().unwrap().as_ptr() as *const c_char
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
