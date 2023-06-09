extern "C" {
    #[no_mangle]
    fn __assert_rtn(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_int,
        _: *const std::os::raw::c_char,
    ) -> !;
}
#[no_mangle]
pub extern "C" fn ti_min_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            - 1 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_min(
    mut size: std::os::raw::c_int,
    mut inputs: *const *const std::os::raw::c_double,
    mut options: *const std::os::raw::c_double,
    mut outputs: *const *mut std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        let mut input: *const std::os::raw::c_double =
            *inputs.offset(0 as std::os::raw::c_int as isize);
        let period: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        if period < 1 as std::os::raw::c_int {
            return 1;
        }
        if size <= ti_min_start(options) {
            return 0;
        }
        let mut trail: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut mini: std::os::raw::c_int = -(1 as std::os::raw::c_int);
        let mut min: std::os::raw::c_double = *input.offset(0 as std::os::raw::c_int as isize);
        let mut i: std::os::raw::c_int = 0;
        let mut j: std::os::raw::c_int = 0;
        i = period - 1 as std::os::raw::c_int;
        while i < size {
            let mut bar: std::os::raw::c_double = *input.offset(i as isize);
            if mini < trail {
                mini = trail;
                min = *input.offset(mini as isize);
                j = trail;
                loop {
                    j += 1;
                    if !(j <= i) {
                        break;
                    }
                    bar = *input.offset(j as isize);
                    if bar <= min {
                        min = bar;
                        mini = j
                    }
                }
            } else if bar <= min {
                mini = i;
                min = bar
            }
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = min;
            i += 1;
            trail += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_min_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 7], &[std::os::raw::c_char; 7]>(b"ti_min\x00"))
                    .as_ptr(),
                b"indicators/min.c\x00" as *const u8 as *const std::os::raw::c_char,
                66 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_min_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
