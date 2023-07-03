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
// SAFETY: machine generated unsafe code
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
    fn puts(__s: *const i8) -> i32;
    fn strtol(_: *const i8, _: *mut *mut i8, _: i32) -> i64;
    fn __ctype_b_loc() -> *mut *const u16;
}
pub const _ISalnum: u32 = 8;
pub const _ISpunct: u32 = 4;
pub const _IScntrl: u32 = 2;
pub const _ISblank: u32 = 1;
pub const _ISgraph: u32 = 32768;
pub const _ISprint: u32 = 16384;
pub const _ISspace: u32 = 8192;
pub const _ISxdigit: u32 = 4096;
pub const _ISdigit: u32 = 2048;
pub const _ISalpha: u32 = 1024;
pub const _ISlower: u32 = 512;
pub const _ISupper: u32 = 256;
#[no_mangle]
pub extern "C" fn get_list(mut s: *const i8, mut e: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut x: i32 = 0;
        loop {
            while i32::from(*(*__ctype_b_loc()).offset(i32::from(*s) as isize)) & _ISspace as i32 != 0_i32 {
                s = s.offset(1);
                s;
            }
            if get_rnge(s, e) == 0_i32 && {
                x = strtol(s, e, 10) as i32;
                *e == s as *mut i8
            } {
                break;
            }
            s = *e;
            while i32::from(*(*__ctype_b_loc()).offset(i32::from(*s) as isize)) & _ISspace as i32 != 0_i32 {
                s = s.offset(1);
                s;
            }
            if i32::from(*s) == '\0' as i32 {
                print!("{}", '\n' as i32);
                return 1_i32;
            }
            if i32::from(*s) != ',' as i32 {
                break;
            }
            s = s.offset(1);
            s;
        }
        let fresh0 = &mut (*e.cast::<*const i8>());
        *fresh0 = s;
        print!(
            "\nSyntax error at {}\n",
            build_str_from_raw_ptr(s as *mut u8)
        );
        0_i32
    }
}

#[no_mangle]
pub extern "C" fn get_rnge(mut s: *const i8, mut e: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut ee: *mut i8 = std::ptr::null_mut::<i8>();
        x = strtol(s, &mut ee, 10) as i32;
        if ee == s as *mut i8 {
            return 0_i32;
        }
        s = ee;
        while i32::from(*(*__ctype_b_loc()).offset(i32::from(*s) as isize)) & _ISspace as i32 != 0_i32 {
            s = s.offset(1);
            s;
        }
        if i32::from(*s) != '-' as i32 {
            let fresh1 = &mut (*e.cast::<*const i8>());
            *fresh1 = s;
            return 0_i32;
        }
        s = s.offset(1);
        s;
        y = strtol(s, e, 10) as i32;
        if *e == s as *mut i8 {
            return 0_i32;
        }
        add_range(x, y)
    }
}

#[no_mangle]
pub extern "C" fn add_number(mut x: i32) {
    print!("{} ", x);
}

#[no_mangle]
pub extern "C" fn add_range(mut x: i32, mut y: i32) -> i32 {
    if y <= x {
        return 0_i32;
    }
    while x <= y {
        let fresh2 = x;
        x = x.wrapping_add(1);
        print!("{} ", fresh2);
    }
    1_i32
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut end: *mut i8 = std::ptr::null_mut::<i8>();
        if get_list(
            (b"-6,-3--1,3-5,7-11,14,15,17-20\0" as *const u8).cast::<i8>(),
            &mut end,
        ) != 0_i32
        {
            puts((b"Ok\0" as *const u8).cast::<i8>());
        }
        get_list(
            (b"-6 -3--1,3-5,7-11,14,15,17-20\0" as *const u8).cast::<i8>(),
            &mut end,
        );
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
