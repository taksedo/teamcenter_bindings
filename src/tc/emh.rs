use std::ptr::null_mut;
use tc_bindgen_sys::EMH_ask_error_text;
use crate::string_conversions::c_char_to_string_cp1251;

pub fn tc_emh_ask_error_text(ifail: i32) -> String {
    unsafe {
        let mut text = null_mut();

        if EMH_ask_error_text(ifail, &mut text) == 1 {};
        // let result_string = {
        //     if text.is_null() || (*text).is_null() {
        //         panic!("Pointer is null!");
        //     }
        //
        //     // // Dereference the double pointer to get the inner pointer
        //     // let c_str_ptr: *mut c_char = *text;
        //     //
        //     // // Interpret the *mut c_char as a C-style string
        //     // let c_str = CStr::from_ptr(c_str_ptr);
        //     //
        //     // // Convert the CStr to a Rust String
        //     // c_str.to_string_lossy().into_owned()
        //
        // };
        // result_string

        c_char_to_string_cp1251(text).unwrap()
    }
}