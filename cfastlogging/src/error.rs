use std::ffi::{c_char, CString};

pub static EINIT: isize = 100;
pub static EINVAL: isize = 22;

#[repr(C)]
pub struct Error {
    magic: u32,
    msg: CString,
    code: isize,
}

///  Some C code uses magic values in structures to determine if the pointer
/// is of the correct type.
const ERROR_MAGIC: u32 = 0xdeadbeef;

/// Adding this so that we can get a message printed when the Error is freed.
impl Drop for Error {
    fn drop(&mut self) {
        println!("Error struct being dropped ...");
    }
}

/// # Safety
///
/// Create new error.
pub fn error_new<S: Into<String>>(code: isize, message: S) -> Box<Error> {
    Box::new(Error {
        magic: ERROR_MAGIC,
        msg: CString::new(message.into()).unwrap(),
        code,
    })
}

/// # Safety
///
/// Drop error.
/// We take ownership as we are passing by value, so when function
/// exits the drop gets run.  Handles being passed null.
#[no_mangle]
pub extern "C" fn error_free(_: Option<Box<Error>>) {}

/// # Safety
///
/// Return error message.
/// Our example "getter" methods which work on the Error type. The value
/// returned is only valid as long as the Error has not been freed. If C
/// caller needs a longer lifetime they need to copy the value.
#[no_mangle]
pub unsafe extern "C" fn error_msg(e: &Error) -> *const c_char {
    e.msg.as_ptr()
}

#[no_mangle]
pub extern "C" fn error_code(e: &Error) -> isize {
    e.code
}
