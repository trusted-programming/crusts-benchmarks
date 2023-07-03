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
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub extern "C" fn incr(mut s: *mut i8) -> *mut i8 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut begin: i32 = 0;
        let mut tail: i32 = 0;
        let mut len: i32 = 0;
        let mut neg: i32 = i32::from(i32::from(*s) == '-' as i32);
        let mut tgt: i8 = (if neg != 0 { '0' as i32 } else { '9' as i32 }) as i8;
        if strcmp(s, (b"-1\0" as *const u8).cast::<i8>()) == 0_i32 {
            *s.offset(0_isize) = '0' as i8;
            *s.offset(1_isize) = '\0' as i8;
            return s;
        }
        len = strlen(s) as i32;
        begin = if i32::from(*s) == '-' as i32 || i32::from(*s) == '+' as i32 {
            1_i32
        } else {
            0_i32
        };
        tail = len.wrapping_sub(1);
        while tail >= begin && i32::from(*s.offset(tail as isize)) == i32::from(tgt) {
            tail = tail.wrapping_sub(1);
            tail;
        }
        if tail < begin && neg == 0_i32 {
            if begin == 0_i32 {
                s = realloc(s.cast::<libc::c_void>(), (len + 2i32) as u64).cast::<i8>();
            };
            *s.offset(0_isize) = '1' as i8;
            i = 1_i32;
            while i <= len - begin {
                *s.offset(i as isize) = '0' as i8;
                i = i.wrapping_add(1);
                i;
            }
            *s.offset((len + 1i32) as isize) = '\0' as i8;
        } else if tail == begin && neg != 0_i32 && i32::from(*s.offset(1_isize)) == '1' as i32 {
            i = 1_i32;
            while i < len - begin {
                *s.offset(i as isize) = '9' as i8;
                i = i.wrapping_add(1);
                i;
            }
            *s.offset((len - 1i32) as isize) = '\0' as i8;
        } else {
            i = len.wrapping_sub(1);
            while i > tail {
                *s.offset(i as isize) = (if neg != 0_i32 { '9' as i32 } else { '0' as i32 }) as i8;
                i = i.wrapping_sub(1);
                i;
            }
            let fresh0 = &mut (*s.offset(tail as isize));
            *fresh0 = (i32::from(*fresh0) + if neg != 0_i32 { -1i32 } else { 1_i32 }) as i8;
        }
        s
    }
}

#[no_mangle]
pub extern "C" fn string_test(mut s: *const i8) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut ret: *mut i8 = malloc(strlen(s)).cast::<i8>();
        strcpy(ret, s);
        println!("text: {}", build_str_from_raw_ptr(ret.cast::<u8>()));
        ret = incr(ret);
        println!("  ->: {}", build_str_from_raw_ptr(ret.cast::<u8>()));
        free(ret.cast::<libc::c_void>());
    }
}

fn main_0() -> i32 {
    string_test((b"+0\0" as *const u8).cast::<i8>());
    string_test((b"-1\0" as *const u8).cast::<i8>());
    string_test((b"-41\0" as *const u8).cast::<i8>());
    string_test((b"+41\0" as *const u8).cast::<i8>());
    string_test((b"999\0" as *const u8).cast::<i8>());
    string_test((b"+999\0" as *const u8).cast::<i8>());
    string_test((b"109999999999999999999999999999999999999999\0" as *const u8).cast::<i8>());
    string_test((b"-100000000000000000000000000000000000000000000\0" as *const u8).cast::<i8>());
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
