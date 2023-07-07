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
// SAFETY: machine generated unsafe code
pub type intFn = Option<unsafe extern "C" fn(i32, i32) -> i32>;
#[no_mangle]
pub extern "C" fn reduce(mut fn_0: intFn, mut size: i32, mut elms: *mut i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut val: i32 = *elms;
        i = 1_i32;
        while i < size {
            val = fn_0.expect("non-null function pointer")(val, *elms.offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        val
    }
}

#[no_mangle]
pub extern "C" fn add(mut a: i32, mut b: i32) -> i32 {
    a.wrapping_add(b)
}

#[no_mangle]
pub extern "C" fn sub(mut a: i32, mut b: i32) -> i32 {
    a - b
}

#[no_mangle]
pub extern "C" fn mul(mut a: i32, mut b: i32) -> i32 {
    a.wrapping_mul(b)
}

fn main_0() -> i32 {
    let mut nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!(
        "{}",
        reduce(
// SAFETY: machine generated unsafe code
            Some(add as unsafe extern "C" fn(i32, i32) -> i32),
            5,
            nums.as_mut_ptr(),
        )
    );
    println!(
        "{}",
        reduce(
// SAFETY: machine generated unsafe code
            Some(sub as unsafe extern "C" fn(i32, i32) -> i32),
            5,
            nums.as_mut_ptr(),
        )
    );
    println!(
        "{}",
        reduce(
// SAFETY: machine generated unsafe code
            Some(mul as unsafe extern "C" fn(i32, i32) -> i32),
            5,
            nums.as_mut_ptr(),
        )
    );
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
