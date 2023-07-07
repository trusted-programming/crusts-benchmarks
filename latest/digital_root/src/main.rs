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
pub extern "C" fn droot(mut x: i64, mut base: i32, mut pers: *mut i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut d: i32 = 0;
        if !pers.is_null() {
            *pers = 0_i32;
            while x >= i64::from(base) {
                d = 0_i32;
                while x != 0 {
                    d = (i64::from(d) + x % i64::from(base)) as i32;
                    x /= i64::from(base);
                }
                x = i64::from(d);
                *pers += 1_i32;
                *pers;
            }
        } else if x != 0 && {
            d = (x % i64::from(base.wrapping_sub(1i32))) as i32;
            d == 0_i32
        } {
            d = base.wrapping_sub(1);
        }
        d
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut d: i32 = 0;
    let mut pers: i32 = 0;
    let mut x: [i64; 4] = [627615, 39390, 588225, 393900588225];
    i = 0_i32;
    while i < 4_i32 {
        d = droot(x[i as usize], 10, &mut pers);
        println!("{}: pers {}, root {}", x[i as usize], pers, d);
        i = i.wrapping_add(1);
        i;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
