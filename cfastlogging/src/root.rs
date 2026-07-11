use std::{
    ffi::{c_char, c_double, c_uint, CString},
    path::PathBuf,
    ptr::null,
    slice,
};

use crate::{
    def::{
        Cu32StringVec, Cu32u16Vec, CusizeVec, ServerConfig, ServerConfigs, WriterConfigEnums,
        WriterEnum, WriterEnums,
    },
    util::char2string,
    EncryptionMethodEnum, KeyStruct,
};

/// # Safety
///
/// Create new logging instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_init() {
    fastlogging::root::root_init();
}

/// # Safety
///
/// Shutdown root
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_shutdown(now: bool) -> isize {
    if let Err(err) = fastlogging::root::shutdown(now) {
        eprintln!("shutdown failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// Set logging level.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_set_level(wid: c_uint, level: u8) -> isize {
    if let Err(err) = fastlogging::root::set_level(wid as usize, level) {
        eprintln!("set_level failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// Set logging domain.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_set_domain(domain: *const c_char) {
    fastlogging::root::set_domain(char2string(domain));
}

/// # Safety
///
/// Set log level symbols.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_set_level2sym(level2sym: u8) {
    fastlogging::root::set_level2sym(if level2sym == 0 {
        &fastlogging::LevelSyms::Sym
    } else if level2sym == 1 {
        &fastlogging::LevelSyms::Short
    } else {
        &fastlogging::LevelSyms::Str
    });
}

/// # Safety
///
/// Set extended configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_set_ext_config(ext_config: &fastlogging::ExtConfig) {
    fastlogging::root::set_ext_config(ext_config);
}

/// # Safety
///
/// Add logger.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_add_logger(logger: &mut fastlogging::Logger) {
    fastlogging::root::add_logger(logger);
}

/// # Safety
///
/// Remove logger.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_remove_logger(logger: &mut fastlogging::Logger) {
    fastlogging::root::remove_logger(logger);
}

/// # Safety
///
/// Add writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_set_root_writer_config(
    config: *mut fastlogging::WriterConfigEnum,
) -> isize {
    unsafe {
        match fastlogging::root::set_root_writer_config(&Box::from_raw(config)) {
            Ok(_r) => 0,
            Err(err) => {
                eprintln!("set_root_writer_config failed: {err:?}");
                err.as_int() as isize
            }
        }
    }
}

/// # Safety
///
/// Add writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_set_root_writer(writer: *mut fastlogging::WriterEnum) -> isize {
    unsafe {
        match fastlogging::root::set_root_writer(*Box::from_raw(writer)) {
            Ok(r) => Box::into_raw(Box::new(r)) as isize,
            Err(err) => {
                eprintln!("set_root_writer failed: {err:?}");
                err.as_int() as isize
            }
        }
    }
}

/// # Safety
///
/// Add writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_add_writer_config(
    config: *mut fastlogging::WriterConfigEnum,
) -> isize {
    unsafe {
        let config = *Box::from_raw(config);
        match fastlogging::root::add_writer_config(&config) {
            Ok(r) => Box::into_raw(Box::new(r)) as isize,
            Err(err) => {
                eprintln!("add_writer_config failed: {err:?}");
                err.as_int() as isize
            }
        }
    }
}

/// # Safety
///
/// Add writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_add_writer(writer: *mut fastlogging::WriterEnum) -> usize {
    unsafe { fastlogging::root::add_writer(*Box::from_raw(writer)) }
}

/// # Safety
///
/// Remove writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_remove_writer(wid: usize) -> *const fastlogging::WriterEnum {
    match fastlogging::root::remove_writer(wid) {
        Some(w) => Box::into_raw(Box::new(w)),
        None => null(),
    }
}

