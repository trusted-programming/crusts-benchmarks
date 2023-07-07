#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {
    fn atoi(__nptr: *const i8) -> i32;
}
#[no_mangle]
pub extern "C" fn f(mut n: i32, mut x: i32, mut y: i32) -> i32 {
    (x + y * 2_i32 + 1_i32) % n
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut n: i32 = 0;
        if argc != 2_i32 {
            return 1_i32;
        }
        n = atoi(*argv.offset(1_isize));
        if n < 3_i32 || n % 2_i32 == 0_i32 {
            return 2_i32;
        }
        i = 0_i32;
        while i < n {
            j = 0_i32;
            while j < n {
                print!("% 4d");
                j = j.wrapping_add(1);
                j;
            }
            print!("{}", '\n' as i32);
            i = i.wrapping_add(1);
            i;
        }
        print!("\n Magic Constant: {}.\n", (n * n + 1_i32) / 2_i32 * n);
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
