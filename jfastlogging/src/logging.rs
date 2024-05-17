use std::path::PathBuf;

use jni::JNIEnv;

use jni::objects::{ JClass, JString };

use jni::sys::{ jboolean, jdouble, jint, jlong };

use fastlogging::{ LevelSyms, Logger, Logging, NOLOG };

#[inline]
fn throw_exception(env: &mut JNIEnv, error: String) {
    eprintln!("{error}");
    env.throw(error).unwrap();
}

#[inline]
fn get_string(env: &mut JNIEnv, s: JString) -> String {
    match env.get_string(&s) {
        Ok(s) => s.into(),
        Err(err) => {
            throw_exception(env, err.to_string());
            unreachable!();
        }
    }
}

#[inline]
fn get_option_vec_u8(env: &mut JNIEnv, s: JString) -> Option<Vec<u8>> {
    match env.get_string(&s) {
        Ok(k) => {
            let k: String = k.into();
            Some(k.into_bytes())
        }
        Err(_) => None,
    }
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
    console: jboolean, // If true start ConsoleLogging
    file: JString, // If path is defined start FileLogging
    server: JString, // If address is defined start LoggingServer
    connect: JString, // If address is defined start ClientLogging
    max_size: jint, // Maximum size of log files
    backlog: jint // Maximum number of backup log files
) -> jlong {
    let domain: Option<String> = match domain.is_null() {
        true => None,
        false => {
            match env.get_string(&domain) {
                Ok(s) => Some(s.into()),
                Err(err) => {
                    eprintln!("{err:?}");
                    None
                }
            }
        }
    };
    let console: Option<bool> = Some(console != 0);
    let file: Option<PathBuf> = match file.is_null() {
        true => None,
        false => {
            let s: String = env.get_string(&file).unwrap().into();
            Some(PathBuf::from(s))
        }
    };
    let server: Option<String> = match server.is_null() {
        true => None,
        false => Some(env.get_string(&server).unwrap().into()),
    };
    let connect: Option<String> = match connect.is_null() {
        true => None,
        false => Some(env.get_string(&connect).unwrap().into()),
    };
    let max_size: Option<usize> = Some(max_size as usize);
    let backlog: Option<usize> = Some(backlog as usize);
    let instance = Logging::new(
        Some(level as u8),
        domain,
        console,
        file,
        server,
        connect,
        max_size,
        backlog
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
    now: jboolean
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Err(err) = instance.shutdown(Some(now != 0)) {
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
    logger_ptr: jlong
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
    logger_ptr: jlong
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
    mut _env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    level: jint
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    instance.set_level(level as u8);
}

/// # Safety
///
/// Set log domain.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSetDomain(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    domain: JString
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
    level2sym_ptr: jlong
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    let level2sym = &mut *(level2sym_ptr as *mut LevelSyms);

    instance.set_level2sym(level2sym.to_owned());
}

/// # Safety
///
/// Set console logger level
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSetConsoleWriter(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    level: jint
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    if
        let Err(err) = (if level < (NOLOG as i32) {
            instance.set_console_writer(Some(level as u8))
        } else {
            instance.set_console_writer(None)
        })
    {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// This function destroys an instance.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSetConsoleColors(
    mut _env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    colors: jboolean
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    instance.set_console_colors(colors != 0);
}

/// # Safety
///
/// Set console logger level
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSetFileWriter(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    level: jint,
    path: JString,
    max_size: jint,
    backlog: jint
) {
    let instance = &mut *(logging_ptr as *mut Logging);
    if let Ok(path) = env.get_string(&path) {
        let path: String = path.into();
        if
            let Err(err) = instance.set_file_writer(
                Some(level as u8),
                Some(PathBuf::from(path)),
                Some(max_size as usize),
                Some(backlog as usize)
            )
        {
            throw_exception(&mut env, err.to_string());
            unreachable!();
        }
    }
}

/// # Safety
///
/// This function destroys an instance.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingRotate(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    if let Err(err) = instance.rotate() {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// This function destroys an instance.
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSync(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    timeout: jdouble
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    if let Err(err) = instance.sync(timeout as f64) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// Connect to fastlogging server
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingConnect(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    address: JString,
    level: jint,
    key: JString
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    if
        let Err(err) = instance.connect(
            get_string(&mut env, address),
            level as u8,
            get_option_vec_u8(&mut env, key)
        )
    {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// Connect to fastlogging server
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingDisconnect(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    address: JString
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    if let Err(err) = instance.disconnect(&get_string(&mut env, address)) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// Connect to fastlogging server
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSetClientLevel(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    address: JString,
    level: jint
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    if let Err(err) = instance.set_client_level(&get_string(&mut env, address), level as u8) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// Connect to fastlogging server
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSetClientEncryption(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    address: JString,
    key: JString
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    if
        let Err(err) = instance.set_client_encryption(
            &get_string(&mut env, address),
            get_option_vec_u8(&mut env, key)
        )
    {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// Connect to fastlogging server
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingServerStart(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    address: JString,
    level: jint,
    key: JString
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    if
        let Err(err) = instance.server_start(
            get_string(&mut env, address),
            level as u8,
            get_option_vec_u8(&mut env, key)
        )
    {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// Connect to fastlogging server
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingServerShutdown(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    if let Err(err) = instance.server_shutdown() {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// Connect to fastlogging server
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSetServerLevel(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    level: jint
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    if let Err(err) = instance.set_server_level(level as u8) {
        throw_exception(&mut env, err.to_string());
        unreachable!();
    }
}

/// # Safety
///
/// Connect to fastlogging server
#[no_mangle]
pub unsafe extern "system" fn Java_org_logging_FastLogging_loggingSetServerEncryption(
    mut env: JNIEnv,
    _class: JClass,
    logging_ptr: jlong,
    key: JString
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    if let Err(err) = instance.set_server_encryption(get_option_vec_u8(&mut env, key)) {
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
    message: JString
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
    message: JString
) {
    let instance = &mut *(logging_ptr as *mut Logging);

    if let Err(err) = instance.info(get_string(&mut env, message)) {
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
    message: JString
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
    message: JString
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
    message: JString
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
    message: JString
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
    message: JString
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
