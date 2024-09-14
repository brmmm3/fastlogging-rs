use std::ffi::{c_char, CStr};
use std::os::raw::c_uchar;

use fastlogging::Logger;

/// # Safety
///
/// Create new logger.
#[no_mangle]
pub unsafe extern "C" fn logger_new(
    level: c_uchar, // Global log level
    domain: *const c_char,
) -> *mut Logger {
    let domain = if !domain.is_null() {
        let c_str = unsafe { CStr::from_ptr(domain) };
        c_str.to_str().unwrap().to_string()
    } else {
        "".to_string()
    };
    Box::into_raw(Box::new(Logger::new(level, domain)))
}

/// # Safety
///
/// Create new logger with extended configuration.
#[no_mangle]
pub unsafe extern "C" fn logger_new_ext(
    level: c_uchar, // Global log level
    domain: *const c_char,
    tname: c_char,
    tid: c_char,
) -> *mut Logger {
    let domain = if !domain.is_null() {
        let c_str = unsafe { CStr::from_ptr(domain) };
        c_str.to_str().unwrap().to_string()
    } else {
        "".to_string()
    };
    Box::into_raw(Box::new(Logger::new_ext(
        level,
        domain,
        tname != 0,
        tid != 0,
    )))
}

/// # Safety
///
/// Set log level.
#[no_mangle]
pub unsafe extern "C" fn logger_set_level(logger: &mut Logger, level: u8) {
    logger.set_level(level);
}

/// # Safety
///
/// Set domain.
#[no_mangle]
pub unsafe extern "C" fn logger_set_domain(logger: &mut Logger, domain: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(domain) };
    logger.set_domain(c_str.to_str().unwrap().to_string());
}

// Logger calls

/// # Safety
///
/// trace message.
#[no_mangle]
pub unsafe extern "C" fn logger_trace(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.trace(c_str.to_str().unwrap().to_string()) {
        eprintln!("logger_trace failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// debug message.
#[no_mangle]
pub unsafe extern "C" fn logger_debug(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.debug(c_str.to_str().unwrap().to_string()) {
        eprintln!("logger_debug failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// info message.
#[no_mangle]
pub unsafe extern "C" fn logger_info(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.info(c_str.to_str().unwrap().to_string()) {
        eprintln!("logger_info failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// success message.
#[no_mangle]
pub unsafe extern "C" fn logger_success(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.success(c_str.to_str().unwrap().to_string()) {
        eprintln!("logger_success failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// warning message.
#[no_mangle]
pub unsafe extern "C" fn logger_warning(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.warning(c_str.to_str().unwrap().to_string()) {
        eprintln!("logger_warning failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// error message.
#[no_mangle]
pub unsafe extern "C" fn logger_error(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.error(c_str.to_str().unwrap().to_string()) {
        eprintln!("logger_error failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// critical message.
#[no_mangle]
pub unsafe extern "C" fn logger_critical(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.critical(c_str.to_str().unwrap().to_string()) {
        eprintln!("logger_critical failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// fatal message.
#[no_mangle]
pub unsafe extern "C" fn logger_fatal(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.fatal(c_str.to_str().unwrap().to_string()) {
        eprintln!("logger_fatal failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}

/// # Safety
///
/// exception message.
#[no_mangle]
pub unsafe extern "C" fn logger_exception(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.exception(c_str.to_str().unwrap().to_string()) {
        eprintln!("logger_exception failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}
