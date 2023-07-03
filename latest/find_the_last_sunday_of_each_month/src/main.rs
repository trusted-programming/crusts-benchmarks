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
    fn atoi(__nptr: *const i8) -> i32;
}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut days: [i32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut m: i32 = 0;
        let mut y: i32 = 0;
        let mut w: i32 = 0;
        if argc < 2_i32 || {
            y = atoi(*argv.offset(1_isize));
            y <= 1_752_i32
        } {
            return 1_i32;
        }
        days[1_usize] -= i32::from(y % 4_i32 != 0_i32 || y % 100_i32 == 0_i32 && y % 400_i32 != 0_i32);
        w = y * 365_i32 + 97_i32 * (y - 1_i32) / 400_i32 + 4_i32;
        m = 0_i32;
        while m < 12_i32 {
            w = (w + days[m as usize]) % 7_i32;
            println!("{}-{:02}-{}", y, m + 1_i32, days[m as usize] - w);
            m = m.wrapping_add(1);
            m;
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
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr()));
}
