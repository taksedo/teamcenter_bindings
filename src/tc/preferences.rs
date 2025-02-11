use crate::string_conversions::c_char_to_string_cp1251;
use libc::c_char;
use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr::null_mut;
use tc_bindgen_sys::{ITK_ok, MEM_free, PREF_ask_char_value};

pub fn tc_pref_ask_char_value(preference_name: &str, index: i32) -> Result<Option<String>, i32> {
    unsafe {
        let preference_name = CString::new(preference_name).expect("Failed to create CString");
        let preference_name_ptr = preference_name.as_ptr();
        let mut value = null_mut();
        let status = PREF_ask_char_value(preference_name_ptr, index, &mut value);
        let result = c_char_to_string_cp1251(value as *mut c_char);
        MEM_free(value as *mut c_void);

        if status != ITK_ok as i32 {
            return Err(status);
        }
        Ok(result)
    }
}
