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
fn main_0() -> i32 {
    let mut a: [i32; 3] = [1, 3, -5];
    let mut b: [i32; 3] = [4, -2, -1];
    println!(
        "{}",
        dot_product(
            a.as_mut_ptr(),
            b.as_mut_ptr(),
            (::core::mem::size_of::<[i32; 3]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64),
        )
    );
    0_i32
}

#[no_mangle]
pub extern "C" fn dot_product(mut a: *mut i32, mut b: *mut i32, mut n: u64) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut sum: i32 = 0;
        let mut i: u64 = 0;
        i = 0;
        while i < n {
            sum += *a.offset(i as isize) * *b.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        sum
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
