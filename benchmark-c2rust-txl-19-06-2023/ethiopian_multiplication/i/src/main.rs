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
pub extern "C" fn halve(mut x: *mut i32) {
    unsafe {
        *x >>= 1;
    }
}

#[no_mangle]
pub extern "C" fn doublit(mut x: *mut i32) {
    unsafe {
        *x <<= 1;
    }
}

#[no_mangle]
pub extern "C" fn iseven(x: i32) -> bool {
    return x & 1 == 0;
}

#[no_mangle]
pub extern "C" fn ethiopian(mut plier: i32, mut plicand: i32, tutor: bool) -> i32 {
    let mut result: i32 = 0;
    unsafe {
        if tutor {
            printf(
                b"ethiopian multiplication of %d by %d\n\0" as *const u8 as *const i8,
                plier,
                plicand,
            );
        }
        while plier >= 1 {
            if iseven(plier) {
                if tutor {
                    printf(
                        b"%4d %6d struck\n\0" as *const u8 as *const i8,
                        plier,
                        plicand,
                    );
                }
            } else {
                if tutor {
                    printf(
                        b"%4d %6d kept\n\0" as *const u8 as *const i8,
                        plier,
                        plicand,
                    );
                }
                result += plicand;
            }
            halve(&mut plier);
            doublit(&mut plicand);
        }
    }
    return result;
}

fn main_0() -> i32 {
    unsafe {
        printf(
            b"%d\n\0" as *const u8 as *const i8,
            ethiopian(17, 34, 1 != 0),
        );
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
