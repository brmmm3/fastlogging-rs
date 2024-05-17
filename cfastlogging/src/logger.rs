use std::ptr::null;
use std::ffi::{ c_char, CStr };
use std::os::raw::c_uchar;

use fastlogging::Logger;

#[no_mangle]
pub unsafe extern "C" fn logger_new(
    level: c_uchar, // Global log level
    domain: *const c_char
) -> Box<Logger> {
    let domain = if domain != null() {
        let c_str = unsafe { CStr::from_ptr(domain) };
        c_str.to_str().unwrap().to_string()
    } else {
        "".to_string()
    };
    Box::new(Logger::new(level as u8, domain))
}

#[no_mangle]
pub unsafe extern "C" fn logger_set_level(logger: &mut Logger, level: u8) {
    logger.set_level(level);
}

#[no_mangle]
pub unsafe extern "C" fn logger_set_domain(logger: &mut Logger, domain: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(domain) };
    logger.set_domain(c_str.to_str().unwrap().to_string());
}

// Logger calls

#[no_mangle]
pub unsafe extern "C" fn logger_debug(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.debug(c_str.to_str().unwrap().to_string()) {
        eprint!("logger_debug failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logger_info(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.info(c_str.to_str().unwrap().to_string()) {
        eprint!("logger_info failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logger_warning(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.warning(c_str.to_str().unwrap().to_string()) {
        eprint!("logger_warning failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logger_error(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.error(c_str.to_str().unwrap().to_string()) {
        eprint!("logger_error failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logger_critical(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.critical(c_str.to_str().unwrap().to_string()) {
        eprint!("logger_critical failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logger_fatal(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.fatal(c_str.to_str().unwrap().to_string()) {
        eprint!("logger_fatal failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logger_exception(logger: &Logger, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logger.exception(c_str.to_str().unwrap().to_string()) {
        eprint!("logger_exception failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}
