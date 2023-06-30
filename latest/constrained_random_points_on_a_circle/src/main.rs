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
pub extern "C" fn randn(mut m: i32) -> i32 {
    let mut rand_max: i32 = 2147483647 - 2147483647 % m;
    let mut r: i32 = 0;
    unsafe {
        loop {
            r = rand();
            if r <= rand_max {
                break;
            }
        }
    }
    r / (rand_max / m)
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut r2: i32 = 0;
    let mut buf: [u64; 31] = [0; 31];
    i = 0_i32;
    while i < 100_i32 {
        x = randn(31) - 15_i32;
        y = randn(31) - 15_i32;
        r2 = x * x + y * y;
        if (100_i32..=225_i32).contains(&r2) {
            buf[(15_i32 + y) as usize] |= (1i32 << (x + 15_i32)) as u64;
            i = i.wrapping_add(1);
            i;
        }
    }
    y = 0_i32;
    while y < 31_i32 {
        x = 0_i32;
        while x < 31_i32 {
            if buf[y as usize] & (1i32 << x) as u64 != 0 {
                print!(". ")
            } else {
                print!("  ")
            };
            x = x.wrapping_add(1);
            x;
        }
        println!();
        y = y.wrapping_add(1);
        y;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
