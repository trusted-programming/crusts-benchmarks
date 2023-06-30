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
pub extern "C" fn proper_divisors(n: i32, mut print_flag: bool) -> i32 {
    let mut count: i32 = 0;
    let mut i: i32 = 1;
    while i < n {
        if n % i == 0_i32 {
            count = count.wrapping_add(1);
            count;
            if print_flag {
                print!("{} ", i);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if print_flag {
        println!();
    }
    count
}

#[no_mangle]
pub extern "C" fn countProperDivisors(mut n: i32) -> i32 {
    let mut prod: i32 = 1;
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    while n % 2_i32 == 0_i32 {
        count = count.wrapping_add(1);
        count;
        n = n.wrapping_div(2);
    }
    prod *= 1_i32 + count;
    i = 3_i32;
    while i * i <= n {
        count = 0_i32;
        while n % i == 0_i32 {
            count = count.wrapping_add(1);
            count;
            n = n.wrapping_div(i);
        }
        prod *= 1_i32 + count;
        i = i.wrapping_add(2);
    }
    if n > 2_i32 {
        prod *= 2_i32;
    }
    prod.wrapping_sub(1)
}

fn main_0() -> i32 {
    let mut i: i32 = 1;
    while i <= 10_i32 {
        print!("{}: ", i);
        proper_divisors(i, 1_i32 != 0_i32);
        i = i.wrapping_add(1);
        i;
    }
    let mut max: i32 = 0;
    let mut max_i: i32 = 1;
    let mut i_0: i32 = 1;
    while i_0 <= 20_000_i32 {
        let mut v: i32 = countProperDivisors(i_0);
        if v >= max {
            max = v;
            max_i = i_0;
        }
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    println!("{} with {} divisors", max_i, max);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
