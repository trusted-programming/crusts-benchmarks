#[no_mangle]
pub extern "C" fn ti_obv_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn ti_obv(
    mut size: std::os::raw::c_int,
    mut inputs: *const *const std::os::raw::c_double,
    mut options: *const std::os::raw::c_double,
    mut outputs: *const *mut std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        let mut close: *const std::os::raw::c_double =
            *inputs.offset(0 as std::os::raw::c_int as isize);
        let mut volume: *const std::os::raw::c_double =
            *inputs.offset(1 as std::os::raw::c_int as isize);
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = sum;
        let mut prev: std::os::raw::c_double = *close.offset(0 as std::os::raw::c_int as isize);
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < size {
            if *close.offset(i as isize) > prev {
                sum += *volume.offset(i as isize)
            } else if *close.offset(i as isize) < prev {
                sum -= *volume.offset(i as isize)
            }
            prev = *close.offset(i as isize);
            let fresh1 = output;
            output = output.offset(1);
            *fresh1 = sum;
            i += 1
        }
        return 0;
    }
}
