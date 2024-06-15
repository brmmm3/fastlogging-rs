use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::{jboolean, jint, jlong};

use fastlogging::Logger;

/// # Safety
///
/// This function creates a new instance.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint, // Global log level
    domain: JString,
) -> jlong {
    let domain: String = env.get_string(&domain).unwrap().into();
    let logger = Logger::new(level as u8, domain);
    Box::into_raw(Box::new(logger)) as jlong
}

/// # Safety
///
/// This function creates a new extended instance.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerNewExt(
    mut env: JNIEnv,
    _class: JClass,
    level: jint, // Global log level
    domain: JString,
    tname: jboolean,
    tid: jboolean,
) -> jlong {
    let domain: String = env.get_string(&domain).unwrap().into();
    let logger = Logger::new_ext(level as u8, domain, tname != 0, tid != 0);
    Box::into_raw(Box::new(logger)) as jlong
}

/// # Safety
///
/// Set log level.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerSetLevel(
    mut _env: JNIEnv,
    _class: JClass,
    logger: &mut Logger,
    level: jint,
) {
    logger.set_level(level as u8);
}

/// # Safety
///
/// Set log domain.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerSetDomain(
    mut env: JNIEnv,
    _class: JClass,
    logger: &mut Logger,
    domain: JString,
) {
    let domain: String = env.get_string(&domain).unwrap().into();
    logger.set_domain(domain);
}

/// # Safety
///
/// trace message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerTrace(
    mut env: JNIEnv,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) {
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = logger.trace(message) {
        env.throw(err.to_string()).unwrap();
        unreachable!();
    }
}

/// # Safety
///
/// debug message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerDebug(
    mut env: JNIEnv,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) {
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = logger.debug(message) {
        env.throw(err.to_string()).unwrap();
        unreachable!();
    }
}

/// # Safety
///
/// debug message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerInfo(
    mut env: JNIEnv,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) {
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = logger.info(message) {
        env.throw(err.to_string()).unwrap();
        unreachable!();
    }
}

/// # Safety
///
/// trace message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerSuccess(
    mut env: JNIEnv,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) {
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = logger.success(message) {
        env.throw(err.to_string()).unwrap();
        unreachable!();
    }
}

/// # Safety
///
/// debug message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerWarning(
    mut env: JNIEnv,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) {
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = logger.warning(message) {
        env.throw(err.to_string()).unwrap();
        unreachable!();
    }
}

/// # Safety
///
/// error message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerError(
    mut env: JNIEnv,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) {
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = logger.error(message) {
        env.throw(err.to_string()).unwrap();
        unreachable!();
    }
}

/// # Safety
///
/// error message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerCritical(
    mut env: JNIEnv,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) {
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = logger.critical(message) {
        env.throw(err.to_string()).unwrap();
        unreachable!();
    }
}

/// # Safety
///
/// error message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerFatal(
    mut env: JNIEnv,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) {
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = logger.fatal(message) {
        env.throw(err.to_string()).unwrap();
        unreachable!();
    }
}

/// # Safety
///
/// error message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerException(
    mut env: JNIEnv,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) {
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = logger.exception(message) {
        env.throw(err.to_string()).unwrap();
        unreachable!();
    }
}
