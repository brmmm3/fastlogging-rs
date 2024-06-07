use std::ops::Add;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::{jboolean, jint, jlong};

use fastlogging::{
    ClientWriterConfig, CompressionMethodEnum, ConsoleWriterConfig, EncryptionMethod,
    FileWriterConfig, ServerConfig, SyslogWriterConfig,
};

#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingConsoleWriterConfigNew(
    _env: JNIEnv,
    _class: JClass,
    level: jint,
    colors: jboolean,
) -> Box<ConsoleWriterConfig> {
    Box::new(ConsoleWriterConfig::new(level as u8, colors != 0))
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingFileWriterConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    path: JString,
    size: jint,
    backlog: jint,
    timeout: jint,
    time: jlong,
    compression: *mut CompressionMethodEnum,
) -> Box<FileWriterConfig> {
    let path: String = env.get_string(&path).unwrap().into();
    let timeout = if timeout < 0 {
        None
    } else {
        Some(Duration::from_secs(timeout as u64))
    };
    let time = if time < 0 {
        None
    } else {
        Some(SystemTime::now().add(Duration::from_secs(time as u64)))
    };
    let compression = if compression.is_null() {
        None
    } else {
        Some(*Box::from_raw(compression))
    };
    Box::new(
        FileWriterConfig::new(
            level as u8,
            PathBuf::from(path),
            size as usize,
            backlog as usize,
            timeout,
            time,
            compression,
        )
        .unwrap(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingClientWriterConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    address: JString,
    encryption: jint,
    key: JString,
) -> Box<ClientWriterConfig> {
    let address: String = env.get_string(&address).unwrap().into();
    let key = if encryption == 0 || key.is_null() {
        EncryptionMethod::NONE
    } else {
        let key: String = env.get_string(&key).unwrap().into();
        if encryption == 1 {
            EncryptionMethod::AuthKey(key.into_bytes())
        } else {
            EncryptionMethod::AES(key.into_bytes())
        }
    };
    Box::new(ClientWriterConfig::new(level as u8, address, key))
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingServerConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    address: JString,
    encryption: jint,
    key: JString,
) -> Box<ServerConfig> {
    let address: String = env.get_string(&address).unwrap().into();
    let key = if encryption == 0 || key.is_null() {
        EncryptionMethod::NONE
    } else {
        let key: String = env.get_string(&key).unwrap().into();
        if encryption == 1 {
            EncryptionMethod::AuthKey(key.into_bytes())
        } else {
            EncryptionMethod::AES(key.into_bytes())
        }
    };
    Box::new(ServerConfig::new(level as u8, address, key))
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_loggingSyslogWriterConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    hostname: JString,
    pname: JString,
    pid: jint,
) -> jlong {
    let hostname: Option<String> = env.get_string(&hostname).ok().map(|s| s.into());
    let pname: String = env.get_string(&pname).unwrap().into();
    Box::into_raw(Box::new(SyslogWriterConfig::new(
        level as u8,
        hostname,
        pname,
        pid as u32,
    ))) as jlong
}
