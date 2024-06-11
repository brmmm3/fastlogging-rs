use std::ffi::{c_char, c_double, c_uchar, CStr, CString};
use std::path::{Path, PathBuf};
use std::ptr::null;

use fastlogging::{
    ClientWriterConfig, ConsoleWriterConfig, EncryptionMethod, ExtConfig, FileWriterConfig,
    LevelSyms, Logger, Logging, MessageStructEnum, ServerConfig, WriterConfigEnum, WriterTypeEnum,
    LOGGING,
};

use crate::util::{char2string, option_char2string};

#[repr(C)]
pub struct CServerConfig {
    level: u8,
    address: *const c_char,
    port: u16,
    key: EncryptionMethod,
}

/// # Safety
///
/// Create extended configuration.
#[no_mangle]
pub unsafe extern "C" fn ext_config_new(
    structured: c_uchar,
    hostname: c_char,
    pname: c_char,
    pid: c_char,
    tname: c_char,
    tid: c_char,
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

/// # Safety
///
/// Create default logging instance.
#[no_mangle]
pub unsafe extern "C" fn logging_init() -> &'static Logging {
    fastlogging::logging_init()
}

/// For further reading ...
/// #[no_mangle] - // https://internals.rust-lang.org/t/precise-semantics-of-no-mangle/4098

/// # Safety
///
/// Create new logging instance.
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
) -> *mut Logging {
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
    Box::into_raw(Box::new(
        Logging::new(
            level,
            domain,
            ext_config,
            console,
            file,
            server,
            connect,
            syslog,
            option_char2string(config).map(PathBuf::from),
        )
        .unwrap(),
    ))
}

