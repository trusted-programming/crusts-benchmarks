extern "C" {
    #[no_mangle]
    fn __assert_rtn(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_int,
        _: *const std::os::raw::c_char,
    ) -> !;
    #[no_mangle]
    fn ti_buffer_new(size: std::os::raw::c_int) -> *mut ti_buffer;
    #[no_mangle]
    fn ti_buffer_free(buffer: *mut ti_buffer);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_buffer {
    pub size: std::os::raw::c_int,
    pub pushes: std::os::raw::c_int,
    pub index: std::os::raw::c_int,
    pub sum: std::os::raw::c_double,
    pub vals: [std::os::raw::c_double; 1],
}
#[no_mangle]
pub extern "C" fn ti_cvi_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        let n: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        return n * 2 as std::os::raw::c_int - 1 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_cvi(
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
        let period: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        if period < 1 as std::os::raw::c_int {
            return 1;
        }
        if size <= ti_cvi_start(options) {
            return 0;
        }
        let per: std::os::raw::c_double = 2 as std::os::raw::c_int as std::os::raw::c_double
            / (period as std::os::raw::c_double
                + 1 as std::os::raw::c_int as std::os::raw::c_double);
        let mut lag: *mut ti_buffer = ti_buffer_new(period);
        let mut val: std::os::raw::c_double = *high.offset(0 as std::os::raw::c_int as isize)
            - *low.offset(0 as std::os::raw::c_int as isize);
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < period * 2 as std::os::raw::c_int - 1 as std::os::raw::c_int {
            val = (*high.offset(i as isize) - *low.offset(i as isize) - val) * per + val;
            *(*lag).vals.as_mut_ptr().offset((*lag).index as isize) = val;
            (*lag).index = (*lag).index + 1 as std::os::raw::c_int;
            if (*lag).index >= (*lag).size {
                (*lag).index = 0 as std::os::raw::c_int
            }
            i += 1
        }
        i = period * 2 as std::os::raw::c_int - 1 as std::os::raw::c_int;
        while i < size {
            val = (*high.offset(i as isize) - *low.offset(i as isize) - val) * per + val;
            let old: std::os::raw::c_double =
                *(*lag).vals.as_mut_ptr().offset((*lag).index as isize);
            let fresh0 = output;
            output = output.offset(1);
            *fresh0 = 100.0f64 * (val - old) / old;
            *(*lag).vals.as_mut_ptr().offset((*lag).index as isize) = val;
            (*lag).index = (*lag).index + 1 as std::os::raw::c_int;
            if (*lag).index >= (*lag).size {
                (*lag).index = 0 as std::os::raw::c_int
            }
            i += 1
        }
        ti_buffer_free(lag);
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_cvi_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 7], &[std::os::raw::c_char; 7]>(b"ti_cvi\x00"))
                    .as_ptr(),
                b"indicators/cvi.c\x00" as *const u8 as *const std::os::raw::c_char,
                66 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_cvi_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
