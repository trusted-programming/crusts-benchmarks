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
    fn puts(__s: *const i8) -> i32;
}
#[no_mangle]
pub extern "C" fn mapRange(mut a1: f64, mut a2: f64, mut b1: f64, mut b2: f64, mut s: f64) -> f64 {
    b1 + (s - a1) * (b2 - b1) / (a2 - a1)
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        puts((b"Mapping [0,10] to [-1,0] at intervals of 1:\0" as *const u8).cast::<i8>());
    }
    i = 0_i32;
    while i <= 10_i32 {
        println!(
            "f({}) = {}",
            i,
            mapRange(f64::from(0_i32), 10_f64, -1_f64, f64::from(0_i32), f64::from(i))
        );
        i = i.wrapping_add(1);
        i;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
