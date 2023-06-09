use libc;
pub type ContextLut = *const u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length: [u8; 32],
    pub offsets_by_length: [u32; 32],
    pub data_size: u64,
    pub data: *const u8,
}
pub const BROTLI_MODE_FONT: u32 = 2;
pub const BROTLI_MODE_TEXT: u32 = 1;
pub const BROTLI_MODE_GENERIC: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DictWord {
    pub len: u8,
    pub transform: u8,
    pub idx: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderDictionary {
    pub words: *const BrotliDictionary,
    pub num_transforms: u32,
    pub cutoffTransformsCount: u32,
    pub cutoffTransforms: u64,
    pub hash_table_words: *const u16,
    pub hash_table_lengths: *const u8,
    pub buckets: *const u16,
    pub dict_words: *const DictWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliHasherParams {
    pub type_0: i32,
    pub bucket_bits: i32,
    pub block_bits: i32,
    pub hash_len: i32,
    pub num_last_distances_to_check: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDistanceParams {
    pub distance_postfix_bits: u32,
    pub num_direct_distance_codes: u32,
    pub alphabet_size_max: u32,
    pub alphabet_size_limit: u32,
    pub max_distance: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderParams {
    pub mode: u32,
    pub quality: i32,
    pub lgwin: i32,
    pub lgblock: i32,
    pub stream_offset: u64,
    pub size_hint: u64,
    pub disable_literal_context_modeling: i32,
    pub large_window: i32,
    pub hasher: BrotliHasherParams,
    pub dist: BrotliDistanceParams,
    pub dictionary: BrotliEncoderDictionary,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Command {
    pub insert_len_: u32,
    pub copy_len_: u32,
    pub dist_extra_: u32,
    pub cmd_prefix_: u16,
    pub dist_prefix_: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HasherCommon {
    pub extra: *mut libc::c_void,
    pub dict_num_lookups: u64,
    pub dict_num_matches: u64,
    pub params: BrotliHasherParams,
    pub is_prepared_: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HasherSearchResult {
    pub len: u64,
    pub distance: u64,
    pub score: u64,
    pub len_code_delta: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H10 {
    pub window_mask_: u64,
    pub buckets_: *mut u32,
    pub invalid_pos_: u32,
    pub forest_: *mut u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H3 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H4 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H5 {
    pub bucket_size_: u64,
    pub block_size_: u64,
    pub hash_shift_: i32,
    pub block_mask_: u32,
    pub block_bits_: i32,
    pub num_last_distances_to_check_: i32,
    pub common_: *mut HasherCommon,
    pub num_: *mut u16,
    pub buckets_: *mut u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H6 {
    pub bucket_size_: u64,
    pub block_size_: u64,
    pub hash_shift_: i32,
    pub hash_mask_: u64,
    pub block_mask_: u32,
    pub block_bits_: i32,
    pub num_last_distances_to_check_: i32,
    pub common_: *mut HasherCommon,
    pub num_: *mut u16,
    pub buckets_: *mut u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SlotH40 {
    pub delta: u16,
    pub next: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BankH40 {
    pub slots: [SlotH40; 65536],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H40 {
    pub free_slot_idx: [u16; 1],
    pub max_hops: u64,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SlotH41 {
    pub delta: u16,
    pub next: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BankH41 {
    pub slots: [SlotH41; 65536],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H41 {
    pub free_slot_idx: [u16; 1],
    pub max_hops: u64,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SlotH42 {
    pub delta: u16,
    pub next: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BankH42 {
    pub slots: [SlotH42; 512],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H42 {
    pub free_slot_idx: [u16; 512],
    pub max_hops: u64,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H54 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HROLLING_FAST {
    pub state: u32,
    pub table: *mut u32,
    pub next_ix: u64,
    pub chunk_len: u32,
    pub factor: u32,
    pub factor_remove: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HROLLING {
    pub state: u32,
    pub table: *mut u32,
    pub next_ix: u64,
    pub chunk_len: u32,
    pub factor: u32,
    pub factor_remove: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H35 {
    pub ha: H3,
    pub hb: HROLLING_FAST,
    pub hb_common: HasherCommon,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
    pub fresh: i32,
    pub params: *const BrotliEncoderParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H55 {
    pub ha: H54,
    pub hb: HROLLING_FAST,
    pub hb_common: HasherCommon,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
    pub fresh: i32,
    pub params: *const BrotliEncoderParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H65 {
    pub ha: H6,
    pub hb: HROLLING,
    pub hb_common: HasherCommon,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
    pub fresh: i32,
    pub params: *const BrotliEncoderParams,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hasher {
    pub common: HasherCommon,
    pub privat: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _H2: H2,
    pub _H3: H3,
    pub _H4: H4,
    pub _H5: H5,
    pub _H6: H6,
    pub _H40: H40,
    pub _H41: H41,
    pub _H42: H42,
    pub _H54: H54,
    pub _H35: H35,
    pub _H55: H55,
    pub _H65: H65,
    pub _H10: H10,
}
#[inline(always)]
extern "C" fn brotli_max_size_t(mut a: u64, mut b: u64) -> u64 {
    return if a > b { a } else { b };
}

#[inline(always)]
extern "C" fn brotli_min_size_t(mut a: u64, mut b: u64) -> u64 {
    return if a < b { a } else { b };
}

#[inline(always)]
extern "C" fn BrotliUnalignedRead64(mut p: *const libc::c_void) -> u64 {
    unsafe {
        return *(p as *const u64);
    }
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
extern "C" fn PrefixEncodeCopyDistance(
    mut distance_code: u64,
    mut num_direct_codes: u64,
    mut postfix_bits: u64,
    mut code: *mut u16,
    mut extra_bits: *mut u32,
) {
    unsafe {
        if distance_code < 16u64.wrapping_add(num_direct_codes) {
            *code = distance_code as u16;
            *extra_bits = 0;
            return;
        } else {
            let mut dist = (1u64 << postfix_bits.wrapping_add(2)).wrapping_add(
                distance_code
                    .wrapping_sub(16)
                    .wrapping_sub(num_direct_codes),
            );
            let mut bucket = (Log2FloorNonZero(dist)).wrapping_sub(1) as u64;
            let mut postfix_mask = (1u32 << postfix_bits).wrapping_sub(1) as u64;
            let mut postfix = dist & postfix_mask;
            let mut prefix = dist >> bucket & 1;
            let mut offset = 2u64.wrapping_add(prefix) << bucket;
            let mut nbits = bucket.wrapping_sub(postfix_bits);
            *code = (nbits << 10
                | 16u64
                    .wrapping_add(num_direct_codes)
                    .wrapping_add(
                        2u64.wrapping_mul(nbits.wrapping_sub(1u64))
                            .wrapping_add(prefix)
                            << postfix_bits,
                    )
                    .wrapping_add(postfix)) as u16;
            *extra_bits = (dist.wrapping_sub(offset) >> postfix_bits) as u32;
        };
    }
}

#[inline(always)]
extern "C" fn GetInsertLengthCode(mut insertlen: u64) -> u16 {
    if insertlen < 6 {
        return insertlen as u16;
    } else if insertlen < 130 {
        let mut nbits = (Log2FloorNonZero(insertlen.wrapping_sub(2u64))).wrapping_sub(1);
        return ((nbits << 1i32) as u64)
            .wrapping_add(insertlen.wrapping_sub(2) >> nbits)
            .wrapping_add(2) as u16;
    } else if insertlen < 2114 {
        return (Log2FloorNonZero(insertlen.wrapping_sub(66u64))).wrapping_add(10) as u16;
    } else if insertlen < 6210 {
        return 21;
    } else if insertlen < 22594 {
        return 22;
    } else {
        return 23;
    };
}

#[inline(always)]
extern "C" fn GetCopyLengthCode(mut copylen: u64) -> u16 {
    if copylen < 10 {
        return copylen.wrapping_sub(2) as u16;
    } else if copylen < 134 {
        let mut nbits = (Log2FloorNonZero(copylen.wrapping_sub(6u64))).wrapping_sub(1);
        return ((nbits << 1i32) as u64)
            .wrapping_add(copylen.wrapping_sub(6) >> nbits)
            .wrapping_add(4) as u16;
    } else if copylen < 2118 {
        return (Log2FloorNonZero(copylen.wrapping_sub(70u64))).wrapping_add(12) as u16;
    } else {
        return 23;
    };
}

#[inline(always)]
extern "C" fn CombineLengthCodes(
    mut inscode: u16,
    mut copycode: u16,
    mut use_last_distance: i32,
) -> u16 {
    let mut bits64 = (copycode as u32 & 0x7 | (inscode as u32 & 0x7u32) << 3) as u16;
    unsafe {
        if use_last_distance != 0 && (inscode as u32) < 8 && (copycode as u32) < 16 {
            return (if (copycode as u32) < 8u32 {
                bits64 as u32
            } else {
                bits64 as u32 | 64
            }) as u16;
        } else {
            let mut offset = 2u32.wrapping_mul(
                ((copycode as i32 >> 3u32) as u32)
                    .wrapping_add(3u32.wrapping_mul((inscode as i32 >> 3u32) as u32)),
            );
            offset = (offset << 5u32)
                .wrapping_add(0x40)
                .wrapping_add(0x520d40 >> offset & 0xc0);
            return (offset | bits64 as u32) as u16;
        };
    }
}

#[inline(always)]
extern "C" fn GetLengthCode(
    mut insertlen: u64,
    mut copylen: u64,
    mut use_last_distance: i32,
    mut code: *mut u16,
) {
    unsafe {
        let mut inscode = GetInsertLengthCode(insertlen);
        let mut copycode = GetCopyLengthCode(copylen);
        *code = CombineLengthCodes(inscode, copycode, use_last_distance);
    }
}

#[inline(always)]
extern "C" fn InitCommand(
    mut self_0: *mut Command,
    mut dist: *const BrotliDistanceParams,
    mut insertlen: u64,
    mut copylen: u64,
    mut copylen_code_delta: i32,
    mut distance_code: u64,
) {
    unsafe {
        let mut delta = copylen_code_delta as u32;
        (*self_0).insert_len_ = insertlen as u32;
        (*self_0).copy_len_ = (copylen | (delta << 25i32) as u64) as u32;
        PrefixEncodeCopyDistance(
            distance_code,
            (*dist).num_direct_distance_codes as u64,
            (*dist).distance_postfix_bits as u64,
            &mut (*self_0).dist_prefix_,
            &mut (*self_0).dist_extra_,
        );
        GetLengthCode(
            insertlen,
            (copylen as i32 + copylen_code_delta) as u64,
            if (*self_0).dist_prefix_ as i32 & 0x3ff == 0 {
                1
            } else {
                0
            },
            &mut (*self_0).cmd_prefix_,
        );
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
extern "C" fn LiteralSpreeLengthForSparseSearch(mut params: *const BrotliEncoderParams) -> u64 {
    unsafe {
        return (if (*params).quality < 9 { 64 } else { 512 }) as u64;
    }
}

static mut kHashMul32: u32 = 0x1e35a7bd;
static mut kHashMul64: u64 = 0x1e35a7bd << 32 | 0x1e35a7bd;
static mut kHashMul64Long: u64 = 0x1fe35a7b << 32 | 0xd3579bd3;
#[inline(always)]
extern "C" fn Hash14(mut data: *const u8) -> u32 {
    unsafe {
        let mut h = (BrotliUnalignedRead32(data as *const libc::c_void)).wrapping_mul(kHashMul32);
        return h >> 32 - 14;
    }
}

#[inline(always)]
extern "C" fn PrepareDistanceCache(mut distance_cache: *mut i32, num_distances: i32) {
    unsafe {
        if num_distances > 4 {
            let mut last_distance = *distance_cache.offset(0 as isize);
            *distance_cache.offset(4 as isize) = last_distance - 1;
            *distance_cache.offset(5 as isize) = last_distance + 1;
            *distance_cache.offset(6 as isize) = last_distance - 2;
            *distance_cache.offset(7 as isize) = last_distance + 2;
            *distance_cache.offset(8 as isize) = last_distance - 3;
            *distance_cache.offset(9 as isize) = last_distance + 3;
            if num_distances > 10 {
                let mut next_last_distance = *distance_cache.offset(1 as isize);
                *distance_cache.offset(10 as isize) = next_last_distance - 1;
                *distance_cache.offset(11 as isize) = next_last_distance + 1;
                *distance_cache.offset(12 as isize) = next_last_distance - 2;
                *distance_cache.offset(13 as isize) = next_last_distance + 2;
                *distance_cache.offset(14 as isize) = next_last_distance - 3;
                *distance_cache.offset(15 as isize) = next_last_distance + 3;
            }
        }
    }
}

#[inline(always)]
extern "C" fn BackwardReferenceScore(
    mut copy_length: u64,
    mut backward_reference_offset: u64,
) -> u64 {
    return ((30 * 8i32) as u64)
        .wrapping_mul(::std::mem::size_of::<u64>() as u64)
        .wrapping_add(135u64.wrapping_mul(copy_length))
        .wrapping_sub(30u32.wrapping_mul(Log2FloorNonZero(backward_reference_offset)) as u64);
}

#[inline(always)]
extern "C" fn BackwardReferenceScoreUsingLastDistance(mut copy_length: u64) -> u64 {
    return 135u64
        .wrapping_mul(copy_length)
        .wrapping_add(((30 * 8i32) as u64).wrapping_mul(::std::mem::size_of::<u64>() as u64))
        .wrapping_add(15);
}

#[inline(always)]
extern "C" fn BackwardReferencePenaltyUsingLastDistance(mut distance_short_code: u64) -> u64 {
    return 39u64.wrapping_add((0x1ca10 >> (distance_short_code & 0xeu64) & 0xe) as u64);
}

#[inline(always)]
extern "C" fn TestStaticDictionaryItem(
    mut dictionary: *const BrotliEncoderDictionary,
    mut len: u64,
    mut word_idx: u64,
    mut data: *const u8,
    mut max_length: u64,
    mut max_backward: u64,
    mut max_distance: u64,
    mut out: *mut HasherSearchResult,
) -> i32 {
    unsafe {
        let mut offset: u64 = 0;
        let mut matchlen: u64 = 0;
        let mut backward: u64 = 0;
        let mut score: u64 = 0;
        offset = ((*(*dictionary).words).offsets_by_length[len as usize] as u64)
            .wrapping_add(len.wrapping_mul(word_idx));
        if len > max_length {
            return 0;
        }
        matchlen = FindMatchLengthWithLimit(
            data,
            &*((*(*dictionary).words).data).offset(offset as isize),
            len,
        );
        if matchlen.wrapping_add((*dictionary).cutoffTransformsCount as u64) <= len || matchlen == 0
        {
            return 0;
        }
        let mut cut = len.wrapping_sub(matchlen);
        let mut transform_id = (cut << 2i32)
            .wrapping_add((*dictionary).cutoffTransforms >> cut.wrapping_mul(6) & 0x3f);
        backward = max_backward
            .wrapping_add(1)
            .wrapping_add(word_idx)
            .wrapping_add(
                transform_id << (*(*dictionary).words).size_bits_by_length[len as usize] as i32,
            );
        if backward > max_distance {
            return 0;
        }
        score = BackwardReferenceScore(matchlen, backward);
        if score < (*out).score {
            return 0;
        };
        (*out).len = matchlen;
        (*out).len_code_delta = len as i32 - matchlen as i32;
        (*out).distance = backward;
        (*out).score = score;
        return 1;
    }
}

#[inline(always)]
extern "C" fn SearchInStaticDictionary(
    mut dictionary: *const BrotliEncoderDictionary,
    mut common: *mut HasherCommon,
    mut data: *const u8,
    mut max_length: u64,
    mut max_backward: u64,
    mut max_distance: u64,
    mut out: *mut HasherSearchResult,
    mut shallow: i32,
) {
    unsafe {
        let mut key: u64 = 0;
        let mut i: u64 = 0;
        if (*common).dict_num_matches < (*common).dict_num_lookups >> 7 {
            return;
        }
        key = (Hash14(data) << 1i32) as u64;
        i = 0;
        while i < (if shallow != 0 { 1 } else { 2 }) as u64 {
            let ref mut fresh0 = (*common).dict_num_lookups;
            *fresh0 = (*fresh0).wrapping_add(1);
            if *((*dictionary).hash_table_lengths).offset(key as isize) as i32 != 0 {
                let mut item_matches = TestStaticDictionaryItem(
                    dictionary,
                    *((*dictionary).hash_table_lengths).offset(key as isize) as u64,
                    *((*dictionary).hash_table_words).offset(key as isize) as u64,
                    data,
                    max_length,
                    max_backward,
                    max_distance,
                    out,
                );
                if item_matches != 0 {
                    let ref mut fresh1 = (*common).dict_num_matches;
                    *fresh1 = (*fresh1).wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
            key = key.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH2() -> u64 {
    return 8;
}

#[inline(always)]
extern "C" fn StoreLookaheadH2() -> u64 {
    return 8;
}

extern "C" fn HashBytesH2(mut data: *const u8) -> u32 {
    unsafe {
        let h = (BrotliUnalignedRead64(data as *const libc::c_void) << 64 - 8 * 5i32)
            .wrapping_mul(kHashMul64);
        return (h >> 64 - 16i32) as u32;
    }
}

#[inline(always)]
extern "C" fn StoreH2(mut self_0: *mut H2, mut data: *const u8, mask: u64, ix: u64) {
    unsafe {
        let key = HashBytesH2(&*data.offset((ix & mask) as isize));
        if 1 << 0 == 1 {
            *((*self_0).buckets_).offset(key as isize) = ix as u32;
        } else {
            let off = (ix & (((1i32 << 0i32) - 1) << 3) as u64) as u32;
            *((*self_0).buckets_)
                .offset((key.wrapping_add(off) & ((1i32 << 16i32) - 1i32) as u32) as isize) =
                ix as u32;
        };
    }
}

#[inline(always)]
extern "C" fn StoreRangeH2(
    mut self_0: *mut H2,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
    unsafe {
        let mut i: u64 = 0;
        i = ix_start;
        while i < ix_end {
            StoreH2(self_0, data, mask, i);
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheH2(mut self_0: *mut H2, mut distance_cache: *mut i32) {}

#[inline(always)]
extern "C" fn FindLongestMatchH2(
    mut self_0: *mut H2,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        let mut buckets = (*self_0).buckets_;
        let best_len_in = (*out).len;
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let mut compare_char =
            *data.offset(cur_ix_masked.wrapping_add(best_len_in) as isize) as i32;
        let mut key = HashBytesH2(&*data.offset(cur_ix_masked as isize)) as u64;
        let mut key_out: u64 = 0;
        let mut min_score = (*out).score;
        let mut best_score = (*out).score;
        let mut best_len = best_len_in;
        let mut cached_backward = *distance_cache.offset(0 as isize) as u64;
        let mut prev_ix = cur_ix.wrapping_sub(cached_backward);
        (*out).len_code_delta = 0;
        if prev_ix < cur_ix {
            prev_ix &= ring_buffer_mask as u64;
            if compare_char == *data.offset(prev_ix.wrapping_add(best_len) as isize) as i32 {
                let len = FindMatchLengthWithLimit(
                    &*data.offset(prev_ix as isize),
                    &*data.offset(cur_ix_masked as isize),
                    max_length,
                );
                if len >= 4 {
                    let score = BackwardReferenceScoreUsingLastDistance(len);
                    if best_score < score {
                        (*out).len = len;
                        (*out).distance = cached_backward;
                        (*out).score = score;
                        if 1 << 0 == 1 {
                            *buckets.offset(key as isize) = cur_ix as u32;
                            return;
                        } else {
                            best_len = len;
                            best_score = score;
                            compare_char =
                                *data.offset(cur_ix_masked.wrapping_add(len) as isize) as i32;
                        }
                    }
                }
            }
        }
        if 1 << 0 == 1 {
            let mut backward: u64 = 0;
            let mut len_0: u64 = 0;
            prev_ix = *buckets.offset(key as isize) as u64;
            *buckets.offset(key as isize) = cur_ix as u32;
            backward = cur_ix.wrapping_sub(prev_ix);
            prev_ix &= ring_buffer_mask as u64;
            if compare_char != *data.offset(prev_ix.wrapping_add(best_len_in) as isize) as i32 {
                return;
            }
            if (backward == 0 || backward > max_backward) as i64 != 0 {
                return;
            }
            len_0 = FindMatchLengthWithLimit(
                &*data.offset(prev_ix as isize),
                &*data.offset(cur_ix_masked as isize),
                max_length,
            );
            if len_0 >= 4 {
                let score_0 = BackwardReferenceScore(len_0, backward);
                if best_score < score_0 {
                    (*out).len = len_0;
                    (*out).distance = backward;
                    (*out).score = score_0;
                    return;
                }
            }
        }
        if 1 != 0 && min_score == (*out).score {
            SearchInStaticDictionary(
                dictionary,
                (*self_0).common,
                &*data.offset(cur_ix_masked as isize),
                max_length,
                dictionary_distance,
                max_distance,
                out,
                1,
            );
        }
        if 1 << 0 != 1 {
            *buckets.offset(key_out as isize) = cur_ix as u32;
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH3() -> u64 {
    return 8;
}

#[inline(always)]
extern "C" fn StoreLookaheadH3() -> u64 {
    return 8;
}

extern "C" fn HashBytesH3(mut data: *const u8) -> u32 {
    unsafe {
        let h = (BrotliUnalignedRead64(data as *const libc::c_void) << 64 - 8 * 5i32)
            .wrapping_mul(kHashMul64);
        return (h >> 64 - 16i32) as u32;
    }
}

#[inline(always)]
extern "C" fn StoreH3(mut self_0: *mut H3, mut data: *const u8, mask: u64, ix: u64) {
    unsafe {
        let key = HashBytesH3(&*data.offset((ix & mask) as isize));
        if 1 << 1 == 1 {
            *((*self_0).buckets_).offset(key as isize) = ix as u32;
        } else {
            let off = (ix & (((1i32 << 1i32) - 1) << 3) as u64) as u32;
            *((*self_0).buckets_)
                .offset((key.wrapping_add(off) & ((1i32 << 16i32) - 1i32) as u32) as isize) =
                ix as u32;
        };
    }
}

#[inline(always)]
extern "C" fn StoreRangeH3(
    mut self_0: *mut H3,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
    unsafe {
        let mut i: u64 = 0;
        i = ix_start;
        while i < ix_end {
            StoreH3(self_0, data, mask, i);
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheH3(mut self_0: *mut H3, mut distance_cache: *mut i32) {}

#[inline(always)]
extern "C" fn FindLongestMatchH3(
    mut self_0: *mut H3,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        let mut buckets = (*self_0).buckets_;
        let best_len_in = (*out).len;
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let mut compare_char =
            *data.offset(cur_ix_masked.wrapping_add(best_len_in) as isize) as i32;
        let mut key = HashBytesH3(&*data.offset(cur_ix_masked as isize)) as u64;
        let mut key_out: u64 = 0;
        let mut min_score = (*out).score;
        let mut best_score = (*out).score;
        let mut best_len = best_len_in;
        let mut cached_backward = *distance_cache.offset(0 as isize) as u64;
        let mut prev_ix = cur_ix.wrapping_sub(cached_backward);
        (*out).len_code_delta = 0;
        if prev_ix < cur_ix {
            prev_ix &= ring_buffer_mask as u64;
            if compare_char == *data.offset(prev_ix.wrapping_add(best_len) as isize) as i32 {
                let len = FindMatchLengthWithLimit(
                    &*data.offset(prev_ix as isize),
                    &*data.offset(cur_ix_masked as isize),
                    max_length,
                );
                if len >= 4 {
                    let score = BackwardReferenceScoreUsingLastDistance(len);
                    if best_score < score {
                        (*out).len = len;
                        (*out).distance = cached_backward;
                        (*out).score = score;
                        if 1 << 1 == 1 {
                            *buckets.offset(key as isize) = cur_ix as u32;
                            return;
                        } else {
                            best_len = len;
                            best_score = score;
                            compare_char =
                                *data.offset(cur_ix_masked.wrapping_add(len) as isize) as i32;
                        }
                    }
                }
            }
        }
        {
            let mut keys: [u64; 2] = [0; 2];
            let mut i: u64 = 0;
            i = 0;
            while i < (1i32 << 1) as u64 {
                keys[i as usize] = key.wrapping_add(i << 3) & ((1i32 << 16) - 1) as u64;
                i = i.wrapping_add(1);
            }
            key_out = keys[((cur_ix & (((1i32 << 1i32) - 1i32) << 3) as u64) >> 3) as usize];
            i = 0;
            while i < (1i32 << 1) as u64 {
                let mut len_1: u64 = 0;
                let mut backward_0: u64 = 0;
                prev_ix = *buckets.offset(keys[i as usize] as isize) as u64;
                backward_0 = cur_ix.wrapping_sub(prev_ix);
                prev_ix &= ring_buffer_mask as u64;
                if !(compare_char != *data.offset(prev_ix.wrapping_add(best_len) as isize) as i32) {
                    if !((backward_0 == 0 || backward_0 > max_backward) as i64 != 0) {
                        len_1 = FindMatchLengthWithLimit(
                            &*data.offset(prev_ix as isize),
                            &*data.offset(cur_ix_masked as isize),
                            max_length,
                        );
                        if len_1 >= 4 {
                            let score_1 = BackwardReferenceScore(len_1, backward_0);
                            if best_score < score_1 {
                                best_len = len_1;
                                (*out).len = len_1;
                                compare_char =
                                    *data.offset(cur_ix_masked.wrapping_add(len_1) as isize) as i32;
                                best_score = score_1;
                                (*out).score = score_1;
                                (*out).distance = backward_0;
                            }
                        }
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        if 1 << 1 != 1 {
            *buckets.offset(key_out as isize) = cur_ix as u32;
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH4() -> u64 {
    return 8;
}

#[inline(always)]
extern "C" fn StoreLookaheadH4() -> u64 {
    return 8;
}

extern "C" fn HashBytesH4(mut data: *const u8) -> u32 {
    unsafe {
        let h = (BrotliUnalignedRead64(data as *const libc::c_void) << 64 - 8 * 5i32)
            .wrapping_mul(kHashMul64);
        return (h >> 64 - 17i32) as u32;
    }
}

#[inline(always)]
extern "C" fn StoreH4(mut self_0: *mut H4, mut data: *const u8, mask: u64, ix: u64) {
    unsafe {
        let key = HashBytesH4(&*data.offset((ix & mask) as isize));
        if 1 << 2 == 1 {
            *((*self_0).buckets_).offset(key as isize) = ix as u32;
        } else {
            let off = (ix & (((1i32 << 2i32) - 1) << 3) as u64) as u32;
            *((*self_0).buckets_)
                .offset((key.wrapping_add(off) & ((1i32 << 17i32) - 1i32) as u32) as isize) =
                ix as u32;
        };
    }
}

#[inline(always)]
extern "C" fn StoreRangeH4(
    mut self_0: *mut H4,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
    unsafe {
        let mut i: u64 = 0;
        i = ix_start;
        while i < ix_end {
            StoreH4(self_0, data, mask, i);
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheH4(mut self_0: *mut H4, mut distance_cache: *mut i32) {}

#[inline(always)]
extern "C" fn FindLongestMatchH4(
    mut self_0: *mut H4,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        let mut buckets = (*self_0).buckets_;
        let best_len_in = (*out).len;
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let mut compare_char =
            *data.offset(cur_ix_masked.wrapping_add(best_len_in) as isize) as i32;
        let mut key = HashBytesH4(&*data.offset(cur_ix_masked as isize)) as u64;
        let mut key_out: u64 = 0;
        let mut min_score = (*out).score;
        let mut best_score = (*out).score;
        let mut best_len = best_len_in;
        let mut cached_backward = *distance_cache.offset(0 as isize) as u64;
        let mut prev_ix = cur_ix.wrapping_sub(cached_backward);
        (*out).len_code_delta = 0;
        if prev_ix < cur_ix {
            prev_ix &= ring_buffer_mask as u64;
            if compare_char == *data.offset(prev_ix.wrapping_add(best_len) as isize) as i32 {
                let len = FindMatchLengthWithLimit(
                    &*data.offset(prev_ix as isize),
                    &*data.offset(cur_ix_masked as isize),
                    max_length,
                );
                if len >= 4 {
                    let score = BackwardReferenceScoreUsingLastDistance(len);
                    if best_score < score {
                        (*out).len = len;
                        (*out).distance = cached_backward;
                        (*out).score = score;
                        if 1 << 2 == 1 {
                            *buckets.offset(key as isize) = cur_ix as u32;
                            return;
                        } else {
                            best_len = len;
                            best_score = score;
                            compare_char =
                                *data.offset(cur_ix_masked.wrapping_add(len) as isize) as i32;
                        }
                    }
                }
            }
        }
        {
            let mut keys: [u64; 4] = [0; 4];
            let mut i: u64 = 0;
            i = 0;
            while i < (1i32 << 2) as u64 {
                keys[i as usize] = key.wrapping_add(i << 3) & ((1i32 << 17) - 1) as u64;
                i = i.wrapping_add(1);
            }
            key_out = keys[((cur_ix & (((1i32 << 2i32) - 1i32) << 3) as u64) >> 3) as usize];
            i = 0;
            while i < (1i32 << 2) as u64 {
                let mut len_1: u64 = 0;
                let mut backward_0: u64 = 0;
                prev_ix = *buckets.offset(keys[i as usize] as isize) as u64;
                backward_0 = cur_ix.wrapping_sub(prev_ix);
                prev_ix &= ring_buffer_mask as u64;
                if !(compare_char != *data.offset(prev_ix.wrapping_add(best_len) as isize) as i32) {
                    if !((backward_0 == 0 || backward_0 > max_backward) as i64 != 0) {
                        len_1 = FindMatchLengthWithLimit(
                            &*data.offset(prev_ix as isize),
                            &*data.offset(cur_ix_masked as isize),
                            max_length,
                        );
                        if len_1 >= 4 {
                            let score_1 = BackwardReferenceScore(len_1, backward_0);
                            if best_score < score_1 {
                                best_len = len_1;
                                (*out).len = len_1;
                                compare_char =
                                    *data.offset(cur_ix_masked.wrapping_add(len_1) as isize) as i32;
                                best_score = score_1;
                                (*out).score = score_1;
                                (*out).distance = backward_0;
                            }
                        }
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        if 1 != 0 && min_score == (*out).score {
            SearchInStaticDictionary(
                dictionary,
                (*self_0).common,
                &*data.offset(cur_ix_masked as isize),
                max_length,
                dictionary_distance,
                max_distance,
                out,
                1,
            );
        }
        if 1 << 2 != 1 {
            *buckets.offset(key_out as isize) = cur_ix as u32;
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH5() -> u64 {
    return 4;
}

#[inline(always)]
extern "C" fn StoreLookaheadH5() -> u64 {
    return 4;
}

extern "C" fn HashBytesH5(mut data: *const u8, shift: i32) -> u32 {
    unsafe {
        let mut h = (BrotliUnalignedRead32(data as *const libc::c_void)).wrapping_mul(kHashMul32);
        return h >> shift;
    }
}

#[inline(always)]
extern "C" fn StoreH5(mut self_0: *mut H5, mut data: *const u8, mask: u64, ix: u64) {
    unsafe {
        let key = HashBytesH5(&*data.offset((ix & mask) as isize), (*self_0).hash_shift_);
        let minor_ix =
            (*((*self_0).num_).offset(key as isize) as u32 & (*self_0).block_mask_) as u64;
        let offset = minor_ix.wrapping_add((key << (*self_0).block_bits_) as u64);
        *((*self_0).buckets_).offset(offset as isize) = ix as u32;
        let ref mut fresh2 = *((*self_0).num_).offset(key as isize);
        *fresh2 = (*fresh2).wrapping_add(1);
    }
}

#[inline(always)]
extern "C" fn StoreRangeH5(
    mut self_0: *mut H5,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
    unsafe {
        let mut i: u64 = 0;
        i = ix_start;
        while i < ix_end {
            StoreH5(self_0, data, mask, i);
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheH5(mut self_0: *mut H5, mut distance_cache: *mut i32) {
    unsafe {
        PrepareDistanceCache(distance_cache, (*self_0).num_last_distances_to_check_);
    }
}

#[inline(always)]
extern "C" fn FindLongestMatchH5(
    mut self_0: *mut H5,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        let mut num = (*self_0).num_;
        let mut buckets = (*self_0).buckets_;
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let mut min_score = (*out).score;
        let mut best_score = (*out).score;
        let mut best_len = (*out).len;
        let mut i: u64 = 0;
        (*out).len = 0;
        (*out).len_code_delta = 0;
        i = 0;
        while i < (*self_0).num_last_distances_to_check_ as u64 {
            let backward = *distance_cache.offset(i as isize) as u64;
            let mut prev_ix = cur_ix.wrapping_sub(backward);
            if !(prev_ix >= cur_ix) {
                if !((backward > max_backward) as i64 != 0) {
                    prev_ix &= ring_buffer_mask;
                    if !(cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
                        || prev_ix.wrapping_add(best_len) > ring_buffer_mask
                        || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as i32
                            != *data.offset(prev_ix.wrapping_add(best_len) as isize) as i32)
                    {
                        let len = FindMatchLengthWithLimit(
                            &*data.offset(prev_ix as isize),
                            &*data.offset(cur_ix_masked as isize),
                            max_length,
                        );
                        if len >= 3 || len == 2 && i < 2 {
                            let mut score = BackwardReferenceScoreUsingLastDistance(len);
                            if best_score < score {
                                if i != 0 {
                                    score = (score as u64)
                                        .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i))
                                        as u64;
                                }
                                if best_score < score {
                                    best_score = score;
                                    best_len = len;
                                    (*out).len = best_len;
                                    (*out).distance = backward;
                                    (*out).score = best_score;
                                }
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        let key = HashBytesH5(&*data.offset(cur_ix_masked as isize), (*self_0).hash_shift_);
        let mut bucket: *mut u32 =
            &mut *buckets.offset((key << (*self_0).block_bits_) as isize) as *mut u32;
        let down = if *num.offset(key as isize) as u64 > (*self_0).block_size_ {
            (*num.offset(key as isize) as u64).wrapping_sub((*self_0).block_size_)
        } else {
            0
        };
        i = *num.offset(key as isize) as u64;
        while i > down {
            i = i.wrapping_sub(1);
            let mut prev_ix_0 = *bucket.offset((i & (*self_0).block_mask_ as u64) as isize) as u64;
            let backward_0 = cur_ix.wrapping_sub(prev_ix_0);
            if (backward_0 > max_backward) as i64 != 0 {
                break;
            }
            prev_ix_0 &= ring_buffer_mask;
            if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
                || prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
                || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as i32
                    != *data.offset(prev_ix_0.wrapping_add(best_len) as isize) as i32
            {
                continue;
            }
            let len_0 = FindMatchLengthWithLimit(
                &*data.offset(prev_ix_0 as isize),
                &*data.offset(cur_ix_masked as isize),
                max_length,
            );
            if len_0 >= 4 {
                let mut score_0 = BackwardReferenceScore(len_0, backward_0);
                if best_score < score_0 {
                    best_score = score_0;
                    best_len = len_0;
                    (*out).len = best_len;
                    (*out).distance = backward_0;
                    (*out).score = best_score;
                }
            }
        }
        *bucket.offset((*num.offset(key as isize) as u32 & (*self_0).block_mask_) as isize) =
            cur_ix as u32;
        let ref mut fresh3 = *num.offset(key as isize);
        *fresh3 = (*fresh3).wrapping_add(1);
        if min_score == (*out).score {
            SearchInStaticDictionary(
                dictionary,
                (*self_0).common_,
                &*data.offset(cur_ix_masked as isize),
                max_length,
                dictionary_distance,
                max_distance,
                out,
                0,
            );
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH6() -> u64 {
    return 8;
}

#[inline(always)]
extern "C" fn StoreLookaheadH6() -> u64 {
    return 8;
}

#[inline(always)]
extern "C" fn HashBytesH6(mut data: *const u8, mask: u64, shift: i32) -> u32 {
    unsafe {
        let h = (BrotliUnalignedRead64(data as *const libc::c_void) & mask)
            .wrapping_mul(kHashMul64Long);
        return (h >> shift) as u32;
    }
}

#[inline(always)]
extern "C" fn StoreH6(mut self_0: *mut H6, mut data: *const u8, mask: u64, ix: u64) {
    unsafe {
        let mut num = (*self_0).num_;
        let mut buckets = (*self_0).buckets_;
        let key = HashBytesH6(
            &*data.offset((ix & mask) as isize),
            (*self_0).hash_mask_,
            (*self_0).hash_shift_,
        );
        let minor_ix = (*num.offset(key as isize) as u32 & (*self_0).block_mask_) as u64;
        let offset = minor_ix.wrapping_add((key << (*self_0).block_bits_) as u64);
        let ref mut fresh4 = *num.offset(key as isize);
        *fresh4 = (*fresh4).wrapping_add(1);
        *buckets.offset(offset as isize) = ix as u32;
    }
}

#[inline(always)]
extern "C" fn StoreRangeH6(
    mut self_0: *mut H6,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
    unsafe {
        let mut i: u64 = 0;
        i = ix_start;
        while i < ix_end {
            StoreH6(self_0, data, mask, i);
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheH6(mut self_0: *mut H6, mut distance_cache: *mut i32) {
    unsafe {
        PrepareDistanceCache(distance_cache, (*self_0).num_last_distances_to_check_);
    }
}

#[inline(always)]
extern "C" fn FindLongestMatchH6(
    mut self_0: *mut H6,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        let mut num = (*self_0).num_;
        let mut buckets = (*self_0).buckets_;
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let mut min_score = (*out).score;
        let mut best_score = (*out).score;
        let mut best_len = (*out).len;
        let mut i: u64 = 0;
        (*out).len = 0;
        (*out).len_code_delta = 0;
        i = 0;
        while i < (*self_0).num_last_distances_to_check_ as u64 {
            let backward = *distance_cache.offset(i as isize) as u64;
            let mut prev_ix = cur_ix.wrapping_sub(backward);
            if !(prev_ix >= cur_ix) {
                if !((backward > max_backward) as i64 != 0) {
                    prev_ix &= ring_buffer_mask;
                    if !(cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
                        || prev_ix.wrapping_add(best_len) > ring_buffer_mask
                        || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as i32
                            != *data.offset(prev_ix.wrapping_add(best_len) as isize) as i32)
                    {
                        let len = FindMatchLengthWithLimit(
                            &*data.offset(prev_ix as isize),
                            &*data.offset(cur_ix_masked as isize),
                            max_length,
                        );
                        if len >= 3 || len == 2 && i < 2 {
                            let mut score = BackwardReferenceScoreUsingLastDistance(len);
                            if best_score < score {
                                if i != 0 {
                                    score = (score as u64)
                                        .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i))
                                        as u64;
                                }
                                if best_score < score {
                                    best_score = score;
                                    best_len = len;
                                    (*out).len = best_len;
                                    (*out).distance = backward;
                                    (*out).score = best_score;
                                }
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        let key = HashBytesH6(
            &*data.offset(cur_ix_masked as isize),
            (*self_0).hash_mask_,
            (*self_0).hash_shift_,
        );
        let mut bucket: *mut u32 =
            &mut *buckets.offset((key << (*self_0).block_bits_) as isize) as *mut u32;
        let down = if *num.offset(key as isize) as u64 > (*self_0).block_size_ {
            (*num.offset(key as isize) as u64).wrapping_sub((*self_0).block_size_)
        } else {
            0
        };
        i = *num.offset(key as isize) as u64;
        while i > down {
            i = i.wrapping_sub(1);
            let mut prev_ix_0 = *bucket.offset((i & (*self_0).block_mask_ as u64) as isize) as u64;
            let backward_0 = cur_ix.wrapping_sub(prev_ix_0);
            if (backward_0 > max_backward) as i64 != 0 {
                break;
            }
            prev_ix_0 &= ring_buffer_mask;
            if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
                || prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
                || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as i32
                    != *data.offset(prev_ix_0.wrapping_add(best_len) as isize) as i32
            {
                continue;
            }
            let len_0 = FindMatchLengthWithLimit(
                &*data.offset(prev_ix_0 as isize),
                &*data.offset(cur_ix_masked as isize),
                max_length,
            );
            if len_0 >= 4 {
                let mut score_0 = BackwardReferenceScore(len_0, backward_0);
                if best_score < score_0 {
                    best_score = score_0;
                    best_len = len_0;
                    (*out).len = best_len;
                    (*out).distance = backward_0;
                    (*out).score = best_score;
                }
            }
        }
        *bucket.offset((*num.offset(key as isize) as u32 & (*self_0).block_mask_) as isize) =
            cur_ix as u32;
        let ref mut fresh5 = *num.offset(key as isize);
        *fresh5 = (*fresh5).wrapping_add(1);
        if min_score == (*out).score {
            SearchInStaticDictionary(
                dictionary,
                (*self_0).common_,
                &*data.offset(cur_ix_masked as isize),
                max_length,
                dictionary_distance,
                max_distance,
                out,
                0,
            );
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH40() -> u64 {
    return 4;
}

#[inline(always)]
extern "C" fn StoreLookaheadH40() -> u64 {
    return 4;
}

#[inline(always)]
extern "C" fn HashBytesH40(mut data: *const u8) -> u64 {
    unsafe {
        let h = (BrotliUnalignedRead32(data as *const libc::c_void)).wrapping_mul(kHashMul32);
        return (h >> 32 - 15i32) as u64;
    }
}

extern "C" fn AddrH40(mut extra: *mut libc::c_void) -> *mut u32 {
    unsafe {
        return extra as *mut u32;
    }
}

extern "C" fn HeadH40(mut extra: *mut libc::c_void) -> *mut u16 {
    unsafe {
        return &mut *((AddrH40 as unsafe extern "C" fn(*mut libc::c_void) -> *mut u32)(extra))
            .offset((1i32 << 15i32) as isize) as *mut u32 as *mut u16;
    }
}

extern "C" fn TinyHashH40(mut extra: *mut libc::c_void) -> *mut u8 {
    unsafe {
        return &mut *((HeadH40 as unsafe extern "C" fn(*mut libc::c_void) -> *mut u16)(extra))
            .offset((1i32 << 15i32) as isize) as *mut u16 as *mut u8;
    }
}

extern "C" fn BanksH40(mut extra: *mut libc::c_void) -> *mut BankH40 {
    unsafe {
        return &mut *((TinyHashH40 as unsafe extern "C" fn(*mut libc::c_void) -> *mut u8)(extra))
            .offset(65536 as isize) as *mut u8 as *mut BankH40;
    }
}

#[inline(always)]
extern "C" fn StoreH40(mut self_0: *mut H40, mut data: *const u8, mask: u64, ix: u64) {
    unsafe {
        let mut addr = AddrH40((*self_0).extra);
        let mut head = HeadH40((*self_0).extra);
        let mut tiny_hash = TinyHashH40((*self_0).extra);
        let mut banks = BanksH40((*self_0).extra);
        let key = HashBytesH40(&*data.offset((ix & mask) as isize));
        let bank = key & (1 - 1i32) as u64;
        let ref mut fresh6 = (*self_0).free_slot_idx[bank as usize];
        let fresh7 = *fresh6;
        *fresh6 = (*fresh6).wrapping_add(1);
        let idx = (fresh7 as i32 & (1i32 << 16) - 1) as u64;
        let mut delta = ix.wrapping_sub(*addr.offset(key as isize) as u64);
        *tiny_hash.offset(ix as u16 as isize) = key as u8;
        if delta > 0xffff {
            delta = (if 0 != 0 { 0 } else { 0xffff }) as u64;
        };
        (*banks.offset(bank as isize)).slots[idx as usize].delta = delta as u16;
        (*banks.offset(bank as isize)).slots[idx as usize].next = *head.offset(key as isize);
        *addr.offset(key as isize) = ix as u32;
        *head.offset(key as isize) = idx as u16;
    }
}

#[inline(always)]
extern "C" fn StoreRangeH40(
    mut self_0: *mut H40,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
    unsafe {
        let mut i: u64 = 0;
        i = ix_start;
        while i < ix_end {
            StoreH40(self_0, data, mask, i);
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheH40(mut self_0: *mut H40, mut distance_cache: *mut i32) {
    unsafe {
        PrepareDistanceCache(distance_cache, 4);
    }
}

#[inline(always)]
extern "C" fn FindLongestMatchH40(
    mut self_0: *mut H40,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        let mut addr = AddrH40((*self_0).extra);
        let mut head = HeadH40((*self_0).extra);
        let mut tiny_hashes = TinyHashH40((*self_0).extra);
        let mut banks = BanksH40((*self_0).extra);
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let mut min_score = (*out).score;
        let mut best_score = (*out).score;
        let mut best_len = (*out).len;
        let mut i: u64 = 0;
        let key = HashBytesH40(&*data.offset(cur_ix_masked as isize));
        let tiny_hash = key as u8;
        (*out).len = 0;
        (*out).len_code_delta = 0;
        i = 0;
        while i < 4 {
            let backward = *distance_cache.offset(i as isize) as u64;
            let mut prev_ix = cur_ix.wrapping_sub(backward);
            if !(i > 0 && *tiny_hashes.offset(prev_ix as u16 as isize) as i32 != tiny_hash as i32) {
                if !(prev_ix >= cur_ix || backward > max_backward) {
                    prev_ix &= ring_buffer_mask;
                    let len = FindMatchLengthWithLimit(
                        &*data.offset(prev_ix as isize),
                        &*data.offset(cur_ix_masked as isize),
                        max_length,
                    );
                    if len >= 2 {
                        let mut score = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0 {
                                score = (score as u64)
                                    .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i))
                                    as u64;
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        let bank = key & (1 - 1i32) as u64;
        let mut backward_0 = 0;
        let mut hops = (*self_0).max_hops;
        let mut delta = cur_ix.wrapping_sub(*addr.offset(key as isize) as u64);
        let mut slot = *head.offset(key as isize) as u64;
        loop {
            let fresh8 = hops;
            hops = hops.wrapping_sub(1);
            if !(fresh8 != 0) {
                break;
            }
            let mut prev_ix_0: u64 = 0;
            let mut last = slot;
            backward_0 = (backward_0 as u64).wrapping_add(delta) as u64;
            if backward_0 > max_backward || 0 != 0 && delta == 0 {
                break;
            }
            prev_ix_0 = cur_ix.wrapping_sub(backward_0) & ring_buffer_mask;
            slot = (*banks.offset(bank as isize)).slots[last as usize].next as u64;
            delta = (*banks.offset(bank as isize)).slots[last as usize].delta as u64;
            if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
                || prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
                || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as i32
                    != *data.offset(prev_ix_0.wrapping_add(best_len) as isize) as i32
            {
                continue;
            }
            let len_0 = FindMatchLengthWithLimit(
                &*data.offset(prev_ix_0 as isize),
                &*data.offset(cur_ix_masked as isize),
                max_length,
            );
            if len_0 >= 4 {
                let mut score_0 = BackwardReferenceScore(len_0, backward_0);
                if best_score < score_0 {
                    best_score = score_0;
                    best_len = len_0;
                    (*out).len = best_len;
                    (*out).distance = backward_0;
                    (*out).score = best_score;
                }
            }
        }
        StoreH40(self_0, data, ring_buffer_mask, cur_ix);
        if (*out).score == min_score {
            SearchInStaticDictionary(
                dictionary,
                (*self_0).common,
                &*data.offset(cur_ix_masked as isize),
                max_length,
                dictionary_distance,
                max_distance,
                out,
                0,
            );
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH41() -> u64 {
    return 4;
}

#[inline(always)]
extern "C" fn StoreLookaheadH41() -> u64 {
    return 4;
}

#[inline(always)]
extern "C" fn HashBytesH41(mut data: *const u8) -> u64 {
    unsafe {
        let h = (BrotliUnalignedRead32(data as *const libc::c_void)).wrapping_mul(kHashMul32);
        return (h >> 32 - 15i32) as u64;
    }
}

extern "C" fn AddrH41(mut extra: *mut libc::c_void) -> *mut u32 {
    unsafe {
        return extra as *mut u32;
    }
}

extern "C" fn HeadH41(mut extra: *mut libc::c_void) -> *mut u16 {
    unsafe {
        return &mut *((AddrH41 as unsafe extern "C" fn(*mut libc::c_void) -> *mut u32)(extra))
            .offset((1i32 << 15i32) as isize) as *mut u32 as *mut u16;
    }
}

extern "C" fn TinyHashH41(mut extra: *mut libc::c_void) -> *mut u8 {
    unsafe {
        return &mut *((HeadH41 as unsafe extern "C" fn(*mut libc::c_void) -> *mut u16)(extra))
            .offset((1i32 << 15i32) as isize) as *mut u16 as *mut u8;
    }
}

extern "C" fn BanksH41(mut extra: *mut libc::c_void) -> *mut BankH41 {
    unsafe {
        return &mut *((TinyHashH41 as unsafe extern "C" fn(*mut libc::c_void) -> *mut u8)(extra))
            .offset(65536 as isize) as *mut u8 as *mut BankH41;
    }
}

#[inline(always)]
extern "C" fn StoreH41(mut self_0: *mut H41, mut data: *const u8, mask: u64, ix: u64) {
    unsafe {
        let mut addr = AddrH41((*self_0).extra);
        let mut head = HeadH41((*self_0).extra);
        let mut tiny_hash = TinyHashH41((*self_0).extra);
        let mut banks = BanksH41((*self_0).extra);
        let key = HashBytesH41(&*data.offset((ix & mask) as isize));
        let bank = key & (1 - 1i32) as u64;
        let ref mut fresh9 = (*self_0).free_slot_idx[bank as usize];
        let fresh10 = *fresh9;
        *fresh9 = (*fresh9).wrapping_add(1);
        let idx = (fresh10 as i32 & (1i32 << 16) - 1) as u64;
        let mut delta = ix.wrapping_sub(*addr.offset(key as isize) as u64);
        *tiny_hash.offset(ix as u16 as isize) = key as u8;
        if delta > 0xffff {
            delta = (if 0 != 0 { 0 } else { 0xffff }) as u64;
        };
        (*banks.offset(bank as isize)).slots[idx as usize].delta = delta as u16;
        (*banks.offset(bank as isize)).slots[idx as usize].next = *head.offset(key as isize);
        *addr.offset(key as isize) = ix as u32;
        *head.offset(key as isize) = idx as u16;
    }
}

#[inline(always)]
extern "C" fn StoreRangeH41(
    mut self_0: *mut H41,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
    unsafe {
        let mut i: u64 = 0;
        i = ix_start;
        while i < ix_end {
            StoreH41(self_0, data, mask, i);
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheH41(mut self_0: *mut H41, mut distance_cache: *mut i32) {
    unsafe {
        PrepareDistanceCache(distance_cache, 10);
    }
}

#[inline(always)]
extern "C" fn FindLongestMatchH41(
    mut self_0: *mut H41,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        let mut addr = AddrH41((*self_0).extra);
        let mut head = HeadH41((*self_0).extra);
        let mut tiny_hashes = TinyHashH41((*self_0).extra);
        let mut banks = BanksH41((*self_0).extra);
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let mut min_score = (*out).score;
        let mut best_score = (*out).score;
        let mut best_len = (*out).len;
        let mut i: u64 = 0;
        let key = HashBytesH41(&*data.offset(cur_ix_masked as isize));
        let tiny_hash = key as u8;
        (*out).len = 0;
        (*out).len_code_delta = 0;
        i = 0;
        while i < 10 {
            let backward = *distance_cache.offset(i as isize) as u64;
            let mut prev_ix = cur_ix.wrapping_sub(backward);
            if !(i > 0 && *tiny_hashes.offset(prev_ix as u16 as isize) as i32 != tiny_hash as i32) {
                if !(prev_ix >= cur_ix || backward > max_backward) {
                    prev_ix &= ring_buffer_mask;
                    let len = FindMatchLengthWithLimit(
                        &*data.offset(prev_ix as isize),
                        &*data.offset(cur_ix_masked as isize),
                        max_length,
                    );
                    if len >= 2 {
                        let mut score = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0 {
                                score = (score as u64)
                                    .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i))
                                    as u64;
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        let bank = key & (1 - 1i32) as u64;
        let mut backward_0 = 0;
        let mut hops = (*self_0).max_hops;
        let mut delta = cur_ix.wrapping_sub(*addr.offset(key as isize) as u64);
        let mut slot = *head.offset(key as isize) as u64;
        loop {
            let fresh11 = hops;
            hops = hops.wrapping_sub(1);
            if !(fresh11 != 0) {
                break;
            }
            let mut prev_ix_0: u64 = 0;
            let mut last = slot;
            backward_0 = (backward_0 as u64).wrapping_add(delta) as u64;
            if backward_0 > max_backward || 0 != 0 && delta == 0 {
                break;
            }
            prev_ix_0 = cur_ix.wrapping_sub(backward_0) & ring_buffer_mask;
            slot = (*banks.offset(bank as isize)).slots[last as usize].next as u64;
            delta = (*banks.offset(bank as isize)).slots[last as usize].delta as u64;
            if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
                || prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
                || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as i32
                    != *data.offset(prev_ix_0.wrapping_add(best_len) as isize) as i32
            {
                continue;
            }
            let len_0 = FindMatchLengthWithLimit(
                &*data.offset(prev_ix_0 as isize),
                &*data.offset(cur_ix_masked as isize),
                max_length,
            );
            if len_0 >= 4 {
                let mut score_0 = BackwardReferenceScore(len_0, backward_0);
                if best_score < score_0 {
                    best_score = score_0;
                    best_len = len_0;
                    (*out).len = best_len;
                    (*out).distance = backward_0;
                    (*out).score = best_score;
                }
            }
        }
        StoreH41(self_0, data, ring_buffer_mask, cur_ix);
        if (*out).score == min_score {
            SearchInStaticDictionary(
                dictionary,
                (*self_0).common,
                &*data.offset(cur_ix_masked as isize),
                max_length,
                dictionary_distance,
                max_distance,
                out,
                0,
            );
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH42() -> u64 {
    return 4;
}

#[inline(always)]
extern "C" fn StoreLookaheadH42() -> u64 {
    return 4;
}

#[inline(always)]
extern "C" fn HashBytesH42(mut data: *const u8) -> u64 {
    unsafe {
        let h = (BrotliUnalignedRead32(data as *const libc::c_void)).wrapping_mul(kHashMul32);
        return (h >> 32 - 15i32) as u64;
    }
}

extern "C" fn AddrH42(mut extra: *mut libc::c_void) -> *mut u32 {
    unsafe {
        return extra as *mut u32;
    }
}

extern "C" fn HeadH42(mut extra: *mut libc::c_void) -> *mut u16 {
    unsafe {
        return &mut *((AddrH42 as unsafe extern "C" fn(*mut libc::c_void) -> *mut u32)(extra))
            .offset((1i32 << 15i32) as isize) as *mut u32 as *mut u16;
    }
}

extern "C" fn TinyHashH42(mut extra: *mut libc::c_void) -> *mut u8 {
    unsafe {
        return &mut *((HeadH42 as unsafe extern "C" fn(*mut libc::c_void) -> *mut u16)(extra))
            .offset((1i32 << 15i32) as isize) as *mut u16 as *mut u8;
    }
}

extern "C" fn BanksH42(mut extra: *mut libc::c_void) -> *mut BankH42 {
    unsafe {
        return &mut *((TinyHashH42 as unsafe extern "C" fn(*mut libc::c_void) -> *mut u8)(extra))
            .offset(65536 as isize) as *mut u8 as *mut BankH42;
    }
}

#[inline(always)]
extern "C" fn StoreH42(mut self_0: *mut H42, mut data: *const u8, mask: u64, ix: u64) {
    unsafe {
        let mut addr = AddrH42((*self_0).extra);
        let mut head = HeadH42((*self_0).extra);
        let mut tiny_hash = TinyHashH42((*self_0).extra);
        let mut banks = BanksH42((*self_0).extra);
        let key = HashBytesH42(&*data.offset((ix & mask) as isize));
        let bank = key & (512 - 1i32) as u64;
        let ref mut fresh12 = (*self_0).free_slot_idx[bank as usize];
        let fresh13 = *fresh12;
        *fresh12 = (*fresh12).wrapping_add(1);
        let idx = (fresh13 as i32 & (1i32 << 9) - 1) as u64;
        let mut delta = ix.wrapping_sub(*addr.offset(key as isize) as u64);
        *tiny_hash.offset(ix as u16 as isize) = key as u8;
        if delta > 0xffff {
            delta = (if 0 != 0 { 0 } else { 0xffff }) as u64;
        };
        (*banks.offset(bank as isize)).slots[idx as usize].delta = delta as u16;
        (*banks.offset(bank as isize)).slots[idx as usize].next = *head.offset(key as isize);
        *addr.offset(key as isize) = ix as u32;
        *head.offset(key as isize) = idx as u16;
    }
}

#[inline(always)]
extern "C" fn StoreRangeH42(
    mut self_0: *mut H42,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
    unsafe {
        let mut i: u64 = 0;
        i = ix_start;
        while i < ix_end {
            StoreH42(self_0, data, mask, i);
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheH42(mut self_0: *mut H42, mut distance_cache: *mut i32) {
    unsafe {
        PrepareDistanceCache(distance_cache, 16);
    }
}

#[inline(always)]
extern "C" fn FindLongestMatchH42(
    mut self_0: *mut H42,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        let mut addr = AddrH42((*self_0).extra);
        let mut head = HeadH42((*self_0).extra);
        let mut tiny_hashes = TinyHashH42((*self_0).extra);
        let mut banks = BanksH42((*self_0).extra);
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let mut min_score = (*out).score;
        let mut best_score = (*out).score;
        let mut best_len = (*out).len;
        let mut i: u64 = 0;
        let key = HashBytesH42(&*data.offset(cur_ix_masked as isize));
        let tiny_hash = key as u8;
        (*out).len = 0;
        (*out).len_code_delta = 0;
        i = 0;
        while i < 16 {
            let backward = *distance_cache.offset(i as isize) as u64;
            let mut prev_ix = cur_ix.wrapping_sub(backward);
            if !(i > 0 && *tiny_hashes.offset(prev_ix as u16 as isize) as i32 != tiny_hash as i32) {
                if !(prev_ix >= cur_ix || backward > max_backward) {
                    prev_ix &= ring_buffer_mask;
                    let len = FindMatchLengthWithLimit(
                        &*data.offset(prev_ix as isize),
                        &*data.offset(cur_ix_masked as isize),
                        max_length,
                    );
                    if len >= 2 {
                        let mut score = BackwardReferenceScoreUsingLastDistance(len);
                        if best_score < score {
                            if i != 0 {
                                score = (score as u64)
                                    .wrapping_sub(BackwardReferencePenaltyUsingLastDistance(i))
                                    as u64;
                            }
                            if best_score < score {
                                best_score = score;
                                best_len = len;
                                (*out).len = best_len;
                                (*out).distance = backward;
                                (*out).score = best_score;
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        let bank = key & (512 - 1i32) as u64;
        let mut backward_0 = 0;
        let mut hops = (*self_0).max_hops;
        let mut delta = cur_ix.wrapping_sub(*addr.offset(key as isize) as u64);
        let mut slot = *head.offset(key as isize) as u64;
        loop {
            let fresh14 = hops;
            hops = hops.wrapping_sub(1);
            if !(fresh14 != 0) {
                break;
            }
            let mut prev_ix_0: u64 = 0;
            let mut last = slot;
            backward_0 = (backward_0 as u64).wrapping_add(delta) as u64;
            if backward_0 > max_backward || 0 != 0 && delta == 0 {
                break;
            }
            prev_ix_0 = cur_ix.wrapping_sub(backward_0) & ring_buffer_mask;
            slot = (*banks.offset(bank as isize)).slots[last as usize].next as u64;
            delta = (*banks.offset(bank as isize)).slots[last as usize].delta as u64;
            if cur_ix_masked.wrapping_add(best_len) > ring_buffer_mask
                || prev_ix_0.wrapping_add(best_len) > ring_buffer_mask
                || *data.offset(cur_ix_masked.wrapping_add(best_len) as isize) as i32
                    != *data.offset(prev_ix_0.wrapping_add(best_len) as isize) as i32
            {
                continue;
            }
            let len_0 = FindMatchLengthWithLimit(
                &*data.offset(prev_ix_0 as isize),
                &*data.offset(cur_ix_masked as isize),
                max_length,
            );
            if len_0 >= 4 {
                let mut score_0 = BackwardReferenceScore(len_0, backward_0);
                if best_score < score_0 {
                    best_score = score_0;
                    best_len = len_0;
                    (*out).len = best_len;
                    (*out).distance = backward_0;
                    (*out).score = best_score;
                }
            }
        }
        StoreH42(self_0, data, ring_buffer_mask, cur_ix);
        if (*out).score == min_score {
            SearchInStaticDictionary(
                dictionary,
                (*self_0).common,
                &*data.offset(cur_ix_masked as isize),
                max_length,
                dictionary_distance,
                max_distance,
                out,
                0,
            );
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH54() -> u64 {
    return 8;
}

#[inline(always)]
extern "C" fn StoreLookaheadH54() -> u64 {
    return 8;
}

extern "C" fn HashBytesH54(mut data: *const u8) -> u32 {
    unsafe {
        let h = (BrotliUnalignedRead64(data as *const libc::c_void) << 64 - 8 * 7i32)
            .wrapping_mul(kHashMul64);
        return (h >> 64 - 20i32) as u32;
    }
}

#[inline(always)]
extern "C" fn StoreH54(mut self_0: *mut H54, mut data: *const u8, mask: u64, ix: u64) {
    unsafe {
        let key = HashBytesH54(&*data.offset((ix & mask) as isize));
        if 1 << 2 == 1 {
            *((*self_0).buckets_).offset(key as isize) = ix as u32;
        } else {
            let off = (ix & (((1i32 << 2i32) - 1) << 3) as u64) as u32;
            *((*self_0).buckets_)
                .offset((key.wrapping_add(off) & ((1i32 << 20i32) - 1i32) as u32) as isize) =
                ix as u32;
        };
    }
}

#[inline(always)]
extern "C" fn StoreRangeH54(
    mut self_0: *mut H54,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
    unsafe {
        let mut i: u64 = 0;
        i = ix_start;
        while i < ix_end {
            StoreH54(self_0, data, mask, i);
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheH54(mut self_0: *mut H54, mut distance_cache: *mut i32) {}

#[inline(always)]
extern "C" fn FindLongestMatchH54(
    mut self_0: *mut H54,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        let mut buckets = (*self_0).buckets_;
        let best_len_in = (*out).len;
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let mut compare_char =
            *data.offset(cur_ix_masked.wrapping_add(best_len_in) as isize) as i32;
        let mut key = HashBytesH54(&*data.offset(cur_ix_masked as isize)) as u64;
        let mut key_out: u64 = 0;
        let mut min_score = (*out).score;
        let mut best_score = (*out).score;
        let mut best_len = best_len_in;
        let mut cached_backward = *distance_cache.offset(0 as isize) as u64;
        let mut prev_ix = cur_ix.wrapping_sub(cached_backward);
        (*out).len_code_delta = 0;
        if prev_ix < cur_ix {
            prev_ix &= ring_buffer_mask as u64;
            if compare_char == *data.offset(prev_ix.wrapping_add(best_len) as isize) as i32 {
                let len = FindMatchLengthWithLimit(
                    &*data.offset(prev_ix as isize),
                    &*data.offset(cur_ix_masked as isize),
                    max_length,
                );
                if len >= 4 {
                    let score = BackwardReferenceScoreUsingLastDistance(len);
                    if best_score < score {
                        (*out).len = len;
                        (*out).distance = cached_backward;
                        (*out).score = score;
                        if 1 << 2 == 1 {
                            *buckets.offset(key as isize) = cur_ix as u32;
                            return;
                        } else {
                            best_len = len;
                            best_score = score;
                            compare_char =
                                *data.offset(cur_ix_masked.wrapping_add(len) as isize) as i32;
                        }
                    }
                }
            }
        }
        {
            let mut keys: [u64; 4] = [0; 4];
            let mut i: u64 = 0;
            i = 0;
            while i < (1i32 << 2) as u64 {
                keys[i as usize] = key.wrapping_add(i << 3) & ((1i32 << 20) - 1) as u64;
                i = i.wrapping_add(1);
            }
            key_out = keys[((cur_ix & (((1i32 << 2i32) - 1i32) << 3) as u64) >> 3) as usize];
            i = 0;
            while i < (1i32 << 2) as u64 {
                let mut len_1: u64 = 0;
                let mut backward_0: u64 = 0;
                prev_ix = *buckets.offset(keys[i as usize] as isize) as u64;
                backward_0 = cur_ix.wrapping_sub(prev_ix);
                prev_ix &= ring_buffer_mask as u64;
                if !(compare_char != *data.offset(prev_ix.wrapping_add(best_len) as isize) as i32) {
                    if !((backward_0 == 0 || backward_0 > max_backward) as i64 != 0) {
                        len_1 = FindMatchLengthWithLimit(
                            &*data.offset(prev_ix as isize),
                            &*data.offset(cur_ix_masked as isize),
                            max_length,
                        );
                        if len_1 >= 4 {
                            let score_1 = BackwardReferenceScore(len_1, backward_0);
                            if best_score < score_1 {
                                best_len = len_1;
                                (*out).len = len_1;
                                compare_char =
                                    *data.offset(cur_ix_masked.wrapping_add(len_1) as isize) as i32;
                                best_score = score_1;
                                (*out).score = score_1;
                                (*out).distance = backward_0;
                            }
                        }
                    }
                }
                i = i.wrapping_add(1);
            }
        }
        if 1 << 2 != 1 {
            *buckets.offset(key_out as isize) = cur_ix as u32;
        }
    }
}

static mut kInvalidPosHROLLING_FAST: u32 = 0xffffffff;
#[inline(always)]
extern "C" fn HashTypeLengthHROLLING_FAST() -> u64 {
    return 4;
}

#[inline(always)]
extern "C" fn StoreLookaheadHROLLING_FAST() -> u64 {
    return 4;
}

extern "C" fn HashByteHROLLING_FAST(mut byte: u8) -> u32 {
    return (byte as u32).wrapping_add(1);
}

extern "C" fn HashRollingFunctionHROLLING_FAST(
    mut state: u32,
    mut add: u8,
    mut rem: u8,
    mut factor: u32,
    mut factor_remove: u32,
) -> u32 {
    return factor
        .wrapping_mul(state)
        .wrapping_add(HashByteHROLLING_FAST(add))
        .wrapping_sub(factor_remove.wrapping_mul(HashByteHROLLING_FAST(rem)));
}

#[inline(always)]
extern "C" fn StoreHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut data: *const u8,
    mask: u64,
    ix: u64,
) {
}

#[inline(always)]
extern "C" fn StoreRangeHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut distance_cache: *mut i32,
) {
}

#[inline(always)]
extern "C" fn FindLongestMatchHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let mut pos: u64 = 0;
        if cur_ix & (4 - 1i32) as u64 != 0 {
            return;
        }
        if max_length < 32 {
            return;
        }
        pos = (*self_0).next_ix;
        while pos <= cur_ix {
            let mut code = (*self_0).state & (16777216 * 64 - 1i32) as u32;
            let mut rem = *data.offset((pos & ring_buffer_mask) as isize);
            let mut add = *data.offset((pos.wrapping_add(32u64) & ring_buffer_mask) as isize);
            let mut found_ix = kInvalidPosHROLLING_FAST as u64;
            (*self_0).state = HashRollingFunctionHROLLING_FAST(
                (*self_0).state,
                add,
                rem,
                (*self_0).factor,
                (*self_0).factor_remove,
            );
            if code < 16777216 {
                found_ix = *((*self_0).table).offset(code as isize) as u64;
                *((*self_0).table).offset(code as isize) = pos as u32;
                if pos == cur_ix && found_ix != kInvalidPosHROLLING_FAST as u64 {
                    let mut backward = cur_ix.wrapping_sub(found_ix) as u64;
                    if backward <= max_backward {
                        let found_ix_masked = found_ix & ring_buffer_mask;
                        let len = FindMatchLengthWithLimit(
                            &*data.offset(found_ix_masked as isize),
                            &*data.offset(cur_ix_masked as isize),
                            max_length,
                        );
                        if len >= 4 && len > (*out).len {
                            let mut score = BackwardReferenceScore(len, backward);
                            if score > (*out).score {
                                (*out).len = len;
                                (*out).distance = backward;
                                (*out).score = score;
                                (*out).len_code_delta = 0;
                            }
                        }
                    }
                }
            }
            pos = (pos).wrapping_add(4) as u64;
        }
        (*self_0).next_ix = cur_ix.wrapping_add(4);
    }
}

static mut kInvalidPosHROLLING: u32 = 0xffffffff;
#[inline(always)]
extern "C" fn HashTypeLengthHROLLING() -> u64 {
    return 4;
}

#[inline(always)]
extern "C" fn StoreLookaheadHROLLING() -> u64 {
    return 4;
}

extern "C" fn HashByteHROLLING(mut byte: u8) -> u32 {
    return (byte as u32).wrapping_add(1);
}

extern "C" fn HashRollingFunctionHROLLING(
    mut state: u32,
    mut add: u8,
    mut rem: u8,
    mut factor: u32,
    mut factor_remove: u32,
) -> u32 {
    return factor
        .wrapping_mul(state)
        .wrapping_add(HashByteHROLLING(add))
        .wrapping_sub(factor_remove.wrapping_mul(HashByteHROLLING(rem)));
}

#[inline(always)]
extern "C" fn StoreHROLLING(mut self_0: *mut HROLLING, mut data: *const u8, mask: u64, ix: u64) {}

#[inline(always)]
extern "C" fn StoreRangeHROLLING(
    mut self_0: *mut HROLLING,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheHROLLING(
    mut self_0: *mut HROLLING,
    mut distance_cache: *mut i32,
) {
}

#[inline(always)]
extern "C" fn FindLongestMatchHROLLING(
    mut self_0: *mut HROLLING,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let mut pos: u64 = 0;
        if cur_ix & (1 - 1i32) as u64 != 0 {
            return;
        }
        if max_length < 32 {
            return;
        }
        pos = (*self_0).next_ix;
        while pos <= cur_ix {
            let mut code = (*self_0).state & (16777216 * 64 - 1i32) as u32;
            let mut rem = *data.offset((pos & ring_buffer_mask) as isize);
            let mut add = *data.offset((pos.wrapping_add(32u64) & ring_buffer_mask) as isize);
            let mut found_ix = kInvalidPosHROLLING as u64;
            (*self_0).state = HashRollingFunctionHROLLING(
                (*self_0).state,
                add,
                rem,
                (*self_0).factor,
                (*self_0).factor_remove,
            );
            if code < 16777216 {
                found_ix = *((*self_0).table).offset(code as isize) as u64;
                *((*self_0).table).offset(code as isize) = pos as u32;
                if pos == cur_ix && found_ix != kInvalidPosHROLLING as u64 {
                    let mut backward = cur_ix.wrapping_sub(found_ix) as u64;
                    if backward <= max_backward {
                        let found_ix_masked = found_ix & ring_buffer_mask;
                        let len = FindMatchLengthWithLimit(
                            &*data.offset(found_ix_masked as isize),
                            &*data.offset(cur_ix_masked as isize),
                            max_length,
                        );
                        if len >= 4 && len > (*out).len {
                            let mut score = BackwardReferenceScore(len, backward);
                            if score > (*out).score {
                                (*out).len = len;
                                (*out).distance = backward;
                                (*out).score = score;
                                (*out).len_code_delta = 0;
                            }
                        }
                    }
                }
            }
            pos = (pos).wrapping_add(1) as u64;
        }
        (*self_0).next_ix = cur_ix.wrapping_add(1);
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH35() -> u64 {
    let mut a = HashTypeLengthH3();
    let mut b = HashTypeLengthHROLLING_FAST();
    return if a > b { a } else { b };
}

#[inline(always)]
extern "C" fn StoreLookaheadH35() -> u64 {
    let mut a = StoreLookaheadH3();
    let mut b = StoreLookaheadHROLLING_FAST();
    return if a > b { a } else { b };
}

#[inline(always)]
extern "C" fn StoreH35(mut self_0: *mut H35, mut data: *const u8, mask: u64, ix: u64) {
    unsafe {
        StoreH3(&mut (*self_0).ha, data, mask, ix);
        StoreHROLLING_FAST(&mut (*self_0).hb, data, mask, ix);
    }
}

#[inline(always)]
extern "C" fn StoreRangeH35(
    mut self_0: *mut H35,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
    unsafe {
        StoreRangeH3(&mut (*self_0).ha, data, mask, ix_start, ix_end);
        StoreRangeHROLLING_FAST(&mut (*self_0).hb, data, mask, ix_start, ix_end);
    }
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheH35(mut self_0: *mut H35, mut distance_cache: *mut i32) {
    unsafe {
        PrepareDistanceCacheH3(&mut (*self_0).ha, distance_cache);
        PrepareDistanceCacheHROLLING_FAST(&mut (*self_0).hb, distance_cache);
    }
}

#[inline(always)]
extern "C" fn FindLongestMatchH35(
    mut self_0: *mut H35,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        FindLongestMatchH3(
            &mut (*self_0).ha,
            dictionary,
            data,
            ring_buffer_mask,
            distance_cache,
            cur_ix,
            max_length,
            max_backward,
            dictionary_distance,
            max_distance,
            out,
        );
        FindLongestMatchHROLLING_FAST(
            &mut (*self_0).hb,
            dictionary,
            data,
            ring_buffer_mask,
            distance_cache,
            cur_ix,
            max_length,
            max_backward,
            dictionary_distance,
            max_distance,
            out,
        );
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH55() -> u64 {
    let mut a = HashTypeLengthH54();
    let mut b = HashTypeLengthHROLLING_FAST();
    return if a > b { a } else { b };
}

#[inline(always)]
extern "C" fn StoreLookaheadH55() -> u64 {
    let mut a = StoreLookaheadH54();
    let mut b = StoreLookaheadHROLLING_FAST();
    return if a > b { a } else { b };
}

#[inline(always)]
extern "C" fn StoreH55(mut self_0: *mut H55, mut data: *const u8, mask: u64, ix: u64) {
    unsafe {
        StoreH54(&mut (*self_0).ha, data, mask, ix);
        StoreHROLLING_FAST(&mut (*self_0).hb, data, mask, ix);
    }
}

#[inline(always)]
extern "C" fn StoreRangeH55(
    mut self_0: *mut H55,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
    unsafe {
        StoreRangeH54(&mut (*self_0).ha, data, mask, ix_start, ix_end);
        StoreRangeHROLLING_FAST(&mut (*self_0).hb, data, mask, ix_start, ix_end);
    }
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheH55(mut self_0: *mut H55, mut distance_cache: *mut i32) {
    unsafe {
        PrepareDistanceCacheH54(&mut (*self_0).ha, distance_cache);
        PrepareDistanceCacheHROLLING_FAST(&mut (*self_0).hb, distance_cache);
    }
}

#[inline(always)]
extern "C" fn FindLongestMatchH55(
    mut self_0: *mut H55,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        FindLongestMatchH54(
            &mut (*self_0).ha,
            dictionary,
            data,
            ring_buffer_mask,
            distance_cache,
            cur_ix,
            max_length,
            max_backward,
            dictionary_distance,
            max_distance,
            out,
        );
        FindLongestMatchHROLLING_FAST(
            &mut (*self_0).hb,
            dictionary,
            data,
            ring_buffer_mask,
            distance_cache,
            cur_ix,
            max_length,
            max_backward,
            dictionary_distance,
            max_distance,
            out,
        );
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH65() -> u64 {
    let mut a = HashTypeLengthH6();
    let mut b = HashTypeLengthHROLLING();
    return if a > b { a } else { b };
}

#[inline(always)]
extern "C" fn StoreLookaheadH65() -> u64 {
    let mut a = StoreLookaheadH6();
    let mut b = StoreLookaheadHROLLING();
    return if a > b { a } else { b };
}

#[inline(always)]
extern "C" fn StoreH65(mut self_0: *mut H65, mut data: *const u8, mask: u64, ix: u64) {
    unsafe {
        StoreH6(&mut (*self_0).ha, data, mask, ix);
        StoreHROLLING(&mut (*self_0).hb, data, mask, ix);
    }
}

#[inline(always)]
extern "C" fn StoreRangeH65(
    mut self_0: *mut H65,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
    unsafe {
        StoreRangeH6(&mut (*self_0).ha, data, mask, ix_start, ix_end);
        StoreRangeHROLLING(&mut (*self_0).hb, data, mask, ix_start, ix_end);
    }
}

#[inline(always)]
extern "C" fn PrepareDistanceCacheH65(mut self_0: *mut H65, mut distance_cache: *mut i32) {
    unsafe {
        PrepareDistanceCacheH6(&mut (*self_0).ha, distance_cache);
        PrepareDistanceCacheHROLLING(&mut (*self_0).hb, distance_cache);
    }
}

#[inline(always)]
extern "C" fn FindLongestMatchH65(
    mut self_0: *mut H65,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    mut distance_cache: *const i32,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    max_distance: u64,
    mut out: *mut HasherSearchResult,
) {
    unsafe {
        FindLongestMatchH6(
            &mut (*self_0).ha,
            dictionary,
            data,
            ring_buffer_mask,
            distance_cache,
            cur_ix,
            max_length,
            max_backward,
            dictionary_distance,
            max_distance,
            out,
        );
        FindLongestMatchHROLLING(
            &mut (*self_0).hb,
            dictionary,
            data,
            ring_buffer_mask,
            distance_cache,
            cur_ix,
            max_length,
            max_backward,
            dictionary_distance,
            max_distance,
            out,
        );
    }
}

#[inline(always)]
extern "C" fn ComputeDistanceCode(
    mut distance: u64,
    mut max_distance: u64,
    mut dist_cache: *const i32,
) -> u64 {
    unsafe {
        if distance <= max_distance {
            let mut distance_plus_3 = distance.wrapping_add(3);
            let mut offset0 = distance_plus_3.wrapping_sub(*dist_cache.offset(0 as isize) as u64);
            let mut offset1 = distance_plus_3.wrapping_sub(*dist_cache.offset(1 as isize) as u64);
            if distance == *dist_cache.offset(0 as isize) as u64 {
                return 0;
            } else {
                if distance == *dist_cache.offset(1 as isize) as u64 {
                    return 1;
                } else {
                    if offset0 < 7 {
                        return (0x9750468 >> 4u64.wrapping_mul(offset0) & 0xfi32) as u64;
                    } else {
                        if offset1 < 7 {
                            return (0xfdb1ace >> 4u64.wrapping_mul(offset1) & 0xfi32) as u64;
                        } else {
                            if distance == *dist_cache.offset(2 as isize) as u64 {
                                return 2;
                            } else {
                                if distance == *dist_cache.offset(3 as isize) as u64 {
                                    return 3;
                                }
                            }
                        }
                    }
                }
            }
        }
        return distance.wrapping_add(16).wrapping_sub(1);
    }
}

#[inline(never)]
extern "C" fn CreateBackwardReferencesNH54(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut commands: *mut Command,
    mut num_commands: *mut u64,
    mut num_literals: *mut u64,
) {
    unsafe {
        let mut privat: *mut H54 = &mut (*hasher).privat._H54;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let position_offset = (*params).stream_offset;
        let orig_commands: *const Command = commands;
        let mut insert_length = *last_insert_len;
        let pos_end = position.wrapping_add(num_bytes);
        let store_end = if num_bytes >= StoreLookaheadH54() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH54())
                .wrapping_add(1)
        } else {
            position
        };
        let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
        let mut apply_random_heuristics = position.wrapping_add(random_heuristics_window_size);
        let gap = 0;
        let kMinScore = ((30 * 8i32) as u64)
            .wrapping_mul(::std::mem::size_of::<u64>() as u64)
            .wrapping_add(100);
        PrepareDistanceCacheH54(privat, dist_cache);
        while position.wrapping_add(HashTypeLengthH54()) < pos_end {
            let mut max_length = pos_end.wrapping_sub(position);
            let mut max_distance = brotli_min_size_t(position, max_backward_limit);
            let mut dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut sr = HasherSearchResult {
                len: 0,
                distance: 0,
                score: 0,
                len_code_delta: 0,
            };
            sr.len = 0;
            sr.len_code_delta = 0;
            sr.distance = 0;
            sr.score = kMinScore;
            FindLongestMatchH54(
                privat,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                dist_cache,
                position,
                max_length,
                max_distance,
                dictionary_start.wrapping_add(gap),
                (*params).dist.max_distance,
                &mut sr,
            );
            if sr.score > kMinScore {
                let mut delayed_backward_references_in_row = 0;
                max_length = max_length.wrapping_sub(1);
                loop {
                    let cost_diff_lazy = 175;
                    let mut sr2 = HasherSearchResult {
                        len: 0,
                        distance: 0,
                        score: 0,
                        len_code_delta: 0,
                    };
                    sr2.len = if (*params).quality < 5 {
                        brotli_min_size_t((sr.len).wrapping_sub(1), max_length)
                    } else {
                        0
                    };
                    sr2.len_code_delta = 0;
                    sr2.distance = 0;
                    sr2.score = kMinScore;
                    max_distance = brotli_min_size_t(position.wrapping_add(1), max_backward_limit);
                    dictionary_start = brotli_min_size_t(
                        position.wrapping_add(1).wrapping_add(position_offset),
                        max_backward_limit,
                    );
                    FindLongestMatchH54(
                        privat,
                        &(*params).dictionary,
                        ringbuffer,
                        ringbuffer_mask,
                        dist_cache,
                        position.wrapping_add(1),
                        max_length,
                        max_distance,
                        dictionary_start.wrapping_add(gap),
                        (*params).dist.max_distance,
                        &mut sr2,
                    );
                    if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                        break;
                    }
                    position = position.wrapping_add(1);
                    insert_length = insert_length.wrapping_add(1);
                    sr = sr2;
                    delayed_backward_references_in_row += 1;
                    if !(delayed_backward_references_in_row < 4
                        && position.wrapping_add(HashTypeLengthH54()) < pos_end)
                    {
                        break;
                    }
                    max_length = max_length.wrapping_sub(1);
                }
                apply_random_heuristics = position
                    .wrapping_add(2u64.wrapping_mul(sr.len))
                    .wrapping_add(random_heuristics_window_size);
                dictionary_start =
                    brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
                let mut distance_code = ComputeDistanceCode(
                    sr.distance,
                    dictionary_start.wrapping_add(gap),
                    dist_cache,
                );
                if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 {
                    *dist_cache.offset(3 as isize) = *dist_cache.offset(2 as isize);
                    *dist_cache.offset(2 as isize) = *dist_cache.offset(1 as isize);
                    *dist_cache.offset(1 as isize) = *dist_cache.offset(0 as isize);
                    *dist_cache.offset(0 as isize) = sr.distance as i32;
                    PrepareDistanceCacheH54(privat, dist_cache);
                }
                let fresh15 = commands;
                commands = commands.offset(1);
                InitCommand(
                    fresh15,
                    &(*params).dist,
                    insert_length,
                    sr.len,
                    sr.len_code_delta,
                    distance_code,
                );
                *num_literals = (*num_literals as u64).wrapping_add(insert_length) as u64;
                insert_length = 0;
                let mut range_start = position.wrapping_add(2);
                let mut range_end = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
                if sr.distance < sr.len >> 2 {
                    range_start = brotli_min_size_t(
                        range_end,
                        brotli_max_size_t(
                            range_start,
                            position.wrapping_add(sr.len).wrapping_sub(sr.distance << 2),
                        ),
                    );
                }
                StoreRangeH54(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
                position = (position as u64).wrapping_add(sr.len) as u64;
            } else {
                insert_length = insert_length.wrapping_add(1);
                position = position.wrapping_add(1);
                if position > apply_random_heuristics {
                    if position
                        > apply_random_heuristics
                            .wrapping_add(4u64.wrapping_mul(random_heuristics_window_size))
                    {
                        let kMargin = brotli_max_size_t((StoreLookaheadH54()).wrapping_sub(1), 4);
                        let mut pos_jump = brotli_min_size_t(
                            position.wrapping_add(16),
                            pos_end.wrapping_sub(kMargin),
                        );
                        while position < pos_jump {
                            StoreH54(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(4) as u64;
                            position = (position as u64).wrapping_add(4) as u64;
                        }
                    } else {
                        let kMargin_0 = brotli_max_size_t((StoreLookaheadH54()).wrapping_sub(1), 2);
                        let mut pos_jump_0 = brotli_min_size_t(
                            position.wrapping_add(8),
                            pos_end.wrapping_sub(kMargin_0),
                        );
                        while position < pos_jump_0 {
                            StoreH54(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(2) as u64;
                            position = (position as u64).wrapping_add(2) as u64;
                        }
                    }
                }
            }
        }
        insert_length = (insert_length as u64).wrapping_add(pos_end.wrapping_sub(position)) as u64;
        *last_insert_len = insert_length;
        *num_commands =
            (*num_commands as u64).wrapping_add(commands.offset_from(orig_commands) as u64) as u64;
    }
}

#[inline(never)]
extern "C" fn CreateBackwardReferencesNH35(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut commands: *mut Command,
    mut num_commands: *mut u64,
    mut num_literals: *mut u64,
) {
    unsafe {
        let mut privat: *mut H35 = &mut (*hasher).privat._H35;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let position_offset = (*params).stream_offset;
        let orig_commands: *const Command = commands;
        let mut insert_length = *last_insert_len;
        let pos_end = position.wrapping_add(num_bytes);
        let store_end = if num_bytes >= StoreLookaheadH35() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH35())
                .wrapping_add(1)
        } else {
            position
        };
        let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
        let mut apply_random_heuristics = position.wrapping_add(random_heuristics_window_size);
        let gap = 0;
        let kMinScore = ((30 * 8i32) as u64)
            .wrapping_mul(::std::mem::size_of::<u64>() as u64)
            .wrapping_add(100);
        PrepareDistanceCacheH35(privat, dist_cache);
        while position.wrapping_add(HashTypeLengthH35()) < pos_end {
            let mut max_length = pos_end.wrapping_sub(position);
            let mut max_distance = brotli_min_size_t(position, max_backward_limit);
            let mut dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut sr = HasherSearchResult {
                len: 0,
                distance: 0,
                score: 0,
                len_code_delta: 0,
            };
            sr.len = 0;
            sr.len_code_delta = 0;
            sr.distance = 0;
            sr.score = kMinScore;
            FindLongestMatchH35(
                privat,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                dist_cache,
                position,
                max_length,
                max_distance,
                dictionary_start.wrapping_add(gap),
                (*params).dist.max_distance,
                &mut sr,
            );
            if sr.score > kMinScore {
                let mut delayed_backward_references_in_row = 0;
                max_length = max_length.wrapping_sub(1);
                loop {
                    let cost_diff_lazy = 175;
                    let mut sr2 = HasherSearchResult {
                        len: 0,
                        distance: 0,
                        score: 0,
                        len_code_delta: 0,
                    };
                    sr2.len = if (*params).quality < 5 {
                        brotli_min_size_t((sr.len).wrapping_sub(1), max_length)
                    } else {
                        0
                    };
                    sr2.len_code_delta = 0;
                    sr2.distance = 0;
                    sr2.score = kMinScore;
                    max_distance = brotli_min_size_t(position.wrapping_add(1), max_backward_limit);
                    dictionary_start = brotli_min_size_t(
                        position.wrapping_add(1).wrapping_add(position_offset),
                        max_backward_limit,
                    );
                    FindLongestMatchH35(
                        privat,
                        &(*params).dictionary,
                        ringbuffer,
                        ringbuffer_mask,
                        dist_cache,
                        position.wrapping_add(1),
                        max_length,
                        max_distance,
                        dictionary_start.wrapping_add(gap),
                        (*params).dist.max_distance,
                        &mut sr2,
                    );
                    if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                        break;
                    }
                    position = position.wrapping_add(1);
                    insert_length = insert_length.wrapping_add(1);
                    sr = sr2;
                    delayed_backward_references_in_row += 1;
                    if !(delayed_backward_references_in_row < 4
                        && position.wrapping_add(HashTypeLengthH35()) < pos_end)
                    {
                        break;
                    }
                    max_length = max_length.wrapping_sub(1);
                }
                apply_random_heuristics = position
                    .wrapping_add(2u64.wrapping_mul(sr.len))
                    .wrapping_add(random_heuristics_window_size);
                dictionary_start =
                    brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
                let mut distance_code = ComputeDistanceCode(
                    sr.distance,
                    dictionary_start.wrapping_add(gap),
                    dist_cache,
                );
                if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 {
                    *dist_cache.offset(3 as isize) = *dist_cache.offset(2 as isize);
                    *dist_cache.offset(2 as isize) = *dist_cache.offset(1 as isize);
                    *dist_cache.offset(1 as isize) = *dist_cache.offset(0 as isize);
                    *dist_cache.offset(0 as isize) = sr.distance as i32;
                    PrepareDistanceCacheH35(privat, dist_cache);
                }
                let fresh16 = commands;
                commands = commands.offset(1);
                InitCommand(
                    fresh16,
                    &(*params).dist,
                    insert_length,
                    sr.len,
                    sr.len_code_delta,
                    distance_code,
                );
                *num_literals = (*num_literals as u64).wrapping_add(insert_length) as u64;
                insert_length = 0;
                let mut range_start = position.wrapping_add(2);
                let mut range_end = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
                if sr.distance < sr.len >> 2 {
                    range_start = brotli_min_size_t(
                        range_end,
                        brotli_max_size_t(
                            range_start,
                            position.wrapping_add(sr.len).wrapping_sub(sr.distance << 2),
                        ),
                    );
                }
                StoreRangeH35(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
                position = (position as u64).wrapping_add(sr.len) as u64;
            } else {
                insert_length = insert_length.wrapping_add(1);
                position = position.wrapping_add(1);
                if position > apply_random_heuristics {
                    if position
                        > apply_random_heuristics
                            .wrapping_add(4u64.wrapping_mul(random_heuristics_window_size))
                    {
                        let kMargin = brotli_max_size_t((StoreLookaheadH35()).wrapping_sub(1), 4);
                        let mut pos_jump = brotli_min_size_t(
                            position.wrapping_add(16),
                            pos_end.wrapping_sub(kMargin),
                        );
                        while position < pos_jump {
                            StoreH35(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(4) as u64;
                            position = (position as u64).wrapping_add(4) as u64;
                        }
                    } else {
                        let kMargin_0 = brotli_max_size_t((StoreLookaheadH35()).wrapping_sub(1), 2);
                        let mut pos_jump_0 = brotli_min_size_t(
                            position.wrapping_add(8),
                            pos_end.wrapping_sub(kMargin_0),
                        );
                        while position < pos_jump_0 {
                            StoreH35(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(2) as u64;
                            position = (position as u64).wrapping_add(2) as u64;
                        }
                    }
                }
            }
        }
        insert_length = (insert_length as u64).wrapping_add(pos_end.wrapping_sub(position)) as u64;
        *last_insert_len = insert_length;
        *num_commands =
            (*num_commands as u64).wrapping_add(commands.offset_from(orig_commands) as u64) as u64;
    }
}

#[inline(never)]
extern "C" fn CreateBackwardReferencesNH42(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut commands: *mut Command,
    mut num_commands: *mut u64,
    mut num_literals: *mut u64,
) {
    unsafe {
        let mut privat: *mut H42 = &mut (*hasher).privat._H42;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let position_offset = (*params).stream_offset;
        let orig_commands: *const Command = commands;
        let mut insert_length = *last_insert_len;
        let pos_end = position.wrapping_add(num_bytes);
        let store_end = if num_bytes >= StoreLookaheadH42() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH42())
                .wrapping_add(1)
        } else {
            position
        };
        let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
        let mut apply_random_heuristics = position.wrapping_add(random_heuristics_window_size);
        let gap = 0;
        let kMinScore = ((30 * 8i32) as u64)
            .wrapping_mul(::std::mem::size_of::<u64>() as u64)
            .wrapping_add(100);
        PrepareDistanceCacheH42(privat, dist_cache);
        while position.wrapping_add(HashTypeLengthH42()) < pos_end {
            let mut max_length = pos_end.wrapping_sub(position);
            let mut max_distance = brotli_min_size_t(position, max_backward_limit);
            let mut dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut sr = HasherSearchResult {
                len: 0,
                distance: 0,
                score: 0,
                len_code_delta: 0,
            };
            sr.len = 0;
            sr.len_code_delta = 0;
            sr.distance = 0;
            sr.score = kMinScore;
            FindLongestMatchH42(
                privat,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                dist_cache,
                position,
                max_length,
                max_distance,
                dictionary_start.wrapping_add(gap),
                (*params).dist.max_distance,
                &mut sr,
            );
            if sr.score > kMinScore {
                let mut delayed_backward_references_in_row = 0;
                max_length = max_length.wrapping_sub(1);
                loop {
                    let cost_diff_lazy = 175;
                    let mut sr2 = HasherSearchResult {
                        len: 0,
                        distance: 0,
                        score: 0,
                        len_code_delta: 0,
                    };
                    sr2.len = if (*params).quality < 5 {
                        brotli_min_size_t((sr.len).wrapping_sub(1), max_length)
                    } else {
                        0
                    };
                    sr2.len_code_delta = 0;
                    sr2.distance = 0;
                    sr2.score = kMinScore;
                    max_distance = brotli_min_size_t(position.wrapping_add(1), max_backward_limit);
                    dictionary_start = brotli_min_size_t(
                        position.wrapping_add(1).wrapping_add(position_offset),
                        max_backward_limit,
                    );
                    FindLongestMatchH42(
                        privat,
                        &(*params).dictionary,
                        ringbuffer,
                        ringbuffer_mask,
                        dist_cache,
                        position.wrapping_add(1),
                        max_length,
                        max_distance,
                        dictionary_start.wrapping_add(gap),
                        (*params).dist.max_distance,
                        &mut sr2,
                    );
                    if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                        break;
                    }
                    position = position.wrapping_add(1);
                    insert_length = insert_length.wrapping_add(1);
                    sr = sr2;
                    delayed_backward_references_in_row += 1;
                    if !(delayed_backward_references_in_row < 4
                        && position.wrapping_add(HashTypeLengthH42()) < pos_end)
                    {
                        break;
                    }
                    max_length = max_length.wrapping_sub(1);
                }
                apply_random_heuristics = position
                    .wrapping_add(2u64.wrapping_mul(sr.len))
                    .wrapping_add(random_heuristics_window_size);
                dictionary_start =
                    brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
                let mut distance_code = ComputeDistanceCode(
                    sr.distance,
                    dictionary_start.wrapping_add(gap),
                    dist_cache,
                );
                if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 {
                    *dist_cache.offset(3 as isize) = *dist_cache.offset(2 as isize);
                    *dist_cache.offset(2 as isize) = *dist_cache.offset(1 as isize);
                    *dist_cache.offset(1 as isize) = *dist_cache.offset(0 as isize);
                    *dist_cache.offset(0 as isize) = sr.distance as i32;
                    PrepareDistanceCacheH42(privat, dist_cache);
                }
                let fresh17 = commands;
                commands = commands.offset(1);
                InitCommand(
                    fresh17,
                    &(*params).dist,
                    insert_length,
                    sr.len,
                    sr.len_code_delta,
                    distance_code,
                );
                *num_literals = (*num_literals as u64).wrapping_add(insert_length) as u64;
                insert_length = 0;
                let mut range_start = position.wrapping_add(2);
                let mut range_end = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
                if sr.distance < sr.len >> 2 {
                    range_start = brotli_min_size_t(
                        range_end,
                        brotli_max_size_t(
                            range_start,
                            position.wrapping_add(sr.len).wrapping_sub(sr.distance << 2),
                        ),
                    );
                }
                StoreRangeH42(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
                position = (position as u64).wrapping_add(sr.len) as u64;
            } else {
                insert_length = insert_length.wrapping_add(1);
                position = position.wrapping_add(1);
                if position > apply_random_heuristics {
                    if position
                        > apply_random_heuristics
                            .wrapping_add(4u64.wrapping_mul(random_heuristics_window_size))
                    {
                        let kMargin = brotli_max_size_t((StoreLookaheadH42()).wrapping_sub(1), 4);
                        let mut pos_jump = brotli_min_size_t(
                            position.wrapping_add(16),
                            pos_end.wrapping_sub(kMargin),
                        );
                        while position < pos_jump {
                            StoreH42(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(4) as u64;
                            position = (position as u64).wrapping_add(4) as u64;
                        }
                    } else {
                        let kMargin_0 = brotli_max_size_t((StoreLookaheadH42()).wrapping_sub(1), 2);
                        let mut pos_jump_0 = brotli_min_size_t(
                            position.wrapping_add(8),
                            pos_end.wrapping_sub(kMargin_0),
                        );
                        while position < pos_jump_0 {
                            StoreH42(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(2) as u64;
                            position = (position as u64).wrapping_add(2) as u64;
                        }
                    }
                }
            }
        }
        insert_length = (insert_length as u64).wrapping_add(pos_end.wrapping_sub(position)) as u64;
        *last_insert_len = insert_length;
        *num_commands =
            (*num_commands as u64).wrapping_add(commands.offset_from(orig_commands) as u64) as u64;
    }
}

#[inline(never)]
extern "C" fn CreateBackwardReferencesNH41(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut commands: *mut Command,
    mut num_commands: *mut u64,
    mut num_literals: *mut u64,
) {
    unsafe {
        let mut privat: *mut H41 = &mut (*hasher).privat._H41;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let position_offset = (*params).stream_offset;
        let orig_commands: *const Command = commands;
        let mut insert_length = *last_insert_len;
        let pos_end = position.wrapping_add(num_bytes);
        let store_end = if num_bytes >= StoreLookaheadH41() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH41())
                .wrapping_add(1)
        } else {
            position
        };
        let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
        let mut apply_random_heuristics = position.wrapping_add(random_heuristics_window_size);
        let gap = 0;
        let kMinScore = ((30 * 8i32) as u64)
            .wrapping_mul(::std::mem::size_of::<u64>() as u64)
            .wrapping_add(100);
        PrepareDistanceCacheH41(privat, dist_cache);
        while position.wrapping_add(HashTypeLengthH41()) < pos_end {
            let mut max_length = pos_end.wrapping_sub(position);
            let mut max_distance = brotli_min_size_t(position, max_backward_limit);
            let mut dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut sr = HasherSearchResult {
                len: 0,
                distance: 0,
                score: 0,
                len_code_delta: 0,
            };
            sr.len = 0;
            sr.len_code_delta = 0;
            sr.distance = 0;
            sr.score = kMinScore;
            FindLongestMatchH41(
                privat,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                dist_cache,
                position,
                max_length,
                max_distance,
                dictionary_start.wrapping_add(gap),
                (*params).dist.max_distance,
                &mut sr,
            );
            if sr.score > kMinScore {
                let mut delayed_backward_references_in_row = 0;
                max_length = max_length.wrapping_sub(1);
                loop {
                    let cost_diff_lazy = 175;
                    let mut sr2 = HasherSearchResult {
                        len: 0,
                        distance: 0,
                        score: 0,
                        len_code_delta: 0,
                    };
                    sr2.len = if (*params).quality < 5 {
                        brotli_min_size_t((sr.len).wrapping_sub(1), max_length)
                    } else {
                        0
                    };
                    sr2.len_code_delta = 0;
                    sr2.distance = 0;
                    sr2.score = kMinScore;
                    max_distance = brotli_min_size_t(position.wrapping_add(1), max_backward_limit);
                    dictionary_start = brotli_min_size_t(
                        position.wrapping_add(1).wrapping_add(position_offset),
                        max_backward_limit,
                    );
                    FindLongestMatchH41(
                        privat,
                        &(*params).dictionary,
                        ringbuffer,
                        ringbuffer_mask,
                        dist_cache,
                        position.wrapping_add(1),
                        max_length,
                        max_distance,
                        dictionary_start.wrapping_add(gap),
                        (*params).dist.max_distance,
                        &mut sr2,
                    );
                    if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                        break;
                    }
                    position = position.wrapping_add(1);
                    insert_length = insert_length.wrapping_add(1);
                    sr = sr2;
                    delayed_backward_references_in_row += 1;
                    if !(delayed_backward_references_in_row < 4
                        && position.wrapping_add(HashTypeLengthH41()) < pos_end)
                    {
                        break;
                    }
                    max_length = max_length.wrapping_sub(1);
                }
                apply_random_heuristics = position
                    .wrapping_add(2u64.wrapping_mul(sr.len))
                    .wrapping_add(random_heuristics_window_size);
                dictionary_start =
                    brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
                let mut distance_code = ComputeDistanceCode(
                    sr.distance,
                    dictionary_start.wrapping_add(gap),
                    dist_cache,
                );
                if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 {
                    *dist_cache.offset(3 as isize) = *dist_cache.offset(2 as isize);
                    *dist_cache.offset(2 as isize) = *dist_cache.offset(1 as isize);
                    *dist_cache.offset(1 as isize) = *dist_cache.offset(0 as isize);
                    *dist_cache.offset(0 as isize) = sr.distance as i32;
                    PrepareDistanceCacheH41(privat, dist_cache);
                }
                let fresh18 = commands;
                commands = commands.offset(1);
                InitCommand(
                    fresh18,
                    &(*params).dist,
                    insert_length,
                    sr.len,
                    sr.len_code_delta,
                    distance_code,
                );
                *num_literals = (*num_literals as u64).wrapping_add(insert_length) as u64;
                insert_length = 0;
                let mut range_start = position.wrapping_add(2);
                let mut range_end = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
                if sr.distance < sr.len >> 2 {
                    range_start = brotli_min_size_t(
                        range_end,
                        brotli_max_size_t(
                            range_start,
                            position.wrapping_add(sr.len).wrapping_sub(sr.distance << 2),
                        ),
                    );
                }
                StoreRangeH41(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
                position = (position as u64).wrapping_add(sr.len) as u64;
            } else {
                insert_length = insert_length.wrapping_add(1);
                position = position.wrapping_add(1);
                if position > apply_random_heuristics {
                    if position
                        > apply_random_heuristics
                            .wrapping_add(4u64.wrapping_mul(random_heuristics_window_size))
                    {
                        let kMargin = brotli_max_size_t((StoreLookaheadH41()).wrapping_sub(1), 4);
                        let mut pos_jump = brotli_min_size_t(
                            position.wrapping_add(16),
                            pos_end.wrapping_sub(kMargin),
                        );
                        while position < pos_jump {
                            StoreH41(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(4) as u64;
                            position = (position as u64).wrapping_add(4) as u64;
                        }
                    } else {
                        let kMargin_0 = brotli_max_size_t((StoreLookaheadH41()).wrapping_sub(1), 2);
                        let mut pos_jump_0 = brotli_min_size_t(
                            position.wrapping_add(8),
                            pos_end.wrapping_sub(kMargin_0),
                        );
                        while position < pos_jump_0 {
                            StoreH41(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(2) as u64;
                            position = (position as u64).wrapping_add(2) as u64;
                        }
                    }
                }
            }
        }
        insert_length = (insert_length as u64).wrapping_add(pos_end.wrapping_sub(position)) as u64;
        *last_insert_len = insert_length;
        *num_commands =
            (*num_commands as u64).wrapping_add(commands.offset_from(orig_commands) as u64) as u64;
    }
}

#[inline(never)]
extern "C" fn CreateBackwardReferencesNH40(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut commands: *mut Command,
    mut num_commands: *mut u64,
    mut num_literals: *mut u64,
) {
    unsafe {
        let mut privat: *mut H40 = &mut (*hasher).privat._H40;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let position_offset = (*params).stream_offset;
        let orig_commands: *const Command = commands;
        let mut insert_length = *last_insert_len;
        let pos_end = position.wrapping_add(num_bytes);
        let store_end = if num_bytes >= StoreLookaheadH40() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH40())
                .wrapping_add(1)
        } else {
            position
        };
        let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
        let mut apply_random_heuristics = position.wrapping_add(random_heuristics_window_size);
        let gap = 0;
        let kMinScore = ((30 * 8i32) as u64)
            .wrapping_mul(::std::mem::size_of::<u64>() as u64)
            .wrapping_add(100);
        PrepareDistanceCacheH40(privat, dist_cache);
        while position.wrapping_add(HashTypeLengthH40()) < pos_end {
            let mut max_length = pos_end.wrapping_sub(position);
            let mut max_distance = brotli_min_size_t(position, max_backward_limit);
            let mut dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut sr = HasherSearchResult {
                len: 0,
                distance: 0,
                score: 0,
                len_code_delta: 0,
            };
            sr.len = 0;
            sr.len_code_delta = 0;
            sr.distance = 0;
            sr.score = kMinScore;
            FindLongestMatchH40(
                privat,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                dist_cache,
                position,
                max_length,
                max_distance,
                dictionary_start.wrapping_add(gap),
                (*params).dist.max_distance,
                &mut sr,
            );
            if sr.score > kMinScore {
                let mut delayed_backward_references_in_row = 0;
                max_length = max_length.wrapping_sub(1);
                loop {
                    let cost_diff_lazy = 175;
                    let mut sr2 = HasherSearchResult {
                        len: 0,
                        distance: 0,
                        score: 0,
                        len_code_delta: 0,
                    };
                    sr2.len = if (*params).quality < 5 {
                        brotli_min_size_t((sr.len).wrapping_sub(1), max_length)
                    } else {
                        0
                    };
                    sr2.len_code_delta = 0;
                    sr2.distance = 0;
                    sr2.score = kMinScore;
                    max_distance = brotli_min_size_t(position.wrapping_add(1), max_backward_limit);
                    dictionary_start = brotli_min_size_t(
                        position.wrapping_add(1).wrapping_add(position_offset),
                        max_backward_limit,
                    );
                    FindLongestMatchH40(
                        privat,
                        &(*params).dictionary,
                        ringbuffer,
                        ringbuffer_mask,
                        dist_cache,
                        position.wrapping_add(1),
                        max_length,
                        max_distance,
                        dictionary_start.wrapping_add(gap),
                        (*params).dist.max_distance,
                        &mut sr2,
                    );
                    if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                        break;
                    }
                    position = position.wrapping_add(1);
                    insert_length = insert_length.wrapping_add(1);
                    sr = sr2;
                    delayed_backward_references_in_row += 1;
                    if !(delayed_backward_references_in_row < 4
                        && position.wrapping_add(HashTypeLengthH40()) < pos_end)
                    {
                        break;
                    }
                    max_length = max_length.wrapping_sub(1);
                }
                apply_random_heuristics = position
                    .wrapping_add(2u64.wrapping_mul(sr.len))
                    .wrapping_add(random_heuristics_window_size);
                dictionary_start =
                    brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
                let mut distance_code = ComputeDistanceCode(
                    sr.distance,
                    dictionary_start.wrapping_add(gap),
                    dist_cache,
                );
                if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 {
                    *dist_cache.offset(3 as isize) = *dist_cache.offset(2 as isize);
                    *dist_cache.offset(2 as isize) = *dist_cache.offset(1 as isize);
                    *dist_cache.offset(1 as isize) = *dist_cache.offset(0 as isize);
                    *dist_cache.offset(0 as isize) = sr.distance as i32;
                    PrepareDistanceCacheH40(privat, dist_cache);
                }
                let fresh19 = commands;
                commands = commands.offset(1);
                InitCommand(
                    fresh19,
                    &(*params).dist,
                    insert_length,
                    sr.len,
                    sr.len_code_delta,
                    distance_code,
                );
                *num_literals = (*num_literals as u64).wrapping_add(insert_length) as u64;
                insert_length = 0;
                let mut range_start = position.wrapping_add(2);
                let mut range_end = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
                if sr.distance < sr.len >> 2 {
                    range_start = brotli_min_size_t(
                        range_end,
                        brotli_max_size_t(
                            range_start,
                            position.wrapping_add(sr.len).wrapping_sub(sr.distance << 2),
                        ),
                    );
                }
                StoreRangeH40(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
                position = (position as u64).wrapping_add(sr.len) as u64;
            } else {
                insert_length = insert_length.wrapping_add(1);
                position = position.wrapping_add(1);
                if position > apply_random_heuristics {
                    if position
                        > apply_random_heuristics
                            .wrapping_add(4u64.wrapping_mul(random_heuristics_window_size))
                    {
                        let kMargin = brotli_max_size_t((StoreLookaheadH40()).wrapping_sub(1), 4);
                        let mut pos_jump = brotli_min_size_t(
                            position.wrapping_add(16),
                            pos_end.wrapping_sub(kMargin),
                        );
                        while position < pos_jump {
                            StoreH40(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(4) as u64;
                            position = (position as u64).wrapping_add(4) as u64;
                        }
                    } else {
                        let kMargin_0 = brotli_max_size_t((StoreLookaheadH40()).wrapping_sub(1), 2);
                        let mut pos_jump_0 = brotli_min_size_t(
                            position.wrapping_add(8),
                            pos_end.wrapping_sub(kMargin_0),
                        );
                        while position < pos_jump_0 {
                            StoreH40(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(2) as u64;
                            position = (position as u64).wrapping_add(2) as u64;
                        }
                    }
                }
            }
        }
        insert_length = (insert_length as u64).wrapping_add(pos_end.wrapping_sub(position)) as u64;
        *last_insert_len = insert_length;
        *num_commands =
            (*num_commands as u64).wrapping_add(commands.offset_from(orig_commands) as u64) as u64;
    }
}

#[inline(never)]
extern "C" fn CreateBackwardReferencesNH6(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut commands: *mut Command,
    mut num_commands: *mut u64,
    mut num_literals: *mut u64,
) {
    unsafe {
        let mut privat: *mut H6 = &mut (*hasher).privat._H6;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let position_offset = (*params).stream_offset;
        let orig_commands: *const Command = commands;
        let mut insert_length = *last_insert_len;
        let pos_end = position.wrapping_add(num_bytes);
        let store_end = if num_bytes >= StoreLookaheadH6() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH6())
                .wrapping_add(1)
        } else {
            position
        };
        let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
        let mut apply_random_heuristics = position.wrapping_add(random_heuristics_window_size);
        let gap = 0;
        let kMinScore = ((30 * 8i32) as u64)
            .wrapping_mul(::std::mem::size_of::<u64>() as u64)
            .wrapping_add(100);
        PrepareDistanceCacheH6(privat, dist_cache);
        while position.wrapping_add(HashTypeLengthH6()) < pos_end {
            let mut max_length = pos_end.wrapping_sub(position);
            let mut max_distance = brotli_min_size_t(position, max_backward_limit);
            let mut dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut sr = HasherSearchResult {
                len: 0,
                distance: 0,
                score: 0,
                len_code_delta: 0,
            };
            sr.len = 0;
            sr.len_code_delta = 0;
            sr.distance = 0;
            sr.score = kMinScore;
            FindLongestMatchH6(
                privat,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                dist_cache,
                position,
                max_length,
                max_distance,
                dictionary_start.wrapping_add(gap),
                (*params).dist.max_distance,
                &mut sr,
            );
            if sr.score > kMinScore {
                let mut delayed_backward_references_in_row = 0;
                max_length = max_length.wrapping_sub(1);
                loop {
                    let cost_diff_lazy = 175;
                    let mut sr2 = HasherSearchResult {
                        len: 0,
                        distance: 0,
                        score: 0,
                        len_code_delta: 0,
                    };
                    sr2.len = if (*params).quality < 5 {
                        brotli_min_size_t((sr.len).wrapping_sub(1), max_length)
                    } else {
                        0
                    };
                    sr2.len_code_delta = 0;
                    sr2.distance = 0;
                    sr2.score = kMinScore;
                    max_distance = brotli_min_size_t(position.wrapping_add(1), max_backward_limit);
                    dictionary_start = brotli_min_size_t(
                        position.wrapping_add(1).wrapping_add(position_offset),
                        max_backward_limit,
                    );
                    FindLongestMatchH6(
                        privat,
                        &(*params).dictionary,
                        ringbuffer,
                        ringbuffer_mask,
                        dist_cache,
                        position.wrapping_add(1),
                        max_length,
                        max_distance,
                        dictionary_start.wrapping_add(gap),
                        (*params).dist.max_distance,
                        &mut sr2,
                    );
                    if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                        break;
                    }
                    position = position.wrapping_add(1);
                    insert_length = insert_length.wrapping_add(1);
                    sr = sr2;
                    delayed_backward_references_in_row += 1;
                    if !(delayed_backward_references_in_row < 4
                        && position.wrapping_add(HashTypeLengthH6()) < pos_end)
                    {
                        break;
                    }
                    max_length = max_length.wrapping_sub(1);
                }
                apply_random_heuristics = position
                    .wrapping_add(2u64.wrapping_mul(sr.len))
                    .wrapping_add(random_heuristics_window_size);
                dictionary_start =
                    brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
                let mut distance_code = ComputeDistanceCode(
                    sr.distance,
                    dictionary_start.wrapping_add(gap),
                    dist_cache,
                );
                if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 {
                    *dist_cache.offset(3 as isize) = *dist_cache.offset(2 as isize);
                    *dist_cache.offset(2 as isize) = *dist_cache.offset(1 as isize);
                    *dist_cache.offset(1 as isize) = *dist_cache.offset(0 as isize);
                    *dist_cache.offset(0 as isize) = sr.distance as i32;
                    PrepareDistanceCacheH6(privat, dist_cache);
                }
                let fresh20 = commands;
                commands = commands.offset(1);
                InitCommand(
                    fresh20,
                    &(*params).dist,
                    insert_length,
                    sr.len,
                    sr.len_code_delta,
                    distance_code,
                );
                *num_literals = (*num_literals as u64).wrapping_add(insert_length) as u64;
                insert_length = 0;
                let mut range_start = position.wrapping_add(2);
                let mut range_end = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
                if sr.distance < sr.len >> 2 {
                    range_start = brotli_min_size_t(
                        range_end,
                        brotli_max_size_t(
                            range_start,
                            position.wrapping_add(sr.len).wrapping_sub(sr.distance << 2),
                        ),
                    );
                }
                StoreRangeH6(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
                position = (position as u64).wrapping_add(sr.len) as u64;
            } else {
                insert_length = insert_length.wrapping_add(1);
                position = position.wrapping_add(1);
                if position > apply_random_heuristics {
                    if position
                        > apply_random_heuristics
                            .wrapping_add(4u64.wrapping_mul(random_heuristics_window_size))
                    {
                        let kMargin = brotli_max_size_t((StoreLookaheadH6()).wrapping_sub(1), 4);
                        let mut pos_jump = brotli_min_size_t(
                            position.wrapping_add(16),
                            pos_end.wrapping_sub(kMargin),
                        );
                        while position < pos_jump {
                            StoreH6(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(4) as u64;
                            position = (position as u64).wrapping_add(4) as u64;
                        }
                    } else {
                        let kMargin_0 = brotli_max_size_t((StoreLookaheadH6()).wrapping_sub(1), 2);
                        let mut pos_jump_0 = brotli_min_size_t(
                            position.wrapping_add(8),
                            pos_end.wrapping_sub(kMargin_0),
                        );
                        while position < pos_jump_0 {
                            StoreH6(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(2) as u64;
                            position = (position as u64).wrapping_add(2) as u64;
                        }
                    }
                }
            }
        }
        insert_length = (insert_length as u64).wrapping_add(pos_end.wrapping_sub(position)) as u64;
        *last_insert_len = insert_length;
        *num_commands =
            (*num_commands as u64).wrapping_add(commands.offset_from(orig_commands) as u64) as u64;
    }
}

#[inline(never)]
extern "C" fn CreateBackwardReferencesNH5(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut commands: *mut Command,
    mut num_commands: *mut u64,
    mut num_literals: *mut u64,
) {
    unsafe {
        let mut privat: *mut H5 = &mut (*hasher).privat._H5;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let position_offset = (*params).stream_offset;
        let orig_commands: *const Command = commands;
        let mut insert_length = *last_insert_len;
        let pos_end = position.wrapping_add(num_bytes);
        let store_end = if num_bytes >= StoreLookaheadH5() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH5())
                .wrapping_add(1)
        } else {
            position
        };
        let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
        let mut apply_random_heuristics = position.wrapping_add(random_heuristics_window_size);
        let gap = 0;
        let kMinScore = ((30 * 8i32) as u64)
            .wrapping_mul(::std::mem::size_of::<u64>() as u64)
            .wrapping_add(100);
        PrepareDistanceCacheH5(privat, dist_cache);
        while position.wrapping_add(HashTypeLengthH5()) < pos_end {
            let mut max_length = pos_end.wrapping_sub(position);
            let mut max_distance = brotli_min_size_t(position, max_backward_limit);
            let mut dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut sr = HasherSearchResult {
                len: 0,
                distance: 0,
                score: 0,
                len_code_delta: 0,
            };
            sr.len = 0;
            sr.len_code_delta = 0;
            sr.distance = 0;
            sr.score = kMinScore;
            FindLongestMatchH5(
                privat,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                dist_cache,
                position,
                max_length,
                max_distance,
                dictionary_start.wrapping_add(gap),
                (*params).dist.max_distance,
                &mut sr,
            );
            if sr.score > kMinScore {
                let mut delayed_backward_references_in_row = 0;
                max_length = max_length.wrapping_sub(1);
                loop {
                    let cost_diff_lazy = 175;
                    let mut sr2 = HasherSearchResult {
                        len: 0,
                        distance: 0,
                        score: 0,
                        len_code_delta: 0,
                    };
                    sr2.len = if (*params).quality < 5 {
                        brotli_min_size_t((sr.len).wrapping_sub(1), max_length)
                    } else {
                        0
                    };
                    sr2.len_code_delta = 0;
                    sr2.distance = 0;
                    sr2.score = kMinScore;
                    max_distance = brotli_min_size_t(position.wrapping_add(1), max_backward_limit);
                    dictionary_start = brotli_min_size_t(
                        position.wrapping_add(1).wrapping_add(position_offset),
                        max_backward_limit,
                    );
                    FindLongestMatchH5(
                        privat,
                        &(*params).dictionary,
                        ringbuffer,
                        ringbuffer_mask,
                        dist_cache,
                        position.wrapping_add(1),
                        max_length,
                        max_distance,
                        dictionary_start.wrapping_add(gap),
                        (*params).dist.max_distance,
                        &mut sr2,
                    );
                    if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                        break;
                    }
                    position = position.wrapping_add(1);
                    insert_length = insert_length.wrapping_add(1);
                    sr = sr2;
                    delayed_backward_references_in_row += 1;
                    if !(delayed_backward_references_in_row < 4
                        && position.wrapping_add(HashTypeLengthH5()) < pos_end)
                    {
                        break;
                    }
                    max_length = max_length.wrapping_sub(1);
                }
                apply_random_heuristics = position
                    .wrapping_add(2u64.wrapping_mul(sr.len))
                    .wrapping_add(random_heuristics_window_size);
                dictionary_start =
                    brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
                let mut distance_code = ComputeDistanceCode(
                    sr.distance,
                    dictionary_start.wrapping_add(gap),
                    dist_cache,
                );
                if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 {
                    *dist_cache.offset(3 as isize) = *dist_cache.offset(2 as isize);
                    *dist_cache.offset(2 as isize) = *dist_cache.offset(1 as isize);
                    *dist_cache.offset(1 as isize) = *dist_cache.offset(0 as isize);
                    *dist_cache.offset(0 as isize) = sr.distance as i32;
                    PrepareDistanceCacheH5(privat, dist_cache);
                }
                let fresh21 = commands;
                commands = commands.offset(1);
                InitCommand(
                    fresh21,
                    &(*params).dist,
                    insert_length,
                    sr.len,
                    sr.len_code_delta,
                    distance_code,
                );
                *num_literals = (*num_literals as u64).wrapping_add(insert_length) as u64;
                insert_length = 0;
                let mut range_start = position.wrapping_add(2);
                let mut range_end = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
                if sr.distance < sr.len >> 2 {
                    range_start = brotli_min_size_t(
                        range_end,
                        brotli_max_size_t(
                            range_start,
                            position.wrapping_add(sr.len).wrapping_sub(sr.distance << 2),
                        ),
                    );
                }
                StoreRangeH5(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
                position = (position as u64).wrapping_add(sr.len) as u64;
            } else {
                insert_length = insert_length.wrapping_add(1);
                position = position.wrapping_add(1);
                if position > apply_random_heuristics {
                    if position
                        > apply_random_heuristics
                            .wrapping_add(4u64.wrapping_mul(random_heuristics_window_size))
                    {
                        let kMargin = brotli_max_size_t((StoreLookaheadH5()).wrapping_sub(1), 4);
                        let mut pos_jump = brotli_min_size_t(
                            position.wrapping_add(16),
                            pos_end.wrapping_sub(kMargin),
                        );
                        while position < pos_jump {
                            StoreH5(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(4) as u64;
                            position = (position as u64).wrapping_add(4) as u64;
                        }
                    } else {
                        let kMargin_0 = brotli_max_size_t((StoreLookaheadH5()).wrapping_sub(1), 2);
                        let mut pos_jump_0 = brotli_min_size_t(
                            position.wrapping_add(8),
                            pos_end.wrapping_sub(kMargin_0),
                        );
                        while position < pos_jump_0 {
                            StoreH5(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(2) as u64;
                            position = (position as u64).wrapping_add(2) as u64;
                        }
                    }
                }
            }
        }
        insert_length = (insert_length as u64).wrapping_add(pos_end.wrapping_sub(position)) as u64;
        *last_insert_len = insert_length;
        *num_commands =
            (*num_commands as u64).wrapping_add(commands.offset_from(orig_commands) as u64) as u64;
    }
}

#[inline(never)]
extern "C" fn CreateBackwardReferencesNH4(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut commands: *mut Command,
    mut num_commands: *mut u64,
    mut num_literals: *mut u64,
) {
    unsafe {
        let mut privat: *mut H4 = &mut (*hasher).privat._H4;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let position_offset = (*params).stream_offset;
        let orig_commands: *const Command = commands;
        let mut insert_length = *last_insert_len;
        let pos_end = position.wrapping_add(num_bytes);
        let store_end = if num_bytes >= StoreLookaheadH4() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH4())
                .wrapping_add(1)
        } else {
            position
        };
        let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
        let mut apply_random_heuristics = position.wrapping_add(random_heuristics_window_size);
        let gap = 0;
        let kMinScore = ((30 * 8i32) as u64)
            .wrapping_mul(::std::mem::size_of::<u64>() as u64)
            .wrapping_add(100);
        PrepareDistanceCacheH4(privat, dist_cache);
        while position.wrapping_add(HashTypeLengthH4()) < pos_end {
            let mut max_length = pos_end.wrapping_sub(position);
            let mut max_distance = brotli_min_size_t(position, max_backward_limit);
            let mut dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut sr = HasherSearchResult {
                len: 0,
                distance: 0,
                score: 0,
                len_code_delta: 0,
            };
            sr.len = 0;
            sr.len_code_delta = 0;
            sr.distance = 0;
            sr.score = kMinScore;
            FindLongestMatchH4(
                privat,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                dist_cache,
                position,
                max_length,
                max_distance,
                dictionary_start.wrapping_add(gap),
                (*params).dist.max_distance,
                &mut sr,
            );
            if sr.score > kMinScore {
                let mut delayed_backward_references_in_row = 0;
                max_length = max_length.wrapping_sub(1);
                loop {
                    let cost_diff_lazy = 175;
                    let mut sr2 = HasherSearchResult {
                        len: 0,
                        distance: 0,
                        score: 0,
                        len_code_delta: 0,
                    };
                    sr2.len = if (*params).quality < 5 {
                        brotli_min_size_t((sr.len).wrapping_sub(1), max_length)
                    } else {
                        0
                    };
                    sr2.len_code_delta = 0;
                    sr2.distance = 0;
                    sr2.score = kMinScore;
                    max_distance = brotli_min_size_t(position.wrapping_add(1), max_backward_limit);
                    dictionary_start = brotli_min_size_t(
                        position.wrapping_add(1).wrapping_add(position_offset),
                        max_backward_limit,
                    );
                    FindLongestMatchH4(
                        privat,
                        &(*params).dictionary,
                        ringbuffer,
                        ringbuffer_mask,
                        dist_cache,
                        position.wrapping_add(1),
                        max_length,
                        max_distance,
                        dictionary_start.wrapping_add(gap),
                        (*params).dist.max_distance,
                        &mut sr2,
                    );
                    if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                        break;
                    }
                    position = position.wrapping_add(1);
                    insert_length = insert_length.wrapping_add(1);
                    sr = sr2;
                    delayed_backward_references_in_row += 1;
                    if !(delayed_backward_references_in_row < 4
                        && position.wrapping_add(HashTypeLengthH4()) < pos_end)
                    {
                        break;
                    }
                    max_length = max_length.wrapping_sub(1);
                }
                apply_random_heuristics = position
                    .wrapping_add(2u64.wrapping_mul(sr.len))
                    .wrapping_add(random_heuristics_window_size);
                dictionary_start =
                    brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
                let mut distance_code = ComputeDistanceCode(
                    sr.distance,
                    dictionary_start.wrapping_add(gap),
                    dist_cache,
                );
                if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 {
                    *dist_cache.offset(3 as isize) = *dist_cache.offset(2 as isize);
                    *dist_cache.offset(2 as isize) = *dist_cache.offset(1 as isize);
                    *dist_cache.offset(1 as isize) = *dist_cache.offset(0 as isize);
                    *dist_cache.offset(0 as isize) = sr.distance as i32;
                    PrepareDistanceCacheH4(privat, dist_cache);
                }
                let fresh22 = commands;
                commands = commands.offset(1);
                InitCommand(
                    fresh22,
                    &(*params).dist,
                    insert_length,
                    sr.len,
                    sr.len_code_delta,
                    distance_code,
                );
                *num_literals = (*num_literals as u64).wrapping_add(insert_length) as u64;
                insert_length = 0;
                let mut range_start = position.wrapping_add(2);
                let mut range_end = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
                if sr.distance < sr.len >> 2 {
                    range_start = brotli_min_size_t(
                        range_end,
                        brotli_max_size_t(
                            range_start,
                            position.wrapping_add(sr.len).wrapping_sub(sr.distance << 2),
                        ),
                    );
                }
                StoreRangeH4(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
                position = (position as u64).wrapping_add(sr.len) as u64;
            } else {
                insert_length = insert_length.wrapping_add(1);
                position = position.wrapping_add(1);
                if position > apply_random_heuristics {
                    if position
                        > apply_random_heuristics
                            .wrapping_add(4u64.wrapping_mul(random_heuristics_window_size))
                    {
                        let kMargin = brotli_max_size_t((StoreLookaheadH4()).wrapping_sub(1), 4);
                        let mut pos_jump = brotli_min_size_t(
                            position.wrapping_add(16),
                            pos_end.wrapping_sub(kMargin),
                        );
                        while position < pos_jump {
                            StoreH4(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(4) as u64;
                            position = (position as u64).wrapping_add(4) as u64;
                        }
                    } else {
                        let kMargin_0 = brotli_max_size_t((StoreLookaheadH4()).wrapping_sub(1), 2);
                        let mut pos_jump_0 = brotli_min_size_t(
                            position.wrapping_add(8),
                            pos_end.wrapping_sub(kMargin_0),
                        );
                        while position < pos_jump_0 {
                            StoreH4(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(2) as u64;
                            position = (position as u64).wrapping_add(2) as u64;
                        }
                    }
                }
            }
        }
        insert_length = (insert_length as u64).wrapping_add(pos_end.wrapping_sub(position)) as u64;
        *last_insert_len = insert_length;
        *num_commands =
            (*num_commands as u64).wrapping_add(commands.offset_from(orig_commands) as u64) as u64;
    }
}

#[inline(never)]
extern "C" fn CreateBackwardReferencesNH3(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut commands: *mut Command,
    mut num_commands: *mut u64,
    mut num_literals: *mut u64,
) {
    unsafe {
        let mut privat: *mut H3 = &mut (*hasher).privat._H3;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let position_offset = (*params).stream_offset;
        let orig_commands: *const Command = commands;
        let mut insert_length = *last_insert_len;
        let pos_end = position.wrapping_add(num_bytes);
        let store_end = if num_bytes >= StoreLookaheadH3() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH3())
                .wrapping_add(1)
        } else {
            position
        };
        let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
        let mut apply_random_heuristics = position.wrapping_add(random_heuristics_window_size);
        let gap = 0;
        let kMinScore = ((30 * 8i32) as u64)
            .wrapping_mul(::std::mem::size_of::<u64>() as u64)
            .wrapping_add(100);
        PrepareDistanceCacheH3(privat, dist_cache);
        while position.wrapping_add(HashTypeLengthH3()) < pos_end {
            let mut max_length = pos_end.wrapping_sub(position);
            let mut max_distance = brotli_min_size_t(position, max_backward_limit);
            let mut dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut sr = HasherSearchResult {
                len: 0,
                distance: 0,
                score: 0,
                len_code_delta: 0,
            };
            sr.len = 0;
            sr.len_code_delta = 0;
            sr.distance = 0;
            sr.score = kMinScore;
            FindLongestMatchH3(
                privat,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                dist_cache,
                position,
                max_length,
                max_distance,
                dictionary_start.wrapping_add(gap),
                (*params).dist.max_distance,
                &mut sr,
            );
            if sr.score > kMinScore {
                let mut delayed_backward_references_in_row = 0;
                max_length = max_length.wrapping_sub(1);
                loop {
                    let cost_diff_lazy = 175;
                    let mut sr2 = HasherSearchResult {
                        len: 0,
                        distance: 0,
                        score: 0,
                        len_code_delta: 0,
                    };
                    sr2.len = if (*params).quality < 5 {
                        brotli_min_size_t((sr.len).wrapping_sub(1), max_length)
                    } else {
                        0
                    };
                    sr2.len_code_delta = 0;
                    sr2.distance = 0;
                    sr2.score = kMinScore;
                    max_distance = brotli_min_size_t(position.wrapping_add(1), max_backward_limit);
                    dictionary_start = brotli_min_size_t(
                        position.wrapping_add(1).wrapping_add(position_offset),
                        max_backward_limit,
                    );
                    FindLongestMatchH3(
                        privat,
                        &(*params).dictionary,
                        ringbuffer,
                        ringbuffer_mask,
                        dist_cache,
                        position.wrapping_add(1),
                        max_length,
                        max_distance,
                        dictionary_start.wrapping_add(gap),
                        (*params).dist.max_distance,
                        &mut sr2,
                    );
                    if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                        break;
                    }
                    position = position.wrapping_add(1);
                    insert_length = insert_length.wrapping_add(1);
                    sr = sr2;
                    delayed_backward_references_in_row += 1;
                    if !(delayed_backward_references_in_row < 4
                        && position.wrapping_add(HashTypeLengthH3()) < pos_end)
                    {
                        break;
                    }
                    max_length = max_length.wrapping_sub(1);
                }
                apply_random_heuristics = position
                    .wrapping_add(2u64.wrapping_mul(sr.len))
                    .wrapping_add(random_heuristics_window_size);
                dictionary_start =
                    brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
                let mut distance_code = ComputeDistanceCode(
                    sr.distance,
                    dictionary_start.wrapping_add(gap),
                    dist_cache,
                );
                if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 {
                    *dist_cache.offset(3 as isize) = *dist_cache.offset(2 as isize);
                    *dist_cache.offset(2 as isize) = *dist_cache.offset(1 as isize);
                    *dist_cache.offset(1 as isize) = *dist_cache.offset(0 as isize);
                    *dist_cache.offset(0 as isize) = sr.distance as i32;
                    PrepareDistanceCacheH3(privat, dist_cache);
                }
                let fresh23 = commands;
                commands = commands.offset(1);
                InitCommand(
                    fresh23,
                    &(*params).dist,
                    insert_length,
                    sr.len,
                    sr.len_code_delta,
                    distance_code,
                );
                *num_literals = (*num_literals as u64).wrapping_add(insert_length) as u64;
                insert_length = 0;
                let mut range_start = position.wrapping_add(2);
                let mut range_end = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
                if sr.distance < sr.len >> 2 {
                    range_start = brotli_min_size_t(
                        range_end,
                        brotli_max_size_t(
                            range_start,
                            position.wrapping_add(sr.len).wrapping_sub(sr.distance << 2),
                        ),
                    );
                }
                StoreRangeH3(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
                position = (position as u64).wrapping_add(sr.len) as u64;
            } else {
                insert_length = insert_length.wrapping_add(1);
                position = position.wrapping_add(1);
                if position > apply_random_heuristics {
                    if position
                        > apply_random_heuristics
                            .wrapping_add(4u64.wrapping_mul(random_heuristics_window_size))
                    {
                        let kMargin = brotli_max_size_t((StoreLookaheadH3()).wrapping_sub(1), 4);
                        let mut pos_jump = brotli_min_size_t(
                            position.wrapping_add(16),
                            pos_end.wrapping_sub(kMargin),
                        );
                        while position < pos_jump {
                            StoreH3(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(4) as u64;
                            position = (position as u64).wrapping_add(4) as u64;
                        }
                    } else {
                        let kMargin_0 = brotli_max_size_t((StoreLookaheadH3()).wrapping_sub(1), 2);
                        let mut pos_jump_0 = brotli_min_size_t(
                            position.wrapping_add(8),
                            pos_end.wrapping_sub(kMargin_0),
                        );
                        while position < pos_jump_0 {
                            StoreH3(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(2) as u64;
                            position = (position as u64).wrapping_add(2) as u64;
                        }
                    }
                }
            }
        }
        insert_length = (insert_length as u64).wrapping_add(pos_end.wrapping_sub(position)) as u64;
        *last_insert_len = insert_length;
        *num_commands =
            (*num_commands as u64).wrapping_add(commands.offset_from(orig_commands) as u64) as u64;
    }
}

#[inline(never)]
extern "C" fn CreateBackwardReferencesNH2(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut commands: *mut Command,
    mut num_commands: *mut u64,
    mut num_literals: *mut u64,
) {
    unsafe {
        let mut privat: *mut H2 = &mut (*hasher).privat._H2;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let position_offset = (*params).stream_offset;
        let orig_commands: *const Command = commands;
        let mut insert_length = *last_insert_len;
        let pos_end = position.wrapping_add(num_bytes);
        let store_end = if num_bytes >= StoreLookaheadH2() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH2())
                .wrapping_add(1)
        } else {
            position
        };
        let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
        let mut apply_random_heuristics = position.wrapping_add(random_heuristics_window_size);
        let gap = 0;
        let kMinScore = ((30 * 8i32) as u64)
            .wrapping_mul(::std::mem::size_of::<u64>() as u64)
            .wrapping_add(100);
        PrepareDistanceCacheH2(privat, dist_cache);
        while position.wrapping_add(HashTypeLengthH2()) < pos_end {
            let mut max_length = pos_end.wrapping_sub(position);
            let mut max_distance = brotli_min_size_t(position, max_backward_limit);
            let mut dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut sr = HasherSearchResult {
                len: 0,
                distance: 0,
                score: 0,
                len_code_delta: 0,
            };
            sr.len = 0;
            sr.len_code_delta = 0;
            sr.distance = 0;
            sr.score = kMinScore;
            FindLongestMatchH2(
                privat,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                dist_cache,
                position,
                max_length,
                max_distance,
                dictionary_start.wrapping_add(gap),
                (*params).dist.max_distance,
                &mut sr,
            );
            if sr.score > kMinScore {
                let mut delayed_backward_references_in_row = 0;
                max_length = max_length.wrapping_sub(1);
                loop {
                    let cost_diff_lazy = 175;
                    let mut sr2 = HasherSearchResult {
                        len: 0,
                        distance: 0,
                        score: 0,
                        len_code_delta: 0,
                    };
                    sr2.len = if (*params).quality < 5 {
                        brotli_min_size_t((sr.len).wrapping_sub(1), max_length)
                    } else {
                        0
                    };
                    sr2.len_code_delta = 0;
                    sr2.distance = 0;
                    sr2.score = kMinScore;
                    max_distance = brotli_min_size_t(position.wrapping_add(1), max_backward_limit);
                    dictionary_start = brotli_min_size_t(
                        position.wrapping_add(1).wrapping_add(position_offset),
                        max_backward_limit,
                    );
                    FindLongestMatchH2(
                        privat,
                        &(*params).dictionary,
                        ringbuffer,
                        ringbuffer_mask,
                        dist_cache,
                        position.wrapping_add(1),
                        max_length,
                        max_distance,
                        dictionary_start.wrapping_add(gap),
                        (*params).dist.max_distance,
                        &mut sr2,
                    );
                    if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                        break;
                    }
                    position = position.wrapping_add(1);
                    insert_length = insert_length.wrapping_add(1);
                    sr = sr2;
                    delayed_backward_references_in_row += 1;
                    if !(delayed_backward_references_in_row < 4
                        && position.wrapping_add(HashTypeLengthH2()) < pos_end)
                    {
                        break;
                    }
                    max_length = max_length.wrapping_sub(1);
                }
                apply_random_heuristics = position
                    .wrapping_add(2u64.wrapping_mul(sr.len))
                    .wrapping_add(random_heuristics_window_size);
                dictionary_start =
                    brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
                let mut distance_code = ComputeDistanceCode(
                    sr.distance,
                    dictionary_start.wrapping_add(gap),
                    dist_cache,
                );
                if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 {
                    *dist_cache.offset(3 as isize) = *dist_cache.offset(2 as isize);
                    *dist_cache.offset(2 as isize) = *dist_cache.offset(1 as isize);
                    *dist_cache.offset(1 as isize) = *dist_cache.offset(0 as isize);
                    *dist_cache.offset(0 as isize) = sr.distance as i32;
                    PrepareDistanceCacheH2(privat, dist_cache);
                }
                let fresh24 = commands;
                commands = commands.offset(1);
                InitCommand(
                    fresh24,
                    &(*params).dist,
                    insert_length,
                    sr.len,
                    sr.len_code_delta,
                    distance_code,
                );
                *num_literals = (*num_literals as u64).wrapping_add(insert_length) as u64;
                insert_length = 0;
                let mut range_start = position.wrapping_add(2);
                let mut range_end = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
                if sr.distance < sr.len >> 2 {
                    range_start = brotli_min_size_t(
                        range_end,
                        brotli_max_size_t(
                            range_start,
                            position.wrapping_add(sr.len).wrapping_sub(sr.distance << 2),
                        ),
                    );
                }
                StoreRangeH2(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
                position = (position as u64).wrapping_add(sr.len) as u64;
            } else {
                insert_length = insert_length.wrapping_add(1);
                position = position.wrapping_add(1);
                if position > apply_random_heuristics {
                    if position
                        > apply_random_heuristics
                            .wrapping_add(4u64.wrapping_mul(random_heuristics_window_size))
                    {
                        let kMargin = brotli_max_size_t((StoreLookaheadH2()).wrapping_sub(1), 4);
                        let mut pos_jump = brotli_min_size_t(
                            position.wrapping_add(16),
                            pos_end.wrapping_sub(kMargin),
                        );
                        while position < pos_jump {
                            StoreH2(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(4) as u64;
                            position = (position as u64).wrapping_add(4) as u64;
                        }
                    } else {
                        let kMargin_0 = brotli_max_size_t((StoreLookaheadH2()).wrapping_sub(1), 2);
                        let mut pos_jump_0 = brotli_min_size_t(
                            position.wrapping_add(8),
                            pos_end.wrapping_sub(kMargin_0),
                        );
                        while position < pos_jump_0 {
                            StoreH2(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(2) as u64;
                            position = (position as u64).wrapping_add(2) as u64;
                        }
                    }
                }
            }
        }
        insert_length = (insert_length as u64).wrapping_add(pos_end.wrapping_sub(position)) as u64;
        *last_insert_len = insert_length;
        *num_commands =
            (*num_commands as u64).wrapping_add(commands.offset_from(orig_commands) as u64) as u64;
    }
}

#[inline(never)]
extern "C" fn CreateBackwardReferencesNH55(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut commands: *mut Command,
    mut num_commands: *mut u64,
    mut num_literals: *mut u64,
) {
    unsafe {
        let mut privat: *mut H55 = &mut (*hasher).privat._H55;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let position_offset = (*params).stream_offset;
        let orig_commands: *const Command = commands;
        let mut insert_length = *last_insert_len;
        let pos_end = position.wrapping_add(num_bytes);
        let store_end = if num_bytes >= StoreLookaheadH55() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH55())
                .wrapping_add(1)
        } else {
            position
        };
        let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
        let mut apply_random_heuristics = position.wrapping_add(random_heuristics_window_size);
        let gap = 0;
        let kMinScore = ((30 * 8i32) as u64)
            .wrapping_mul(::std::mem::size_of::<u64>() as u64)
            .wrapping_add(100);
        PrepareDistanceCacheH55(privat, dist_cache);
        while position.wrapping_add(HashTypeLengthH55()) < pos_end {
            let mut max_length = pos_end.wrapping_sub(position);
            let mut max_distance = brotli_min_size_t(position, max_backward_limit);
            let mut dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut sr = HasherSearchResult {
                len: 0,
                distance: 0,
                score: 0,
                len_code_delta: 0,
            };
            sr.len = 0;
            sr.len_code_delta = 0;
            sr.distance = 0;
            sr.score = kMinScore;
            FindLongestMatchH55(
                privat,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                dist_cache,
                position,
                max_length,
                max_distance,
                dictionary_start.wrapping_add(gap),
                (*params).dist.max_distance,
                &mut sr,
            );
            if sr.score > kMinScore {
                let mut delayed_backward_references_in_row = 0;
                max_length = max_length.wrapping_sub(1);
                loop {
                    let cost_diff_lazy = 175;
                    let mut sr2 = HasherSearchResult {
                        len: 0,
                        distance: 0,
                        score: 0,
                        len_code_delta: 0,
                    };
                    sr2.len = if (*params).quality < 5 {
                        brotli_min_size_t((sr.len).wrapping_sub(1), max_length)
                    } else {
                        0
                    };
                    sr2.len_code_delta = 0;
                    sr2.distance = 0;
                    sr2.score = kMinScore;
                    max_distance = brotli_min_size_t(position.wrapping_add(1), max_backward_limit);
                    dictionary_start = brotli_min_size_t(
                        position.wrapping_add(1).wrapping_add(position_offset),
                        max_backward_limit,
                    );
                    FindLongestMatchH55(
                        privat,
                        &(*params).dictionary,
                        ringbuffer,
                        ringbuffer_mask,
                        dist_cache,
                        position.wrapping_add(1),
                        max_length,
                        max_distance,
                        dictionary_start.wrapping_add(gap),
                        (*params).dist.max_distance,
                        &mut sr2,
                    );
                    if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                        break;
                    }
                    position = position.wrapping_add(1);
                    insert_length = insert_length.wrapping_add(1);
                    sr = sr2;
                    delayed_backward_references_in_row += 1;
                    if !(delayed_backward_references_in_row < 4
                        && position.wrapping_add(HashTypeLengthH55()) < pos_end)
                    {
                        break;
                    }
                    max_length = max_length.wrapping_sub(1);
                }
                apply_random_heuristics = position
                    .wrapping_add(2u64.wrapping_mul(sr.len))
                    .wrapping_add(random_heuristics_window_size);
                dictionary_start =
                    brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
                let mut distance_code = ComputeDistanceCode(
                    sr.distance,
                    dictionary_start.wrapping_add(gap),
                    dist_cache,
                );
                if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 {
                    *dist_cache.offset(3 as isize) = *dist_cache.offset(2 as isize);
                    *dist_cache.offset(2 as isize) = *dist_cache.offset(1 as isize);
                    *dist_cache.offset(1 as isize) = *dist_cache.offset(0 as isize);
                    *dist_cache.offset(0 as isize) = sr.distance as i32;
                    PrepareDistanceCacheH55(privat, dist_cache);
                }
                let fresh25 = commands;
                commands = commands.offset(1);
                InitCommand(
                    fresh25,
                    &(*params).dist,
                    insert_length,
                    sr.len,
                    sr.len_code_delta,
                    distance_code,
                );
                *num_literals = (*num_literals as u64).wrapping_add(insert_length) as u64;
                insert_length = 0;
                let mut range_start = position.wrapping_add(2);
                let mut range_end = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
                if sr.distance < sr.len >> 2 {
                    range_start = brotli_min_size_t(
                        range_end,
                        brotli_max_size_t(
                            range_start,
                            position.wrapping_add(sr.len).wrapping_sub(sr.distance << 2),
                        ),
                    );
                }
                StoreRangeH55(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
                position = (position as u64).wrapping_add(sr.len) as u64;
            } else {
                insert_length = insert_length.wrapping_add(1);
                position = position.wrapping_add(1);
                if position > apply_random_heuristics {
                    if position
                        > apply_random_heuristics
                            .wrapping_add(4u64.wrapping_mul(random_heuristics_window_size))
                    {
                        let kMargin = brotli_max_size_t((StoreLookaheadH55()).wrapping_sub(1), 4);
                        let mut pos_jump = brotli_min_size_t(
                            position.wrapping_add(16),
                            pos_end.wrapping_sub(kMargin),
                        );
                        while position < pos_jump {
                            StoreH55(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(4) as u64;
                            position = (position as u64).wrapping_add(4) as u64;
                        }
                    } else {
                        let kMargin_0 = brotli_max_size_t((StoreLookaheadH55()).wrapping_sub(1), 2);
                        let mut pos_jump_0 = brotli_min_size_t(
                            position.wrapping_add(8),
                            pos_end.wrapping_sub(kMargin_0),
                        );
                        while position < pos_jump_0 {
                            StoreH55(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(2) as u64;
                            position = (position as u64).wrapping_add(2) as u64;
                        }
                    }
                }
            }
        }
        insert_length = (insert_length as u64).wrapping_add(pos_end.wrapping_sub(position)) as u64;
        *last_insert_len = insert_length;
        *num_commands =
            (*num_commands as u64).wrapping_add(commands.offset_from(orig_commands) as u64) as u64;
    }
}

#[inline(never)]
extern "C" fn CreateBackwardReferencesNH65(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut commands: *mut Command,
    mut num_commands: *mut u64,
    mut num_literals: *mut u64,
) {
    unsafe {
        let mut privat: *mut H65 = &mut (*hasher).privat._H65;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let position_offset = (*params).stream_offset;
        let orig_commands: *const Command = commands;
        let mut insert_length = *last_insert_len;
        let pos_end = position.wrapping_add(num_bytes);
        let store_end = if num_bytes >= StoreLookaheadH65() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH65())
                .wrapping_add(1)
        } else {
            position
        };
        let random_heuristics_window_size = LiteralSpreeLengthForSparseSearch(params);
        let mut apply_random_heuristics = position.wrapping_add(random_heuristics_window_size);
        let gap = 0;
        let kMinScore = ((30 * 8i32) as u64)
            .wrapping_mul(::std::mem::size_of::<u64>() as u64)
            .wrapping_add(100);
        PrepareDistanceCacheH65(privat, dist_cache);
        while position.wrapping_add(HashTypeLengthH65()) < pos_end {
            let mut max_length = pos_end.wrapping_sub(position);
            let mut max_distance = brotli_min_size_t(position, max_backward_limit);
            let mut dictionary_start =
                brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
            let mut sr = HasherSearchResult {
                len: 0,
                distance: 0,
                score: 0,
                len_code_delta: 0,
            };
            sr.len = 0;
            sr.len_code_delta = 0;
            sr.distance = 0;
            sr.score = kMinScore;
            FindLongestMatchH65(
                privat,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                dist_cache,
                position,
                max_length,
                max_distance,
                dictionary_start.wrapping_add(gap),
                (*params).dist.max_distance,
                &mut sr,
            );
            if sr.score > kMinScore {
                let mut delayed_backward_references_in_row = 0;
                max_length = max_length.wrapping_sub(1);
                loop {
                    let cost_diff_lazy = 175;
                    let mut sr2 = HasherSearchResult {
                        len: 0,
                        distance: 0,
                        score: 0,
                        len_code_delta: 0,
                    };
                    sr2.len = if (*params).quality < 5 {
                        brotli_min_size_t((sr.len).wrapping_sub(1), max_length)
                    } else {
                        0
                    };
                    sr2.len_code_delta = 0;
                    sr2.distance = 0;
                    sr2.score = kMinScore;
                    max_distance = brotli_min_size_t(position.wrapping_add(1), max_backward_limit);
                    dictionary_start = brotli_min_size_t(
                        position.wrapping_add(1).wrapping_add(position_offset),
                        max_backward_limit,
                    );
                    FindLongestMatchH65(
                        privat,
                        &(*params).dictionary,
                        ringbuffer,
                        ringbuffer_mask,
                        dist_cache,
                        position.wrapping_add(1),
                        max_length,
                        max_distance,
                        dictionary_start.wrapping_add(gap),
                        (*params).dist.max_distance,
                        &mut sr2,
                    );
                    if !(sr2.score >= (sr.score).wrapping_add(cost_diff_lazy)) {
                        break;
                    }
                    position = position.wrapping_add(1);
                    insert_length = insert_length.wrapping_add(1);
                    sr = sr2;
                    delayed_backward_references_in_row += 1;
                    if !(delayed_backward_references_in_row < 4
                        && position.wrapping_add(HashTypeLengthH65()) < pos_end)
                    {
                        break;
                    }
                    max_length = max_length.wrapping_sub(1);
                }
                apply_random_heuristics = position
                    .wrapping_add(2u64.wrapping_mul(sr.len))
                    .wrapping_add(random_heuristics_window_size);
                dictionary_start =
                    brotli_min_size_t(position.wrapping_add(position_offset), max_backward_limit);
                let mut distance_code = ComputeDistanceCode(
                    sr.distance,
                    dictionary_start.wrapping_add(gap),
                    dist_cache,
                );
                if sr.distance <= dictionary_start.wrapping_add(gap) && distance_code > 0 {
                    *dist_cache.offset(3 as isize) = *dist_cache.offset(2 as isize);
                    *dist_cache.offset(2 as isize) = *dist_cache.offset(1 as isize);
                    *dist_cache.offset(1 as isize) = *dist_cache.offset(0 as isize);
                    *dist_cache.offset(0 as isize) = sr.distance as i32;
                    PrepareDistanceCacheH65(privat, dist_cache);
                }
                let fresh26 = commands;
                commands = commands.offset(1);
                InitCommand(
                    fresh26,
                    &(*params).dist,
                    insert_length,
                    sr.len,
                    sr.len_code_delta,
                    distance_code,
                );
                *num_literals = (*num_literals as u64).wrapping_add(insert_length) as u64;
                insert_length = 0;
                let mut range_start = position.wrapping_add(2);
                let mut range_end = brotli_min_size_t(position.wrapping_add(sr.len), store_end);
                if sr.distance < sr.len >> 2 {
                    range_start = brotli_min_size_t(
                        range_end,
                        brotli_max_size_t(
                            range_start,
                            position.wrapping_add(sr.len).wrapping_sub(sr.distance << 2),
                        ),
                    );
                }
                StoreRangeH65(privat, ringbuffer, ringbuffer_mask, range_start, range_end);
                position = (position as u64).wrapping_add(sr.len) as u64;
            } else {
                insert_length = insert_length.wrapping_add(1);
                position = position.wrapping_add(1);
                if position > apply_random_heuristics {
                    if position
                        > apply_random_heuristics
                            .wrapping_add(4u64.wrapping_mul(random_heuristics_window_size))
                    {
                        let kMargin = brotli_max_size_t((StoreLookaheadH65()).wrapping_sub(1), 4);
                        let mut pos_jump = brotli_min_size_t(
                            position.wrapping_add(16),
                            pos_end.wrapping_sub(kMargin),
                        );
                        while position < pos_jump {
                            StoreH65(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(4) as u64;
                            position = (position as u64).wrapping_add(4) as u64;
                        }
                    } else {
                        let kMargin_0 = brotli_max_size_t((StoreLookaheadH65()).wrapping_sub(1), 2);
                        let mut pos_jump_0 = brotli_min_size_t(
                            position.wrapping_add(8),
                            pos_end.wrapping_sub(kMargin_0),
                        );
                        while position < pos_jump_0 {
                            StoreH65(privat, ringbuffer, ringbuffer_mask, position);
                            insert_length = (insert_length as u64).wrapping_add(2) as u64;
                            position = (position as u64).wrapping_add(2) as u64;
                        }
                    }
                }
            }
        }
        insert_length = (insert_length as u64).wrapping_add(pos_end.wrapping_sub(position)) as u64;
        *last_insert_len = insert_length;
        *num_commands =
            (*num_commands as u64).wrapping_add(commands.offset_from(orig_commands) as u64) as u64;
    }
}

#[no_mangle]
pub extern "C" fn BrotliCreateBackwardReferences(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut hasher: *mut Hasher,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut commands: *mut Command,
    mut num_commands: *mut u64,
    mut num_literals: *mut u64,
) {
    unsafe {
        match (*params).hasher.type_0 {
            2 => {
                CreateBackwardReferencesNH2(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            3 => {
                CreateBackwardReferencesNH3(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            4 => {
                CreateBackwardReferencesNH4(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            5 => {
                CreateBackwardReferencesNH5(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            6 => {
                CreateBackwardReferencesNH6(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            40 => {
                CreateBackwardReferencesNH40(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            41 => {
                CreateBackwardReferencesNH41(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            42 => {
                CreateBackwardReferencesNH42(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            54 => {
                CreateBackwardReferencesNH54(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            35 => {
                CreateBackwardReferencesNH35(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            55 => {
                CreateBackwardReferencesNH55(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            65 => {
                CreateBackwardReferencesNH65(
                    num_bytes,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    literal_context_lut,
                    params,
                    hasher,
                    dist_cache,
                    last_insert_len,
                    commands,
                    num_commands,
                    num_literals,
                );
                return;
            }
            _ => {}
        };
    }
}
