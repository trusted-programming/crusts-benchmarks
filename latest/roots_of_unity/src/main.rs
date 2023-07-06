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
    fn atan2(_: f64, _: f64) -> f64;
    fn cos(_: f64) -> f64;
    fn sin(_: f64) -> f64;
}
fn main_0() -> i32 {
    let mut a: f64 = 0.;
    let mut c: f64 = 0.;
    let mut s: f64 = 0.;
// SAFETY: machine generated unsafe code
    unsafe {
        let mut PI2: f64 = atan2(1_f64, 1_f64) * 8_f64;
        let mut n: i32 = 0;
        let mut i: i32 = 0;
        n = 1_i32;
        while n < 10_i32 {
            i = 0_i32;
            while i < n {
                s = f64::from(0_i32);
                c = s;
                if i == 0_i32 {
                    c = 1_f64;
                } else if n == 4_i32 * i {
                    s = 1_f64;
                } else if n == 2_i32 * i {
                    c = -1_f64;
                } else if 3_i32 * n == 4_i32 * i {
                    s = -1_f64;
                } else {
                    a = f64::from(i) * PI2 / f64::from(n);
                    c = cos(a);
                    s = sin(a);
                }
                if c != 0.0_f64 {
                    print!("{:.2}", c);
                }
                if s == 1_f64 {
                    print!("i")
                } else if s == -1_f64 {
                    print!("-i")
                } else if s != 0.0_f64 {
                    print!("{:+.2}i", s)
                } else {
                    print!("")
                };
                if i == n - 1_i32 {
                    println!()
                } else {
                    print!(",  ")
                };
                i += 1_i32;
                i;
            }
            n += 1_i32;
            n;
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
