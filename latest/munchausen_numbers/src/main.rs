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
}
fn main_0() -> i32 {
    let mut i: i32 = 1;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 5_000_i32 {
            let mut sum: i32 = 0;
            let mut number: i32 = i;
            while number > 0_i32 {
                let mut digit: i32 = number % 10;
                sum = (f64::from(sum) + pow(f64::from(digit), f64::from(digit))) as i32;
                number = number.wrapping_div(10);
            }
            if sum == i {
                println!("{}", i);
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
