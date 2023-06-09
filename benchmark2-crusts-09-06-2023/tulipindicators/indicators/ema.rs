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
pub extern "C" fn ti_ema_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn ti_ema(
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
        if size <= ti_ema_start(options) {
            return 0;
        }
        let per: std::os::raw::c_double = 2 as std::os::raw::c_int as std::os::raw::c_double
            / (period as std::os::raw::c_double
                + 1 as std::os::raw::c_int as std::os::raw::c_double);
        let mut val: std::os::raw::c_double = *input.offset(0 as std::os::raw::c_int as isize);
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = val;
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < size {
            val = (*input.offset(i as isize) - val) * per + val;
            let fresh1 = output;
            output = output.offset(1);
            *fresh1 = val;
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_ema_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 7], &[std::os::raw::c_char; 7]>(b"ti_ema\x00"))
                    .as_ptr(),
                b"indicators/ema.c\x00" as *const u8 as *const std::os::raw::c_char,
                54 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_ema_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
