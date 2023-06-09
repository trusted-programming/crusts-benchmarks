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
pub extern "C" fn ti_rsi_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_rsi(
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
        let per: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
        if period < 1 as std::os::raw::c_int {
            return 1;
        }
        if size <= ti_rsi_start(options) {
            return 0;
        }
        let mut smooth_up: std::os::raw::c_double =
            0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut smooth_down: std::os::raw::c_double =
            0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i <= period {
            let upward: std::os::raw::c_double = if *input.offset(i as isize)
                > *input.offset((i - 1 as std::os::raw::c_int) as isize)
            {
                (*input.offset(i as isize)) - *input.offset((i - 1 as std::os::raw::c_int) as isize)
            } else {
                0 as std::os::raw::c_int as std::os::raw::c_double
            };
            let downward: std::os::raw::c_double = if *input.offset(i as isize)
                < *input.offset((i - 1 as std::os::raw::c_int) as isize)
            {
                (*input.offset((i - 1 as std::os::raw::c_int) as isize)) - *input.offset(i as isize)
            } else {
                0 as std::os::raw::c_int as std::os::raw::c_double
            };
            smooth_up += upward;
            smooth_down += downward;
            i += 1
        }
        smooth_up /= period as std::os::raw::c_double;
        smooth_down /= period as std::os::raw::c_double;
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = 100.0f64 * (smooth_up / (smooth_up + smooth_down));
        i = period + 1 as std::os::raw::c_int;
        while i < size {
            let upward_0: std::os::raw::c_double = if *input.offset(i as isize)
                > *input.offset((i - 1 as std::os::raw::c_int) as isize)
            {
                (*input.offset(i as isize)) - *input.offset((i - 1 as std::os::raw::c_int) as isize)
            } else {
                0 as std::os::raw::c_int as std::os::raw::c_double
            };
            let downward_0: std::os::raw::c_double = if *input.offset(i as isize)
                < *input.offset((i - 1 as std::os::raw::c_int) as isize)
            {
                (*input.offset((i - 1 as std::os::raw::c_int) as isize)) - *input.offset(i as isize)
            } else {
                0 as std::os::raw::c_int as std::os::raw::c_double
            };
            smooth_up = (upward_0 - smooth_up) * per + smooth_up;
            smooth_down = (downward_0 - smooth_down) * per + smooth_down;
            let fresh1 = output;
            output = output.offset(1);
            *fresh1 = 100.0f64 * (smooth_up / (smooth_up + smooth_down));
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_rsi_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 7], &[std::os::raw::c_char; 7]>(b"ti_rsi\x00"))
                    .as_ptr(),
                b"indicators/rsi.c\x00" as *const u8 as *const std::os::raw::c_char,
                66 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_rsi_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
