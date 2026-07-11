pub fn get_str_result<'a>(ptr: *const u8, len: usize) -> Result<&'a str, std::str::Utf8Error> {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    std::str::from_utf8(slice)
}

pub fn get_option_str<'a>(ptr: *const u8, len: usize) -> Option<&'a str> {
    if len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        std::str::from_utf8(slice).ok()
    } else {
        None
    }
}
