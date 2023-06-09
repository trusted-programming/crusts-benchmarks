use libc;
extern "C" {
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn log2(_: f64) -> f64;
    static kBrotliLog2Table: [f64; 256];
    fn BrotliConvertBitDepthsToSymbols(depth: *const u8, len: u64, bits: *mut u16);
    fn BrotliBuildAndStoreHuffmanTreeFast(
        m: *mut MemoryManager,
        histogram: *const u32,
        histogram_total: u64,
        max_bits: u64,
        depth: *mut u8,
        bits: *mut u16,
        storage_ix: *mut u64,
        storage: *mut u8,
    );
    fn BrotliStoreHuffmanTree(
        depths: *const u8,
        num: u64,
        tree: *mut HuffmanTree,
        storage_ix: *mut u64,
        storage: *mut u8,
    );
    fn BrotliCreateHuffmanTree(
        data: *const u32,
        length: u64,
        tree_limit: i32,
        tree: *mut HuffmanTree,
        depth: *mut u8,
    );
}
pub type brotli_alloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTree {
    pub total_count_: u32,
    pub index_left_: i16,
    pub index_right_or_value_: i16,
}
#[inline(always)]
extern "C" fn BrotliUnalignedRead32(mut p: *const libc::c_void) -> u32 {
    unsafe {
        return *(p as *const u32);
    }
}

#[inline(always)]
extern "C" fn BrotliUnalignedRead64(mut p: *const libc::c_void) -> u64 {
    unsafe {
        return *(p as *const u64);
    }
}

#[inline(always)]
extern "C" fn BrotliUnalignedWrite64(mut p: *mut libc::c_void, mut v: u64) {
    unsafe {
        *(p as *mut u64) = v;
    }
}

#[inline(always)]
extern "C" fn brotli_min_size_t(mut a: u64, mut b: u64) -> u64 {
    return if a < b { a } else { b };
}

static mut kCompressFragmentTwoPassBlockSize: u64 = (1i32 << 17) as u64;
#[inline(always)]
extern "C" fn ShannonEntropy(
    mut population: *const u32,
    mut size: u64,
    mut total: *mut u64,
) -> f64 {
    unsafe {
        let mut current_block: u64;
        let mut sum = 0;
        let mut retval = 0 as f64;
        let mut population_end = population.offset(size as isize);
        let mut p: u64 = 0;
        if size & 1 != 0 {
            current_block = 18167736425092750904;
        } else {
            current_block = 715039052867723359;
        }
        loop {
            match current_block {
                715039052867723359 => {
                    if !(population < population_end) {
                        break;
                    }
                    let fresh0 = population;
                    population = population.offset(1);
                    p = *fresh0 as u64;
                    sum = (sum as u64).wrapping_add(p) as u64;
                    retval -= p as f64 * FastLog2(p);
                    current_block = 18167736425092750904;
                }
                _ => {
                    let fresh1 = population;
                    population = population.offset(1);
                    p = *fresh1 as u64;
                    sum = (sum as u64).wrapping_add(p) as u64;
                    retval -= p as f64 * FastLog2(p);
                    current_block = 715039052867723359;
                }
            }
        }
        if sum != 0 {
            retval += sum as f64 * FastLog2(sum);
        }
        *total = sum;
        return retval;
    }
}

#[inline(always)]
extern "C" fn Log2FloorNonZero(mut n: u64) -> u32 {
    return 31 ^ (n as u32).leading_zeros() as u32;
}

#[inline(always)]
extern "C" fn FastLog2(mut v: u64) -> f64 {
    unsafe {
        if v < 256 {
            return kBrotliLog2Table[v as usize];
        }
        return log2(v as f64);
    }
}

#[inline(always)]
extern "C" fn BitsEntropy(mut population: *const u32, mut size: u64) -> f64 {
    unsafe {
        let mut sum: u64 = 0;
        let mut retval = ShannonEntropy(population, size, &mut sum);
        if retval < sum as f64 {
            retval = sum as f64;
        }
        return retval;
    }
}

#[inline(always)]
extern "C" fn FindMatchLengthWithLimit(
    mut s1: *const u8,
    mut s2: *const u8,
    mut limit: u64,
) -> u64 {
    unsafe {
        let mut matched = 0;
        let mut limit2 = (limit >> 3i32).wrapping_add(1);
        loop {
            limit2 = limit2.wrapping_sub(1);
            if !((limit2 != 0) as i64 != 0) {
                break;
            }
            if (BrotliUnalignedRead64(s2 as *const libc::c_void)
                == BrotliUnalignedRead64(s1.offset(matched as isize) as *const libc::c_void))
                as i64
                != 0
            {
                s2 = s2.offset(8 as isize);
                matched = (matched as u64).wrapping_add(8) as u64;
            } else {
                let mut x = BrotliUnalignedRead64(s2 as *const libc::c_void)
                    ^ BrotliUnalignedRead64(s1.offset(matched as isize) as *const libc::c_void);
                let mut matching_bits = (x as u64).trailing_zeros() as u64;
                matched = (matched as u64).wrapping_add(matching_bits >> 3) as u64;
                return matched;
            }
        }
        limit = (limit & 7u64).wrapping_add(1);
        loop {
            limit = limit.wrapping_sub(1);
            if !(limit != 0) {
                break;
            }
            if (*s1.offset(matched as isize) as i32 == *s2 as i32) as i64 != 0 {
                s2 = s2.offset(1);
                matched = matched.wrapping_add(1);
            } else {
                return matched;
            }
        }
        return matched;
    }
}

