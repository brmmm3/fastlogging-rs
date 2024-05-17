use std::path::PathBuf;
use std::ptr::null;
use std::sync::OnceLock;
use std::ffi::{ c_char, c_double, CStr };
use std::os::raw::{ c_uchar, c_int };

use fastlogging::{ LevelSyms, Logger, Logging };

pub static LOGGING: OnceLock<Logging> = OnceLock::new();

#[no_mangle]
pub unsafe extern "C" fn logging_init() -> &'static Logging {
    match LOGGING.get() {
        Some(l) => { l }
        None => {
            LOGGING.set(
                Logging::new(None, None, Some(true), None, None, None, None, None).unwrap()
            ).unwrap();
            LOGGING.get().unwrap()
        }
    }
}

/// For further reading ...
/// #[no_mangle] - // https://internals.rust-lang.org/t/precise-semantics-of-no-mangle/4098

#[no_mangle]
pub unsafe extern "C" fn logging_new(
    level: c_uchar, // Global log level
    domain: *const c_char,
    console: c_int, // If true start ConsoleLogging
    file: *const c_char, // If path is defined start FileLogging
    server: *const c_char, // If address is defined start LoggingServer
    connect: *const c_char, // If address is defined start ClientLogging
    max_size: c_int, // Maximum size of log files
    backlog: c_int // Maximum number of backup log files
) -> Box<Logging> {
    let domain = if domain != null() {
        let c_str = unsafe { CStr::from_ptr(domain) };
        Some(c_str.to_str().unwrap().to_string())
    } else {
        None
    };
    let file: Option<PathBuf> = if file != null() {
        let c_str = unsafe { CStr::from_ptr(file) };
        Some(PathBuf::from(c_str.to_str().unwrap().to_string()))
    } else {
        None
    };
    let server = if server != null() {
        let c_str = unsafe { CStr::from_ptr(server) };
        Some(c_str.to_str().unwrap().to_string())
    } else {
        None
    };
    let connect = if connect != null() {
        let c_str = unsafe { CStr::from_ptr(connect) };
        Some(c_str.to_str().unwrap().to_string())
    } else {
        None
    };
    let max_size = if max_size < 0 { None } else { Some(max_size as usize) };
    let backlog = if backlog < 0 { None } else { Some(backlog as usize) };
    Box::new(
        Logging::new(
            Some(level as u8),
            domain,
            Some(console != 0),
            file,
            server,
            connect,
            max_size,
            backlog
        ).unwrap()
    )
}

