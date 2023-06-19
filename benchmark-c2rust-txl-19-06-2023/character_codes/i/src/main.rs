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
    unsafe {
        printf(b"%d\n\0" as *const u8 as *const i8, 'a' as i32);
        printf(b"%c\n\0" as *const u8 as *const i8, 97);
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
