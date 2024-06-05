use std::path::{Path, PathBuf};

use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::{jboolean, jbyte, jchar, jdouble, jint, jlong};

use fastlogging::{
    ClientWriterConfig, ConsoleWriterConfig, EncryptionMethod, ExtConfig, FileWriterConfig,
    LevelSyms, Logger, Logging, ServerConfig, WriterTypeEnum,
};

use crate::{get_string, throw_exception};

/// # Safety
///
/// Create new instance.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint, // Global log level
    domain: JString,
    ext_config: *mut ExtConfig,
    console: *mut ConsoleWriterConfig,
    file: *mut FileWriterConfig,
    server: *mut ServerConfig,
    connect: *mut ClientWriterConfig,
    syslog: jbyte,   // Syslog log level
    config: JString, // Optional configuration file path
) -> jlong {
    let domain: Option<String> = match domain.is_null() {
        true => None,
        false => match env.get_string(&domain) {
            Ok(s) => Some(s.into()),
            Err(err) => {
                eprintln!("{err:?}");
                None
            }
        },
    };
    let ext_config = if ext_config.is_null() {
        None
    } else {
        Some(*Box::from_raw(ext_config))
    };
    let console = if console.is_null() {
        None
    } else {
        Some(*Box::from_raw(console))
    };
    let file = if file.is_null() {
        None
    } else {
        Some(*Box::from_raw(file))
    };
    let server = if server.is_null() {
        None
    } else {
        Some(*Box::from_raw(server))
    };
    let connect = if connect.is_null() {
        None
    } else {
        Some(*Box::from_raw(connect))
    };
    let syslog = if syslog >= 0 {
        Some(syslog as u8)
    } else {
        None
    };
    let config: Option<PathBuf> = match config.is_null() {
        false => Some(PathBuf::from(
            env.get_string(&config).unwrap().to_str().unwrap(),
        )),
        true => None,
    };
    let instance = Logging::new(
        Some(level as u8),
        domain,
        ext_config,
        console,
        file,
        server,
        connect,
        syslog,
        config,
    );

    Box::into_raw(Box::new(instance)) as jlong
}

/// # Safety
///
/// This function destroys an instance.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingShutdown(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    now: jboolean,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Err(err) = instance.shutdown(now != 0) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
    let _boxed_logging = Box::from_raw(logging_ptr as *mut Logging);
}

/// # Safety
///
/// Add a Logger instance
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingAddLogger(
    mut _env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    logger_ptr: jlong,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    let logger = &mut *(logger_ptr as *mut Logger);
    instance.add_logger(logger);
}

/// # Safety
///
/// Remove a Logger instance
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingRemoveLogger(
    mut _env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    logger_ptr: jlong,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    let logger = &mut *(logger_ptr as *mut Logger);
    instance.remove_logger(logger);
}

/// # Safety
///
/// Set log level.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSetLevel(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    writer: *mut WriterTypeEnum,
    level: jint,
) -> jlong {
    let instance = &mut *(logging_ptr as *mut Logging);
    let writer = (*Box::from_raw(writer));
    match instance.set_level(writer, level as u8) {
        Ok(_) => 0,
        Err(err) => {
            throw_exception(&mut env, err.to_string());
            unreachable!();
        }
    }
}

/// # Safety
///
/// Set log domain.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSetDomain(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    domain: JString,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    instance.set_domain(get_string(&mut env, domain));
}

/// # Safety
///
/// Set log level symbols.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSetLevel2Sym(
    mut _env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    level2sym_ptr: jlong,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    let level2sym = &mut *(level2sym_ptr as *mut LevelSyms);
    instance.set_level2sym(level2sym.to_owned());
}

/// # Safety
///
/// This function destroys an instance.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSync(
    mut env: JNIEnv,
    _class: JClass,
    console: jboolean,
    file: jboolean,
    client: jboolean,
    syslog: jboolean,
    logging_ptr: jlong,
    timeout: jdouble,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Err(err) = instance.sync(
        console != 0,
        file != 0,
        client != 0,
        syslog != 0,
        timeout as f64,
    ) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// This function destroys an instance.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSyncAll(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    timeout: jdouble,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Err(err) = instance.sync_all(timeout as f64) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// This function destroys an instance.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingRotate(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    path: JString,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    let path = if !path.is_null() {
        Some(PathBuf::from(get_string(&mut env, path)))
    } else {
        None
    };
    if let Err(err) = instance.rotate(path) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// Set server/client encryption
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSetEncryption(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    writer: WriterTypeEnum,
    key: EncryptionMethod,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Err(err) = instance.set_encryption(writer, key) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// Get configuration
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingGetConfig(
    _env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    writer: WriterTypeEnum,
) -> jlong {
    let instance = &mut *(logging_ptr as *mut Logging);
    Box::into_raw(Box::new(instance.get_config(writer))) as jlong
}

/// # Safety
///
/// Get server configuration
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingGetServerConfig(
    _env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
) -> jlong {
    let instance = &mut *(logging_ptr as *mut Logging);
    Box::into_raw(Box::new(instance.get_server_config())) as jlong
}

/// # Safety
///
/// Get server configuration
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingGetServerAuthKey(
    _env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
) -> jlong {
    let instance = &mut *(logging_ptr as *mut Logging);
    Box::into_raw(Box::new(instance.get_server_auth_key())) as jlong
}

/// # Safety
///
/// Get server configuration
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingGetConfigString(
    _env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
) -> jlong {
    let instance = &mut *(logging_ptr as *mut Logging);
    Box::into_raw(Box::new(instance.get_config_string())) as jlong
}

/// # Safety
///
/// Get server configuration
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSaveConfig(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    path: JString,
) -> jlong {
    let instance = &mut *(logging_ptr as *mut Logging);
    Box::into_raw(Box::new(
        instance.save_config(Path::new(&get_string(&mut env, path))),
    )) as jlong
}

/// # Safety
///
/// trace message.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingTrace(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Err(err) = instance.trace(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// debug message.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingDebug(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Err(err) = instance.debug(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// info message.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingInfo(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Err(err) = instance.info(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// success message.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSuccess(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Err(err) = instance.success(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// warning message.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingWarning(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Err(err) = instance.warning(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// error message.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingError(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Err(err) = instance.error(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// critical error message.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingCritical(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Err(err) = instance.critical(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// fatal error message.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingFatal(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Err(err) = instance.fatal(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// exception error message.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingException(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    let message: String = if message.is_null() {
        "EXCEPTION".to_owned()
    } else {
        get_string(&mut env, message)
    };
    if let Err(err) = instance.exception(message) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}
