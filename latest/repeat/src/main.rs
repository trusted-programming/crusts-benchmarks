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
#[no_mangle]
// SAFETY: machine generated unsafe code
pub extern "C" fn repeat(mut f: Option<unsafe extern "C" fn() -> ()>, mut n: u32) {
// SAFETY: machine generated unsafe code
    unsafe {
        loop {
            let fresh0 = n;
            n = n.wrapping_sub(1);
            if fresh0 <= 0 {
                break;
            };
            f.expect("non-null function pointer")();
        }
    }
}

#[no_mangle]
pub extern "C" fn example() {
    println!("Example");
}

fn main_0(mut _argc: i32, mut _argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        repeat(
            ::core::mem::transmute::<
// SAFETY: machine generated unsafe code
                Option<unsafe extern "C" fn() -> ()>,
// SAFETY: machine generated unsafe code
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(::core::mem::transmute::<
// SAFETY: machine generated unsafe code
                unsafe extern "C" fn() -> (),
// SAFETY: machine generated unsafe code
                unsafe extern "C" fn() -> (),
            >(example))),
            4,
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
