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
pub extern "C" fn ti_bbands_start(
    mut options: *const std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            - 1 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_bbands(
    mut size: std::os::raw::c_int,
    mut inputs: *const *const std::os::raw::c_double,
    mut options: *const std::os::raw::c_double,
    mut outputs: *const *mut std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        let mut input: *const std::os::raw::c_double =
            *inputs.offset(0 as std::os::raw::c_int as isize);
        let mut lower: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        let mut middle: *mut std::os::raw::c_double =
            *outputs.offset(1 as std::os::raw::c_int as isize);
        let mut upper: *mut std::os::raw::c_double =
            *outputs.offset(2 as std::os::raw::c_int as isize);
        let period: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let stddev: std::os::raw::c_double = *options.offset(1 as std::os::raw::c_int as isize);
        let scale: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
        if period < 1 as std::os::raw::c_int {
            return 1;
        }
        if size <= ti_bbands_start(options) {
            return 0;
        }
        let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut sum2: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < period {
            sum += *input.offset(i as isize);
            sum2 += *input.offset(i as isize) * *input.offset(i as isize);
            i += 1
        }
        let mut sd: std::os::raw::c_double = sqrt(sum2 * scale - sum * scale * (sum * scale));
        *middle = sum * scale;
        let fresh0 = lower;
        lower = lower.offset(1);
        *fresh0 = *middle - stddev * sd;
        let fresh1 = upper;
        upper = upper.offset(1);
        *fresh1 = *middle + stddev * sd;
        middle = middle.offset(1);
        i = period;
        while i < size {
            sum += *input.offset(i as isize);
            sum2 += *input.offset(i as isize) * *input.offset(i as isize);
            sum -= *input.offset((i - period) as isize);
            sum2 -= *input.offset((i - period) as isize) * *input.offset((i - period) as isize);
            sd = sqrt(sum2 * scale - sum * scale * (sum * scale));
            *middle = sum * scale;
            let fresh2 = upper;
            upper = upper.offset(1);
            *fresh2 = *middle + stddev * sd;
            let fresh3 = lower;
            lower = lower.offset(1);
            *fresh3 = *middle - stddev * sd;
            middle = middle.offset(1);
            i += 1
        }
        if !(lower.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_bbands_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 10], &[std::os::raw::c_char; 10]>(
                    b"ti_bbands\x00",
                ))
                .as_ptr(),
                b"indicators/bbands.c\x00" as *const u8 as *const std::os::raw::c_char,
                76 as std::os::raw::c_int,
                b"lower - outputs[0] == size - ti_bbands_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        if !(middle.offset_from(*outputs.offset(1 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_bbands_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 10], &[std::os::raw::c_char; 10]>(
                    b"ti_bbands\x00",
                ))
                .as_ptr(),
                b"indicators/bbands.c\x00" as *const u8 as *const std::os::raw::c_char,
                77 as std::os::raw::c_int,
                b"middle - outputs[1] == size - ti_bbands_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        if !(upper.offset_from(*outputs.offset(2 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_bbands_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 10], &[std::os::raw::c_char; 10]>(
                    b"ti_bbands\x00",
                ))
                .as_ptr(),
                b"indicators/bbands.c\x00" as *const u8 as *const std::os::raw::c_char,
                78 as std::os::raw::c_int,
                b"upper - outputs[2] == size - ti_bbands_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