#[inline(always)]
extern "C" fn BrotliWriteBits(
    mut n_bits: u64,
    mut bits: u64,
    mut pos: *mut u64,
    mut array: *mut u8,
) {
    unsafe {
        let mut p: *mut u8 = &mut *array.offset((*pos >> 3i32) as isize) as *mut u8;
        let mut v = *p as u64;
        v |= bits << (*pos & 7);
        BrotliUnalignedWrite64(p as *mut libc::c_void, v);
        *pos = (*pos as u64).wrapping_add(n_bits) as u64;
    }
}

static mut kHashMul32: u32 = 0x1e35a7bd;
#[inline(always)]
extern "C" fn Hash(mut p: *const u8, mut shift: u64, mut length: u64) -> u32 {
    unsafe {
        let h = (BrotliUnalignedRead64(p as *const libc::c_void)
            << 8u64.wrapping_sub(length).wrapping_mul(8))
        .wrapping_mul(kHashMul32 as u64);
        return (h >> shift) as u32;
    }
}

#[inline(always)]
extern "C" fn HashBytesAtOffset(
    mut v: u64,
    mut offset: u64,
    mut shift: u64,
    mut length: u64,
) -> u32 {
    unsafe {
        let h = (v >> 8u64.wrapping_mul(offset) << 8u64.wrapping_sub(length).wrapping_mul(8))
            .wrapping_mul(kHashMul32 as u64);
        return (h >> shift) as u32;
    }
}

#[inline(always)]
extern "C" fn IsMatch(mut p1: *const u8, mut p2: *const u8, mut length: u64) -> i32 {
    unsafe {
        if BrotliUnalignedRead32(p1 as *const libc::c_void)
            == BrotliUnalignedRead32(p2 as *const libc::c_void)
        {
            if length == 4 {
                return 1;
            }
            return if *p1.offset(4 as isize) as i32 == *p2.offset(4 as isize) as i32
                && *p1.offset(5 as isize) as i32 == *p2.offset(5 as isize) as i32
            {
                1
            } else {
                0
            };
        }
        return 0;
    }
}

