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
fn main_0() -> i32 {
    let mut r: libc::c_float = 7.125f64 as libc::c_float;
    unsafe {
        printf(b" %9.3f\n\0" as *const u8 as *const i8, -r as f64);
        printf(b" %9.3f\n\0" as *const u8 as *const i8, r as f64);
        printf(b" %-9.3f\n\0" as *const u8 as *const i8, r as f64);
        printf(b" %09.3f\n\0" as *const u8 as *const i8, -r as f64);
        printf(b" %09.3f\n\0" as *const u8 as *const i8, r as f64);
        printf(b" %-09.3f\n\0" as *const u8 as *const i8, r as f64);
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
