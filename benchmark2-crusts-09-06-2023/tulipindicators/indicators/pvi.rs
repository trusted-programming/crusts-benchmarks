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
pub extern "C" fn ti_pvi_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn ti_pvi(
    mut size: std::os::raw::c_int,
    mut inputs: *const *const std::os::raw::c_double,
    mut options: *const std::os::raw::c_double,
    mut outputs: *const *mut std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        let mut close: *const std::os::raw::c_double =
            *inputs.offset(0 as std::os::raw::c_int as isize);
        let mut volume: *const std::os::raw::c_double =
            *inputs.offset(1 as std::os::raw::c_int as isize);
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        if size <= ti_pvi_start(options) {
            return 0;
        }
        let mut pvi: std::os::raw::c_double = 1000 as std::os::raw::c_int as std::os::raw::c_double;
        let fresh0 = output;
        output = output.offset(1);
        *fresh0 = pvi;
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < size {
            if *volume.offset(i as isize) > *volume.offset((i - 1 as std::os::raw::c_int) as isize)
            {
                pvi += (*close.offset(i as isize)
                    - *close.offset((i - 1 as std::os::raw::c_int) as isize))
                    / *close.offset((i - 1 as std::os::raw::c_int) as isize)
                    * pvi
            }
            let fresh1 = output;
            output = output.offset(1);
            *fresh1 = pvi;
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_pvi_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 7], &[std::os::raw::c_char; 7]>(b"ti_pvi\x00"))
                    .as_ptr(),
                b"indicators/pvi.c\x00" as *const u8 as *const std::os::raw::c_char,
                56 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_pvi_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
