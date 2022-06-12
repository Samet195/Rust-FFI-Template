//! Rust FFI Template

use std::os::raw::c_char;

#[link(name = "library", kind = "static")]
extern "C" {
    fn function() -> *const c_char;
}

unsafe fn str_convert(c_str: *const c_char) -> String {
    std::ffi::CStr::from_ptr(c_str)
        .to_string_lossy()
        .into_owned()
}

pub fn get_str() -> String {
    unsafe { str_convert(function()) }
}
