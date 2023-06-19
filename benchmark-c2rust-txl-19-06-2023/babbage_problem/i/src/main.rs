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
    let mut current: i32 = 0;
    let mut square: i32 = 0;
    loop {
        square = current * current;
        if !(square % 1000000 != 269696 && square < 2147483647) {
            break;
        }
        current += 1;
        current;
    }
    unsafe {
        if square > 2147483647 {
            printf(b"Condition not satisfied before INT_MAX reached.\0" as *const u8 as *const i8);
        } else {
            printf(
                b"The smallest number whose square ends in 269696 is %d\n\0" as *const u8
                    as *const i8,
                current,
            );
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
