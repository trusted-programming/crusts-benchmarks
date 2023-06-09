#[no_mangle]
pub extern "C" fn ti_decay_start(
    mut options: *const std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn ti_decay(
    mut size: std::os::raw::c_int,
    mut inputs: *const *const std::os::raw::c_double,
    mut options: *const std::os::raw::c_double,
    mut outputs: *const *mut std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        let mut input: *const std::os::raw::c_double =
            *inputs.offset(0 as std::os::raw::c_int as isize);
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        let period: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let scale: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = *input.offset(0 as std::os::raw::c_int as isize);
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < size {
            let mut d: std::os::raw::c_double =
                *output.offset(-(1 as std::os::raw::c_int) as isize) - scale;
            let fresh1 = output;
            output = output.offset(1);
            *fresh1 = if *input.offset(i as isize) > d {
                *input.offset(i as isize)
            } else {
                d
            };
            i += 1
        }
        return 0;
    }
}
