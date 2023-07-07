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
pub extern "C" fn gcd(mut m: i32, mut n: i32) -> i32 {
    let mut tmp: i32 = 0;
    while m != 0_i32 {
        tmp = m;
        m = n.wrapping_rem(m);
        n = tmp;
    }
    n
}

#[no_mangle]
pub extern "C" fn lcm(mut m: i32, mut n: i32) -> i32 {
    m / gcd(m, n) * n
}

fn main_0() -> i32 {
    println!("lcm(35, 21) = {}", lcm(21, 35));
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
