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
    fn __ctype_b_loc() -> *mut *const u16;
    fn tolower(_: i32) -> i32;
}
pub const _ISalnum: u32 = 8;
pub const _ISpunct: u32 = 4;
pub const _IScntrl: u32 = 2;
pub const _ISblank: u32 = 1;
pub const _ISgraph: u32 = 32768;
pub const _ISprint: u32 = 16384;
pub const _ISspace: u32 = 8192;
pub const _ISxdigit: u32 = 4096;
pub const _ISdigit: u32 = 2048;
pub const _ISalpha: u32 = 1024;
pub const _ISlower: u32 = 512;
pub const _ISupper: u32 = 256;
#[no_mangle]
pub extern "C" fn rot(mut c: i32, mut str: *mut i8) {
    unsafe {
        let mut l: i32 = strlen(str) as i32;
        let mut alpha: [*const i8; 2] = [
            (b"abcdefghijklmnopqrstuvwxyz\0" as *const u8).cast::<i8>(),
            (b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\0" as *const u8).cast::<i8>(),
        ];
        let mut i: i32 = 0;
        i = 0_i32;
        while i < l {
            if i32::from(*(*__ctype_b_loc()).offset(i32::from(*str.offset(i as isize)) as isize))
                & _ISalpha as i32 != 0_i32
            {
                if i32::from(*(*__ctype_b_loc()).offset(i32::from(*str.offset(i as isize)) as isize))
                    & _ISupper as i32
                    != 0_i32
                {
                    *str.offset(i as isize) = *(alpha[1_usize]).offset(
                        ((tolower(i32::from(*str.offset(i as isize))) - 'a' as i32 + c) % 26i32)
                            as isize,
                    );
                } else {
                    *str.offset(i as isize) = *(alpha[0_usize]).offset(
                        ((tolower(i32::from(*str.offset(i as isize))) - 'a' as i32 + c) % 26i32)
                            as isize,
                    );
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut str: [i8; 35] = *::core::mem::transmute::<&[u8; 35], &mut [i8; 35]>(
            b"This is a top secret text message!\0",
        );
        println!(
            "Original: {}",
            build_str_from_raw_ptr(str.as_mut_ptr().cast::<u8>())
        );
        rot(13, str.as_mut_ptr());
        println!(
            "Encrypted: {}",
            build_str_from_raw_ptr(str.as_mut_ptr().cast::<u8>())
        );
        rot(13, str.as_mut_ptr());
        println!(
            "Decrypted: {}",
            build_str_from_raw_ptr(str.as_mut_ptr().cast::<u8>())
        );
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
