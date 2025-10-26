use core::slice;
use std::ffi::{CString, c_char, c_double, c_uint};
use std::path::PathBuf;
use std::ptr::null;

use fastlogging::{
    EncryptionMethod, ExtConfig, LevelSyms, Logger, Logging, WriterConfigEnum, WriterEnum,
    WriterTypeEnum,
};

use crate::def::{
    CServerConfig, CServerConfigs, CWriterConfigEnums, CWriterEnum, CWriterEnums, Cu32StringVec,
    Cu32u16Vec, CusizeVec,
};
use crate::util::char2string;
use crate::{CEncryptionMethodEnum, CKeyStruct};

/// For further reading ...
/// [](https://internals.rust-lang.org/t/precise-semantics-of-no-mangle/4098)

/// # Safety
///
/// Create new logging instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_new_default() -> *mut Logging {
    Box::into_raw(Box::new(fastlogging::logging_new_default().unwrap()))
}

/// # Safety
///
/// Create new logging instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_new(
    level: c_char, // Global log level
    domain: *const c_char,
    configs: *mut *mut WriterConfigEnum,
    config_count: usize,
    ext_config: *mut ExtConfig,
    config_path: *const c_char, // Optional path to config file
) -> *mut Logging {
    let domain = if domain.is_null() {
        "root".to_string()
    } else {
        char2string(domain)
    };
    let configs = if configs.is_null() {
        None
    } else {
        let config_ptrs = unsafe { std::slice::from_raw_parts(configs, config_count) };
        let config_vec: Vec<WriterConfigEnum> = config_ptrs
            .iter()
            .map(|&ptr| unsafe { *Box::from_raw(ptr) })
            .collect();
        Some(config_vec)
    };
    let ext_config = if ext_config.is_null() {
        None
    } else {
        Some(unsafe { *Box::from_raw(ext_config) })
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_apply_config(logging: &mut Logging, path: *const c_char) -> isize {
    let path = PathBuf::from(char2string(path));
    let result = if let Err(err) = logging.apply_config(&path) {
        eprintln!("logging_apply_config failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    };
    if logging.drop {
        drop(unsafe { Box::from_raw(logging) });
    }
    result
}

/// # Safety
///
/// Shutdown logging.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_shutdown(logging: &mut Logging, now: i8) -> isize {
    let result = if let Err(err) = logging.shutdown(now != 0) {
        eprintln!("logging_shutdown failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    };
    if logging.drop {
        drop(unsafe { Box::from_raw(logging) });
    }
    result
}

/// # Safety
///
/// Set logging level.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_set_domain(logging: &mut Logging, domain: *const c_char) {
    logging.set_domain(&char2string(domain));
}

/// # Safety
///
/// Set log level symbols.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_set_ext_config(logging: &mut Logging, ext_config: &ExtConfig) {
    logging.set_ext_config(ext_config);
}

/// # Safety
///
/// Add logger.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_add_logger(logging: &mut Logging, logger: &mut Logger) {
    logging.add_logger(logger);
}

/// # Safety
///
/// Remove logger.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_remove_logger(logging: &mut Logging, logger: &mut Logger) {
    logging.remove_logger(logger);
}

/// # Safety
///
/// Add writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_set_root_writer_config(
    logging: &mut Logging,
    config: *mut WriterConfigEnum,
) -> isize {
    unsafe {
        match logging.set_root_writer_config(&Box::from_raw(config)) {
            Ok(_r) => 0,
            Err(err) => {
                eprintln!("logging_set_root_writer_config failed: {err:?}");
                err.as_int() as isize
            }
        }
    }
}

/// # Safety
///
/// Add writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_set_root_writer(
    logging: &mut Logging,
    writer: *mut WriterEnum,
) -> isize {
    unsafe {
        match logging.set_root_writer(*Box::from_raw(writer)) {
            Ok(r) => Box::into_raw(Box::new(r)) as isize,
            Err(err) => {
                eprintln!("logging_set_root_writer failed: {err:?}");
                err.as_int() as isize
            }
        }
    }
}

/// # Safety
///
/// Add writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_add_writer_config(
    logging: &mut Logging,
    config: *mut WriterConfigEnum,
) -> isize {
    let config = unsafe { *Box::from_raw(config) };
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_add_writer(
    logging: &mut Logging,
    writer: *mut WriterEnum,
) -> usize {
    logging.add_writer(unsafe { *Box::from_raw(writer) })
}

/// # Safety
///
/// Remove writer.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_add_writer_configs(
    logging: &mut Logging,
    configs_ptr: *const *mut Vec<WriterConfigEnum>,
) -> isize {
    let configs: Box<Vec<WriterConfigEnum>> =
        unsafe { Box::from_raw(configs_ptr as *mut Vec<WriterConfigEnum>) };
    match logging.add_writer_configs(*configs) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_add_writers(
    logging: &mut Logging,
    writers_ptr: *mut Vec<WriterEnum>,
) -> *mut CusizeVec {
    let writers = unsafe { Box::from_raw(writers_ptr) };
    let wids = logging.add_writers(*writers);
    Box::into_raw(Box::new(CusizeVec {
        cnt: wids.len() as u32,
        values: wids,
    }))
}

/// # Safety
///
/// Remove writers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_remove_writers(
    logging: &mut Logging,
    wids: *mut u32,
    wid_cnt: u32,
) -> *mut CWriterEnums {
    let wids: Option<&[u32]> = if wids as *const _ != null() {
        Some(unsafe { slice::from_raw_parts(wids, wid_cnt as usize) })
    } else {
        None
    };
    let wids = wids.map(|w| w.iter().map(|w| *w as usize).collect::<Vec<usize>>());
    let writers = logging.remove_writers(wids);
    let writers = writers
        .into_iter()
        .map(|w| w.into())
        .collect::<Vec<CWriterEnum>>();
    Box::into_raw(Box::new(CWriterEnums {
        cnt: writers.len() as u32,
        values: Box::into_raw(Box::new(writers)) as *const CWriterEnum,
    }))
}

