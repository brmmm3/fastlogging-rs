// FFM: No JNI imports needed

use fastlogging::Logger;

use crate::{get_option_str, log_message};

/// # Safety
///
/// This function creates a new instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggerNew(
    level: i32,
    domain_ptr: *const u8,
    domain_len: usize,
) -> *mut Logger {
    let domain = if !domain_ptr.is_null() && domain_len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(domain_ptr, domain_len) };
        match std::str::from_utf8(slice) {
            Ok(s) => s.to_string(),
            Err(_) => return std::ptr::null_mut(),
        }
    } else {
        String::new()
    };
    let logger = Logger::new(level as u8, domain);
    Box::into_raw(Box::new(logger))
}

/// # Safety
///
/// This function creates a new extended instance.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggerNewExt(
    level: i32,
    domain_ptr: *const u8,
    domain_len: usize,
    tname: i32,
    tid: i32,
) -> *mut Logger {
    let domain = if !domain_ptr.is_null() && domain_len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(domain_ptr, domain_len) };
        match std::str::from_utf8(slice) {
            Ok(s) => s.to_string(),
            Err(_) => return std::ptr::null_mut(),
        }
    } else {
        String::new()
    };
    let logger = Logger::new_ext(level as u8, domain, tname != 0, tid != 0);
    Box::into_raw(Box::new(logger))
}

/// # Safety
///
/// Set log level (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggerSetLevel(logger: *mut Logger, level: i32) {
    if let Some(logger) = unsafe { logger.as_mut() } {
        logger.set_level(level as u8);
    }
}

/// # Safety
///
/// Set log domain (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggerSetDomain(
    logger: *mut Logger,
    domain_ptr: *const u8,
    domain_len: usize,
) -> i32 {
    if let Some(logger) = unsafe { logger.as_mut() } {
        let domain = if !domain_ptr.is_null() && domain_len > 0 {
            match get_option_str(domain_ptr, domain_len) {
                Some(s) => s,
                None => return -1,
            }
        } else {
            ""
        };
        logger.set_domain(domain);
        0
    } else {
        -1
    }
}

/// # Safety
///
/// trace message (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggerTrace(
    logger: *mut Logger,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logger, msg_ptr, msg_len, trace)
}

/// # Safety
///
/// debug message (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggerDebug(
    logger: *mut Logger,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logger, msg_ptr, msg_len, debug)
}

/// # Safety
///
/// info message (FFM).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggerInfo(
    logger: *mut Logger,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logger, msg_ptr, msg_len, info)
}

/// # Safety
///
/// trace message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggerSuccess(
    logger: *mut Logger,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logger, msg_ptr, msg_len, success)
}

/// # Safety
///
/// debug message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggerWarning(
    logger: *mut Logger,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logger, msg_ptr, msg_len, warning)
}

/// # Safety
///
/// error message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggerError(
    logger: *mut Logger,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logger, msg_ptr, msg_len, error)
}

/// # Safety
///
/// error message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggerCritical(
    logger: *mut Logger,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logger, msg_ptr, msg_len, critical)
}

/// # Safety
///
/// error message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggerFatal(
    logger: *mut Logger,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logger, msg_ptr, msg_len, fatal)
}

/// # Safety
///
/// error message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn loggerException(
    logger: *mut Logger,
    msg_ptr: *const u8,
    msg_len: usize,
) -> i32 {
    log_message!(logger, msg_ptr, msg_len, exception)
}
