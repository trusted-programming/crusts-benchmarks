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
pub extern "C" fn a(mut in_0: bool) -> bool {
    println!("I am a");
    in_0
}

#[no_mangle]
pub extern "C" fn b(mut in_0: bool) -> bool {
    println!("I am b");
    in_0
}

fn main_0() -> i32 {
    let mut x: bool = false;
    x = i32::from(a(0_i32 != 0_i32)) != 0_i32 && i32::from(b(1_i32 != 0_i32)) != 0_i32;
    if i32::from(x) != 0_i32 {
        print!("false && true = true\0\n\n")
    } else {
        print!("false && true = false\0\n\n")
    };
    x = i32::from(a(1_i32 != 0_i32)) != 0_i32 || i32::from(b(0_i32 != 0_i32)) != 0_i32;
    if i32::from(x) != 0_i32 {
        print!("true || false = true\0\n\n")
    } else {
        print!("true || false = false\0\n\n")
    };
    x = i32::from(a(1_i32 != 0_i32)) != 0_i32 && i32::from(b(0_i32 != 0_i32)) != 0_i32;
    if i32::from(x) != 0_i32 {
        print!("true && false = true\0\n\n")
    } else {
        print!("true && false = false\0\n\n")
    };
    x = i32::from(a(0_i32 != 0_i32)) != 0_i32 || i32::from(b(0_i32 != 0_i32)) != 0_i32;
    if i32::from(x) != 0_i32 {
        print!("false || false = true\0\n\n")
    } else {
        print!("false || false = false\0\n\n")
    };
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
