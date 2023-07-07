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
    fn puts(__s: *const i8) -> i32;
    fn exp(_: f64) -> f64;
}
fn main_0(mut _argc: i32, mut _argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut e: f64 = 0.;
        puts ((b"The double precision in C give about 15 significant digits.\nValues below are presented with 16 digits after the decimal point.\n\0" as * const u8).cast::<i8>(),);
        e = exp(1_f64);
        println!("Euler constant e = {:.16}", e);
        let mut n: i32 = 8192;
        e = 1.0f64 + 1.0f64 / f64::from(n);
        let mut i: i32 = 0;
        while i < 13_i32 {
            e *= e;
            i = i.wrapping_add(1);
            i;
        }
        println!("Euler constant e = {:.16}", e);
        let N: i32 = 1000;
        let mut a: [f64; 1000] = [0.; 1000];
        a[0_usize] = 1.0f64;
        let mut i_0: i32 = 1;
        while i_0 < N {
            a[i_0 as usize] = a[(i_0.wrapping_sub(1i32)) as usize] / f64::from(i_0);
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
        e = 1.0f64;
        let mut i_1: i32 = N.wrapping_sub(1);
        while i_1 > 0_i32 {
            e += a[i_1 as usize];
            i_1 = i_1.wrapping_sub(1);
            i_1;
        }
        println!("Euler constant e = {:.16}", e);
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
