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
pub extern "C" fn x(mut i: f64) -> f64 {
// SAFETY: machine generated unsafe code
    unsafe {
        static mut _n: f64 = 1.0f64;
        _n += i;
        _n
    }
}

#[no_mangle]
pub extern "C" fn y(mut i: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        static mut _n: i32 = 3_i32;
        _n += i;
        _n
    }
}

#[no_mangle]
pub extern "C" fn z(mut i: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        static mut _n: i32 = 'a' as i32;
        _n += i;
        _n
    }
}

fn main_0() -> i32 {
    println!("{}", x(5_f64));
    println!("{}", x(2.3f64));
    println!("{}", y(5.0f64 as i32));
    println!("{}", y(3.3f64 as i32));
    println!("{}", z(5));
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
