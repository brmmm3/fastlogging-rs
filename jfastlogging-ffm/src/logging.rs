use std::path::{Path, PathBuf};

use fastlogging::{
    EncryptionMethod, ExtConfig, LevelSyms, Logger, Logging, WriterConfigEnum, WriterEnum,
    WriterTypeEnum,
};

use crate::{get_option_str, log_message};

/// # Safety
///
/// Create new default instance (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingNewDefault() -> *mut Logging {
    Box::into_raw(Box::new(Logging::default()))
}

/// # Safety
///
/// Create new instance (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingNew(
    level: i32,
    domain_ptr: *const u8,
    domain_len: usize,
    configs_ptr: *const *mut WriterConfigEnum,
    configs_len: usize,
    ext_config: *mut ExtConfig,
    config_path_ptr: *const u8,
    config_path_len: usize,
) -> *mut Logging {
    let domain = if domain_ptr.is_null() {
        "root"
    } else {
        get_option_str(domain_ptr, domain_len).unwrap_or("root")
    };
    let configs = if !configs_ptr.is_null() && configs_len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(configs_ptr, configs_len) };
        slice
            .iter()
            .map(|&w| *unsafe { Box::from_raw(w) })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let ext_config = if ext_config.is_null() {
        None
    } else {
        Some(*unsafe { Box::from_raw(ext_config) })
    };
    let config_path = if !config_path_ptr.is_null() && config_path_len > 0 {
        get_option_str(config_path_ptr, config_path_len).map(|s| PathBuf::from(s))
    } else {
        None
    };
    let instance = Logging::new(level as u8, domain, Some(configs), ext_config, config_path);
    Box::into_raw(Box::new(instance.unwrap()))
}

/// # Safety
///
/// This function destroys an instance (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingShutdown(logging: *mut Logging, now: i32) {
    if let Some(logging) = unsafe { logging.as_mut() } {
        let _ = logging.shutdown(now != 0);
        let _boxed_logging = unsafe { Box::from_raw(logging) };
    }
}

