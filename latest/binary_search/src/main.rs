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
pub extern "C" fn bsearch(mut a: *mut i32, mut n: i32, mut x: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = n.wrapping_sub(1);
        while i <= j {
            let mut k: i32 = i + (j - i) / 2;
            if *a.offset(k as isize) == x {
                return k;
            } else if *a.offset(k as isize) < x {
                i = k.wrapping_add(1);
            } else {
                j = k.wrapping_sub(1);
            }
        }
        -1_i32
    }
}

#[no_mangle]
pub extern "C" fn bsearch_r(mut a: *mut i32, mut x: i32, mut i: i32, mut j: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if j < i {
            return -1_i32;
        }
        let mut k: i32 = i + (j - i) / 2;
        if *a.offset(k as isize) == x {
            k
        } else if *a.offset(k as isize) < x {
            return bsearch_r(a, x, k.wrapping_add(1), j);
        } else {
            return bsearch_r(a, x, i, k.wrapping_sub(1));
        }
    }
}

fn main_0() -> i32 {
    let mut a: [i32; 10] = [-31, 0, 1, 2, 2, 4, 65, 83, 99, 782];
    let mut n: i32 = (::core::mem::size_of::<[i32; 10]>() as u64)
        .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32;
    let mut x: i32 = 2;
    let mut i: i32 = bsearch(a.as_mut_ptr(), n, x);
    println!("{} is at index {}", x, i);
    x = 5_i32;
    i = bsearch_r(a.as_mut_ptr(), x, 0, n.wrapping_sub(1));
    println!("{} is at index {}", x, i);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
