extern "C" {
    #[no_mangle]
    fn strcmp(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;
    #[no_mangle]
    fn atof(_: *const std::os::raw::c_char) -> std::os::raw::c_double;
    #[no_mangle]
    fn __assert_rtn(
        _: *const std::os::raw::c_char,
        _: *const std::os::raw::c_char,
        _: std::os::raw::c_int,
        _: *const std::os::raw::c_char,
    ) -> !;
    #[no_mangle]
    static mut ti_indicators: [ti_indicator_info; 0];
    #[no_mangle]
    fn ti_find_indicator(name: *const std::os::raw::c_char) -> *const ti_indicator_info;
}
pub type ti_indicator_start_function =
    Option<unsafe extern "C" fn(_: *const std::os::raw::c_double) -> std::os::raw::c_int>;
pub type ti_indicator_function = Option<
    unsafe extern "C" fn(
        _: std::os::raw::c_int,
        _: *const *const std::os::raw::c_double,
        _: *const std::os::raw::c_double,
        _: *const *mut std::os::raw::c_double,
    ) -> std::os::raw::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_indicator_info {
    pub name: *mut std::os::raw::c_char,
    pub full_name: *mut std::os::raw::c_char,
    pub start: ti_indicator_start_function,
    pub indicator: ti_indicator_function,
    pub type_0: std::os::raw::c_int,
    pub inputs: std::os::raw::c_int,
    pub options: std::os::raw::c_int,
    pub outputs: std::os::raw::c_int,
    pub input_names: [*mut std::os::raw::c_char; 10],
    pub option_names: [*mut std::os::raw::c_char; 10],
    pub output_names: [*mut std::os::raw::c_char; 10],
}
#[no_mangle]
pub static mut out: [[std::os::raw::c_double; 15]; 5] = [[0.; 15]; 5];
#[no_mangle]
pub static mut datet: [*const std::os::raw::c_char; 15] = [
    b"2005-11-01\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-02\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-03\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-04\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-07\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-08\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-09\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-10\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-11\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-14\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-15\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-16\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-17\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-18\x00" as *const u8 as *const std::os::raw::c_char,
    b"2005-11-21\x00" as *const u8 as *const std::os::raw::c_char,
];
#[no_mangle]
pub static mut date: [std::os::raw::c_double; 15] = [
    51101 as std::os::raw::c_int as std::os::raw::c_double,
    51102 as std::os::raw::c_int as std::os::raw::c_double,
    51103 as std::os::raw::c_int as std::os::raw::c_double,
    51104 as std::os::raw::c_int as std::os::raw::c_double,
    51107 as std::os::raw::c_int as std::os::raw::c_double,
    51108 as std::os::raw::c_int as std::os::raw::c_double,
    51109 as std::os::raw::c_int as std::os::raw::c_double,
    51110 as std::os::raw::c_int as std::os::raw::c_double,
    51111 as std::os::raw::c_int as std::os::raw::c_double,
    51114 as std::os::raw::c_int as std::os::raw::c_double,
    51115 as std::os::raw::c_int as std::os::raw::c_double,
    51116 as std::os::raw::c_int as std::os::raw::c_double,
    51117 as std::os::raw::c_int as std::os::raw::c_double,
    51118 as std::os::raw::c_int as std::os::raw::c_double,
    51121 as std::os::raw::c_int as std::os::raw::c_double,
];
#[no_mangle]
pub static mut open: [std::os::raw::c_double; 15] = [
    81.85f64,
    81.2f64,
    81.55f64,
    82.91f64,
    83.1f64,
    83.41f64,
    82.71f64,
    82.7f64,
    84.2f64,
    84.25f64,
    84.03f64,
    85.45f64,
    86.18f64,
    88 as std::os::raw::c_int as std::os::raw::c_double,
    87.6f64,
];
#[no_mangle]
pub static mut high: [std::os::raw::c_double; 15] = [
    82.15f64,
    81.89f64,
    83.03f64,
    83.3f64,
    83.85f64,
    83.9f64,
    83.33f64,
    84.3f64,
    84.84f64,
    85 as std::os::raw::c_int as std::os::raw::c_double,
    85.9f64,
    86.58f64,
    86.98f64,
    88 as std::os::raw::c_int as std::os::raw::c_double,
    87.87f64,
];
#[no_mangle]
pub static mut low: [std::os::raw::c_double; 15] = [
    81.29f64, 80.64f64, 81.31f64, 82.65f64, 83.07f64, 83.11f64, 82.49f64, 82.3f64, 84.15f64,
    84.11f64, 84.03f64, 85.39f64, 85.76f64, 87.17f64, 87.01f64,
];
#[no_mangle]
pub static mut close: [std::os::raw::c_double; 15] = [
    81.59f64,
    81.06f64,
    82.87f64,
    83 as std::os::raw::c_int as std::os::raw::c_double,
    83.61f64,
    83.15f64,
    82.84f64,
    83.99f64,
    84.55f64,
    84.36f64,
    85.53f64,
    86.54f64,
    86.89f64,
    87.77f64,
    87.29f64,
];
#[no_mangle]
pub static mut volume: [std::os::raw::c_double; 15] = [
    5653100 as std::os::raw::c_int as std::os::raw::c_double,
    6447400 as std::os::raw::c_int as std::os::raw::c_double,
    7690900 as std::os::raw::c_int as std::os::raw::c_double,
    3831400 as std::os::raw::c_int as std::os::raw::c_double,
    4455100 as std::os::raw::c_int as std::os::raw::c_double,
    3798000 as std::os::raw::c_int as std::os::raw::c_double,
    3936200 as std::os::raw::c_int as std::os::raw::c_double,
    4732000 as std::os::raw::c_int as std::os::raw::c_double,
    4841300 as std::os::raw::c_int as std::os::raw::c_double,
    3915300 as std::os::raw::c_int as std::os::raw::c_double,
    6830800 as std::os::raw::c_int as std::os::raw::c_double,
    6694100 as std::os::raw::c_int as std::os::raw::c_double,
    5293600 as std::os::raw::c_int as std::os::raw::c_double,
    7985800 as std::os::raw::c_int as std::os::raw::c_double,
    4807900 as std::os::raw::c_int as std::os::raw::c_double,
];
#[no_mangle]
pub static mut alternative: [std::os::raw::c_double; 15] = [
    0.2f64,
    0.3f64,
    0.4f64,
    0.3f64,
    0.5f64,
    0.7f64,
    0.75f64,
    0.9f64,
    0.9f64,
    1 as std::os::raw::c_int as std::os::raw::c_double,
    1 as std::os::raw::c_int as std::os::raw::c_double,
    0.2f64,
    0.1f64,
    -0.1f64,
    -0.5f64,
];
fn main_0(
    mut argc: std::os::raw::c_int,
    mut argv: *mut *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    unsafe {
        let mut info: *const ti_indicator_info = ti_indicators.as_mut_ptr();
        if argc < 2 as std::os::raw::c_int {
            print!("No indicator given.\n\"");
            print!("Example:\n\"");
            print!("	sample ma 5\"");
            return 1;
        }
        if strcmp(
            *argv.offset(1 as std::os::raw::c_int as isize),
            b"--version\x00" as *const u8 as *const std::os::raw::c_char,
        ) == 0 as std::os::raw::c_int
        {
            print!("TI VERSION: {}, TI BUILD: {}\n\",9998,9999");
            return 0;
        }
        if strcmp(
            *argv.offset(1 as std::os::raw::c_int as isize),
            b"--list\x00" as *const u8 as *const std::os::raw::c_char,
        ) == 0 as std::os::raw::c_int
        {
            loop {
                if (*info).type_0 == 1 as std::os::raw::c_int {
                    print!("type overlay \"");
                } else if (*info).type_0 == 2 as std::os::raw::c_int {
                    print!("type indicator \"");
                } else if (*info).type_0 == 3 as std::os::raw::c_int {
                    print!("type math \"");
                } else if (*info).type_0 == 4 as std::os::raw::c_int {
                    print!("type simple \"");
                } else {
                    print!("type unknown \"");
                }
                print!("name {} \",9998");
                print!("full_name {{{}}} \",9998");
                let mut i: std::os::raw::c_int = 0;
                print!("inputs {{\"");
                i = 0 as std::os::raw::c_int;
                while i < (*info).inputs {
                    if i != 0 {
                        print!("{}{}\",9998,9998")
                    } else {
                        print!("{}{}\",9998,9998")
                    };
                    i += 1
                }
                print!("}} \"");
                print!("options {{\"");
                i = 0 as std::os::raw::c_int;
                while i < (*info).options {
                    if i != 0 {
                        print!("{}{{{}}}\",9998,9998")
                    } else {
                        print!("{}{{{}}}\",9998,9998")
                    };
                    i += 1
                }
                print!("}} \"");
                print!("outputs {{\"");
                i = 0 as std::os::raw::c_int;
                while i < (*info).outputs {
                    if i != 0 {
                        print!("{}{{{}}}\",9998,9998")
                    } else {
                        print!("{}{{{}}}\",9998,9998")
                    };
                    i += 1
                }
                print!("}}\"");
                print!("\n\"");
                info = info.offset(1);
                if (*info).name.is_null() {
                    break;
                }
            }
            return 0;
        }
        info = ti_find_indicator(*argv.offset(1 as std::os::raw::c_int as isize));
        if info.is_null() {
            print!("mpty");
            return 1;
        }
        let mut inputs: [*const std::os::raw::c_double; 5] = [
            0 as *const std::os::raw::c_double,
            0 as *const std::os::raw::c_double,
            0 as *const std::os::raw::c_double,
            0 as *const std::os::raw::c_double,
            0 as *const std::os::raw::c_double,
        ];
        let mut outputs: [*mut std::os::raw::c_double; 5] = [
            out[0 as std::os::raw::c_int as usize].as_mut_ptr(),
            out[1 as std::os::raw::c_int as usize].as_mut_ptr(),
            out[2 as std::os::raw::c_int as usize].as_mut_ptr(),
            out[3 as std::os::raw::c_int as usize].as_mut_ptr(),
            out[4 as std::os::raw::c_int as usize].as_mut_ptr(),
        ];
        let mut o: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut h: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut l: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut c: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut r: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut r2: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut v: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut a: std::os::raw::c_int = 0 as std::os::raw::c_int;
        let mut j: std::os::raw::c_int = 0;
        j = 0 as std::os::raw::c_int;
        while j < (*info).inputs {
            if strcmp(
                (*info).input_names[j as usize],
                b"open\x00" as *const u8 as *const std::os::raw::c_char,
            ) == 0 as std::os::raw::c_int
            {
                inputs[j as usize] = open.as_mut_ptr();
                o = 1 as std::os::raw::c_int
            } else if strcmp(
                (*info).input_names[j as usize],
                b"high\x00" as *const u8 as *const std::os::raw::c_char,
            ) == 0 as std::os::raw::c_int
            {
                inputs[j as usize] = high.as_mut_ptr();
                h = 1 as std::os::raw::c_int
            } else if strcmp(
                (*info).input_names[j as usize],
                b"low\x00" as *const u8 as *const std::os::raw::c_char,
            ) == 0 as std::os::raw::c_int
            {
                inputs[j as usize] = low.as_mut_ptr();
                l = 1 as std::os::raw::c_int
            } else if strcmp(
                (*info).input_names[j as usize],
                b"close\x00" as *const u8 as *const std::os::raw::c_char,
            ) == 0 as std::os::raw::c_int
            {
                inputs[j as usize] = close.as_mut_ptr();
                c = 1 as std::os::raw::c_int
            } else if strcmp(
                (*info).input_names[j as usize],
                b"volume\x00" as *const u8 as *const std::os::raw::c_char,
            ) == 0 as std::os::raw::c_int
            {
                inputs[j as usize] = volume.as_mut_ptr();
                v = 1 as std::os::raw::c_int
            } else if strcmp(
                (*info).input_names[j as usize],
                b"real\x00" as *const u8 as *const std::os::raw::c_char,
            ) == 0 as std::os::raw::c_int
            {
                if r == 0 {
                    inputs[j as usize] = close.as_mut_ptr();
                    r = 1 as std::os::raw::c_int
                } else {
                    inputs[j as usize] = open.as_mut_ptr();
                    r2 = 1 as std::os::raw::c_int
                }
            } else {
                if (0 as std::os::raw::c_int == 0) as std::os::raw::c_int as std::os::raw::c_long
                    != 0
                {
                    __assert_rtn(
                        (*::std::mem::transmute::<&[u8; 5], &[std::os::raw::c_char; 5]>(
                            b"main\x00",
                        ))
                        .as_ptr(),
                        b"sample.c\x00" as *const u8 as *const std::os::raw::c_char,
                        144 as std::os::raw::c_int,
                        b"0\x00" as *const u8 as *const std::os::raw::c_char,
                    );
                } else {
                };
            }
            j += 1
        }
        let mut alts: [*const std::os::raw::c_char; 8] = [
            b"acos\x00" as *const u8 as *const std::os::raw::c_char,
            b"asin\x00" as *const u8 as *const std::os::raw::c_char,
            b"atan\x00" as *const u8 as *const std::os::raw::c_char,
            b"cosh\x00" as *const u8 as *const std::os::raw::c_char,
            b"sinh\x00" as *const u8 as *const std::os::raw::c_char,
            b"tanh\x00" as *const u8 as *const std::os::raw::c_char,
            b"todeg\x00" as *const u8 as *const std::os::raw::c_char,
            0 as *const std::os::raw::c_char,
        ];
        let mut alt: *mut *const std::os::raw::c_char = alts.as_mut_ptr();
        while !(*alt).is_null() {
            if strcmp(*alt, (*info).name) == 0 as std::os::raw::c_int {
                r = 0 as std::os::raw::c_int;
                a = 1 as std::os::raw::c_int;
                j = 0 as std::os::raw::c_int;
                while j < (*info).inputs {
                    inputs[j as usize] = alternative.as_mut_ptr();
                    j += 1
                }
                break;
            } else {
                alt = alt.offset(1)
            }
        }
        let mut options: [std::os::raw::c_double; 10] = [0.; 10];
        let mut i_0: std::os::raw::c_int = 0;
        i_0 = 0 as std::os::raw::c_int;
        while i_0 < (*info).options {
            if argc < 3 as std::os::raw::c_int + i_0 {
                print!("*ERROR NOT ENOUGH OPTIONS*\n\"");
                return 1;
            }
            options[i_0 as usize] = atof(*argv.offset((2 as std::os::raw::c_int + i_0) as isize));
            i_0 += 1
        }
        let mut start: std::os::raw::c_int =
            (*info).start.expect("non-null function pointer")(options.as_mut_ptr());
        let ret: std::os::raw::c_int = (*info).indicator.expect("non-null function pointer")(
            15 as std::os::raw::c_int,
            inputs.as_mut_ptr(),
            options.as_mut_ptr(),
            outputs.as_mut_ptr(),
        );
        if ret == 0 as std::os::raw::c_int {
            let mut i_1: std::os::raw::c_int = 0;
            let mut k: std::os::raw::c_int = 0;
            let mut bad: std::os::raw::c_int = 0 as std::os::raw::c_int;
            print!("date        \"");
            if o != 0 {
                print!(" open   \"");
            }
            if h != 0 {
                print!(" high   \"");
            }
            if l != 0 {
                print!(" low    \"");
            }
            if c != 0 {
                print!(" close  \"");
            }
            if v != 0 {
                print!(" volume \"");
            }
            if r != 0 {
                print!(" input  \"");
            }
            if r2 != 0 {
                print!(" input2  \"");
            }
            if a != 0 {
                print!(" input  \"");
            }
            i_1 = 0 as std::os::raw::c_int;
            while i_1 < (*info).outputs {
                print!(" {}\",9998");
                i_1 += 1
            }
            print!("\n\"");
            i_1 = 0 as std::os::raw::c_int;
            while i_1 < 15 as std::os::raw::c_int {
                print!("{}\",9998");
                if o != 0 {
                    print!(" {:8.2}\",9999");
                }
                if h != 0 {
                    print!(" {:8.2}\",9999");
                }
                if l != 0 {
                    print!(" {:8.2}\",9999");
                }
                if c != 0 {
                    print!(" {:8.2}\",9999");
                }
                if v != 0 {
                    print!(" {:8.0}\",9999");
                }
                if r != 0 {
                    print!(" {:8.2}\",9999");
                }
                if r2 != 0 {
                    print!(" {:8.2}\",9999");
                }
                if a != 0 {
                    print!(" {:8.2}\",9999");
                }
                if i_1 >= start {
                    k = 0 as std::os::raw::c_int;
                    while k < (*info).outputs {
                        if out[k as usize][(i_1 - start) as usize]
                            != out[k as usize][(i_1 - start) as usize]
                        {
                            bad = 1 as std::os::raw::c_int
                        }
                        print!(" {:8.3}\",9999");
                        k += 1
                    }
                }
                print!("\n\"");
                i_1 += 1
            }
            if bad != 0 {
                print!("\nERROR NaN in outputs ({}).,9998");
                return 1;
            }
            return 0;
        } else {
            if ret == 1 as std::os::raw::c_int {
                print!("*ERROR INVALID OPTION*\n\"");
            } else {
                print!("*ERROR*\n\"");
            }
            return 1;
        };
    }
}
