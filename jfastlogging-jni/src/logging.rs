use std::path::{Path, PathBuf};

use jni::jni_mangle;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jdouble, jint, jlong};

use fastlogging::{
    EncryptionMethod, ExtConfig, LevelSyms, Logger, Logging, WriterConfigEnum, WriterTypeEnum,
};

use crate::{enter_jni, log_message};

/// # Safety
///
/// Create new default instance.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingNewDefault")]
pub fn loggingNewDefault(_env: jni::EnvUnowned, _class: JClass) -> jlong {
    Box::into_raw(Box::new(Logging::default())) as jlong
}

/// # Safety
///
/// Create new instance.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingNew")]
pub unsafe fn loggingNew(
    env: jni::EnvUnowned,
    _class: JClass,
    level: jint, // Global log level
    domain: JString,
    configs_ptr: jlong,
    ext_config: *mut ExtConfig,
    config_path: JString, // Optional configuration file path
) -> jlong {
    enter_jni(env, |env| {
        let domain = if domain.is_null() {
            "root".to_string()
        } else {
            JString::to_string(&domain)
        };
        let configs = if configs_ptr == 0 {
            None
        } else {
            // Reconstruct the Box<Vec<WriterConfigEnum>> from JNI pointer
            let configs: Box<Vec<WriterConfigEnum>> =
                unsafe { Box::from_raw(configs_ptr as *mut Vec<WriterConfigEnum>) };
            Some(*configs)
        };
        let ext_config = if ext_config.is_null() {
            None
        } else {
            Some(*unsafe { Box::from_raw(ext_config) })
        };
        let config_path = if config_path.is_null() {
            None
        } else {
            Some(PathBuf::from(JString::to_string(&config_path)))
        };
        match Logging::new(level as u8, domain, configs, ext_config, config_path) {
            Ok(instance) => Ok(Box::into_raw(Box::new(instance)) as jlong),
            Err(err) => {
                env.throw(err.to_string())?;
                Ok(0)
            }
        }
    })
}

/// # Safety
///
/// This function destroys an instance.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingShutdown")]
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
#[jni_mangle("org.logging.FastLogging", "loggingSetLevel")]
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
#[jni_mangle("org.logging.FastLogging", "loggingSetDomain")]
pub fn loggingSetDomain(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    domain: JString,
) -> jint {
    enter_jni(env, |_env| {
        let domain: String = JString::to_string(&domain);
        logging.set_domain(&domain);
        Ok(0)
    })
}

/// # Safety
///
/// Set log level symbols.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingSetLevel2Sym")]
pub fn loggingSetLevel2Sym(
    _env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    level2sym: jint,
) {
    let level2sym = match level2sym {
        0 => LevelSyms::Sym,
        1 => LevelSyms::Short,
        2 => LevelSyms::Str,
        _ => LevelSyms::Sym,
    };
    logging.set_level2sym(&level2sym);
}

/// # Safety
///
/// Set extended configuration.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingSetExtConfig")]
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
#[jni_mangle("org.logging.FastLogging", "loggingAddLogger")]
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
#[jni_mangle("org.logging.FastLogging", "loggingRemoveLogger")]
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
#[jni_mangle("org.logging.FastLogging", "loggingAddWriterConfig")]
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
#[jni_mangle("org.logging.FastLogging", "loggingAddWriter")]
pub fn loggingAddWriter(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    writer_ptr: jlong,
) -> jint {
    enter_jni(env, |env| {
        if writer_ptr == 0 {
            return Ok(0);
        }
        let config: Box<WriterConfigEnum> =
            unsafe { Box::from_raw(writer_ptr as *mut WriterConfigEnum) };
        match logging.add_writer_config(&config) {
            Ok(v) => Ok(v as jint),
            Err(err) => {
                env.throw(err.to_string())?;
                Ok(-1)
            }
        }
    })
}

/// # Safety
///
/// Remove a Writer instance
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingRemoveWriter")]
pub fn loggingRemoveWriter(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    wid: jint,
    key: JString,
) {
    enter_jni(env, |_env| {
        let _key = if key.is_null() {
            None
        } else {
            Some(JString::to_string(&key))
        };
        logging.remove_writer(wid as usize);
        Ok(())
    });
}