/// # Safety
///
/// Set log level (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingSetLevel(logging: *mut Logging, wid: usize, level: u8) -> i32 {
    if logging.is_null() {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    match logging.set_level(wid, level) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// # Safety
///
/// Set log domain (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingSetDomain(
    logging: *mut Logging,
    domain_ptr: *const u8,
    domain_len: usize,
) -> i32 {
    if logging.is_null() || domain_ptr.is_null() {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    if let Some(domain) = get_option_str(domain_ptr, domain_len) {
        logging.set_domain(domain);
        0
    } else {
        -1
    }
}

/// # Safety
///
/// Set log level symbols (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingSetLevel2Sym(
    logging: *mut Logging,
    level2sym: *mut LevelSyms,
) -> i32 {
    if logging.is_null() || level2sym.is_null() {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    let level2sym = unsafe { &mut *level2sym };
    logging.set_level2sym(level2sym);
    0
}

/// # Safety
///
/// Set extended configuration (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingSetExtConfig(
    logging: *mut Logging,
    ext_config: *mut ExtConfig,
) -> i32 {
    if logging.is_null() || ext_config.is_null() {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    let ext_config = unsafe { &*ext_config };
    logging.set_ext_config(ext_config);
    0
}

/// # Safety
///
/// Add a Logger instance (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingAddLogger(logging: *mut Logging, logger: *mut Logger) -> i32 {
    if logging.is_null() || logger.is_null() {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    let logger = unsafe { &mut *logger };
    logging.add_logger(logger);
    0
}

/// # Safety
///
/// Remove a Logger instance (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingRemoveLogger(logging: *mut Logging, logger: *mut Logger) -> i32 {
    if logging.is_null() || logger.is_null() {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    let logger = unsafe { &mut *logger };
    logging.remove_logger(logger);
    0
}

/// # Safety
///
/// Add a WriterConfig instance (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingAddWriterConfig(
    logging: *mut Logging,
    config: *mut WriterConfigEnum,
) -> i32 {
    if logging.is_null() || config.is_null() {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    let config = unsafe { &mut *config };
    match logging.add_writer_config(config) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// # Safety
///
/// Add a Writer instance (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingAddWriter(logging: *mut Logging, writer: *mut WriterEnum) -> i32 {
    if logging.is_null() || writer.is_null() {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    let writer = unsafe { Box::from_raw(writer) };
    logging.add_writer(*writer);
    0
}

/// # Safety
///
/// Remove a Writer instance (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingRemoveWriter(logging: *mut Logging, wid: usize) -> i32 {
    if logging.is_null() {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    logging.remove_writer(wid);
    0
}

/// # Safety
///
/// Add multiple WriterConfig instances (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingAddWriterConfigs(
    logging: *mut Logging,
    configs_ptr: *const *mut WriterConfigEnum,
    configs_len: usize,
) -> i32 {
    if logging.is_null() || configs_ptr.is_null() || configs_len == 0 {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    let configs = unsafe {
        std::slice::from_raw_parts(configs_ptr, configs_len)
            .iter()
            .map(|&w| *Box::from_raw(w))
            .collect::<Vec<WriterConfigEnum>>()
    };
    match logging.add_writer_configs(configs) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// # Safety
///
/// Sync writers (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingSync(
    logging: *mut Logging,
    writer_ids_ptr: *const WriterTypeEnum,
    writer_ids_len: usize,
    timeout: f64,
) -> i32 {
    if logging.is_null() || (writer_ids_ptr.is_null() && writer_ids_len > 0) {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    let writer_ids: Vec<WriterTypeEnum> = if writer_ids_len > 0 {
        unsafe { std::slice::from_raw_parts(writer_ids_ptr, writer_ids_len) }.to_vec()
    } else {
        Vec::new()
    };
    match logging.sync(writer_ids, timeout) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// # Safety
///
/// Sync all writers (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingSyncAll(logging: *mut Logging, timeout: f64) -> i32 {
    if logging.is_null() {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    match logging.sync_all(timeout) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// # Safety
///
/// Rotate log file (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingRotate(
    logging: *mut Logging,
    path_ptr: *const u8,
    path_len: usize,
) -> i32 {
    if logging.is_null() {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    let path = if !path_ptr.is_null() && path_len > 0 {
        get_option_str(path_ptr, path_len).map(PathBuf::from)
    } else {
        None
    };
    match logging.rotate(path) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// # Safety
/// # Safety
///
/// Set server/client encryption (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingSetEncryption(
    logging: *mut Logging,
    wid: usize,
    method: i32,
    key_ptr: *const u8,
    key_len: usize,
) -> i32 {
    if logging.is_null() {
        return -1;
    }
    let logging = unsafe { &mut *logging };
    let key = if !key_ptr.is_null() && key_len > 0 {
        unsafe { std::slice::from_raw_parts(key_ptr, key_len) }
    } else {
        &[]
    };
    let method = match method as i8 {
        0 => EncryptionMethod::NONE,
        1 => EncryptionMethod::AuthKey(key.to_vec()),
        2 => EncryptionMethod::AES(key.to_vec()),
        _ => return -1,
    };
    match logging.set_encryption(wid, method) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// # Safety
///
/// Get configuration (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingGetWriterConfig(
    logging: *mut Logging,
    wid: usize,
) -> *mut std::ffi::c_void {
    if logging.is_null() {
        return std::ptr::null_mut();
    }
    let logging = unsafe { &mut *logging };
    let config = logging.get_writer_config(wid);
    Box::into_raw(Box::new(config)) as *mut std::ffi::c_void
}

/// # Safety
///
/// Get server configuration (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingGetServerConfig(
    logging: *mut Logging,
    wid: usize,
) -> *mut std::ffi::c_void {
    if logging.is_null() {
        return std::ptr::null_mut();
    }
    let logging = unsafe { &mut *logging };
    let config = logging.get_server_config(wid);
    Box::into_raw(Box::new(config)) as *mut std::ffi::c_void
}

/// # Safety
///
/// Get server configurations (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingGetServerConfigs(logging: *mut Logging) -> *mut std::ffi::c_void {
    if logging.is_null() {
        return std::ptr::null_mut();
    }
    let logging = unsafe { &mut *logging };
    let configs = logging.get_server_configs();
    Box::into_raw(Box::new(configs)) as *mut std::ffi::c_void
}

/// # Safety
///
/// Get server addresses (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingGetServerAddresses(logging: *mut Logging) -> *mut std::ffi::c_void {
    if logging.is_null() {
        return std::ptr::null_mut();
    }
    let logging = unsafe { &mut *logging };
    let addresses = logging.get_server_addresses();
    Box::into_raw(Box::new(addresses)) as *mut std::ffi::c_void
}

/// # Safety
///
/// Get server ports (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingGetServerPorts(logging: *mut Logging) -> *mut std::ffi::c_void {
    if logging.is_null() {
        return std::ptr::null_mut();
    }
    let logging = unsafe { &mut *logging };
    let ports = logging.get_server_ports();
    Box::into_raw(Box::new(ports)) as *mut std::ffi::c_void
}

/// # Safety
///
/// Get server auth key (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingGetServerAuthKey(logging: *mut Logging) -> *mut std::ffi::c_void {
    if logging.is_null() {
        return std::ptr::null_mut();
    }
    let logging = unsafe { &mut *logging };
    let key = logging.get_server_auth_key();
    Box::into_raw(Box::new(key)) as *mut std::ffi::c_void
}

