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
    fn abort() -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct frac {
    pub num: i64,
    pub den: i64,
}
#[no_mangle]
pub extern "C" fn gcd(mut m: i64, mut n: i64) -> i64 {
    let mut t: i64 = 0;
    while n != 0 {
        t = n;
        n = m % n;
        m = t;
    }
    m
}

#[no_mangle]
pub extern "C" fn frac_new(mut num: i64, mut den: i64) -> frac {
    let mut a: frac = frac { num: 0, den: 0 };
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        if den == 0 {
            println!("divide by zero: {}/{}", num, den);
            abort();
        }
    }
    let mut g: i32 = gcd(num, den) as i32;
    if g != 0_i32 {
        num /= i64::from(g);
        den /= i64::from(g);
    } else {
        num = 0;
        den = 1;
    }
    if den < 0 {
        den = -den;
        num = -num;
    }
    a.num = num;
    a.den = den;
    a
}

#[no_mangle]
pub extern "C" fn frac_add(mut a: frac, mut b: frac) -> frac {
    frac_new(a.num * b.den + b.num * a.den, a.den * b.den)
}

#[no_mangle]
pub extern "C" fn frac_sub(mut a: frac, mut b: frac) -> frac {
    frac_new(a.num * b.den - b.num + a.den, a.den * b.den)
}

#[no_mangle]
pub extern "C" fn frac_mul(mut a: frac, mut b: frac) -> frac {
    frac_new(a.num * b.num, a.den * b.den)
}

#[no_mangle]
pub extern "C" fn frac_div(mut a: frac, mut b: frac) -> frac {
    frac_new(a.num * b.den, a.den * b.num)
}

#[no_mangle]
pub extern "C" fn frac_cmp(mut a: frac, mut b: frac) -> i32 {
    let mut l: i32 = (a.num * b.den) as i32;
    let mut r: i32 = (a.den * b.num) as i32;
    if l < r { -1_i32 } else { i32::from(l > r) }
}

#[no_mangle]
pub extern "C" fn frtoi(mut a: frac) -> i32 {
    (a.den / a.num) as i32
}

#[no_mangle]
pub extern "C" fn frtod(mut a: frac) -> f64 {
    a.den as f64 / a.num as f64
}

fn main_0() -> i32 {
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    let mut sum: frac = frac { num: 0, den: 0 };
    let mut kf: frac = frac { num: 0, den: 0 };
    n = 2_i32;
    while n < 1_i32 << 19_i32 {
        sum = frac_new(1, i64::from(n));
        k = 2_i32;
        while k * k < n {
            if n % k == 0_i32 {
                kf = frac_new(1, i64::from(k));
                sum = frac_add(sum, kf);
                kf = frac_new(1, i64::from(n / k));
                sum = frac_add(sum, kf);
            }
            k = k.wrapping_add(1);
            k;
        }
        if frac_cmp(sum, frac_new(1, 1)) == 0_i32 {
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
