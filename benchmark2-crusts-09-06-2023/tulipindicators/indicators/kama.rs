extern "C" {
    #[no_mangle]
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn __assert_rtn(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_int,
        _: *const std::os::raw::c_char,
    ) -> !;
}
#[no_mangle]
pub extern "C" fn ti_kama_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            - 1 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_kama(
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
        if size <= ti_kama_start(options) {
            return 0;
        }
        let short_per: std::os::raw::c_double = 2 as std::os::raw::c_int as std::os::raw::c_double
            / (2.0f64 + 1 as std::os::raw::c_int as std::os::raw::c_double);
        let long_per: std::os::raw::c_double = 2 as std::os::raw::c_int as std::os::raw::c_double
            / (30.0f64 + 1 as std::os::raw::c_int as std::os::raw::c_double);
        let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < period {
            sum += fabs(
                *input.offset(i as isize) - *input.offset((i - 1 as std::os::raw::c_int) as isize),
            );
            i += 1
        }
        let mut kama: std::os::raw::c_double =
            *input.offset((period - 1 as std::os::raw::c_int) as isize);
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = kama;
        let mut er: std::os::raw::c_double = 0.;
        let mut sc: std::os::raw::c_double = 0.;
        i = period;
        while i < size {
            sum += fabs(
                *input.offset(i as isize) - *input.offset((i - 1 as std::os::raw::c_int) as isize),
            );
            if i > period {
                sum -= fabs(
                    *input.offset((i - period) as isize)
                        - *input.offset((i - period - 1 as std::os::raw::c_int) as isize),
                )
            }
            if sum != 0.0f64 {
                er = fabs(*input.offset(i as isize) - *input.offset((i - period) as isize)) / sum
            } else {
                er = 1.0f64
            }
            sc = pow(
                er * (short_per - long_per) + long_per,
                2 as std::os::raw::c_int as std::os::raw::c_double,
            );
            kama = kama + sc * (*input.offset(i as isize) - kama);
            let fresh1 = output;
            output = output.offset(1);
            *fresh1 = kama;
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_kama_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 8], &[std::os::raw::c_char; 8]>(b"ti_kama\x00"))
                    .as_ptr(),
                b"indicators/kama.c\x00" as *const u8 as *const std::os::raw::c_char,
                75 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_kama_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
