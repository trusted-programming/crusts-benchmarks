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
pub extern "C" fn ti_tsf_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            - 1 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_tsf(
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
        if size <= ti_tsf_start(options) {
            return 0;
        }
        let mut x: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut x2: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut y: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut xy: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let p: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < period - 1 as std::os::raw::c_int {
            x += (i + 1 as std::os::raw::c_int) as std::os::raw::c_double;
            x2 += ((i + 1 as std::os::raw::c_int) * (i + 1 as std::os::raw::c_int))
                as std::os::raw::c_double;
            xy += *input.offset(i as isize)
                * (i + 1 as std::os::raw::c_int) as std::os::raw::c_double;
            y += *input.offset(i as isize);
            i += 1
        }
        x += period as std::os::raw::c_double;
        x2 += (period * period) as std::os::raw::c_double;
        let bd: std::os::raw::c_double = 1.0f64 / (period as std::os::raw::c_double * x2 - x * x);
        i = period - 1 as std::os::raw::c_int;
        while i < size {
            xy += *input.offset(i as isize) * period as std::os::raw::c_double;
            y += *input.offset(i as isize);
            let b: std::os::raw::c_double = (period as std::os::raw::c_double * xy - x * y) * bd;
            let a: std::os::raw::c_double = (y - b * x) * p;
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = a + b * (period + 1 as std::os::raw::c_int) as std::os::raw::c_double;
            xy -= y;
            y -= *input.offset((i - period + 1 as std::os::raw::c_int) as isize);
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_tsf_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 7], &[std::os::raw::c_char; 7]>(b"ti_tsf\x00"))
                    .as_ptr(),
                b"indicators/tsf.c\x00" as *const u8 as *const std::os::raw::c_char,
                44 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_tsf_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
