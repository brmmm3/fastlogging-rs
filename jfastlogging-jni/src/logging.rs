use std::path::{Path, PathBuf};

use jni::Env;

use jni::objects::{JClass, JString};

use jni::sys::{jboolean, jdouble, jint, jlong};

use fastlogging::{
    EncryptionMethod, ExtConfig, LevelSyms, Logger, Logging, WriterConfigEnum, WriterEnum,
    WriterTypeEnum,
};

use crate::{get_pathbuf, get_string, log_message};

/// # Safety
///
/// Create new default instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingNewDefault(
    _env: Env,
    _class: JClass,
) -> jlong {
    Box::into_raw(Box::new(Logging::default())) as jlong
}

/// # Safety
///
/// Create new instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingNew(
    mut env: Env,
    _class: JClass,
    level: jint, // Global log level
    domain: JString,
    configs_ptr: jlong,
    ext_config: *mut ExtConfig,
    config_path: JString, // Optional configuration file path
) -> jlong {
    println!("Java_org_logging_FastLogging_loggingNew");
    let domain: String = domain
        .try_to_string(&env)
        .map(|s| s.into())
        .ok()
        .unwrap_or_else(|| "root".to_string());
    let configs = if configs_ptr == 0 {
        None
    } else {
        // Reconstruct the Box<Vec<Box<WriterConfigEnum>>> from JNI pointer
        let configs: Box<Vec<WriterConfigEnum>> =
            unsafe { Box::from_raw(configs_ptr as *mut Vec<WriterConfigEnum>) };
        Some(*configs)
    };
    let ext_config = if ext_config.is_null() {
        None
    } else {
        Some(*unsafe { Box::from_raw(ext_config) })
    };
    let config_path = match config_path.is_null() {
        false => Some(get_pathbuf!(env, config_path, 0)),
        true => None,
    };
    let instance = Logging::new(level as u8, domain, configs, ext_config, config_path);
    Box::into_raw(Box::new(instance.unwrap())) as jlong
}

/// # Safety
///
/// This function destroys an instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingShutdown(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    now: jboolean,
) {
    if let Err(err) = logging.shutdown(now) {
        env.throw(err.to_string()).unwrap();
    }
    let _boxed_logging = unsafe { Box::from_raw(logging) };
}

/// # Safety
///
/// Set log level.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSetLevel(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    wid: jint,
    level: jint,
) -> jint {
    if let Err(err) = logging.set_level(wid as usize, level as u8) {
        env.throw(err.to_string()).unwrap();
        return -1;
    }
    0
}

/// # Safety
///
/// Set log domain.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSetDomain(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    domain: JString,
) -> jint {
    logging.set_domain(&get_string!(env, domain));
    0
}

/// # Safety
///
/// Set log level symbols.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSetLevel2Sym(
    mut _env: Env,
    _class: JClass,
    logging: &mut Logging,
    level2sym: &mut LevelSyms,
) {
    logging.set_level2sym(level2sym);
}

/// # Safety
///
/// Set extended configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSetExtConfig(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    ext_config: *mut ExtConfig,
) -> jint {
    if ext_config.is_null() {
        env.throw("ext_config is null").unwrap();
        return -1;
    }
    let ext_config = *unsafe { Box::from_raw(ext_config) };
    logging.set_ext_config(&ext_config);
    0
}

/// # Safety
///
/// Add a Logger instance
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingAddLogger(
    mut _env: Env,
    _class: JClass,
    logging: &mut Logging,
    logger: &mut Logger,
) {
    logging.add_logger(logger);
}

/// # Safety
///
/// Remove a Logger instance
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingRemoveLogger(
    mut _env: Env,
    _class: JClass,
    logging: &mut Logging,
    logger: &mut Logger,
) {
    logging.remove_logger(logger);
}

/// # Safety
///
/// Add a Writer instance
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingAddWriterConfig(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    config: &mut WriterConfigEnum,
) -> jint {
    println!("Java_org_logging_FastLogging_loggingAddWriterConfig");
    match logging.add_writer_config(config) {
        Ok(v) => v as jint,
        Err(err) => {
            env.throw(err.to_string()).unwrap();
            0
        }
    }
}

/// # Safety
///
/// Add a Writer instance
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingAddWriter(
    mut _env: Env,
    _class: JClass,
    logging: &mut Logging,
    writer_ptr: jlong,
) -> jint {
    if writer_ptr == 0 {
        return 0;
    }
    println!("Java_org_logging_FastLogging_loggingAddWriter: {writer_ptr}");
    let writer: Box<WriterEnum> = unsafe { Box::from_raw(writer_ptr as *mut WriterEnum) };
    println!("Java_org_logging_FastLogging_loggingAddWriter: {writer:?}");
    logging.add_writer(*writer) as jint
}

