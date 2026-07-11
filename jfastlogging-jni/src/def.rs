use std::ptr;

use jni::jni_mangle;
use jni::objects::JClass;
use jni::sys::{jboolean, jint};

use fastlogging::{ExtConfig, MessageStructEnum};

use crate::enter_jni;

/// # Safety
///
/// Create new extended configuration.
#[allow(non_snake_case)]
#[jni_mangle("logging.FastLogging.extConfigNew")]
pub fn extConfigNew(
    env: jni::EnvUnowned,
    _class: JClass,
    structured: jint,
    hostname: jboolean,
    pname: jboolean,
    pid: jboolean,
    tname: jboolean,
    tid: jboolean,
) -> *mut ExtConfig {
    enter_jni(env, |env| {
        let structured = match structured {
            0 => MessageStructEnum::String,
            1 => MessageStructEnum::Json,
            2 => MessageStructEnum::Xml,
            _ => {
                env.throw(format!("Invalid value {structured} for structured"))
                    .unwrap();
                return Ok(ptr::null_mut());
            }
        };
        Ok(Box::into_raw(Box::new(ExtConfig::new(
            structured, hostname, pname, pid, tname, tid,
        ))))
    })
}
