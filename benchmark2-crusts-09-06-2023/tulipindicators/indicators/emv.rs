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
pub extern "C" fn ti_emv_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn ti_emv(
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
        let mut volume: *const std::os::raw::c_double =
            *inputs.offset(2 as std::os::raw::c_int as isize);
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        if size <= ti_emv_start(options) {
            return 0;
        }
        let mut last: std::os::raw::c_double = (*high.offset(0 as std::os::raw::c_int as isize)
            + *low.offset(0 as std::os::raw::c_int as isize))
            * 0.5f64;
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < size {
            let mut hl: std::os::raw::c_double =
                (*high.offset(i as isize) + *low.offset(i as isize)) * 0.5f64;
            let mut br: std::os::raw::c_double = *volume.offset(i as isize)
                / 10000.0f64
                / (*high.offset(i as isize) - *low.offset(i as isize));
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = (hl - last) / br;
            last = hl;
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_emv_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 7], &[std::os::raw::c_char; 7]>(b"ti_emv\x00"))
                    .as_ptr(),
                b"indicators/emv.c\x00" as *const u8 as *const std::os::raw::c_char,
                56 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_emv_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
