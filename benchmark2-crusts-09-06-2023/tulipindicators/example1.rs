extern "C" {
    #[no_mangle]
    fn __assert_rtn(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_int,
        _: *const std::os::raw::c_char,
    ) -> !;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn ti_sma_start(options: *const std::os::raw::c_double) -> std::os::raw::c_int;
    #[no_mangle]
    fn ti_sma(
        size: std::os::raw::c_int,
        inputs: *const *const std::os::raw::c_double,
        options: *const std::os::raw::c_double,
        outputs: *const *mut std::os::raw::c_double,
    ) -> std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn print_array(mut p: *const std::os::raw::c_double, size: std::os::raw::c_int) {
    unsafe {
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < size {
            if i != 0 {
                print!(", \"");
            }
            print!("{:.1}\",9999");
            i += 1
        }
        print!("\n\"");
    }
}

fn main_0() -> std::os::raw::c_int {
    unsafe {
        let data_in: [std::os::raw::c_double; 10] = [
            5 as std::os::raw::c_int as std::os::raw::c_double,
            8 as std::os::raw::c_int as std::os::raw::c_double,
            12 as std::os::raw::c_int as std::os::raw::c_double,
            11 as std::os::raw::c_int as std::os::raw::c_double,
            9 as std::os::raw::c_int as std::os::raw::c_double,
            8 as std::os::raw::c_int as std::os::raw::c_double,
            7 as std::os::raw::c_int as std::os::raw::c_double,
            10 as std::os::raw::c_int as std::os::raw::c_double,
            11 as std::os::raw::c_int as std::os::raw::c_double,
            13 as std::os::raw::c_int as std::os::raw::c_double,
        ];
        let input_length: std::os::raw::c_int =
            (::std::mem::size_of::<[std::os::raw::c_double; 10]>() as std::os::raw::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong
                ) as std::os::raw::c_int;
        print!("We have {} bars of input data.\n\",9999");
        print_array(data_in.as_ptr(), input_length);
        let options: [std::os::raw::c_double; 1] =
            [3 as std::os::raw::c_int as std::os::raw::c_double];
        print!("Our option array is: \"");
        print_array(
            options.as_ptr(),
            (::std::mem::size_of::<[std::os::raw::c_double; 1]>() as std::os::raw::c_ulong)
                .wrapping_div(
                    ::std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong
                ) as std::os::raw::c_int,
        );
        let start: std::os::raw::c_int = ti_sma_start(options.as_ptr());
        print!("The start amount is: {}\n\",9999");
        let output_length: std::os::raw::c_int = input_length - start;
        let mut data_out: *mut std::os::raw::c_double = malloc(
            (output_length as std::os::raw::c_uint as std::os::raw::c_ulong).wrapping_mul(
                ::std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong,
            ),
        ) as *mut std::os::raw::c_double;
        if data_out.is_null() as std::os::raw::c_int as std::os::raw::c_long != 0 {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 5], &[std::os::raw::c_char; 5]>(b"main\x00"))
                    .as_ptr(),
                b"example1.c\x00" as *const u8 as *const std::os::raw::c_char,
                56 as std::os::raw::c_int,
                b"data_out != 0\x00" as *const u8 as *const std::os::raw::c_char,
            );
        } else {
        };
        print!("The output length is: {}\n\",9999");
        let mut all_inputs: [*const std::os::raw::c_double; 1] = [data_in.as_ptr()];
        let mut all_outputs: [*mut std::os::raw::c_double; 1] = [data_out];
        let mut error: std::os::raw::c_int = ti_sma(
            input_length,
            all_inputs.as_mut_ptr(),
            options.as_ptr(),
            all_outputs.as_mut_ptr(),
        );
        if !(error == 0 as std::os::raw::c_int) as std::os::raw::c_int as std::os::raw::c_long != 0
        {
            __assert_rtn(
                (*::std::mem::transmute::<&[u8; 5], &[std::os::raw::c_char; 5]>(b"main\x00"))
                    .as_ptr(),
                b"example1.c\x00" as *const u8 as *const std::os::raw::c_char,
                62 as std::os::raw::c_int,
                b"error == TI_OKAY\x00" as *const u8 as *const std::os::raw::c_char,
            );
        } else {
        };
        print!("The output data is: \"");
        print_array(data_out, output_length);
        return 0;
    }
}
