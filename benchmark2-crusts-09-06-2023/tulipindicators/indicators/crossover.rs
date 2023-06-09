#[no_mangle]
pub extern "C" fn ti_crossover_start(
    mut options: *const std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn ti_crossover(
    mut size: std::os::raw::c_int,
    mut inputs: *const *const std::os::raw::c_double,
    mut options: *const std::os::raw::c_double,
    mut outputs: *const *mut std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        let mut a: *const std::os::raw::c_double =
            *inputs.offset(0 as std::os::raw::c_int as isize);
        let mut b: *const std::os::raw::c_double =
            *inputs.offset(1 as std::os::raw::c_int as isize);
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < size {
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = (*a.offset(i as isize) > *b.offset(i as isize)
                && *a.offset((i - 1 as std::os::raw::c_int) as isize)
                    <= *b.offset((i - 1 as std::os::raw::c_int) as isize))
                as std::os::raw::c_int as std::os::raw::c_double;
            i += 1
        }
        return 0;
    }
}
