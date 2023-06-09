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
pub extern "C" fn ti_vidya_start(
    mut options: *const std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int
            - 2 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_vidya(
    mut size: std::os::raw::c_int,
    mut inputs: *const *const std::os::raw::c_double,
    mut options: *const std::os::raw::c_double,
    mut outputs: *const *mut std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        let mut input: *const std::os::raw::c_double =
            *inputs.offset(0 as std::os::raw::c_int as isize);
        let short_period: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let long_period: std::os::raw::c_int =
            *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let alpha: std::os::raw::c_double = *options.offset(2 as std::os::raw::c_int as isize);
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        let short_div: std::os::raw::c_double = 1.0f64 / short_period as std::os::raw::c_double;
        let long_div: std::os::raw::c_double = 1.0f64 / long_period as std::os::raw::c_double;
        if short_period < 1 as std::os::raw::c_int {
            return 1;
        }
        if long_period < short_period {
            return 1;
        }
        if long_period < 2 as std::os::raw::c_int {
            return 1;
        }
        if alpha < 0.0f64 || alpha > 1.0f64 {
            return 1;
        }
        if size <= ti_vidya_start(options) {
            return 0;
        }
        let mut short_sum: std::os::raw::c_double =
            0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut short_sum2: std::os::raw::c_double =
            0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut long_sum: std::os::raw::c_double =
            0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut long_sum2: std::os::raw::c_double =
            0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < long_period {
            long_sum += *input.offset(i as isize);
            long_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
            if i >= long_period - short_period {
                short_sum += *input.offset(i as isize);
                short_sum2 += *input.offset(i as isize) * *input.offset(i as isize)
            }
            i += 1
        }
        let mut val: std::os::raw::c_double =
            *input.offset((long_period - 2 as std::os::raw::c_int) as isize);
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = val;
        if (long_period - 1 as std::os::raw::c_int) < size {
            let mut short_stddev: std::os::raw::c_double =
                sqrt(short_sum2 * short_div - short_sum * short_div * (short_sum * short_div));
            let mut long_stddev: std::os::raw::c_double =
                sqrt(long_sum2 * long_div - long_sum * long_div * (long_sum * long_div));
            let mut k: std::os::raw::c_double = short_stddev / long_stddev;
            if k != k {
                k = 0 as std::os::raw::c_int as std::os::raw::c_double
            }
            k *= alpha;
            val =
                (*input.offset((long_period - 1 as std::os::raw::c_int) as isize) - val) * k + val;
            let fresh1 = output;
            output = output.offset(1);
            *fresh1 = val
        }
        i = long_period;
        while i < size {
            long_sum += *input.offset(i as isize);
            long_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
            short_sum += *input.offset(i as isize);
            short_sum2 += *input.offset(i as isize) * *input.offset(i as isize);
            long_sum -= *input.offset((i - long_period) as isize);
            long_sum2 -= *input.offset((i - long_period) as isize)
                * *input.offset((i - long_period) as isize);
            short_sum -= *input.offset((i - short_period) as isize);
            short_sum2 -= *input.offset((i - short_period) as isize)
                * *input.offset((i - short_period) as isize);
            let mut short_stddev_0: std::os::raw::c_double =
                sqrt(short_sum2 * short_div - short_sum * short_div * (short_sum * short_div));
            let mut long_stddev_0: std::os::raw::c_double =
                sqrt(long_sum2 * long_div - long_sum * long_div * (long_sum * long_div));
            let mut k_0: std::os::raw::c_double = short_stddev_0 / long_stddev_0;
            if k_0 != k_0 {
                k_0 = 0 as std::os::raw::c_int as std::os::raw::c_double
            }
            k_0 *= alpha;
            val = (*input.offset(i as isize) - val) * k_0 + val;
            let fresh2 = output;
            output = output.offset(1);
            *fresh2 = val;
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_vidya_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 9], &[std::os::raw::c_char; 9]>(b"ti_vidya\x00"))
                    .as_ptr(),
                b"indicators/vidya.c\x00" as *const u8 as *const std::os::raw::c_char,
                106 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_vidya_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
