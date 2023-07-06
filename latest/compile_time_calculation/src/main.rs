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
pub static mut val: i32 = 2_i32 * 3_i32 * 4_i32 * 5_i32 * 6_i32 * 7_i32 * 8_i32 * 9_i32 * 10_i32;
fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        println!("10! = {}", val);
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