/// # Safety
///
/// Add a Writer instance
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingAddWriterConfigs")]
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
/// Sync specified writer types.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingSync")]
pub fn loggingSync(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    console: jboolean,
    file: jboolean,
    client: jboolean,
    syslog: jboolean,
    timeout: jdouble,
) -> jint {
    enter_jni(env, |env| {
        let mut types = Vec::new();
        if console {
            types.push(WriterTypeEnum::Console);
        }
        if file {
            types.push(WriterTypeEnum::File(String::new()));
        }
        if client {
            types.push(WriterTypeEnum::Client(String::new()));
        }
        if syslog {
            types.push(WriterTypeEnum::Syslog);
        }
        if let Err(err) = logging.sync(types, timeout) {
            env.throw(err.to_string())?;
            return Ok(-1);
        }
        Ok(0)
    })
}

/// # Safety
///
/// This function destroys an instance.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingSyncAll")]
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
#[jni_mangle("org.logging.FastLogging", "loggingRotate")]
pub fn loggingRotate(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    path: JString,
) -> jint {
    enter_jni(env, |env| {
        let path: Option<PathBuf> = if path.is_null() {
            None
        } else {
            Some(PathBuf::from(JString::to_string(&path)))
        };
        if let Err(err) = logging.rotate(path) {
            env.throw(err.to_string())?;
            return Ok(-1);
        }
        Ok(0)
    })
}

/// # Safety
///
/// Set server/client encryption
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingSetEncryption")]
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
#[jni_mangle("org.logging.FastLogging", "loggingGetWriterConfig")]
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
#[jni_mangle("org.logging.FastLogging", "loggingGetServerConfig")]
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
#[jni_mangle("org.logging.FastLogging", "loggingGetServerConfigs")]
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
#[jni_mangle("org.logging.FastLogging", "loggingGetServerAddresses")]
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
#[jni_mangle("org.logging.FastLogging", "loggingGetServerPorts")]
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
/// # Safety
///
/// Get server auth key.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingGetServerAuthKey")]
pub fn loggingGetServerAuthKey(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
) -> jni::sys::jobject {
    enter_jni(env, |env| {
        let key = logging.get_server_auth_key();
        let key_str = match key {
            EncryptionMethod::NONE => "NONE".to_string(),
            EncryptionMethod::AuthKey(k) => format!("AuthKey({})", String::from_utf8_lossy(&k)),
            EncryptionMethod::AES(k) => format!("AES({})", String::from_utf8_lossy(&k)),
        };
        let jstr = env.new_string(&key_str)?;
        Ok(jstr.into_raw())
    })
}

/// # Safety
///
/// Get server configuration
/// # Safety
///
/// Get config string.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingGetConfigString")]
pub fn loggingGetConfigString(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
) -> jni::sys::jobject {
    enter_jni(env, |env| {
        let cfg = logging.get_config_string();
        let jstr = env.new_string(&cfg)?;
        Ok(jstr.into_raw())
    })
}

/// # Safety
///
/// Save configuration to file.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingSaveConfig")]
pub fn loggingSaveConfig(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    path: JString,
) {
    enter_jni(env, |env| {
        let path: String = JString::to_string(&path);
        if let Err(err) = logging.save_config(Some(Path::new(&path))) {
            env.throw(err.to_string())?;
        }
        Ok(())
    });
}

/// # Safety
///
/// trace message.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggingTrace")]
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
#[jni_mangle("org.logging.FastLogging", "loggingDebug")]
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
#[jni_mangle("org.logging.FastLogging", "loggingInfo")]
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
#[jni_mangle("org.logging.FastLogging", "loggingSuccess")]
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
#[jni_mangle("org.logging.FastLogging", "loggingWarning")]
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
#[jni_mangle("org.logging.FastLogging", "loggingError")]
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
#[jni_mangle("org.logging.FastLogging", "loggingCritical")]
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
#[jni_mangle("org.logging.FastLogging", "loggingFatal")]
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
#[jni_mangle("org.logging.FastLogging", "loggingException")]
pub fn loggingException(
    env: jni::EnvUnowned,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, exception, message)
}
