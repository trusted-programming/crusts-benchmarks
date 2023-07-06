#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]

extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct Transition {
    pub a: u8,
    pub b: u8,
    pub c: u32,
}
#[no_mangle]
pub static mut transitions: [Transition; 100] = [Transition { a: 0, b: 0, c: 0 }; 100];
#[no_mangle]
pub extern "C" fn init() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 10_i32 {
            j = 0_i32;
            while j < 10_i32 {
                let mut idx: i32 = i * 10 + j;
                transitions[idx as usize].a = i as u8;
                transitions[idx as usize].b = j as u8;
                transitions[idx as usize].c = 0;
                j += 1_i32;
                j;
            }
            i += 1_i32;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn record(mut prev: i32, mut curr: i32) {
    let mut pd: u8 = (prev % 10i32) as u8;
    let mut cd: u8 = (curr % 10i32) as u8;
    let mut i: i32 = 0;
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 100_i32 {
            let mut _z: i32 = 0;
            if i32::from(transitions[i as usize].a) == i32::from(pd) {
                let mut _t: i32 = 0;
                if i32::from(transitions[i as usize].b) == i32::from(cd) {
                    transitions[i as usize].c = (transitions[i as usize].c).wrapping_add(1);
                    transitions[i as usize].c;
                    break;
                }
            }
            i += 1_i32;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn printTransitions(mut limit: i32, mut last_prime: i32) {
    let mut i: i32 = 0;
    println!("{} primes, last prime considered: {}", limit, last_prime);
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 100_i32 {
            if transitions[i as usize].c > 0 {
                println!(
                    "{}->{}  count: {:5}  frequency: {:.2}",
                    i32::from(transitions[i as usize].a),
                    i32::from(transitions[i as usize].b),
                    transitions[i as usize].c,
                    100.0f64 * f64::from(transitions[i as usize].c) / f64::from(limit)
                );
            }
            i += 1_i32;
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn isPrime(mut n: i32) -> bool {
    let mut s: i32 = 0;
    let mut t: i32 = 0;
    let mut a1: i32 = 0;
    let mut a2: i32 = 0;
    if n % 2_i32 == 0_i32 {
        return n == 2_i32;
    }
    if n % 3_i32 == 0_i32 {
        return n == 3_i32;
    }
    if n % 5_i32 == 0_i32 {
        return n == 5_i32;
    }
    if n % 7_i32 == 0_i32 {
        return n == 7_i32;
    }
    if n % 11_i32 == 0_i32 {
        return n == 11_i32;
    }
    if n % 13_i32 == 0_i32 {
        return n == 13_i32;
    }
    if n % 17_i32 == 0_i32 {
        return n == 17_i32;
    }
    if n % 19_i32 == 0_i32 {
        return n == 19_i32;
    }
    t = 23_i32;
    a1 = 96_i32;
    a2 = 216_i32;
    s = t * t;
// SAFETY: machine generated unsafe code
    unsafe {
        while s <= n {
            if n % t == 0_i32 {
                return 0_i32 != 0_i32;
            }
            s += a1;
            t += 2_i32;
            a1 += 24_i32;
            if t * t == s {
            } else {
                __assert_fail(
                    (b"t * t == s\0" as *const u8).cast::<i8>(),
                    (b"main.c\0" as *const u8).cast::<i8>(),
                    77,
                    (*::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b"_Bool isPrime(int)\0"))
                        .as_ptr(),
                );
            }
            if t * t == s {
                } else {
                    __assert_fail(
                        (b"t * t == s\0" as *const u8).cast::<i8>(),
                        (b"main.c\0" as *const u8).cast::<i8>(),
                        77,
                        (*::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b"_Bool isPrime(int)\0"))
                            .as_ptr(),
                    );
                };
            if s <= n {
                if n % t == 0_i32 {
                    return 0_i32 != 0_i32;
                }
                s += a2;
                t += 4_i32;
                a2 += 48_i32;
                if t * t == s {
                } else {
                    __assert_fail(
                        (b"t * t == s\0" as *const u8).cast::<i8>(),
                        (b"main.c\0" as *const u8).cast::<i8>(),
                        86,
                        (*::core::mem::transmute::<&[u8; 19], &[i8; 19]>(b"_Bool isPrime(int)\0"))
                            .as_ptr(),
                    );
                }
                if t * t == s {
                    } else {
                        __assert_fail(
                            (b"t * t == s\0" as *const u8).cast::<i8>(),
                            (b"main.c\0" as *const u8).cast::<i8>(),
                            86,
                            (*::core::mem::transmute::<&[u8; 19], &[i8; 19]>(
                                b"_Bool isPrime(int)\0",
                            ))
                            .as_ptr(),
                        );
                    };
            }
        }
    }
    1_i32 != 0_i32
}

fn main_0() -> i32 {
    let mut last_prime: i32 = 3;
    let mut n: i32 = 5;
    let mut count: i32 = 2;
    init();
    record(2, 3);
    while count < 1_000_000_i32 {
        if isPrime(n) {
            record(last_prime, n);
            last_prime = n;
            count += 1_i32;
            count;
        }
        n += 2_i32;
        if count < 1_000_000_i32 {
            if isPrime(n) {
                record(last_prime, n);
                last_prime = n;
                count += 1_i32;
                count;
            }
            n += 4_i32;
        }
    }
    printTransitions(1000000, last_prime);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
