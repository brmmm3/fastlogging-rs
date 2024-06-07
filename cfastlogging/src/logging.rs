use std::ffi::{c_char, c_double, c_uchar, CStr};
use std::path::PathBuf;
use std::ptr::null;

use fastlogging::{
    ClientWriterConfig, ConsoleWriterConfig, EncryptionMethod, ExtConfig, FileWriterConfig,
    LevelSyms, Logger, Logging, MessageStructEnum, ServerConfig, WriterTypeEnum,
};

use crate::util::{char2string, option_char2string};

#[no_mangle]
pub unsafe extern "C" fn ext_config_new(
    structured: c_uchar,
    hostname: c_uchar,
    pname: c_uchar,
    pid: c_uchar,
    tname: c_uchar,
    tid: c_uchar,
) -> Box<ExtConfig> {
    let structured = match structured {
        0 => MessageStructEnum::String,
        1 => MessageStructEnum::Json,
        2 => MessageStructEnum::Xml,
        _ => MessageStructEnum::String,
    };
    Box::new(ExtConfig::new(
        structured,
        hostname != 0,
        pname != 0,
        pid != 0,
        tname != 0,
        tid != 0,
    ))
}

#[no_mangle]
pub unsafe extern "C" fn logging_init() -> &'static Logging {
    fastlogging::logging_init()
}

/// For further reading ...
/// #[no_mangle] - // https://internals.rust-lang.org/t/precise-semantics-of-no-mangle/4098

#[no_mangle]
pub unsafe extern "C" fn logging_new(
    level: c_char, // Global log level
    domain: *const c_char,
    ext_config: *mut ExtConfig,
    console: *mut ConsoleWriterConfig,
    file: *mut FileWriterConfig,
    server: *mut ServerConfig,
    connect: *mut ClientWriterConfig,
    syslog: c_char,        // Syslog log level
    config: *const c_char, // Optional path to config file
) -> Box<Logging> {
    let level = if level < 0 { None } else { Some(level as u8) };
    let domain = option_char2string(domain);
    let ext_config = if ext_config.is_null() {
        None
    } else {
        Some(*Box::from_raw(ext_config))
    };
    let console = if console.is_null() {
        None
    } else {
        Some(*Box::from_raw(console))
    };
    let file = if file.is_null() {
        None
    } else {
        Some(*Box::from_raw(file))
    };
    let server = if server.is_null() {
        None
    } else {
        Some(*Box::from_raw(server))
    };
    let connect = if connect.is_null() {
        None
    } else {
        Some(*Box::from_raw(connect))
    };
    let syslog = if syslog < 0 { None } else { Some(syslog as u8) };
    Box::new(
        Logging::new(
            level,
            domain,
            ext_config,
            console,
            file,
            server,
            connect,
            syslog,
            option_char2string(config).map(|s| PathBuf::from(s)),
        )
        .unwrap(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn logging_shutdown(logging: &mut Logging, now: u8) -> isize {
    if let Err(err) = logging.shutdown(now != 0) {
        eprintln!("logging_shutdown failed: {err:?}");
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
pub unsafe extern "C" fn logging_set_level(
    logging: &mut Logging,
    writer: *mut WriterTypeEnum,
    level: u8,
) -> isize {
    let writer = *Box::from_raw(writer);
    if let Err(err) = logging.set_level(writer, level) {
        eprintln!("logging_set_level failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_set_domain(logging: &mut Logging, domain: *const c_char) {
    logging.set_domain(char2string(domain));
}

#[no_mangle]
pub unsafe extern "C" fn logging_set_level2sym(logging: &mut Logging, level2sym: u8) {
    logging.set_level2sym(if level2sym == 0 {
        LevelSyms::Sym
    } else if level2sym == 1 {
        LevelSyms::Short
    } else {
        LevelSyms::Str
    });
}

// File writer

#[no_mangle]
pub unsafe extern "C" fn logging_rotate(logging: &Logging, path: *mut PathBuf) -> isize {
    let path = if path.is_null() {
        None
    } else {
        Some(*Box::from_raw(path))
    };
    if let Err(err) = logging.rotate(path) {
        eprintln!("logging_rotate failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_sync(
    logging: &Logging,
    console: c_char,
    file: c_char,
    client: c_char,
    syslog: c_char,
    timeout: c_double,
) -> isize {
    if let Err(err) = logging.sync(
        console != 0,
        file != 0,
        client != 0,
        syslog != 0,
        timeout as f64,
    ) {
        eprintln!("logging_sync failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_sync_all(logging: &Logging, timeout: c_double) -> isize {
    if let Err(err) = logging.sync_all(timeout as f64) {
        eprintln!("logging_sync_all failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

// Network

#[no_mangle]
pub unsafe extern "C" fn logging_set_encryption(
    logging: &mut Logging,
    writer: *mut WriterTypeEnum,
    encryption: c_uchar,
    key: *const c_char,
) -> isize {
    let writer = *Box::from_raw(writer);
    let key = if encryption == 0 || key == null() {
        EncryptionMethod::NONE
    } else {
        let key = (unsafe { CStr::from_ptr(key) })
            .to_str()
            .unwrap()
            .as_bytes()
            .to_vec();
        if encryption == 1 {
            EncryptionMethod::AuthKey(key)
        } else {
            EncryptionMethod::AES(key)
        }
    };
    if let Err(err) = logging.set_encryption(writer, key) {
        eprintln!("logging_set_encryption failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

// Logging calls

#[no_mangle]
pub unsafe extern "C" fn logging_trace(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.trace(char2string(message)) {
        eprintln!("logging_trace failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_debug(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.debug(char2string(message)) {
        eprintln!("logging_debug failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_info(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.info(char2string(message)) {
        eprintln!("logging_info failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_success(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.success(char2string(message)) {
        eprintln!("logging_success failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_warning(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.warning(char2string(message)) {
        eprintln!("logging_warning failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_error(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.error(char2string(message)) {
        eprintln!("logging_error failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_critical(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.critical(char2string(message)) {
        eprintln!("logging_critical failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_fatal(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.fatal(char2string(message)) {
        eprintln!("logging_fatal failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn logging_exception(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.exception(char2string(message)) {
        eprintln!("logging_exception failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}
