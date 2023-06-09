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
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn reverse_section(mut s: *mut i8, mut length: u64) -> *mut i8 {
// SAFETY: machine generated unsafe code
    unsafe {
        if length == 0 {
            return s;
        }
        let mut i: u64 = 0;
        let mut temp: i8 = 0;
        i = 0;
        while i < length.wrapping_div(2).wrapping_add(1) {
            temp = *s.offset(i as isize);
            *s.offset(i as isize) = *s.offset(length.wrapping_sub(i) as isize);
            *s.offset(length.wrapping_sub(i) as isize) = temp;
            i = i.wrapping_add(1);
            i;
        }
        s
    }
}

#[no_mangle]
pub extern "C" fn reverse_words_in_order(mut s: *mut i8, mut delim: i8) -> *mut i8 {
// SAFETY: machine generated unsafe code
    unsafe {
        if strlen(s) == 0 {
            return s;
        }
        let mut i: u64 = 0;
        let mut j: u64 = 0;
        i = 0;
        while i < (strlen(s)).wrapping_sub(1) {
            j = 0;
            while i32::from(*s.offset(i.wrapping_add(j) as isize)) != 0_i32
                && i32::from(*s.offset(i.wrapping_add(j) as isize)) != i32::from(delim)
            {
                j = j.wrapping_add(1);
                j;
            }
            reverse_section(s.offset(i as isize), j.wrapping_sub(1));
            s = s.offset(j as isize);
            i = i.wrapping_add(1);
            i;
        }
        s
    }
}

#[no_mangle]
pub extern "C" fn reverse_string(mut s: *mut i8) -> *mut i8 {
// SAFETY: machine generated unsafe code
    unsafe {
        if strlen(s) != 0 {
            reverse_section(s, (strlen(s)).wrapping_sub(1))
        } else {
            s
        }
    }
}

#[no_mangle]
pub extern "C" fn reverse_order_of_words(mut s: *mut i8, mut delim: i8) -> *mut i8 {
// SAFETY: machine generated unsafe code
    unsafe {
        reverse_string(s);
        reverse_words_in_order(s, delim);
        s
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut str: [i8; 29] =
            *::core::mem::transmute::<&[u8; 29], &mut [i8; 29]>(b"rosetta code phrase reversal\0");
        let mut lenstr: u64 = (::core::mem::size_of::<[i8; 29]>() as u64)
            .wrapping_div(::core::mem::size_of::<i8>() as u64);
        let vla = lenstr as usize;
        let mut scopy: Vec<i8> = ::std::vec::from_elem(0, vla);
        let mut delim: i8 = ' ' as i8;
        println!(
            "Original:       \"{}\"",
            build_str_from_raw_ptr(str.as_mut_ptr().cast::<u8>())
        );
        strncpy(scopy.as_mut_ptr(), str.as_mut_ptr(), lenstr);
        reverse_string(scopy.as_mut_ptr());
        println!(
            "Reversed:       \"{}\"",
            build_str_from_raw_ptr(scopy.as_mut_ptr().cast::<u8>())
        );
        strncpy(scopy.as_mut_ptr(), str.as_mut_ptr(), lenstr);
        reverse_words_in_order(scopy.as_mut_ptr(), delim);
        println!(
            "Reversed words: \"{}\"",
            build_str_from_raw_ptr(scopy.as_mut_ptr().cast::<u8>())
        );
        strncpy(scopy.as_mut_ptr(), str.as_mut_ptr(), lenstr);
        reverse_order_of_words(scopy.as_mut_ptr(), delim);
        println!(
            "Reversed order: \"{}\"",
            build_str_from_raw_ptr(scopy.as_mut_ptr().cast::<u8>())
        );
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
