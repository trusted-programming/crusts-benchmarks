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
pub type intFn = Option<unsafe extern "C" fn(i32, i32) -> i32>;
#[no_mangle]
pub extern "C" fn reduce(mut fn_0: intFn, mut size: i32, mut elms: *mut i32) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut val: i32 = *elms;
        i = 1;
        while i < size {
            val = fn_0.expect("non-null function pointer")(val, *elms.offset(i as isize));
            i += 1;
            i;
        }
        return val;
    }
}

#[no_mangle]
pub extern "C" fn add(mut a: i32, mut b: i32) -> i32 {
    return a + b;
}

#[no_mangle]
pub extern "C" fn sub(mut a: i32, mut b: i32) -> i32 {
    return a - b;
}

#[no_mangle]
pub extern "C" fn mul(mut a: i32, mut b: i32) -> i32 {
    return a * b;
}

fn main_0() -> i32 {
    let mut nums: [i32; 5] = [1, 2, 3, 4, 5];
    unsafe {
        printf(
            b"%d\n\0" as *const u8 as *const i8,
            reduce(
                Some(add as unsafe extern "C" fn(i32, i32) -> i32),
                5,
                nums.as_mut_ptr(),
            ),
        );
        printf(
            b"%d\n\0" as *const u8 as *const i8,
            reduce(
                Some(sub as unsafe extern "C" fn(i32, i32) -> i32),
                5,
                nums.as_mut_ptr(),
            ),
        );
        printf(
            b"%d\n\0" as *const u8 as *const i8,
            reduce(
                Some(mul as unsafe extern "C" fn(i32, i32) -> i32),
                5,
                nums.as_mut_ptr(),
            ),
        );
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
