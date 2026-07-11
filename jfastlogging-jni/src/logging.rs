use std::path::{Path, PathBuf};

use jni::jni_mangle;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jdouble, jint, jlong};

use fastlogging::{
    EncryptionMethod, ExtConfig, LevelSyms, Logger, Logging, WriterConfigEnum, WriterEnum,
    WriterTypeEnum,
};

use crate::{enter_jni, log_message};

/// # Safety
///
/// Create new default instance.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingNewDefault")]
pub fn loggingNewDefault(_env: jni::EnvUnowned, _class: JClass) -> jlong {
    Box::into_raw(Box::new(Logging::default())) as jlong
}

/// # Safety
///
/// Create new instance.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingNew")]
pub unsafe fn loggingNew(
    _env: jni::EnvUnowned,
    _class: JClass,
    level: jint, // Global log level
    domain: JString,
    configs_ptr: jlong,
    ext_config: *mut ExtConfig,
    config_path: JString, // Optional configuration file path
) -> jlong {
    println!("Java_org_logging_FastLogging_loggingNew");
    let domain = if domain.is_null() {
        "root".to_string()
    } else {
        JString::to_string(&domain)
    };
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
        false => Some(PathBuf::from(JString::to_string(&config_path))),
        true => None,
    };
    let instance = Logging::new(level as u8, domain, configs, ext_config, config_path);
    Box::into_raw(Box::new(instance.unwrap())) as jlong
}

/// # Safety
///
/// This function destroys an instance.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingShutdown")]
pub fn loggingShutdown(env: jni::EnvUnowned, _class: JClass, logging: &mut Logging, now: jboolean) {
    if let Err(err) = logging.shutdown(now) {
        enter_jni(env, |env| {
            env.throw(err.to_string()).unwrap();
            Ok(())
        });
    }
    let _boxed_logging = unsafe { Box::from_raw(logging) };
}

/// # Safety
///
/// Set log level.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingSetLevel")]
pub fn loggingSetLevel(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    wid: jint,
    level: jint,
) -> jint {
    enter_jni(env, |env| {
        if let Err(err) = logging.set_level(wid as usize, level as u8) {
            env.throw(err.to_string()).unwrap();
            return Ok(-1);
        }
        Ok(0)
    })
}

/// # Safety
///
/// Set log domain.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingSetDomain")]
pub fn loggingSetDomain(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    domain: JString,
) -> jint {
    let domain: String = JString::to_string(&domain);
    logging.set_domain(&domain);
    0
}

/// # Safety
///
/// Set log level symbols.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingSetLevel2Sym")]
pub fn loggingSetLevel2Sym(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    level2sym: &mut LevelSyms,
) {
    logging.set_level2sym(level2sym);
}

/// # Safety
///
/// Set extended configuration.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingSetExtConfig")]
pub unsafe fn loggingSetExtConfig(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    ext_config: *mut ExtConfig,
) -> jint {
    enter_jni(env, |env| {
        if ext_config.is_null() {
            env.throw("ext_config is null").unwrap();
            return Ok(-1);
        }
        let ext_config = *unsafe { Box::from_raw(ext_config) };
        logging.set_ext_config(&ext_config);
        Ok(0)
    })
}

/// # Safety
///
/// Add a Logger instance
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingAddLogger")]
pub fn loggingAddLogger(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    logger: &mut Logger,
) {
    logging.add_logger(logger);
}

/// # Safety
///
/// Remove a Logger instance
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingRemoveLogger")]
pub fn loggingRemoveLogger(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    logger: &mut Logger,
) {
    logging.remove_logger(logger);
}

/// # Safety
///
/// Add a Writer instance
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingAddWriterConfig")]
pub fn loggingAddWriterConfig(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    config: &mut WriterConfigEnum,
) -> jint {
    enter_jni(env, |env| match logging.add_writer_config(config) {
        Ok(v) => Ok(v as isize),
        Err(err) => {
            env.throw(err.to_string()).unwrap();
            Ok(-1)
        }
    }) as jint
}

/// # Safety
///
/// Add a Writer instance
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingAddWriter")]
pub fn loggingAddWriter(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    writer_ptr: jlong,
) -> jint {
    if writer_ptr == 0 {
        return 0;
    }
    let writer: Box<WriterEnum> = unsafe { Box::from_raw(writer_ptr as *mut WriterEnum) };
    logging.add_writer(*writer) as jint
}

/// # Safety
///
/// Remove a Writer instance
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingRemoveWriter")]
pub fn loggingRemoveWriter(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    wid: jint,
) {
    logging.remove_writer(wid as usize);
}

