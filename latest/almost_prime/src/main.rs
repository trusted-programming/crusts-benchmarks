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
pub extern "C" fn kprime(mut n: i32, mut k: i32) -> i32 {
    let mut p: i32 = 0;
    let mut f: i32 = 0;
    p = 2_i32;
    while f < k && p * p <= n {
        while 0_i32 == n % p {
            n /= p;
            f += 1_i32;
            f;
        }
        p += 1_i32;
        p;
    }
    i32::from(f + i32::from(n > 1i32) == k)
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut c: i32 = 0;
    let mut k: i32 = 0;
    k = 1_i32;
    while k <= 5_i32 {
        print!("k = {}:", k);
        i = 2_i32;
        c = 0_i32;
        while c < 10_i32 {
            if kprime(i, k) != 0_i32 {
                print!(" {}", i);
                c += 1_i32;
                c;
            }
            i += 1_i32;
            i;
        }
        print!("{}", '\n' as i32);
        k += 1_i32;
        k;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
