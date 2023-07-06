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
    let mut current: i32 = 0;
    let mut square: i32 = 0;
    loop {
        square = current * current;
        if !(square % 1_000_000_i32 != 269_696_i32 && square < 2_147_483_647_i32) {
            break;
        }
        current += 1_i32;
        current;
    }
    if square > 2_147_483_647_i32 {
        print!("Condition not satisfied before INT_MAX reached.");
    } else {
        println!(
            "The smallest number whose square ends in 269696 is {}",
            current
        );
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
