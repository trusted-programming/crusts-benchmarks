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
    fn scanf(_: *const i8, _: ...) -> i32;
}
fn main_0(mut _argc: i32, mut _argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut user1: i32 = 0;
        let mut user2: i32 = 0;
        print!("Enter two integers.  Space delimited, please:  ");
        scanf(
            (b"%d %d\0" as *const u8).cast::<i8>(),
            &mut user1 as *mut i32,
            &mut user2 as *mut i32,
        );
        let vla = user1 as usize;
        let vla_0 = user2 as usize;
        let mut array: Vec<i32> = ::std::vec::from_elem(0_i32, vla * vla_0);
        *array
            .as_mut_ptr()
            .offset((user1 / 2i32) as isize * vla_0 as isize)
            .offset((user2 / 2i32) as isize) = user1 + user2;
        println!(
            "array[{}][{}] is {}",
            user1 / 2_i32,
            user2 / 2_i32,
            *array
                .as_mut_ptr()
                .offset((user1 / 2i32) as isize * vla_0 as isize)
                .offset((user2 / 2i32) as isize)
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
