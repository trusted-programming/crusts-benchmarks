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
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.add(str_size) != 0 {
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
// SAFETY: machine generated unsafe code
pub static mut BASE64: [u8; 65] = unsafe {
    *::core::mem::transmute::<&[u8; 65], &[u8; 65]>(
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0",
    )
};
#[no_mangle]
pub extern "C" fn findIndex(val: u8) -> i32 {
    if 'A' as i32 <= i32::from(val) && i32::from(val) <= 'Z' as i32 {
        return i32::from(val) - 'A' as i32;
    }
    if 'a' as i32 <= i32::from(val) && i32::from(val) <= 'z' as i32 {
        return i32::from(val) - 'a' as i32 + 26_i32;
    }
    if '0' as i32 <= i32::from(val) && i32::from(val) <= '9' as i32 {
        return i32::from(val) - '0' as i32 + 52_i32;
    }
    if i32::from(val) == '+' as i32 {
        return 62_i32;
    }
    if i32::from(val) == '/' as i32 {
        return 63_i32;
    }
    -1_i32
}

#[no_mangle]
pub extern "C" fn decode(mut source: *const u8, mut sink: *mut u8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let length: u64 = strlen(source.cast::<i8>());
        let mut it: *const u8 = source;
        let mut end: *const u8 = source.offset(length as isize);
        let mut acc: i32 = 0;
        if length.wrapping_rem(4) != 0 {
            return 1_i32;
        }
        while it != end {
            let fresh0 = it;
            it = it.offset(1);
            let b1: u8 = *fresh0;
            let fresh1 = it;
            it = it.offset(1);
            let b2: u8 = *fresh1;
            let fresh2 = it;
            it = it.offset(1);
            let b3: u8 = *fresh2;
            let fresh3 = it;
            it = it.offset(1);
            let b4: u8 = *fresh3;
            let i1: i32 = findIndex(b1);
            let i2: i32 = findIndex(b2);
            acc = i1 << 2_i32;
            acc |= i2 >> 4_i32;
            let fresh4 = sink;
            sink = sink.offset(1);
            *fresh4 = acc as u8;
            if i32::from(b3) != '=' as i32 {
                let i3: i32 = findIndex(b3);
                acc = (i2 & 0x000f_i32) << 4_i32;
                acc += i3 >> 2_i32;
                let fresh5 = sink;
                sink = sink.offset(1);
                *fresh5 = acc as u8;
                if i32::from(b4) != '=' as i32 {
                    let i4: i32 = findIndex(b4);
                    acc = (i3 & 0x0003_i32) << 6_i32;
                    acc |= i4;
                    let fresh6 = sink;
                    sink = sink.offset(1);
                    *fresh6 = acc as u8;
                }
            }
        }
        *sink = b'\0';
        0_i32
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut data : [u8; 117] = * :: core :: mem :: transmute :: < & [u8; 117], & mut [u8; 117], > (b"VG8gZXJyIGlzIGh1bWFuLCBidXQgdG8gcmVhbGx5IGZvdWwgdGhpbmdzIHVwIHlvdSBuZWVkIGEgY29tcHV0ZXIuCiAgICAtLVBhdWwgUi5FaHJsaWNo\0",);
        let mut decoded: [u8; 1024] = [0; 1024];
        print!(
            "{}\n\n",
            build_str_from_raw_ptr(data.as_mut_ptr())
        );
        decode(data.as_mut_ptr() as *const u8, decoded.as_mut_ptr());
        print!(
            "{}\n\n",
            build_str_from_raw_ptr(decoded.as_mut_ptr())
        );
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
