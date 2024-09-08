use std::path::{Path, PathBuf};

use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::{jboolean, jbyte, jdouble, jint, jlong};

use fastlogging::{
    ClientWriterConfig, ConsoleWriterConfig, EncryptionMethod, ExtConfig, FileWriterConfig,
    LevelSyms, Logger, Logging, ServerConfig, WriterConfigEnum, WriterTypeEnum,
};

use crate::{get_pathbuf, get_string, log_message};

/// # Safety
///
/// Create new instance.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingNew(
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
    let domain: Option<String> = env.get_string(&domain).ok().map(|s| s.into());
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
    let config = match config.is_null() {
        false => Some(get_pathbuf!(env, config, 0)),
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
    Box::into_raw(Box::new(instance.unwrap())) as jlong
}

/// # Safety
///
/// This function destroys an instance.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingShutdown(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    now: jboolean,
) {
    if let Err(err) = logging.shutdown(now != 0) {
        env.throw(err.to_string()).unwrap();
    }
    let _boxed_logging = Box::from_raw(logging);
}

/// # Safety
///
/// Set log level.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSetLevel(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    writer: jint,
    key: JString,
    level: jint,
) -> jlong {
    let writer = match writer as i8 {
        0 => WriterTypeEnum::Root,
        1 => WriterTypeEnum::Console,
        2 => WriterTypeEnum::File(get_pathbuf!(env, key)),
        3 => WriterTypeEnum::Client(get_string!(env, key)),
        4 => WriterTypeEnum::Server(get_string!(env, key)),
        5 => WriterTypeEnum::Callback,
        _ => {
            env.throw(format!("Invalid value {writer} for writer."))
                .unwrap();
            return -1;
        }
    };
    if let Err(err) = logging.set_level(&writer, level as u8) {
        env.throw(err.to_string()).unwrap();
        return -1;
    }
    0
}

/// # Safety
///
/// Set log domain.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSetDomain(
    mut env: JNIEnv,
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
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSetLevel2Sym(
    mut _env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    level2sym: &mut LevelSyms,
) {
    logging.set_level2sym(level2sym);
}

/// # Safety
///
/// Set extended configuration.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSetExtConfig(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    ext_config: *mut ExtConfig,
) -> jint {
    if ext_config.is_null() {
        env.throw("ext_config is null").unwrap();
        return -1;
    }
    let ext_config = *Box::from_raw(ext_config);
    logging.set_ext_config(&ext_config);
    0
}

/// # Safety
///
/// Add a Logger instance
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingAddLogger(
    mut _env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    logger: &mut Logger,
) {
    logging.add_logger(logger);
}

/// # Safety
///
/// Remove a Logger instance
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingRemoveLogger(
    mut _env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    logger: &mut Logger,
) {
    logging.remove_logger(logger);
}

/// # Safety
///
/// Add a Writer instance
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingAddWriter(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    writer: &mut WriterConfigEnum,
) {
    if let Err(err) = logging.add_writer(writer) {
        env.throw(err.to_string()).unwrap();
    }
}

/// # Safety
///
/// Remove a Writer instance
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingRemoveWriter(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    writer: jint,
    key: JString,
) -> jint {
    let writer = match writer as i8 {
        0 => WriterTypeEnum::Root,
        1 => WriterTypeEnum::Console,
        2 => WriterTypeEnum::File(get_pathbuf!(env, key)),
        3 => WriterTypeEnum::Client(get_string!(env, key)),
        4 => WriterTypeEnum::Server(get_string!(env, key)),
        5 => WriterTypeEnum::Callback,
        _ => {
            env.throw(format!("Invalid value {writer} for writer."))
                .unwrap();
            return -1;
        }
    };
    if let Err(err) = logging.remove_writer(&writer) {
        env.throw(err.to_string()).unwrap();
        return -1;
    }
    0
}

/// # Safety
///
/// This function destroys an instance.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSync(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    console: jboolean,
    file: jboolean,
    client: jboolean,
    syslog: jboolean,
    callback: jboolean,
    timeout: jdouble,
) -> jint {
    if let Err(err) = logging.sync(
        console != 0,
        file != 0,
        client != 0,
        syslog != 0,
        callback != 0,
        timeout,
    ) {
        env.throw(err.to_string()).unwrap();
        return -1;
    }
    0
}

/// # Safety
///
/// This function destroys an instance.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSyncAll(
    mut env: JNIEnv,
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
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingRotate(
    mut env: JNIEnv,
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
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSetEncryption(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    address: JString,
    method: jint,
    key: JString,
) -> jint {
    let writer = if address.is_null() {
        WriterTypeEnum::Server(get_string!(env, address))
    } else {
        WriterTypeEnum::Client(get_string!(env, address))
    };
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
    if let Err(err) = logging.set_encryption(writer, method) {
        env.throw(err.to_string()).unwrap();
        return -1;
    }
    0
}

/// # Safety
///
/// Get configuration
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetConfig(
    _env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    writer: *mut WriterTypeEnum,
) -> jlong {
    let writer = *Box::from_raw(writer);
    Box::into_raw(Box::new(logging.get_config(&writer))) as jlong
}

/// # Safety
///
/// Get server configuration
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetServerConfig<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
    logging: &mut Logging,
    address: JString,
) -> jlong {
    let address = get_string!(env, address);
    Box::into_raw(Box::new(logging.get_server_config(&address))) as jlong
}

/// # Safety
///
/// Get server configurations
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetServerConfigs(
    _env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_configs())) as jlong
}

/// # Safety
///
/// Get server addresses
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetServerAddresses(
    _env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_addresses())) as jlong
}

/// # Safety
///
/// Get server addresses
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetServerPorts(
    _env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_ports())) as jlong
}

/// # Safety
///
/// Get server configuration
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetServerAuthKey(
    _env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_auth_key())) as jlong
}

/// # Safety
///
/// Get server configuration
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetConfigString(
    _env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_config_string())) as jlong
}

/// # Safety
///
/// Get server configuration
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSaveConfig(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    path: JString,
) -> jlong {
    let path = get_string!(env, path);
    Box::into_raw(Box::new(logging.save_config(Path::new(&path)))) as jlong
}

/// # Safety
///
/// trace message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingTrace(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, trace, message)
}

/// # Safety
///
/// debug message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingDebug(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, debug, message)
}

/// # Safety
///
/// info message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingInfo(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, info, message)
}

/// # Safety
///
/// success message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSuccess(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, success, message)
}

/// # Safety
///
/// warning message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingWarning(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, warning, message)
}

/// # Safety
///
/// error message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingError(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, error, message)
}

/// # Safety
///
/// critical error message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingCritical(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, critical, message)
}

/// # Safety
///
/// fatal error message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingFatal(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, fatal, message)
}

/// # Safety
///
/// exception error message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingException(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    message: JString,
) -> jint {
    log_message!(env, logging, exception, message)
}
