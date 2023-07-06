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
pub static mut a: i32 = 0_i32;
#[no_mangle]
pub static mut b: i32 = 0_i32;
#[no_mangle]
pub static mut c: i32 = 0_i32;
#[no_mangle]
pub static mut d: i32 = 0_i32;
#[no_mangle]
pub static mut e: i32 = 0_i32;
#[no_mangle]
pub static mut f: i32 = 0_i32;
#[no_mangle]
pub static mut g: i32 = 0_i32;
#[no_mangle]
pub static mut lo: i32 = 0_i32;
#[no_mangle]
pub static mut hi: i32 = 0_i32;
#[no_mangle]
pub static mut unique: i32 = 0_i32;
#[no_mangle]
pub static mut show: i32 = 0_i32;
#[no_mangle]
pub static mut solutions: i32 = 0_i32;
#[no_mangle]
pub extern "C" fn bf() {
// SAFETY: machine generated unsafe code
    unsafe {
        f = lo;
        while f <= hi {
            if unique == 0_i32 || f != a && f != c && f != d && f != g && f != e {
                b = e + f - c;
                if b >= lo
                    && b <= hi
                    && (unique == 0_i32 || b != a && b != c && b != d && b != g && b != e && b != f)
                {
                    solutions += 1_i32;
                    solutions;
                    if show != 0_i32 {
                        println!("{} {} {} {} {} {} {}", a, b, c, d, e, f, g);
                    }
                }
            }
            f += 1_i32;
            f;
        }
    }
}

#[no_mangle]
pub extern "C" fn ge() {
// SAFETY: machine generated unsafe code
    unsafe {
        e = lo;
        while e <= hi {
            if unique == 0_i32 || e != a && e != c && e != d {
                g = d + e;
                if g >= lo && g <= hi && (unique == 0_i32 || g != a && g != c && g != d && g != e) {
                    bf();
                }
            }
            e += 1_i32;
            e;
        }
    }
}

#[no_mangle]
pub extern "C" fn acd() {
// SAFETY: machine generated unsafe code
    unsafe {
        c = lo;
        while c <= hi {
            d = lo;
            while d <= hi {
                if unique == 0_i32 || c != d {
                    a = c + d;
                    if a >= lo && a <= hi && (unique == 0_i32 || c != 0_i32 && d != 0_i32) {
                        ge();
                    }
                }
                d += 1_i32;
                d;
            }
            c += 1_i32;
            c;
        }
    }
}

#[no_mangle]
pub extern "C" fn foursquares(mut plo: i32, mut phi: i32, mut punique: i32, mut pshow: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        lo = plo;
        hi = phi;
        unique = punique;
        show = pshow;
        solutions = 0_i32;
    }
    println!();
    acd();
// SAFETY: machine generated unsafe code
    unsafe {
        if unique != 0_i32 {
            print!("\n{} unique solutions in {} to {}\n", solutions, lo, hi);
        } else {
            print!("\n{} non-unique solutions in {} to {}\n", solutions, lo, hi);
        };
    }
}

fn main_0() -> i32 {
    foursquares(1, 7, 1, 1);
    foursquares(3, 9, 1, 1);
    foursquares(0, 9, 0, 0);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
