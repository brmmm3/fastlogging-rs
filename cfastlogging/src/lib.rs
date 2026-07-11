pub mod error;
mod logging;
use std::{ffi::c_uint, ptr::null};

pub mod def;
pub use logging::*;
mod util;
mod writer;
use once_cell::sync::Lazy;
use rand::{RngExt, distr::Alphanumeric, rng};
pub use writer::*;
mod logger;
pub use logger::*;
pub mod root;

#[unsafe(no_mangle)]
pub static AUTH_KEY: Lazy<Vec<u8>> =
    Lazy::new(|| rng().sample_iter(&Alphanumeric).take(32).collect());

#[repr(C)]
pub struct KeyStruct {
    pub typ: EncryptionMethodEnum,
    pub len: c_uint,
    pub key: *const u8,
}

/// # Safety
///
/// Create encryption key.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_key(
    typ: EncryptionMethodEnum,
    len: c_uint,
    key: *const u8,
) -> *const KeyStruct {
    let (key_len, key_ptr) = match typ {
        EncryptionMethodEnum::NONE => (0, null()),
        EncryptionMethodEnum::AuthKey | EncryptionMethodEnum::AES => {
            if !key.is_null() {
                (len, key)
            } else {
                let mut key = AUTH_KEY.to_vec();
                key.shrink_to_fit();
                let key_len = key.len() as c_uint;
                let key_ptr = key.as_ptr();
                std::mem::forget(key);
                (key_len, key_ptr)
            }
        }
    };
    Box::into_raw(Box::new(KeyStruct {
        typ,
        len: key_len,
        key: key_ptr,
    }))
}

/// # Safety
///
/// Create encryption key.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_random_key(typ: EncryptionMethodEnum) -> *const KeyStruct {
    unsafe { create_key(typ, 0, null()) }
}
