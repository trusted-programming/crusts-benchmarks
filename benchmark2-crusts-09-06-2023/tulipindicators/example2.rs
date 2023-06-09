extern "C" {
    #[no_mangle]
    static mut ti_indicators: [ti_indicator_info; 0];
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
fn main_0() -> std::os::raw::c_int {
    unsafe {
        print!("This program is an example of looping through\n\"");
        print!("each of the available indicators.\n");
        let mut info: *const ti_indicator_info = ti_indicators.as_mut_ptr();
        while !(*info).name.is_null() {
            let mut i: std::os::raw::c_int = 0;
            print! ("{} ({}) has type {} with: {} inputs, {} options, {} outputs.\n\",9998,9998,9999,9999,9999,9999");
            print!("   inputs: \"");
            i = 0 as std::os::raw::c_int;
            while i < (*info).inputs {
                if i != 0 {
                    print!("{}{}\",9998,9998")
                } else {
                    print!("{}{}\",9998,9998")
                };
                i += 1
            }
            print!("\n\"");
            print!("   options: \"");
            i = 0 as std::os::raw::c_int;
            while i < (*info).options {
                if i != 0 {
                    print!("{}{}\",9998,9998")
                } else {
                    print!("{}{}\",9998,9998")
                };
                i += 1
            }
            print!("\n\"");
            print!("   outputs: \"");
            i = 0 as std::os::raw::c_int;
            while i < (*info).outputs {
                if i != 0 {
                    print!("{}{}\",9998,9998")
                } else {
                    print!("{}{}\",9998,9998")
                };
                i += 1
            }
            print!("\n\"");
            print!("\n\"");
            info = info.offset(1)
        }
        return 0;
    }
}
