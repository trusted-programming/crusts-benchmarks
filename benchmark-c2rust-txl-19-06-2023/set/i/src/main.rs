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
}
#[no_mangle]
pub extern "C" fn show_set(mut x: u32, mut name: *const i8) {
    unsafe {
        let mut i: i32 = 0;
        printf(b"%s is:\0" as *const u8 as *const i8, name);
        i = 0;
        while 1 << i <= x {
            if x & 1 << i != 0 {
                printf(b" %d\0" as *const u8 as *const i8, i);
            }
            i += 1;
            i;
        }
        print!("{}", '\n' as i32);
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut c: u32 = 0;
    a = 0;
    i = 0;
    while i < 10 {
        a |= 1 << i;
        i += 3;
    }
    show_set(a, b"a\0" as *const u8 as *const i8);
    i = 0;
    unsafe {
        while i < 5 {
            if a & 1 << i != 0 {
                printf(
                    b"\t%d%s in set a\n\0" as *const u8 as *const i8,
                    i,
                    b"\0" as *const u8 as *const i8,
                )
            } else {
                printf(
                    b"\t%d%s in set a\n\0" as *const u8 as *const i8,
                    i,
                    b" not\0" as *const u8 as *const i8,
                )
            };
            i += 1;
            i;
        }
    }
    b = a;
    b |= 1 << 5;
    b |= 1 << 10;
    b &= !(1 << 0);
    show_set(b, b"b\0" as *const u8 as *const i8);
    show_set(a | b, b"union(a, b)\0" as *const u8 as *const i8);
    c = a & b;
    show_set(c, b"c = common(a, b)\0" as *const u8 as *const i8);
    show_set(a & !b, b"a - b\0" as *const u8 as *const i8);
    show_set(b & !a, b"b - a\0" as *const u8 as *const i8);
    unsafe {
        if b & !a == 0 {
            printf(
                b"b is%s a subset of a\n\0" as *const u8 as *const i8,
                b"\0" as *const u8 as *const i8,
            )
        } else {
            printf(
                b"b is%s a subset of a\n\0" as *const u8 as *const i8,
                b" not\0" as *const u8 as *const i8,
            )
        };
        if c & !a == 0 {
            printf(
                b"c is%s a subset of a\n\0" as *const u8 as *const i8,
                b"\0" as *const u8 as *const i8,
            )
        } else {
            printf(
                b"c is%s a subset of a\n\0" as *const u8 as *const i8,
                b" not\0" as *const u8 as *const i8,
            )
        };
        if (a | b) & !(a & b) == a & !b | b & !a {
            printf(
                b"union(a, b) - common(a, b) %s union(a - b, b - a)\n\0" as *const u8 as *const i8,
                b"equals\0" as *const u8 as *const i8,
            )
        } else {
            printf(
                b"union(a, b) - common(a, b) %s union(a - b, b - a)\n\0" as *const u8 as *const i8,
                b"does not equal\0" as *const u8 as *const i8,
            )
        };
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
