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
    fn sqrt(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn perfect(mut n: i32) -> i32 {
    unsafe {
        let mut max: i32 = sqrt(f64::from(n)) as i32 + 1;
        let mut tot: i32 = 1;
        let mut i: i32 = 0;
        i = 2_i32;
        while i < max {
            if n % i == 0_i32 {
                tot = tot.wrapping_add(i);
                let mut q: i32 = n / i;
                if q > i {
                    tot = tot.wrapping_add(q);
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        i32::from(tot == n)
    }
}

fn main_0() -> i32 {
    let mut n: i32 = 0;
    n = 2_i32;
    while n < 200_i32 {
        if perfect(n) != 0_i32 {
            println!("{}", n);
        }
        n = n.wrapping_add(1);
        n;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
