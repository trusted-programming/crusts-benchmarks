use libc;
extern "C" {
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn log2(_: f64) -> f64;
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
    fn BrotliCreateHuffmanTree(
        data: *const u32,
        length: u64,
        tree_limit: i32,
        tree: *mut HuffmanTree,
        depth: *mut u8,
    );
    fn BrotliConvertBitDepthsToSymbols(depth: *const u8, len: u64, bits: *mut u16);
    fn BrotliStoreHuffmanTree(
        depths: *const u8,
        num: u64,
        tree: *mut HuffmanTree,
        storage_ix: *mut u64,
        storage: *mut u8,
    );
    static kBrotliLog2Table: [f64; 256];
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

#[inline(always)]
extern "C" fn brotli_min_uint32_t(mut a: u32, mut b: u32) -> u32 {
    return if a < b { a } else { b };
}

#[inline(always)]
extern "C" fn BrotliUnalignedRead32(mut p: *const libc::c_void) -> u32 {
    unsafe {
        return *(p as *const u32);
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
extern "C" fn Hash(mut p: *const u8, mut shift: u64) -> u32 {
    unsafe {
        let h = (BrotliUnalignedRead64(p as *const libc::c_void) << 24i32)
            .wrapping_mul(kHashMul32 as u64);
        return (h >> shift) as u32;
    }
}

#[inline(always)]
extern "C" fn HashBytesAtOffset(mut v: u64, mut offset: i32, mut shift: u64) -> u32 {
    unsafe {
        let h = (v >> 8 * offset << 24i32).wrapping_mul(kHashMul32 as u64);
        return (h >> shift) as u32;
    }
}

#[inline(always)]
extern "C" fn IsMatch(mut p1: *const u8, mut p2: *const u8) -> i32 {
    unsafe {
        return if BrotliUnalignedRead32(p1 as *const libc::c_void)
            == BrotliUnalignedRead32(p2 as *const libc::c_void)
            && *p1.offset(4 as isize) as i32 == *p2.offset(4 as isize) as i32
        {
            1
        } else {
            0
        };
    }
}

extern "C" fn BuildAndStoreLiteralPrefixCode(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    input_size: u64,
    mut depths: *mut u8,
    mut bits: *mut u16,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) -> u64 {
    unsafe {
        let mut histogram: [u32; 256] = [0; 256];
        let mut histogram_total: u64 = 0;
        let mut i: u64 = 0;
        if input_size < (1i32 << 15) as u64 {
            i = 0;
            while i < input_size {
                histogram[*input.offset(i as isize) as usize] =
                    (histogram[*input.offset(i as isize) as usize]).wrapping_add(1);
                i = i.wrapping_add(1);
            }
            histogram_total = input_size;
            i = 0;
            while i < 256 {
                let adjust = 2u32.wrapping_mul(brotli_min_uint32_t(histogram[i as usize], 11));
                histogram[i as usize] = (histogram[i as usize] as u32).wrapping_add(adjust) as u32;
                histogram_total = (histogram_total).wrapping_add(adjust as u64) as u64;
                i = i.wrapping_add(1);
            }
        } else {
            static mut kSampleRate: u64 = 29;
            i = 0;
            while i < input_size {
                histogram[*input.offset(i as isize) as usize] =
                    (histogram[*input.offset(i as isize) as usize]).wrapping_add(1);
                i = (i).wrapping_add(kSampleRate) as u64;
            }
            histogram_total = input_size
                .wrapping_add(kSampleRate)
                .wrapping_sub(1)
                .wrapping_div(kSampleRate);
            i = 0;
            while i < 256 {
                let adjust_0 = 1u32.wrapping_add(
                    2u32.wrapping_mul(brotli_min_uint32_t(histogram[i as usize], 11)),
                );
                histogram[i as usize] =
                    (histogram[i as usize] as u32).wrapping_add(adjust_0) as u32;
                histogram_total = (histogram_total).wrapping_add(adjust_0 as u64) as u64;
                i = i.wrapping_add(1);
            }
        }
        BrotliBuildAndStoreHuffmanTreeFast(
            m,
            histogram.as_mut_ptr(),
            histogram_total,
            8,
            depths,
            bits,
            storage_ix,
            storage,
        );
        if 0 != 0 {
            return 0;
        }
        let mut literal_ratio = 0;
        i = 0;
        while i < 256 {
            if histogram[i as usize] != 0 {
                literal_ratio = (literal_ratio as u64).wrapping_add(
                    (histogram[i as usize]).wrapping_mul(*depths.offset(i as isize) as u32) as u64,
                ) as u64;
            }
            i = i.wrapping_add(1);
        }
        return literal_ratio
            .wrapping_mul(125)
            .wrapping_div(histogram_total);
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
            depth as *const libc::c_void,
            24,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(24 as isize) as *mut libc::c_void,
            depth.offset(40 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(32 as isize) as *mut libc::c_void,
            depth.offset(24 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(40 as isize) as *mut libc::c_void,
            depth.offset(48 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(48 as isize) as *mut libc::c_void,
            depth.offset(32 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(56 as isize) as *mut libc::c_void,
            depth.offset(56 as isize) as *const libc::c_void,
            8,
        );
        BrotliConvertBitDepthsToSymbols(cmd_depth.as_mut_ptr(), 64, cmd_bits.as_mut_ptr());
        memcpy(
            bits as *mut libc::c_void,
            cmd_bits.as_mut_ptr() as *const libc::c_void,
            48,
        );
        memcpy(
            bits.offset(24 as isize) as *mut libc::c_void,
            cmd_bits.as_mut_ptr().offset(32 as isize) as *const libc::c_void,
            16,
        );
        memcpy(
            bits.offset(32 as isize) as *mut libc::c_void,
            cmd_bits.as_mut_ptr().offset(48 as isize) as *const libc::c_void,
            16,
        );
        memcpy(
            bits.offset(40 as isize) as *mut libc::c_void,
            cmd_bits.as_mut_ptr().offset(24 as isize) as *const libc::c_void,
            16,
        );
        memcpy(
            bits.offset(48 as isize) as *mut libc::c_void,
            cmd_bits.as_mut_ptr().offset(40 as isize) as *const libc::c_void,
            16,
        );
        memcpy(
            bits.offset(56 as isize) as *mut libc::c_void,
            cmd_bits.as_mut_ptr().offset(56 as isize) as *const libc::c_void,
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
            depth as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(64 as isize) as *mut libc::c_void,
            depth.offset(8 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(128 as isize) as *mut libc::c_void,
            depth.offset(16 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(192 as isize) as *mut libc::c_void,
            depth.offset(24 as isize) as *const libc::c_void,
            8,
        );
        memcpy(
            cmd_depth.as_mut_ptr().offset(384 as isize) as *mut libc::c_void,
            depth.offset(32 as isize) as *const libc::c_void,
            8,
        );
        i = 0;
        while i < 8 {
            cmd_depth[128u64.wrapping_add(8u64.wrapping_mul(i)) as usize] =
                *depth.offset(40u64.wrapping_add(i) as isize);
            cmd_depth[256u64.wrapping_add(8u64.wrapping_mul(i)) as usize] =
                *depth.offset(48u64.wrapping_add(i) as isize);
            cmd_depth[448u64.wrapping_add(8u64.wrapping_mul(i)) as usize] =
                *depth.offset(56u64.wrapping_add(i) as isize);
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
extern "C" fn EmitInsertLen(
    mut insertlen: u64,
    mut depth: *const u8,
    mut bits: *const u16,
    mut histo: *mut u32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        if insertlen < 6 {
            let code = insertlen.wrapping_add(40);
            BrotliWriteBits(
                *depth.offset(code as isize) as u64,
                *bits.offset(code as isize) as u64,
                storage_ix,
                storage,
            );
            let ref mut fresh0 = *histo.offset(code as isize);
            *fresh0 = (*fresh0).wrapping_add(1);
        } else if insertlen < 130 {
            let tail = insertlen.wrapping_sub(2);
            let nbits = (Log2FloorNonZero(tail)).wrapping_sub(1);
            let prefix = tail >> nbits;
            let inscode = ((nbits << 1i32) as u64)
                .wrapping_add(prefix)
                .wrapping_add(42);
            BrotliWriteBits(
                *depth.offset(inscode as isize) as u64,
                *bits.offset(inscode as isize) as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                nbits as u64,
                tail.wrapping_sub(prefix << nbits),
                storage_ix,
                storage,
            );
            let ref mut fresh1 = *histo.offset(inscode as isize);
            *fresh1 = (*fresh1).wrapping_add(1);
        } else if insertlen < 2114 {
            let tail_0 = insertlen.wrapping_sub(66);
            let nbits_0 = Log2FloorNonZero(tail_0);
            let code_0 = nbits_0.wrapping_add(50) as u64;
            BrotliWriteBits(
                *depth.offset(code_0 as isize) as u64,
                *bits.offset(code_0 as isize) as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                nbits_0 as u64,
                tail_0.wrapping_sub(1 << nbits_0),
                storage_ix,
                storage,
            );
            let ref mut fresh2 = *histo.offset(code_0 as isize);
            *fresh2 = (*fresh2).wrapping_add(1);
        } else {
            BrotliWriteBits(
                *depth.offset(61 as isize) as u64,
                *bits.offset(61 as isize) as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(12, insertlen.wrapping_sub(2114), storage_ix, storage);
            let ref mut fresh3 = *histo.offset(61 as isize);
            *fresh3 = (*fresh3).wrapping_add(1);
        };
    }
}

#[inline(always)]
extern "C" fn EmitLongInsertLen(
    mut insertlen: u64,
    mut depth: *const u8,
    mut bits: *const u16,
    mut histo: *mut u32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        if insertlen < 22594 {
            BrotliWriteBits(
                *depth.offset(62 as isize) as u64,
                *bits.offset(62 as isize) as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(14, insertlen.wrapping_sub(6210), storage_ix, storage);
            let ref mut fresh4 = *histo.offset(62 as isize);
            *fresh4 = (*fresh4).wrapping_add(1);
        } else {
            BrotliWriteBits(
                *depth.offset(63 as isize) as u64,
                *bits.offset(63 as isize) as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(24, insertlen.wrapping_sub(22594), storage_ix, storage);
            let ref mut fresh5 = *histo.offset(63 as isize);
            *fresh5 = (*fresh5).wrapping_add(1);
        };
    }
}

#[inline(always)]
extern "C" fn EmitCopyLen(
    mut copylen: u64,
    mut depth: *const u8,
    mut bits: *const u16,
    mut histo: *mut u32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        if copylen < 10 {
            BrotliWriteBits(
                *depth.offset(copylen.wrapping_add(14) as isize) as u64,
                *bits.offset(copylen.wrapping_add(14) as isize) as u64,
                storage_ix,
                storage,
            );
            let ref mut fresh6 = *histo.offset(copylen.wrapping_add(14) as isize);
            *fresh6 = (*fresh6).wrapping_add(1);
        } else if copylen < 134 {
            let tail = copylen.wrapping_sub(6);
            let nbits = (Log2FloorNonZero(tail)).wrapping_sub(1);
            let prefix = tail >> nbits;
            let code = ((nbits << 1i32) as u64)
                .wrapping_add(prefix)
                .wrapping_add(20);
            BrotliWriteBits(
                *depth.offset(code as isize) as u64,
                *bits.offset(code as isize) as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                nbits as u64,
                tail.wrapping_sub(prefix << nbits),
                storage_ix,
                storage,
            );
            let ref mut fresh7 = *histo.offset(code as isize);
            *fresh7 = (*fresh7).wrapping_add(1);
        } else if copylen < 2118 {
            let tail_0 = copylen.wrapping_sub(70);
            let nbits_0 = Log2FloorNonZero(tail_0);
            let code_0 = nbits_0.wrapping_add(28) as u64;
            BrotliWriteBits(
                *depth.offset(code_0 as isize) as u64,
                *bits.offset(code_0 as isize) as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                nbits_0 as u64,
                tail_0.wrapping_sub(1 << nbits_0),
                storage_ix,
                storage,
            );
            let ref mut fresh8 = *histo.offset(code_0 as isize);
            *fresh8 = (*fresh8).wrapping_add(1);
        } else {
            BrotliWriteBits(
                *depth.offset(39 as isize) as u64,
                *bits.offset(39 as isize) as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(24, copylen.wrapping_sub(2118), storage_ix, storage);
            let ref mut fresh9 = *histo.offset(39 as isize);
            *fresh9 = (*fresh9).wrapping_add(1);
        };
    }
}

#[inline(always)]
extern "C" fn EmitCopyLenLastDistance(
    mut copylen: u64,
    mut depth: *const u8,
    mut bits: *const u16,
    mut histo: *mut u32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        if copylen < 12 {
            BrotliWriteBits(
                *depth.offset(copylen.wrapping_sub(4) as isize) as u64,
                *bits.offset(copylen.wrapping_sub(4) as isize) as u64,
                storage_ix,
                storage,
            );
            let ref mut fresh10 = *histo.offset(copylen.wrapping_sub(4) as isize);
            *fresh10 = (*fresh10).wrapping_add(1);
        } else if copylen < 72 {
            let tail = copylen.wrapping_sub(8);
            let nbits = (Log2FloorNonZero(tail)).wrapping_sub(1);
            let prefix = tail >> nbits;
            let code = ((nbits << 1i32) as u64)
                .wrapping_add(prefix)
                .wrapping_add(4);
            BrotliWriteBits(
                *depth.offset(code as isize) as u64,
                *bits.offset(code as isize) as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                nbits as u64,
                tail.wrapping_sub(prefix << nbits),
                storage_ix,
                storage,
            );
            let ref mut fresh11 = *histo.offset(code as isize);
            *fresh11 = (*fresh11).wrapping_add(1);
        } else if copylen < 136 {
            let tail_0 = copylen.wrapping_sub(8);
            let code_0 = (tail_0 >> 5i32).wrapping_add(30);
            BrotliWriteBits(
                *depth.offset(code_0 as isize) as u64,
                *bits.offset(code_0 as isize) as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(5, tail_0 & 31, storage_ix, storage);
            BrotliWriteBits(
                *depth.offset(64 as isize) as u64,
                *bits.offset(64 as isize) as u64,
                storage_ix,
                storage,
            );
            let ref mut fresh12 = *histo.offset(code_0 as isize);
            *fresh12 = (*fresh12).wrapping_add(1);
            let ref mut fresh13 = *histo.offset(64 as isize);
            *fresh13 = (*fresh13).wrapping_add(1);
        } else if copylen < 2120 {
            let tail_1 = copylen.wrapping_sub(72);
            let nbits_0 = Log2FloorNonZero(tail_1);
            let code_1 = nbits_0.wrapping_add(28) as u64;
            BrotliWriteBits(
                *depth.offset(code_1 as isize) as u64,
                *bits.offset(code_1 as isize) as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                nbits_0 as u64,
                tail_1.wrapping_sub(1 << nbits_0),
                storage_ix,
                storage,
            );
            BrotliWriteBits(
                *depth.offset(64 as isize) as u64,
                *bits.offset(64 as isize) as u64,
                storage_ix,
                storage,
            );
            let ref mut fresh14 = *histo.offset(code_1 as isize);
            *fresh14 = (*fresh14).wrapping_add(1);
            let ref mut fresh15 = *histo.offset(64 as isize);
            *fresh15 = (*fresh15).wrapping_add(1);
        } else {
            BrotliWriteBits(
                *depth.offset(39 as isize) as u64,
                *bits.offset(39 as isize) as u64,
                storage_ix,
                storage,
            );
            BrotliWriteBits(24, copylen.wrapping_sub(2120), storage_ix, storage);
            BrotliWriteBits(
                *depth.offset(64 as isize) as u64,
                *bits.offset(64 as isize) as u64,
                storage_ix,
                storage,
            );
            let ref mut fresh16 = *histo.offset(39 as isize);
            *fresh16 = (*fresh16).wrapping_add(1);
            let ref mut fresh17 = *histo.offset(64 as isize);
            *fresh17 = (*fresh17).wrapping_add(1);
        };
    }
}

#[inline(always)]
extern "C" fn EmitDistance(
    mut distance: u64,
    mut depth: *const u8,
    mut bits: *const u16,
    mut histo: *mut u32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let d = distance.wrapping_add(3);
        let nbits = (Log2FloorNonZero(d)).wrapping_sub(1);
        let prefix = d >> nbits & 1;
        let offset = 2u64.wrapping_add(prefix) << nbits;
        let distcode = (2u32.wrapping_mul(nbits.wrapping_sub(1)) as u64)
            .wrapping_add(prefix)
            .wrapping_add(80);
        BrotliWriteBits(
            *depth.offset(distcode as isize) as u64,
            *bits.offset(distcode as isize) as u64,
            storage_ix,
            storage,
        );
        BrotliWriteBits(nbits as u64, d.wrapping_sub(offset), storage_ix, storage);
        let ref mut fresh18 = *histo.offset(distcode as isize);
        *fresh18 = (*fresh18).wrapping_add(1);
    }
}

#[inline(always)]
extern "C" fn EmitLiterals(
    mut input: *const u8,
    len: u64,
    mut depth: *const u8,
    mut bits: *const u16,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut j: u64 = 0;
        j = 0;
        while j < len {
            let lit = *input.offset(j as isize);
            BrotliWriteBits(
                *depth.offset(lit as isize) as u64,
                *bits.offset(lit as isize) as u64,
                storage_ix,
                storage,
            );
            j = j.wrapping_add(1);
        }
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

extern "C" fn UpdateBits(mut n_bits: u64, mut bits: u32, mut pos: u64, mut array: *mut u8) {
    unsafe {
        while n_bits > 0 {
            let mut byte_pos = pos >> 3;
            let mut n_unchanged_bits = pos & 7;
            let mut n_changed_bits = brotli_min_size_t(n_bits, 8u64.wrapping_sub(n_unchanged_bits));
            let mut total_bits = n_unchanged_bits.wrapping_add(n_changed_bits);
            let mut mask =
                !(1u32 << total_bits).wrapping_sub(1) | (1u32 << n_unchanged_bits).wrapping_sub(1);
            let mut unchanged_bits = *array.offset(byte_pos as isize) as u32 & mask;
            let mut changed_bits = bits & (1u32 << n_changed_bits).wrapping_sub(1);
            *array.offset(byte_pos as isize) =
                (changed_bits << n_unchanged_bits | unchanged_bits) as u8;
            n_bits = (n_bits as u64).wrapping_sub(n_changed_bits) as u64;
            bits >>= n_changed_bits;
            pos = (pos as u64).wrapping_add(n_changed_bits) as u64;
        }
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
        let ref mut fresh19 = *storage.offset((new_storage_ix >> 3i32) as isize);
        *fresh19 = (*fresh19 as i32 & mask as i32) as u8;
        *storage_ix = new_storage_ix;
    }
}

extern "C" fn ShouldMergeBlock(mut data: *const u8, mut len: u64, mut depths: *const u8) -> i32 {
    unsafe {
        let mut histo: [u64; 256] = [0; 256];
        static mut kSampleRate: u64 = 43;
        let mut i: u64 = 0;
        i = 0;
        while i < len {
            histo[*data.offset(i as isize) as usize] =
                (histo[*data.offset(i as isize) as usize]).wrapping_add(1);
            i = (i).wrapping_add(kSampleRate) as u64;
        }
        let total = len
            .wrapping_add(kSampleRate)
            .wrapping_sub(1)
            .wrapping_div(kSampleRate);
        let mut r = (FastLog2(total) + 0.5f64) * total as f64 + 200 as f64;
        i = 0;
        while i < 256 {
            r -= histo[i as usize] as f64
                * (*depths.offset(i as isize) as i32 as f64 + FastLog2(histo[i as usize]));
            i = i.wrapping_add(1);
        }
        return if r >= 0.0f64 { 1 } else { 0 };
    }
}

#[inline(always)]
extern "C" fn ShouldUseUncompressedMode(
    mut metablock_start: *const u8,
    mut next_emit: *const u8,
    insertlen: u64,
    literal_ratio: u64,
) -> i32 {
    unsafe {
        let compressed = next_emit.offset_from(metablock_start) as u64;
        if compressed.wrapping_mul(50) > insertlen {
            return 0;
        } else {
            return if literal_ratio > 980 { 1 } else { 0 };
        };
    }
}

extern "C" fn EmitUncompressedMetaBlock(
    mut begin: *const u8,
    mut end: *const u8,
    storage_ix_start: u64,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let len = end.offset_from(begin) as u64;
        RewindBitPosition(storage_ix_start, storage_ix, storage);
        BrotliStoreMetaBlockHeader(len, 1, storage_ix, storage);
        *storage_ix = (*storage_ix).wrapping_add(7) & !7 as u64;
        memcpy(
            &mut *storage.offset((*storage_ix >> 3i32) as isize) as *mut u8 as *mut libc::c_void,
            begin as *const libc::c_void,
            len,
        );
        *storage_ix = (*storage_ix as u64).wrapping_add(len << 3) as u64;
        *storage.offset((*storage_ix >> 3i32) as isize) = 0;
    }
}

static mut kCmdHistoSeed: [u32; 128] = [
    0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0,
];
#[inline(always)]
extern "C" fn BrotliCompressFragmentFastImpl(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut table: *mut i32,
    mut table_bits: u64,
    mut cmd_depth: *mut u8,
    mut cmd_bits: *mut u16,
    mut cmd_code_numbits: *mut u64,
    mut cmd_code: *mut u8,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut current_block: u64;
        let mut cmd_histo: [u32; 128] = [0; 128];
        let mut ip_end = 0 as *const u8;
        let mut next_emit = input;
        let mut base_ip = input;
        static mut kFirstBlockSize: u64 = (3i32 << 15) as u64;
        static mut kMergeBlockSize: u64 = (1i32 << 16) as u64;
        let kInputMarginBytes = 16;
        let kMinMatchLen = 5;
        let mut metablock_start = input;
        let mut block_size = brotli_min_size_t(input_size, kFirstBlockSize);
        let mut total_block_size = block_size;
        let mut mlen_storage_ix = (*storage_ix).wrapping_add(3);
        let mut lit_depth: [u8; 256] = [0; 256];
        let mut lit_bits: [u16; 256] = [0; 256];
        let mut literal_ratio: u64 = 0;
        let mut ip = 0 as *const u8;
        let mut last_distance: i32 = 0;
        let shift = 64u64.wrapping_sub(table_bits);
        BrotliStoreMetaBlockHeader(block_size, 0, storage_ix, storage);
        BrotliWriteBits(13, 0, storage_ix, storage);
        literal_ratio = BuildAndStoreLiteralPrefixCode(
            m,
            input,
            block_size,
            lit_depth.as_mut_ptr(),
            lit_bits.as_mut_ptr(),
            storage_ix,
            storage,
        );
        if 0 != 0 {
            return;
        }
        let mut i: u64 = 0;
        i = 0;
        while i.wrapping_add(7) < *cmd_code_numbits {
            BrotliWriteBits(
                8,
                *cmd_code.offset((i >> 3i32) as isize) as u64,
                storage_ix,
                storage,
            );
            i = (i).wrapping_add(8) as u64;
        }
        BrotliWriteBits(
            *cmd_code_numbits & 7,
            *cmd_code.offset((*cmd_code_numbits >> 3i32) as isize) as u64,
            storage_ix,
            storage,
        );
        loop {
            memcpy(
                cmd_histo.as_mut_ptr() as *mut libc::c_void,
                kCmdHistoSeed.as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[u32; 128]>() as u64,
            );
            ip = input;
            last_distance = -1;
            ip_end = input.offset(block_size as isize);
            if (block_size >= kInputMarginBytes) as i64 != 0 {
                let len_limit = brotli_min_size_t(
                    block_size.wrapping_sub(kMinMatchLen),
                    input_size.wrapping_sub(kInputMarginBytes),
                );
                let mut ip_limit = input.offset(len_limit as isize);
                let mut next_hash: u32 = 0;
                ip = ip.offset(1);
                next_hash = Hash(ip, shift);
                's_147: loop {
                    let mut skip = 32;
                    let mut next_ip = ip;
                    let mut candidate = 0 as *const u8;
                    loop {
                        let mut hash = next_hash;
                        let fresh20 = skip;
                        skip = skip.wrapping_add(1);
                        let mut bytes_between_hash_lookups = fresh20 >> 5;
                        ip = next_ip;
                        next_ip = ip.offset(bytes_between_hash_lookups as isize);
                        if (next_ip > ip_limit) as i64 != 0 {
                            current_block = 2904036176499606090;
                            break 's_147;
                        }
                        next_hash = Hash(next_ip, shift);
                        candidate = ip.offset(-(last_distance as isize));
                        if IsMatch(ip, candidate) != 0 {
                            if (candidate < ip) as i64 != 0 {
                                *table.offset(hash as isize) = ip.offset_from(base_ip) as i32;
                                current_block = 2989495919056355252;
                            } else {
                                current_block = 1847472278776910194;
                            }
                        } else {
                            current_block = 1847472278776910194;
                        }
                        match current_block {
                            1847472278776910194 => {
                                candidate = base_ip.offset(*table.offset(hash as isize) as isize);
                                *table.offset(hash as isize) = ip.offset_from(base_ip) as i32;
                                if (IsMatch(ip, candidate) == 0) as i64 != 0 {
                                    continue;
                                }
                            }
                            _ => {}
                        }
                        if !(ip.offset_from(candidate) as i64
                            > (1u64 << 18).wrapping_sub(16) as i64)
                        {
                            break;
                        }
                    }
                    let mut base = ip;
                    let mut matched = 5u64.wrapping_add(FindMatchLengthWithLimit(
                        candidate.offset(5 as isize),
                        ip.offset(5 as isize),
                        (ip_end.offset_from(ip) as u64).wrapping_sub(5),
                    ));
                    let mut distance = base.offset_from(candidate) as i32;
                    let mut insert = base.offset_from(next_emit) as u64;
                    ip = ip.offset(matched as isize);
                    if (insert < 6210) as i64 != 0 {
                        EmitInsertLen(
                            insert,
                            cmd_depth as *const u8,
                            cmd_bits as *const u16,
                            cmd_histo.as_mut_ptr(),
                            storage_ix,
                            storage,
                        );
                    } else if ShouldUseUncompressedMode(
                        metablock_start,
                        next_emit,
                        insert,
                        literal_ratio,
                    ) != 0
                    {
                        EmitUncompressedMetaBlock(
                            metablock_start,
                            base,
                            mlen_storage_ix.wrapping_sub(3),
                            storage_ix,
                            storage,
                        );
                        input_size =
                            (input_size as u64).wrapping_sub(base.offset_from(input) as u64) as u64;
                        input = base;
                        next_emit = input;
                        current_block = 17084314706199238786;
                        break;
                    } else {
                        EmitLongInsertLen(
                            insert,
                            cmd_depth as *const u8,
                            cmd_bits as *const u16,
                            cmd_histo.as_mut_ptr(),
                            storage_ix,
                            storage,
                        );
                    }
                    EmitLiterals(
                        next_emit,
                        insert,
                        lit_depth.as_mut_ptr() as *const u8,
                        lit_bits.as_mut_ptr() as *const u16,
                        storage_ix,
                        storage,
                    );
                    if distance == last_distance {
                        BrotliWriteBits(
                            *cmd_depth.offset(64 as isize) as u64,
                            *cmd_bits.offset(64 as isize) as u64,
                            storage_ix,
                            storage,
                        );
                        cmd_histo[64 as usize] = (cmd_histo[64 as usize]).wrapping_add(1);
                    } else {
                        EmitDistance(
                            distance as u64,
                            cmd_depth as *const u8,
                            cmd_bits as *const u16,
                            cmd_histo.as_mut_ptr(),
                            storage_ix,
                            storage,
                        );
                        last_distance = distance;
                    }
                    EmitCopyLenLastDistance(
                        matched,
                        cmd_depth as *const u8,
                        cmd_bits as *const u16,
                        cmd_histo.as_mut_ptr(),
                        storage_ix,
                        storage,
                    );
                    next_emit = ip;
                    if (ip >= ip_limit) as i64 != 0 {
                        current_block = 2904036176499606090;
                        break;
                    }
                    let mut input_bytes =
                        BrotliUnalignedRead64(ip.offset(-(3 as isize)) as *const libc::c_void);
                    let mut prev_hash = HashBytesAtOffset(input_bytes, 0, shift);
                    let mut cur_hash = HashBytesAtOffset(input_bytes, 3, shift);
                    *table.offset(prev_hash as isize) =
                        (ip.offset_from(base_ip) as i64 - 3i64) as i32;
                    prev_hash = HashBytesAtOffset(input_bytes, 1, shift);
                    *table.offset(prev_hash as isize) =
                        (ip.offset_from(base_ip) as i64 - 2i64) as i32;
                    prev_hash = HashBytesAtOffset(input_bytes, 2, shift);
                    *table.offset(prev_hash as isize) =
                        (ip.offset_from(base_ip) as i64 - 1i64) as i32;
                    candidate = base_ip.offset(*table.offset(cur_hash as isize) as isize);
                    *table.offset(cur_hash as isize) = ip.offset_from(base_ip) as i32;
                    while IsMatch(ip, candidate) != 0 {
                        let mut base_0 = ip;
                        let mut matched_0 = 5u64.wrapping_add(FindMatchLengthWithLimit(
                            candidate.offset(5 as isize),
                            ip.offset(5 as isize),
                            (ip_end.offset_from(ip) as u64).wrapping_sub(5),
                        ));
                        if ip.offset_from(candidate) as i64 > (1u64 << 18).wrapping_sub(16) as i64 {
                            break;
                        }
                        ip = ip.offset(matched_0 as isize);
                        last_distance = base_0.offset_from(candidate) as i32;
                        EmitCopyLen(
                            matched_0,
                            cmd_depth as *const u8,
                            cmd_bits as *const u16,
                            cmd_histo.as_mut_ptr(),
                            storage_ix,
                            storage,
                        );
                        EmitDistance(
                            last_distance as u64,
                            cmd_depth as *const u8,
                            cmd_bits as *const u16,
                            cmd_histo.as_mut_ptr(),
                            storage_ix,
                            storage,
                        );
                        next_emit = ip;
                        if (ip >= ip_limit) as i64 != 0 {
                            current_block = 2904036176499606090;
                            break 's_147;
                        }
                        let mut input_bytes_0 =
                            BrotliUnalignedRead64(ip.offset(-(3 as isize)) as *const libc::c_void);
                        let mut prev_hash_0 = HashBytesAtOffset(input_bytes_0, 0, shift);
                        let mut cur_hash_0 = HashBytesAtOffset(input_bytes_0, 3, shift);
                        *table.offset(prev_hash_0 as isize) =
                            (ip.offset_from(base_ip) as i64 - 3i64) as i32;
                        prev_hash_0 = HashBytesAtOffset(input_bytes_0, 1, shift);
                        *table.offset(prev_hash_0 as isize) =
                            (ip.offset_from(base_ip) as i64 - 2i64) as i32;
                        prev_hash_0 = HashBytesAtOffset(input_bytes_0, 2, shift);
                        *table.offset(prev_hash_0 as isize) =
                            (ip.offset_from(base_ip) as i64 - 1i64) as i32;
                        candidate = base_ip.offset(*table.offset(cur_hash_0 as isize) as isize);
                        *table.offset(cur_hash_0 as isize) = ip.offset_from(base_ip) as i32;
                    }
                    ip = ip.offset(1);
                    next_hash = Hash(ip, shift);
                }
            } else {
                current_block = 2904036176499606090;
            }
            match current_block {
                2904036176499606090 => {
                    input = input.offset(block_size as isize);
                    input_size = (input_size as u64).wrapping_sub(block_size) as u64;
                    block_size = brotli_min_size_t(input_size, kMergeBlockSize);
                    if input_size > 0
                        && total_block_size.wrapping_add(block_size) <= (1i32 << 20) as u64
                        && ShouldMergeBlock(input, block_size, lit_depth.as_mut_ptr()) != 0
                    {
                        total_block_size =
                            (total_block_size as u64).wrapping_add(block_size) as u64;
                        UpdateBits(
                            20,
                            total_block_size.wrapping_sub(1) as u32,
                            mlen_storage_ix,
                            storage,
                        );
                        continue;
                    } else {
                        if next_emit < ip_end {
                            let insert_0 = ip_end.offset_from(next_emit) as u64;
                            if (insert_0 < 6210) as i64 != 0 {
                                EmitInsertLen(
                                    insert_0,
                                    cmd_depth as *const u8,
                                    cmd_bits as *const u16,
                                    cmd_histo.as_mut_ptr(),
                                    storage_ix,
                                    storage,
                                );
                                EmitLiterals(
                                    next_emit,
                                    insert_0,
                                    lit_depth.as_mut_ptr() as *const u8,
                                    lit_bits.as_mut_ptr() as *const u16,
                                    storage_ix,
                                    storage,
                                );
                            } else if ShouldUseUncompressedMode(
                                metablock_start,
                                next_emit,
                                insert_0,
                                literal_ratio,
                            ) != 0
                            {
                                EmitUncompressedMetaBlock(
                                    metablock_start,
                                    ip_end,
                                    mlen_storage_ix.wrapping_sub(3),
                                    storage_ix,
                                    storage,
                                );
                            } else {
                                EmitLongInsertLen(
                                    insert_0,
                                    cmd_depth as *const u8,
                                    cmd_bits as *const u16,
                                    cmd_histo.as_mut_ptr(),
                                    storage_ix,
                                    storage,
                                );
                                EmitLiterals(
                                    next_emit,
                                    insert_0,
                                    lit_depth.as_mut_ptr() as *const u8,
                                    lit_bits.as_mut_ptr() as *const u16,
                                    storage_ix,
                                    storage,
                                );
                            }
                        }
                        next_emit = ip_end;
                    }
                }
                _ => {}
            }
            if !(input_size > 0) {
                break;
            }
            metablock_start = input;
            block_size = brotli_min_size_t(input_size, kFirstBlockSize);
            total_block_size = block_size;
            mlen_storage_ix = (*storage_ix).wrapping_add(3);
            BrotliStoreMetaBlockHeader(block_size, 0, storage_ix, storage);
            BrotliWriteBits(13, 0, storage_ix, storage);
            literal_ratio = BuildAndStoreLiteralPrefixCode(
                m,
                input,
                block_size,
                lit_depth.as_mut_ptr(),
                lit_bits.as_mut_ptr(),
                storage_ix,
                storage,
            );
            if 0 != 0 {
                return;
            }
            BuildAndStoreCommandPrefixCode(
                cmd_histo.as_mut_ptr() as *const u32,
                cmd_depth,
                cmd_bits,
                storage_ix,
                storage,
            );
        }
        if is_last == 0 {
            *cmd_code.offset(0 as isize) = 0;
            *cmd_code_numbits = 0;
            BuildAndStoreCommandPrefixCode(
                cmd_histo.as_mut_ptr() as *const u32,
                cmd_depth,
                cmd_bits,
                cmd_code_numbits,
                cmd_code,
            );
        }
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentFastImpl13(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut table: *mut i32,
    mut cmd_depth: *mut u8,
    mut cmd_bits: *mut u16,
    mut cmd_code_numbits: *mut u64,
    mut cmd_code: *mut u8,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        BrotliCompressFragmentFastImpl(
            m,
            input,
            input_size,
            is_last,
            table,
            13,
            cmd_depth,
            cmd_bits,
            cmd_code_numbits,
            cmd_code,
            storage_ix,
            storage,
        );
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentFastImpl11(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut table: *mut i32,
    mut cmd_depth: *mut u8,
    mut cmd_bits: *mut u16,
    mut cmd_code_numbits: *mut u64,
    mut cmd_code: *mut u8,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        BrotliCompressFragmentFastImpl(
            m,
            input,
            input_size,
            is_last,
            table,
            11,
            cmd_depth,
            cmd_bits,
            cmd_code_numbits,
            cmd_code,
            storage_ix,
            storage,
        );
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentFastImpl9(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut table: *mut i32,
    mut cmd_depth: *mut u8,
    mut cmd_bits: *mut u16,
    mut cmd_code_numbits: *mut u64,
    mut cmd_code: *mut u8,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        BrotliCompressFragmentFastImpl(
            m,
            input,
            input_size,
            is_last,
            table,
            9,
            cmd_depth,
            cmd_bits,
            cmd_code_numbits,
            cmd_code,
            storage_ix,
            storage,
        );
    }
}

#[inline(never)]
extern "C" fn BrotliCompressFragmentFastImpl15(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut table: *mut i32,
    mut cmd_depth: *mut u8,
    mut cmd_bits: *mut u16,
    mut cmd_code_numbits: *mut u64,
    mut cmd_code: *mut u8,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        BrotliCompressFragmentFastImpl(
            m,
            input,
            input_size,
            is_last,
            table,
            15,
            cmd_depth,
            cmd_bits,
            cmd_code_numbits,
            cmd_code,
            storage_ix,
            storage,
        );
    }
}

#[no_mangle]
pub extern "C" fn BrotliCompressFragmentFast(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut input_size: u64,
    mut is_last: i32,
    mut table: *mut i32,
    mut table_size: u64,
    mut cmd_depth: *mut u8,
    mut cmd_bits: *mut u16,
    mut cmd_code_numbits: *mut u64,
    mut cmd_code: *mut u8,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let initial_storage_ix = *storage_ix;
        let table_bits = Log2FloorNonZero(table_size) as u64;
        if input_size == 0 {
            BrotliWriteBits(1, 1, storage_ix, storage);
            BrotliWriteBits(1, 1, storage_ix, storage);
            *storage_ix = (*storage_ix).wrapping_add(7) & !7 as u64;
            return;
        }
        match table_bits {
            9 => {
                BrotliCompressFragmentFastImpl9(
                    m,
                    input,
                    input_size,
                    is_last,
                    table,
                    cmd_depth,
                    cmd_bits,
                    cmd_code_numbits,
                    cmd_code,
                    storage_ix,
                    storage,
                );
            }
            11 => {
                BrotliCompressFragmentFastImpl11(
                    m,
                    input,
                    input_size,
                    is_last,
                    table,
                    cmd_depth,
                    cmd_bits,
                    cmd_code_numbits,
                    cmd_code,
                    storage_ix,
                    storage,
                );
            }
            13 => {
                BrotliCompressFragmentFastImpl13(
                    m,
                    input,
                    input_size,
                    is_last,
                    table,
                    cmd_depth,
                    cmd_bits,
                    cmd_code_numbits,
                    cmd_code,
                    storage_ix,
                    storage,
                );
            }
            15 => {
                BrotliCompressFragmentFastImpl15(
                    m,
                    input,
                    input_size,
                    is_last,
                    table,
                    cmd_depth,
                    cmd_bits,
                    cmd_code_numbits,
                    cmd_code,
                    storage_ix,
                    storage,
                );
            }
            _ => {}
        }
        if (*storage_ix).wrapping_sub(initial_storage_ix) > 31u64.wrapping_add(input_size << 3) {
            EmitUncompressedMetaBlock(
                input,
                input.offset(input_size as isize),
                initial_storage_ix,
                storage_ix,
                storage,
            );
        }
        if is_last != 0 {
            BrotliWriteBits(1, 1, storage_ix, storage);
            BrotliWriteBits(1, 1, storage_ix, storage);
            *storage_ix = (*storage_ix).wrapping_add(7) & !7 as u64;
        }
    }
}
