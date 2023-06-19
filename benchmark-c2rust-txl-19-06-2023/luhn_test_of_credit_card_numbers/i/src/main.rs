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
    fn strlen(_: *const i8) -> u64;
    fn printf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub extern "C" fn luhn(mut cc: *const i8) -> i32 {
    unsafe {
        let m: [i32; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
        let mut i: i32 = 0;
        let mut odd: i32 = 1;
        let mut sum: i32 = 0;
        i = strlen(cc) as i32;
        loop {
            let fresh0 = i;
            i = i - 1;
            if !(fresh0 != 0) {
                break;
            }
            let mut digit: i32 = *cc.offset(i as isize) as i32 - '0' as i32;
            sum += if odd != 0 { digit } else { m[digit as usize] };
            odd = (odd == 0) as i32;
        }
        return (sum % 10 == 0i32) as i32;
    }
}

fn main_0() -> i32 {
    let mut cc: [*const i8; 5] = [
        b"49927398716\0" as *const u8 as *const i8,
        b"49927398717\0" as *const u8 as *const i8,
        b"1234567812345678\0" as *const u8 as *const i8,
        b"1234567812345670\0" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut i: i32 = 0;
    i = 0;
    unsafe {
        while !(cc[i as usize]).is_null() {
            if luhn(cc[i as usize]) != 0 {
                printf(
                    b"%16s\t%s\n\0" as *const u8 as *const i8,
                    cc[i as usize],
                    b"ok\0" as *const u8 as *const i8,
                )
            } else {
                printf(
                    b"%16s\t%s\n\0" as *const u8 as *const i8,
                    cc[i as usize],
                    b"not ok\0" as *const u8 as *const i8,
                )
            };
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
