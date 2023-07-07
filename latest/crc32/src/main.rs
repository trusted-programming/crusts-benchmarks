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
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn rc_crc32(mut crc: u32, mut buf: *const i8, mut len: u64) -> u32 {
// SAFETY: machine generated unsafe code
    unsafe {
        static mut table: [u32; 256] = [0; 256];
        static mut have_table: i32 = 0_i32;
        let mut rem: u32 = 0;
        let mut octet: u8 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut p: *const i8 = std::ptr::null::<i8>();
        let mut q: *const i8 = std::ptr::null::<i8>();
        if have_table == 0_i32 {
            i = 0_i32;
            while i < 256_i32 {
                rem = i as u32;
                j = 0_i32;
                while j < 8_i32 {
                    if rem & 1 != 0 {
                        rem >>= 1_i32;
                        rem ^= 0xedb88320;
                    } else {
                        rem >>= 1_i32;
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                table[i as usize] = rem;
                i = i.wrapping_add(1);
                i;
            }
            have_table = 1_i32;
        }
        crc = !crc;
        q = buf.offset(len as isize);
        p = buf;
        while p < q {
            octet = *p as u8;
            crc = crc >> 8_i32 ^ table[(crc & 0xff ^ u32::from(octet)) as usize];
            p = p.offset(1);
            p;
        }
        !crc
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut s: *const i8 =
            (b"The quick brown fox jumps over the lazy dog\0" as *const u8).cast::<i8>();
        println!("{:X}", rc_crc32(0, s, strlen(s)));
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
