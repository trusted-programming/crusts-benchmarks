use libc;
extern "C" fn BrotliParseAsUTF8(mut symbol: *mut i32, mut input: *const u8, mut size: u64) -> u64 {
    unsafe {
        if *input.offset(0 as isize) as i32 & 0x80 == 0 {
            *symbol = *input.offset(0 as isize) as i32;
            if *symbol > 0 {
                return 1;
            }
        }
        if size > 1
            && *input.offset(0 as isize) as i32 & 0xe0 == 0xc0
            && *input.offset(1 as isize) as i32 & 0xc0 == 0x80
        {
            *symbol = (*input.offset(0 as isize) as i32 & 0x1f) << 6
                | *input.offset(1 as isize) as i32 & 0x3f;
            if *symbol > 0x7f {
                return 2;
            }
        }
        if size > 2
            && *input.offset(0 as isize) as i32 & 0xf0 == 0xe0
            && *input.offset(1 as isize) as i32 & 0xc0 == 0x80
            && *input.offset(2 as isize) as i32 & 0xc0 == 0x80
        {
            *symbol = (*input.offset(0 as isize) as i32 & 0xf) << 12
                | (*input.offset(1 as isize) as i32 & 0x3f) << 6
                | *input.offset(2 as isize) as i32 & 0x3f;
            if *symbol > 0x7ff {
                return 3;
            }
        }
        if size > 3
            && *input.offset(0 as isize) as i32 & 0xf8 == 0xf0
            && *input.offset(1 as isize) as i32 & 0xc0 == 0x80
            && *input.offset(2 as isize) as i32 & 0xc0 == 0x80
            && *input.offset(3 as isize) as i32 & 0xc0 == 0x80
        {
            *symbol = (*input.offset(0 as isize) as i32 & 0x7) << 18
                | (*input.offset(1 as isize) as i32 & 0x3f) << 12
                | (*input.offset(2 as isize) as i32 & 0x3f) << 6
                | *input.offset(3 as isize) as i32 & 0x3f;
            if *symbol > 0xffff && *symbol <= 0x10ffff {
                return 4;
            }
        }
        *symbol = 0x110000 | *input.offset(0 as isize) as i32;
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn BrotliIsMostlyUTF8(
    mut data: *const u8,
    pos: u64,
    mask: u64,
    length: u64,
    min_fraction: f64,
) -> i32 {
    unsafe {
        let mut size_utf8 = 0;
        let mut i = 0;
        while i < length {
            let mut symbol: i32 = 0;
            let mut bytes_read = BrotliParseAsUTF8(
                &mut symbol,
                &*data.offset((pos.wrapping_add(i) & mask) as isize),
                length.wrapping_sub(i),
            );
            i = (i as u64).wrapping_add(bytes_read) as u64;
            if symbol < 0x110000 {
                size_utf8 = (size_utf8 as u64).wrapping_add(bytes_read) as u64;
            }
        }
        return if size_utf8 as f64 > min_fraction * length as f64 {
            1
        } else {
            0
        };
    }
}
