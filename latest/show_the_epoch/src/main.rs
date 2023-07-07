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
    fn gmtime(__timer: *const i64) -> *mut tm;
    fn asctime(__tp: *const tm) -> *mut i8;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
fn main_0() -> i32 {
    let mut t: i64 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        print!(
            "{}",
            build_str_from_raw_ptr(asctime(gmtime(&mut t)).cast::<u8>())
        );
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
