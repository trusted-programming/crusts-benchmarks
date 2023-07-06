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
    let mut i: i32 = 0;
    i = 1_i32;
    while i <= 10_i32 {
        print!("{}", i);
        if i == 10_i32 {
            println!()
        } else {
            print!(", ")
        };
        i += 1_i32;
        i;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
