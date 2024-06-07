use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::{jint, jlong};

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
    let instance = Logger::new(level as u8, domain);

    Box::into_raw(Box::new(instance)) as jlong
}

/// # Safety
///
/// Set log level.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerSetLevel(
    mut _env: JNIEnv,
    _class: JClass,
    logger_ptr: jlong,
    level: jint,
) {
    let instance = &mut *(logger_ptr as *mut Logger);

    instance.set_level(level as u8);
}

/// # Safety
///
/// Set log domain.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerSetDomain(
    mut env: JNIEnv,
    _class: JClass,
    logger_ptr: jlong,
    domain: JString,
) {
    let instance = &mut *(logger_ptr as *mut Logger);
    let domain: String = env.get_string(&domain).unwrap().into();

    instance.set_domain(domain);
}

/// # Safety
///
/// trace message.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggerTrace(
    mut env: JNIEnv,
    _class: JClass,
    logger_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logger_ptr as *mut Logger);
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = instance.trace(message) {
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
    logger_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logger_ptr as *mut Logger);
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = instance.debug(message) {
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
    logger_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logger_ptr as *mut Logger);
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = instance.info(message) {
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
    logger_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logger_ptr as *mut Logger);
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = instance.success(message) {
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
    logger_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logger_ptr as *mut Logger);
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = instance.warning(message) {
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
    logger_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logger_ptr as *mut Logger);
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = instance.error(message) {
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
    logger_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logger_ptr as *mut Logger);
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = instance.critical(message) {
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
    logger_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logger_ptr as *mut Logger);
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = instance.fatal(message) {
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
    logger_ptr: jlong,
    message: JString,
) {
    let instance = &mut *(logger_ptr as *mut Logger);
    let message: String = env.get_string(&message).unwrap().into();
    if let Err(err) = instance.exception(message) {
        env.throw(err.to_string()).unwrap();
        unreachable!();
    }
}
