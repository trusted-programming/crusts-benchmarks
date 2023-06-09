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
pub extern "C" fn ti_ultosc_start(
    mut options: *const std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_ultosc(
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
        let short_period: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let medium_period: std::os::raw::c_int =
            *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let long_period: std::os::raw::c_int =
            *options.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let mut output: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        if short_period < 1 as std::os::raw::c_int {
            return 1;
        }
        if medium_period < short_period {
            return 1;
        }
        if long_period < medium_period {
            return 1;
        }
        if size <= ti_ultosc_start(options) {
            return 0;
        }
        let mut bp_buf: *mut ti_buffer = ti_buffer_new(long_period);
        let mut r_buf: *mut ti_buffer = ti_buffer_new(long_period);
        let mut bp_short_sum: std::os::raw::c_double =
            0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut bp_medium_sum: std::os::raw::c_double =
            0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut r_short_sum: std::os::raw::c_double =
            0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut r_medium_sum: std::os::raw::c_double =
            0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut i: std::os::raw::c_int = 0;
        i = 1 as std::os::raw::c_int;
        while i < size {
            let true_low: std::os::raw::c_double = if *low.offset(i as isize)
                < *close.offset((i - 1 as std::os::raw::c_int) as isize)
            {
                *low.offset(i as isize)
            } else {
                *close.offset((i - 1 as std::os::raw::c_int) as isize)
            };
            let true_high: std::os::raw::c_double = if *high.offset(i as isize)
                > *close.offset((i - 1 as std::os::raw::c_int) as isize)
            {
                *high.offset(i as isize)
            } else {
                *close.offset((i - 1 as std::os::raw::c_int) as isize)
            };
            let bp: std::os::raw::c_double = *close.offset(i as isize) - true_low;
            let r: std::os::raw::c_double = true_high - true_low;
            bp_short_sum += bp;
            bp_medium_sum += bp;
            r_short_sum += r;
            r_medium_sum += r;
            if (*bp_buf).pushes >= (*bp_buf).size {
                (*bp_buf).sum -= *(*bp_buf).vals.as_mut_ptr().offset((*bp_buf).index as isize)
            };
            (*bp_buf).sum += bp;
            *(*bp_buf).vals.as_mut_ptr().offset((*bp_buf).index as isize) = bp;
            (*bp_buf).pushes += 1 as std::os::raw::c_int;
            (*bp_buf).index = (*bp_buf).index + 1 as std::os::raw::c_int;
            if (*bp_buf).index >= (*bp_buf).size {
                (*bp_buf).index = 0 as std::os::raw::c_int
            }
            if (*r_buf).pushes >= (*r_buf).size {
                (*r_buf).sum -= *(*r_buf).vals.as_mut_ptr().offset((*r_buf).index as isize)
            };
            (*r_buf).sum += r;
            *(*r_buf).vals.as_mut_ptr().offset((*r_buf).index as isize) = r;
            (*r_buf).pushes += 1 as std::os::raw::c_int;
            (*r_buf).index = (*r_buf).index + 1 as std::os::raw::c_int;
            if (*r_buf).index >= (*r_buf).size {
                (*r_buf).index = 0 as std::os::raw::c_int
            }
            if i > short_period {
                let mut short_index: std::os::raw::c_int =
                    (*bp_buf).index - short_period - 1 as std::os::raw::c_int;
                if short_index < 0 as std::os::raw::c_int {
                    short_index += long_period
                }
                bp_short_sum -= *(*bp_buf).vals.as_mut_ptr().offset(short_index as isize);
                r_short_sum -= *(*r_buf).vals.as_mut_ptr().offset(short_index as isize);
                if i > medium_period {
                    let mut medium_index: std::os::raw::c_int =
                        (*bp_buf).index - medium_period - 1 as std::os::raw::c_int;
                    if medium_index < 0 as std::os::raw::c_int {
                        medium_index += long_period
                    }
                    bp_medium_sum -= *(*bp_buf).vals.as_mut_ptr().offset(medium_index as isize);
                    r_medium_sum -= *(*r_buf).vals.as_mut_ptr().offset(medium_index as isize)
                }
            }
            if i >= long_period {
                let first: std::os::raw::c_double =
                    4 as std::os::raw::c_int as std::os::raw::c_double * bp_short_sum / r_short_sum;
                let second: std::os::raw::c_double =
                    2 as std::os::raw::c_int as std::os::raw::c_double * bp_medium_sum
                        / r_medium_sum;
                let third: std::os::raw::c_double =
                    1 as std::os::raw::c_int as std::os::raw::c_double * (*bp_buf).sum
                        / (*r_buf).sum;
                let ult: std::os::raw::c_double = (first + second + third) * 100.0f64 / 7.0f64;
                let fresh0 = output;
                output = output.offset(1);
                *fresh0 = ult
            }
            i += 1
        }
        ti_buffer_free(bp_buf);
        ti_buffer_free(r_buf);
        if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_ultosc_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 10], &[std::os::raw::c_char; 10]>(
                    b"ti_ultosc\x00",
                ))
                .as_ptr(),
                b"indicators/ultosc.c\x00" as *const u8 as *const std::os::raw::c_char,
                103 as std::os::raw::c_int,
                b"output - outputs[0] == size - ti_ultosc_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
