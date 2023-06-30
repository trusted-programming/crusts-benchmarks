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
pub extern "C" fn isPrime(mut n: i32) -> i32 {
    if n % 2_i32 == 0_i32 {
        return i32::from(n == 2_i32);
    }
    if n % 3_i32 == 0_i32 {
        return i32::from(n == 3_i32);
    }
    let mut d: i32 = 5;
    while d * d <= n {
        if n % d == 0_i32 {
            return 0_i32;
        }
        d = d.wrapping_add(2);
        if n % d == 0_i32 {
            return 0_i32;
        }
        d = d.wrapping_add(4);
    }
    1_i32
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut d: i32 = 0;
    let mut p: i32 = 0;
    let mut r: i32 = 0;
    let mut q: i32 = 929;
    if isPrime(q) == 0_i32 {
        return 1_i32;
    }
    r = q;
    while r > 0_i32 {
        r <<= 1_i32;
    }
    d = 2_i32 * q + 1_i32;
    loop {
        p = r;
        i = 1_i32;
        while p != 0_i32 {
            i = (i64::from(i) * i64::from(i) % i64::from(d)) as i32;
            if p < 0_i32 {
                i *= 2_i32;
            }
            if i > d {
                i = i.wrapping_sub(d);
            }
            p <<= 1_i32;
        }
        if i == 1_i32 {
            break;
        }
        d += 2_i32 * q;
    }
    println!("2^{} - 1 = 0 (mod {})", q, d);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
