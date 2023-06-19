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
    fn mktime(__tp: *mut tm) -> i64;
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
static mut months: [*const i8; 12] = [
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
static mut long_months: [i32; 7] = [0, 2, 4, 6, 7, 9, 11];
fn main_0() -> i32 {
    let mut n: i32 = 0;
    let mut y: i32 = 0;
    let mut i: i32 = 0;
    let mut m: i32 = 0;
    let mut t: tm = {
        let mut init = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const i8,
        };
        init
    };
    unsafe {
        printf(b"Months with five weekends:\n\0" as *const u8 as *const i8);
    }
    y = 1900;
    unsafe {
        while y <= 2100 {
            i = 0;
            while i < 7 {
                m = long_months[i as usize];
                t.tm_year = y - 1900;
                t.tm_mon = m;
                t.tm_mday = 1;
                if mktime(&mut t) == -1 as i64 {
                    printf(
                        b"Error: %d %s\n\0" as *const u8 as *const i8,
                        y,
                        months[m as usize],
                    );
                } else if t.tm_wday == 5 {
                    printf(
                        b"  %d %s\n\0" as *const u8 as *const i8,
                        y,
                        months[m as usize],
                    );
                    n += 1;
                    n;
                }
                i += 1;
                i;
            }
            y += 1;
            y;
        }
        printf(b"%d total\n\0" as *const u8 as *const i8, n);
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
