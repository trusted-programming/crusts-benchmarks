#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {}
fn main_0(mut _argc: i32, mut _argv: *mut *const i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        println!("F(0) = 3 -> PRIME");
        let mut i: u32 = 1;
        while i < 7 {
            i = i.wrapping_add(1);
            i;
        }
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
    ::std::process::exit(
        main_0((args.len() - 1) as i32, args.as_mut_ptr().cast::<*const i8>()),
    );
}
