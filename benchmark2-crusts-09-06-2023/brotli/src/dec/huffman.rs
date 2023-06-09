use libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode {
    pub bits: u8,
    pub value: u16,
}
#[inline(always)]
extern "C" fn ConstructHuffmanCode(bits: u8, value: u16) -> HuffmanCode {
    let mut h = HuffmanCode { bits: 0, value: 0 };
    h.bits = bits;
    h.value = value;
    return h;
}

static mut kReverseBits: [u8; 256] = [
    0, 0x80, 0x40, 0xc0, 0x20, 0xa0, 0x60, 0xe0, 0x10, 0x90, 0x50, 0xd0, 0x30, 0xb0, 0x70, 0xf0,
    0x8, 0x88, 0x48, 0xc8, 0x28, 0xa8, 0x68, 0xe8, 0x18, 0x98, 0x58, 0xd8, 0x38, 0xb8, 0x78, 0xf8,
    0x4, 0x84, 0x44, 0xc4, 0x24, 0xa4, 0x64, 0xe4, 0x14, 0x94, 0x54, 0xd4, 0x34, 0xb4, 0x74, 0xf4,
    0xc, 0x8c, 0x4c, 0xcc, 0x2c, 0xac, 0x6c, 0xec, 0x1c, 0x9c, 0x5c, 0xdc, 0x3c, 0xbc, 0x7c, 0xfc,
    0x2, 0x82, 0x42, 0xc2, 0x22, 0xa2, 0x62, 0xe2, 0x12, 0x92, 0x52, 0xd2, 0x32, 0xb2, 0x72, 0xf2,
    0xa, 0x8a, 0x4a, 0xca, 0x2a, 0xaa, 0x6a, 0xea, 0x1a, 0x9a, 0x5a, 0xda, 0x3a, 0xba, 0x7a, 0xfa,
    0x6, 0x86, 0x46, 0xc6, 0x26, 0xa6, 0x66, 0xe6, 0x16, 0x96, 0x56, 0xd6, 0x36, 0xb6, 0x76, 0xf6,
    0xe, 0x8e, 0x4e, 0xce, 0x2e, 0xae, 0x6e, 0xee, 0x1e, 0x9e, 0x5e, 0xde, 0x3e, 0xbe, 0x7e, 0xfe,
    0x1, 0x81, 0x41, 0xc1, 0x21, 0xa1, 0x61, 0xe1, 0x11, 0x91, 0x51, 0xd1, 0x31, 0xb1, 0x71, 0xf1,
    0x9, 0x89, 0x49, 0xc9, 0x29, 0xa9, 0x69, 0xe9, 0x19, 0x99, 0x59, 0xd9, 0x39, 0xb9, 0x79, 0xf9,
    0x5, 0x85, 0x45, 0xc5, 0x25, 0xa5, 0x65, 0xe5, 0x15, 0x95, 0x55, 0xd5, 0x35, 0xb5, 0x75, 0xf5,
    0xd, 0x8d, 0x4d, 0xcd, 0x2d, 0xad, 0x6d, 0xed, 0x1d, 0x9d, 0x5d, 0xdd, 0x3d, 0xbd, 0x7d, 0xfd,
    0x3, 0x83, 0x43, 0xc3, 0x23, 0xa3, 0x63, 0xe3, 0x13, 0x93, 0x53, 0xd3, 0x33, 0xb3, 0x73, 0xf3,
    0xb, 0x8b, 0x4b, 0xcb, 0x2b, 0xab, 0x6b, 0xeb, 0x1b, 0x9b, 0x5b, 0xdb, 0x3b, 0xbb, 0x7b, 0xfb,
    0x7, 0x87, 0x47, 0xc7, 0x27, 0xa7, 0x67, 0xe7, 0x17, 0x97, 0x57, 0xd7, 0x37, 0xb7, 0x77, 0xf7,
    0xf, 0x8f, 0x4f, 0xcf, 0x2f, 0xaf, 0x6f, 0xef, 0x1f, 0x9f, 0x5f, 0xdf, 0x3f, 0xbf, 0x7f, 0xff,
];
#[inline(always)]
extern "C" fn BrotliReverseBits(mut num: u64) -> u64 {
    unsafe {
        return kReverseBits[num as usize] as u64;
    }
}

#[inline(always)]
extern "C" fn ReplicateValue(
    mut table: *mut HuffmanCode,
    mut step: i32,
    mut end: i32,
    mut code: HuffmanCode,
) {
    unsafe {
        loop {
            end -= step;
            *table.offset(end as isize) = code;
            if !(end > 0) {
                break;
            }
        }
    }
}

