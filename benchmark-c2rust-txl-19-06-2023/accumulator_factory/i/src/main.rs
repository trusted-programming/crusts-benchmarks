#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn printf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub extern "C" fn x(mut i: f64) -> f64 {
    unsafe {
        static mut _n: f64 = 1.0f64;
        _n += i;
        return _n;
    }
}

#[no_mangle]
pub extern "C" fn y(mut i: i32) -> i32 {
    unsafe {
        static mut _n: i32 = 3;
        _n += i;
        return _n;
    }
}

#[no_mangle]
pub extern "C" fn z(mut i: i32) -> i32 {
    unsafe {
        static mut _n: i32 = 'a' as i32;
        _n += i;
        return _n;
    }
}

fn main_0() -> i32 {
    unsafe {
        printf(b"%f\n\0" as *const u8 as *const i8, x(5 as f64));
        printf(b"%f\n\0" as *const u8 as *const i8, x(2.3f64));
        printf(b"%i\n\0" as *const u8 as *const i8, y(5.0f64 as i32));
        printf(b"%i\n\0" as *const u8 as *const i8, y(3.3f64 as i32));
        printf(b"%c\n\0" as *const u8 as *const i8, z(5));
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