/// # Safety
///
/// Shutdown logging.
#[no_mangle]
pub unsafe extern "C" fn logging_shutdown(logging: &mut Logging, now: u8) -> isize {
    let result = if let Err(err) = logging.shutdown(now != 0) {
        eprintln!("logging_shutdown failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    };
    if logging.drop {
        drop(Box::from_raw(logging));
    }
    result
}

/// # Safety
///
/// Add logger.
#[no_mangle]
pub unsafe extern "C" fn logging_add_logger(logging: &mut Logging, logger: &mut Logger) {
    logging.add_logger(logger);
}

/// # Safety
///
/// Remove logger.
#[no_mangle]
pub unsafe extern "C" fn logging_remove_logger(logging: &mut Logging, logger: &mut Logger) {
    logging.remove_logger(logger);
}

/// # Safety
///
/// Set logging level.
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

/// # Safety
///
/// Set logging domain.
#[no_mangle]
pub unsafe extern "C" fn logging_set_domain(logging: &mut Logging, domain: *const c_char) {
    logging.set_domain(char2string(domain));
}

/// # Safety
///
/// Set log level symbols.
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

/// # Safety
///
/// Set extended configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_set_ext_config(logging: &mut Logging, ext_config: &ExtConfig) {
    logging.set_ext_config(ext_config);
}

/// # Safety
///
/// Add writer.
#[no_mangle]
pub unsafe extern "C" fn logging_add_writer(
    logging: &mut Logging,
    writer: &WriterConfigEnum,
) -> isize {
    if let Err(err) = logging.add_writer(writer) {
        eprintln!("logging_add_writer failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

/// # Safety
///
/// Remove writer.
#[no_mangle]
pub unsafe extern "C" fn logging_remove_writer(
    logging: &mut Logging,
    writer: &WriterTypeEnum,
) -> isize {
    if let Err(err) = logging.remove_writer(writer) {
        eprintln!("logging_remove_writer failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

/// # Safety
///
/// Sync specific writers.
#[no_mangle]
pub unsafe extern "C" fn logging_sync(
    logging: &Logging,
    console: c_char,
    file: c_char,
    client: c_char,
    syslog: c_char,
    timeout: c_double,
) -> isize {
    if let Err(err) = logging.sync(console != 0, file != 0, client != 0, syslog != 0, timeout) {
        eprintln!("logging_sync failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

/// # Safety
///
/// Sync all writers.
#[no_mangle]
pub unsafe extern "C" fn logging_sync_all(logging: &Logging, timeout: c_double) -> isize {
    if let Err(err) = logging.sync_all(timeout) {
        eprintln!("logging_sync_all failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

// File writer

/// # Safety
///
/// Rotate file.
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

// Network

/// # Safety
///
/// Set encryption.
#[no_mangle]
pub unsafe extern "C" fn logging_set_encryption(
    logging: &mut Logging,
    writer: *mut WriterTypeEnum,
    encryption: c_uchar,
    key: *const c_char,
) -> isize {
    let writer = *Box::from_raw(writer);
    let key = if encryption == 0 || key.is_null() {
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

// Config

/// # Safety
///
/// Get configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_config(
    logging: &Logging,
    writer: &WriterTypeEnum,
) -> *const WriterConfigEnum {
    match logging.get_config(writer) {
        Ok(config) => &config,
        Err(err) => {
            eprintln!("logging_get_config failed: {err:?}");
            null()
        }
    }
}

/// # Safety
///
/// Get server configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_server_config(logging: &Logging) -> *mut CServerConfig {
    if let Some(config) = logging.get_server_config() {
        Box::into_raw(Box::new(CServerConfig {
            level: config.level,
            address: CString::new(config.address)
                .expect("Error: CString::new()")
                .into_raw(),
            port: config.port,
            key: config.key,
        }))
    } else {
        null::<CServerConfig>() as *mut _
    }
}

/// # Safety
///
/// Get server configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_get_server_address(logging: &Logging) -> *const char {
    if let Some(config) = logging.get_server_config() {
        CString::new(format!("{}:{}", config.address, config.port))
            .expect("Error: CString::new()")
            .into_raw() as *const char
    } else {
        null()
    }
}

/// # Safety
///
/// Get server authentification key.
#[no_mangle]
pub unsafe extern "C" fn logging_get_server_auth_key(logging: &Logging) -> *const c_char {
    logging.get_server_auth_key().as_ptr() as *const c_char
}

/// # Safety
///
/// Get configuration as string.
#[no_mangle]
pub unsafe extern "C" fn logging_get_config_string(logging: &Logging) -> *const c_char {
    logging.get_config_string().as_ptr() as *const c_char
}

/// # Safety
///
/// Save configuration.
#[no_mangle]
pub unsafe extern "C" fn logging_save_config(logging: &Logging, path: *const c_char) -> isize {
    if let Err(err) = logging.save_config(Path::new(&char2string(path))) {
        eprintln!("logging_get_server_config failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EFAULT as i32) as isize
    } else {
        0
    }
}

// Logging calls

/// # Safety
///
/// trace message.
#[no_mangle]
pub unsafe extern "C" fn logging_trace(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.trace(char2string(message)) {
        eprintln!("logging_trace failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

/// # Safety
///
/// debug message.
#[no_mangle]
pub unsafe extern "C" fn logging_debug(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.debug(char2string(message)) {
        eprintln!("logging_debug failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

/// # Safety
///
/// info message.
#[no_mangle]
pub unsafe extern "C" fn logging_info(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.info(char2string(message)) {
        eprintln!("logging_info failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

/// # Safety
///
/// success message.
#[no_mangle]
pub unsafe extern "C" fn logging_success(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.success(char2string(message)) {
        eprintln!("logging_success failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

/// # Safety
///
/// warning message.
#[no_mangle]
pub unsafe extern "C" fn logging_warning(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.warning(char2string(message)) {
        eprintln!("logging_warning failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

/// # Safety
///
/// error message.
#[no_mangle]
pub unsafe extern "C" fn logging_error(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.error(char2string(message)) {
        eprintln!("logging_error failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

/// # Safety
///
/// critical message.
#[no_mangle]
pub unsafe extern "C" fn logging_critical(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.critical(char2string(message)) {
        eprintln!("logging_critical failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

/// # Safety
///
/// fatal message.
#[no_mangle]
pub unsafe extern "C" fn logging_fatal(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.fatal(char2string(message)) {
        eprintln!("logging_fatal failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}

/// # Safety
///
/// exception message.
#[no_mangle]
pub unsafe extern "C" fn logging_exception(logging: &Logging, message: *const c_char) -> isize {
    if let Err(err) = logging.exception(char2string(message)) {
        eprintln!("logging_exception failed: {err:?}");
        err.raw_os_error().unwrap_or(nix::Error::EPIPE as i32) as isize
    } else {
        0
    }
}