/// # Safety
///
/// Add a Writer instance
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingAddWriterConfigs")]
pub fn loggingAddWriterConfigs(
    env: jni::EnvUnowned,
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
        enter_jni(env, |env| {
            env.throw(err.to_string()).unwrap();
            Ok(())
        })
    }
}

/// # Safety
///
/// This function destroys an instance.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingSync")]
pub fn loggingSync(
    env: jni::EnvUnowned,
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
    enter_jni(env, |env| {
        if let Err(err) = logging.sync(*types, timeout) {
            env.throw(err.to_string()).unwrap();
            return Ok(-1);
        }
        Ok(0)
    })
}

/// # Safety
///
/// This function destroys an instance.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingSyncAll")]
pub fn loggingSyncAll(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    timeout: jdouble,
) -> jint {
    enter_jni(env, |env| {
        if let Err(err) = logging.sync_all(timeout) {
            env.throw(err.to_string()).unwrap();
            return Ok(-1);
        }
        Ok(0)
    })
}

/// # Safety
///
/// This function destroys an instance.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingRotate")]
pub fn loggingRotate(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    path: JString,
) -> jint {
    let path: Option<PathBuf> = if path.is_null() {
        None
    } else {
        Some(PathBuf::from(JString::to_string(&path)))
    };
    enter_jni(env, |env| {
        if let Err(err) = logging.rotate(path) {
            env.throw(err.to_string()).unwrap();
            return Ok(-1);
        }
        Ok(0)
    })
}

/// # Safety
///
/// Set server/client encryption
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingSetEncryption")]
pub fn loggingSetEncryption(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    wid: jint,
    method: jint,
    key: JString,
) -> jint {
    enter_jni(env, |env| {
        let method = match method as i8 {
            0 => EncryptionMethod::NONE,
            1 => EncryptionMethod::AuthKey(JString::to_string(&key).as_bytes().to_vec()),
            2 => EncryptionMethod::AES(JString::to_string(&key).as_bytes().to_vec()),
            _ => {
                env.throw(format!("Invalid value {method} for method."))
                    .unwrap();
                return Ok(-1);
            }
        };
        if let Err(err) = logging.set_encryption(wid as usize, method) {
            env.throw(err.to_string()).unwrap();
            return Ok(-1);
        }
        Ok(0)
    })
}

/// # Safety
///
/// Get writer configuration
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingGetWriterConfig")]
pub fn loggingGetWriterConfig(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    wid: jint,
) -> jlong {
    Box::into_raw(Box::new(logging.get_writer_config(wid as usize))) as jlong
}

/// # Safety
///
/// Get server configuration
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingGetServerConfig")]
pub fn loggingGetServerConfig(
    mut _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    wid: jint,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_config(wid as usize))) as jlong
}

/// # Safety
///
/// Get server configurations
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingGetServerConfigs")]
pub fn loggingGetServerConfigs(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_configs())) as jlong
}

/// # Safety
///
/// Get server addresses
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingGetServerAddresses")]
pub fn loggingGetServerAddresses(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_addresses())) as jlong
}

/// # Safety
///
/// Get server addresses
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingGetServerPorts")]
pub fn loggingGetServerPorts(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_ports())) as jlong
}

/// # Safety
///
/// Get server configuration
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingGetServerAuthKey")]
pub fn loggingGetServerAuthKey(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_auth_key())) as jlong
}

/// # Safety
///
/// Get server configuration
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingGetConfigString")]
pub fn loggingGetConfigString(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_config_string())) as jlong
}

/// # Safety
///
/// Get server configuration
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingSaveConfig")]
pub fn loggingSaveConfig(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    path: JString,
) -> jlong {
    let path: String = JString::to_string(&path);
    Box::into_raw(Box::new(logging.save_config(Some(Path::new(&path))))) as jlong
}

/// # Safety
///
/// trace message.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingTrace")]
pub fn loggingTrace(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, trace, message)
}

/// # Safety
///
/// debug message.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingDebug")]
pub fn loggingDebug(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, debug, message)
}

/// # Safety
///
/// info message.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingInfo")]
pub fn loggingInfo(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, info, message)
}

/// # Safety
///
/// success message.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingSuccess")]
pub fn loggingSuccess(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, success, message)
}

/// # Safety
///
/// warning message.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingWarning")]
pub fn loggingWarning(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, warning, message)
}

/// # Safety
///
/// error message.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingError")]
pub fn loggingError(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, error, message)
}

/// # Safety
///
/// critical error message.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingCritical")]
pub fn loggingCritical(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, critical, message)
}

/// # Safety
///
/// fatal error message.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingFatal")]
pub fn loggingFatal(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, fatal, message)
}

/// # Safety
///
/// exception error message.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.loggingError")]
pub fn loggingException(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, exception, message)
}
