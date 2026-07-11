use std::ffi::{CStr, c_char, c_void};
use std::os::raw::c_uchar;

pub type Logger = *mut c_void;

/// # Safety
///
/// Create new logger.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_new(
    level: c_uchar, // Global log level
    domain: *const c_char,
) -> *mut fastlogging::Logger {
    let domain = if !domain.is_null() {
        let c_str = unsafe { CStr::from_ptr(domain) };
        c_str.to_str().unwrap().to_string()
    } else {
        "".to_string()
    };
    Box::into_raw(Box::new(fastlogging::Logger::new(level, domain)))
}

/// # Safety
///
/// Create new logger with extended configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_new_ext(
    level: c_uchar, // Global log level
    domain: *const c_char,
    tname: c_char,
    tid: c_char,
) -> *mut fastlogging::Logger {
    let domain = if !domain.is_null() {
        let c_str = unsafe { CStr::from_ptr(domain) };
        c_str.to_str().unwrap().to_string()
    } else {
        "".to_string()
    };
    Box::into_raw(Box::new(fastlogging::Logger::new_ext(
        level,
        domain,
        tname != 0,
        tid != 0,
    )))
}

/// # Safety
///
/// Set log level.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_set_level(logger: &mut fastlogging::Logger, level: u8) {
    logger.set_level(level);
}

/// # Safety
///
/// Set domain.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_set_domain(
    logger: &mut fastlogging::Logger,
    domain: *const c_char,
) {
    let c_str = unsafe { CStr::from_ptr(domain) };
    logger.set_domain(c_str.to_str().unwrap());
}

// Logger calls

/// # Safety
///
/// trace message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_trace(
    logger: &fastlogging::Logger,
    message: *const c_char,
) -> isize {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_debug(
    logger: &fastlogging::Logger,
    message: *const c_char,
) -> isize {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_info(
    logger: &fastlogging::Logger,
    message: *const c_char,
) -> isize {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_success(
    logger: &fastlogging::Logger,
    message: *const c_char,
) -> isize {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_warning(
    logger: &fastlogging::Logger,
    message: *const c_char,
) -> isize {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_error(
    logger: &fastlogging::Logger,
    message: *const c_char,
) -> isize {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_critical(
    logger: &fastlogging::Logger,
    message: *const c_char,
) -> isize {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_fatal(
    logger: &fastlogging::Logger,
    message: *const c_char,
) -> isize {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn logger_exception(
    logger: &fastlogging::Logger,
    message: *const c_char,
) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.exception(c_str.to_str().unwrap().to_string()) {
        eprintln!("logger_exception failed: {err:?}");
        err.as_int() as isize
    } else {
        0
    }
}
