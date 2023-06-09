extern "C" {
    #[no_mangle]
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
}
#[no_mangle]
pub extern "C" fn ti_tr_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn ti_tr(
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
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        let mut truerange: std::os::raw::c_double = 0.;
        *output.offset(0 as std::os::raw::c_int as isize) = *high
            .offset(0 as std::os::raw::c_int as isize)
            - *low.offset(0 as std::os::raw::c_int as isize);
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < size {
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
            *output.offset(i as isize) = truerange;
            i += 1
        }
        return 0;
    }
}
