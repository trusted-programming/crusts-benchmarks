#[no_mangle]
pub extern "C" fn ti_bop_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn ti_bop(
    mut size: std::os::raw::c_int,
    mut inputs: *const *const std::os::raw::c_double,
    mut options: *const std::os::raw::c_double,
    mut outputs: *const *mut std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        let mut open: *const std::os::raw::c_double =
            *inputs.offset(0 as std::os::raw::c_int as isize);
        let mut high: *const std::os::raw::c_double =
            *inputs.offset(1 as std::os::raw::c_int as isize);
        let mut low: *const std::os::raw::c_double =
            *inputs.offset(2 as std::os::raw::c_int as isize);
        let mut close: *const std::os::raw::c_double =
            *inputs.offset(3 as std::os::raw::c_int as isize);
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < size {
            let mut hl: std::os::raw::c_double = *high.offset(i as isize) - *low.offset(i as isize);
            if hl <= 0.0f64 {
                *output.offset(i as isize) = 0 as std::os::raw::c_int as std::os::raw::c_double
            } else {
                *output.offset(i as isize) =
                    (*close.offset(i as isize) - *open.offset(i as isize)) / hl
            }
            i += 1
        }
        return 0;
    }
}
