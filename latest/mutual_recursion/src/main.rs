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
pub extern "C" fn F(n: i32) -> i32 {
    if n == 0_i32 { 1_i32 } else { n - M(F(n - 1)) }
}

#[no_mangle]
pub extern "C" fn M(n: i32) -> i32 {
    if n == 0_i32 { 0_i32 } else { n - F(M(n - 1)) }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    i = 0_i32;
    while i < 20_i32 {
        print!("{:2} ", F(i));
        i += 1_i32;
        i;
    }
    println!();
    i = 0_i32;
    while i < 20_i32 {
        print!("{:2} ", M(i));
        i += 1_i32;
        i;
    }
    println!();
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
