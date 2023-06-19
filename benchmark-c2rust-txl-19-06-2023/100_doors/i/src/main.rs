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
    let mut is_open: [i8; 100] = [0; 100];
    let mut pass: i32 = 0;
    let mut door: i32 = 0;
    pass = 0;
    while pass < 100 {
        door = pass;
        while door < 100 {
            is_open[door as usize] = (is_open[door as usize] == 0) as i8;
            door += pass + 1;
        }
        pass += 1;
        pass;
    }
    door = 0;
    unsafe {
        while door < 100 {
            if is_open[door as usize] as i32 != 0 {
                printf(
                    b"door #%d is %s.\n\0" as *const u8 as *const i8,
                    door + 1,
                    b"open\0" as *const u8 as *const i8,
                )
            } else {
                printf(
                    b"door #%d is %s.\n\0" as *const u8 as *const i8,
                    door + 1,
                    b"closed\0" as *const u8 as *const i8,
                )
            };
            door += 1;
            door;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
