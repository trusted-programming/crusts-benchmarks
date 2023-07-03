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
    fn pow(_: f64, _: f64) -> f64;
    fn sqrt(_: f64) -> f64;
    fn ceil(_: f64) -> f64;
    fn floor(_: f64) -> f64;
}
#[no_mangle]
pub static mut shades: *const i8 = (b".:!*oe&#%@\0" as *const u8).cast::<i8>();
#[no_mangle]
pub static mut light: [f64; 3] = [30_f64, 30_f64, -50_f64];
#[no_mangle]
pub extern "C" fn normalize(mut v: *mut f64) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut len: f64 = sqrt(
            (*v.offset(2_isize)).mul_add(*v.offset(2_isize), (*v.offset(0_isize)).mul_add(*v.offset(0_isize), *v.offset(1_isize) * *v.offset(1_isize))),
        );
        *v.offset(0_isize) /= len;
        *v.offset(1_isize) /= len;
        *v.offset(2_isize) /= len;
    }
}

#[no_mangle]
pub extern "C" fn dot(mut x: *mut f64, mut y: *mut f64) -> f64 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut d: f64 = (*x.offset(2_isize)).mul_add(*y.offset(2_isize), (*x.offset(0_isize)).mul_add(*y.offset(0_isize), *x.offset(1_isize) * *y.offset(1_isize)));
        if d < f64::from(0_i32) { -d } else { f64::from(0_i32) }
    }
}

#[no_mangle]
pub extern "C" fn draw_sphere(mut R: f64, mut k: f64, mut ambient: f64) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut intensity: i32 = 0;
    let mut b: f64 = 0.;
    let mut vec: [f64; 3] = [0.; 3];
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        i = floor(-R) as i32;
        while f64::from(i) <= ceil(R) {
            x = f64::from(i) + 0.5f64;
            j = floor(-2_f64 * R) as i32;
            while f64::from(j) <= ceil(2_f64 * R) {
                y = f64::from(j) / 2.0f64 + 0.5f64;
                if x.mul_add(x, y * y) <= R * R {
                    vec[0_usize] = x;
                    vec[1_usize] = y;
                    vec[2_usize] = sqrt(y.mul_add(-y, R.mul_add(R, -x * x)));
                    normalize(vec.as_mut_ptr());
                    b = pow(dot(light.as_mut_ptr(), vec.as_mut_ptr()), k) + ambient;
                    intensity = ((1_f64 - b)
                        * (::core::mem::size_of::<*const i8>() as u64).wrapping_sub(1u64) as f64)
                        as i32;
                    if intensity < 0_i32 {
                        intensity = 0_i32;
                    }
                    if intensity as u64
                        >= (::core::mem::size_of::<*const i8>() as u64).wrapping_sub(1)
                    {
                        intensity =
                            (::core::mem::size_of::<*const i8>() as u64).wrapping_sub(2) as i32;
                    }
                    print!("{}", i32::from(*shades.offset(intensity as isize)));
                } else {
                    print!("{}", ' ' as i32);
                }
                j = j.wrapping_add(1);
                j;
            }
            print!("{}", '\n' as i32);
            i = i.wrapping_add(1);
            i;
        }
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        normalize(light.as_mut_ptr());
    }
    draw_sphere(20_f64, 4_f64, 0.1f64);
    draw_sphere(10_f64, 2_f64, 0.4f64);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
