#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {}
#[no_mangle]
pub extern "C" fn semiprime(mut n: i32) -> i32 {
    let mut p: i32 = 0;
    let mut f: i32 = 0;
    p = 2_i32;
    while f < 2_i32 && p * p <= n {
        while 0_i32 == n % p {
            n /= p;
            f += 1_i32;
            f;
        }
        p += 1_i32;
        p;
    }
    i32::from(f + i32::from(n > 1i32) == 2_i32)
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    i = 2_i32;
    while i < 100_i32 {
        if semiprime(i) != 0_i32 {
            print!(" {}", i);
        }
        i += 1_i32;
        i;
    }
    print!("{}", '\n' as i32);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
