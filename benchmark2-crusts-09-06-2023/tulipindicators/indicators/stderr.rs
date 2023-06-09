extern "C" {
    #[no_mangle]
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn __assert_rtn(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_int,
        _: *const std::os::raw::c_char,
    ) -> !;
}
#[no_mangle]
pub extern "C" fn ti_stderr_start(
    mut options: *const std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            - 1 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_stderr(
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
        let scale: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
        if period < 1 as std::os::raw::c_int {
            return 1;
        }
        if size <= ti_stderr_start(options) {
            return 0;
        }
        let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut sum2: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mul: std::os::raw::c_double = 1.0f64 / sqrt(period as std::os::raw::c_double);
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < period {
            sum += *input.offset(i as isize);
            sum2 += *input.offset(i as isize) * *input.offset(i as isize);
            i += 1
        }
        let mut s2s2: std::os::raw::c_double = sum2 * scale - sum * scale * (sum * scale);
        if s2s2 > 0.0f64 {
            s2s2 = sqrt(s2s2)
        }
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = mul * s2s2;
        i = period;
        while i < size {
            sum += *input.offset(i as isize);
            sum2 += *input.offset(i as isize) * *input.offset(i as isize);
            sum -= *input.offset((i - period) as isize);
            sum2 -= *input.offset((i - period) as isize) * *input.offset((i - period) as isize);
            let mut s2s2_0: std::os::raw::c_double = sum2 * scale - sum * scale * (sum * scale);
            if s2s2_0 > 0.0f64 {
                s2s2_0 = sqrt(s2s2_0)
            }
            let fresh1 = output;
            output = output.offset(1);
            *fresh1 = mul * s2s2_0;
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_stderr_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 10], &[std::os::raw::c_char; 10]>(
                    b"ti_stderr\x00",
                ))
                .as_ptr(),
                b"indicators/stderr.c\x00" as *const u8 as *const std::os::raw::c_char,
                71 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_stderr_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
