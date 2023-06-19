#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn printf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub extern "C" fn jacobi(mut a: u64, mut n: u64) -> i32 {
    if a >= n {
        a = a.wrapping_rem(n);
    }
    let mut result: i32 = 1;
    while a != 0 {
        while a & 1 == 0 {
            a >>= 1;
            if n & 7 == 3 || n & 7 == 5 {
                result = -result;
            }
        }
        a ^= n;
        n ^= a;
        a ^= n;
        if a & 3 == 3 && n & 3 == 3 {
            result = -result;
        }
        a = a.wrapping_rem(n);
    }
    if n == 1 {
        return result;
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn print_table(mut kmax: u32, mut nmax: u32) {
    unsafe {
        printf(b"n\\k|\0" as *const u8 as *const i8);
    }
    let mut k: i32 = 0;
    unsafe {
        while k as u32 <= kmax {
            printf(b"%'3u\0" as *const u8 as *const i8, k);
            k += 1;
            k;
        }
        printf(b"\n----\0" as *const u8 as *const i8);
    }
    let mut k_0: i32 = 0;
    unsafe {
        while k_0 as u32 <= kmax {
            printf(b"---\0" as *const u8 as *const i8);
            k_0 += 1;
            k_0;
        }
    }
    print!("{}", '\n' as i32);
    let mut n: i32 = 1;
    unsafe {
        while n as u32 <= nmax {
            printf(b"%-2u |\0" as *const u8 as *const i8, n);
            let mut k_1: i32 = 0;
            while k_1 as u32 <= kmax {
                printf(
                    b"%'3d\0" as *const u8 as *const i8,
                    jacobi(k_1 as u64, n as u64),
                );
                k_1 += 1;
                k_1;
            }
            print!("{}", '\n' as i32);
            n += 2;
        }
    }
}

fn main_0() -> i32 {
    print_table(20, 21);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
