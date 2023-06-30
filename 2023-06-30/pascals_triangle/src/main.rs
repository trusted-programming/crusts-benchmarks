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
pub static mut N: i32 = 15_i32;
fn main_0() -> i32 {
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut num: u64 = 0;
    let mut den: u64 = 0;
    let mut catalan: i32 = 0;
    print!("1 ");
    n = 2_i32;
    unsafe {
        while n <= N {
            den = 1;
            num = den;
            k = 2_i32;
            while k <= n {
                num = num.wrapping_mul((n + k) as u64);
                den = den.wrapping_mul(k as u64);
                catalan = num.wrapping_div(den) as i32;
                k = k.wrapping_add(1);
                k;
            }
            print!("{} ", catalan);
            n = n.wrapping_add(1);
            n;
        }
    }
    println!();
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
