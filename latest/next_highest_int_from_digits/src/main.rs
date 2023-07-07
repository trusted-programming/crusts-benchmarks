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
// SAFETY: machine generated unsafe code
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.add(str_size) != 0 {
            str_size = str_size.wrapping_add(1);
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn strtoul(_: *const i8, _: *mut *mut i8, _: i32) -> u64;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn swap(mut str: *mut i8, mut i: i32, mut j: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut c: i8 = *str.offset(i as isize);
        *str.offset(i as isize) = *str.offset(j as isize);
        *str.offset(j as isize) = c;
    }
}

#[no_mangle]
pub extern "C" fn reverse(mut str: *mut i8, mut i: i32, mut j: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        while i < j {
            swap(str, i, j);
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_sub(1);
            j;
        }
    }
}

#[no_mangle]
pub extern "C" fn next_permutation(mut str: *mut i8) -> bool {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut len: i32 = strlen(str) as i32;
        if len < 2_i32 {
            return 0_i32 != 0_i32;
        }
        let mut i: i32 = len.wrapping_sub(1);
        while i > 0_i32 {
            let mut j: i32 = i;
            let mut k: i32 = 0;
            i = i.wrapping_sub(1);
            if i32::from(*str.offset(i as isize)) < i32::from(*str.offset(j as isize)) {
                k = len;
                loop {
                    k = k.wrapping_sub(1);
                    if i32::from(*str.offset(i as isize)) < i32::from(*str.offset(k as isize)) {
                        break;
                    }
                }
                swap(str, i, k);
                reverse(str, j, len.wrapping_sub(1));
                return 1_i32 != 0_i32;
            }
        }
        0_i32 != 0_i32
    }
}

#[no_mangle]
pub extern "C" fn next_highest_int(mut n: u32) -> u32 {
    let mut str: [i8; 16] = [0; 16];
// SAFETY: machine generated unsafe code
    unsafe {
        snprintf(
            str.as_mut_ptr(),
            ::core::mem::size_of::<[i8; 16]>() as u64,
            (b"%u\0" as *const u8).cast::<i8>(),
            n,
        );
    }
    if !next_permutation(str.as_mut_ptr()) {
        return 0;
    }
// SAFETY: machine generated unsafe code
    unsafe {
        strtoul(str.as_mut_ptr(), std::ptr::null_mut::<*mut i8>(), 10) as u32
    }
}

fn main_0() -> i32 {
    let mut numbers: [u32; 8] = [0, 9, 12, 21, 12453, 738440, 45072010, 95322020];
    let count: i32 = (::core::mem::size_of::<[u32; 8]>() as u64)
        .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32;
    let mut i: i32 = 0;
    while i < count {
        println!(
            "{} -> {}",
            numbers[i as usize],
            next_highest_int(numbers[i as usize])
        );
        i = i.wrapping_add(1);
        i;
    }
// SAFETY: machine generated unsafe code
    unsafe {
        let big: [i8; 23] =
            *::core::mem::transmute::<&[u8; 23], &[i8; 23]>(b"9589776899767587796600\0");
        let mut next: [i8; 23] = [0; 23];
        memcpy(
            next.as_mut_ptr().cast::<libc::c_void>(),
            big.as_ptr().cast::<libc::c_void>(),
            ::core::mem::size_of::<[i8; 23]>() as u64,
        );
        next_permutation(next.as_mut_ptr());
        println!(
            "{} -> {}",
            build_str_from_raw_ptr(big.as_ptr() as *mut u8),
            build_str_from_raw_ptr(next.as_mut_ptr().cast::<u8>())
        );
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