/// # Safety
///
/// Remove a Writer instance
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingRemoveWriter(
    mut _env: Env,
    _class: JClass,
    logging: &mut Logging,
    wid: jint,
) {
    logging.remove_writer(wid as usize);
}

/// # Safety
///
/// Add a Writer instance
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingAddWriterConfigs(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    configs_ptr: jlong,
) {
    if configs_ptr == 0 {
        return;
    }
    // Reconstruct the Box<Vec<Box<WriterConfigEnum>>> from JNI pointer
    let configs: Box<Vec<WriterConfigEnum>> =
        unsafe { Box::from_raw(configs_ptr as *mut Vec<WriterConfigEnum>) };
    if let Err(err) = logging.add_writer_configs(*configs) {
        env.throw(err.to_string()).unwrap();
    }
}

/// # Safety
///
/// This function destroys an instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSync(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    types_ptr: jlong,
    timeout: jdouble,
) -> jint {
    if types_ptr == 0 {
        eprintln!("loggingSync: null types_ptr");
        return -1;
    }
    // Reconstruct the Box<Vec<Box<WriterTypeEnum>>> from JNI pointer
    let types: Box<Vec<WriterTypeEnum>> =
        unsafe { Box::from_raw(types_ptr as *mut Vec<WriterTypeEnum>) };
    if let Err(err) = logging.sync(*types, timeout) {
        env.throw(err.to_string()).unwrap();
        return -1;
    }
    0
}

/// # Safety
///
/// This function destroys an instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSyncAll(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    timeout: jdouble,
) -> jint {
    if let Err(err) = logging.sync_all(timeout) {
        env.throw(err.to_string()).unwrap();
        return -1;
    }
    0
}

/// # Safety
///
/// This function destroys an instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingRotate(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    path: JString,
) -> jint {
    let path = if !path.is_null() {
        Some(get_pathbuf!(env, path))
    } else {
        None
    };
    if let Err(err) = logging.rotate(path) {
        env.throw(err.to_string()).unwrap();
        return -1;
    }
    0
}

/// # Safety
///
/// Set server/client encryption
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSetEncryption(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    wid: jint,
    method: jint,
    key: JString,
) -> jint {
    let method = match method as i8 {
        0 => EncryptionMethod::NONE,
        1 => EncryptionMethod::AuthKey(get_string!(env, key).as_bytes().to_vec()),
        2 => EncryptionMethod::AES(get_string!(env, key).as_bytes().to_vec()),
        _ => {
            env.throw(format!("Invalid value {method} for method."))
                .unwrap();
            return -1;
        }
    };
    if let Err(err) = logging.set_encryption(wid as usize, method) {
        env.throw(err.to_string()).unwrap();
        return -1;
    }
    0
}

/// # Safety
///
/// Get writer configuration
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetWriterConfig(
    _env: Env,
    _class: JClass,
    logging: &mut Logging,
    wid: jint,
) -> jlong {
    Box::into_raw(Box::new(logging.get_writer_config(wid as usize))) as jlong
}

/// # Safety
///
/// Get server configuration
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetServerConfig<'a>(
    mut _env: Env<'a>,
    _class: JClass<'a>,
    logging: &mut Logging,
    wid: jint,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_config(wid as usize))) as jlong
}

/// # Safety
///
/// Get server configurations
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetServerConfigs(
    _env: Env,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_configs())) as jlong
}

/// # Safety
///
/// Get server addresses
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetServerAddresses(
    _env: Env,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_addresses())) as jlong
}

/// # Safety
///
/// Get server addresses
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetServerPorts(
    _env: Env,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_ports())) as jlong
}

/// # Safety
///
/// Get server configuration
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetServerAuthKey(
    _env: Env,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_auth_key())) as jlong
}

/// # Safety
///
/// Get server configuration
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetConfigString(
    _env: Env,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_config_string())) as jlong
}

/// # Safety
///
/// Get server configuration
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSaveConfig(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    path: JString,
) -> jlong {
    let path = get_string!(env, path);
    Box::into_raw(Box::new(logging.save_config(Some(Path::new(&path))))) as jlong
}

/// # Safety
///
/// trace message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingTrace(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, trace, message)
}

/// # Safety
///
/// debug message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingDebug(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, debug, message)
}

/// # Safety
///
/// info message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingInfo(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, info, message)
}

/// # Safety
///
/// success message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSuccess(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, success, message)
}

/// # Safety
///
/// warning message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingWarning(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, warning, message)
}

/// # Safety
///
/// error message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingError(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, error, message)
}

/// # Safety
///
/// critical error message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingCritical(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, critical, message)
}

/// # Safety
///
/// fatal error message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingFatal(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, fatal, message)
}

/// # Safety
///
/// exception error message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingException(
    mut env: Env,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, exception, message)
}
