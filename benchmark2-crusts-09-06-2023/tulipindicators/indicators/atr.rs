extern "C" {
    #[no_mangle]
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn __assert_rtn(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_int,
        _: *const std::os::raw::c_char,
    ) -> !;
}
#[no_mangle]
pub extern "C" fn ti_atr_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            - 1 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_atr(
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
        if size <= ti_atr_start(options) {
            return 0;
        }
        let per: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
        let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut truerange: std::os::raw::c_double = 0.;
        sum += *high.offset(0 as std::os::raw::c_int as isize)
            - *low.offset(0 as std::os::raw::c_int as isize);
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < period {
            let l: std::os::raw::c_double = *low.offset(i as isize);
            let h: std::os::raw::c_double = *high.offset(i as isize);
            let c: std::os::raw::c_double = *close.offset((i - 1 as std::os::raw::c_int) as isize);
            let ych: std::os::raw::c_double = fabs(h - c);
            let ycl: std::os::raw::c_double = fabs(l - c);
            let mut v: std::os::raw::c_double = h - l;
            if ych > v {
                v = ych
            }
            if ycl > v {
                v = ycl
            }
            truerange = v;
            sum += truerange;
            i += 1
        }
        let mut val: std::os::raw::c_double = sum / period as std::os::raw::c_double;
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = val;
        i = period;
        while i < size {
            let l_0: std::os::raw::c_double = *low.offset(i as isize);
            let h_0: std::os::raw::c_double = *high.offset(i as isize);
            let c_0: std::os::raw::c_double =
                *close.offset((i - 1 as std::os::raw::c_int) as isize);
            let ych_0: std::os::raw::c_double = fabs(h_0 - c_0);
            let ycl_0: std::os::raw::c_double = fabs(l_0 - c_0);
            let mut v_0: std::os::raw::c_double = h_0 - l_0;
            if ych_0 > v_0 {
                v_0 = ych_0
            }
            if ycl_0 > v_0 {
                v_0 = ycl_0
            }
            truerange = v_0;
            val = (truerange - val) * per + val;
            let fresh1 = output;
            output = output.offset(1);
            *fresh1 = val;
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_atr_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 7], &[std::os::raw::c_char; 7]>(b"ti_atr\x00"))
                    .as_ptr(),
                b"indicators/atr.c\x00" as *const u8 as *const std::os::raw::c_char,
                69 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_atr_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
