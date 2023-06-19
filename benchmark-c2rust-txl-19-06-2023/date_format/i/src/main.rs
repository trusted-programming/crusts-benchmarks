#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::time::SystemTime;
pub fn rust_time(ref_result: Option<&mut i64>) -> i64 {
    let result = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    match ref_result {
        Some(r) => *r = result,
        None => {}
    }
    return result as i64;
}

use c2rust_out::*;
extern "C" {
    fn printf(_: *const i8, _: ...) -> i32;
    fn strftime(__s: *mut i8, __maxsize: u64, __format: *const i8, __tp: *const tm) -> u64;
    fn localtime(__timer: *const i64) -> *mut tm;
}
#[derive(Copy, Clone)]
#[repr(C)]
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
    unsafe {
        let mut buf: [i8; 50] = [0; 50];
        let mut seconds: i64 = rust_time(None);
        let mut now: *mut tm = localtime(&mut seconds);
        let mut months: [*const i8; 12] = [
            b"January\0" as *const u8 as *const i8,
            b"February\0" as *const u8 as *const i8,
            b"March\0" as *const u8 as *const i8,
            b"April\0" as *const u8 as *const i8,
            b"May\0" as *const u8 as *const i8,
            b"June\0" as *const u8 as *const i8,
            b"July\0" as *const u8 as *const i8,
            b"August\0" as *const u8 as *const i8,
            b"September\0" as *const u8 as *const i8,
            b"October\0" as *const u8 as *const i8,
            b"November\0" as *const u8 as *const i8,
            b"December\0" as *const u8 as *const i8,
        ];
        let mut days: [*const i8; 7] = [
            b"Sunday\0" as *const u8 as *const i8,
            b"Monday\0" as *const u8 as *const i8,
            b"Tuesday\0" as *const u8 as *const i8,
            b"Wednesday\0" as *const u8 as *const i8,
            b"Thursday\0" as *const u8 as *const i8,
            b"Friday\0" as *const u8 as *const i8,
            b"Saturday\0" as *const u8 as *const i8,
        ];
        printf(
            b"%d-%d-%d\n\0" as *const u8 as *const i8,
            (*now).tm_year + 1900,
            (*now).tm_mon + 1,
            (*now).tm_mday,
        );
        printf(
            b"%s, %s %d, %d\n\0" as *const u8 as *const i8,
            days[(*now).tm_wday as usize],
            months[(*now).tm_mon as usize],
            (*now).tm_mday,
            (*now).tm_year + 1900,
        );
        strftime(
            buf.as_mut_ptr(),
            50,
            b"%A, %B %e, %Y\0" as *const u8 as *const i8,
            now,
        );
        printf(b"%s\n\0" as *const u8 as *const i8, buf.as_mut_ptr());
        return 0;
    }
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
