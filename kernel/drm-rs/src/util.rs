
pub fn get_string_from_chars(chars: &[u8]) -> String {
    String::from(
        std::ffi::CStr::from_bytes_until_nul(chars).unwrap().to_str().unwrap()
    )
}

#[allow(dead_code)]
pub(crate) fn get_string_from_ptr(ptr: *const libc::c_char) -> Option<String> {
    match ptr {
        ptr if ptr != std::ptr::null() => Some(String::from(
            unsafe { std::ffi::CStr::from_ptr(ptr) }.to_str().unwrap(),
        )),
        _ => None,
    }
}
