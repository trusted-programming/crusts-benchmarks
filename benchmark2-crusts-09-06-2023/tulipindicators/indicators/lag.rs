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
pub extern "C" fn ti_lag_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_lag(
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
        if period < 0 as std::os::raw::c_int {
            return 1;
        }
        if size <= ti_lag_start(options) {
            return 0;
        }
        let mut i: std::os::raw::c_int = 0;
        i = period;
        while i < size {
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = *input.offset((i - period) as isize);
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_lag_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 7], &[std::os::raw::c_char; 7]>(b"ti_lag\x00"))
                    .as_ptr(),
                b"indicators/lag.c\x00" as *const u8 as *const std::os::raw::c_char,
                46 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_lag_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
