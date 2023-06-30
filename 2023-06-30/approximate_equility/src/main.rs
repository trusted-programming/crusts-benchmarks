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
    fn sqrt(_: f64) -> f64;
    fn fabs(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn approxEquals(mut value: f64, mut other: f64, mut epsilon: f64) -> bool {
    unsafe {
        fabs(value - other) < epsilon
    }
}

#[no_mangle]
pub extern "C" fn test(mut a: f64, mut b: f64) {
    let mut epsilon: f64 = 1e-18f64;
    println!("{}, {} => {}", a, b, i32::from(approxEquals(a, b, epsilon)));
}

fn main_0() -> i32 {
    test(100000000000000.01f64, 100_000_000_000_000.02_f64);
    test(100.01f64, 100.011f64);
    test(10000000000000.001f64 / 10000.0f64, 1_000_000_000.000_000_1_f64);
    test(0.001f64, 0.0010000001f64);
    test(0.000000000000000000000101f64, 0.0f64);
    unsafe {
        test(sqrt(2.0f64) * sqrt(2.0f64), 2.0f64);
        test(-sqrt(2.0f64) * sqrt(2.0f64), -2.0f64);
    }
    test(3.141_592_653_589_793_f64, 3.141_592_653_589_793_f64);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
