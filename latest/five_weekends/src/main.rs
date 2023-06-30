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


extern "C" {
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
    (b"January\0" as *const u8).cast::<i8>(),
    (b"February\0" as *const u8).cast::<i8>(),
    (b"March\0" as *const u8).cast::<i8>(),
    (b"April\0" as *const u8).cast::<i8>(),
    (b"May\0" as *const u8).cast::<i8>(),
    (b"June\0" as *const u8).cast::<i8>(),
    (b"July\0" as *const u8).cast::<i8>(),
    (b"August\0" as *const u8).cast::<i8>(),
    (b"September\0" as *const u8).cast::<i8>(),
    (b"October\0" as *const u8).cast::<i8>(),
    (b"November\0" as *const u8).cast::<i8>(),
    (b"December\0" as *const u8).cast::<i8>(),
];
static mut long_months: [i32; 7] = [0_i32, 2_i32, 4_i32, 6_i32, 7_i32, 9_i32, 11_i32];
fn main_0() -> i32 {
    let mut n: i32 = 0;
    let mut y: i32 = 0;
    let mut i: i32 = 0;
    let mut m: i32 = 0;
    let mut t: tm = {
        
        tm {
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
            tm_zone: std::ptr::null::<i8>(),
        }
    };
    println!("Months with five weekends:");
    y = 1_900_i32;
    unsafe {
        while y <= 2_100_i32 {
            i = 0_i32;
            while i < 7_i32 {
                m = long_months[i as usize];
                t.tm_year = y - 1_900_i32;
                t.tm_mon = m;
                t.tm_mday = 1_i32;
                if mktime(&mut t) == -1_i64 {
                    println!(
                        "Error: {} {}",
                        y,
                        build_str_from_raw_ptr(months[m as usize] as *mut u8)
                    );
                } else if t.tm_wday == 5_i32 {
                    println!(
                        "  {} {}",
                        y,
                        build_str_from_raw_ptr(months[m as usize] as *mut u8)
                    );
                    n = n.wrapping_add(1);
                    n;
                }
                i = i.wrapping_add(1);
                i;
            }
            y = y.wrapping_add(1);
            y;
        }
    }
    println!("{} total", n);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
