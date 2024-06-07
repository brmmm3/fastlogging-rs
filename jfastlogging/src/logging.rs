use std::ops::Add;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::{jboolean, jbyte, jdouble, jint, jlong};

use fastlogging::{
    ClientWriterConfig, CompressionMethodEnum, ConsoleWriterConfig, EncryptionMethod, ExtConfig,
    FileWriterConfig, LevelSyms, Logger, Logging, MessageStructEnum, ServerConfig,
    SyslogWriterConfig, WriterTypeEnum,
};

use crate::{get_string, throw_exception};

#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingExtConfigNew(
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

#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingConsoleWriterConfigNew(
    _env: JNIEnv,
    _class: JClass,
    level: jint,
    colors: jboolean,
) -> Box<ConsoleWriterConfig> {
    Box::new(ConsoleWriterConfig::new(level as u8, colors != 0))
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingFileWriterConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    path: JString,
    size: jint,
    backlog: jint,
    timeout: jint,
    time: jlong,
    compression: *mut CompressionMethodEnum,
) -> Box<FileWriterConfig> {
    let path: String = env.get_string(&path).unwrap().into();
    let timeout = if timeout < 0 {
        None
    } else {
        Some(Duration::from_secs(timeout as u64))
    };
    let time = if time < 0 {
        None
    } else {
        Some(SystemTime::now().add(Duration::from_secs(time as u64)))
    };
    let compression = if compression.is_null() {
        None
    } else {
        Some(*Box::from_raw(compression))
    };
    Box::new(
        FileWriterConfig::new(
            level as u8,
            PathBuf::from(path),
            size as usize,
            backlog as usize,
            timeout,
            time,
            compression,
        )
        .unwrap(),
    )
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingClientWriterConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    address: JString,
    encryption: jint,
    key: JString,
) -> Box<ClientWriterConfig> {
    let address: String = env.get_string(&address).unwrap().into();
    let key = if encryption == 0 || key.is_null() {
        EncryptionMethod::NONE
    } else {
        let key: String = env.get_string(&key).unwrap().into();
        if encryption == 1 {
            EncryptionMethod::AuthKey(key.into_bytes())
        } else {
            EncryptionMethod::AES(key.into_bytes())
        }
    };
    Box::new(ClientWriterConfig::new(level as u8, address, key))
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingServerConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    address: JString,
    encryption: jint,
    key: JString,
) -> Box<ServerConfig> {
    let address: String = env.get_string(&address).unwrap().into();
    let key = if encryption == 0 || key.is_null() {
        EncryptionMethod::NONE
    } else {
        let key: String = env.get_string(&key).unwrap().into();
        if encryption == 1 {
            EncryptionMethod::AuthKey(key.into_bytes())
        } else {
            EncryptionMethod::AES(key.into_bytes())
        }
    };
    Box::new(ServerConfig::new(level as u8, address, key))
}

#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSyslogWriterConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    hostname: JString,
    pname: JString,
    pid: jint,
) -> jlong {
    let hostname: Option<String> = env.get_string(&hostname).ok().map(|s| s.into());
    let pname: String = env.get_string(&pname).unwrap().into();
    Box::into_raw(Box::new(SyslogWriterConfig::new(
        level as u8,
        hostname,
        pname,
        pid as u32,
    ))) as jlong
}

/// # Safety
///
/// Create new default instance.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingInit(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    Box::into_raw(Box::new(fastlogging::logging_init())) as jlong
}

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
    let writer = *Box::from_raw(writer);
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
    writer: *mut WriterTypeEnum,
    key: *mut EncryptionMethod,
) {
    let writer = *Box::from_raw(writer);
    let key = *Box::from_raw(key);
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
    writer: *mut WriterTypeEnum,
) -> jlong {
    let writer = *Box::from_raw(writer);
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
