use std::ops::Add;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use jni::objects::{JClass, JObject, JString};
use jni::refs::Global;
use jni::sys::{jboolean, jint, jlong};
use jni::vm::JavaVM;
use jni::{JValue, jni_mangle, jni_sig, jni_str};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;

use fastlogging::{
    CallbackWriterConfig, ClientWriterConfig, CompressionMethodEnum, ConsoleWriterConfig,
    EncryptionMethod, FileWriterConfig, ServerConfig, SyslogWriterConfig,
};

use crate::enter_jni;

static GLOBAL_JVM: OnceCell<JavaVM> = OnceCell::new();

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub fn JNI_OnLoad(vm: JavaVM, _reserved: *mut std::os::raw::c_void) -> jint {
    GLOBAL_JVM.set(vm).expect("Failed to set GLOBAL_JVM once.");
    jni::sys::JNI_VERSION_1_8
}

#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.consoleWriterConfigNew")]
pub fn consoleWriterConfigNew(
    _env: jni::EnvUnowned,
    _class: JClass,
    level: jint,
    colors: jboolean,
) -> jlong {
    let console = ConsoleWriterConfig::new(level as u8, colors);
    Box::into_raw(Box::new(console)) as jlong
}

#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.fileWriterConfigNew")]
pub fn fileWriterConfigNew(
    env: jni::EnvUnowned,
    _class: JClass,
    level: jint,
    path: JString,
    size: jint,
    backlog: jint,
    timeout: jint,
    time: jlong,
    compression: jint,
) -> jlong {
    enter_jni(env, |env| {
        let path: String = JString::to_string(&path);
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
                return Ok(0);
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
                return Ok(0);
            }
        };
        Ok(Box::into_raw(Box::new(writer)) as jlong)
    })
}

#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.clientWriterConfigNew")]
pub fn clientWriterConfigNew(
    _env: jni::EnvUnowned,
    _class: JClass,
    level: jint,
    address: JString,
    encryption: jint,
    key: JString,
) -> jlong {
    let address: String = JString::to_string(&address);
    let key = if encryption == 0 || key.is_null() {
        EncryptionMethod::NONE
    } else {
        let key: String = JString::to_string(&key);
        if encryption == 1 {
            EncryptionMethod::AuthKey(key.into_bytes())
        } else {
            EncryptionMethod::AES(key.into_bytes())
        }
    };
    Box::into_raw(Box::new(ClientWriterConfig::new(level as u8, address, key))) as jlong
}

#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.serverConfigNew")]
pub fn serverConfigNew(
    _env: jni::EnvUnowned,
    _class: JClass,
    level: jint,
    address: JString,
    encryption: jint,
    key: JString,
) -> jlong {
    let address: String = JString::to_string(&address);
    let key = if encryption == 0 || key.is_null() {
        EncryptionMethod::NONE
    } else {
        let key: String = JString::to_string(&key);
        if encryption == 1 {
            EncryptionMethod::AuthKey(key.into_bytes())
        } else {
            EncryptionMethod::AES(key.into_bytes())
        }
    };
    Box::into_raw(Box::new(ServerConfig::new(level as u8, address, key))) as jlong
}

#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.syslogWriterConfigNew")]
pub fn syslogWriterConfigNew(
    _env: jni::EnvUnowned,
    _class: JClass,
    level: jint,
    hostname: JString,
    pname: JString,
    pid: jint,
) -> jlong {
    let hostname: Option<String> = if hostname.is_null() {
        None
    } else {
        Some(JString::to_string(&hostname))
    };
    let pname: String = JString::to_string(&pname);
    Box::into_raw(Box::new(SyslogWriterConfig::new(
        level as u8,
        hostname,
        pname,
        pid as u32,
    ))) as jlong
}

pub static CALLBACK_JAVA_FUNC: RwLock<Option<Global<JObject<'static>>>> = RwLock::new(None);

pub fn callback_func(
    level: u8,
    domain: String,
    message: String,
) -> Result<(), fastlogging::LoggingError> {
    if let Some(ref callback_ref) = *CALLBACK_JAVA_FUNC.read() {
        let jvm = GLOBAL_JVM
            .get()
            .expect("JVM not initialized. JNI_OnLoad not called?");
        jvm.attach_current_thread(|env| {
            let local_ref: &JObject = callback_ref.as_obj();
            let jlevel = level as jint;
            let jname = jni_str!("onLog");
            let jsig = jni_sig!("(ILjava/lang/String;Ljava/lang/String;)V");
            let jdomain = env.new_string(&domain)?;
            let jmessage = env.new_string(&message)?;
            env.call_method(
                local_ref,
                jname,
                jsig,
                &[
                    JValue::Int(jlevel),
                    JValue::Object(&jdomain),
                    JValue::Object(&jmessage),
                ],
            )
            .unwrap()
            .v()
        })
        .unwrap();
    }
    Ok(())
}

#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.callbackWriterConfigNew")]
pub fn callbackWriterConfigNew(
    env: jni::EnvUnowned,
    _class: JClass,
    level: jint,
    callback: JObject,
) -> jlong {
    enter_jni(env, |env| {
        let callback_ref = env
            .new_global_ref(callback)
            .expect("Failed to create global reference for callback_instance");
        *CALLBACK_JAVA_FUNC.write() = Some(callback_ref);
        Ok(Box::into_raw(Box::new(CallbackWriterConfig::new(
            level as u8,
            Some(Box::new(callback_func)),
        ))) as jlong)
    })
}
