use libc::c_int;
use tc_bindgen_sys::ITK_initialize_text_services;

pub fn tc_itk_initialize_text_services(i: i32) -> i32 {
    unsafe { ITK_initialize_text_services(i as c_int) }
}