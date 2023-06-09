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
pub extern "C" fn ti_willr_start(
    mut options: *const std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            - 1 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_willr(
    mut size: std::os::raw::c_int,
    mut inputs: *const *const std::os::raw::c_double,
    mut options: *const std::os::raw::c_double,
    mut outputs: *const *mut std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        let mut high: *const std::os::raw::c_double =
            *inputs.offset(0 as std::os::raw::c_int as isize);
        let mut low: *const std::os::raw::c_double =
            *inputs.offset(1 as std::os::raw::c_int as isize);
        let mut close: *const std::os::raw::c_double =
            *inputs.offset(2 as std::os::raw::c_int as isize);
        let period: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        if period < 1 as std::os::raw::c_int {
            return 1;
        }
        if size <= ti_willr_start(options) {
            return 0;
        }
        let mut trail: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut maxi: std::os::raw::c_int = -(1 as std::os::raw::c_int);
        let mut mini: std::os::raw::c_int = -(1 as std::os::raw::c_int);
        let mut max: std::os::raw::c_double = *high.offset(0 as std::os::raw::c_int as isize);
        let mut min: std::os::raw::c_double = *low.offset(0 as std::os::raw::c_int as isize);
        let mut bar: std::os::raw::c_double = 0.;
        let mut i: std::os::raw::c_int = 0;
        let mut j: std::os::raw::c_int = 0;
        i = period - 1 as std::os::raw::c_int;
        while i < size {
            bar = *high.offset(i as isize);
            if maxi < trail {
                maxi = trail;
                max = *high.offset(maxi as isize);
                j = trail;
                loop {
                    j += 1;
                    if !(j <= i) {
                        break;
                    }
                    bar = *high.offset(j as isize);
                    if bar >= max {
                        max = bar;
                        maxi = j
                    }
                }
            } else if bar >= max {
                maxi = i;
                max = bar
            }
            bar = *low.offset(i as isize);
            if mini < trail {
                mini = trail;
                min = *low.offset(mini as isize);
                j = trail;
                loop {
                    j += 1;
                    if !(j <= i) {
                        break;
                    }
                    bar = *low.offset(j as isize);
                    if bar <= min {
                        min = bar;
                        mini = j
                    }
                }
            } else if bar <= min {
                mini = i;
                min = bar
            }
            let highlow: std::os::raw::c_double = max - min;
            let r: std::os::raw::c_double = if highlow == 0.0f64 {
                0.0f64
            } else {
                (-(100 as std::os::raw::c_int) as std::os::raw::c_double)
                    * ((max - *close.offset(i as isize)) / highlow)
            };
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = r;
            i += 1;
            trail += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_willr_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 9], &[std::os::raw::c_char; 9]>(b"ti_willr\x00"))
                    .as_ptr(),
                b"indicators/willr.c\x00" as *const u8 as *const std::os::raw::c_char,
                96 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_willr_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
