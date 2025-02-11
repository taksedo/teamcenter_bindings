use std::ffi::CStr;
use encoding::all::WINDOWS_1251;
use encoding::{DecoderTrap, Encoding};
use libc::c_char;

pub fn c_char_to_string_cp1251(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        eprintln!("Pointer is null.");
        return None;
    }
    unsafe {
        let bytes_c_str = CStr::from_ptr(ptr);
        let bytes = bytes_c_str.to_bytes();
        match WINDOWS_1251.decode(bytes, DecoderTrap::Strict) {
            Ok(decoded_str) => Some(decoded_str),
            Err(_) => {
                eprintln!("Ошибка декодирования строки из CP1251.");
                None
            }
        }
    }
}