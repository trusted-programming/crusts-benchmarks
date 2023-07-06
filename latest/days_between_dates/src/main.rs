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
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    fn strlen(_: *const i8) -> u64;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}
#[no_mangle]
pub extern "C" fn extractDate(mut str: *mut i8) -> date {
// SAFETY: machine generated unsafe code
    unsafe {
        {
            
            date {
                year: 1000 * (i32::from(*str.offset(0_isize)) - '0' as i32)
                    + 100 * (i32::from(*str.offset(1_isize)) - '0' as i32)
                    + 10 * (i32::from(*str.offset(2_isize)) - '0' as i32)
                    + (i32::from(*str.offset(3_isize)) - '0' as i32),
                month: 10 * (i32::from(*str.offset(5_isize)) - '0' as i32)
                    + (i32::from(*str.offset(6_isize)) - '0' as i32),
                day: 10 * (i32::from(*str.offset(8_isize)) - '0' as i32)
                    + (i32::from(*str.offset(9_isize)) - '0' as i32),
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn isValidDate(mut str: *mut i8) -> bool {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut newDate: date = date {
            year: 0,
            month: 0,
            day: 0,
        };
        if strlen(str) != 10
            && i32::from(*str.offset(4_isize)) != '-' as i32
            && i32::from(*str.offset(7_isize)) != '-' as i32
        {
            return 0_i32 != 0_i32;
        }
        newDate = extractDate(str);
        if newDate.year <= 0_i32
            || newDate.month <= 0_i32
            || newDate.day <= 0_i32
            || newDate.month > 12_i32
            || newDate.month == 2_i32 && newDate.day > 29_i32
            || (newDate.month == 1_i32
                || newDate.month == 3_i32
                || newDate.month == 5_i32
                || newDate.month == 7_i32
                || newDate.month == 8_i32
                || newDate.month == 10_i32
                || newDate.month == 12_i32)
                && newDate.day > 31_i32
            || newDate.day > 30_i32
            || newDate.year % 4_i32 == 0_i32 && newDate.month == 2_i32 && newDate.month > 28_i32
        {
            return 0_i32 != 0_i32;
        }
        1_i32 != 0_i32
    }
}

#[no_mangle]
pub extern "C" fn diffDays(mut date1: date, mut date2: date) -> i32 {
    let mut days1: i32 = 0;
    let mut days2: i32 = 0;
    date1.month = (date1.month + 9_i32) % 12_i32;
    date1.year -= date1.month / 10_i32;
    date2.month = (date2.month + 9_i32) % 12_i32;
    date2.year -= date2.month / 10_i32;
    days1 = 365_i32 * date1.year + date1.year / 4_i32 - date1.year / 100_i32
        + date1.year / 400_i32
        + (date1.month * 306_i32 + 5_i32) / 10_i32
        + (date1.day - 1_i32);
    days2 = 365_i32 * date2.year + date2.year / 4_i32 - date2.year / 100_i32
        + date2.year / 400_i32
        + (date2.month * 306_i32 + 5_i32) / 10_i32
        + (date2.day - 1_i32);
    days2 - days1
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if argc != 3_i32 {
            let str_to_print = format!(
                "Usage : {} <yyyy-mm-dd> <yyyy-mm-dd>",
                build_str_from_raw_ptr((*argv.offset(0_isize)).cast::<u8>())
            );
            print!("{}", str_to_print);
            return str_to_print.chars().count() as i32;
        }
        if i32::from(isValidDate(*argv.offset(1_isize))) != 0_i32
            && i32::from(isValidDate(*argv.offset(2_isize))) == 0_i32
        {
            let str_to_print = "Dates are invalid.\n".to_string();
            print!("{}", str_to_print);
            return str_to_print.chars().count() as i32;
        }
        println!(
            "Days Difference : {}",
            diffDays(
                extractDate(*argv.offset(1_isize)),
                extractDate(*argv.offset(2_isize)),
            )
        );
        0_i32
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr()));
}
