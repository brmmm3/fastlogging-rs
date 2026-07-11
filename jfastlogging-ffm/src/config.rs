use std::ptr;

// FFM: No JNI imports needed

use fastlogging::{ExtConfig, MessageStructEnum};

/// # Safety
///
/// Create new extended configuration.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn extConfigNew(
    structured: i32,
    hostname: bool,
    pname: bool,
    pid: bool,
    tname: bool,
    tid: bool,
) -> *mut ExtConfig {
    let structured = match structured {
        0 => MessageStructEnum::String,
        1 => MessageStructEnum::Json,
        2 => MessageStructEnum::Xml,
        _ => {
            // FFM: No exception mechanism, just return null
            return ptr::null_mut();
        }
    };
    Box::into_raw(Box::new(ExtConfig::new(
        structured, hostname, pname, pid, tname, tid,
    )))
}
