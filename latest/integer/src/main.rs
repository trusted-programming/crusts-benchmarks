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
    fn exit(_: i32) -> !;
}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        if argc < 3_i32 {
            exit(1);
        }
        argc = argc.wrapping_sub(1);
        b = atoi(*argv.offset(argc as isize));
        if b == 0_i32 {
            exit(2);
        }
        argc = argc.wrapping_sub(1);
        a = atoi(*argv.offset(argc as isize));
        println!("a+b = {}", a + b);
        println!("a-b = {}", a - b);
        println!("a*b = {}", a * b);
        println!("a/b = {}", a / b);
        println!("a%b = {}", a % b);
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
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr()),
        );
    }
}
