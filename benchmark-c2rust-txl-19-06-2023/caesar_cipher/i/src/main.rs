#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn printf(_: *const i8, _: ...) -> i32;
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
            b"abcdefghijklmnopqrstuvwxyz\0" as *const u8 as *const i8,
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\0" as *const u8 as *const i8,
        ];
        let mut i: i32 = 0;
        i = 0;
        while i < l {
            if !(*(*__ctype_b_loc()).offset(*str.offset(i as isize) as i32 as isize) as i32
                & _ISalpha as i32
                == 0)
            {
                if *(*__ctype_b_loc()).offset(*str.offset(i as isize) as i32 as isize) as i32
                    & _ISupper as i32
                    != 0
                {
                    *str.offset(i as isize) = *(alpha[1 as usize]).offset(
                        ((tolower(*str.offset(i as isize) as i32) - 'a' as i32 + c) % 26i32)
                            as isize,
                    );
                } else {
                    *str.offset(i as isize) = *(alpha[0 as usize]).offset(
                        ((tolower(*str.offset(i as isize) as i32) - 'a' as i32 + c) % 26i32)
                            as isize,
                    );
                }
            }
            i += 1;
            i;
        }
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut str: [i8; 35] = *::core::mem::transmute::<&[u8; 35], &mut [i8; 35]>(
            b"This is a top secret text message!\0",
        );
        printf(
            b"Original: %s\n\0" as *const u8 as *const i8,
            str.as_mut_ptr(),
        );
        rot(13, str.as_mut_ptr());
        printf(
            b"Encrypted: %s\n\0" as *const u8 as *const i8,
            str.as_mut_ptr(),
        );
        rot(13, str.as_mut_ptr());
        printf(
            b"Decrypted: %s\n\0" as *const u8 as *const i8,
            str.as_mut_ptr(),
        );
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
