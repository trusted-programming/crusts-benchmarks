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
fn main_0() -> i32 {
    println!("{}", 'a' as i32);
    println!("{}", 97_i32);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