/// # Safety
///
/// Add writer.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_enable_type(
    logging: &mut Logging,
    typ: *mut WriterTypeEnum,
) -> isize {
    match logging.enable_type(unsafe { *Box::from_raw(typ) }) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_disable_type(
    logging: &mut Logging,
    typ: *mut WriterTypeEnum,
) -> isize {
    match logging.disable_type(unsafe { *Box::from_raw(typ) }) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_sync(
    logging: &Logging,
    types: *mut Vec<WriterTypeEnum>,
    timeout: c_double,
) -> isize {
    let types: Box<Vec<WriterTypeEnum>> = unsafe { Box::from_raw(types) };
    if let Err(err) = logging.sync(*types, timeout) {
        eprintln!("logging_sync failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// Sync all writers.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_rotate(logging: &Logging, path: *mut PathBuf) -> isize {
    let path = if path.is_null() {
        None
    } else {
        Some(unsafe { *Box::from_raw(path) })
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_set_encryption(
    logging: &mut Logging,
    wid: c_uint,
    key: *mut CKeyStruct,
) -> isize {
    let key = if key.is_null() {
        EncryptionMethod::NONE
    } else {
        let c_key = unsafe { *Box::from_raw(key) };
        let key = unsafe { slice::from_raw_parts(c_key.key, c_key.len as usize) }.to_vec();
        if c_key.typ == CEncryptionMethodEnum::AuthKey {
            EncryptionMethod::AuthKey(key)
        } else {
            EncryptionMethod::AES(key)
        }
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
/// Get configuration.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_get_writer_configs(
    logging: &Logging,
) -> *const CWriterConfigEnums {
    let mut configs = CWriterConfigEnums {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_get_server_config(
    logging: &Logging,
    wid: usize,
) -> *mut CServerConfig {
    match logging.get_server_config(wid) {
        Ok(c) => Box::into_raw(Box::new(c.into())),
        Err(_err) => null::<CServerConfig>() as *mut _,
    }
}

/// # Safety
///
/// Get configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_get_server_configs(logging: &Logging) -> *const CServerConfigs {
    let mut keys: Vec<u32> = Vec::new();
    let mut values: Vec<CServerConfig> = Vec::new();
    let configs = logging.get_server_configs();
    for (k, c) in configs.into_iter() {
        keys.push(k as u32);
        values.push(c.into());
    }
    let configs = CServerConfigs {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_get_root_server_address_port(logging: &Logging) -> *const char {
    match logging.get_root_server_address_port() {
        Some(s) => CString::new(s).expect("Error: CString::new()").into_raw() as *const char,
        None => null(),
    }
}

/// # Safety
///
/// Get server configuration.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_get_server_addresses(logging: &Logging) -> *const Cu32StringVec {
    Box::into_raw(Box::new(Cu32StringVec::from(
        logging.get_server_addresses(),
    )))
}

/// # Safety
///
/// Get server configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_get_server_ports(logging: &Logging) -> *const Cu32u16Vec {
    Box::into_raw(Box::new(Cu32u16Vec::from(logging.get_server_ports())))
}

/// # Safety
///
/// Get server authentification key.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_get_server_auth_key(logging: &Logging) -> *mut CKeyStruct {
    let mut key = logging.get_server_auth_key().key_cloned().unwrap();
    key.shrink_to_fit();
    let c_key = CKeyStruct {
        typ: CEncryptionMethodEnum::AuthKey,
        len: key.len() as c_uint,
        key: key.as_ptr(),
    };
    std::mem::forget(key);
    Box::into_raw(Box::new(c_key))
}

/// # Safety
///
/// Get configuration as string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_get_config_string(logging: &Logging) -> *const c_char {
    let config = logging.get_config_string();
    let ptr = config.as_ptr() as *const c_char;
    std::mem::forget(config);
    ptr
}

/// # Safety
///
/// Save configuration.
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_exception(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.exception(char2string(message)) {
        eprintln!("logging_exception failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// Set debug level.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logging_set_debug(logging: &mut Logging, debug: u8) {
    logging.set_debug(debug);
}
