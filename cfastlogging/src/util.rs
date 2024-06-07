use std::ffi::{c_char, CStr};
use std::ptr::null;

#[inline]
pub fn char2string(s: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(s) };
    c_str.to_str().unwrap().to_string()
}

#[inline]
pub fn option_char2string(s: *const c_char) -> Option<String> {
    if s != null() {
        Some(char2string(s))
    } else {
        None
    }
}
