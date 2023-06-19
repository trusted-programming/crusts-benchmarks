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
    let mut one: i32 = 1;
    unsafe {
        printf(
            b"word size = %d bits\n\0" as *const u8 as *const i8,
            8u64.wrapping_mul(::core::mem::size_of::<u64>() as u64) as i32,
        );
        if *(&mut one as *mut i32 as *mut i8) != 0 {
            printf(b"little endian\n\0" as *const u8 as *const i8);
        } else {
            printf(b"big endian\n\0" as *const u8 as *const i8);
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
