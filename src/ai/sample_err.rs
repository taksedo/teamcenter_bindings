use crate::{tc_emh_ask_error_text, tc_tc_printf};

pub fn tc_call<T>(input_fn: fn() -> Result<T, i32>) -> Result<T, ()> {
    match input_fn() {
        Err(i_fail) => {
            let err_string = tc_emh_ask_error_text(i_fail);
            tc_tc_printf(
                "ERROR: %d ERROR MSG: %s.\n",
                vec![i_fail.to_string(), err_string.to_string()],
            );
            let file_name = file!();
            let line_name = line!();
            let function = stringify!(input_fn);
            println!(
                "Function: {} FILE: {} LINE: {}\n",
                function, file_name, line_name
            );
            Err(())
        }
        Ok(result) => Ok(result),
    }
}