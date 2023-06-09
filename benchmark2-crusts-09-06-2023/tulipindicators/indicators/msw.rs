extern "C" {
    #[no_mangle]
    fn atan(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn cos(_: std::os::raw::c_double) -> std::os::raw::c_double;
    #[no_mangle]
    fn sin(_: std::os::raw::c_double) -> std::os::raw::c_double;
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
pub extern "C" fn ti_msw_start(mut options: *const std::os::raw::c_double) -> std::os::raw::c_int {
    unsafe {
        return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    }
}

#[no_mangle]
pub extern "C" fn ti_msw(
    mut size: std::os::raw::c_int,
    mut inputs: *const *const std::os::raw::c_double,
    mut options: *const std::os::raw::c_double,
    mut outputs: *const *mut std::os::raw::c_double,
) -> std::os::raw::c_int {
    unsafe {
        let mut input: *const std::os::raw::c_double =
            *inputs.offset(0 as std::os::raw::c_int as isize);
        let mut sine: *mut std::os::raw::c_double =
            *outputs.offset(0 as std::os::raw::c_int as isize);
        let mut lead: *mut std::os::raw::c_double =
            *outputs.offset(1 as std::os::raw::c_int as isize);
        let period: std::os::raw::c_int =
            *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
        if period < 1 as std::os::raw::c_int {
            return 1;
        }
        if size <= ti_msw_start(options) {
            return 0;
        }
        let pi: std::os::raw::c_double = 3.1415926f64;
        let tpi: std::os::raw::c_double = 2 as std::os::raw::c_int as std::os::raw::c_double * pi;
        let mut weight: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut phase: std::os::raw::c_double = 0.;
        let mut rp: std::os::raw::c_double = 0.;
        let mut ip: std::os::raw::c_double = 0.;
        let mut i: std::os::raw::c_int = 0;
        let mut j: std::os::raw::c_int = 0;
        i = period;
        while i < size {
            rp = 0 as std::os::raw::c_int as std::os::raw::c_double;
            ip = 0 as std::os::raw::c_int as std::os::raw::c_double;
            j = 0 as std::os::raw::c_int;
            while j < period {
                weight = *input.offset((i - j) as isize);
                rp = rp
                    + cos(tpi * j as std::os::raw::c_double / period as std::os::raw::c_double)
                        * weight;
                ip = ip
                    + sin(tpi * j as std::os::raw::c_double / period as std::os::raw::c_double)
                        * weight;
                j += 1
            }
            if fabs(rp) > 0.001f64 {
                phase = atan(ip / rp)
            } else {
                phase = tpi / 2.0f64
                    * (if ip < 0 as std::os::raw::c_int as std::os::raw::c_double {
                        -1.0f64
                    } else {
                        1.0f64
                    })
            }
            if rp < 0.0f64 {
                phase += pi
            }
            phase += pi / 2.0f64;
            if phase < 0.0f64 {
                phase += tpi
            }
            if phase > tpi {
                phase -= tpi
            }
            let fresh0 = sine;
            sine = sine.offset(1);
            *fresh0 = sin(phase);
            let fresh1 = lead;
            lead = lead.offset(1);
            *fresh1 = sin(phase + pi / 4.0f64);
            i += 1
        }
        if !(sine.offset_from(*outputs.offset(0 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_msw_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 7], &[std::os::raw::c_char; 7]>(b"ti_msw\x00"))
                    .as_ptr(),
                b"indicators/msw.c\x00" as *const u8 as *const std::os::raw::c_char,
                73 as std::os::raw::c_int,
                b"sine - outputs[0] == size - ti_msw_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        if !(lead.offset_from(*outputs.offset(1 as std::os::raw::c_int as isize))
            as std::os::raw::c_long
            == (size - ti_msw_start(options)) as std::os::raw::c_long)
            as std::os::raw::c_int as std::os::raw::c_long
            != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 7], &[std::os::raw::c_char; 7]>(b"ti_msw\x00"))
                    .as_ptr(),
                b"indicators/msw.c\x00" as *const u8 as *const std::os::raw::c_char,
                74 as std::os::raw::c_int,
                b"lead - outputs[1] == size - ti_msw_start(options)\x00" as *const u8
                    as *const std::os::raw::c_char,
            );
        } else {
        };
        return 0;
    }
}
