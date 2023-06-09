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
pub extern "C" fn ti_wad_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn ti_wad(
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
        if size <= ti_wad_start(options) {
            return 0;
        }
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        let mut sum: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut yc: std::os::raw::c_double = *close.offset(0 as std::os::raw::c_int as isize);
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < size {
            let c: std::os::raw::c_double = *close.offset(i as isize);
            if c > yc {
                sum += c
                    - (if yc < *low.offset(i as isize) {
                        yc
                    } else {
                        *low.offset(i as isize)
                    })
            } else if c < yc {
                sum += c
                    - (if yc > *high.offset(i as isize) {
                        yc
                    } else {
                        *high.offset(i as isize)
                    })
            }
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = sum;
            yc = *close.offset(i as isize);
            i += 1
        }
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_wad_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 7], &[std::os::raw::c_char; 7]>(b"ti_wad\x00"))
                    .as_ptr(),
                b"indicators/wad.c\x00" as *const u8 as *const std::os::raw::c_char,
                66 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_wad_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
