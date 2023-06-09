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
pub extern "C" fn is_prime(mut n: u32) -> i32 {
    if n.wrapping_rem(2) == 0 || n.wrapping_rem(3) == 0 {
        return 0_i32;
    }
    let mut p: u32 = 1;
    while p.wrapping_mul(p) < n {
        p = (p).wrapping_add(4);
        if n.wrapping_rem(p) == 0 || {
            p = (p).wrapping_add(2);
            n.wrapping_rem(p) == 0
        } {
            return 0_i32;
        }
    }
    1_i32
}

#[no_mangle]
pub extern "C" fn reverse(mut n: u32) -> u32 {
    let mut r: u32 = 0;
    r = 0;
    while n != 0 {
        r = r.wrapping_mul(10).wrapping_add(n.wrapping_rem(10));
        n = n.wrapping_div(10);
    }
    r
}

#[no_mangle]
pub extern "C" fn is_emirp(mut n: u32) -> i32 {
    let mut r: u32 = reverse(n);
    i32::from(r != n && is_prime(n) != 0_i32 && is_prime(r) != 0_i32)
}

fn main_0(mut argc: i32, mut _argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut x: u32 = 0;
        let mut c: u32 = 0;
        match argc {
            1_i32 => {
                x = 11;
                while c < 20 {
                    if is_emirp(x) != 0_i32 {
                        print!(" {}", x);
                        c = c.wrapping_add(1);
                        c;
                    }
                    x = (x).wrapping_add(2);
                }
            }
            2_i32 => {
                x = 7701;
                while x < 8000 {
                    if is_emirp(x) != 0_i32 {
                        print!(" {}", x);
                    }
                    x = (x).wrapping_add(2);
                }
            }
            _ => {
                x = 11;
                loop {
                    if is_emirp(x) != 0_i32 && {
                        c = c.wrapping_add(1);
                        c == 10000
                    } {
                        print!("{}", x);
                        break;
                    } else {
                        x = (x).wrapping_add(2);
                    }
                }
            }
        }
        print!("{}", '\n' as i32);
        0_i32
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr()));
}
