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
pub extern "C" fn ti_stochrsi_start(
    mut options: *const std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int
            * 2 as std::os::raw::c_int
            - 1 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_stochrsi(
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
        let per: std::os::raw::c_double = 1.0f64 / period as std::os::raw::c_double;
        if period < 2 as std::os::raw::c_int {
            return 1;
        }
        if size <= ti_stochrsi_start(options) {
            return 0;
        }
        let mut rsi: *mut ti_buffer = ti_buffer_new(period);
        let mut smooth_up: std::os::raw::c_double =
            0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut smooth_down: std::os::raw::c_double =
            0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i <= period {
            let upward: std::os::raw::c_double = if *input.offset(i as isize)
                > *input.offset((i - 1 as std::os::raw::c_int) as isize)
            {
                (*input.offset(i as isize)) - *input.offset((i - 1 as std::os::raw::c_int) as isize)
            } else {
                0 as std::os::raw::c_int as std::os::raw::c_double
            };
            let downward: std::os::raw::c_double = if *input.offset(i as isize)
                < *input.offset((i - 1 as std::os::raw::c_int) as isize)
            {
                (*input.offset((i - 1 as std::os::raw::c_int) as isize)) - *input.offset(i as isize)
            } else {
                0 as std::os::raw::c_int as std::os::raw::c_double
            };
            smooth_up += upward;
            smooth_down += downward;
            i += 1
        }
        smooth_up /= period as std::os::raw::c_double;
        smooth_down /= period as std::os::raw::c_double;
        let mut r: std::os::raw::c_double = 100.0f64 * (smooth_up / (smooth_up + smooth_down));
        if (*rsi).pushes >= (*rsi).size {
            (*rsi).sum -= *(*rsi).vals.as_mut_ptr().offset((*rsi).index as isize)
        };
        (*rsi).sum += r;
        *(*rsi).vals.as_mut_ptr().offset((*rsi).index as isize) = r;
        (*rsi).pushes += 1 as std::os::raw::c_int;
        (*rsi).index = (*rsi).index + 1 as std::os::raw::c_int;
        if (*rsi).index >= (*rsi).size {
            (*rsi).index = 0 as std::os::raw::c_int
        }
        let mut min: std::os::raw::c_double = r;
        let mut max: std::os::raw::c_double = r;
        let mut mini: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut maxi: std::os::raw::c_int = 0 as std::os::raw::c_int;
        i = period + 1 as std::os::raw::c_int;
        while i < size {
            let upward_0: std::os::raw::c_double = if *input.offset(i as isize)
                > *input.offset((i - 1 as std::os::raw::c_int) as isize)
            {
                (*input.offset(i as isize)) - *input.offset((i - 1 as std::os::raw::c_int) as isize)
            } else {
                0 as std::os::raw::c_int as std::os::raw::c_double
            };
            let downward_0: std::os::raw::c_double = if *input.offset(i as isize)
                < *input.offset((i - 1 as std::os::raw::c_int) as isize)
            {
                (*input.offset((i - 1 as std::os::raw::c_int) as isize)) - *input.offset(i as isize)
            } else {
                0 as std::os::raw::c_int as std::os::raw::c_double
            };
            smooth_up = (upward_0 - smooth_up) * per + smooth_up;
            smooth_down = (downward_0 - smooth_down) * per + smooth_down;
            r = 100.0f64 * (smooth_up / (smooth_up + smooth_down));
            if r > max {
                max = r;
                maxi = (*rsi).index
            } else if maxi == (*rsi).index {
                max = r;
                let mut j: std::os::raw::c_int = 0;
                j = 0 as std::os::raw::c_int;
                while j < (*rsi).size {
                    if !(j == (*rsi).index) {
                        if *(*rsi).vals.as_mut_ptr().offset(j as isize) > max {
                            max = *(*rsi).vals.as_mut_ptr().offset(j as isize);
                            maxi = j
                        }
                    }
                    j += 1
                }
            }
            if r < min {
                min = r;
                mini = (*rsi).index
            } else if mini == (*rsi).index {
                min = r;
                let mut j_0: std::os::raw::c_int = 0;
                j_0 = 0 as std::os::raw::c_int;
                while j_0 < (*rsi).size {
                    if !(j_0 == (*rsi).index) {
                        if *(*rsi).vals.as_mut_ptr().offset(j_0 as isize) < min {
                            min = *(*rsi).vals.as_mut_ptr().offset(j_0 as isize);
                            mini = j_0
                        }
                    }
                    j_0 += 1
                }
            };
            *(*rsi).vals.as_mut_ptr().offset((*rsi).index as isize) = r;
            (*rsi).index = (*rsi).index + 1 as std::os::raw::c_int;
            if (*rsi).index >= (*rsi).size {
                (*rsi).index = 0 as std::os::raw::c_int
            }
            if i > period * 2 as std::os::raw::c_int - 2 as std::os::raw::c_int {
                let diff: std::os::raw::c_double = max - min;
                if diff == 0.0f64 {
                    let fresh0 = output;
                    output = output.offset(1);
                    *fresh0 = 0.0f64
                } else {
                    let fresh1 = output;
                    output = output.offset(1);
                    *fresh1 = (r - min) / diff
                }
            }
            i += 1
        }
        ti_buffer_free(rsi);
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_stochrsi_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 12], &[std::os::raw::c_char; 12]>(
                    b"ti_stochrsi\x00",
                ))
                .as_ptr(),
                b"indicators/stochrsi.c\x00" as *const u8 as *const std::os::raw::c_char,
                115 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_stochrsi_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
