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
pub extern "C" fn F(n: i32) -> i32 {
    return if n == 0 { 1 } else { n - M(F(n - 1)) };
}

#[no_mangle]
pub extern "C" fn M(n: i32) -> i32 {
    return if n == 0 { 0 } else { n - F(M(n - 1)) };
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    i = 0;
    unsafe {
        while i < 20 {
            printf(b"%2d \0" as *const u8 as *const i8, F(i));
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const i8);
    }
    i = 0;
    unsafe {
        while i < 20 {
            printf(b"%2d \0" as *const u8 as *const i8, M(i));
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const i8);
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
