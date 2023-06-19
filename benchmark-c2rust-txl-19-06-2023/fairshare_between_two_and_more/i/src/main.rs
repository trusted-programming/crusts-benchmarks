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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub extern "C" fn turn(mut base: i32, mut n: i32) -> i32 {
    let mut sum: i32 = 0;
    while n != 0 {
        let mut rem: i32 = n % base;
        n = n / base;
        sum += rem;
    }
    return sum % base;
}

#[no_mangle]
pub extern "C" fn fairshare(mut base: i32, mut count: i32) {
    let mut i: i32 = 0;
    unsafe {
        printf(b"Base %2d:\0" as *const u8 as *const i8, base);
    }
    i = 0;
    unsafe {
        while i < count {
            let mut t: i32 = turn(base, i);
            printf(b" %2d\0" as *const u8 as *const i8, t);
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const i8);
    }
}

#[no_mangle]
pub extern "C" fn turnCount(mut base: i32, mut count: i32) {
    unsafe {
        let mut cnt: *mut i32 =
            calloc(base as u64, ::core::mem::size_of::<i32>() as u64) as *mut i32;
        let mut i: i32 = 0;
        let mut minTurn: i32 = 0;
        let mut maxTurn: i32 = 0;
        let mut portion: i32 = 0;
        if cnt.is_null() {
            printf(
                b"Failed to allocate space to determine the spread of turns.\n\0" as *const u8
                    as *const i8,
            );
            return;
        }
        i = 0;
        while i < count {
            let mut t: i32 = turn(base, i);
            let ref mut fresh0 = *cnt.offset(t as isize);
            *fresh0 += 1;
            *fresh0;
            i += 1;
            i;
        }
        minTurn = 2147483647;
        maxTurn = -2147483647 - 1;
        portion = 0;
        i = 0;
        while i < base {
            if *cnt.offset(i as isize) > 0 {
                portion += 1;
                portion;
            }
            if *cnt.offset(i as isize) < minTurn {
                minTurn = *cnt.offset(i as isize);
            }
            if *cnt.offset(i as isize) > maxTurn {
                maxTurn = *cnt.offset(i as isize);
            }
            i += 1;
            i;
        }
        printf(b"  With %d people: \0" as *const u8 as *const i8, base);
        if 0 == minTurn {
            printf(
                b"Only %d have a turn\n\0" as *const u8 as *const i8,
                portion,
            );
        } else if minTurn == maxTurn {
            printf(b"%d\n\0" as *const u8 as *const i8, minTurn);
        } else {
            printf(b"%d or %d\n\0" as *const u8 as *const i8, minTurn, maxTurn);
        }
        free(cnt as *mut libc::c_void);
    }
}

fn main_0() -> i32 {
    fairshare(2, 25);
    fairshare(3, 25);
    fairshare(5, 25);
    fairshare(11, 25);
    unsafe {
        printf(
            b"How many times does each get a turn in 50000 iterations?\n\0" as *const u8
                as *const i8,
        );
    }
    turnCount(191, 50000);
    turnCount(1377, 50000);
    turnCount(49999, 50000);
    turnCount(50000, 50000);
    turnCount(50001, 50000);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
