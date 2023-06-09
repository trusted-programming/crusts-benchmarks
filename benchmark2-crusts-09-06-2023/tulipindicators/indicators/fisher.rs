extern "C" {
    #[no_mangle]
    fn log(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn __assert_rtn(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_int,
        _: *const std::os::raw::c_char,
    ) -> !;
}
#[no_mangle]
pub extern "C" fn ti_fisher_start(
    mut options: *const std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            - 1 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_fisher(
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
        let mut fisher: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        let mut signal: *mut std::os::raw::c_double =
            *outputs.offset(1 as std::os::raw::c_int as isize);
        let period: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        if period < 1 as std::os::raw::c_int {
            return 1;
        }
        if size <= ti_fisher_start(options) {
            return 0;
        }
        let mut trail: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut maxi: std::os::raw::c_int = -(1 as std::os::raw::c_int);
        let mut mini: std::os::raw::c_int = -(1 as std::os::raw::c_int);
        let mut max: std::os::raw::c_double = 0.5f64
            * (*high.offset(0 as std::os::raw::c_int as isize)
                + *low.offset(0 as std::os::raw::c_int as isize));
        let mut min: std::os::raw::c_double = 0.5f64
            * (*high.offset(0 as std::os::raw::c_int as isize)
                + *low.offset(0 as std::os::raw::c_int as isize));
        let mut val1: std::os::raw::c_double = 0.0f64;
        let mut bar: std::os::raw::c_double = 0.;
        let mut fish: std::os::raw::c_double = 0.0f64;
        let mut i: std::os::raw::c_int = 0;
        let mut j: std::os::raw::c_int = 0;
        i = period - 1 as std::os::raw::c_int;
        while i < size {
            bar = 0.5f64 * (*high.offset(i as isize) + *low.offset(i as isize));
            if maxi < trail {
                maxi = trail;
                max = 0.5f64 * (*high.offset(maxi as isize) + *low.offset(maxi as isize));
                j = trail;
                loop {
                    j += 1;
                    if !(j <= i) {
                        break;
                    }
                    bar = 0.5f64 * (*high.offset(j as isize) + *low.offset(j as isize));
                    if bar >= max {
                        max = bar;
                        maxi = j
                    }
                }
            } else if bar >= max {
                maxi = i;
                max = bar
            }
            bar = 0.5f64 * (*high.offset(i as isize) + *low.offset(i as isize));
            if mini < trail {
                mini = trail;
                min = 0.5f64 * (*high.offset(mini as isize) + *low.offset(mini as isize));
                j = trail;
                loop {
                    j += 1;
                    if !(j <= i) {
                        break;
                    }
                    bar = 0.5f64 * (*high.offset(j as isize) + *low.offset(j as isize));
                    if bar <= min {
                        min = bar;
                        mini = j
                    }
                }
            } else if bar <= min {
                mini = i;
                min = bar
            }
            let mut mm: std::os::raw::c_double = max - min;
            if mm == 0.0f64 {
                mm = 0.001f64
            }
            val1 = 0.33f64
                * 2.0f64
                * ((0.5f64 * (*high.offset(i as isize) + *low.offset(i as isize)) - min) / mm
                    - 0.5f64)
                + 0.67f64 * val1;
            if val1 > 0.99f64 {
                val1 = 0.999f64
            }
            if val1 < -0.99f64 {
                val1 = -0.999f64
            }
            let fresh0 = signal;
            signal = signal.offset(1);
            *fresh0 = fish;
            fish = 0.5f64 * log((1.0f64 + val1) / (1.0f64 - val1)) + 0.5f64 * fish;
            let fresh1 = fisher;
            fisher = fisher.offset(1);
            *fresh1 = fish;
            i += 1;
            trail += 1
        }
        if !(fisher.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_fisher_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 10], &[std::os::raw::c_char; 10]>(
                    b"ti_fisher\x00",
                ))
                .as_ptr(),
                b"indicators/fisher.c\x00" as *const u8 as *const std::os::raw::c_char,
                103 as std::os::raw::c_int,
                b"fisher - outputs[0] == size - ti_fisher_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        if !(signal.offset_from(*outputs.offset(1 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_fisher_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 10], &[std::os::raw::c_char; 10]>(
                    b"ti_fisher\x00",
                ))
                .as_ptr(),
                b"indicators/fisher.c\x00" as *const u8 as *const std::os::raw::c_char,
                104 as std::os::raw::c_int,
                b"signal - outputs[1] == size - ti_fisher_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
