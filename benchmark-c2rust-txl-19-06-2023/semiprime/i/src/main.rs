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
pub extern "C" fn semiprime(mut n: i32) -> i32 {
    let mut p: i32 = 0;
    let mut f: i32 = 0;
    p = 2;
    while f < 2 && p * p <= n {
        while 0 == n % p {
            n /= p;
            f += 1;
            f;
        }
        p += 1;
        p;
    }
    return (f + (n > 1i32) as i32 == 2) as i32;
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    i = 2;
    unsafe {
        while i < 100 {
            if semiprime(i) != 0 {
                printf(b" %d\0" as *const u8 as *const i8, i);
            }
            i += 1;
            i;
        }
    }
    print!("{}", '\n' as i32);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
