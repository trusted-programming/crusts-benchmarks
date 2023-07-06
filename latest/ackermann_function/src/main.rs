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
pub extern "C" fn ackermann(mut m: i32, mut n: i32) -> i32 {
    if m == 0_i32 {
        return n + 1_i32;
    }
    if n == 0_i32 {
        return ackermann(m - 1, 1);
    }
    ackermann(m - 1, ackermann(m, n - 1))
}

fn main_0() -> i32 {
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    m = 0_i32;
    while m <= 4_i32 {
        n = 0_i32;
        while n < 6_i32 - m {
            println!("A({}, {}) = {}", m, n, ackermann(m, n));
            n += 1_i32;
            n;
        }
        m += 1_i32;
        m;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
