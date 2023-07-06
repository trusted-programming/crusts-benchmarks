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
    fn asin(_: f64) -> f64;
    fn cos(_: f64) -> f64;
    fn sin(_: f64) -> f64;
    fn sqrt(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn dist(mut th1: f64, mut ph1: f64, mut th2: f64, mut ph2: f64) -> f64 {
    let mut dx: f64 = 0.;
    let mut dy: f64 = 0.;
    let mut dz: f64 = 0.;
    ph1 -= ph2;
    ph1 *= 3.1415926536f64 / 180_f64;
    th1 *= 3.1415926536f64 / 180_f64;
    th2 *= 3.1415926536f64 / 180_f64;
// SAFETY: machine generated unsafe code
    unsafe {
        dz = sin(th1) - sin(th2);
        dx = cos(ph1).mul_add(cos(th1), -cos(th2));
        dy = sin(ph1) * cos(th1);
        asin(sqrt(dz.mul_add(dz, dx.mul_add(dx, dy * dy))) / 2_f64) * 2_f64 * 6371_f64
    }
}

fn main_0() -> i32 {
    let mut d: f64 = dist(36.12f64, -86.67f64, 33.94f64, -118.4f64);
    println!("dist: {:.1} km ({:.1} mi.)", d, d / 1.609344f64);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