#[inline(always)]
extern "C" fn NextTableBitSize(count: *const u16, mut len: i32, mut root_bits: i32) -> i32 {
    unsafe {
        let mut left = 1 << len - root_bits;
        while len < 15 {
            left -= *count.offset(len as isize) as i32;
            if left <= 0 {
                break;
            }
            len += 1;
            left <<= 1;
        }
        return len - root_bits;
    }
}

#[no_mangle]
pub extern "C" fn BrotliBuildCodeLengthsHuffmanTable(
    mut table: *mut HuffmanCode,
    code_lengths: *const u8,
    mut count: *mut u16,
) {
    unsafe {
        let mut code = HuffmanCode { bits: 0, value: 0 };
        let mut symbol: i32 = 0;
        let mut key: u64 = 0;
        let mut key_step: u64 = 0;
        let mut step: i32 = 0;
        let mut table_size: i32 = 0;
        let mut sorted: [i32; 18] = [0; 18];
        let mut offset: [i32; 6] = [0; 6];
        let mut bits: i32 = 0;
        let mut bits_count: i32 = 0;
        symbol = -1;
        bits = 1;
        if 5 & 1 != 0 {
            symbol += *count.offset(bits as isize) as i32;
            offset[bits as usize] = symbol;
            bits += 1;
        }
        if 5 & 2 != 0 {
            symbol += *count.offset(bits as isize) as i32;
            offset[bits as usize] = symbol;
            bits += 1;
            symbol += *count.offset(bits as isize) as i32;
            offset[bits as usize] = symbol;
            bits += 1;
        }
        if 5 & 4 != 0 {
            symbol += *count.offset(bits as isize) as i32;
            offset[bits as usize] = symbol;
            bits += 1;
            symbol += *count.offset(bits as isize) as i32;
            offset[bits as usize] = symbol;
            bits += 1;
            symbol += *count.offset(bits as isize) as i32;
            offset[bits as usize] = symbol;
            bits += 1;
            symbol += *count.offset(bits as isize) as i32;
            offset[bits as usize] = symbol;
            bits += 1;
        }
        offset[0 as usize] = 17 + 1 - 1;
        symbol = 17 + 1;
        loop {
            if 6 & 1 != 0 {
                symbol -= 1;
                let fresh0 = offset[*code_lengths.offset(symbol as isize) as usize];
                offset[*code_lengths.offset(symbol as isize) as usize] =
                    offset[*code_lengths.offset(symbol as isize) as usize] - 1;
                sorted[fresh0 as usize] = symbol;
            }
            if 6 & 2 != 0 {
                symbol -= 1;
                let fresh1 = offset[*code_lengths.offset(symbol as isize) as usize];
                offset[*code_lengths.offset(symbol as isize) as usize] =
                    offset[*code_lengths.offset(symbol as isize) as usize] - 1;
                sorted[fresh1 as usize] = symbol;
                symbol -= 1;
                let fresh2 = offset[*code_lengths.offset(symbol as isize) as usize];
                offset[*code_lengths.offset(symbol as isize) as usize] =
                    offset[*code_lengths.offset(symbol as isize) as usize] - 1;
                sorted[fresh2 as usize] = symbol;
            }
            if 6 & 4 != 0 {
                symbol -= 1;
                let fresh3 = offset[*code_lengths.offset(symbol as isize) as usize];
                offset[*code_lengths.offset(symbol as isize) as usize] =
                    offset[*code_lengths.offset(symbol as isize) as usize] - 1;
                sorted[fresh3 as usize] = symbol;
                symbol -= 1;
                let fresh4 = offset[*code_lengths.offset(symbol as isize) as usize];
                offset[*code_lengths.offset(symbol as isize) as usize] =
                    offset[*code_lengths.offset(symbol as isize) as usize] - 1;
                sorted[fresh4 as usize] = symbol;
                symbol -= 1;
                let fresh5 = offset[*code_lengths.offset(symbol as isize) as usize];
                offset[*code_lengths.offset(symbol as isize) as usize] =
                    offset[*code_lengths.offset(symbol as isize) as usize] - 1;
                sorted[fresh5 as usize] = symbol;
                symbol -= 1;
                let fresh6 = offset[*code_lengths.offset(symbol as isize) as usize];
                offset[*code_lengths.offset(symbol as isize) as usize] =
                    offset[*code_lengths.offset(symbol as isize) as usize] - 1;
                sorted[fresh6 as usize] = symbol;
            }
            if !(symbol != 0) {
                break;
            }
        }
        table_size = 1 << 5;
        if offset[0 as usize] == 0 {
            code = ConstructHuffmanCode(0, sorted[0 as usize] as u16);
            key = 0;
            while key < table_size as u64 {
                *table.offset(key as isize) = code;
                key = key.wrapping_add(1);
            }
            return;
        }
        key = 0;
        key_step = 1 << 8 - 1 + 0;
        symbol = 0;
        bits = 1;
        step = 2;
        loop {
            bits_count = *count.offset(bits as isize) as i32;
            while bits_count != 0 {
                let fresh7 = symbol;
                symbol = symbol + 1;
                code = ConstructHuffmanCode(bits as u8, sorted[fresh7 as usize] as u16);
                ReplicateValue(
                    &mut *table.offset(
                        (BrotliReverseBits as unsafe extern "C" fn(u64) -> u64)(key) as isize,
                    ),
                    step,
                    table_size,
                    code,
                );
                key = (key).wrapping_add(key_step) as u64;
                bits_count -= 1;
            }
            step <<= 1;
            key_step >>= 1;
            bits += 1;
            if !(bits <= 5) {
                break;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliBuildHuffmanTable(
    mut root_table: *mut HuffmanCode,
    mut root_bits: i32,
    symbol_lists: *const u16,
    mut count: *mut u16,
) -> u32 {
    unsafe {
        let mut code = HuffmanCode { bits: 0, value: 0 };
        let mut table = 0 as *mut HuffmanCode;
        let mut len: i32 = 0;
        let mut symbol: i32 = 0;
        let mut key: u64 = 0;
        let mut key_step: u64 = 0;
        let mut sub_key: u64 = 0;
        let mut sub_key_step: u64 = 0;
        let mut step: i32 = 0;
        let mut table_bits: i32 = 0;
        let mut table_size: i32 = 0;
        let mut total_size: i32 = 0;
        let mut max_length = -1;
        let mut bits: i32 = 0;
        let mut bits_count: i32 = 0;
        while *symbol_lists.offset(max_length as isize) as i32 == 0xffff {
            max_length -= 1;
        }
        max_length += 15 + 1;
        table = root_table;
        table_bits = root_bits;
        table_size = 1 << table_bits;
        total_size = table_size;
        if table_bits > max_length {
            table_bits = max_length;
            table_size = 1 << table_bits;
        }
        key = 0;
        key_step = 1 << 8 - 1 + 0;
        bits = 1;
        step = 2;
        loop {
            symbol = bits - (15 + 1);
            bits_count = *count.offset(bits as isize) as i32;
            while bits_count != 0 {
                symbol = *symbol_lists.offset(symbol as isize) as i32;
                code = ConstructHuffmanCode(bits as u8, symbol as u16);
                ReplicateValue(
                    &mut *table.offset(
                        (BrotliReverseBits as unsafe extern "C" fn(u64) -> u64)(key) as isize,
                    ),
                    step,
                    table_size,
                    code,
                );
                key = (key).wrapping_add(key_step) as u64;
                bits_count -= 1;
            }
            step <<= 1;
            key_step >>= 1;
            bits += 1;
            if !(bits <= table_bits) {
                break;
            }
        }
        while total_size != table_size {
            memcpy(
                &mut *table.offset(table_size as isize) as *mut HuffmanCode as *mut libc::c_void,
                &mut *table.offset(0 as isize) as *mut HuffmanCode as *const libc::c_void,
                (table_size as u64).wrapping_mul(::std::mem::size_of::<HuffmanCode>() as u64),
            );
            table_size <<= 1;
        }
        key_step = 1 << 8 - 1 + 0 >> root_bits - 1;
        sub_key = (1 << 8 - 1 + 0) << 1;
        sub_key_step = 1 << 8 - 1 + 0;
        len = root_bits + 1;
        step = 2;
        while len <= max_length {
            symbol = len - (15 + 1);
            while *count.offset(len as isize) as i32 != 0 {
                if sub_key == (1 << 8 - 1 + 0) << 1 {
                    table = table.offset(table_size as isize);
                    table_bits = NextTableBitSize(count, len, root_bits);
                    table_size = 1 << table_bits;
                    total_size += table_size;
                    sub_key = BrotliReverseBits(key);
                    key = (key).wrapping_add(key_step) as u64;
                    *root_table.offset(sub_key as isize) = ConstructHuffmanCode(
                        (table_bits + root_bits) as u8,
                        (table.offset_from(root_table) as u64).wrapping_sub(sub_key) as u16,
                    );
                    sub_key = 0;
                }
                symbol = *symbol_lists.offset(symbol as isize) as i32;
                code = ConstructHuffmanCode((len - root_bits) as u8, symbol as u16);
                ReplicateValue(
                    &mut *table.offset((BrotliReverseBits as unsafe extern "C" fn(u64) -> u64)(
                        sub_key,
                    ) as isize),
                    step,
                    table_size,
                    code,
                );
                sub_key = (sub_key).wrapping_add(sub_key_step) as u64;
                let ref mut fresh8 = *count.offset(len as isize);
                *fresh8 = (*fresh8).wrapping_sub(1);
            }
            step <<= 1;
            sub_key_step >>= 1;
            len += 1;
        }
        return total_size as u32;
    }
}

#[no_mangle]
pub extern "C" fn BrotliBuildSimpleHuffmanTable(
    mut table: *mut HuffmanCode,
    mut root_bits: i32,
    mut val: *mut u16,
    mut num_symbols: u32,
) -> u32 {
    unsafe {
        let mut table_size = 1;
        let goal_size = 1 << root_bits;
        match num_symbols {
            0 => {
                *table.offset(0 as isize) = ConstructHuffmanCode(0, *val.offset(0 as isize));
            }
            1 => {
                if *val.offset(1 as isize) as i32 > *val.offset(0 as isize) as i32 {
                    *table.offset(0 as isize) = ConstructHuffmanCode(1, *val.offset(0 as isize));
                    *table.offset(1 as isize) = ConstructHuffmanCode(1, *val.offset(1 as isize));
                } else {
                    *table.offset(0 as isize) = ConstructHuffmanCode(1, *val.offset(1 as isize));
                    *table.offset(1 as isize) = ConstructHuffmanCode(1, *val.offset(0 as isize));
                }
                table_size = 2;
            }
            2 => {
                *table.offset(0 as isize) = ConstructHuffmanCode(1, *val.offset(0 as isize));
                *table.offset(2 as isize) = ConstructHuffmanCode(1, *val.offset(0 as isize));
                if *val.offset(2 as isize) as i32 > *val.offset(1 as isize) as i32 {
                    *table.offset(1 as isize) = ConstructHuffmanCode(2, *val.offset(1 as isize));
                    *table.offset(3 as isize) = ConstructHuffmanCode(2, *val.offset(2 as isize));
                } else {
                    *table.offset(1 as isize) = ConstructHuffmanCode(2, *val.offset(2 as isize));
                    *table.offset(3 as isize) = ConstructHuffmanCode(2, *val.offset(1 as isize));
                }
                table_size = 4;
            }
            3 => {
                let mut i: i32 = 0;
                let mut k: i32 = 0;
                i = 0;
                while i < 3 {
                    k = i + 1;
                    while k < 4 {
                        if (*val.offset(k as isize) as i32) < *val.offset(i as isize) as i32 {
                            let mut t = *val.offset(k as isize);
                            *val.offset(k as isize) = *val.offset(i as isize);
                            *val.offset(i as isize) = t;
                        }
                        k += 1;
                    }
                    i += 1;
                }
                *table.offset(0 as isize) = ConstructHuffmanCode(2, *val.offset(0 as isize));
                *table.offset(2 as isize) = ConstructHuffmanCode(2, *val.offset(1 as isize));
                *table.offset(1 as isize) = ConstructHuffmanCode(2, *val.offset(2 as isize));
                *table.offset(3 as isize) = ConstructHuffmanCode(2, *val.offset(3 as isize));
                table_size = 4;
            }
            4 => {
                if (*val.offset(3 as isize) as i32) < *val.offset(2 as isize) as i32 {
                    let mut t_0 = *val.offset(3 as isize);
                    *val.offset(3 as isize) = *val.offset(2 as isize);
                    *val.offset(2 as isize) = t_0;
                };
                *table.offset(0 as isize) = ConstructHuffmanCode(1, *val.offset(0 as isize));
                *table.offset(1 as isize) = ConstructHuffmanCode(2, *val.offset(1 as isize));
                *table.offset(2 as isize) = ConstructHuffmanCode(1, *val.offset(0 as isize));
                *table.offset(3 as isize) = ConstructHuffmanCode(3, *val.offset(2 as isize));
                *table.offset(4 as isize) = ConstructHuffmanCode(1, *val.offset(0 as isize));
                *table.offset(5 as isize) = ConstructHuffmanCode(2, *val.offset(1 as isize));
                *table.offset(6 as isize) = ConstructHuffmanCode(1, *val.offset(0 as isize));
                *table.offset(7 as isize) = ConstructHuffmanCode(3, *val.offset(3 as isize));
                table_size = 8;
            }
            _ => {}
        }
        while table_size != goal_size {
            memcpy(
                &mut *table.offset(table_size as isize) as *mut HuffmanCode as *mut libc::c_void,
                &mut *table.offset(0 as isize) as *mut HuffmanCode as *const libc::c_void,
                (table_size as u64).wrapping_mul(::std::mem::size_of::<HuffmanCode>() as u64),
            );
            table_size <<= 1;
        }
        return goal_size;
    }
}
