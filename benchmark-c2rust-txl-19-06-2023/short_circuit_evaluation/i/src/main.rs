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
pub extern "C" fn a(mut in_0: bool) -> bool {
    unsafe {
        printf(b"I am a\n\0" as *const u8 as *const i8);
    }
    return in_0;
}

#[no_mangle]
pub extern "C" fn b(mut in_0: bool) -> bool {
    unsafe {
        printf(b"I am b\n\0" as *const u8 as *const i8);
    }
    return in_0;
}

fn main_0() -> i32 {
    let mut x: bool = false;
    x = a(0 != 0) as i32 != 0 && b(1 != 0) as i32 != 0;
    unsafe {
        if x as i32 != 0 {
            printf(
                b"false && true = %s\n\n\0" as *const u8 as *const i8,
                b"true\0" as *const u8 as *const i8,
            )
        } else {
            printf(
                b"false && true = %s\n\n\0" as *const u8 as *const i8,
                b"false\0" as *const u8 as *const i8,
            )
        };
    }
    x = a(1 != 0) as i32 != 0 || b(0 != 0) as i32 != 0;
    unsafe {
        if x as i32 != 0 {
            printf(
                b"true || false = %s\n\n\0" as *const u8 as *const i8,
                b"true\0" as *const u8 as *const i8,
            )
        } else {
            printf(
                b"true || false = %s\n\n\0" as *const u8 as *const i8,
                b"false\0" as *const u8 as *const i8,
            )
        };
    }
    x = a(1 != 0) as i32 != 0 && b(0 != 0) as i32 != 0;
    unsafe {
        if x as i32 != 0 {
            printf(
                b"true && false = %s\n\n\0" as *const u8 as *const i8,
                b"true\0" as *const u8 as *const i8,
            )
        } else {
            printf(
                b"true && false = %s\n\n\0" as *const u8 as *const i8,
                b"false\0" as *const u8 as *const i8,
            )
        };
    }
    x = a(0 != 0) as i32 != 0 || b(0 != 0) as i32 != 0;
    unsafe {
        if x as i32 != 0 {
            printf(
                b"false || false = %s\n\n\0" as *const u8 as *const i8,
                b"true\0" as *const u8 as *const i8,
            )
        } else {
            printf(
                b"false || false = %s\n\n\0" as *const u8 as *const i8,
                b"false\0" as *const u8 as *const i8,
            )
        };
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
