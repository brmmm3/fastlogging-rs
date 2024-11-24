use std::{
    ffi::{c_char, c_double, c_uint, CString},
    path::PathBuf,
    ptr::null,
    slice,
};

use fastlogging::{
    root, EncryptionMethod, ExtConfig, LevelSyms, Logger, WriterConfigEnum, WriterEnum,
    WriterTypeEnum,
};

use crate::{
    util::char2string, CEncryptionMethodEnum, CKeyStruct, CServerConfig, CServerConfigs,
    CWriterConfigEnums, CWriterEnum, CWriterEnums, Cu32StringVec, Cu32u16Vec, CusizeVec,
};

/// # Safety
///
/// Create new logging instance.
#[no_mangle]
pub unsafe extern "C" fn root_init() {
    root::root_init();
}

/// # Safety
///
/// Shutdown fastroot::
#[no_mangle]
pub unsafe extern "C" fn root_shutdown(now: bool) -> isize {
    if let Err(err) = root::shutdown(now) {
        eprintln!("shutdown failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// Set logging level.
#[no_mangle]
pub unsafe extern "C" fn root_set_level(wid: c_uint, level: u8) -> isize {
    if let Err(err) = root::set_level(wid as usize, level) {
        eprintln!("set_level failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// Set logging domain.
#[no_mangle]
pub unsafe extern "C" fn root_set_domain(domain: *const c_char) {
    root::set_domain(&char2string(domain));
}

/// # Safety
///
/// Set log level symbols.
#[no_mangle]
pub unsafe extern "C" fn root_set_level2sym(level2sym: u8) {
    root::set_level2sym(if level2sym == 0 {
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
pub unsafe extern "C" fn root_set_ext_config(ext_config: &ExtConfig) {
    root::set_ext_config(ext_config);
}

/// # Safety
///
/// Add logger.
#[no_mangle]
pub unsafe extern "C" fn root_add_logger(logger: &mut Logger) {
    root::add_logger(logger);
}

/// # Safety
///
/// Remove logger.
#[no_mangle]
pub unsafe extern "C" fn root_remove_logger(logger: &mut Logger) {
    root::remove_logger(logger);
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn root_set_root_writer_config(config: *mut WriterConfigEnum) -> isize {
    match root::set_root_writer_config(&*Box::from_raw(config)) {
        Ok(_r) => 0,
        Err(err) => {
            eprintln!("set_root_writer_config failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn root_set_root_writer(writer: *mut WriterEnum) -> isize {
    match root::set_root_writer(*Box::from_raw(writer)) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("set_root_writer failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn root_add_writer_config(config: *mut WriterConfigEnum) -> isize {
    let config = *Box::from_raw(config);
    match root::add_writer_config(&config) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("add_writer_config failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn root_add_writer(writer: *mut WriterEnum) -> usize {
    root::add_writer(*Box::from_raw(writer))
}

/// # Safety
///
/// Remove writer.
#[no_mangle]
pub unsafe extern "C" fn root_remove_writer(wid: usize) -> *const WriterEnum {
    match root::remove_writer(wid) {
        Some(w) => Box::into_raw(Box::new(w)),
        None => null(),
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn root_add_writer_configs(
    configs: *mut WriterConfigEnum,
    config_cnt: usize,
) -> isize {
    match root::add_writer_configs(slice::from_raw_parts(configs, config_cnt)) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("add_writer_configs failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn root_add_writers(
    writers: *mut WriterEnum,
    writer_cnt: usize,
) -> *mut CusizeVec {
    let wids = root::add_writers(Vec::from_raw_parts(writers, writer_cnt, writer_cnt));
    Box::into_raw(Box::new(CusizeVec {
        cnt: wids.len() as u32,
        values: wids,
    }))
}

/// # Safety
///
/// Remove writers.
#[no_mangle]
pub unsafe extern "C" fn root_remove_writers(wids: *mut u32, wid_cnt: u32) -> *mut CWriterEnums {
    let wids: Option<&[u32]> = if wids as *const _ != null() {
        Some(slice::from_raw_parts(wids, wid_cnt as usize))
    } else {
        None
    };
    let wids = wids.map(|w| w.into_iter().map(|w| *w as usize).collect::<Vec<usize>>());
    let writers = root::remove_writers(wids);
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
#[no_mangle]
pub unsafe extern "C" fn root_enable(wid: usize) -> isize {
    match root::enable(wid) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("enable failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn root_disable(wid: usize) -> isize {
    match root::disable(wid) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("disable failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn root_enable_type(typ: *mut WriterTypeEnum) -> isize {
    match root::enable_type(*Box::from_raw(typ)) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("enable failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn root_disable_type(typ: *mut WriterTypeEnum) -> isize {
    match root::disable_type(*Box::from_raw(typ)) {
        Ok(r) => Box::into_raw(Box::new(r)) as isize,
        Err(err) => {
            eprintln!("disable_type failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Sync specific writers.
#[no_mangle]
pub unsafe extern "C" fn root_sync(
    types: *mut WriterTypeEnum,
    type_cnt: c_uint,
    timeout: c_double,
) -> isize {
    let types = Vec::from_raw_parts(types, type_cnt as usize, type_cnt as usize);
    if let Err(err) = root::sync(types, timeout) {
        eprintln!("sync failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// Sync all writers.
#[no_mangle]
pub unsafe extern "C" fn root_sync_all(timeout: c_double) -> isize {
    if let Err(err) = root::sync_all(timeout) {
        eprintln!("sync_all failed: {err:?}");
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
pub unsafe extern "C" fn root_rotate(path: *mut PathBuf) -> isize {
    let path = if path.is_null() {
        None
    } else {
        Some(*Box::from_raw(path))
    };
    if let Err(err) = root::rotate(path) {
        eprintln!("rotate failed: {err:?}");
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
pub unsafe extern "C" fn root_set_encryption(wid: c_uint, key: *mut CKeyStruct) -> isize {
    let key = if key.is_null() {
        EncryptionMethod::NONE
    } else {
        let c_key = *Box::from_raw(key);
        let key = unsafe { slice::from_raw_parts(c_key.key, c_key.len as usize) }.to_vec();
        if c_key.typ == CEncryptionMethodEnum::AuthKey {
            EncryptionMethod::AuthKey(key)
        } else {
            EncryptionMethod::AES(key)
        }
    };
    if let Err(err) = root::set_encryption(wid as usize, key) {
        eprintln!("set_encryption failed: {err:?}");
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
pub unsafe extern "C" fn root_set_debug(debug: u8) {
    root::set_debug(debug);
}

/// # Safety
///
/// Get configuration.
#[no_mangle]
pub unsafe extern "C" fn root_get_writer_config(wid: c_uint) -> *const WriterConfigEnum {
    match root::get_writer_config(wid as usize) {
        Some(config) => &config,
        None => null(),
    }
}

/// # Safety
///
/// Get configuration.
#[no_mangle]
pub unsafe extern "C" fn root_get_writer_configs() -> *const CWriterConfigEnums {
    let mut configs = CWriterConfigEnums {
        cnt: 0,
        keys: Vec::new(),
        values: Vec::new(),
    };
    for (k, v) in root::get_writer_configs().into_iter() {
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
pub unsafe extern "C" fn root_get_server_config(wid: usize) -> *mut CServerConfig {
    match root::get_server_config(wid) {
        Ok(c) => Box::into_raw(Box::new(c.into())),
        Err(_err) => null::<CServerConfig>() as *mut _,
    }
}

/// # Safety
///
/// Get configuration.
#[no_mangle]
pub unsafe extern "C" fn root_get_server_configs() -> *const CServerConfigs {
    let mut keys: Vec<u32> = Vec::new();
    let mut values: Vec<CServerConfig> = Vec::new();
    let configs = root::get_server_configs();
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
#[no_mangle]
pub unsafe extern "C" fn root_get_root_server_address_port() -> *const char {
    match root::get_root_server_address_port() {
        Some(s) => CString::new(s).expect("Error: CString::new()").into_raw() as *const char,
        None => null(),
    }
}

/// # Safety
///
/// Get server configuration.
#[no_mangle]
pub unsafe extern "C" fn root_get_server_addresses_ports() -> *const Cu32StringVec {
    Box::into_raw(Box::new(Cu32StringVec::from(
        root::get_server_addresses_ports(),
    )))
}

/// # Safety
///
/// Get server configuration.
#[no_mangle]
pub unsafe extern "C" fn root_get_server_addresses() -> *const Cu32StringVec {
    Box::into_raw(Box::new(Cu32StringVec::from(root::get_server_addresses())))
}

/// # Safety
///
/// Get server configuration.
#[no_mangle]
pub unsafe extern "C" fn root_get_server_ports() -> *const Cu32u16Vec {
    Box::into_raw(Box::new(Cu32u16Vec::from(root::get_server_ports())))
}

/// # Safety
///
/// Get server authentification key.
#[no_mangle]
pub unsafe extern "C" fn root_get_server_auth_key() -> *mut CKeyStruct {
    let mut key = root::get_server_auth_key().key_cloned().unwrap();
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
#[no_mangle]
pub unsafe extern "C" fn root_get_config_string() -> *const c_char {
    let config = root::get_config_string();
    let ptr = config.as_ptr() as *const c_char;
    std::mem::forget(config);
    ptr
}

/// # Safety
///
/// Save configuration.
#[no_mangle]
pub unsafe extern "C" fn root_save_config(path: *const c_char) -> isize {
    let path = if path.is_null() {
        None
    } else {
        Some(PathBuf::from(char2string(path)))
    };
    if let Err(err) = root::save_config(path.as_deref()) {
        eprintln!("get_server_config failed: {err:?}");
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
pub unsafe extern "C" fn root_trace(message: *const c_char) -> isize {
    if let Err(err) = root::trace(char2string(message)) {
        eprintln!("trace failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// debug message.
#[no_mangle]
pub unsafe extern "C" fn root_debug(message: *const c_char) -> isize {
    if let Err(err) = root::debug(char2string(message)) {
        eprintln!("debug failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// info message.
#[no_mangle]
pub unsafe extern "C" fn root_info(message: *const c_char) -> isize {
    if let Err(err) = root::info(char2string(message)) {
        eprintln!("info failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// success message.
#[no_mangle]
pub unsafe extern "C" fn root_success(message: *const c_char) -> isize {
    if let Err(err) = root::success(char2string(message)) {
        eprintln!("success failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// warning message.
#[no_mangle]
pub unsafe extern "C" fn root_warning(message: *const c_char) -> isize {
    if let Err(err) = root::warning(char2string(message)) {
        eprintln!("warning failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// error message.
#[no_mangle]
pub unsafe extern "C" fn root_error(message: *const c_char) -> isize {
    if let Err(err) = root::error(char2string(message)) {
        eprintln!("error failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// critical message.
#[no_mangle]
pub unsafe extern "C" fn root_critical(message: *const c_char) -> isize {
    if let Err(err) = root::critical(char2string(message)) {
        eprintln!("critical failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// fatal message.
#[no_mangle]
pub unsafe extern "C" fn root_fatal(message: *const c_char) -> isize {
    if let Err(err) = root::fatal(char2string(message)) {
        eprintln!("fatal failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// exception message.
#[no_mangle]
pub unsafe extern "C" fn root_exception(message: *const c_char) -> isize {
    if let Err(err) = root::exception(char2string(message)) {
        eprintln!("exception failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}
