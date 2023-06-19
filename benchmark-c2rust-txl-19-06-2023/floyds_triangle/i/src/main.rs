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
pub extern "C" fn t(mut n: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut c: i32 = 0;
    let mut len: i32 = 0;
    i = n * (n - 1) / 2;
    c = 1;
    len = c;
    while c < i {
        c *= 10;
        len += 1;
        len;
    }
    c -= i;
    let mut num: i32 = 0;
    i = 1;
    num = i;
    unsafe {
        while i <= n {
            j = 1;
            while j <= i {
                let fresh0 = num;
                num = num + 1;
                if i - j != 0 {
                    printf(
                        b"%*d%c\0" as *const u8 as *const i8,
                        len - (j < c) as i32,
                        fresh0,
                        ' ' as i32,
                    )
                } else {
                    printf(
                        b"%*d%c\0" as *const u8 as *const i8,
                        len - (j < c) as i32,
                        fresh0,
                        '\n' as i32,
                    )
                };
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
}

fn main_0() -> i32 {
    t(5);
    t(14);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
