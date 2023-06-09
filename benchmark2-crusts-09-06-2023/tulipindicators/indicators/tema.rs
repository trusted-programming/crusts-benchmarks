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
pub extern "C" fn ti_tema_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        let period: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        return (period - 1 as std::os::raw::c_int) * 3 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_tema(
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
        if size <= ti_tema_start(options) {
            return 0;
        }
        let per: std::os::raw::c_double = 2 as std::os::raw::c_int as std::os::raw::c_double
            / (period as std::os::raw::c_double
                + 1 as std::os::raw::c_int as std::os::raw::c_double);
        let per1: std::os::raw::c_double = 1.0f64 - per;
        let mut ema: std::os::raw::c_double = *input.offset(0 as std::os::raw::c_int as isize);
        let mut ema2: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut ema3: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < size {
            ema = ema * per1 + *input.offset(i as isize) * per;
            if i == period - 1 as std::os::raw::c_int {
                ema2 = ema
            }
            if i >= period - 1 as std::os::raw::c_int {
                ema2 = ema2 * per1 + ema * per;
                if i == (period - 1 as std::os::raw::c_int) * 2 as std::os::raw::c_int {
                    ema3 = ema2
                }
                if i >= (period - 1 as std::os::raw::c_int) * 2 as std::os::raw::c_int {
                    ema3 = ema3 * per1 + ema2 * per;
                    if i >= (period - 1 as std::os::raw::c_int) * 3 as std::os::raw::c_int {
                        *output = 3 as std::os::raw::c_int as std::os::raw::c_double * ema
                            - 3 as std::os::raw::c_int as std::os::raw::c_double * ema2
                            + ema3;
                        output = output.offset(1)
                    }
                }
            }
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_tema_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 8], &[std::os::raw::c_char; 8]>(b"ti_tema\x00"))
                    .as_ptr(),
                b"indicators/tema.c\x00" as *const u8 as *const std::os::raw::c_char,
                76 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_tema_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
