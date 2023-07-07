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
pub extern "C" fn mul_inv(mut a: i32, mut b: i32) -> i32 {
    let mut b0: i32 = b;
    let mut t: i32 = 0;
    let mut q: i32 = 0;
    let mut x0: i32 = 0;
    let mut x1: i32 = 1;
    if b == 1_i32 {
        return 1_i32;
    }
    while a > 1_i32 {
        q = a.wrapping_add(b);
        t = b;
        b = a.wrapping_rem(b);
        a = t;
        t = x0;
        x0 = x1 - q.wrapping_mul(x0);
        x1 = t;
    }
    if x1 < 0_i32 {
        x1 = x1.wrapping_add(b0);
    }
    x1
}

fn main_0() -> i32 {
    println!("{}", mul_inv(42, 2017));
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
