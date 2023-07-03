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
    fn pow(_: f64, _: f64) -> f64;
}
fn main_0() -> i32 {
    let N: i32 = 2;
    let mut base: i32 = 10;
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut k: i32 = 0;
    k = 1_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while f64::from(k) < pow(f64::from(base), f64::from(N)) {
            c1 = c1.wrapping_add(1);
            c1;
            if k % (base - 1_i32) == k * k % (base - 1_i32) {
                c2 = c2.wrapping_add(1);
                c2;
                print!("{} ", k);
            }
            k = k.wrapping_add(1);
            k;
        }
    }
    print!(
        "\nTring {} numbers instead of {} numbers saves {}%\n",
        c2,
        c1,
        100.0f64 - 100.0f64 * f64::from(c2) / f64::from(c1)
    );
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