extern "C" fn BuildAndStoreCommandPrefixCode(
    mut histogram: *const u32,
    mut depth: *mut u8,
    mut bits: *mut u16,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut tree: [HuffmanTree; 129] = [HuffmanTree {
            total_count_: 0,
            index_left_: 0,
            index_right_or_value_: 0,
        }; 129];
        let mut cmd_depth: [u8; 704] = [0; 704];
        let mut cmd_bits: [u16; 64] = [0; 64];
        BrotliCreateHuffmanTree(histogram, 64, 15, tree.as_mut_ptr(), depth);
        BrotliCreateHuffmanTree(
            &*histogram.offset(64 as isize),
            64,
            14,
            tree.as_mut_ptr(),
            &mut *depth.offset(64 as isize),
        );
        memcpy(
            cmd_depth.as_mut_ptr() as *mut libc::c_void,
            depth.offset(24 as isize) as *const libc::c_void,
            24,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(24 as isize) as *mut libc::c_void,
            depth as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(32 as isize) as *mut libc::c_void,
            depth.offset(48 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(40 as isize) as *mut libc::c_void,
            depth.offset(8 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(48 as isize) as *mut libc::c_void,
            depth.offset(56 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(56 as isize) as *mut libc::c_void,
            depth.offset(16 as isize) as *const libc::c_void,
            8,
        );
        BrotliConvertBitDepthsToSymbols(cmd_depth.as_mut_ptr(), 64, cmd_bits.as_mut_ptr());
        memcpy(
            bits as *mut libc::c_void,
            cmd_bits.as_mut_ptr().offset(24 as isize) as *const libc::c_void,
            16,
        );
        memcpy(
            bits.offset(8 as isize) as *mut libc::c_void,
            cmd_bits.as_mut_ptr().offset(40 as isize) as *const libc::c_void,
            16,
        );
        memcpy(
            bits.offset(16 as isize) as *mut libc::c_void,
            cmd_bits.as_mut_ptr().offset(56 as isize) as *const libc::c_void,
            16,
        );
        memcpy(
            bits.offset(24 as isize) as *mut libc::c_void,
            cmd_bits.as_mut_ptr() as *const libc::c_void,
            48,
        );
        memcpy(
            bits.offset(48 as isize) as *mut libc::c_void,
            cmd_bits.as_mut_ptr().offset(32 as isize) as *const libc::c_void,
            16,
        );
        memcpy(
            bits.offset(56 as isize) as *mut libc::c_void,
            cmd_bits.as_mut_ptr().offset(48 as isize) as *const libc::c_void,
            16,
        );
        BrotliConvertBitDepthsToSymbols(
            &mut *depth.offset(64 as isize),
            64,
            &mut *bits.offset(64 as isize),
        );
        let mut i: u64 = 0;
        memset(cmd_depth.as_mut_ptr() as *mut libc::c_void, 0, 64);
        memcpy(
            cmd_depth.as_mut_ptr() as *mut libc::c_void,
            depth.offset(24 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(64 as isize) as *mut libc::c_void,
            depth.offset(32 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(128 as isize) as *mut libc::c_void,
            depth.offset(40 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(192 as isize) as *mut libc::c_void,
            depth.offset(48 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(384 as isize) as *mut libc::c_void,
            depth.offset(56 as isize) as *const libc::c_void,
            8,
        );
        i = 0;
        while i < 8 {
            cmd_depth[128u64.wrapping_add(8u64.wrapping_mul(i)) as usize] =
                *depth.offset(i as isize);
            cmd_depth[256u64.wrapping_add(8u64.wrapping_mul(i)) as usize] =
                *depth.offset(8u64.wrapping_add(i) as isize);
            cmd_depth[448u64.wrapping_add(8u64.wrapping_mul(i)) as usize] =
                *depth.offset(16u64.wrapping_add(i) as isize);
            i = i.wrapping_add(1);
        }
        BrotliStoreHuffmanTree(
            cmd_depth.as_mut_ptr(),
            704,
            tree.as_mut_ptr(),
            storage_ix,
            storage,
        );
        BrotliStoreHuffmanTree(
            &mut *depth.offset(64 as isize),
            64,
            tree.as_mut_ptr(),
            storage_ix,
            storage,
        );
    }
}

#[inline(always)]
extern "C" fn EmitInsertLen(mut insertlen: u32, mut commands: *mut *mut u32) {
    unsafe {
        if insertlen < 6 {
            **commands = insertlen;
        } else if insertlen < 130 {
            let tail = insertlen.wrapping_sub(2);
            let nbits = (Log2FloorNonZero(tail as u64)).wrapping_sub(1);
            let prefix = tail >> nbits;
            let inscode = (nbits << 1i32).wrapping_add(prefix).wrapping_add(2);
            let extra = tail.wrapping_sub(prefix << nbits);
            **commands = inscode | extra << 8;
        } else if insertlen < 2114 {
            let tail_0 = insertlen.wrapping_sub(66);
            let nbits_0 = Log2FloorNonZero(tail_0 as u64);
            let code = nbits_0.wrapping_add(10);
            let extra_0 = tail_0.wrapping_sub(1 << nbits_0);
            **commands = code | extra_0 << 8;
        } else if insertlen < 6210 {
            let extra_1 = insertlen.wrapping_sub(2114);
            **commands = 21 | extra_1 << 8;
        } else if insertlen < 22594 {
            let extra_2 = insertlen.wrapping_sub(6210);
            **commands = 22 | extra_2 << 8;
        } else {
            let extra_3 = insertlen.wrapping_sub(22594);
            **commands = 23 | extra_3 << 8;
        }
        *commands = (*commands).offset(1);
    }
}

#[inline(always)]
extern "C" fn EmitCopyLen(mut copylen: u64, mut commands: *mut *mut u32) {
    unsafe {
        if copylen < 10 {
            **commands = copylen.wrapping_add(38) as u32;
        } else if copylen < 134 {
            let tail = copylen.wrapping_sub(6);
            let nbits = (Log2FloorNonZero(tail)).wrapping_sub(1) as u64;
            let prefix = tail >> nbits;
            let code = (nbits << 1i32).wrapping_add(prefix).wrapping_add(44);
            let extra = tail.wrapping_sub(prefix << nbits);
            **commands = (code | extra << 8i32) as u32;
        } else if copylen < 2118 {
            let tail_0 = copylen.wrapping_sub(70);
            let nbits_0 = Log2FloorNonZero(tail_0) as u64;
            let code_0 = nbits_0.wrapping_add(52);
            let extra_0 = tail_0.wrapping_sub(1 << nbits_0);
            **commands = (code_0 | extra_0 << 8i32) as u32;
        } else {
            let extra_1 = copylen.wrapping_sub(2118);
            **commands = (63 | extra_1 << 8i32) as u32;
        }
        *commands = (*commands).offset(1);
    }
}

#[inline(always)]
extern "C" fn EmitCopyLenLastDistance(mut copylen: u64, mut commands: *mut *mut u32) {
    unsafe {
        if copylen < 12 {
            **commands = copylen.wrapping_add(20) as u32;
            *commands = (*commands).offset(1);
        } else if copylen < 72 {
            let tail = copylen.wrapping_sub(8);
            let nbits = (Log2FloorNonZero(tail)).wrapping_sub(1) as u64;
            let prefix = tail >> nbits;
            let code = (nbits << 1i32).wrapping_add(prefix).wrapping_add(28);
            let extra = tail.wrapping_sub(prefix << nbits);
            **commands = (code | extra << 8i32) as u32;
            *commands = (*commands).offset(1);
        } else if copylen < 136 {
            let tail_0 = copylen.wrapping_sub(8);
            let code_0 = (tail_0 >> 5i32).wrapping_add(54);
            let extra_0 = tail_0 & 31;
            **commands = (code_0 | extra_0 << 8i32) as u32;
            *commands = (*commands).offset(1);
            **commands = 64;
            *commands = (*commands).offset(1);
        } else if copylen < 2120 {
            let tail_1 = copylen.wrapping_sub(72);
            let nbits_0 = Log2FloorNonZero(tail_1) as u64;
            let code_1 = nbits_0.wrapping_add(52);
            let extra_1 = tail_1.wrapping_sub(1 << nbits_0);
            **commands = (code_1 | extra_1 << 8i32) as u32;
            *commands = (*commands).offset(1);
            **commands = 64;
            *commands = (*commands).offset(1);
        } else {
            let extra_2 = copylen.wrapping_sub(2120);
            **commands = (63 | extra_2 << 8i32) as u32;
            *commands = (*commands).offset(1);
            **commands = 64;
            *commands = (*commands).offset(1);
        };
    }
}

#[inline(always)]
extern "C" fn EmitDistance(mut distance: u32, mut commands: *mut *mut u32) {
    unsafe {
        let mut d = distance.wrapping_add(3);
        let mut nbits = (Log2FloorNonZero(d as u64)).wrapping_sub(1);
        let prefix = d >> nbits & 1;
        let offset = 2u32.wrapping_add(prefix) << nbits;
        let distcode = 2u32
            .wrapping_mul(nbits.wrapping_sub(1))
            .wrapping_add(prefix)
            .wrapping_add(80);
        let mut extra = d.wrapping_sub(offset);
        **commands = distcode | extra << 8;
        *commands = (*commands).offset(1);
    }
}

extern "C" fn BrotliStoreMetaBlockHeader(
    mut len: u64,
    mut is_uncompressed: i32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut nibbles = 6;
        BrotliWriteBits(1, 0, storage_ix, storage);
        if len <= (1u32 << 16) as u64 {
            nibbles = 4;
        } else if len <= (1u32 << 20) as u64 {
            nibbles = 5;
        }
        BrotliWriteBits(2, nibbles.wrapping_sub(4), storage_ix, storage);
        BrotliWriteBits(
            nibbles.wrapping_mul(4),
            len.wrapping_sub(1),
            storage_ix,
            storage,
        );
        BrotliWriteBits(1, is_uncompressed as u64, storage_ix, storage);
    }
}

#[inline(always)]
extern "C" fn CreateCommands(
    mut input: *const u8,
    mut block_size: u64,
    mut input_size: u64,
    mut base_ip: *const u8,
    mut table: *mut i32,
    mut table_bits: u64,
    mut min_match: u64,
    mut literals: *mut *mut u8,
    mut commands: *mut *mut u32,
) {
    unsafe {
        let mut current_block: u64;
        let mut ip = input;
        let shift = 64u64.wrapping_sub(table_bits);
        let mut ip_end = input.offset(block_size as isize);
        let mut next_emit = input;
        let mut last_distance = -1;
        let kInputMarginBytes = 16;
        if (block_size >= kInputMarginBytes) as i64 != 0 {
            let len_limit = brotli_min_size_t(
                block_size.wrapping_sub(min_match),
                input_size.wrapping_sub(kInputMarginBytes),
            );
            let mut ip_limit = input.offset(len_limit as isize);
            let mut next_hash: u32 = 0;
            ip = ip.offset(1);
            next_hash = Hash(ip, shift, min_match);
            's_41: loop {
                let mut skip = 32;
                let mut next_ip = ip;
                let mut candidate = 0 as *const u8;
                loop {
                    let mut hash = next_hash;
                    let fresh2 = skip;
                    skip = skip.wrapping_add(1);
                    let mut bytes_between_hash_lookups = fresh2 >> 5;
                    ip = next_ip;
                    next_ip = ip.offset(bytes_between_hash_lookups as isize);
                    if (next_ip > ip_limit) as i64 != 0 {
                        break 's_41;
                    }
                    next_hash = Hash(next_ip, shift, min_match);
                    candidate = ip.offset(-(last_distance as isize));
                    if IsMatch(ip, candidate, min_match) != 0 {
                        if (candidate < ip) as i64 != 0 {
                            *table.offset(hash as isize) = ip.offset_from(base_ip) as i32;
                            current_block = 14818589718467733107;
                        } else {
                            current_block = 1109700713171191020;
                        }
                    } else {
                        current_block = 1109700713171191020;
                    }
                    match current_block {
                        1109700713171191020 => {
                            candidate = base_ip.offset(*table.offset(hash as isize) as isize);
                            *table.offset(hash as isize) = ip.offset_from(base_ip) as i32;
                            if (IsMatch(ip, candidate, min_match) == 0) as i64 != 0 {
                                continue;
                            }
                        }
                        _ => {}
                    }
                    if !(ip.offset_from(candidate) as i64 > (1u64 << 18).wrapping_sub(16) as i64) {
                        break;
                    }
                }
                let mut base = ip;
                let mut matched = min_match.wrapping_add(FindMatchLengthWithLimit(
                    candidate.offset(min_match as isize),
                    ip.offset(min_match as isize),
                    (ip_end.offset_from(ip) as u64).wrapping_sub(min_match),
                ));
                let mut distance = base.offset_from(candidate) as i32;
                let mut insert = base.offset_from(next_emit) as i32;
                ip = ip.offset(matched as isize);
                EmitInsertLen(insert as u32, commands);
                memcpy(
                    *literals as *mut libc::c_void,
                    next_emit as *const libc::c_void,
                    insert as u64,
                );
                *literals = (*literals).offset(insert as isize);
                if distance == last_distance {
                    **commands = 64;
                    *commands = (*commands).offset(1);
                } else {
                    EmitDistance(distance as u32, commands);
                    last_distance = distance;
                }
                EmitCopyLenLastDistance(matched, commands);
                next_emit = ip;
                if (ip >= ip_limit) as i64 != 0 {
                    break;
                }
                let mut input_bytes: u64 = 0;
                let mut cur_hash: u32 = 0;
                let mut prev_hash: u32 = 0;
                if min_match == 4 {
                    input_bytes =
                        BrotliUnalignedRead64(ip.offset(-(3 as isize)) as *const libc::c_void);
                    cur_hash = HashBytesAtOffset(input_bytes, 3, shift, min_match);
                    prev_hash = HashBytesAtOffset(input_bytes, 0, shift, min_match);
                    *table.offset(prev_hash as isize) =
                        (ip.offset_from(base_ip) as i64 - 3i64) as i32;
                    prev_hash = HashBytesAtOffset(input_bytes, 1, shift, min_match);
                    *table.offset(prev_hash as isize) =
                        (ip.offset_from(base_ip) as i64 - 2i64) as i32;
                    prev_hash = HashBytesAtOffset(input_bytes, 0, shift, min_match);
                    *table.offset(prev_hash as isize) =
                        (ip.offset_from(base_ip) as i64 - 1i64) as i32;
                } else {
                    input_bytes =
                        BrotliUnalignedRead64(ip.offset(-(5 as isize)) as *const libc::c_void);
                    prev_hash = HashBytesAtOffset(input_bytes, 0, shift, min_match);
                    *table.offset(prev_hash as isize) =
                        (ip.offset_from(base_ip) as i64 - 5i64) as i32;
                    prev_hash = HashBytesAtOffset(input_bytes, 1, shift, min_match);
                    *table.offset(prev_hash as isize) =
                        (ip.offset_from(base_ip) as i64 - 4i64) as i32;
                    prev_hash = HashBytesAtOffset(input_bytes, 2, shift, min_match);
                    *table.offset(prev_hash as isize) =
                        (ip.offset_from(base_ip) as i64 - 3i64) as i32;
                    input_bytes =
                        BrotliUnalignedRead64(ip.offset(-(2 as isize)) as *const libc::c_void);
                    cur_hash = HashBytesAtOffset(input_bytes, 2, shift, min_match);
                    prev_hash = HashBytesAtOffset(input_bytes, 0, shift, min_match);
                    *table.offset(prev_hash as isize) =
                        (ip.offset_from(base_ip) as i64 - 2i64) as i32;
                    prev_hash = HashBytesAtOffset(input_bytes, 1, shift, min_match);
                    *table.offset(prev_hash as isize) =
                        (ip.offset_from(base_ip) as i64 - 1i64) as i32;
                }
                candidate = base_ip.offset(*table.offset(cur_hash as isize) as isize);
                *table.offset(cur_hash as isize) = ip.offset_from(base_ip) as i32;
                while ip.offset_from(candidate) as i64 <= (1u64 << 18).wrapping_sub(16) as i64
                    && IsMatch(ip, candidate, min_match) != 0
                {
                    let mut base_0 = ip;
                    let mut matched_0 = min_match.wrapping_add(FindMatchLengthWithLimit(
                        candidate.offset(min_match as isize),
                        ip.offset(min_match as isize),
                        (ip_end.offset_from(ip) as u64).wrapping_sub(min_match),
                    ));
                    ip = ip.offset(matched_0 as isize);
                    last_distance = base_0.offset_from(candidate) as i32;
                    EmitCopyLen(matched_0, commands);
                    EmitDistance(last_distance as u32, commands);
                    next_emit = ip;
                    if (ip >= ip_limit) as i64 != 0 {
                        break 's_41;
                    }
                    let mut input_bytes_0: u64 = 0;
                    let mut cur_hash_0: u32 = 0;
                    let mut prev_hash_0: u32 = 0;
                    if min_match == 4 {
                        input_bytes_0 =
                            BrotliUnalignedRead64(ip.offset(-(3 as isize)) as *const libc::c_void);
                        cur_hash_0 = HashBytesAtOffset(input_bytes_0, 3, shift, min_match);
                        prev_hash_0 = HashBytesAtOffset(input_bytes_0, 0, shift, min_match);
                        *table.offset(prev_hash_0 as isize) =
                            (ip.offset_from(base_ip) as i64 - 3i64) as i32;
                        prev_hash_0 = HashBytesAtOffset(input_bytes_0, 1, shift, min_match);
                        *table.offset(prev_hash_0 as isize) =
                            (ip.offset_from(base_ip) as i64 - 2i64) as i32;
                        prev_hash_0 = HashBytesAtOffset(input_bytes_0, 2, shift, min_match);
                        *table.offset(prev_hash_0 as isize) =
                            (ip.offset_from(base_ip) as i64 - 1i64) as i32;
                    } else {
                        input_bytes_0 =
                            BrotliUnalignedRead64(ip.offset(-(5 as isize)) as *const libc::c_void);
                        prev_hash_0 = HashBytesAtOffset(input_bytes_0, 0, shift, min_match);
                        *table.offset(prev_hash_0 as isize) =
                            (ip.offset_from(base_ip) as i64 - 5i64) as i32;
                        prev_hash_0 = HashBytesAtOffset(input_bytes_0, 1, shift, min_match);
                        *table.offset(prev_hash_0 as isize) =
                            (ip.offset_from(base_ip) as i64 - 4i64) as i32;
                        prev_hash_0 = HashBytesAtOffset(input_bytes_0, 2, shift, min_match);
                        *table.offset(prev_hash_0 as isize) =
                            (ip.offset_from(base_ip) as i64 - 3i64) as i32;
                        input_bytes_0 =
                            BrotliUnalignedRead64(ip.offset(-(2 as isize)) as *const libc::c_void);
                        cur_hash_0 = HashBytesAtOffset(input_bytes_0, 2, shift, min_match);
                        prev_hash_0 = HashBytesAtOffset(input_bytes_0, 0, shift, min_match);
                        *table.offset(prev_hash_0 as isize) =
                            (ip.offset_from(base_ip) as i64 - 2i64) as i32;
                        prev_hash_0 = HashBytesAtOffset(input_bytes_0, 1, shift, min_match);
                        *table.offset(prev_hash_0 as isize) =
                            (ip.offset_from(base_ip) as i64 - 1i64) as i32;
                    }
                    candidate = base_ip.offset(*table.offset(cur_hash_0 as isize) as isize);
                    *table.offset(cur_hash_0 as isize) = ip.offset_from(base_ip) as i32;
                }
                ip = ip.offset(1);
                next_hash = Hash(ip, shift, min_match);
            }
        }
        if next_emit < ip_end {
            let insert_0 = ip_end.offset_from(next_emit) as u32;
            EmitInsertLen(insert_0, commands);
            memcpy(
                *literals as *mut libc::c_void,
                next_emit as *const libc::c_void,
                insert_0 as u64,
            );
            *literals = (*literals).offset(insert_0 as isize);
        }
    }
}

extern "C" fn StoreCommands(
    mut m: *mut MemoryManager,
    mut literals: *const u8,
    num_literals: u64,
    mut commands: *const u32,
    num_commands: u64,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        static mut kNumExtraBits: [u32; 128] = [
            0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 7, 8, 9, 10, 12, 14, 24, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5,
            5, 6, 7, 8, 9, 10, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 3,
            3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15,
            16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24,
        ];
        static mut kInsertOffset: [u32; 24] = [
            0, 1, 2, 3, 4, 5, 6, 8, 10, 14, 18, 26, 34, 50, 66, 98, 130, 194, 322, 578, 1090, 2114,
            6210, 22594,
        ];
        let mut lit_depths: [u8; 256] = [0; 256];
        let mut lit_bits: [u16; 256] = [0; 256];
        let mut lit_histo: [u32; 256] = [0; 256];
        let mut cmd_depths: [u8; 128] = [0; 128];
        let mut cmd_bits: [u16; 128] = [0; 128];
        let mut cmd_histo: [u32; 128] = [0; 128];
        let mut i: u64 = 0;
        i = 0;
        while i < num_literals {
            lit_histo[*literals.offset(i as isize) as usize] =
                (lit_histo[*literals.offset(i as isize) as usize]).wrapping_add(1);
            i = i.wrapping_add(1);
        }
        BrotliBuildAndStoreHuffmanTreeFast(
            m,
            lit_histo.as_mut_ptr(),
            num_literals,
            8,
            lit_depths.as_mut_ptr(),
            lit_bits.as_mut_ptr(),
            storage_ix,
            storage,
        );
        if 0 != 0 {
            return;
        }
        i = 0;
        while i < num_commands {
            let code = *commands.offset(i as isize) & 0xff;
            cmd_histo[code as usize] = (cmd_histo[code as usize]).wrapping_add(1);
            i = i.wrapping_add(1);
        }
        cmd_histo[1 as usize] = (cmd_histo[1 as usize] as u32).wrapping_add(1) as u32;
        cmd_histo[2 as usize] = (cmd_histo[2 as usize] as u32).wrapping_add(1) as u32;
        cmd_histo[64 as usize] = (cmd_histo[64 as usize] as u32).wrapping_add(1) as u32;
        cmd_histo[84 as usize] = (cmd_histo[84 as usize] as u32).wrapping_add(1) as u32;
        BuildAndStoreCommandPrefixCode(
            cmd_histo.as_mut_ptr() as *const u32,
            cmd_depths.as_mut_ptr(),
            cmd_bits.as_mut_ptr(),
            storage_ix,
            storage,
        );
        i = 0;
        while i < num_commands {
            let cmd = *commands.offset(i as isize);
            let code_0 = cmd & 0xff;
            let extra = cmd >> 8;
            BrotliWriteBits(
                cmd_depths[code_0 as usize] as u64,
                cmd_bits[code_0 as usize] as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                kNumExtraBits[code_0 as usize] as u64,
                extra as u64,
                storage_ix,
                storage,
            );
            if code_0 < 24 {
                let insert = (kInsertOffset[code_0 as usize]).wrapping_add(extra);
                let mut j: u32 = 0;
                j = 0;
                while j < insert {
                    let lit = *literals;
                    BrotliWriteBits(
                        lit_depths[lit as usize] as u64,
                        lit_bits[lit as usize] as u64,
                        storage_ix,
                        storage,
                    );
                    literals = literals.offset(1);
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn ShouldCompress(
    mut input: *const u8,
    mut input_size: u64,
    mut num_literals: u64,
) -> i32 {
    unsafe {
        let mut corpus_size = input_size as f64;
        if (num_literals as f64) < 0.98f64 * corpus_size {
            return 1;
        } else {
            let mut literal_histo: [u32; 256] = [0; 256];
            let max_total_bit_cost = corpus_size * 8 as f64 * 0.98f64 / 43 as f64;
            let mut i: u64 = 0;
            i = 0;
            while i < input_size {
                literal_histo[*input.offset(i as isize) as usize] =
                    (literal_histo[*input.offset(i as isize) as usize]).wrapping_add(1);
                i = (i).wrapping_add(43) as u64;
            }
            return if BitsEntropy(literal_histo.as_mut_ptr(), 256) < max_total_bit_cost {
                1
            } else {
                0
            };
        };
    }
}

extern "C" fn RewindBitPosition(
    new_storage_ix: u64,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let bitpos = new_storage_ix & 7;
        let mask = (1u32 << bitpos).wrapping_sub(1) as u64;
        let ref mut fresh3 = *storage.offset((new_storage_ix >> 3i32) as isize);
        *fresh3 = (*fresh3 as i32 & mask as i32) as u8;
        *storage_ix = new_storage_ix;
    }
}

extern "C" fn EmitUncompressedMetaBlock(
    mut input: *const u8,
    mut input_size: u64,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        BrotliStoreMetaBlockHeader(input_size, 1, storage_ix, storage);
        *storage_ix = (*storage_ix).wrapping_add(7) & !7 as u64;
        memcpy(
            &mut *storage.offset((*storage_ix >> 3i32) as isize) as *mut u8 as *mut libc::c_void,
            input as *const libc::c_void,
            input_size,
        );
        *storage_ix = (*storage_ix as u64).wrapping_add(input_size << 3) as u64;
        *storage.offset((*storage_ix >> 3i32) as isize) = 0;
    }
}

#[inline(always)]
extern "C" fn BrotliCompressFragmentTwoPassImpl(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut command_buf: *mut u32,
    mut literal_buf: *mut u8,
    mut table: *mut i32,
    mut table_bits: u64,
    mut min_match: u64,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut base_ip = input;
        while input_size > 0 {
            let mut block_size = brotli_min_size_t(input_size, kCompressFragmentTwoPassBlockSize);
            let mut commands = command_buf;
            let mut literals = literal_buf;
            let mut num_literals: u64 = 0;
            CreateCommands(
                input,
                block_size,
                input_size,
                base_ip,
                table,
                table_bits,
                min_match,
                &mut literals,
                &mut commands,
            );
            num_literals = literals.offset_from(literal_buf) as u64;
            if ShouldCompress(input, block_size, num_literals) != 0 {
                let num_commands = commands.offset_from(command_buf) as u64;
                BrotliStoreMetaBlockHeader(block_size, 0, storage_ix, storage);
                BrotliWriteBits(13, 0, storage_ix, storage);
                StoreCommands(
                    m,
                    literal_buf,
                    num_literals,
                    command_buf,
                    num_commands,
                    storage_ix,
                    storage,
                );
                if 0 != 0 {
                    return;
                }
            } else {
                EmitUncompressedMetaBlock(input, block_size, storage_ix, storage);
            }
            input = input.offset(block_size as isize);
            input_size = (input_size as u64).wrapping_sub(block_size) as u64;
        }
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentTwoPassImpl10(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut command_buf: *mut u32,
    mut literal_buf: *mut u8,
    mut table: *mut i32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut min_match = (if 10 <= 15 { 4 } else { 6 }) as u64;
        BrotliCompressFragmentTwoPassImpl(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            10,
            min_match,
            storage_ix,
            storage,
        );
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentTwoPassImpl16(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut command_buf: *mut u32,
    mut literal_buf: *mut u8,
    mut table: *mut i32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut min_match = (if 16 <= 15 { 4 } else { 6 }) as u64;
        BrotliCompressFragmentTwoPassImpl(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            16,
            min_match,
            storage_ix,
            storage,
        );
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentTwoPassImpl15(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut command_buf: *mut u32,
    mut literal_buf: *mut u8,
    mut table: *mut i32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut min_match = (if 15 <= 15 { 4 } else { 6 }) as u64;
        BrotliCompressFragmentTwoPassImpl(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            15,
            min_match,
            storage_ix,
            storage,
        );
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentTwoPassImpl14(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut command_buf: *mut u32,
    mut literal_buf: *mut u8,
    mut table: *mut i32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut min_match = (if 14 <= 15 { 4 } else { 6 }) as u64;
        BrotliCompressFragmentTwoPassImpl(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            14,
            min_match,
            storage_ix,
            storage,
        );
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentTwoPassImpl13(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut command_buf: *mut u32,
    mut literal_buf: *mut u8,
    mut table: *mut i32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut min_match = (if 13 <= 15 { 4 } else { 6 }) as u64;
        BrotliCompressFragmentTwoPassImpl(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            13,
            min_match,
            storage_ix,
            storage,
        );
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentTwoPassImpl12(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut command_buf: *mut u32,
    mut literal_buf: *mut u8,
    mut table: *mut i32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut min_match = (if 12 <= 15 { 4 } else { 6 }) as u64;
        BrotliCompressFragmentTwoPassImpl(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            12,
            min_match,
            storage_ix,
            storage,
        );
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentTwoPassImpl11(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut command_buf: *mut u32,
    mut literal_buf: *mut u8,
    mut table: *mut i32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut min_match = (if 11 <= 15 { 4 } else { 6 }) as u64;
        BrotliCompressFragmentTwoPassImpl(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            11,
            min_match,
            storage_ix,
            storage,
        );
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentTwoPassImpl17(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut command_buf: *mut u32,
    mut literal_buf: *mut u8,
    mut table: *mut i32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut min_match = (if 17 <= 15 { 4 } else { 6 }) as u64;
        BrotliCompressFragmentTwoPassImpl(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            17,
            min_match,
            storage_ix,
            storage,
        );
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentTwoPassImpl9(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut command_buf: *mut u32,
    mut literal_buf: *mut u8,
    mut table: *mut i32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut min_match = (if 9 <= 15 { 4 } else { 6 }) as u64;
        BrotliCompressFragmentTwoPassImpl(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            9,
            min_match,
            storage_ix,
            storage,
        );
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentTwoPassImpl8(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut command_buf: *mut u32,
    mut literal_buf: *mut u8,
    mut table: *mut i32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut min_match = (if 8 <= 15 { 4 } else { 6 }) as u64;
        BrotliCompressFragmentTwoPassImpl(
            m,
            input,
            input_size,
            is_last,
            command_buf,
            literal_buf,
            table,
            8,
            min_match,
            storage_ix,
            storage,
        );
    }
}

#[no_mangle]
pub extern "C" fn BrotliCompressFragmentTwoPass(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut command_buf: *mut u32,
    mut literal_buf: *mut u8,
    mut table: *mut i32,
    mut table_size: u64,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let initial_storage_ix = *storage_ix;
        let table_bits = Log2FloorNonZero(table_size) as u64;
        match table_bits {
            8 => {
                BrotliCompressFragmentTwoPassImpl8(
                    m,
                    input,
                    input_size,
                    is_last,
                    command_buf,
                    literal_buf,
                    table,
                    storage_ix,
                    storage,
                );
            }
            9 => {
                BrotliCompressFragmentTwoPassImpl9(
                    m,
                    input,
                    input_size,
                    is_last,
                    command_buf,
                    literal_buf,
                    table,
                    storage_ix,
                    storage,
                );
            }
            10 => {
                BrotliCompressFragmentTwoPassImpl10(
                    m,
                    input,
                    input_size,
                    is_last,
                    command_buf,
                    literal_buf,
                    table,
                    storage_ix,
                    storage,
                );
            }
            11 => {
                BrotliCompressFragmentTwoPassImpl11(
                    m,
                    input,
                    input_size,
                    is_last,
                    command_buf,
                    literal_buf,
                    table,
                    storage_ix,
                    storage,
                );
            }
            12 => {
                BrotliCompressFragmentTwoPassImpl12(
                    m,
                    input,
                    input_size,
                    is_last,
                    command_buf,
                    literal_buf,
                    table,
                    storage_ix,
                    storage,
                );
            }
            13 => {
                BrotliCompressFragmentTwoPassImpl13(
                    m,
                    input,
                    input_size,
                    is_last,
                    command_buf,
                    literal_buf,
                    table,
                    storage_ix,
                    storage,
                );
            }
            14 => {
                BrotliCompressFragmentTwoPassImpl14(
                    m,
                    input,
                    input_size,
                    is_last,
                    command_buf,
                    literal_buf,
                    table,
                    storage_ix,
                    storage,
                );
            }
            15 => {
                BrotliCompressFragmentTwoPassImpl15(
                    m,
                    input,
                    input_size,
                    is_last,
                    command_buf,
                    literal_buf,
                    table,
                    storage_ix,
                    storage,
                );
            }
            16 => {
                BrotliCompressFragmentTwoPassImpl16(
                    m,
                    input,
                    input_size,
                    is_last,
                    command_buf,
                    literal_buf,
                    table,
                    storage_ix,
                    storage,
                );
            }
            17 => {
                BrotliCompressFragmentTwoPassImpl17(
                    m,
                    input,
                    input_size,
                    is_last,
                    command_buf,
                    literal_buf,
                    table,
                    storage_ix,
                    storage,
                );
            }
            _ => {}
        }
        if (*storage_ix).wrapping_sub(initial_storage_ix) > 31u64.wrapping_add(input_size << 3) {
            RewindBitPosition(initial_storage_ix, storage_ix, storage);
            EmitUncompressedMetaBlock(input, input_size, storage_ix, storage);
        }
        if is_last != 0 {
            BrotliWriteBits(1, 1, storage_ix, storage);
            BrotliWriteBits(1, 1, storage_ix, storage);
            *storage_ix = (*storage_ix).wrapping_add(7) & !7 as u64;
        }
    }
}
