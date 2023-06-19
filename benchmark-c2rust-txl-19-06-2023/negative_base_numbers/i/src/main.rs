#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn printf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub static mut DIGITS: [i8; 63] = unsafe {
    *::core::mem::transmute::<&[u8; 63], &[i8; 63]>(
        b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz\0",
    )
};
#[no_mangle]
pub static mut DIGITS_LEN: i32 = 64;
#[no_mangle]
pub extern "C" fn encodeNegativeBase(mut n: i64, mut base: i64, mut out: *mut i8) {
    unsafe {
        let mut ptr: *mut i8 = out;
        if base > -1 as i64 || base < -62 as i64 {
            out = b"\0" as *const u8 as *const i8 as *mut i8;
            return;
        }
        if n == 0 {
            out = b"0\0" as *const u8 as *const i8 as *mut i8;
            return;
        }
        while n != 0 {
            let mut rem: i64 = n % base;
            n = n / base;
            if rem < 0 {
                n += 1;
                n;
                rem = rem - base;
            }
            *ptr = DIGITS[rem as usize];
            ptr = ptr.offset(1);
            ptr;
        }
        *ptr = 0;
        ptr = ptr.offset(-1);
        ptr;
        while out < ptr {
            let mut t: i8 = *out;
            *out = *ptr;
            *ptr = t;
            out = out.offset(1);
            out;
            ptr = ptr.offset(-1);
            ptr;
        }
    }
}

#[no_mangle]
pub extern "C" fn decodeNegativeBase(mut ns: *const i8, mut base: i64) -> i64 {
    unsafe {
        let mut value: i64 = 0;
        let mut bb: i64 = 0;
        let mut i: i32 = 0;
        let mut ptr: *const i8 = 0 as *const i8;
        if base < -62 as i64 || base > -1 as i64 {
            return 0;
        }
        if *ns.offset(0 as isize) as i32 == 0
            || *ns.offset(0 as isize) as i32 == '0' as i32 && *ns.offset(1 as isize) as i32 == 0
        {
            return 0;
        }
        ptr = ns;
        while *ptr as i32 != 0 {
            ptr = ptr.offset(1);
            ptr;
        }
        value = 0;
        bb = 1;
        ptr = ptr.offset(-1);
        ptr;
        while ptr >= ns {
            i = 0;
            while i < DIGITS_LEN {
                if *ptr as i32 == DIGITS[i as usize] as i32 {
                    value = value + i as i64 * bb;
                    bb = bb * base;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
            ptr = ptr.offset(-1);
            ptr;
        }
        return value;
    }
}

#[no_mangle]
pub extern "C" fn driver(mut n: i64, mut b: i64) {
    let mut buf: [i8; 64] = [0; 64];
    let mut value: i64 = 0;
    encodeNegativeBase(n, b, buf.as_mut_ptr());
    unsafe {
        printf(
            b"%12d encoded in base %3d = %12s\n\0" as *const u8 as *const i8,
            n,
            b,
            buf.as_mut_ptr(),
        );
    }
    value = decodeNegativeBase(buf.as_mut_ptr(), b);
    unsafe {
        printf(
            b"%12s decoded in base %3d = %12d\n\0" as *const u8 as *const i8,
            buf.as_mut_ptr(),
            b,
            value,
        );
        printf(b"\n\0" as *const u8 as *const i8);
    }
}

fn main_0() -> i32 {
    driver(10, -2 as i64);
    driver(146, -3 as i64);
    driver(15, -10 as i64);
    driver(12, -62 as i64);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
