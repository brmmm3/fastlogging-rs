use jni::jni_mangle;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jint, jlong};

use fastlogging::Logger;

use crate::{enter_jni, log_message};

/// # Safety
///
/// This function creates a new instance.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggerNew")]
pub fn loggerNew(
    env: jni::EnvUnowned,
    _class: JClass,
    level: jint, // Global log level
    domain: JString,
) -> jlong {
    enter_jni(env, |env| {
        let domain: String = match domain.try_to_string(env) {
            Ok(s) => s,
            Err(err) => {
                env.throw(err.to_string()).unwrap();
                return Ok(0);
            }
        };
        let logger = Logger::new(level as u8, domain);
        Ok(Box::into_raw(Box::new(logger)) as jlong)
    })
}

/// # Safety
///
/// This function creates a new extended instance.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggerNewExt")]
pub fn loggerNewExt(
    env: jni::EnvUnowned,
    _class: JClass,
    level: jint, // Global log level
    domain: JString,
    tname: jboolean,
    tid: jboolean,
) -> jlong {
    enter_jni(env, |env| {
        let domain: String = match domain.try_to_string(env) {
            Ok(s) => s,
            Err(err) => {
                env.throw(err.to_string())?;
                return Ok(0);
            }
        };
        let logger = Logger::new_ext(level as u8, domain, tname, tid);
        Ok(Box::into_raw(Box::new(logger)) as jlong)
    })
}

/// # Safety
///
/// Set log level.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggerSetLevel")]
pub fn loggerSetLevel(_env: jni::EnvUnowned, _class: JClass, logger: &mut Logger, level: jint) {
    logger.set_level(level as u8);
}

/// # Safety
///
/// Set log domain.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggerSetDomain")]
pub fn loggerSetDomain(
    env: jni::EnvUnowned,
    _class: JClass,
    logger: &mut Logger,
    domain: JString,
) -> jint {
    enter_jni(env, |_env| {
        let domain: String = JString::to_string(&domain);
        logger.set_domain(&domain);
        Ok(0)
    })
}

/// # Safety
///
/// trace message.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggerTrace")]
pub fn loggerTrace(
    env: jni::EnvUnowned,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) -> jint {
    enter_jni(env, |env| {
        let message: String = JString::to_string(&message);
        if let Err(err) = logger.trace(&message) {
            env.throw(err.to_string()).unwrap();
            return Ok(-1);
        }
        Ok(0)
    })
}

/// # Safety
///
/// debug message.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggerDebug")]
pub fn loggerDebug(
    env: jni::EnvUnowned,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) -> jint {
    log_message!(env, logger, debug, message)
}

/// # Safety
///
/// debug message.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggerInfo")]
pub fn loggerInfo(
    env: jni::EnvUnowned,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) -> jint {
    log_message!(env, logger, info, message)
}

/// # Safety
///
/// trace message.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggerSuccess")]
pub fn loggerSuccess(
    env: jni::EnvUnowned,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) -> jint {
    log_message!(env, logger, success, message)
}

/// # Safety
///
/// debug message.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggerWarning")]
pub fn loggerWarning(
    env: jni::EnvUnowned,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) -> jint {
    log_message!(env, logger, warning, message)
}

/// # Safety
///
/// error message.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggerError")]
pub fn loggerError(
    env: jni::EnvUnowned,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) -> jint {
    log_message!(env, logger, error, message)
}

/// # Safety
///
/// error message.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggerCritical")]
pub fn loggerCritical(
    env: jni::EnvUnowned,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) -> jint {
    log_message!(env, logger, critical, message)
}

/// # Safety
///
/// error message.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggerFatal")]
pub fn loggerFatal(
    env: jni::EnvUnowned,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) -> jint {
    log_message!(env, logger, fatal, message)
}

/// # Safety
///
/// error message.
#[allow(non_snake_case)]
#[jni_mangle("org.logging.FastLogging", "loggerException")]
pub fn loggerException(
    env: jni::EnvUnowned,
    _class: JClass,
    logger: &mut Logger,
    message: JString,
) -> jint {
    log_message!(env, logger, exception, message)
}
