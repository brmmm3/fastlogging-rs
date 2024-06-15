use std::path::{Path, PathBuf};

use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::{jboolean, jbyte, jdouble, jint, jlong, jstring};

use fastlogging::{
    ClientWriterConfig, ConsoleWriterConfig, EncryptionMethod, ExtConfig, FileWriterConfig,
    LevelSyms, Logger, Logging, MessageStructEnum, ServerConfig, WriterConfigEnum, WriterTypeEnum,
};

use crate::{get_string, throw_exception};

/// # Safety
///
/// Create new extended configuration.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingExtConfigNew(
    _env: JNIEnv,
    _class: JClass,
    structured: jint,
    hostname: jboolean,
    pname: jboolean,
    pid: jboolean,
    tname: jboolean,
    tid: jboolean,
) -> Box<ExtConfig> {
    let structured = match structured {
        0 => MessageStructEnum::String,
        1 => MessageStructEnum::Json,
        2 => MessageStructEnum::Xml,
        _ => MessageStructEnum::String,
    };
    Box::new(ExtConfig::new(
        structured,
        hostname != 0,
        pname != 0,
        pid != 0,
        tname != 0,
        tid != 0,
    ))
}

/// # Safety
///
/// Create new default instance.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingInit(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    Box::into_raw(Box::new(fastlogging::logging_init())) as jlong
}

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
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingShutdown(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    now: jboolean,
) {
    if let Err(err) = logging.shutdown(now != 0) {
        throw_exception(&mut env, err.to_string());
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
    writer: *mut WriterTypeEnum,
    level: jint,
) -> jlong {
    let writer = *Box::from_raw(writer);
    match logging.set_level(writer, level as u8) {
        Ok(_) => 0,
        Err(err) => {
            throw_exception(&mut env, err.to_string());
            -1
        }
    }
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
) {
    logging.set_domain(get_string(&mut env, domain));
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
    logging.set_level2sym(level2sym.to_owned());
}

/// # Safety
///
/// Set extended configuration.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSetExtConfig(
    mut _env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    ext_config: &mut ExtConfig,
) {
    logging.set_ext_config(ext_config);
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
        throw_exception(&mut env, err.to_string());
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
    writer: &mut WriterTypeEnum,
) {
    if let Err(err) = logging.remove_writer(writer) {
        throw_exception(&mut env, err.to_string());
    }
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
    timeout: jdouble,
) {
    if let Err(err) = logging.sync(console != 0, file != 0, client != 0, syslog != 0, timeout) {
        throw_exception(&mut env, err.to_string());
    }
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
) {
    if let Err(err) = logging.sync_all(timeout) {
        throw_exception(&mut env, err.to_string());
    }
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
) {
    let path = if !path.is_null() {
        Some(PathBuf::from(get_string(&mut env, path)))
    } else {
        None
    };
    if let Err(err) = logging.rotate(path) {
        throw_exception(&mut env, err.to_string());
    }
}

/// # Safety
///
/// Set server/client encryption
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSetEncryption(
    mut env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
    writer: *mut WriterTypeEnum,
    key: *mut EncryptionMethod,
) {
    let writer = *Box::from_raw(writer);
    let key = *Box::from_raw(key);
    if let Err(err) = logging.set_encryption(writer, key) {
        throw_exception(&mut env, err.to_string());
    }
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
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetServerConfig(
    _env: JNIEnv,
    _class: JClass,
    logging: &mut Logging,
) -> jlong {
    Box::into_raw(Box::new(logging.get_server_config())) as jlong
}

/// # Safety
///
/// Get server configuration
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingGetServerAddress<'a>(
    env: JNIEnv<'a>,
    _class: JClass<'a>,
    logging: &mut Logging,
) -> jstring {
    let address: String = logging.get_server_address().unwrap();
    let address: JString = env.new_string(address).unwrap();
    address.into_raw()
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
    Box::into_raw(Box::new(
        logging.save_config(Path::new(&get_string(&mut env, path))),
    )) as jlong
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
) {
    if let Err(err) = logging.trace(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
    }
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
) {
    if let Err(err) = logging.debug(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
    }
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
) {
    if let Err(err) = logging.info(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
    }
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
) {
    if let Err(err) = logging.success(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
    }
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
) {
    if let Err(err) = logging.warning(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
    }
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
) {
    if let Err(err) = logging.error(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
    }
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
) {
    if let Err(err) = logging.critical(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
    }
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
) {
    if let Err(err) = logging.fatal(get_string(&mut env, message)) {
        throw_exception(&mut env, err.to_string());
    }
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
) {
    let message: String = if message.is_null() {
        "EXCEPTION".to_owned()
    } else {
        get_string(&mut env, message)
    };
    if let Err(err) = logging.exception(message) {
        throw_exception(&mut env, err.to_string());
    }
}
