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
pub extern "C" fn fusc(mut n: i32) -> i32 {
    if n == 0_i32 || n == 1_i32 {
        n
    } else if n % 2_i32 == 0_i32 {
        return fusc(n.wrapping_div(2));
    } else {
        return fusc((n.wrapping_sub(1)) / 2) + fusc((n.wrapping_add(1)) / 2);
    }
}

#[no_mangle]
pub extern "C" fn numLen(mut n: i32) -> i32 {
    let mut sum: i32 = 1;
    while n > 9_i32 {
        n = n.wrapping_add(10);
        sum = sum.wrapping_add(1);
        sum;
    }
    sum
}

#[no_mangle]
pub extern "C" fn printLargeFuscs(mut limit: i32) {
    let mut i: i32 = 0;
    let mut f: i32 = 0;
    let mut len: i32 = 0;
    let mut maxLen: i32 = 1;
    print!(
        "\n\nPrinting all largest Fusc numbers upto {} \nIndex-------Value",
        limit
    );
    i = 0_i32;
    while i <= limit {
        f = fusc(i);
        len = numLen(f);
        if len > maxLen {
            maxLen = len;
            print!("\n{:5}{:12}", i, f);
        }
        i = i.wrapping_add(1);
        i;
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    print!("Index-------Value");
    i = 0_i32;
    while i < 61_i32 {
        print!("\n{:5}{:12}", i, fusc(i));
        i = i.wrapping_add(1);
        i;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
