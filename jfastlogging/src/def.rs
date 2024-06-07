use std::path::PathBuf;
use std::time::{Duration, UNIX_EPOCH};

use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::{jboolean, jint, jlong};

use fastlogging::{
    ClientWriterConfig, CompressionMethodEnum, ConsoleWriterConfig, EncryptionMethod, ExtConfig,
    FileWriterConfig, MessageStructEnum, ServerConfig, SyslogWriterConfig,
};

#[inline]
pub fn throw_exception(env: &mut JNIEnv, error: String) {
    eprintln!("{error}");
    env.throw(error).unwrap();
}

#[inline]
pub fn get_string(env: &mut JNIEnv, s: JString) -> String {
    match env.get_string(&s) {
        Ok(s) => s.into(),
        Err(err) => {
            throw_exception(env, err.to_string());
            unreachable!();
        }
    }
}

#[inline]
pub fn get_option_vec_u8(env: &mut JNIEnv, s: JString) -> Option<Vec<u8>> {
    match env.get_string(&s) {
        Ok(k) => {
            let k: String = k.into();
            Some(k.into_bytes())
        }
        Err(_) => None,
    }
}

/// # Safety
///
/// Create new instance.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_defExtConfigNew(
    _env: JNIEnv,
    _class: JClass,
    structured: *mut MessageStructEnum,
    hostname: jint,
    pname: jint,
    pid: jint,
    tname: jint,
    tid: jint,
) -> jlong {
    let structured = *Box::from_raw(structured);
    let instance = ExtConfig::new(
        structured,
        hostname != 0,
        pname != 0,
        pid != 0,
        tname != 0,
        tid != 0,
    );
    Box::into_raw(Box::new(instance)) as jlong
}

/// # Safety
///
/// Create new instance.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_defConsoleWriterConfigNew(
    _env: JNIEnv,
    _class: JClass,
    level: jint,
    colors: jboolean,
) -> jlong {
    let instance = ConsoleWriterConfig::new(level as u8, colors != 0);
    Box::into_raw(Box::new(instance)) as jlong
}

/// # Safety
///
/// Create new instance.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_defFileWriterConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    path: JString,
    size: jint,
    backlog: jint,
    timeout: jint,
    time: jlong,
    compression: *mut CompressionMethodEnum,
) -> jlong {
    let compression = if compression.is_null() {
        None
    } else {
        Some(*Box::from_raw(compression))
    };
    let timeout = if timeout > 0 {
        Some(Duration::from_secs(timeout as u64))
    } else {
        None
    };
    let time = if time > 0 {
        Some(UNIX_EPOCH + Duration::from_secs(time as u64))
    } else {
        None
    };
    let instance = FileWriterConfig::new(
        level as u8,
        PathBuf::from(get_string(&mut env, path)),
        size as usize,
        backlog as usize,
        timeout,
        time,
        compression,
    );
    Box::into_raw(Box::new(instance)) as jlong
}

/// # Safety
///
/// Create new instance.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_defClientWriterConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    address: JString,
    key: *mut EncryptionMethod,
) -> jlong {
    let key = *Box::from_raw(key);
    let instance = ClientWriterConfig::new(level as u8, get_string(&mut env, address), key);
    Box::into_raw(Box::new(instance)) as jlong
}

/// # Safety
///
/// Create new instance.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_defServerConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    address: JString,
    key: *mut EncryptionMethod,
) -> jlong {
    let key = *Box::from_raw(key);
    let instance = ServerConfig::new(level as u8, get_string(&mut env, address), key);
    Box::into_raw(Box::new(instance)) as jlong
}

/// # Safety
///
/// Create new instance.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_defSyslogWriterConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    hostname: JString,
    pname: JString,
    pid: jint,
) -> jlong {
    let hostname = if !hostname.is_null() {
        Some(get_string(&mut env, hostname))
    } else {
        None
    };
    let instance = SyslogWriterConfig::new(
        level as u8,
        hostname,
        get_string(&mut env, pname),
        pid as u32,
    );
    Box::into_raw(Box::new(instance)) as jlong
}
