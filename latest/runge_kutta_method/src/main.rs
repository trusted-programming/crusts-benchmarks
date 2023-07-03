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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn pow(_: f64, _: f64) -> f64;
    fn sqrt(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn rk4(
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    mut f: Option<unsafe extern "C" fn(f64, f64) -> f64>,
    mut dx: f64,
    mut x: f64,
    mut y: f64,
) -> f64 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut k1: f64 = dx * f.expect("non-null function pointer")(x, y);
        let mut k2: f64 =
            dx * f.expect("non-null function pointer")(x + dx / 2_f64, y + k1 / 2_f64);
        let mut k3: f64 =
            dx * f.expect("non-null function pointer")(x + dx / 2_f64, y + k2 / 2_f64);
        let mut k4: f64 = dx * f.expect("non-null function pointer")(x + dx, y + k3);
        y + (2_f64.mul_add(k3, 2_f64.mul_add(k2, k1)) + k4) / 6_f64
    }
}

#[no_mangle]
pub extern "C" fn rate(mut x: f64, mut y: f64) -> f64 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        x * sqrt(y)
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut y: *mut f64 = std::ptr::null_mut::<f64>();
        let mut x: f64 = 0.;
        let mut y2: f64 = 0.;
        let mut x0: f64 = f64::from(0_i32);
        let mut x1: f64 = 10_f64;
        let mut dx: f64 = 0.1f64;
        let mut i: i32 = 0;
        let mut n: i32 = (1_f64 + (x1 - x0) / dx) as i32;
        y = malloc((::core::mem::size_of::<f64>() as u64).wrapping_mul(n as u64)).cast::<f64>();
        *y.offset(0_isize) = 1_f64;
        i = 1_i32;
        while i < n {
            *y.offset(i as isize) = rk4(
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
                Some(rate as unsafe extern "C" fn(f64, f64) -> f64),
                dx,
                dx.mul_add(f64::from(i - 1i32), x0),
                *y.offset((i - 1i32) as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        print!("x	y	rel. err.\n------------\n");
        i = 0_i32;
        while i < n {
            x = dx.mul_add(f64::from(i), x0);
            y2 = pow(x * x / 4_f64 + 1_f64, 2_f64);
            println!(
                "{}	{}	{}",
                x,
                *y.offset(i as isize),
                *y.offset(i as isize) / y2 - 1_f64
            );
            i = i.wrapping_add(10);
        }
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
