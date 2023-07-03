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
#[no_mangle]
pub extern "C" fn egyptian_division(
    mut dividend: u64,
    mut divisor: u64,
    mut remainder: *mut u64,
) -> u64 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        static mut powers: [u64; 64] = [0; 64];
        static mut doublings: [u64; 64] = [0; 64];
        let mut i: i32 = 0;
        i = 0_i32;
        while i < 64_i32 {
            powers[i as usize] = (1i32 << i) as u64;
            doublings[i as usize] = divisor << i;
            if doublings[i as usize] > dividend {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
        let mut answer: u64 = 0;
        let mut accumulator: u64 = 0;
        i = i.wrapping_sub(1);
        while i >= 0_i32 {
            if accumulator.wrapping_add(doublings[i as usize]) <= dividend {
                accumulator = (accumulator).wrapping_add(doublings[i as usize]);
                answer = (answer).wrapping_add(powers[i as usize]);
            }
            i = i.wrapping_sub(1);
            i;
        }
        if !remainder.is_null() {
            *remainder = dividend.wrapping_sub(accumulator);
        }
        answer
    }
}

#[no_mangle]
pub extern "C" fn go(mut a: u64, mut b: u64) {
    let mut x: u64 = 0;
    let mut y: u64 = 0;
    x = egyptian_division(a, b, &mut y);
    println!("{} / {} = {} remainder {}", a, b, x, y);
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        if a == b.wrapping_mul(x).wrapping_add(y) {
        } else {
            __assert_fail(
                (b"a == b * x + y\0" as *const u8).cast::<i8>(),
                (b"main.c\0" as *const u8).cast::<i8>(),
                43,
                (*::core::mem::transmute::<&[u8; 28], &[i8; 28]>(b"void go(uint64_t, uint64_t)\0"))
                    .as_ptr(),
            );
        }
        if a == b.wrapping_mul(x).wrapping_add(y) {
            } else {
                __assert_fail(
                    (b"a == b * x + y\0" as *const u8).cast::<i8>(),
                    (b"main.c\0" as *const u8).cast::<i8>(),
                    43,
                    (*::core::mem::transmute::<&[u8; 28], &[i8; 28]>(
                        b"void go(uint64_t, uint64_t)\0",
                    ))
                    .as_ptr(),
                );
            };
    }
}

fn main_0() -> i32 {
    go(580, 32);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
