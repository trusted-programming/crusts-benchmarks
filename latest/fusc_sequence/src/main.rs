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
        return fusc(n / 2);
    } else {
        return fusc((n - 1) / 2) + fusc((n + 1) / 2);
    }
}

#[no_mangle]
pub extern "C" fn numLen(mut n: i32) -> i32 {
    let mut sum: i32 = 1;
    while n > 9_i32 {
        n /= 10_i32;
        sum += 1_i32;
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
        i += 1_i32;
        i;
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    print!("Index-------Value");
    i = 0_i32;
    while i < 61_i32 {
        print!("\n{:5}{:12}", i, fusc(i));
        i += 1_i32;
        i;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
