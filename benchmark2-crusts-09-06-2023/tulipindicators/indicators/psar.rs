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
pub extern "C" fn ti_psar_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn ti_psar(
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
        let accel_step: std::os::raw::c_double = *options.offset(0 as std::os::raw::c_int as isize);
        let accel_max: std::os::raw::c_double = *options.offset(1 as std::os::raw::c_int as isize);
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        if accel_step <= 0 as std::os::raw::c_int as std::os::raw::c_double {
            return 1;
        }
        if accel_max <= accel_step {
            return 1;
        }
        if size < 2 as std::os::raw::c_int {
            return 0;
        }
        let mut lng: std::os::raw::c_int = 0;
        if *high.offset(0 as std::os::raw::c_int as isize)
            + *low.offset(0 as std::os::raw::c_int as isize)
            <= *high.offset(1 as std::os::raw::c_int as isize)
                + *low.offset(1 as std::os::raw::c_int as isize)
        {
            lng = 1 as std::os::raw::c_int
        } else {
            lng = 0 as std::os::raw::c_int
        }
        let mut sar: std::os::raw::c_double = 0.;
        let mut extreme: std::os::raw::c_double = 0.;
        if lng != 0 {
            extreme = *high.offset(0 as std::os::raw::c_int as isize);
            sar = *low.offset(0 as std::os::raw::c_int as isize)
        } else {
            extreme = *low.offset(0 as std::os::raw::c_int as isize);
            sar = *high.offset(0 as std::os::raw::c_int as isize)
        }
        let mut accel: std::os::raw::c_double = accel_step;
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < size {
            sar = (extreme - sar) * accel + sar;
            if lng != 0 {
                if i >= 2 as std::os::raw::c_int
                    && sar > *low.offset((i - 2 as std::os::raw::c_int) as isize)
                {
                    sar = *low.offset((i - 2 as std::os::raw::c_int) as isize)
                }
                if sar > *low.offset((i - 1 as std::os::raw::c_int) as isize) {
                    sar = *low.offset((i - 1 as std::os::raw::c_int) as isize)
                }
                if accel < accel_max && *high.offset(i as isize) > extreme {
                    accel += accel_step;
                    if accel > accel_max {
                        accel = accel_max
                    }
                }
                if *high.offset(i as isize) > extreme {
                    extreme = *high.offset(i as isize)
                }
            } else {
                if i >= 2 as std::os::raw::c_int
                    && sar < *high.offset((i - 2 as std::os::raw::c_int) as isize)
                {
                    sar = *high.offset((i - 2 as std::os::raw::c_int) as isize)
                }
                if sar < *high.offset((i - 1 as std::os::raw::c_int) as isize) {
                    sar = *high.offset((i - 1 as std::os::raw::c_int) as isize)
                }
                if accel < accel_max && *low.offset(i as isize) < extreme {
                    accel += accel_step;
                    if accel > accel_max {
                        accel = accel_max
                    }
                }
                if *low.offset(i as isize) < extreme {
                    extreme = *low.offset(i as isize)
                }
            }
            if lng != 0 && *low.offset(i as isize) < sar
                || lng == 0 && *high.offset(i as isize) > sar
            {
                accel = accel_step;
                sar = extreme;
                lng = (lng == 0) as std::os::raw::c_int;
                if lng == 0 {
                    extreme = *low.offset(i as isize)
                } else {
                    extreme = *high.offset(i as isize)
                }
            }
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = sar;
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_psar_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 8], &[std::os::raw::c_char; 8]>(b"ti_psar\x00"))
                    .as_ptr(),
                b"indicators/psar.c\x00" as *const u8 as *const std::os::raw::c_char,
                122 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_psar_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
