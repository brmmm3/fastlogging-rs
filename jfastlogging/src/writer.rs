use std::ops::Add;
use std::path::PathBuf;
use std::ptr::null_mut;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::{
    jboolean, jclass, jint, jlong, jmethodID, jobject, JNIInvokeInterface_, JNI_GetCreatedJavaVMs,
    JavaVM,
};

use fastlogging::{
    CallbackWriterConfig, ClientWriterConfig, CompressionMethodEnum, ConsoleWriterConfig,
    EncryptionMethod, FileWriterConfig, ServerConfig, SyslogWriterConfig,
};
use once_cell::sync::Lazy;

use crate::get_string;

#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_consoleWriterConfigNew(
    _env: JNIEnv,
    _class: JClass,
    level: jint,
    colors: jboolean,
) -> jlong {
    let console = ConsoleWriterConfig::new(level as u8, colors != 0);
    Box::into_raw(Box::new(console)) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_fileWriterConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    path: JString,
    size: jint,
    backlog: jint,
    timeout: jint,
    time: jlong,
    compression: jint,
) -> jlong {
    let path: String = get_string!(env, path, 0);
    let timeout = if timeout > 0 {
        Some(Duration::from_secs(timeout as u64))
    } else {
        None
    };
    let time = if time > 0 {
        Some(SystemTime::now().add(Duration::from_secs(time as u64)))
    } else {
        None
    };
    let compression = Some(match compression as i8 {
        0 => CompressionMethodEnum::Store,
        1 => CompressionMethodEnum::Deflate,
        2 => CompressionMethodEnum::Zstd,
        3 => CompressionMethodEnum::Lzma,
        _ => {
            env.throw(format!("Invalid value {compression} for compression."))
                .unwrap();
            return 0;
        }
    });
    let writer = match FileWriterConfig::new(
        level as u8,
        PathBuf::from(path),
        size as usize,
        backlog as usize,
        timeout,
        time,
        compression,
    ) {
        Ok(w) => w,
        Err(err) => {
            env.throw(err.to_string()).unwrap();
            return 0;
        }
    };
    Box::into_raw(Box::new(writer)) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_clientWriterConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    address: JString,
    encryption: jint,
    key: JString,
) -> jlong {
    let address = get_string!(env, address, 0);
    let key = if encryption == 0 || key.is_null() {
        EncryptionMethod::NONE
    } else {
        let key = get_string!(env, key);
        if encryption == 1 {
            EncryptionMethod::AuthKey(key.into_bytes())
        } else {
            EncryptionMethod::AES(key.into_bytes())
        }
    };
    Box::into_raw(Box::new(ClientWriterConfig::new(level as u8, address, key))) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_serverConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    level: jint,
    address: JString,
    encryption: jint,
    key: JString,
) -> jlong {
    let address = get_string!(env, address);
    let key = if encryption == 0 || key.is_null() {
        EncryptionMethod::NONE
    } else {
        let key = get_string!(env, key);
        if encryption == 1 {
            EncryptionMethod::AuthKey(key.into_bytes())
        } else {
            EncryptionMethod::AES(key.into_bytes())
        }
    };
    Box::into_raw(Box::new(ServerConfig::new(level as u8, address, key))) as jlong
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_syslogWriterConfigNew(
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

pub static CALLBACK_JAVA_FUNC: Lazy<Mutex<jlong>> = Lazy::new(|| Mutex::new(0));

pub fn callback_func(
    level: u8,
    domain: String,
    message: String,
) -> Result<(), fastlogging::LoggingError> {
    let callable = *CALLBACK_JAVA_FUNC.lock().unwrap();
    if callable != 0 {
        let mut vms = vec![null_mut(); 1];
        let count = null_mut();
        unsafe {
            JNI_GetCreatedJavaVMs(vms.as_mut_ptr(), 1, count);
        }
        let jvm: *mut *const JNIInvokeInterface_ = vms.first().unwrap().clone();
        println!("CB: jvm={jvm:?}");
        let env: JNIEnv;
        /*let rs: jint = jvm.AttachCurrentThread(jvm, &mut env, 0);
        if unsafe { *count } > 0 {
            let mainClass: jclass = jvm_ptr.FindClass("net/minecraft/client/main/Main");
            let constructor: jmethodID = jni->GetStaticMethodID(mainClass, "Main", "()V");
            jni->CallStaticVoidMethod(mainClass, constructor);
        }*/
    }
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_callbackWriterConfigNew(
    env: JNIEnv,
    _class: JClass,
    level: jint,
    callback: jobject,
) -> jlong {
    let jvm = env.get_java_vm().unwrap();
    println!("jvm={jvm:?}");
    *CALLBACK_JAVA_FUNC.lock().unwrap() = Box::into_raw(Box::new(callback)) as jlong;
    Box::into_raw(Box::new(CallbackWriterConfig::new(
        level as u8,
        Some(Box::new(callback_func)),
    ))) as jlong
}
