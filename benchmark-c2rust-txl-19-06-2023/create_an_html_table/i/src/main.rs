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
    fn rand() -> i32;
}
fn main_0() -> i32 {
    let mut i: i32 = 0;
    unsafe {
        printf (b"<table style=\"text-align:center; border: 1px solid\"><th></th><th>X</th><th>Y</th><th>Z</th>\0" as * const u8 as * const i8,);
    }
    i = 0;
    unsafe {
        while i < 4 {
            printf(
                b"<tr><th>%d</th><td>%d</td><td>%d</td><td>%d</td></tr>\0" as *const u8
                    as *const i8,
                i,
                rand() % 10000,
                rand() % 10000,
                rand() % 10000,
            );
            i += 1;
            i;
        }
        printf(b"</table>\0" as *const u8 as *const i8);
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
