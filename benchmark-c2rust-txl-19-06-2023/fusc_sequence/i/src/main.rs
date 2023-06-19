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
pub extern "C" fn fusc(mut n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    } else if n % 2 == 0 {
        return fusc(n / 2);
    } else {
        return fusc((n - 1) / 2) + fusc((n + 1) / 2);
    };
}

#[no_mangle]
pub extern "C" fn numLen(mut n: i32) -> i32 {
    let mut sum: i32 = 1;
    while n > 9 {
        n = n / 10;
        sum += 1;
        sum;
    }
    return sum;
}

#[no_mangle]
pub extern "C" fn printLargeFuscs(mut limit: i32) {
    let mut i: i32 = 0;
    let mut f: i32 = 0;
    let mut len: i32 = 0;
    let mut maxLen: i32 = 1;
    unsafe {
        printf(
            b"\n\nPrinting all largest Fusc numbers upto %d \nIndex-------Value\0" as *const u8
                as *const i8,
            limit,
        );
    }
    i = 0;
    unsafe {
        while i <= limit {
            f = fusc(i);
            len = numLen(f);
            if len > maxLen {
                maxLen = len;
                printf(b"\n%5d%12d\0" as *const u8 as *const i8, i, f);
            }
            i += 1;
            i;
        }
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    unsafe {
        printf(b"Index-------Value\0" as *const u8 as *const i8);
    }
    i = 0;
    unsafe {
        while i < 61 {
            printf(b"\n%5d%12d\0" as *const u8 as *const i8, i, fusc(i));
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
