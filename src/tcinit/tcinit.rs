use tc_bindgen_sys::ITK_auto_login;

pub fn tc_itk_auto_login() -> Result<(), i32> {
    unsafe {
        match ITK_auto_login() {
            0 => Ok(()),
            i_fail => Err(i_fail),
        }
    }
}