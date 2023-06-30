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
pub extern "C" fn is_prime(mut n: u32) -> i32 {
    if n <= 3 {
        i32::from(n > 1)
    } else if n.wrapping_rem(2) == 0 || n.wrapping_rem(3) == 0 {
        return 0_i32;
    } else {
        let mut i: u32 = 0;
        i = 5;
        while i.wrapping_mul(i) <= n {
            if n.wrapping_rem(i) == 0 || n.wrapping_rem(i.wrapping_add(2)) == 0 {
                return 0_i32;
            }
            i = i.wrapping_add(6);
        }
        return 1_i32;
    }
}

#[no_mangle]
pub extern "C" fn carmichael3(mut p1: i32) {
    if is_prime(p1 as u32) == 0_i32 {
        return;
    }
    let mut h3: i32 = 0;
    let mut d: i32 = 0;
    let mut p2: i32 = 0;
    let mut p3: i32 = 0;
    h3 = 1_i32;
    while h3 < p1 {
        d = 1_i32;
        while d < h3 + p1 {
            if (h3 + p1) * (p1 - 1_i32) % d == 0_i32 && (-p1 * p1 % h3 + h3) % h3 == d % h3 {
                p2 = 1_i32 + (p1 - 1_i32) * (h3 + p1) / d;
                if is_prime(p2 as u32) != 0_i32 {
                    p3 = 1_i32 + p1 * p2 / h3;
                    if !(is_prime(p3 as u32) == 0_i32 || p2 * p3 % (p1 - 1_i32) != 1_i32) {
                        println!("{} {} {}", p1, p2, p3);
                    }
                }
            }
            d = d.wrapping_add(1);
            d;
        }
        h3 = h3.wrapping_add(1);
        h3;
    }
}

fn main_0() -> i32 {
    let mut p1: i32 = 0;
    p1 = 2_i32;
    while p1 < 62_i32 {
        carmichael3(p1);
        p1 = p1.wrapping_add(1);
        p1;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
