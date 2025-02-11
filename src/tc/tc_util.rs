use std::ffi::CString;
use tc_bindgen_sys::TC_printf;

pub fn tc_tc_printf(text: &str, _args: Vec<String>) -> i32 {
    unsafe {
        let tc_printf_text = CString::new(text).expect("Cannot print CString");
        let tc_printf_text_ptr = tc_printf_text.as_ptr();
        TC_printf(tc_printf_text_ptr)
    }
}