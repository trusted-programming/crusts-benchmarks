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
pub extern "C" fn ti_md_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            - 1 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_md(
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
        let scale: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
        if period < 1 as std::os::raw::c_int {
            return 1;
        }
        if size <= ti_md_start(options) {
            return 0;
        }
        let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut i: std::os::raw::c_int = 0;
        let mut j: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < size {
            let today: std::os::raw::c_double = *input.offset(i as isize);
            sum += today;
            if i >= period {
                sum -= *input.offset((i - period) as isize)
            }
            let avg: std::os::raw::c_double = sum * scale;
            if i >= period - 1 as std::os::raw::c_int {
                let mut acc: std::os::raw::c_double =
                    0 as std::os::raw::c_int as std::os::raw::c_double;
                j = 0 as std::os::raw::c_int;
                while j < period {
                    acc += fabs(avg - *input.offset((i - j) as isize));
                    j += 1
                }
                let fresh0 = output;
                output = output.offset(1);
                *fresh0 = acc * scale
            }
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_md_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 6], &[std::os::raw::c_char; 6]>(b"ti_md\x00"))
                    .as_ptr(),
                b"indicators/md.c\x00" as *const u8 as *const std::os::raw::c_char,
                63 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_md_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
