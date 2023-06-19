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
    fn atoi(__nptr: *const i8) -> i32;
}
#[no_mangle]
pub extern "C" fn f(mut n: i32, mut x: i32, mut y: i32) -> i32 {
    return (x + y * 2 + 1) % n;
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut n: i32 = 0;
        if argc != 2 {
            return 1;
        }
        n = atoi(*argv.offset(1 as isize));
        if n < 3 || n % 2 == 0 {
            return 2;
        }
        i = 0;
        while i < n {
            j = 0;
            while j < n {
                printf(
                    b"% 4d\0" as *const u8 as *const i8,
                    f(n, n - j - 1, i) * n + f(n, j, i) + 1,
                );
                j += 1;
                j;
            }
            print!("{}", '\n' as i32);
            i += 1;
            i;
        }
        printf(
            b"\n Magic Constant: %d.\n\0" as *const u8 as *const i8,
            (n * n + 1) / 2 * n,
        );
        return 0;
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
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32);
}