/// # Safety
///
/// Get config string (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingGetConfigString(logging: *mut Logging) -> *mut std::ffi::c_void {
    if logging.is_null() {
        return std::ptr::null_mut();
    }
    let logging = unsafe { &mut *logging };
    let config_str = logging.get_config_string();
    Box::into_raw(Box::new(config_str)) as *mut std::ffi::c_void
}

/// # Safety
///
/// Save config (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingSaveConfig(
    logging: *mut Logging,
    path_ptr: *const u8,
    path_len: usize,
) -> *mut std::ffi::c_void {
    if logging.is_null() {
        return std::ptr::null_mut();
    }
    let logging = unsafe { &mut *logging };
    let path = get_option_str(path_ptr, path_len).map(Path::new);
    let result = logging.save_config(path);
    Box::into_raw(Box::new(result)) as *mut std::ffi::c_void
}

/// # Safety
///
/// trace message (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingTrace(
    logging: *mut Logging,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logging, msg_ptr, msg_len, trace)
}

/// # Safety
///
/// debug message (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingDebug(
    logging: *mut Logging,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logging, msg_ptr, msg_len, debug)
}

/// # Safety
///
/// info message (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingInfo(
    logging: *mut Logging,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logging, msg_ptr, msg_len, info)
}

/// # Safety
///
/// success message (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingSuccess(
    logging: *mut Logging,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logging, msg_ptr, msg_len, success)
}

/// # Safety
///
/// warning message (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingWarning(
    logging: *mut Logging,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logging, msg_ptr, msg_len, warning)
}

/// # Safety
///
/// error message (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingError(
    logging: *mut Logging,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logging, msg_ptr, msg_len, error)
}

/// # Safety
///
/// critical error message (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingCritical(
    logging: *mut Logging,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logging, msg_ptr, msg_len, critical)
}

/// # Safety
///
/// fatal error message (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingFatal(
    logging: *mut Logging,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logging, msg_ptr, msg_len, fatal)
}

/// # Safety
///
/// exception error message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggingException(
    logging: *mut Logging,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logging, msg_ptr, msg_len, exception)
}
