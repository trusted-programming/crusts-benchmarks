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
    fn rand() -> i32;
    fn srand(__seed: u32);
    fn printf(_: *const i8, _: ...) -> i32;
}
fn main_0() -> i32 {
    let mut a: [[i32; 10]; 10] = [[0; 10]; 10];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    unsafe {
        srand(rust_time(None) as u32);
    }
    i = 0;
    unsafe {
        while i < 10 {
            j = 0;
            while j < 10 {
                a[i as usize][j as usize] = rand() % 20 + 1;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
    }
    i = 0;
    unsafe {
        's_32: while i < 10 {
            j = 0;
            while j < 10 {
                printf(
                    b" %d\0" as *const u8 as *const i8,
                    a[i as usize][j as usize],
                );
                if a[i as usize][j as usize] == 20 {
                    break 's_32;
                }
                j += 1;
                j;
            }
            printf(b"\n\0" as *const u8 as *const i8);
            i += 1;
            i;
        }
        printf(b"\n\0" as *const u8 as *const i8);
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