/// # Safety
///
/// Add writers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_add_writer_configs(
    configs_ptr: *const *mut Vec<fastlogging::WriterConfigEnum>,
) -> isize {
    let configs: Box<Vec<fastlogging::WriterConfigEnum>> =
        unsafe { Box::from_raw(configs_ptr as *mut Vec<fastlogging::WriterConfigEnum>) };
    match fastlogging::root::add_writer_configs(*configs) {
        Ok(wids) => Box::into_raw(Box::new(CusizeVec {
            cnt: wids.len() as u32,
            values: wids,
        })) as isize,
        Err(err) => {
            eprintln!("add_writer_configs failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writers top root logger.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_add_writers(
    writers_ptr: *mut Vec<fastlogging::WriterEnum>,
) -> *mut CusizeVec {
    let writers = unsafe { Box::from_raw(writers_ptr) };
    let wids = fastlogging::root::add_writers(*writers);
    Box::into_raw(Box::new(CusizeVec {
        cnt: wids.len() as u32,
        values: wids,
    }))
}

/// # Safety
///
/// Remove writers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_remove_writers(wids: *mut u32, wid_cnt: u32) -> *mut WriterEnums {
    let wids: Option<&[u32]> = if !(wids as *const u32).is_null() {
        Some(unsafe { slice::from_raw_parts(wids, wid_cnt as usize) })
    } else {
        None
    };
    let wids = wids.map(|w| w.iter().map(|w| *w as usize).collect::<Vec<usize>>());
    let writers = fastlogging::root::remove_writers(wids);
    let writers = writers
        .into_iter()
        .map(|w| w.into())
        .collect::<Vec<WriterEnum>>();
    Box::into_raw(Box::new(WriterEnums {
        cnt: writers.len() as u32,
        values: Box::into_raw(Box::new(writers)) as *const WriterEnum,
    }))
}

/// # Safety
///
/// Add writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_enable(wid: usize) -> isize {
    match fastlogging::root::enable(wid) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("enable failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_disable(wid: usize) -> isize {
    match fastlogging::root::disable(wid) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("disable failed: {err:?}");
            err.as_int() as isize
        }
    }
}

/// # Safety
///
/// Add writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_enable_type(typ: *mut fastlogging::WriterTypeEnum) -> isize {
    unsafe {
        match fastlogging::root::enable_type(*Box::from_raw(typ)) {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("enable failed: {err:?}");
                err.as_int() as isize
            }
        }
    }
}

/// # Safety
///
/// Add writer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_disable_type(typ: *mut fastlogging::WriterTypeEnum) -> isize {
    unsafe {
        match fastlogging::root::disable_type(*Box::from_raw(typ)) {
            Ok(_) => 0,
            Err(err) => {
                eprintln!("disable_type failed: {err:?}");
                err.as_int() as isize
            }
        }
    }
}

