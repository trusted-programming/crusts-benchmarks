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
    let mut i: i32 = 0;
    i = 1;
    unsafe {
        while i <= 10 {
            printf(b"%d\0" as *const u8 as *const i8, i);
            if i == 10 {
                printf(b"\n\0" as *const u8 as *const i8)
            } else {
                printf(b", \0" as *const u8 as *const i8)
            };
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
