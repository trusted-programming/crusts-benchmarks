#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {
    fn rand() -> i32;
    fn cos(_: f64) -> f64;
    fn log(_: f64) -> f64;
    fn sqrt(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn drand() -> f64 {
// SAFETY: machine generated unsafe code
    unsafe {
        (f64::from(rand()) + 1.0f64) / (2147483647_f64 + 1.0f64)
    }
}

#[no_mangle]
pub extern "C" fn random_normal() -> f64 {
// SAFETY: machine generated unsafe code
    unsafe {
        sqrt(-2_f64 * log(drand()))
            * cos(2_f64 * 3.141_592_653_589_793_f64 * drand())
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut rands: [f64; 1000] = [0.; 1000];
    i = 0_i32;
    while i < 1_000_i32 {
        rands[i as usize] = 0.5f64.mul_add(random_normal(), 1.0f64);
        i = i.wrapping_add(1);
        i;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
