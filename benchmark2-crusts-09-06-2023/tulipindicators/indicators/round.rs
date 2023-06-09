extern "C" {
    #[no_mangle]
    fn floor(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[no_mangle]
pub extern "C" fn ti_round(
    mut size: std::os::raw::c_int,
    mut inputs: *const *const std::os::raw::c_double,
    mut options: *const std::os::raw::c_double,
    mut outputs: *const *mut std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        let mut in1: *const std::os::raw::c_double =
            *inputs.offset(0 as std::os::raw::c_int as isize);
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < size {
            *output.offset(i as isize) = floor(*in1.offset(i as isize) + 0.5f64);
            i += 1
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn ti_round_start(
    mut options: *const std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        return 0;
    }
}
