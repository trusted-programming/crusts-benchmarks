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
pub extern "C" fn ti_apo_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn ti_apo(
    mut size: std::os::raw::c_int,
    mut inputs: *const *const std::os::raw::c_double,
    mut options: *const std::os::raw::c_double,
    mut outputs: *const *mut std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        let mut input: *const std::os::raw::c_double =
            *inputs.offset(0 as std::os::raw::c_int as isize);
        let mut apo: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        let short_period: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let long_period: std::os::raw::c_int =
            *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        if short_period < 1 as std::os::raw::c_int {
            return 1;
        }
        if long_period < 2 as std::os::raw::c_int {
            return 1;
        }
        if long_period < short_period {
            return 1;
        }
        if size <= ti_apo_start(options) {
            return 0;
        }
        let mut short_per: std::os::raw::c_double = 2 as std::os::raw::c_int
            as std::os::raw::c_double
            / (short_period as std::os::raw::c_double
                + 1 as std::os::raw::c_int as std::os::raw::c_double);
        let mut long_per: std::os::raw::c_double = 2 as std::os::raw::c_int
            as std::os::raw::c_double
            / (long_period as std::os::raw::c_double
                + 1 as std::os::raw::c_int as std::os::raw::c_double);
        let mut short_ema: std::os::raw::c_double =
            *input.offset(0 as std::os::raw::c_int as isize);
        let mut long_ema: std::os::raw::c_double = *input.offset(0 as std::os::raw::c_int as isize);
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < size {
            short_ema = (*input.offset(i as isize) - short_ema) * short_per + short_ema;
            long_ema = (*input.offset(i as isize) - long_ema) * long_per + long_ema;
            let out: std::os::raw::c_double = short_ema - long_ema;
            let fresh0 = apo;
            apo = apo.offset(1);
            *fresh0 = out;
            i += 1
        }
        if !(apo.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_apo_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 7], &[std::os::raw::c_char; 7]>(b"ti_apo\x00"))
                    .as_ptr(),
                b"indicators/apo.c\x00" as *const u8 as *const std::os::raw::c_char,
                63 as std::os::raw::c_int,
                b"apo - outputs[0] == size - ti_apo_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
