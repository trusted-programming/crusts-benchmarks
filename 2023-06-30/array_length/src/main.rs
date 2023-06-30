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
    let mut _fruit: [*const i8; 2] = [
        (b"apples\0" as *const u8).cast::<i8>(),
        (b"oranges\0" as *const u8).cast::<i8>(),
    ];
    let mut length: i32 = (::core::mem::size_of::<[*const i8; 2]>() as u64)
        .wrapping_div(::core::mem::size_of::<*const i8>() as u64) as i32;
    println!("{}", length);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
