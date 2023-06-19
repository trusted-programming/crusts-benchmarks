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
fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 12;
    j = 1;
    unsafe {
        while j <= n {
            if j != n {
                printf(b"%3d%c\0" as *const u8 as *const i8, j, ' ' as i32)
            } else {
                printf(b"%3d%c\0" as *const u8 as *const i8, j, '\n' as i32)
            };
            j += 1;
            j;
        }
    }
    j = 0;
    unsafe {
        while j <= n {
            if j != n {
                printf(b"----\0" as *const u8 as *const i8)
            } else {
                printf(b"+\n\0" as *const u8 as *const i8)
            };
            j += 1;
            j;
        }
    }
    i = 1;
    unsafe {
        while i <= n {
            j = 1;
            while j <= n {
                if j < i {
                    printf(b"    \0" as *const u8 as *const i8, i * j)
                } else {
                    printf(b"%3d \0" as *const u8 as *const i8, i * j)
                };
                j += 1;
                j;
            }
            printf(b"| %d\n\0" as *const u8 as *const i8, i);
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