/// # Safety
///
/// Sync specific writers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_sync(
    types: *mut Vec<fastlogging::WriterTypeEnum>,
    timeout: c_double,
) -> isize {
    let types: Box<Vec<fastlogging::WriterTypeEnum>> = unsafe { Box::from_raw(types) };
    if let Err(err) = fastlogging::root::sync(*types, timeout) {
        eprintln!("sync failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// Sync all writers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_sync_all(timeout: c_double) -> isize {
    if let Err(err) = fastlogging::root::sync_all(timeout) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_rotate(path: *mut PathBuf) -> isize {
    let path = if path.is_null() {
        None
    } else {
        Some(unsafe { *Box::from_raw(path) })
    };
    if let Err(err) = fastlogging::root::rotate(path) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_set_encryption(wid: c_uint, key: *mut KeyStruct) -> isize {
    let key = if key.is_null() {
        fastlogging::EncryptionMethod::NONE
    } else {
        let c_key = unsafe { *Box::from_raw(key) };
        let key = unsafe { slice::from_raw_parts(c_key.key, c_key.len as usize) }.to_vec();
        if c_key.typ == EncryptionMethodEnum::AuthKey {
            fastlogging::EncryptionMethod::AuthKey(key)
        } else {
            fastlogging::EncryptionMethod::AES(key)
        }
    };
    if let Err(err) = fastlogging::root::set_encryption(wid as usize, key) {
        eprintln!("set_encryption failed: {err:?}");
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
pub unsafe extern "C" fn root_get_writer_config(
    wid: c_uint,
) -> *const fastlogging::WriterConfigEnum {
    match fastlogging::root::get_writer_config(wid as usize) {
        Some(config) => &config,
        None => null(),
    }
}

/// # Safety
///
/// Get configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_get_writer_configs() -> *const WriterConfigEnums {
    let mut configs = WriterConfigEnums {
        cnt: 0,
        keys: Vec::new(),
        values: Vec::new(),
    };
    for (k, v) in fastlogging::root::get_writer_configs().into_iter() {
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
pub unsafe extern "C" fn root_get_server_config(wid: usize) -> *mut ServerConfig {
    match fastlogging::root::get_server_config(wid) {
        Ok(c) => Box::into_raw(Box::new(c.into())),
        Err(_err) => null::<ServerConfig>() as *mut _,
    }
}

/// # Safety
///
/// Get configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_get_server_configs() -> *const ServerConfigs {
    let mut keys: Vec<u32> = Vec::new();
    let mut values: Vec<ServerConfig> = Vec::new();
    let configs = fastlogging::root::get_server_configs();
    for (k, c) in configs.into_iter() {
        keys.push(k as u32);
        values.push(c.into());
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_get_root_server_address_port() -> *const char {
    match fastlogging::root::get_root_server_address_port() {
        Some(s) => CString::new(s).expect("Error: CString::new()").into_raw() as *const char,
        None => null(),
    }
}

/// # Safety
///
/// Get server configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_get_server_addresses_ports() -> *const Cu32StringVec {
    Box::into_raw(Box::new(Cu32StringVec::from(
        fastlogging::root::get_server_addresses_ports(),
    )))
}

/// # Safety
///
/// Get server configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_get_server_addresses() -> *const Cu32StringVec {
    Box::into_raw(Box::new(Cu32StringVec::from(
        fastlogging::root::get_server_addresses(),
    )))
}

/// # Safety
///
/// Get server configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_get_server_ports() -> *const Cu32u16Vec {
    Box::into_raw(Box::new(Cu32u16Vec::from(
        fastlogging::root::get_server_ports(),
    )))
}

/// # Safety
///
/// Get server authentification key.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_get_server_auth_key() -> *mut KeyStruct {
    let mut key = fastlogging::root::get_server_auth_key()
        .key_cloned()
        .unwrap();
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_get_config_string() -> *const c_char {
    let config = fastlogging::root::get_config_string();
    let ptr = config.as_ptr() as *const c_char;
    std::mem::forget(config);
    ptr
}

/// # Safety
///
/// Save configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_save_config(path: *const c_char) -> isize {
    let path = if path.is_null() {
        None
    } else {
        Some(PathBuf::from(char2string(path)))
    };
    if let Err(err) = fastlogging::root::save_config(path.as_deref()) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_trace(message: *const c_char) -> isize {
    if let Err(err) = fastlogging::root::trace(char2string(message)) {
        eprintln!("trace failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// debug message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_debug(message: *const c_char) -> isize {
    if let Err(err) = fastlogging::root::debug(char2string(message)) {
        eprintln!("debug failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// info message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_info(message: *const c_char) -> isize {
    if let Err(err) = fastlogging::root::info(char2string(message)) {
        eprintln!("info failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// success message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_success(message: *const c_char) -> isize {
    if let Err(err) = fastlogging::root::success(char2string(message)) {
        eprintln!("success failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// warning message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_warning(message: *const c_char) -> isize {
    if let Err(err) = fastlogging::root::warning(char2string(message)) {
        eprintln!("warning failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// error message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_error(message: *const c_char) -> isize {
    if let Err(err) = fastlogging::root::error(char2string(message)) {
        eprintln!("error failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// critical message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_critical(message: *const c_char) -> isize {
    if let Err(err) = fastlogging::root::critical(char2string(message)) {
        eprintln!("critical failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// fatal message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_fatal(message: *const c_char) -> isize {
    if let Err(err) = fastlogging::root::fatal(char2string(message)) {
        eprintln!("fatal failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// exception message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_exception(message: *const c_char) -> isize {
    if let Err(err) = fastlogging::root::exception(char2string(message)) {
        eprintln!("exception failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// Set debug level.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn root_set_debug(debug: u8) {
    fastlogging::root::set_debug(debug);
}
