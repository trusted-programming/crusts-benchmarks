#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.add(str_size) != 0 {
            str_size = str_size.wrapping_add(1);
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {}
#[no_mangle]
pub static mut DIGITS: [i8; 63] = unsafe {
    *::core::mem::transmute::<&[u8; 63], &[i8; 63]>(
        b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz\0",
    )
};
#[no_mangle]
pub static mut DIGITS_LEN: i32 = 64_i32;
#[no_mangle]
pub extern "C" fn encodeNegativeBase(mut n: i64, mut base: i64, mut out: *mut i8) {
    unsafe {
        let mut ptr: *mut i8 = out;
        if !(-62_i64..=-1_i64).contains(&base) {
            out = (b"\0" as *const u8).cast::<i8>() as *mut i8;
            return;
        }
        if n == 0 {
            out = (b"0\0" as *const u8).cast::<i8>() as *mut i8;
            return;
        }
        while n != 0 {
            let mut rem: i64 = n % base;
            n /= base;
            if rem < 0 {
                n = n.wrapping_add(1);
                n;
                rem -= base;
            }
            *ptr = DIGITS[rem as usize];
            ptr = ptr.offset(1);
            ptr;
        }
        *ptr = 0;
        ptr = ptr.offset(-1);
        ptr;
        while out < ptr {
            core::ptr::swap(out, ptr);
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
        let mut ptr: *const i8 = std::ptr::null::<i8>();
        if !(-62_i64..=-1_i64).contains(&base) {
            return 0;
        }
        if i32::from(*ns.offset(0_isize)) == 0_i32
            || i32::from(*ns.offset(0_isize)) == '0' as i32 && i32::from(*ns.offset(1_isize)) == 0_i32
        {
            return 0;
        }
        ptr = ns;
        while i32::from(*ptr) != 0_i32 {
            ptr = ptr.offset(1);
            ptr;
        }
        value = 0;
        bb = 1;
        ptr = ptr.offset(-1);
        ptr;
        while ptr >= ns {
            i = 0_i32;
            while i < DIGITS_LEN {
                if i32::from(*ptr) == i32::from(DIGITS[i as usize]) {
                    value += i64::from(i) * bb;
                    bb *= base;
                    break;
                } else {
                    i = i.wrapping_add(1);
                    i;
                }
            }
            ptr = ptr.offset(-1);
            ptr;
        }
        value
    }
}

#[no_mangle]
pub extern "C" fn driver(mut n: i64, mut b: i64) {
    let mut buf: [i8; 64] = [0; 64];
    let mut value: i64 = 0;
    encodeNegativeBase(n, b, buf.as_mut_ptr());
    unsafe {
        println!(
            "{:12} encoded in base {:3} = {:12}",
            n,
            b,
            build_str_from_raw_ptr(buf.as_mut_ptr().cast::<u8>())
        );
    }
    value = decodeNegativeBase(buf.as_mut_ptr(), b);
    unsafe {
        println!(
            "{:12} decoded in base {:3} = {:12}",
            build_str_from_raw_ptr(buf.as_mut_ptr().cast::<u8>()),
            b,
            value
        );
    }
    println!();
}

fn main_0() -> i32 {
    driver(10, -2_i64);
    driver(146, -3_i64);
    driver(15, -10_i64);
    driver(12, -62_i64);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
