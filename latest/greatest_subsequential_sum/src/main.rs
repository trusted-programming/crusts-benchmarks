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
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct Range {
    pub start: i32,
    pub end: i32,
    pub sum: i32,
}
#[no_mangle]
pub extern "C" fn maxSubseq(mut sequence: *const i32, len: i32) -> Range {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut maxSum: i32 = 0;
        let mut thisSum: i32 = 0;
        let mut i: i32 = 0;
        let mut start: i32 = 0;
        let mut end: i32 = -1;
        let mut j: i32 = 0;
        j = 0_i32;
        while j < len {
            thisSum += *sequence.offset(j as isize);
            if thisSum < 0_i32 {
                i = j.wrapping_add(1);
                thisSum = 0_i32;
            } else if thisSum > maxSum {
                maxSum = thisSum;
                start = i;
                end = j;
            }
            j = j.wrapping_add(1);
            j;
        }
        let mut r: Range = Range {
            start: 0,
            end: 0,
            sum: 0,
        };
        if start <= end && start >= 0_i32 && end >= 0_i32 {
            r.start = start;
            r.end = end + 1_i32;
            r.sum = maxSum;
        } else {
            r.start = 0_i32;
            r.end = 0_i32;
            r.sum = 0_i32;
        }
        r
    }
}

fn main_0(mut _argc: i32, mut _argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut a: [i32; 11] = [-1, -2, 3, 5, 6, -2, -1, 4, -4, 2, -1];
        let mut alength: i32 = (::core::mem::size_of::<[i32; 11]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64)
            as i32;
        let mut r: Range = maxSubseq(a.as_mut_ptr() as *const i32, alength);
        println!("Max sum = {}", r.sum);
        let mut i: i32 = 0;
        i = r.start;
        while i < r.end {
            print!("{} ", a[i as usize]);
            i = i.wrapping_add(1);
            i;
        }
        println!();
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
