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
pub extern "C" fn ti_stoch_start(
    mut options: *const std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        let kperiod: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let kslow: std::os::raw::c_int =
            *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let dperiod: std::os::raw::c_int =
            *options.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        return kperiod + kslow + dperiod - 3 as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_stoch(
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
        let kperiod: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let kslow: std::os::raw::c_int =
            *options.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let dperiod: std::os::raw::c_int =
            *options.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        let kper: std::os::raw::c_double = 1.0f64 / kslow as std::os::raw::c_double;
        let dper: std::os::raw::c_double = 1.0f64 / dperiod as std::os::raw::c_double;
        let mut stoch: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        let mut stoch_ma: *mut std::os::raw::c_double =
            *outputs.offset(1 as std::os::raw::c_int as isize);
        if kperiod < 1 as std::os::raw::c_int {
            return 1;
        }
        if kslow < 1 as std::os::raw::c_int {
            return 1;
        }
        if dperiod < 1 as std::os::raw::c_int {
            return 1;
        }
        if size <= ti_stoch_start(options) {
            return 0;
        }
        let mut trail: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut maxi: std::os::raw::c_int = -(1 as std::os::raw::c_int);
        let mut mini: std::os::raw::c_int = -(1 as std::os::raw::c_int);
        let mut max: std::os::raw::c_double = *high.offset(0 as std::os::raw::c_int as isize);
        let mut min: std::os::raw::c_double = *low.offset(0 as std::os::raw::c_int as isize);
        let mut bar: std::os::raw::c_double = 0.;
        let mut k_sum: *mut ti_buffer = ti_buffer_new(kslow);
        let mut d_sum: *mut ti_buffer = ti_buffer_new(dperiod);
        let mut i: std::os::raw::c_int = 0;
        let mut j: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < size {
            if i >= kperiod {
                trail += 1
            }
            bar = *high.offset(i as isize);
            if maxi < trail {
                maxi = trail;
                max = *high.offset(maxi as isize);
                j = trail;
                loop {
                    j += 1;
                    if !(j <= i) {
                        break;
                    }
                    bar = *high.offset(j as isize);
                    if bar >= max {
                        max = bar;
                        maxi = j
                    }
                }
            } else if bar >= max {
                maxi = i;
                max = bar
            }
            bar = *low.offset(i as isize);
            if mini < trail {
                mini = trail;
                min = *low.offset(mini as isize);
                j = trail;
                loop {
                    j += 1;
                    if !(j <= i) {
                        break;
                    }
                    bar = *low.offset(j as isize);
                    if bar <= min {
                        min = bar;
                        mini = j
                    }
                }
            } else if bar <= min {
                mini = i;
                min = bar
            }
            let kdiff: std::os::raw::c_double = max - min;
            let kfast: std::os::raw::c_double = if kdiff == 0.0f64 {
                0.0f64
            } else {
                (100 as std::os::raw::c_int as std::os::raw::c_double)
                    * ((*close.offset(i as isize) - min) / kdiff)
            };
            if (*k_sum).pushes >= (*k_sum).size {
                (*k_sum).sum -= *(*k_sum).vals.as_mut_ptr().offset((*k_sum).index as isize)
            };
            (*k_sum).sum += kfast;
            *(*k_sum).vals.as_mut_ptr().offset((*k_sum).index as isize) = kfast;
            (*k_sum).pushes += 1 as std::os::raw::c_int;
            (*k_sum).index = (*k_sum).index + 1 as std::os::raw::c_int;
            if (*k_sum).index >= (*k_sum).size {
                (*k_sum).index = 0 as std::os::raw::c_int
            }
            if i >= kperiod - 1 as std::os::raw::c_int + kslow - 1 as std::os::raw::c_int {
                let k: std::os::raw::c_double = (*k_sum).sum * kper;
                if (*d_sum).pushes >= (*d_sum).size {
                    (*d_sum).sum -= *(*d_sum).vals.as_mut_ptr().offset((*d_sum).index as isize)
                };
                (*d_sum).sum += k;
                *(*d_sum).vals.as_mut_ptr().offset((*d_sum).index as isize) = k;
                (*d_sum).pushes += 1 as std::os::raw::c_int;
                (*d_sum).index = (*d_sum).index + 1 as std::os::raw::c_int;
                if (*d_sum).index >= (*d_sum).size {
                    (*d_sum).index = 0 as std::os::raw::c_int
                }
                if i >= kperiod - 1 as std::os::raw::c_int + kslow - 1 as std::os::raw::c_int
                    + dperiod
                    - 1 as std::os::raw::c_int
                {
                    let fresh0 = stoch;
                    stoch = stoch.offset(1);
                    *fresh0 = k;
                    let fresh1 = stoch_ma;
                    stoch_ma = stoch_ma.offset(1);
                    *fresh1 = (*d_sum).sum * dper
                }
            }
            i += 1
        }
        ti_buffer_free(k_sum);
        ti_buffer_free(d_sum);
        if !(stoch.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_stoch_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 9], &[std::os::raw::c_char; 9]>(b"ti_stoch\x00"))
                    .as_ptr(),
                b"indicators/stoch.c\x00" as *const u8 as *const std::os::raw::c_char,
                125 as std::os::raw::c_int,
                b"stoch - outputs[0] == size - ti_stoch_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        if !(stoch_ma.offset_from(*outputs.offset(1 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_stoch_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 9], &[std::os::raw::c_char; 9]>(b"ti_stoch\x00"))
                    .as_ptr(),
                b"indicators/stoch.c\x00" as *const u8 as *const std::os::raw::c_char,
                126 as std::os::raw::c_int,
                b"stoch_ma - outputs[1] == size - ti_stoch_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
