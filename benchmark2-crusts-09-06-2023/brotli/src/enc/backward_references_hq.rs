use libc;
extern "C" {
    fn log2(_: f64) -> f64;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    static kBrotliLog2Table: [f64; 256];
    static kBrotliInsExtra: [u32; 24];
    static kBrotliCopyExtra: [u32; 24];
    fn BrotliAllocate(m: *mut MemoryManager, n: u64) -> *mut libc::c_void;
    fn BrotliFree(m: *mut MemoryManager, p: *mut libc::c_void);
    fn BrotliFindAllStaticDictionaryMatches(
        dictionary: *const BrotliEncoderDictionary,
        data: *const u8,
        min_length: u64,
        max_length: u64,
        matches: *mut u32,
    ) -> i32;
    fn BrotliEstimateBitCostsForLiterals(
        pos: u64,
        len: u64,
        mask: u64,
        data: *const u8,
        cost: *mut libc::c_float,
    );
}
pub type brotli_alloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
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
pub struct MemoryManager {
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub opaque: *mut libc::c_void,
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
pub struct BackwardMatch {
    pub distance: u32,
    pub length_and_code: u32,
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
pub struct H40 {
    pub free_slot_idx: [u16; 1],
    pub max_hops: u64,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliNode {
    pub length: u32,
    pub distance: u32,
    pub dcode_insert_length: u32,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub cost: libc::c_float,
    pub next: u32,
    pub shortcut: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliCostModel {
    pub cost_cmd_: [libc::c_float; 704],
    pub cost_dist_: *mut libc::c_float,
    pub distance_histogram_size: u32,
    pub literal_costs_: *mut libc::c_float,
    pub min_cost_cmd_: libc::c_float,
    pub num_bytes_: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StartPosQueue {
    pub q_: [PosData; 8],
    pub idx_: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PosData {
    pub pos: u64,
    pub distance_cache: [i32; 4],
    pub costdiff: libc::c_float,
    pub cost: libc::c_float,
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
extern "C" fn brotli_min_float(mut a: libc::c_float, mut b: libc::c_float) -> libc::c_float {
    return if a < b { a } else { b };
}

#[inline(always)]
extern "C" fn brotli_min_size_t(mut a: u64, mut b: u64) -> u64 {
    return if a < b { a } else { b };
}

#[inline(always)]
extern "C" fn brotli_max_size_t(mut a: u64, mut b: u64) -> u64 {
    return if a > b { a } else { b };
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
extern "C" fn GetInsertExtra(mut inscode: u16) -> u32 {
    unsafe {
        return kBrotliInsExtra[inscode as usize];
    }
}

#[inline(always)]
extern "C" fn GetCopyExtra(mut copycode: u16) -> u32 {
    unsafe {
        return kBrotliCopyExtra[copycode as usize];
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
extern "C" fn CommandCopyLen(mut self_0: *const Command) -> u32 {
    unsafe {
        return (*self_0).copy_len_ & 0x1ffffff;
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
extern "C" fn MaxZopfliLen(mut params: *const BrotliEncoderParams) -> u64 {
    unsafe {
        return (if (*params).quality <= 10 { 150 } else { 325 }) as u64;
    }
}

#[inline(always)]
extern "C" fn MaxZopfliCandidates(mut params: *const BrotliEncoderParams) -> u64 {
    unsafe {
        return (if (*params).quality <= 10 { 1 } else { 5 }) as u64;
    }
}

static mut kInvalidMatch: u32 = 0xfffffff;
static mut kHashMul32: u32 = 0x1e35a7bd;
#[inline(always)]
extern "C" fn InitBackwardMatch(mut self_0: *mut BackwardMatch, mut dist: u64, mut len: u64) {
    unsafe {
        (*self_0).distance = dist as u32;
        (*self_0).length_and_code = (len << 5i32) as u32;
    }
}

#[inline(always)]
extern "C" fn InitDictionaryBackwardMatch(
    mut self_0: *mut BackwardMatch,
    mut dist: u64,
    mut len: u64,
    mut len_code: u64,
) {
    unsafe {
        (*self_0).distance = dist as u32;
        (*self_0).length_and_code =
            (len << 5 | (if len == len_code { 0u64 } else { len_code })) as u32;
    }
}

#[inline(always)]
extern "C" fn BackwardMatchLength(mut self_0: *const BackwardMatch) -> u64 {
    unsafe {
        return ((*self_0).length_and_code >> 5i32) as u64;
    }
}

#[inline(always)]
extern "C" fn BackwardMatchLengthCode(mut self_0: *const BackwardMatch) -> u64 {
    unsafe {
        let mut code = ((*self_0).length_and_code & 31u32) as u64;
        return if code != 0 {
            code
        } else {
            BackwardMatchLength(self_0)
        };
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH10() -> u64 {
    return 4;
}

#[inline(always)]
extern "C" fn StoreLookaheadH10() -> u64 {
    return 128;
}

extern "C" fn HashBytesH10(mut data: *const u8) -> u32 {
    unsafe {
        let mut h = (BrotliUnalignedRead32(data as *const libc::c_void)).wrapping_mul(kHashMul32);
        return h >> 32 - 17;
    }
}

#[inline(always)]
extern "C" fn LeftChildIndexH10(mut self_0: *mut H10, pos: u64) -> u64 {
    unsafe {
        return 2u64.wrapping_mul(pos & (*self_0).window_mask_);
    }
}

#[inline(always)]
extern "C" fn RightChildIndexH10(mut self_0: *mut H10, pos: u64) -> u64 {
    unsafe {
        return 2u64
            .wrapping_mul(pos & (*self_0).window_mask_)
            .wrapping_add(1);
    }
}

#[inline(always)]
extern "C" fn StoreAndFindMatchesH10(
    mut self_0: *mut H10,
    mut data: *const u8,
    cur_ix: u64,
    ring_buffer_mask: u64,
    max_length: u64,
    max_backward: u64,
    best_len: *mut u64,
    mut matches: *mut BackwardMatch,
) -> *mut BackwardMatch {
    unsafe {
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let max_comp_len = brotli_min_size_t(max_length, 128);
        let should_reroot_tree = if max_length >= 128 { 1 } else { 0 };
        let key = HashBytesH10(&*data.offset(cur_ix_masked as isize));
        let mut buckets = (*self_0).buckets_;
        let mut forest = (*self_0).forest_;
        let mut prev_ix = *buckets.offset(key as isize) as u64;
        let mut node_left = LeftChildIndexH10(self_0, cur_ix);
        let mut node_right = RightChildIndexH10(self_0, cur_ix);
        let mut best_len_left = 0;
        let mut best_len_right = 0;
        let mut depth_remaining: u64 = 0;
        if should_reroot_tree != 0 {
            *buckets.offset(key as isize) = cur_ix as u32;
        }
        depth_remaining = 64;
        loop {
            let backward = cur_ix.wrapping_sub(prev_ix);
            let prev_ix_masked = prev_ix & ring_buffer_mask;
            if backward == 0 || backward > max_backward || depth_remaining == 0 {
                if should_reroot_tree != 0 {
                    *forest.offset(node_left as isize) = (*self_0).invalid_pos_;
                    *forest.offset(node_right as isize) = (*self_0).invalid_pos_;
                }
                break;
            } else {
                let cur_len = brotli_min_size_t(best_len_left, best_len_right);
                let mut len: u64 = 0;
                len = cur_len.wrapping_add(FindMatchLengthWithLimit(
                    &*data.offset(cur_ix_masked.wrapping_add(cur_len) as isize),
                    &*data.offset(prev_ix_masked.wrapping_add(cur_len) as isize),
                    max_length.wrapping_sub(cur_len),
                ));
                if !matches.is_null() && len > *best_len {
                    *best_len = len;
                    let fresh0 = matches;
                    matches = matches.offset(1);
                    InitBackwardMatch(fresh0, backward, len);
                }
                if len >= max_comp_len {
                    if should_reroot_tree != 0 {
                        *forest.offset(node_left as isize) =
                            *forest.offset(LeftChildIndexH10(self_0, prev_ix) as isize);
                        *forest.offset(node_right as isize) =
                            *forest.offset(RightChildIndexH10(self_0, prev_ix) as isize);
                    }
                    break;
                } else {
                    if *data.offset(cur_ix_masked.wrapping_add(len) as isize) as i32
                        > *data.offset(prev_ix_masked.wrapping_add(len) as isize) as i32
                    {
                        best_len_left = len;
                        if should_reroot_tree != 0 {
                            *forest.offset(node_left as isize) = prev_ix as u32;
                        }
                        node_left = RightChildIndexH10(self_0, prev_ix);
                        prev_ix = *forest.offset(node_left as isize) as u64;
                    } else {
                        best_len_right = len;
                        if should_reroot_tree != 0 {
                            *forest.offset(node_right as isize) = prev_ix as u32;
                        }
                        node_right = LeftChildIndexH10(self_0, prev_ix);
                        prev_ix = *forest.offset(node_right as isize) as u64;
                    }
                    depth_remaining = depth_remaining.wrapping_sub(1);
                }
            }
        }
        return matches;
    }
}

#[inline(always)]
extern "C" fn FindAllMatchesH10(
    mut self_0: *mut H10,
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    ring_buffer_mask: u64,
    cur_ix: u64,
    max_length: u64,
    max_backward: u64,
    dictionary_distance: u64,
    mut params: *const BrotliEncoderParams,
    mut matches: *mut BackwardMatch,
) -> u64 {
    unsafe {
        let orig_matches = matches;
        let cur_ix_masked = cur_ix & ring_buffer_mask;
        let mut best_len = 1;
        let short_match_max_backward = (if (*params).quality != 11 { 16 } else { 64 }) as u64;
        let mut stop = cur_ix.wrapping_sub(short_match_max_backward);
        let mut dict_matches: [u32; 38] = [0; 38];
        let mut i: u64 = 0;
        if cur_ix < short_match_max_backward {
            stop = 0;
        }
        i = cur_ix.wrapping_sub(1);
        while i > stop && best_len <= 2 {
            let mut prev_ix = i;
            let backward = cur_ix.wrapping_sub(prev_ix);
            if (backward > max_backward) as i64 != 0 {
                break;
            }
            prev_ix &= ring_buffer_mask;
            if !(*data.offset(cur_ix_masked as isize) as i32
                != *data.offset(prev_ix as isize) as i32
                || *data.offset(cur_ix_masked.wrapping_add(1) as isize) as i32
                    != *data.offset(prev_ix.wrapping_add(1) as isize) as i32)
            {
                let len = FindMatchLengthWithLimit(
                    &*data.offset(prev_ix as isize),
                    &*data.offset(cur_ix_masked as isize),
                    max_length,
                );
                if len > best_len {
                    best_len = len;
                    let fresh1 = matches;
                    matches = matches.offset(1);
                    InitBackwardMatch(fresh1, backward, len);
                }
            }
            i = i.wrapping_sub(1);
        }
        if best_len < max_length {
            matches = StoreAndFindMatchesH10(
                self_0,
                data,
                cur_ix,
                ring_buffer_mask,
                max_length,
                max_backward,
                &mut best_len,
                matches,
            );
        }
        i = 0;
        while i <= 37 {
            dict_matches[i as usize] = kInvalidMatch;
            i = i.wrapping_add(1);
        }
        let mut minlen = brotli_max_size_t(4, best_len.wrapping_add(1));
        if BrotliFindAllStaticDictionaryMatches(
            dictionary,
            &*data.offset(cur_ix_masked as isize),
            minlen,
            max_length,
            &mut *dict_matches.as_mut_ptr().offset(0 as isize),
        ) != 0
        {
            let mut maxlen = brotli_min_size_t(37, max_length);
            let mut l: u64 = 0;
            l = minlen;
            while l <= maxlen {
                let mut dict_id = dict_matches[l as usize];
                if dict_id < kInvalidMatch {
                    let mut distance = dictionary_distance
                        .wrapping_add((dict_id >> 5i32) as u64)
                        .wrapping_add(1);
                    if distance <= (*params).dist.max_distance {
                        let fresh2 = matches;
                        matches = matches.offset(1);
                        InitDictionaryBackwardMatch(fresh2, distance, l, (dict_id & 31u32) as u64);
                    }
                }
                l = l.wrapping_add(1);
            }
        }
        return matches.offset_from(orig_matches) as u64;
    }
}

#[inline(always)]
extern "C" fn StoreH10(mut self_0: *mut H10, mut data: *const u8, mask: u64, ix: u64) {
    unsafe {
        let max_backward = ((*self_0).window_mask_).wrapping_sub(16).wrapping_add(1);
        StoreAndFindMatchesH10(
            self_0,
            data,
            ix,
            mask,
            128,
            max_backward,
            0 as *mut u64,
            0 as *mut BackwardMatch,
        );
    }
}

#[inline(always)]
extern "C" fn StoreRangeH10(
    mut self_0: *mut H10,
    mut data: *const u8,
    mask: u64,
    ix_start: u64,
    ix_end: u64,
) {
    unsafe {
        let mut i = ix_start;
        let mut j = ix_start;
        if ix_start.wrapping_add(63) <= ix_end {
            i = ix_end.wrapping_sub(63);
        }
        if ix_start.wrapping_add(512) <= i {
            while j < i {
                StoreH10(self_0, data, mask, j);
                j = (j as u64).wrapping_add(8) as u64;
            }
        }
        while i < ix_end {
            StoreH10(self_0, data, mask, i);
            i = i.wrapping_add(1);
        }
    }
}

static mut kInfinity: libc::c_float = 1.7e38f32;
static mut kDistanceCacheIndex: [u32; 16] = [0, 1, 2, 3, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1];
static mut kDistanceCacheOffset: [i32; 16] = [0, 0, 0, 0, -1, 1, -2, 2, -3, 3, -1, 1, -2, 2, -3, 3];
#[no_mangle]
pub extern "C" fn BrotliInitZopfliNodes(mut array: *mut ZopfliNode, mut length: u64) {
    unsafe {
        let mut stub = ZopfliNode {
            length: 0,
            distance: 0,
            dcode_insert_length: 0,
            u: C2RustUnnamed_0 { cost: 0. },
        };
        let mut i: u64 = 0;
        stub.length = 1;
        stub.distance = 0;
        stub.dcode_insert_length = 0;
        stub.u.cost = kInfinity;
        i = 0;
        while i < length {
            *array.offset(i as isize) = stub;
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn ZopfliNodeCopyLength(mut self_0: *const ZopfliNode) -> u32 {
    unsafe {
        return (*self_0).length & 0x1ffffff;
    }
}

#[inline(always)]
extern "C" fn ZopfliNodeLengthCode(mut self_0: *const ZopfliNode) -> u32 {
    unsafe {
        let modifier = (*self_0).length >> 25;
        return (ZopfliNodeCopyLength(self_0))
            .wrapping_add(9)
            .wrapping_sub(modifier);
    }
}

#[inline(always)]
extern "C" fn ZopfliNodeCopyDistance(mut self_0: *const ZopfliNode) -> u32 {
    unsafe {
        return (*self_0).distance;
    }
}

#[inline(always)]
extern "C" fn ZopfliNodeDistanceCode(mut self_0: *const ZopfliNode) -> u32 {
    unsafe {
        let short_code = (*self_0).dcode_insert_length >> 27;
        return if short_code == 0 {
            (ZopfliNodeCopyDistance(self_0))
                .wrapping_add(16)
                .wrapping_sub(1)
        } else {
            short_code.wrapping_sub(1)
        };
    }
}

#[inline(always)]
extern "C" fn ZopfliNodeCommandLength(mut self_0: *const ZopfliNode) -> u32 {
    unsafe {
        return (ZopfliNodeCopyLength(self_0))
            .wrapping_add((*self_0).dcode_insert_length & 0x7ffffff);
    }
}

extern "C" fn InitZopfliCostModel(
    mut m: *mut MemoryManager,
    mut self_0: *mut ZopfliCostModel,
    mut dist: *const BrotliDistanceParams,
    mut num_bytes: u64,
) {
    unsafe {
        (*self_0).num_bytes_ = num_bytes;
        let ref mut fresh3 = (*self_0).literal_costs_;
        *fresh3 = if num_bytes.wrapping_add(2) > 0 {
            BrotliAllocate(
                m,
                num_bytes
                    .wrapping_add(2)
                    .wrapping_mul(::std::mem::size_of::<libc::c_float>() as u64),
            ) as *mut libc::c_float
        } else {
            0 as *mut libc::c_float
        };
        let ref mut fresh4 = (*self_0).cost_dist_;
        *fresh4 = if (*dist).alphabet_size_limit > 0 {
            BrotliAllocate(
                m,
                ((*dist).alphabet_size_limit as u64)
                    .wrapping_mul(::std::mem::size_of::<libc::c_float>() as u64),
            ) as *mut libc::c_float
        } else {
            0 as *mut libc::c_float
        };
        (*self_0).distance_histogram_size = (*dist).alphabet_size_limit;
        if 0 != 0 {
            return;
        }
    }
}

extern "C" fn CleanupZopfliCostModel(mut m: *mut MemoryManager, mut self_0: *mut ZopfliCostModel) {
    unsafe {
        BrotliFree(m, (*self_0).literal_costs_ as *mut libc::c_void);
        let ref mut fresh5 = (*self_0).literal_costs_;
        *fresh5 = 0 as *mut libc::c_float;
        BrotliFree(m, (*self_0).cost_dist_ as *mut libc::c_void);
        let ref mut fresh6 = (*self_0).cost_dist_;
        *fresh6 = 0 as *mut libc::c_float;
    }
}

extern "C" fn SetCost(
    mut histogram: *const u32,
    mut histogram_size: u64,
    mut literal_histogram: i32,
    mut cost: *mut libc::c_float,
) {
    unsafe {
        let mut sum = 0;
        let mut missing_symbol_sum: u64 = 0;
        let mut log2sum: libc::c_float = 0.;
        let mut missing_symbol_cost: libc::c_float = 0.;
        let mut i: u64 = 0;
        i = 0;
        while i < histogram_size {
            sum = (sum as u64).wrapping_add(*histogram.offset(i as isize) as u64) as u64;
            i = i.wrapping_add(1);
        }
        log2sum = FastLog2(sum) as libc::c_float;
        missing_symbol_sum = sum;
        if literal_histogram == 0 {
            i = 0;
            while i < histogram_size {
                if *histogram.offset(i as isize) == 0 {
                    missing_symbol_sum = missing_symbol_sum.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
        }
        missing_symbol_cost = FastLog2(missing_symbol_sum) as libc::c_float + 2 as libc::c_float;
        i = 0;
        while i < histogram_size {
            if *histogram.offset(i as isize) == 0 {
                *cost.offset(i as isize) = missing_symbol_cost;
            } else {
                *cost.offset(i as isize) =
                    log2sum - FastLog2(*histogram.offset(i as isize) as u64) as libc::c_float;
                if *cost.offset(i as isize) < 1 as libc::c_float {
                    *cost.offset(i as isize) = 1 as libc::c_float;
                }
            }
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn ZopfliCostModelSetFromCommands(
    mut self_0: *mut ZopfliCostModel,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut commands: *const Command,
    mut num_commands: u64,
    mut last_insert_len: u64,
) {
    unsafe {
        let mut histogram_literal: [u32; 256] = [0; 256];
        let mut histogram_cmd: [u32; 704] = [0; 704];
        let mut histogram_dist: [u32; 544] = [0; 544];
        let mut cost_literal: [libc::c_float; 256] = [0.; 256];
        let mut pos = position.wrapping_sub(last_insert_len);
        let mut min_cost_cmd = kInfinity;
        let mut i: u64 = 0;
        let mut cost_cmd = ((*self_0).cost_cmd_).as_mut_ptr();
        memset(
            histogram_literal.as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u32; 256]>() as u64,
        );
        memset(
            histogram_cmd.as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u32; 704]>() as u64,
        );
        memset(
            histogram_dist.as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u32; 544]>() as u64,
        );
        i = 0;
        while i < num_commands {
            let mut inslength = (*commands.offset(i as isize)).insert_len_ as u64;
            let mut copylength = CommandCopyLen(&*commands.offset(i as isize)) as u64;
            let mut distcode =
                ((*commands.offset(i as isize)).dist_prefix_ as i32 & 0x3ffi32) as u64;
            let mut cmdcode = (*commands.offset(i as isize)).cmd_prefix_ as u64;
            let mut j: u64 = 0;
            histogram_cmd[cmdcode as usize] = (histogram_cmd[cmdcode as usize]).wrapping_add(1);
            if cmdcode >= 128 {
                histogram_dist[distcode as usize] =
                    (histogram_dist[distcode as usize]).wrapping_add(1);
            }
            j = 0;
            while j < inslength {
                histogram_literal[*ringbuffer
                    .offset((pos.wrapping_add(j) & ringbuffer_mask) as isize)
                    as usize] = (histogram_literal[*ringbuffer
                    .offset((pos.wrapping_add(j) & ringbuffer_mask) as isize)
                    as usize])
                    .wrapping_add(1);
                j = j.wrapping_add(1);
            }
            pos = (pos as u64).wrapping_add(inslength.wrapping_add(copylength)) as u64;
            i = i.wrapping_add(1);
        }
        SetCost(
            histogram_literal.as_mut_ptr(),
            256,
            1,
            cost_literal.as_mut_ptr(),
        );
        SetCost(histogram_cmd.as_mut_ptr(), 704, 0, cost_cmd);
        SetCost(
            histogram_dist.as_mut_ptr(),
            (*self_0).distance_histogram_size as u64,
            0,
            (*self_0).cost_dist_,
        );
        i = 0;
        while i < 704 {
            min_cost_cmd = brotli_min_float(min_cost_cmd, *cost_cmd.offset(i as isize));
            i = i.wrapping_add(1);
        }
        (*self_0).min_cost_cmd_ = min_cost_cmd;
        let mut literal_costs = (*self_0).literal_costs_;
        let mut literal_carry = 0.0f64 as libc::c_float;
        let mut num_bytes = (*self_0).num_bytes_;
        *literal_costs.offset(0 as isize) = 0.0f64 as libc::c_float;
        i = 0;
        while i < num_bytes {
            literal_carry += cost_literal[*ringbuffer
                .offset((position.wrapping_add(i) & ringbuffer_mask) as isize)
                as usize];
            *literal_costs.offset(i.wrapping_add(1) as isize) =
                *literal_costs.offset(i as isize) + literal_carry;
            literal_carry -= *literal_costs.offset(i.wrapping_add(1) as isize)
                - *literal_costs.offset(i as isize);
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn ZopfliCostModelSetFromLiteralCosts(
    mut self_0: *mut ZopfliCostModel,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
) {
    unsafe {
        let mut literal_costs = (*self_0).literal_costs_;
        let mut literal_carry = 0.0f64 as libc::c_float;
        let mut cost_dist = (*self_0).cost_dist_;
        let mut cost_cmd = ((*self_0).cost_cmd_).as_mut_ptr();
        let mut num_bytes = (*self_0).num_bytes_;
        let mut i: u64 = 0;
        BrotliEstimateBitCostsForLiterals(
            position,
            num_bytes,
            ringbuffer_mask,
            ringbuffer,
            &mut *literal_costs.offset(1 as isize),
        );
        *literal_costs.offset(0 as isize) = 0.0f64 as libc::c_float;
        i = 0;
        while i < num_bytes {
            literal_carry += *literal_costs.offset(i.wrapping_add(1) as isize);
            *literal_costs.offset(i.wrapping_add(1) as isize) =
                *literal_costs.offset(i as isize) + literal_carry;
            literal_carry -= *literal_costs.offset(i.wrapping_add(1) as isize)
                - *literal_costs.offset(i as isize);
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < 704 {
            *cost_cmd.offset(i as isize) =
                FastLog2(11u32.wrapping_add(i as u32) as u64) as libc::c_float;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < (*self_0).distance_histogram_size as u64 {
            *cost_dist.offset(i as isize) =
                FastLog2(20u32.wrapping_add(i as u32) as u64) as libc::c_float;
            i = i.wrapping_add(1);
        }
        (*self_0).min_cost_cmd_ = FastLog2(11) as libc::c_float;
    }
}

#[inline(always)]
extern "C" fn ZopfliCostModelGetCommandCost(
    mut self_0: *const ZopfliCostModel,
    mut cmdcode: u16,
) -> libc::c_float {
    unsafe {
        return (*self_0).cost_cmd_[cmdcode as usize];
    }
}

#[inline(always)]
extern "C" fn ZopfliCostModelGetDistanceCost(
    mut self_0: *const ZopfliCostModel,
    mut distcode: u64,
) -> libc::c_float {
    unsafe {
        return *((*self_0).cost_dist_).offset(distcode as isize);
    }
}

#[inline(always)]
extern "C" fn ZopfliCostModelGetLiteralCosts(
    mut self_0: *const ZopfliCostModel,
    mut from: u64,
    mut to: u64,
) -> libc::c_float {
    unsafe {
        return *((*self_0).literal_costs_).offset(to as isize)
            - *((*self_0).literal_costs_).offset(from as isize);
    }
}

#[inline(always)]
extern "C" fn ZopfliCostModelGetMinCostCmd(mut self_0: *const ZopfliCostModel) -> libc::c_float {
    unsafe {
        return (*self_0).min_cost_cmd_;
    }
}

#[inline(always)]
extern "C" fn UpdateZopfliNode(
    mut nodes: *mut ZopfliNode,
    mut pos: u64,
    mut start_pos: u64,
    mut len: u64,
    mut len_code: u64,
    mut dist: u64,
    mut short_code: u64,
    mut cost: libc::c_float,
) {
    unsafe {
        let mut next: *mut ZopfliNode =
            &mut *nodes.offset(pos.wrapping_add(len) as isize) as *mut ZopfliNode;
        (*next).length = (len | len.wrapping_add(9u64).wrapping_sub(len_code) << 25) as u32;
        (*next).distance = dist as u32;
        (*next).dcode_insert_length = (short_code << 27 | pos.wrapping_sub(start_pos)) as u32;
        (*next).u.cost = cost;
    }
}

#[inline(always)]
extern "C" fn InitStartPosQueue(mut self_0: *mut StartPosQueue) {
    unsafe {
        (*self_0).idx_ = 0;
    }
}

extern "C" fn StartPosQueueSize(mut self_0: *const StartPosQueue) -> u64 {
    unsafe {
        return brotli_min_size_t((*self_0).idx_, 8);
    }
}

extern "C" fn StartPosQueuePush(mut self_0: *mut StartPosQueue, mut posdata: *const PosData) {
    unsafe {
        let ref mut fresh7 = (*self_0).idx_;
        let fresh8 = *fresh7;
        *fresh7 = (*fresh7).wrapping_add(1);
        let mut offset = !fresh8 & 7;
        let mut len = StartPosQueueSize(self_0);
        let mut i: u64 = 0;
        let mut q = ((*self_0).q_).as_mut_ptr();
        *q.offset(offset as isize) = *posdata;
        i = 1;
        while i < len {
            if (*q.offset((offset & 7u64) as isize)).costdiff
                > (*q.offset((offset.wrapping_add(1u64) & 7u64) as isize)).costdiff
            {
                let mut __brotli_swap_tmp = *q.offset((offset & 7u64) as isize);
                *q.offset((offset & 7u64) as isize) =
                    *q.offset((offset.wrapping_add(1u64) & 7u64) as isize);
                *q.offset((offset.wrapping_add(1u64) & 7u64) as isize) = __brotli_swap_tmp;
            }
            offset = offset.wrapping_add(1);
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn StartPosQueueAt(mut self_0: *const StartPosQueue, mut k: u64) -> *const PosData {
    unsafe {
        return &*((*self_0).q_)
            .as_ptr()
            .offset((k.wrapping_sub((*self_0).idx_) & 7u64) as isize)
            as *const PosData;
    }
}

extern "C" fn ComputeMinimumCopyLength(
    start_cost: libc::c_float,
    mut nodes: *const ZopfliNode,
    num_bytes: u64,
    pos: u64,
) -> u64 {
    unsafe {
        let mut min_cost = start_cost;
        let mut len = 2;
        let mut next_len_bucket = 4;
        let mut next_len_offset = 10;
        while pos.wrapping_add(len) <= num_bytes
            && (*nodes.offset(pos.wrapping_add(len) as isize)).u.cost <= min_cost
        {
            len = len.wrapping_add(1);
            if len == next_len_offset {
                min_cost += 1.0f32;
                next_len_offset = (next_len_offset as u64).wrapping_add(next_len_bucket) as u64;
                next_len_bucket = (next_len_bucket as u64).wrapping_mul(2) as u64;
            }
        }
        return len;
    }
}

extern "C" fn ComputeDistanceShortcut(
    block_start: u64,
    pos: u64,
    max_backward_limit: u64,
    gap: u64,
    mut nodes: *const ZopfliNode,
) -> u32 {
    unsafe {
        let clen = ZopfliNodeCopyLength(&*nodes.offset(pos as isize)) as u64;
        let ilen = ((*nodes.offset(pos as isize)).dcode_insert_length & 0x7ffffffu32) as u64;
        let dist = ZopfliNodeCopyDistance(&*nodes.offset(pos as isize)) as u64;
        if pos == 0 {
            return 0;
        } else if dist.wrapping_add(clen) <= block_start.wrapping_add(pos).wrapping_add(gap)
            && dist <= max_backward_limit.wrapping_add(gap)
            && ZopfliNodeDistanceCode(&*nodes.offset(pos as isize)) > 0
        {
            return pos as u32;
        } else {
            return (*nodes.offset(pos.wrapping_sub(clen).wrapping_sub(ilen) as isize))
                .u
                .shortcut;
        };
    }
}

extern "C" fn ComputeDistanceCache(
    pos: u64,
    mut starting_dist_cache: *const i32,
    mut nodes: *const ZopfliNode,
    mut dist_cache: *mut i32,
) {
    unsafe {
        let mut idx = 0;
        let mut p = (*nodes.offset(pos as isize)).u.shortcut as u64;
        while idx < 4 && p > 0 {
            let ilen = ((*nodes.offset(p as isize)).dcode_insert_length & 0x7ffffffu32) as u64;
            let clen = ZopfliNodeCopyLength(&*nodes.offset(p as isize)) as u64;
            let dist = ZopfliNodeCopyDistance(&*nodes.offset(p as isize)) as u64;
            let fresh9 = idx;
            idx = idx + 1;
            *dist_cache.offset(fresh9 as isize) = dist as i32;
            p = (*nodes.offset(p.wrapping_sub(clen).wrapping_sub(ilen) as isize))
                .u
                .shortcut as u64;
        }
        while idx < 4 {
            let fresh10 = starting_dist_cache;
            starting_dist_cache = starting_dist_cache.offset(1);
            *dist_cache.offset(idx as isize) = *fresh10;
            idx += 1;
        }
    }
}

extern "C" fn EvaluateNode(
    block_start: u64,
    pos: u64,
    max_backward_limit: u64,
    gap: u64,
    mut starting_dist_cache: *const i32,
    mut model: *const ZopfliCostModel,
    mut queue: *mut StartPosQueue,
    mut nodes: *mut ZopfliNode,
) {
    unsafe {
        let mut node_cost = (*nodes.offset(pos as isize)).u.cost;
        (*nodes.offset(pos as isize)).u.shortcut =
            ComputeDistanceShortcut(block_start, pos, max_backward_limit, gap, nodes);
        if node_cost <= ZopfliCostModelGetLiteralCosts(model, 0, pos) {
            let mut posdata = PosData {
                pos: 0,
                distance_cache: [0; 4],
                costdiff: 0.,
                cost: 0.,
            };
            posdata.pos = pos;
            posdata.cost = node_cost;
            posdata.costdiff = node_cost - ZopfliCostModelGetLiteralCosts(model, 0, pos);
            ComputeDistanceCache(
                pos,
                starting_dist_cache,
                nodes,
                (posdata.distance_cache).as_mut_ptr(),
            );
            StartPosQueuePush(queue, &mut posdata);
        }
    }
}

extern "C" fn UpdateNodes(
    num_bytes: u64,
    block_start: u64,
    pos: u64,
    mut ringbuffer: *const u8,
    ringbuffer_mask: u64,
    mut params: *const BrotliEncoderParams,
    max_backward_limit: u64,
    mut starting_dist_cache: *const i32,
    num_matches: u64,
    mut matches: *const BackwardMatch,
    mut model: *const ZopfliCostModel,
    mut queue: *mut StartPosQueue,
    mut nodes: *mut ZopfliNode,
) -> u64 {
    unsafe {
        let stream_offset = (*params).stream_offset;
        let cur_ix = block_start.wrapping_add(pos);
        let cur_ix_masked = cur_ix & ringbuffer_mask;
        let max_distance = brotli_min_size_t(cur_ix, max_backward_limit);
        let dictionary_start =
            brotli_min_size_t(cur_ix.wrapping_add(stream_offset), max_backward_limit);
        let max_len = num_bytes.wrapping_sub(pos);
        let max_zopfli_len = MaxZopfliLen(params);
        let max_iters = MaxZopfliCandidates(params);
        let mut min_len: u64 = 0;
        let mut result = 0;
        let mut k: u64 = 0;
        let mut gap = 0;
        EvaluateNode(
            block_start.wrapping_add(stream_offset),
            pos,
            max_backward_limit,
            gap,
            starting_dist_cache,
            model,
            queue,
            nodes,
        );
        let mut posdata = StartPosQueueAt(queue, 0);
        let mut min_cost = (*posdata).cost
            + ZopfliCostModelGetMinCostCmd(model)
            + ZopfliCostModelGetLiteralCosts(model, (*posdata).pos, pos);
        min_len = ComputeMinimumCopyLength(min_cost, nodes, num_bytes, pos);
        k = 0;
        while k < max_iters && k < StartPosQueueSize(queue) {
            let mut posdata_0 = StartPosQueueAt(queue, k);
            let start = (*posdata_0).pos;
            let inscode = GetInsertLengthCode(pos.wrapping_sub(start));
            let start_costdiff = (*posdata_0).costdiff;
            let base_cost = start_costdiff
                + GetInsertExtra(inscode) as libc::c_float
                + ZopfliCostModelGetLiteralCosts(model, 0, pos);
            let mut best_len = min_len.wrapping_sub(1);
            let mut j = 0;
            while j < 16 && best_len < max_len {
                let idx = kDistanceCacheIndex[j as usize] as u64;
                let backward = ((*posdata_0).distance_cache[idx as usize]
                    + kDistanceCacheOffset[j as usize]) as u64;
                let mut prev_ix = cur_ix.wrapping_sub(backward);
                let mut len = 0;
                let mut continuation =
                    *ringbuffer.offset(cur_ix_masked.wrapping_add(best_len) as isize);
                if cur_ix_masked.wrapping_add(best_len) > ringbuffer_mask {
                    break;
                }
                if !((backward > dictionary_start.wrapping_add(gap)) as i64 != 0) {
                    if backward <= max_distance {
                        if !(prev_ix >= cur_ix) {
                            prev_ix &= ringbuffer_mask;
                            if !(prev_ix.wrapping_add(best_len) > ringbuffer_mask
                                || continuation as i32
                                    != *ringbuffer.offset(prev_ix.wrapping_add(best_len) as isize)
                                        as i32)
                            {
                                len = FindMatchLengthWithLimit(
                                    &*ringbuffer.offset(prev_ix as isize),
                                    &*ringbuffer.offset(cur_ix_masked as isize),
                                    max_len,
                                );
                                let dist_cost =
                                    base_cost + ZopfliCostModelGetDistanceCost(model, j);
                                let mut l: u64 = 0;
                                l = best_len.wrapping_add(1);
                                while l <= len {
                                    let copycode = GetCopyLengthCode(l);
                                    let cmdcode =
                                        CombineLengthCodes(inscode, copycode, (j == 0) as i32);
                                    let cost = (if (cmdcode as i32) < 128 {
                                        base_cost
                                    } else {
                                        dist_cost
                                    }) + GetCopyExtra(copycode) as libc::c_float
                                        + ZopfliCostModelGetCommandCost(model, cmdcode);
                                    if cost < (*nodes.offset(pos.wrapping_add(l) as isize)).u.cost {
                                        UpdateZopfliNode(
                                            nodes,
                                            pos,
                                            start,
                                            l,
                                            l,
                                            backward,
                                            j.wrapping_add(1),
                                            cost,
                                        );
                                        result = brotli_max_size_t(result, l);
                                    }
                                    best_len = l;
                                    l = l.wrapping_add(1);
                                }
                            }
                        }
                    }
                }
                j = j.wrapping_add(1);
            }
            if !(k >= 2) {
                let mut len_0 = min_len;
                j = 0;
                while j < num_matches {
                    let mut match_0 = *matches.offset(j as isize);
                    let mut dist = match_0.distance as u64;
                    let mut is_dictionary_match = if dist > dictionary_start.wrapping_add(gap) {
                        1
                    } else {
                        0
                    };
                    let mut dist_code = dist.wrapping_add(16).wrapping_sub(1);
                    let mut dist_symbol: u16 = 0;
                    let mut distextra: u32 = 0;
                    let mut distnumextra: u32 = 0;
                    let mut dist_cost_0: libc::c_float = 0.;
                    let mut max_match_len: u64 = 0;
                    PrefixEncodeCopyDistance(
                        dist_code,
                        (*params).dist.num_direct_distance_codes as u64,
                        (*params).dist.distance_postfix_bits as u64,
                        &mut dist_symbol,
                        &mut distextra,
                    );
                    distnumextra = (dist_symbol as i32 >> 10i32) as u32;
                    dist_cost_0 = base_cost
                        + distnumextra as libc::c_float
                        + ZopfliCostModelGetDistanceCost(
                            model,
                            (dist_symbol as i32 & 0x3ffi32) as u64,
                        );
                    max_match_len = BackwardMatchLength(&mut match_0);
                    if len_0 < max_match_len
                        && (is_dictionary_match != 0 || max_match_len > max_zopfli_len)
                    {
                        len_0 = max_match_len;
                    }
                    while len_0 <= max_match_len {
                        let len_code = if is_dictionary_match != 0 {
                            BackwardMatchLengthCode(&mut match_0)
                        } else {
                            len_0
                        };
                        let copycode_0 = GetCopyLengthCode(len_code);
                        let cmdcode_0 = CombineLengthCodes(inscode, copycode_0, 0);
                        let cost_0 = dist_cost_0
                            + GetCopyExtra(copycode_0) as libc::c_float
                            + ZopfliCostModelGetCommandCost(model, cmdcode_0);
                        if cost_0 < (*nodes.offset(pos.wrapping_add(len_0) as isize)).u.cost {
                            UpdateZopfliNode(nodes, pos, start, len_0, len_code, dist, 0, cost_0);
                            result = brotli_max_size_t(result, len_0);
                        }
                        len_0 = len_0.wrapping_add(1);
                    }
                    j = j.wrapping_add(1);
                }
            }
            k = k.wrapping_add(1);
        }
        return result;
    }
}

extern "C" fn ComputeShortestPathFromNodes(mut num_bytes: u64, mut nodes: *mut ZopfliNode) -> u64 {
    unsafe {
        let mut index = num_bytes;
        let mut num_commands = 0;
        while (*nodes.offset(index as isize)).dcode_insert_length & 0x7ffffff == 0
            && (*nodes.offset(index as isize)).length == 1
        {
            index = index.wrapping_sub(1);
        }
        (*nodes.offset(index as isize)).u.next = !0;
        while index != 0 {
            let mut len = ZopfliNodeCommandLength(&mut *nodes.offset(index as isize)) as u64;
            index = (index as u64).wrapping_sub(len) as u64;
            (*nodes.offset(index as isize)).u.next = len as u32;
            num_commands = num_commands.wrapping_add(1);
        }
        return num_commands;
    }
}

#[no_mangle]
pub extern "C" fn BrotliZopfliCreateCommands(
    num_bytes: u64,
    block_start: u64,
    mut nodes: *const ZopfliNode,
    mut dist_cache: *mut i32,
    mut last_insert_len: *mut u64,
    mut params: *const BrotliEncoderParams,
    mut commands: *mut Command,
    mut num_literals: *mut u64,
) {
    unsafe {
        let stream_offset = (*params).stream_offset;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let mut pos = 0;
        let mut offset = (*nodes.offset(0 as isize)).u.next;
        let mut i: u64 = 0;
        let mut gap = 0;
        i = 0;
        while offset != !0 {
            let mut next: *const ZopfliNode =
                &*nodes.offset(pos.wrapping_add(offset as u64) as isize) as *const ZopfliNode;
            let mut copy_length = ZopfliNodeCopyLength(next) as u64;
            let mut insert_length = ((*next).dcode_insert_length & 0x7ffffffu32) as u64;
            pos = (pos as u64).wrapping_add(insert_length) as u64;
            offset = (*next).u.next;
            if i == 0 {
                insert_length = (insert_length as u64).wrapping_add(*last_insert_len) as u64;
                *last_insert_len = 0;
            }
            let mut distance = ZopfliNodeCopyDistance(next) as u64;
            let mut len_code = ZopfliNodeLengthCode(next) as u64;
            let mut dictionary_start = brotli_min_size_t(
                block_start.wrapping_add(pos).wrapping_add(stream_offset),
                max_backward_limit,
            );
            let mut is_dictionary = if distance > dictionary_start.wrapping_add(gap) {
                1
            } else {
                0
            };
            let mut dist_code = ZopfliNodeDistanceCode(next) as u64;
            InitCommand(
                &mut *commands.offset(i as isize),
                &(*params).dist,
                insert_length,
                copy_length,
                len_code as i32 - copy_length as i32,
                dist_code,
            );
            if is_dictionary == 0 && dist_code > 0 {
                *dist_cache.offset(3 as isize) = *dist_cache.offset(2 as isize);
                *dist_cache.offset(2 as isize) = *dist_cache.offset(1 as isize);
                *dist_cache.offset(1 as isize) = *dist_cache.offset(0 as isize);
                *dist_cache.offset(0 as isize) = distance as i32;
            }
            *num_literals = (*num_literals as u64).wrapping_add(insert_length) as u64;
            pos = (pos as u64).wrapping_add(copy_length) as u64;
            i = i.wrapping_add(1);
        }
        *last_insert_len =
            (*last_insert_len as u64).wrapping_add(num_bytes.wrapping_sub(pos)) as u64;
    }
}

extern "C" fn ZopfliIterate(
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut params: *const BrotliEncoderParams,
    gap: u64,
    mut dist_cache: *const i32,
    mut model: *const ZopfliCostModel,
    mut num_matches: *const u32,
    mut matches: *const BackwardMatch,
    mut nodes: *mut ZopfliNode,
) -> u64 {
    unsafe {
        let stream_offset = (*params).stream_offset;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let max_zopfli_len = MaxZopfliLen(params);
        let mut queue = StartPosQueue {
            q_: [PosData {
                pos: 0,
                distance_cache: [0; 4],
                costdiff: 0.,
                cost: 0.,
            }; 8],
            idx_: 0,
        };
        let mut cur_match_pos = 0;
        let mut i: u64 = 0;
        (*nodes.offset(0 as isize)).length = 0;
        (*nodes.offset(0 as isize)).u.cost = 0 as libc::c_float;
        InitStartPosQueue(&mut queue);
        i = 0;
        while i.wrapping_add(3) < num_bytes {
            let mut skip = UpdateNodes(
                num_bytes,
                position,
                i,
                ringbuffer,
                ringbuffer_mask,
                params,
                max_backward_limit,
                dist_cache,
                *num_matches.offset(i as isize) as u64,
                &*matches.offset(cur_match_pos as isize),
                model,
                &mut queue,
                nodes,
            );
            if skip < 16384 {
                skip = 0;
            }
            cur_match_pos =
                (cur_match_pos as u64).wrapping_add(*num_matches.offset(i as isize) as u64) as u64;
            if *num_matches.offset(i as isize) == 1
                && BackwardMatchLength(&*matches.offset(cur_match_pos.wrapping_sub(1) as isize))
                    > max_zopfli_len
            {
                skip = brotli_max_size_t(
                    BackwardMatchLength(&*matches.offset(cur_match_pos.wrapping_sub(1) as isize)),
                    skip,
                );
            }
            if skip > 1 {
                skip = skip.wrapping_sub(1);
                while skip != 0 {
                    i = i.wrapping_add(1);
                    if i.wrapping_add(3) >= num_bytes {
                        break;
                    }
                    EvaluateNode(
                        position.wrapping_add(stream_offset),
                        i,
                        max_backward_limit,
                        gap,
                        dist_cache,
                        model,
                        &mut queue,
                        nodes,
                    );
                    cur_match_pos = (cur_match_pos as u64)
                        .wrapping_add(*num_matches.offset(i as isize) as u64)
                        as u64;
                    skip = skip.wrapping_sub(1);
                }
            }
            i = i.wrapping_add(1);
        }
        return ComputeShortestPathFromNodes(num_bytes, nodes);
    }
}

#[no_mangle]
pub extern "C" fn BrotliZopfliComputeShortestPath(
    mut m: *mut MemoryManager,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
    mut literal_context_lut: ContextLut,
    mut params: *const BrotliEncoderParams,
    mut dist_cache: *const i32,
    mut hasher: *mut Hasher,
    mut nodes: *mut ZopfliNode,
) -> u64 {
    unsafe {
        let stream_offset = (*params).stream_offset;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let max_zopfli_len = MaxZopfliLen(params);
        let mut model = ZopfliCostModel {
            cost_cmd_: [0.; 704],
            cost_dist_: 0 as *mut libc::c_float,
            distance_histogram_size: 0,
            literal_costs_: 0 as *mut libc::c_float,
            min_cost_cmd_: 0.,
            num_bytes_: 0,
        };
        let mut queue = StartPosQueue {
            q_: [PosData {
                pos: 0,
                distance_cache: [0; 4],
                costdiff: 0.,
                cost: 0.,
            }; 8],
            idx_: 0,
        };
        let mut matches: [BackwardMatch; 384] = [BackwardMatch {
            distance: 0,
            length_and_code: 0,
        }; 384];
        let store_end = if num_bytes >= StoreLookaheadH10() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH10())
                .wrapping_add(1)
        } else {
            position
        };
        let mut i: u64 = 0;
        let mut gap = 0;
        let mut lz_matches_offset = 0;
        (*nodes.offset(0 as isize)).length = 0;
        (*nodes.offset(0 as isize)).u.cost = 0 as libc::c_float;
        InitZopfliCostModel(m, &mut model, &(*params).dist, num_bytes);
        if 0 != 0 {
            return 0;
        }
        ZopfliCostModelSetFromLiteralCosts(&mut model, position, ringbuffer, ringbuffer_mask);
        InitStartPosQueue(&mut queue);
        i = 0;
        while i.wrapping_add(HashTypeLengthH10()).wrapping_sub(1) < num_bytes {
            let pos = position.wrapping_add(i);
            let max_distance = brotli_min_size_t(pos, max_backward_limit);
            let dictionary_start =
                brotli_min_size_t(pos.wrapping_add(stream_offset), max_backward_limit);
            let mut skip: u64 = 0;
            let mut num_matches: u64 = 0;
            num_matches = FindAllMatchesH10(
                &mut (*hasher).privat._H10,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                pos,
                num_bytes.wrapping_sub(i),
                max_distance,
                dictionary_start.wrapping_add(gap),
                params,
                &mut *matches.as_mut_ptr().offset(lz_matches_offset as isize),
            );
            if num_matches > 0
                && BackwardMatchLength(
                    &mut *matches
                        .as_mut_ptr()
                        .offset(num_matches.wrapping_sub(1) as isize),
                ) > max_zopfli_len
            {
                matches[0 as usize] = matches[num_matches.wrapping_sub(1) as usize];
                num_matches = 1;
            }
            skip = UpdateNodes(
                num_bytes,
                position,
                i,
                ringbuffer,
                ringbuffer_mask,
                params,
                max_backward_limit,
                dist_cache,
                num_matches,
                matches.as_mut_ptr(),
                &mut model,
                &mut queue,
                nodes,
            );
            if skip < 16384 {
                skip = 0;
            }
            if num_matches == 1
                && BackwardMatchLength(&mut *matches.as_mut_ptr().offset(0 as isize))
                    > max_zopfli_len
            {
                skip = brotli_max_size_t(
                    BackwardMatchLength(&mut *matches.as_mut_ptr().offset(0 as isize)),
                    skip,
                );
            }
            if skip > 1 {
                StoreRangeH10(
                    &mut (*hasher).privat._H10,
                    ringbuffer,
                    ringbuffer_mask,
                    pos.wrapping_add(1),
                    brotli_min_size_t(pos.wrapping_add(skip), store_end),
                );
                skip = skip.wrapping_sub(1);
                while skip != 0 {
                    i = i.wrapping_add(1);
                    if i.wrapping_add(HashTypeLengthH10()).wrapping_sub(1) >= num_bytes {
                        break;
                    }
                    EvaluateNode(
                        position.wrapping_add(stream_offset),
                        i,
                        max_backward_limit,
                        gap,
                        dist_cache,
                        &mut model,
                        &mut queue,
                        nodes,
                    );
                    skip = skip.wrapping_sub(1);
                }
            }
            i = i.wrapping_add(1);
        }
        CleanupZopfliCostModel(m, &mut model);
        return ComputeShortestPathFromNodes(num_bytes, nodes);
    }
}

#[no_mangle]
pub extern "C" fn BrotliCreateZopfliBackwardReferences(
    mut m: *mut MemoryManager,
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
        let mut nodes = if num_bytes.wrapping_add(1) > 0 {
            BrotliAllocate(
                m,
                num_bytes
                    .wrapping_add(1)
                    .wrapping_mul(::std::mem::size_of::<ZopfliNode>() as u64),
            ) as *mut ZopfliNode
        } else {
            0 as *mut ZopfliNode
        };
        if 0 != 0 || 0 != 0 {
            return;
        }
        BrotliInitZopfliNodes(nodes, num_bytes.wrapping_add(1));
        *num_commands = (*num_commands as u64).wrapping_add(BrotliZopfliComputeShortestPath(
            m,
            num_bytes,
            position,
            ringbuffer,
            ringbuffer_mask,
            literal_context_lut,
            params,
            dist_cache,
            hasher,
            nodes,
        )) as u64;
        if 0 != 0 {
            return;
        }
        BrotliZopfliCreateCommands(
            num_bytes,
            position,
            nodes,
            dist_cache,
            last_insert_len,
            params,
            commands,
            num_literals,
        );
        BrotliFree(m, nodes as *mut libc::c_void);
        nodes = 0 as *mut ZopfliNode;
    }
}

#[no_mangle]
pub extern "C" fn BrotliCreateHqZopfliBackwardReferences(
    mut m: *mut MemoryManager,
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
        let stream_offset = (*params).stream_offset;
        let max_backward_limit = (1u64 << (*params).lgwin).wrapping_sub(16);
        let mut num_matches = if num_bytes > 0 {
            BrotliAllocate(
                m,
                num_bytes.wrapping_mul(::std::mem::size_of::<u32>() as u64),
            ) as *mut u32
        } else {
            0 as *mut u32
        };
        let mut matches_size = 4u64.wrapping_mul(num_bytes);
        let store_end = if num_bytes >= StoreLookaheadH10() {
            position
                .wrapping_add(num_bytes)
                .wrapping_sub(StoreLookaheadH10())
                .wrapping_add(1)
        } else {
            position
        };
        let mut cur_match_pos = 0;
        let mut i: u64 = 0;
        let mut orig_num_literals: u64 = 0;
        let mut orig_last_insert_len: u64 = 0;
        let mut orig_dist_cache: [i32; 4] = [0; 4];
        let mut orig_num_commands: u64 = 0;
        let mut model = ZopfliCostModel {
            cost_cmd_: [0.; 704],
            cost_dist_: 0 as *mut libc::c_float,
            distance_histogram_size: 0,
            literal_costs_: 0 as *mut libc::c_float,
            min_cost_cmd_: 0.,
            num_bytes_: 0,
        };
        let mut nodes = 0 as *mut ZopfliNode;
        let mut matches = if matches_size > 0 {
            BrotliAllocate(
                m,
                matches_size.wrapping_mul(::std::mem::size_of::<BackwardMatch>() as u64),
            ) as *mut BackwardMatch
        } else {
            0 as *mut BackwardMatch
        };
        let mut gap = 0;
        let mut shadow_matches = 0;
        if 0 != 0 || 0 != 0 || 0 != 0 {
            return;
        }
        i = 0;
        while i.wrapping_add(HashTypeLengthH10()).wrapping_sub(1) < num_bytes {
            let pos = position.wrapping_add(i);
            let mut max_distance = brotli_min_size_t(pos, max_backward_limit);
            let mut dictionary_start =
                brotli_min_size_t(pos.wrapping_add(stream_offset), max_backward_limit);
            let mut max_length = num_bytes.wrapping_sub(i);
            let mut num_found_matches: u64 = 0;
            let mut cur_match_end: u64 = 0;
            let mut j: u64 = 0;
            if matches_size < cur_match_pos.wrapping_add(128).wrapping_add(shadow_matches) {
                let mut _new_size = if matches_size == 0 {
                    cur_match_pos.wrapping_add(128).wrapping_add(shadow_matches)
                } else {
                    matches_size
                };
                let mut new_array = 0 as *mut BackwardMatch;
                while _new_size < cur_match_pos.wrapping_add(128).wrapping_add(shadow_matches) {
                    _new_size = (_new_size as u64).wrapping_mul(2) as u64;
                }
                new_array = if _new_size > 0 {
                    BrotliAllocate(
                        m,
                        _new_size.wrapping_mul(::std::mem::size_of::<BackwardMatch>() as u64),
                    ) as *mut BackwardMatch
                } else {
                    0 as *mut BackwardMatch
                };
                if 0 == 0 && 0 == 0 && matches_size != 0 {
                    memcpy(
                        new_array as *mut libc::c_void,
                        matches as *const libc::c_void,
                        matches_size.wrapping_mul(::std::mem::size_of::<BackwardMatch>() as u64),
                    );
                }
                BrotliFree(m, matches as *mut libc::c_void);
                matches = 0 as *mut BackwardMatch;
                matches = new_array;
                matches_size = _new_size;
            }
            if 0 != 0 {
                return;
            }
            num_found_matches = FindAllMatchesH10(
                &mut (*hasher).privat._H10,
                &(*params).dictionary,
                ringbuffer,
                ringbuffer_mask,
                pos,
                max_length,
                max_distance,
                dictionary_start.wrapping_add(gap),
                params,
                &mut *matches.offset(cur_match_pos.wrapping_add(shadow_matches) as isize),
            );
            cur_match_end = cur_match_pos.wrapping_add(num_found_matches);
            j = cur_match_pos;
            while j.wrapping_add(1) < cur_match_end {
                j = j.wrapping_add(1);
            }
            *num_matches.offset(i as isize) = num_found_matches as u32;
            if num_found_matches > 0 {
                let match_len = BackwardMatchLength(
                    &mut *matches.offset(cur_match_end.wrapping_sub(1) as isize),
                );
                if match_len > 325 {
                    let skip = match_len.wrapping_sub(1);
                    let fresh11 = cur_match_pos;
                    cur_match_pos = cur_match_pos.wrapping_add(1);
                    *matches.offset(fresh11 as isize) =
                        *matches.offset(cur_match_end.wrapping_sub(1) as isize);
                    *num_matches.offset(i as isize) = 1;
                    StoreRangeH10(
                        &mut (*hasher).privat._H10,
                        ringbuffer,
                        ringbuffer_mask,
                        pos.wrapping_add(1),
                        brotli_min_size_t(pos.wrapping_add(match_len), store_end),
                    );
                    memset(
                        &mut *num_matches.offset(i.wrapping_add(1) as isize) as *mut u32
                            as *mut libc::c_void,
                        0,
                        skip.wrapping_mul(::std::mem::size_of::<u32>() as u64),
                    );
                    i = (i).wrapping_add(skip) as u64;
                } else {
                    cur_match_pos = cur_match_end;
                }
            }
            i = i.wrapping_add(1);
        }
        orig_num_literals = *num_literals;
        orig_last_insert_len = *last_insert_len;
        memcpy(
            orig_dist_cache.as_mut_ptr() as *mut libc::c_void,
            dist_cache as *const libc::c_void,
            4u64.wrapping_mul(::std::mem::size_of::<i32>() as u64),
        );
        orig_num_commands = *num_commands;
        nodes = if num_bytes.wrapping_add(1) > 0 {
            BrotliAllocate(
                m,
                num_bytes
                    .wrapping_add(1)
                    .wrapping_mul(::std::mem::size_of::<ZopfliNode>() as u64),
            ) as *mut ZopfliNode
        } else {
            0 as *mut ZopfliNode
        };
        if 0 != 0 || 0 != 0 {
            return;
        }
        InitZopfliCostModel(m, &mut model, &(*params).dist, num_bytes);
        if 0 != 0 {
            return;
        }
        i = 0;
        while i < 2 {
            BrotliInitZopfliNodes(nodes, num_bytes.wrapping_add(1));
            if i == 0 {
                ZopfliCostModelSetFromLiteralCosts(
                    &mut model,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                );
            } else {
                ZopfliCostModelSetFromCommands(
                    &mut model,
                    position,
                    ringbuffer,
                    ringbuffer_mask,
                    commands,
                    (*num_commands).wrapping_sub(orig_num_commands),
                    orig_last_insert_len,
                );
            }
            *num_commands = orig_num_commands;
            *num_literals = orig_num_literals;
            *last_insert_len = orig_last_insert_len;
            memcpy(
                dist_cache as *mut libc::c_void,
                orig_dist_cache.as_mut_ptr() as *const libc::c_void,
                4u64.wrapping_mul(::std::mem::size_of::<i32>() as u64),
            );
            *num_commands = (*num_commands as u64).wrapping_add(ZopfliIterate(
                num_bytes,
                position,
                ringbuffer,
                ringbuffer_mask,
                params,
                gap,
                dist_cache,
                &mut model,
                num_matches,
                matches,
                nodes,
            )) as u64;
            BrotliZopfliCreateCommands(
                num_bytes,
                position,
                nodes,
                dist_cache,
                last_insert_len,
                params,
                commands,
                num_literals,
            );
            i = i.wrapping_add(1);
        }
        CleanupZopfliCostModel(m, &mut model);
        BrotliFree(m, nodes as *mut libc::c_void);
        nodes = 0 as *mut ZopfliNode;
        BrotliFree(m, matches as *mut libc::c_void);
        matches = 0 as *mut BackwardMatch;
        BrotliFree(m, num_matches as *mut libc::c_void);
        num_matches = 0 as *mut u32;
    }
}
