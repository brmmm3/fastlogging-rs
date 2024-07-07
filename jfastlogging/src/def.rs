use std::ptr;

use jni::JNIEnv;

use jni::objects::JClass;

use jni::sys::{jboolean, jint};

use fastlogging::{ExtConfig, MessageStructEnum};

/// # Safety
///
/// Create new extended configuration.
#[no_mangle]
pub unsafe extern "C" fn Java_org_logging_FastLogging_extConfigNew(
    mut env: JNIEnv,
    _class: JClass,
    structured: jint,
    hostname: jboolean,
    pname: jboolean,
    pid: jboolean,
    tname: jboolean,
    tid: jboolean,
) -> *mut ExtConfig {
    let structured = match structured {
        0 => MessageStructEnum::String,
        1 => MessageStructEnum::Json,
        2 => MessageStructEnum::Xml,
        _ => {
            env.throw(format!("Invalid value {structured} for structured"))
                .unwrap();
            return ptr::null_mut();
        }
    };
    Box::into_raw(Box::new(ExtConfig::new(
        structured,
        hostname != 0,
        pname != 0,
        pid != 0,
        tname != 0,
        tid != 0,
    )))
}
