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
    let mut is_open: [i8; 100] = [0; 100];
    let mut pass: i32 = 0;
    let mut door: i32 = 0;
    pass = 0_i32;
    while pass < 100_i32 {
        door = pass;
        while door < 100_i32 {
            is_open[door as usize] = i8::from(is_open[door as usize] == 0);
            door += pass + 1_i32;
        }
        pass += 1_i32;
        pass;
    }
    door = 0_i32;
    while door < 100_i32 {
        if i32::from(is_open[door as usize]) != 0_i32 {
            println!("door #{} is open\0.", door + 1_i32)
        } else {
            println!("door #{} is closed\0.", door + 1_i32)
        };
        door += 1_i32;
        door;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