#[no_mangle]
pub unsafe extern "C" fn logging_shutdown(logging: &mut Logging, now: u8) -> isize {
    if let Err(err) = logging.shutdown(Some(now != 0)) {
        eprint!("logging_shutdown failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_add_logger(logging: &mut Logging, logger: &mut Logger) {
    logging.add_logger(logger);
}

#[no_mangle]
pub unsafe extern "C" fn logging_remove_logger(logging: &mut Logging, logger: &mut Logger) {
    logging.remove_logger(logger);
}

#[no_mangle]
pub unsafe extern "C" fn logging_set_level(logging: &mut Logging, level: u8) {
    logging.set_level(level);
}

#[no_mangle]
pub unsafe extern "C" fn logging_set_domain(logging: &mut Logging, domain: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(domain) };
    logging.set_domain(c_str.to_str().unwrap().to_string());
}

#[no_mangle]
pub unsafe extern "C" fn logging_set_level2sym(logging: &mut Logging, level2sym: u8) {
    logging.set_level2sym(
        if level2sym == 0 {
            LevelSyms::Sym
        } else if level2sym == 1 {
            LevelSyms::Short
        } else {
            LevelSyms::Str
        }
    );
}

// Console writer

#[no_mangle]
pub unsafe extern "C" fn logging_set_console_writer(logging: &mut Logging, level: i8) -> isize {
    if let Err(err) = logging.set_console_writer(if level < 0 { None } else { Some(level as u8) }) {
        eprint!("logging_set_console_writer failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_set_console_colors(logging: &mut Logging, colors: u8) {
    logging.set_console_colors(colors != 0);
}

// File writer

#[no_mangle]
pub unsafe extern "C" fn logging_set_file_writer(
    logging: &mut Logging,
    level: i8,
    path: *const c_char,
    max_size: c_int,
    backlog: c_int
) -> isize {
    let level = if level < 0 { None } else { Some(level as u8) };
    let path: Option<PathBuf> = if path != null() {
        let c_str = unsafe { CStr::from_ptr(path) };
        Some(PathBuf::from(c_str.to_str().unwrap().to_string()))
    } else {
        None
    };
    let max_size = if max_size < 0 { None } else { Some(max_size as usize) };
    let backlog = if backlog < 0 { None } else { Some(backlog as usize) };
    if let Err(err) = logging.set_file_writer(level, path, max_size, backlog) {
        eprint!("logging_set_file_writer failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_rotate(logging: &Logging) -> isize {
    if let Err(err) = logging.rotate() {
        eprint!("logging_rotate failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_sync(logging: &Logging, timeout: c_double) -> isize {
    if let Err(err) = logging.sync(timeout as f64) {
        eprint!("logging_sync failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

// Network client

#[no_mangle]
pub unsafe extern "C" fn logging_connect(
    logging: &mut Logging,
    address: *const c_char,
    level: c_uchar,
    key: *const c_char
) -> isize {
    let address = (unsafe { CStr::from_ptr(address) }).to_str().unwrap();
    let key = if key == null() {
        None
    } else {
        Some((unsafe { CStr::from_ptr(key) }).to_str().unwrap().as_bytes().to_vec())
    };
    if let Err(err) = logging.connect(address, level as u8, key) {
        eprint!("logging_connect failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::ETIMEDOUT as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_disconnect(
    logging: &mut Logging,
    address: *const c_char
) -> isize {
    let address = (unsafe { CStr::from_ptr(address) }).to_str().unwrap();
    if let Err(err) = logging.disconnect(address) {
        eprint!("logging_disconnect failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EIO as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_set_client_level(
    logging: &mut Logging,
    address: *const c_char,
    level: c_uchar
) -> isize {
    let address = (unsafe { CStr::from_ptr(address) }).to_str().unwrap();
    if let Err(err) = logging.set_client_level(address, level as u8) {
        eprint!("logging_set_client_level failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_set_client_encryption(
    logging: &mut Logging,
    address: *const c_char,
    key: *const c_char
) -> isize {
    let address = (unsafe { CStr::from_ptr(address) }).to_str().unwrap();
    let key = if key == null() {
        None
    } else {
        Some((unsafe { CStr::from_ptr(key) }).to_str().unwrap().as_bytes().to_vec())
    };
    if let Err(err) = logging.set_client_encryption(address, key) {
        eprint!("logging_set_client_encryption failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

// Network server

#[no_mangle]
pub unsafe extern "C" fn logging_server_start(
    logging: &mut Logging,
    address: *const c_char,
    level: c_uchar,
    key: *const c_char
) -> isize {
    let address = (unsafe { CStr::from_ptr(address) }).to_str().unwrap();
    let key = if key == null() {
        None
    } else {
        Some((unsafe { CStr::from_ptr(key) }).to_str().unwrap().as_bytes().to_vec())
    };
    if let Err(err) = logging.server_start(address, level as u8, key) {
        eprint!("logging_server_start failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::ETIMEDOUT as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_server_shutdown(logging: &mut Logging) -> isize {
    if let Err(err) = logging.server_shutdown() {
        eprint!("logging_server_shutdown failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::ETIMEDOUT as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_set_server_level(logging: &mut Logging, level: c_uchar) -> isize {
    if let Err(err) = logging.set_server_level(level as u8) {
        eprint!("logging_set_server_level failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_set_server_encryption(
    logging: &mut Logging,
    key: *const c_char
) -> isize {
    let key = if key == null() {
        None
    } else {
        Some((unsafe { CStr::from_ptr(key) }).to_str().unwrap().as_bytes().to_vec())
    };
    if let Err(err) = logging.set_server_encryption(key) {
        eprint!("logging_set_server_encryption failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

// Logging calls

#[no_mangle]
pub unsafe extern "C" fn logging_debug(logging: &Logging, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logging.debug(c_str.to_str().unwrap().to_string()) {
        eprint!("logging_debug failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_info(logging: &Logging, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logging.info(c_str.to_str().unwrap().to_string()) {
        eprint!("logging_info failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_warning(logging: &Logging, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logging.warning(c_str.to_str().unwrap().to_string()) {
        eprint!("logging_warning failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_error(logging: &Logging, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logging.error(c_str.to_str().unwrap().to_string()) {
        eprint!("logging_error failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_critical(logging: &Logging, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logging.critical(c_str.to_str().unwrap().to_string()) {
        eprint!("logging_critical failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_fatal(logging: &Logging, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logging.fatal(c_str.to_str().unwrap().to_string()) {
        eprint!("logging_fatal failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_exception(logging: &Logging, message: *const c_char) -> isize {
    let c_str = unsafe { CStr::from_ptr(message) };
    if let Err(err) = logging.exception(c_str.to_str().unwrap().to_string()) {
        eprint!("logging_exception failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}
