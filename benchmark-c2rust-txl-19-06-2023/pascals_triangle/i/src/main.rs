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
pub static mut N: i32 = 15;
fn main_0() -> i32 {
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut num: u64 = 0;
    let mut den: u64 = 0;
    let mut catalan: i32 = 0;
    unsafe {
        printf(b"1 \0" as *const u8 as *const i8);
    }
    n = 2;
    unsafe {
        while n <= N {
            den = 1;
            num = den;
            k = 2;
            while k <= n {
                num = num.wrapping_mul((n + k) as u64);
                den = den.wrapping_mul(k as u64);
                catalan = num.wrapping_div(den) as i32;
                k += 1;
                k;
            }
            printf(b"%d \0" as *const u8 as *const i8, catalan);
            n += 1;
            n;
        }
        printf(b"\n\0" as *const u8 as *const i8);
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
