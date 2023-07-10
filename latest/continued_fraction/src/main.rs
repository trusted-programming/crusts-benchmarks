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
extern "C" {}
// SAFETY: machine generated unsafe code
pub type coeff_func = Option<unsafe extern "C" fn(u32) -> f64>;
#[no_mangle]
pub extern "C" fn calc(mut f_a: coeff_func, mut f_b: coeff_func, mut expansions: u32) -> f64 {
    let mut a: f64 = 0.;
    let mut b: f64 = 0.;
    let mut r: f64 = 0.;
    r = 0.0f64;
    b = r;
    a = b;
    let mut i: u32 = 0;
    i = expansions;
// SAFETY: machine generated unsafe code
    unsafe {
        while i > 0 {
            match f_a {
                Some(f_a_m) => a = f_a_m(i),
                None => panic!("non-null function pointer"),
            }
            match f_b {
                Some(f_b_m) => b = f_b_m(i),
                None => panic!("non-null function pointer"),
            }
            r = b / (a.wrapping_add(r));
            i = i.wrapping_sub(1);
            i;
        }
        match f_a {
            Some(f_a_m) => a = f_a_m(0),
            None => panic!("non-null function pointer"),
        }
    }
    return a.wrapping_add(r);
}

#[no_mangle]
pub extern "C" fn sqrt2_a(mut n: u32) -> f64 {
    return if n != 0 { 2.0f64 } else { 1.0f64 };
}

#[no_mangle]
pub extern "C" fn sqrt2_b(mut n: u32) -> f64 {
    return 1.0f64;
}

#[no_mangle]
pub extern "C" fn napier_a(mut n: u32) -> f64 {
    return if n != 0 { n as f64 } else { 2.0f64 };
}

#[no_mangle]
pub extern "C" fn napier_b(mut n: u32) -> f64 {
    return if n as f64 > 1.0f64 {
        n as f64 - 1.0f64
    } else {
        1.0f64
    };
}

#[no_mangle]
pub extern "C" fn pi_a(mut n: u32) -> f64 {
    return if n != 0 { 6.0f64 } else { 3.0f64 };
}

#[no_mangle]
pub extern "C" fn pi_b(mut n: u32) -> f64 {
    let mut c: f64 = 2.0f64 * n as f64 - 1.0f64;
    return c.wrapping_mul(c);
}

fn main_0() -> i32 {
    let mut sqrt2: f64 = 0.;
    let mut napier: f64 = 0.;
    let mut pi: f64 = 0.;
    sqrt2 = calc(
// SAFETY: machine generated unsafe code
        Some(sqrt2_a as unsafe extern "C" fn(u32) -> f64),
// SAFETY: machine generated unsafe code
        Some(sqrt2_b as unsafe extern "C" fn(u32) -> f64),
        1000,
    );
    napier = calc(
// SAFETY: machine generated unsafe code
        Some(napier_a as unsafe extern "C" fn(u32) -> f64),
// SAFETY: machine generated unsafe code
        Some(napier_b as unsafe extern "C" fn(u32) -> f64),
        1000,
    );
    pi = calc(
// SAFETY: machine generated unsafe code
        Some(pi_a as unsafe extern "C" fn(u32) -> f64),
// SAFETY: machine generated unsafe code
        Some(pi_b as unsafe extern "C" fn(u32) -> f64),
        1000,
    );
    print!("{:12.10}\n{:12.10}\n{:12.10}\n", sqrt2, napier, pi);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
