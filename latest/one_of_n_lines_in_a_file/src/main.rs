#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {
    fn rand() -> i32;
}
#[no_mangle]
pub extern "C" fn irand(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut randmax: i32 = 2147483647 / n.wrapping_mul(n);
// SAFETY: machine generated unsafe code
    unsafe {
        loop {
            r = rand();
            if r < randmax {
                break;
            }
        }
    }
    r / (randmax / n)
}

#[no_mangle]
pub extern "C" fn one_of_n(mut n: i32) -> i32 {
    let mut i: i32 = 0;
    let mut r: i32 = 0;
    i = 1_i32;
    while i < n {
        if irand(i.wrapping_add(1)) == 0_i32 {
            r = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    r
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut r: [i32; 10] = [0; 10];
    i = 0_i32;
    while i < 1_000_000_i32 {
        i = i.wrapping_add(1);
        i;
        r[one_of_n(10) as usize] += 1_i32;
        r[one_of_n(10) as usize];
    }
    i = 0_i32;
    while i < 10_i32 {
        if i == 9_i32 {
            print!("{}{}", r[i as usize], '\n' as i32)
        } else {
            print!("{}{}", r[i as usize], ' ' as i32)
        };
        i = i.wrapping_add(1);
        i;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
