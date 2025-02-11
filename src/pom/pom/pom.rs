use std::ffi::c_void;
use std::ptr::null_mut;
use libc::c_char;
use tc_bindgen_sys::{ITK_ok, MEM_free, POM_get_user, NULL_TAG};
use crate::string_conversions::c_char_to_string_cp1251;

pub fn tc_pom_get_user() -> Result<(String, String), i32> {
    // Инициализируем указатели нулевыми значениями
    let mut user_name = null_mut();
    let mut user_tag = NULL_TAG;
    unsafe {
        // Вызываем C-функцию, передавая ссылки на указатели
        let status = POM_get_user(&mut user_name as *mut *mut c_char, &mut user_tag);
        if status != ITK_ok as i32 {
            return Err(status);
        };

        let result_user_name = c_char_to_string_cp1251(user_name).unwrap_or_default();
        MEM_free(user_name as *mut c_void);

        Ok((result_user_name, user_tag.to_string()))
    }
}