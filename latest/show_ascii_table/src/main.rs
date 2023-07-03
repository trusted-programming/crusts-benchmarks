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
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
}
fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: [i8; 4] = [0; 4];
    i = 0_i32;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 16_i32 {
            j = 32_i32 + i;
            while j < 128_i32 {
                match j {
                    32_i32 => {
                        sprintf(k.as_mut_ptr(), (b"Spc\0" as *const u8).cast::<i8>());
                    }
                    127_i32 => {
                        sprintf(k.as_mut_ptr(), (b"Del\0" as *const u8).cast::<i8>());
                    }
                    _ => {
                        sprintf(k.as_mut_ptr(), (b"%c\0" as *const u8).cast::<i8>(), j);
                    }
                }
                print!(
                    "{:3} : {:-3}   ",
                    j,
                    build_str_from_raw_ptr(k.as_mut_ptr().cast::<u8>())
                );
                j = j.wrapping_add(16);
            }
            println!();
            i = i.wrapping_add(1);
            i;
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
