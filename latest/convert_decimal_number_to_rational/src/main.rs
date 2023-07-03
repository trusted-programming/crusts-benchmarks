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
    fn floor(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn rat_approx(mut f: f64, mut md: i64, mut num: *mut i64, mut denom: *mut i64) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut a: i64 = 0;
        let mut h: [i64; 3] = [0, 1, 0];
        let mut k: [i64; 3] = [1, 0, 0];
        let mut x: i64 = 0;
        let mut d: i64 = 0;
        let mut n: i64 = 1;
        let mut i: i32 = 0;
        let mut neg: i32 = 0;
        if md <= 1 {
            *denom = 1;
            *num = f as i64;
            return;
        }
        if f < f64::from(0_i32) {
            neg = 1_i32;
            f = -f;
        }
        while f != floor(f) {
            n <<= 1_i32;
            f *= 2_f64;
        }
        d = f as i64;
        i = 0_i32;
        while i < 64_i32 {
            a = if n != 0 { d / n } else { 0 };
            if i != 0_i32 && a == 0 {
                break;
            }
            x = d;
            d = n;
            n = x % n;
            x = a;
            if k[1_usize] * a + k[0_usize] >= md {
                x = (md - k[0_usize]) / k[1_usize];
                if !(x * 2 >= a || k[1_usize] >= md) {
                    break;
                }
                i = 65_i32;
            }
            h[2_usize] = x * h[1_usize] + h[0_usize];
            h[0_usize] = h[1_usize];
            h[1_usize] = h[2_usize];
            k[2_usize] = x * k[1_usize] + k[0_usize];
            k[0_usize] = k[1_usize];
            k[1_usize] = k[2_usize];
            i = i.wrapping_add(1);
            i;
        }
        *denom = k[1_usize];
        *num = if neg != 0_i32 {
            -h[1_usize]
        } else {
            h[1_usize]
        };
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut d: i64 = 0;
    let mut n: i64 = 0;
    let mut f: f64 = 0.;
    f = 1.0f64 / 7_f64;
    println!("f = {:16.14}", f);
    i = 1_i32;
    while i <= 20_000_000_i32 {
        print!("denom <= {}: ", i);
        rat_approx(f, i64::from(i), &mut n, &mut d);
        println!("{}/{}", n, d);
        i *= 16_i32;
    }
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        f = atan2(1_f64, 1_f64) * 4_f64;
    }
    print!("\nf = {:16.14}\n", f);
    i = 1_i32;
    while i <= 20_000_000_i32 {
        print!("denom <= {}: ", i);
        rat_approx(f, i64::from(i), &mut n, &mut d);
        println!("{}/{}", n, d);
        i *= 16_i32;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
