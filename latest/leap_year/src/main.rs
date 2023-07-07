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
pub extern "C" fn is_leap_year(mut year: i32) -> i32 {
    if year % 4_i32 == 0_i32 && year % 100_i32 != 0_i32 || year % 400_i32 == 0_i32 {
        1_i32
    } else {
        0_i32
    }
}

fn main_0() -> i32 {
    let mut test_case: [i32; 5] = [1900, 1994, 1996, 1997, 2000];
    let mut key: i32 = 0;
    let mut end: i32 = 0;
    let mut year: i32 = 0;
    key = 0_i32;
    end = (::core::mem::size_of::<[i32; 5]>() as u64)
        .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32;
    while key < end {
        year = test_case[key as usize];
        if is_leap_year(year) == 1_i32 {
            println!("{} is \0a leap year.", year)
        } else {
            println!("{} is not \0a leap year.", year)
        };
        key = key.wrapping_add(1);
        key;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
