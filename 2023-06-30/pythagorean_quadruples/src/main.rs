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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
fn main_0(mut _argc: i32, mut _argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut a2: i32 = 0;
        let mut s: i32 = 3;
        let mut s1: i32 = 0;
        let mut s2: i32 = 0;
        let mut r: [i32; 2201] = [0; 2201];
        memset(
            r.as_mut_ptr().cast::<libc::c_void>(),
            0,
            ::core::mem::size_of::<[i32; 2201]>() as u64,
        );
        let mut ab: *mut i32 = calloc(
            (2200 * 2200 * 2 + 1i32) as u64,
            ::core::mem::size_of::<i32>() as u64,
        ).cast::<i32>();
        a = 1_i32;
        while a <= 2_200_i32 {
            a2 = a * a;
            b = a;
            while b <= 2_200_i32 {
                *ab.offset((a2 + b * b) as isize) = 1_i32;
                b = b.wrapping_add(1);
                b;
            }
            a = a.wrapping_add(1);
            a;
        }
        c = 1_i32;
        while c <= 2_200_i32 {
            s1 = s;
            s = s.wrapping_add(2);
            s2 = s;
            d = c.wrapping_add(1);
            while d <= 2_200_i32 {
                if *ab.offset(s1 as isize) != 0_i32 {
                    r[d as usize] = 1_i32;
                }
                s1 = s1.wrapping_add(s2);
                s2 = s2.wrapping_add(2);
                d = d.wrapping_add(1);
                d;
            }
            c = c.wrapping_add(1);
            c;
        }
        d = 1_i32;
        while d <= 2_200_i32 {
            if r[d as usize] == 0_i32 {
                print!("{} ", d);
            }
            d = d.wrapping_add(1);
            d;
        }
        println!();
        free(ab.cast::<libc::c_void>());
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
