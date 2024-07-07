use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::{jboolean, jint, jlong};

use fastlogging::Logger;

use crate::log_message;

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
    let domain: String = match env.get_string(&domain) {
        Ok(s) => s.into(),
        Err(err) => {
            env.throw(err.to_string()).unwrap();
            return 0;
        }
    };
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
    let domain: String = match env.get_string(&domain) {
        Ok(s) => s.into(),
        Err(err) => {
            env.throw(err.to_string()).unwrap();
            return 0;
        }
    };
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
) -> jint {
    let domain: String = match env.get_string(&domain) {
        Ok(s) => s.into(),
        Err(err) => {
            env.throw(err.to_string()).unwrap();
            return -1;
        }
    };
    logger.set_domain(domain);
    0
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
) -> jint {
    log_message!(env, logger, trace, message)
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
) -> jint {
    log_message!(env, logger, debug, message)
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
) -> jint {
    log_message!(env, logger, info, message)
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
) -> jint {
    log_message!(env, logger, success, message)
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
) -> jint {
    log_message!(env, logger, warning, message)
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
) -> jint {
    log_message!(env, logger, error, message)
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
) -> jint {
    log_message!(env, logger, critical, message)
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
) -> jint {
    log_message!(env, logger, fatal, message)
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
) -> jint {
    log_message!(env, logger, exception, message)
}
