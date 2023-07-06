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
pub extern "C" fn is_prime(mut n: i32) -> i32 {
    let mut d: i32 = 5;
    if n < 2_i32 {
        return 0_i32;
    }
    if n % 2_i32 == 0_i32 {
        return i32::from(n == 2_i32);
    }
    if n % 3_i32 == 0_i32 {
        return i32::from(n == 3_i32);
    }
    while d * d <= n {
        if n % d == 0_i32 {
            return 0_i32;
        }
        d += 2_i32;
        if n % d == 0_i32 {
            return 0_i32;
        }
        d += 4_i32;
    }
    1_i32
}

#[no_mangle]
pub extern "C" fn count_prime_factors(mut n: i32) -> i32 {
    let mut count: i32 = 0;
    let mut f: i32 = 2;
    if n == 1_i32 {
        return 0_i32;
    }
    if is_prime(n) != 0_i32 {
        return 1_i32;
    }
    loop {
        if n % f == 0_i32 {
            count += 1_i32;
            count;
            n /= f;
            if n == 1_i32 {
                return count;
            }
            if is_prime(n) != 0_i32 {
                f = n;
            }
        } else if f >= 3_i32 {
            f += 2_i32;
        } else {
            f = 3_i32;
        }
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut count: i32 = 0;
    println!("The attractive numbers up to and including {} are:", 120_i32);
    i = 1_i32;
    while i <= 120_i32 {
        n = count_prime_factors(i);
        if is_prime(n) != 0_i32 {
            print!("{:4}", i);
            count += 1_i32;
            if count % 20_i32 == 0_i32 {
                println!();
            }
        }
        i += 1_i32;
        i;
    }
    println!();
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
