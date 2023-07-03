#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.add(str_size) != 0 {
            str_size = str_size.wrapping_add(1);
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    fn strlen(_: *const i8) -> u64;
}
fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut len: i32 = 0;
        println!("GMP says size is: {}", len);
        let mut s: *mut i8 = std::ptr::null_mut::<i8>();
        len = strlen(s) as i32;
        println!("size really is {}", len);
        println!(
            "Digits: {:.20}...{}",
            build_str_from_raw_ptr(s.cast::<u8>()),
            build_str_from_raw_ptr(s.offset(len as isize).offset(-20_isize).cast::<u8>())
        );
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
