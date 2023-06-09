use libc;
extern "C" {
    static _kBrotliPrefixCodeRanges: [BrotliPrefixCodeRange; 26];
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    static _kBrotliContextLookupTable: [u8; 2048];
    static kBrotliInsBase: [u32; 24];
    static kBrotliInsExtra: [u32; 24];
    static kBrotliCopyBase: [u32; 24];
    static kBrotliCopyExtra: [u32; 24];
    fn BrotliSetDepth(p: i32, pool: *mut HuffmanTree, depth: *mut u8, max_depth: i32) -> i32;
    fn BrotliCreateHuffmanTree(
        data: *const u32,
        length: u64,
        tree_limit: i32,
        tree: *mut HuffmanTree,
        depth: *mut u8,
    );
    fn BrotliWriteHuffmanTree(
        depth: *const u8,
        num: u64,
        tree_size: *mut u64,
        tree: *mut u8,
        extra_bits_data: *mut u8,
    );
    fn BrotliConvertBitDepthsToSymbols(depth: *const u8, len: u64, bits: *mut u16);
    static kBrotliShellGaps: [u64; 6];
    fn BrotliAllocate(m: *mut MemoryManager, n: u64) -> *mut libc::c_void;
    fn BrotliFree(m: *mut MemoryManager, p: *mut libc::c_void);
}
pub type brotli_alloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
pub const CONTEXT_SIGNED: u32 = 3;
pub const CONTEXT_UTF8: u32 = 2;
pub const CONTEXT_MSB6: u32 = 1;
pub const CONTEXT_LSB6: u32 = 0;
pub type ContextLut = *const u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliPrefixCodeRange {
    pub offset: u16,
    pub nbits: u8,
}
pub const BROTLI_MODE_FONT: u32 = 2;
pub const BROTLI_MODE_TEXT: u32 = 1;
pub const BROTLI_MODE_GENERIC: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length: [u8; 32],
    pub offsets_by_length: [u32; 32],
    pub data_size: u64,
    pub data: *const u8,
}
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
pub struct HuffmanTree {
    pub total_count_: u32,
    pub index_left_: i16,
    pub index_right_or_value_: i16,
}
pub type HuffmanTreeComparator =
    Option<unsafe extern "C" fn(*const HuffmanTree, *const HuffmanTree) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub opaque: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplit {
    pub num_types: u64,
    pub num_blocks: u64,
    pub types: *mut u8,
    pub lengths: *mut u32,
    pub types_alloc_size: u64,
    pub lengths_alloc_size: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramLiteral {
    pub data_: [u32; 256],
    pub total_count_: u64,
    pub bit_cost_: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramCommand {
    pub data_: [u32; 704],
    pub total_count_: u64,
    pub bit_cost_: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HistogramDistance {
    pub data_: [u32; 544],
    pub total_count_: u64,
    pub bit_cost_: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MetaBlockSplit {
    pub literal_split: BlockSplit,
    pub command_split: BlockSplit,
    pub distance_split: BlockSplit,
    pub literal_context_map: *mut u32,
    pub literal_context_map_size: u64,
    pub distance_context_map: *mut u32,
    pub distance_context_map_size: u64,
    pub literal_histograms: *mut HistogramLiteral,
    pub literal_histograms_size: u64,
    pub command_histograms: *mut HistogramCommand,
    pub command_histograms_size: u64,
    pub distance_histograms: *mut HistogramDistance,
    pub distance_histograms_size: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockEncoder {
    pub histogram_length_: u64,
    pub num_block_types_: u64,
    pub block_types_: *const u8,
    pub block_lengths_: *const u32,
    pub num_blocks_: u64,
    pub block_split_code_: BlockSplitCode,
    pub block_ix_: u64,
    pub block_len_: u64,
    pub entropy_ix_: u64,
    pub depths_: *mut u8,
    pub bits_: *mut u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockSplitCode {
    pub type_code_calculator: BlockTypeCodeCalculator,
    pub type_depths: [u8; 258],
    pub type_bits: [u16; 258],
    pub length_depths: [u8; 26],
    pub length_bits: [u16; 26],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockTypeCodeCalculator {
    pub last_type: u64,
    pub second_last_type: u64,
}
#[inline(always)]
extern "C" fn brotli_max_uint32_t(mut a: u32, mut b: u32) -> u32 {
    return if a > b { a } else { b };
}

#[inline(always)]
extern "C" fn brotli_min_uint32_t(mut a: u32, mut b: u32) -> u32 {
    return if a < b { a } else { b };
}

#[inline(always)]
extern "C" fn BrotliUnalignedWrite64(mut p: *mut libc::c_void, mut v: u64) {
    unsafe {
        *(p as *mut u64) = v;
    }
}

#[inline(always)]
extern "C" fn Log2FloorNonZero(mut n: u64) -> u32 {
    return 31 ^ (n as u32).leading_zeros() as u32;
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
extern "C" fn GetInsertBase(mut inscode: u16) -> u32 {
    unsafe {
        return kBrotliInsBase[inscode as usize];
    }
}

#[inline(always)]
extern "C" fn GetInsertExtra(mut inscode: u16) -> u32 {
    unsafe {
        return kBrotliInsExtra[inscode as usize];
    }
}

#[inline(always)]
extern "C" fn GetCopyBase(mut copycode: u16) -> u32 {
    unsafe {
        return kBrotliCopyBase[copycode as usize];
    }
}

#[inline(always)]
extern "C" fn GetCopyExtra(mut copycode: u16) -> u32 {
    unsafe {
        return kBrotliCopyExtra[copycode as usize];
    }
}

#[inline(always)]
extern "C" fn CommandDistanceContext(mut self_0: *const Command) -> u32 {
    unsafe {
        let mut r = ((*self_0).cmd_prefix_ as i32 >> 6i32) as u32;
        let mut c = ((*self_0).cmd_prefix_ as i32 & 7i32) as u32;
        if (r == 0 || r == 2 || r == 4 || r == 7) && c <= 2 {
            return c;
        }
        return 3;
    }
}

#[inline(always)]
extern "C" fn CommandCopyLen(mut self_0: *const Command) -> u32 {
    unsafe {
        return (*self_0).copy_len_ & 0x1ffffff;
    }
}

#[inline(always)]
extern "C" fn CommandCopyLenCode(mut self_0: *const Command) -> u32 {
    unsafe {
        let mut modifier = (*self_0).copy_len_ >> 25;
        let mut delta = (modifier | (modifier & 0x40u32) << 1) as i32;
        return (((*self_0).copy_len_ & 0x1ffffffu32) as i32 + delta) as u32;
    }
}

#[inline(always)]
extern "C" fn InitHuffmanTree(
    mut self_0: *mut HuffmanTree,
    mut count: u32,
    mut left: i16,
    mut right: i16,
) {
    unsafe {
        (*self_0).total_count_ = count;
        (*self_0).index_left_ = left;
        (*self_0).index_right_or_value_ = right;
    }
}

#[inline(always)]
extern "C" fn SortHuffmanTreeItems(
    mut items: *mut HuffmanTree,
    n: u64,
    mut comparator: HuffmanTreeComparator,
) {
    unsafe {
        if n < 13 {
            let mut i: u64 = 0;
            i = 1;
            while i < n {
                let mut tmp = *items.offset(i as isize);
                let mut k = i;
                let mut j = i.wrapping_sub(1);
                while comparator.expect("non-null function pointer")(
                    &mut tmp,
                    &mut *items.offset(j as isize),
                ) != 0
                {
                    *items.offset(k as isize) = *items.offset(j as isize);
                    k = j;
                    let fresh0 = j;
                    j = j.wrapping_sub(1);
                    if fresh0 == 0 {
                        break;
                    }
                }
                *items.offset(k as isize) = tmp;
                i = i.wrapping_add(1);
            }
            return;
        } else {
            let mut g = if n < 57 { 2 } else { 0 };
            while g < 6 {
                let mut gap = kBrotliShellGaps[g as usize];
                let mut i_0: u64 = 0;
                i_0 = gap;
                while i_0 < n {
                    let mut j_0 = i_0;
                    let mut tmp_0 = *items.offset(i_0 as isize);
                    while j_0 >= gap
                        && comparator.expect("non-null function pointer")(
                            &mut tmp_0,
                            &mut *items.offset(j_0.wrapping_sub(gap) as isize),
                        ) != 0
                    {
                        *items.offset(j_0 as isize) = *items.offset(j_0.wrapping_sub(gap) as isize);
                        j_0 = (j_0 as u64).wrapping_sub(gap) as u64;
                    }
                    *items.offset(j_0 as isize) = tmp_0;
                    i_0 = i_0.wrapping_add(1);
                }
                g += 1;
            }
        };
    }
}

#[inline(always)]
extern "C" fn HistogramClearLiteral(mut self_0: *mut HistogramLiteral) {
    unsafe {
        memset(
            ((*self_0).data_).as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u32; 256]>() as u64,
        );
        (*self_0).total_count_ = 0;
        (*self_0).bit_cost_ = ::std::f64::INFINITY;
    }
}

#[inline(always)]
extern "C" fn HistogramAddLiteral(mut self_0: *mut HistogramLiteral, mut val: u64) {
    unsafe {
        let ref mut fresh1 = (*self_0).data_[val as usize];
        *fresh1 = (*fresh1).wrapping_add(1);
        let ref mut fresh2 = (*self_0).total_count_;
        *fresh2 = (*fresh2).wrapping_add(1);
    }
}

#[inline(always)]
extern "C" fn HistogramClearCommand(mut self_0: *mut HistogramCommand) {
    unsafe {
        memset(
            ((*self_0).data_).as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u32; 704]>() as u64,
        );
        (*self_0).total_count_ = 0;
        (*self_0).bit_cost_ = ::std::f64::INFINITY;
    }
}

#[inline(always)]
extern "C" fn HistogramAddCommand(mut self_0: *mut HistogramCommand, mut val: u64) {
    unsafe {
        let ref mut fresh3 = (*self_0).data_[val as usize];
        *fresh3 = (*fresh3).wrapping_add(1);
        let ref mut fresh4 = (*self_0).total_count_;
        *fresh4 = (*fresh4).wrapping_add(1);
    }
}

#[inline(always)]
extern "C" fn HistogramClearDistance(mut self_0: *mut HistogramDistance) {
    unsafe {
        memset(
            ((*self_0).data_).as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u32; 544]>() as u64,
        );
        (*self_0).total_count_ = 0;
        (*self_0).bit_cost_ = ::std::f64::INFINITY;
    }
}

#[inline(always)]
extern "C" fn HistogramAddDistance(mut self_0: *mut HistogramDistance, mut val: u64) {
    unsafe {
        let ref mut fresh5 = (*self_0).data_[val as usize];
        *fresh5 = (*fresh5).wrapping_add(1);
        let ref mut fresh6 = (*self_0).total_count_;
        *fresh6 = (*fresh6).wrapping_add(1);
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

static mut kZeroRepsDepth: [u32; 704] = [
    0, 4, 8, 7, 7, 7, 7, 7, 7, 7, 7, 11, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21, 21,
    21, 21, 21, 21, 21, 21, 21, 21, 21, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28, 28,
    28, 28, 28, 28, 28, 28,
];
static mut kZeroRepsBits: [u64; 704] = [
    0, 0, 0, 0x7, 0x17, 0x27, 0x37, 0x47, 0x57, 0x67, 0x77, 0x770, 0xb87, 0x1387, 0x1b87, 0x2387,
    0x2b87, 0x3387, 0x3b87, 0x397, 0xb97, 0x1397, 0x1b97, 0x2397, 0x2b97, 0x3397, 0x3b97, 0x3a7,
    0xba7, 0x13a7, 0x1ba7, 0x23a7, 0x2ba7, 0x33a7, 0x3ba7, 0x3b7, 0xbb7, 0x13b7, 0x1bb7, 0x23b7,
    0x2bb7, 0x33b7, 0x3bb7, 0x3c7, 0xbc7, 0x13c7, 0x1bc7, 0x23c7, 0x2bc7, 0x33c7, 0x3bc7, 0x3d7,
    0xbd7, 0x13d7, 0x1bd7, 0x23d7, 0x2bd7, 0x33d7, 0x3bd7, 0x3e7, 0xbe7, 0x13e7, 0x1be7, 0x23e7,
    0x2be7, 0x33e7, 0x3be7, 0x3f7, 0xbf7, 0x13f7, 0x1bf7, 0x23f7, 0x2bf7, 0x33f7, 0x3bf7, 0x1c387,
    0x5c387, 0x9c387, 0xdc387, 0x11c387, 0x15c387, 0x19c387, 0x1dc387, 0x1cb87, 0x5cb87, 0x9cb87,
    0xdcb87, 0x11cb87, 0x15cb87, 0x19cb87, 0x1dcb87, 0x1d387, 0x5d387, 0x9d387, 0xdd387, 0x11d387,
    0x15d387, 0x19d387, 0x1dd387, 0x1db87, 0x5db87, 0x9db87, 0xddb87, 0x11db87, 0x15db87, 0x19db87,
    0x1ddb87, 0x1e387, 0x5e387, 0x9e387, 0xde387, 0x11e387, 0x15e387, 0x19e387, 0x1de387, 0x1eb87,
    0x5eb87, 0x9eb87, 0xdeb87, 0x11eb87, 0x15eb87, 0x19eb87, 0x1deb87, 0x1f387, 0x5f387, 0x9f387,
    0xdf387, 0x11f387, 0x15f387, 0x19f387, 0x1df387, 0x1fb87, 0x5fb87, 0x9fb87, 0xdfb87, 0x11fb87,
    0x15fb87, 0x19fb87, 0x1dfb87, 0x1c397, 0x5c397, 0x9c397, 0xdc397, 0x11c397, 0x15c397, 0x19c397,
    0x1dc397, 0x1cb97, 0x5cb97, 0x9cb97, 0xdcb97, 0x11cb97, 0x15cb97, 0x19cb97, 0x1dcb97, 0x1d397,
    0x5d397, 0x9d397, 0xdd397, 0x11d397, 0x15d397, 0x19d397, 0x1dd397, 0x1db97, 0x5db97, 0x9db97,
    0xddb97, 0x11db97, 0x15db97, 0x19db97, 0x1ddb97, 0x1e397, 0x5e397, 0x9e397, 0xde397, 0x11e397,
    0x15e397, 0x19e397, 0x1de397, 0x1eb97, 0x5eb97, 0x9eb97, 0xdeb97, 0x11eb97, 0x15eb97, 0x19eb97,
    0x1deb97, 0x1f397, 0x5f397, 0x9f397, 0xdf397, 0x11f397, 0x15f397, 0x19f397, 0x1df397, 0x1fb97,
    0x5fb97, 0x9fb97, 0xdfb97, 0x11fb97, 0x15fb97, 0x19fb97, 0x1dfb97, 0x1c3a7, 0x5c3a7, 0x9c3a7,
    0xdc3a7, 0x11c3a7, 0x15c3a7, 0x19c3a7, 0x1dc3a7, 0x1cba7, 0x5cba7, 0x9cba7, 0xdcba7, 0x11cba7,
    0x15cba7, 0x19cba7, 0x1dcba7, 0x1d3a7, 0x5d3a7, 0x9d3a7, 0xdd3a7, 0x11d3a7, 0x15d3a7, 0x19d3a7,
    0x1dd3a7, 0x1dba7, 0x5dba7, 0x9dba7, 0xddba7, 0x11dba7, 0x15dba7, 0x19dba7, 0x1ddba7, 0x1e3a7,
    0x5e3a7, 0x9e3a7, 0xde3a7, 0x11e3a7, 0x15e3a7, 0x19e3a7, 0x1de3a7, 0x1eba7, 0x5eba7, 0x9eba7,
    0xdeba7, 0x11eba7, 0x15eba7, 0x19eba7, 0x1deba7, 0x1f3a7, 0x5f3a7, 0x9f3a7, 0xdf3a7, 0x11f3a7,
    0x15f3a7, 0x19f3a7, 0x1df3a7, 0x1fba7, 0x5fba7, 0x9fba7, 0xdfba7, 0x11fba7, 0x15fba7, 0x19fba7,
    0x1dfba7, 0x1c3b7, 0x5c3b7, 0x9c3b7, 0xdc3b7, 0x11c3b7, 0x15c3b7, 0x19c3b7, 0x1dc3b7, 0x1cbb7,
    0x5cbb7, 0x9cbb7, 0xdcbb7, 0x11cbb7, 0x15cbb7, 0x19cbb7, 0x1dcbb7, 0x1d3b7, 0x5d3b7, 0x9d3b7,
    0xdd3b7, 0x11d3b7, 0x15d3b7, 0x19d3b7, 0x1dd3b7, 0x1dbb7, 0x5dbb7, 0x9dbb7, 0xddbb7, 0x11dbb7,
    0x15dbb7, 0x19dbb7, 0x1ddbb7, 0x1e3b7, 0x5e3b7, 0x9e3b7, 0xde3b7, 0x11e3b7, 0x15e3b7, 0x19e3b7,
    0x1de3b7, 0x1ebb7, 0x5ebb7, 0x9ebb7, 0xdebb7, 0x11ebb7, 0x15ebb7, 0x19ebb7, 0x1debb7, 0x1f3b7,
    0x5f3b7, 0x9f3b7, 0xdf3b7, 0x11f3b7, 0x15f3b7, 0x19f3b7, 0x1df3b7, 0x1fbb7, 0x5fbb7, 0x9fbb7,
    0xdfbb7, 0x11fbb7, 0x15fbb7, 0x19fbb7, 0x1dfbb7, 0x1c3c7, 0x5c3c7, 0x9c3c7, 0xdc3c7, 0x11c3c7,
    0x15c3c7, 0x19c3c7, 0x1dc3c7, 0x1cbc7, 0x5cbc7, 0x9cbc7, 0xdcbc7, 0x11cbc7, 0x15cbc7, 0x19cbc7,
    0x1dcbc7, 0x1d3c7, 0x5d3c7, 0x9d3c7, 0xdd3c7, 0x11d3c7, 0x15d3c7, 0x19d3c7, 0x1dd3c7, 0x1dbc7,
    0x5dbc7, 0x9dbc7, 0xddbc7, 0x11dbc7, 0x15dbc7, 0x19dbc7, 0x1ddbc7, 0x1e3c7, 0x5e3c7, 0x9e3c7,
    0xde3c7, 0x11e3c7, 0x15e3c7, 0x19e3c7, 0x1de3c7, 0x1ebc7, 0x5ebc7, 0x9ebc7, 0xdebc7, 0x11ebc7,
    0x15ebc7, 0x19ebc7, 0x1debc7, 0x1f3c7, 0x5f3c7, 0x9f3c7, 0xdf3c7, 0x11f3c7, 0x15f3c7, 0x19f3c7,
    0x1df3c7, 0x1fbc7, 0x5fbc7, 0x9fbc7, 0xdfbc7, 0x11fbc7, 0x15fbc7, 0x19fbc7, 0x1dfbc7, 0x1c3d7,
    0x5c3d7, 0x9c3d7, 0xdc3d7, 0x11c3d7, 0x15c3d7, 0x19c3d7, 0x1dc3d7, 0x1cbd7, 0x5cbd7, 0x9cbd7,
    0xdcbd7, 0x11cbd7, 0x15cbd7, 0x19cbd7, 0x1dcbd7, 0x1d3d7, 0x5d3d7, 0x9d3d7, 0xdd3d7, 0x11d3d7,
    0x15d3d7, 0x19d3d7, 0x1dd3d7, 0x1dbd7, 0x5dbd7, 0x9dbd7, 0xddbd7, 0x11dbd7, 0x15dbd7, 0x19dbd7,
    0x1ddbd7, 0x1e3d7, 0x5e3d7, 0x9e3d7, 0xde3d7, 0x11e3d7, 0x15e3d7, 0x19e3d7, 0x1de3d7, 0x1ebd7,
    0x5ebd7, 0x9ebd7, 0xdebd7, 0x11ebd7, 0x15ebd7, 0x19ebd7, 0x1debd7, 0x1f3d7, 0x5f3d7, 0x9f3d7,
    0xdf3d7, 0x11f3d7, 0x15f3d7, 0x19f3d7, 0x1df3d7, 0x1fbd7, 0x5fbd7, 0x9fbd7, 0xdfbd7, 0x11fbd7,
    0x15fbd7, 0x19fbd7, 0x1dfbd7, 0x1c3e7, 0x5c3e7, 0x9c3e7, 0xdc3e7, 0x11c3e7, 0x15c3e7, 0x19c3e7,
    0x1dc3e7, 0x1cbe7, 0x5cbe7, 0x9cbe7, 0xdcbe7, 0x11cbe7, 0x15cbe7, 0x19cbe7, 0x1dcbe7, 0x1d3e7,
    0x5d3e7, 0x9d3e7, 0xdd3e7, 0x11d3e7, 0x15d3e7, 0x19d3e7, 0x1dd3e7, 0x1dbe7, 0x5dbe7, 0x9dbe7,
    0xddbe7, 0x11dbe7, 0x15dbe7, 0x19dbe7, 0x1ddbe7, 0x1e3e7, 0x5e3e7, 0x9e3e7, 0xde3e7, 0x11e3e7,
    0x15e3e7, 0x19e3e7, 0x1de3e7, 0x1ebe7, 0x5ebe7, 0x9ebe7, 0xdebe7, 0x11ebe7, 0x15ebe7, 0x19ebe7,
    0x1debe7, 0x1f3e7, 0x5f3e7, 0x9f3e7, 0xdf3e7, 0x11f3e7, 0x15f3e7, 0x19f3e7, 0x1df3e7, 0x1fbe7,
    0x5fbe7, 0x9fbe7, 0xdfbe7, 0x11fbe7, 0x15fbe7, 0x19fbe7, 0x1dfbe7, 0x1c3f7, 0x5c3f7, 0x9c3f7,
    0xdc3f7, 0x11c3f7, 0x15c3f7, 0x19c3f7, 0x1dc3f7, 0x1cbf7, 0x5cbf7, 0x9cbf7, 0xdcbf7, 0x11cbf7,
    0x15cbf7, 0x19cbf7, 0x1dcbf7, 0x1d3f7, 0x5d3f7, 0x9d3f7, 0xdd3f7, 0x11d3f7, 0x15d3f7, 0x19d3f7,
    0x1dd3f7, 0x1dbf7, 0x5dbf7, 0x9dbf7, 0xddbf7, 0x11dbf7, 0x15dbf7, 0x19dbf7, 0x1ddbf7, 0x1e3f7,
    0x5e3f7, 0x9e3f7, 0xde3f7, 0x11e3f7, 0x15e3f7, 0x19e3f7, 0x1de3f7, 0x1ebf7, 0x5ebf7, 0x9ebf7,
    0xdebf7, 0x11ebf7, 0x15ebf7, 0x19ebf7, 0x1debf7, 0x1f3f7, 0x5f3f7, 0x9f3f7, 0xdf3f7, 0x11f3f7,
    0x15f3f7, 0x19f3f7, 0x1df3f7, 0x1fbf7, 0x5fbf7, 0x9fbf7, 0xdfbf7, 0x11fbf7, 0x15fbf7, 0x19fbf7,
    0x1dfbf7, 0xe1c387, 0x2e1c387, 0x4e1c387, 0x6e1c387, 0x8e1c387, 0xae1c387, 0xce1c387,
    0xee1c387, 0xe5c387, 0x2e5c387, 0x4e5c387, 0x6e5c387, 0x8e5c387, 0xae5c387, 0xce5c387,
    0xee5c387, 0xe9c387, 0x2e9c387, 0x4e9c387, 0x6e9c387, 0x8e9c387, 0xae9c387, 0xce9c387,
    0xee9c387, 0xedc387, 0x2edc387, 0x4edc387, 0x6edc387, 0x8edc387, 0xaedc387, 0xcedc387,
    0xeedc387, 0xf1c387, 0x2f1c387, 0x4f1c387, 0x6f1c387, 0x8f1c387, 0xaf1c387, 0xcf1c387,
    0xef1c387, 0xf5c387, 0x2f5c387, 0x4f5c387, 0x6f5c387, 0x8f5c387, 0xaf5c387, 0xcf5c387,
    0xef5c387, 0xf9c387, 0x2f9c387, 0x4f9c387, 0x6f9c387, 0x8f9c387, 0xaf9c387, 0xcf9c387,
    0xef9c387, 0xfdc387, 0x2fdc387, 0x4fdc387, 0x6fdc387, 0x8fdc387, 0xafdc387, 0xcfdc387,
    0xefdc387, 0xe1cb87, 0x2e1cb87, 0x4e1cb87, 0x6e1cb87, 0x8e1cb87, 0xae1cb87, 0xce1cb87,
    0xee1cb87, 0xe5cb87, 0x2e5cb87, 0x4e5cb87, 0x6e5cb87, 0x8e5cb87, 0xae5cb87, 0xce5cb87,
    0xee5cb87, 0xe9cb87, 0x2e9cb87, 0x4e9cb87, 0x6e9cb87, 0x8e9cb87, 0xae9cb87, 0xce9cb87,
    0xee9cb87, 0xedcb87, 0x2edcb87, 0x4edcb87, 0x6edcb87, 0x8edcb87, 0xaedcb87, 0xcedcb87,
    0xeedcb87, 0xf1cb87, 0x2f1cb87, 0x4f1cb87, 0x6f1cb87, 0x8f1cb87, 0xaf1cb87, 0xcf1cb87,
    0xef1cb87, 0xf5cb87, 0x2f5cb87, 0x4f5cb87, 0x6f5cb87, 0x8f5cb87, 0xaf5cb87, 0xcf5cb87,
    0xef5cb87, 0xf9cb87, 0x2f9cb87, 0x4f9cb87, 0x6f9cb87, 0x8f9cb87,
];
static mut kCodeLengthDepth: [u8; 18] = [4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 0, 4, 4];
static mut kCodeLengthBits: [u32; 18] =
    [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 15, 31, 0, 11, 7];
#[inline(always)]
extern "C" fn BrotliWriteBitsPrepareStorage(mut pos: u64, mut array: *mut u8) {
    unsafe {
        *array.offset((pos >> 3i32) as isize) = 0;
    }
}

static mut kStaticCommandCodeDepth: [u8; 704] = [
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
];
#[inline(always)]
extern "C" fn StoreStaticCodeLengthCode(mut storage_ix: *mut u64, mut storage: *mut u8) {
    unsafe {
        BrotliWriteBits(40, 0xff << 32 | 0x55555554, storage_ix, storage);
    }
}

static mut kStaticDistanceCodeDepth: [u8; 64] = [6; 64];
static mut kNonZeroRepsBits: [u64; 704] = [
    0xb, 0x1b, 0x2b, 0x3b, 0x2cb, 0x6cb, 0xacb, 0xecb, 0x2db, 0x6db, 0xadb, 0xedb, 0x2eb, 0x6eb,
    0xaeb, 0xeeb, 0x2fb, 0x6fb, 0xafb, 0xefb, 0xb2cb, 0x1b2cb, 0x2b2cb, 0x3b2cb, 0xb6cb, 0x1b6cb,
    0x2b6cb, 0x3b6cb, 0xbacb, 0x1bacb, 0x2bacb, 0x3bacb, 0xbecb, 0x1becb, 0x2becb, 0x3becb, 0xb2db,
    0x1b2db, 0x2b2db, 0x3b2db, 0xb6db, 0x1b6db, 0x2b6db, 0x3b6db, 0xbadb, 0x1badb, 0x2badb,
    0x3badb, 0xbedb, 0x1bedb, 0x2bedb, 0x3bedb, 0xb2eb, 0x1b2eb, 0x2b2eb, 0x3b2eb, 0xb6eb, 0x1b6eb,
    0x2b6eb, 0x3b6eb, 0xbaeb, 0x1baeb, 0x2baeb, 0x3baeb, 0xbeeb, 0x1beeb, 0x2beeb, 0x3beeb, 0xb2fb,
    0x1b2fb, 0x2b2fb, 0x3b2fb, 0xb6fb, 0x1b6fb, 0x2b6fb, 0x3b6fb, 0xbafb, 0x1bafb, 0x2bafb,
    0x3bafb, 0xbefb, 0x1befb, 0x2befb, 0x3befb, 0x2cb2cb, 0x6cb2cb, 0xacb2cb, 0xecb2cb, 0x2db2cb,
    0x6db2cb, 0xadb2cb, 0xedb2cb, 0x2eb2cb, 0x6eb2cb, 0xaeb2cb, 0xeeb2cb, 0x2fb2cb, 0x6fb2cb,
    0xafb2cb, 0xefb2cb, 0x2cb6cb, 0x6cb6cb, 0xacb6cb, 0xecb6cb, 0x2db6cb, 0x6db6cb, 0xadb6cb,
    0xedb6cb, 0x2eb6cb, 0x6eb6cb, 0xaeb6cb, 0xeeb6cb, 0x2fb6cb, 0x6fb6cb, 0xafb6cb, 0xefb6cb,
    0x2cbacb, 0x6cbacb, 0xacbacb, 0xecbacb, 0x2dbacb, 0x6dbacb, 0xadbacb, 0xedbacb, 0x2ebacb,
    0x6ebacb, 0xaebacb, 0xeebacb, 0x2fbacb, 0x6fbacb, 0xafbacb, 0xefbacb, 0x2cbecb, 0x6cbecb,
    0xacbecb, 0xecbecb, 0x2dbecb, 0x6dbecb, 0xadbecb, 0xedbecb, 0x2ebecb, 0x6ebecb, 0xaebecb,
    0xeebecb, 0x2fbecb, 0x6fbecb, 0xafbecb, 0xefbecb, 0x2cb2db, 0x6cb2db, 0xacb2db, 0xecb2db,
    0x2db2db, 0x6db2db, 0xadb2db, 0xedb2db, 0x2eb2db, 0x6eb2db, 0xaeb2db, 0xeeb2db, 0x2fb2db,
    0x6fb2db, 0xafb2db, 0xefb2db, 0x2cb6db, 0x6cb6db, 0xacb6db, 0xecb6db, 0x2db6db, 0x6db6db,
    0xadb6db, 0xedb6db, 0x2eb6db, 0x6eb6db, 0xaeb6db, 0xeeb6db, 0x2fb6db, 0x6fb6db, 0xafb6db,
    0xefb6db, 0x2cbadb, 0x6cbadb, 0xacbadb, 0xecbadb, 0x2dbadb, 0x6dbadb, 0xadbadb, 0xedbadb,
    0x2ebadb, 0x6ebadb, 0xaebadb, 0xeebadb, 0x2fbadb, 0x6fbadb, 0xafbadb, 0xefbadb, 0x2cbedb,
    0x6cbedb, 0xacbedb, 0xecbedb, 0x2dbedb, 0x6dbedb, 0xadbedb, 0xedbedb, 0x2ebedb, 0x6ebedb,
    0xaebedb, 0xeebedb, 0x2fbedb, 0x6fbedb, 0xafbedb, 0xefbedb, 0x2cb2eb, 0x6cb2eb, 0xacb2eb,
    0xecb2eb, 0x2db2eb, 0x6db2eb, 0xadb2eb, 0xedb2eb, 0x2eb2eb, 0x6eb2eb, 0xaeb2eb, 0xeeb2eb,
    0x2fb2eb, 0x6fb2eb, 0xafb2eb, 0xefb2eb, 0x2cb6eb, 0x6cb6eb, 0xacb6eb, 0xecb6eb, 0x2db6eb,
    0x6db6eb, 0xadb6eb, 0xedb6eb, 0x2eb6eb, 0x6eb6eb, 0xaeb6eb, 0xeeb6eb, 0x2fb6eb, 0x6fb6eb,
    0xafb6eb, 0xefb6eb, 0x2cbaeb, 0x6cbaeb, 0xacbaeb, 0xecbaeb, 0x2dbaeb, 0x6dbaeb, 0xadbaeb,
    0xedbaeb, 0x2ebaeb, 0x6ebaeb, 0xaebaeb, 0xeebaeb, 0x2fbaeb, 0x6fbaeb, 0xafbaeb, 0xefbaeb,
    0x2cbeeb, 0x6cbeeb, 0xacbeeb, 0xecbeeb, 0x2dbeeb, 0x6dbeeb, 0xadbeeb, 0xedbeeb, 0x2ebeeb,
    0x6ebeeb, 0xaebeeb, 0xeebeeb, 0x2fbeeb, 0x6fbeeb, 0xafbeeb, 0xefbeeb, 0x2cb2fb, 0x6cb2fb,
    0xacb2fb, 0xecb2fb, 0x2db2fb, 0x6db2fb, 0xadb2fb, 0xedb2fb, 0x2eb2fb, 0x6eb2fb, 0xaeb2fb,
    0xeeb2fb, 0x2fb2fb, 0x6fb2fb, 0xafb2fb, 0xefb2fb, 0x2cb6fb, 0x6cb6fb, 0xacb6fb, 0xecb6fb,
    0x2db6fb, 0x6db6fb, 0xadb6fb, 0xedb6fb, 0x2eb6fb, 0x6eb6fb, 0xaeb6fb, 0xeeb6fb, 0x2fb6fb,
    0x6fb6fb, 0xafb6fb, 0xefb6fb, 0x2cbafb, 0x6cbafb, 0xacbafb, 0xecbafb, 0x2dbafb, 0x6dbafb,
    0xadbafb, 0xedbafb, 0x2ebafb, 0x6ebafb, 0xaebafb, 0xeebafb, 0x2fbafb, 0x6fbafb, 0xafbafb,
    0xefbafb, 0x2cbefb, 0x6cbefb, 0xacbefb, 0xecbefb, 0x2dbefb, 0x6dbefb, 0xadbefb, 0xedbefb,
    0x2ebefb, 0x6ebefb, 0xaebefb, 0xeebefb, 0x2fbefb, 0x6fbefb, 0xafbefb, 0xefbefb, 0xb2cb2cb,
    0x1b2cb2cb, 0x2b2cb2cb, 0x3b2cb2cb, 0xb6cb2cb, 0x1b6cb2cb, 0x2b6cb2cb, 0x3b6cb2cb, 0xbacb2cb,
    0x1bacb2cb, 0x2bacb2cb, 0x3bacb2cb, 0xbecb2cb, 0x1becb2cb, 0x2becb2cb, 0x3becb2cb, 0xb2db2cb,
    0x1b2db2cb, 0x2b2db2cb, 0x3b2db2cb, 0xb6db2cb, 0x1b6db2cb, 0x2b6db2cb, 0x3b6db2cb, 0xbadb2cb,
    0x1badb2cb, 0x2badb2cb, 0x3badb2cb, 0xbedb2cb, 0x1bedb2cb, 0x2bedb2cb, 0x3bedb2cb, 0xb2eb2cb,
    0x1b2eb2cb, 0x2b2eb2cb, 0x3b2eb2cb, 0xb6eb2cb, 0x1b6eb2cb, 0x2b6eb2cb, 0x3b6eb2cb, 0xbaeb2cb,
    0x1baeb2cb, 0x2baeb2cb, 0x3baeb2cb, 0xbeeb2cb, 0x1beeb2cb, 0x2beeb2cb, 0x3beeb2cb, 0xb2fb2cb,
    0x1b2fb2cb, 0x2b2fb2cb, 0x3b2fb2cb, 0xb6fb2cb, 0x1b6fb2cb, 0x2b6fb2cb, 0x3b6fb2cb, 0xbafb2cb,
    0x1bafb2cb, 0x2bafb2cb, 0x3bafb2cb, 0xbefb2cb, 0x1befb2cb, 0x2befb2cb, 0x3befb2cb, 0xb2cb6cb,
    0x1b2cb6cb, 0x2b2cb6cb, 0x3b2cb6cb, 0xb6cb6cb, 0x1b6cb6cb, 0x2b6cb6cb, 0x3b6cb6cb, 0xbacb6cb,
    0x1bacb6cb, 0x2bacb6cb, 0x3bacb6cb, 0xbecb6cb, 0x1becb6cb, 0x2becb6cb, 0x3becb6cb, 0xb2db6cb,
    0x1b2db6cb, 0x2b2db6cb, 0x3b2db6cb, 0xb6db6cb, 0x1b6db6cb, 0x2b6db6cb, 0x3b6db6cb, 0xbadb6cb,
    0x1badb6cb, 0x2badb6cb, 0x3badb6cb, 0xbedb6cb, 0x1bedb6cb, 0x2bedb6cb, 0x3bedb6cb, 0xb2eb6cb,
    0x1b2eb6cb, 0x2b2eb6cb, 0x3b2eb6cb, 0xb6eb6cb, 0x1b6eb6cb, 0x2b6eb6cb, 0x3b6eb6cb, 0xbaeb6cb,
    0x1baeb6cb, 0x2baeb6cb, 0x3baeb6cb, 0xbeeb6cb, 0x1beeb6cb, 0x2beeb6cb, 0x3beeb6cb, 0xb2fb6cb,
    0x1b2fb6cb, 0x2b2fb6cb, 0x3b2fb6cb, 0xb6fb6cb, 0x1b6fb6cb, 0x2b6fb6cb, 0x3b6fb6cb, 0xbafb6cb,
    0x1bafb6cb, 0x2bafb6cb, 0x3bafb6cb, 0xbefb6cb, 0x1befb6cb, 0x2befb6cb, 0x3befb6cb, 0xb2cbacb,
    0x1b2cbacb, 0x2b2cbacb, 0x3b2cbacb, 0xb6cbacb, 0x1b6cbacb, 0x2b6cbacb, 0x3b6cbacb, 0xbacbacb,
    0x1bacbacb, 0x2bacbacb, 0x3bacbacb, 0xbecbacb, 0x1becbacb, 0x2becbacb, 0x3becbacb, 0xb2dbacb,
    0x1b2dbacb, 0x2b2dbacb, 0x3b2dbacb, 0xb6dbacb, 0x1b6dbacb, 0x2b6dbacb, 0x3b6dbacb, 0xbadbacb,
    0x1badbacb, 0x2badbacb, 0x3badbacb, 0xbedbacb, 0x1bedbacb, 0x2bedbacb, 0x3bedbacb, 0xb2ebacb,
    0x1b2ebacb, 0x2b2ebacb, 0x3b2ebacb, 0xb6ebacb, 0x1b6ebacb, 0x2b6ebacb, 0x3b6ebacb, 0xbaebacb,
    0x1baebacb, 0x2baebacb, 0x3baebacb, 0xbeebacb, 0x1beebacb, 0x2beebacb, 0x3beebacb, 0xb2fbacb,
    0x1b2fbacb, 0x2b2fbacb, 0x3b2fbacb, 0xb6fbacb, 0x1b6fbacb, 0x2b6fbacb, 0x3b6fbacb, 0xbafbacb,
    0x1bafbacb, 0x2bafbacb, 0x3bafbacb, 0xbefbacb, 0x1befbacb, 0x2befbacb, 0x3befbacb, 0xb2cbecb,
    0x1b2cbecb, 0x2b2cbecb, 0x3b2cbecb, 0xb6cbecb, 0x1b6cbecb, 0x2b6cbecb, 0x3b6cbecb, 0xbacbecb,
    0x1bacbecb, 0x2bacbecb, 0x3bacbecb, 0xbecbecb, 0x1becbecb, 0x2becbecb, 0x3becbecb, 0xb2dbecb,
    0x1b2dbecb, 0x2b2dbecb, 0x3b2dbecb, 0xb6dbecb, 0x1b6dbecb, 0x2b6dbecb, 0x3b6dbecb, 0xbadbecb,
    0x1badbecb, 0x2badbecb, 0x3badbecb, 0xbedbecb, 0x1bedbecb, 0x2bedbecb, 0x3bedbecb, 0xb2ebecb,
    0x1b2ebecb, 0x2b2ebecb, 0x3b2ebecb, 0xb6ebecb, 0x1b6ebecb, 0x2b6ebecb, 0x3b6ebecb, 0xbaebecb,
    0x1baebecb, 0x2baebecb, 0x3baebecb, 0xbeebecb, 0x1beebecb, 0x2beebecb, 0x3beebecb, 0xb2fbecb,
    0x1b2fbecb, 0x2b2fbecb, 0x3b2fbecb, 0xb6fbecb, 0x1b6fbecb, 0x2b6fbecb, 0x3b6fbecb, 0xbafbecb,
    0x1bafbecb, 0x2bafbecb, 0x3bafbecb, 0xbefbecb, 0x1befbecb, 0x2befbecb, 0x3befbecb, 0xb2cb2db,
    0x1b2cb2db, 0x2b2cb2db, 0x3b2cb2db, 0xb6cb2db, 0x1b6cb2db, 0x2b6cb2db, 0x3b6cb2db, 0xbacb2db,
    0x1bacb2db, 0x2bacb2db, 0x3bacb2db, 0xbecb2db, 0x1becb2db, 0x2becb2db, 0x3becb2db, 0xb2db2db,
    0x1b2db2db, 0x2b2db2db, 0x3b2db2db, 0xb6db2db, 0x1b6db2db, 0x2b6db2db, 0x3b6db2db, 0xbadb2db,
    0x1badb2db, 0x2badb2db, 0x3badb2db, 0xbedb2db, 0x1bedb2db, 0x2bedb2db, 0x3bedb2db, 0xb2eb2db,
    0x1b2eb2db, 0x2b2eb2db, 0x3b2eb2db, 0xb6eb2db, 0x1b6eb2db, 0x2b6eb2db, 0x3b6eb2db, 0xbaeb2db,
    0x1baeb2db, 0x2baeb2db, 0x3baeb2db, 0xbeeb2db, 0x1beeb2db, 0x2beeb2db, 0x3beeb2db, 0xb2fb2db,
    0x1b2fb2db, 0x2b2fb2db, 0x3b2fb2db, 0xb6fb2db, 0x1b6fb2db, 0x2b6fb2db, 0x3b6fb2db, 0xbafb2db,
    0x1bafb2db, 0x2bafb2db, 0x3bafb2db, 0xbefb2db, 0x1befb2db, 0x2befb2db, 0x3befb2db, 0xb2cb6db,
    0x1b2cb6db, 0x2b2cb6db, 0x3b2cb6db, 0xb6cb6db, 0x1b6cb6db, 0x2b6cb6db, 0x3b6cb6db, 0xbacb6db,
    0x1bacb6db, 0x2bacb6db, 0x3bacb6db, 0xbecb6db, 0x1becb6db, 0x2becb6db, 0x3becb6db, 0xb2db6db,
    0x1b2db6db, 0x2b2db6db, 0x3b2db6db, 0xb6db6db, 0x1b6db6db, 0x2b6db6db, 0x3b6db6db, 0xbadb6db,
    0x1badb6db, 0x2badb6db, 0x3badb6db, 0xbedb6db, 0x1bedb6db, 0x2bedb6db, 0x3bedb6db, 0xb2eb6db,
    0x1b2eb6db, 0x2b2eb6db, 0x3b2eb6db, 0xb6eb6db, 0x1b6eb6db, 0x2b6eb6db, 0x3b6eb6db, 0xbaeb6db,
    0x1baeb6db, 0x2baeb6db, 0x3baeb6db,
];
static mut kNonZeroRepsDepth: [u32; 704] = [
    6, 6, 6, 6, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 18, 18, 18, 18, 18,
    18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18,
    18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18,
    18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 18, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24, 24,
    24, 24, 24, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
    30, 30, 30, 30, 30, 30, 30,
];
static mut kStaticCommandCodeBits: [u16; 704] = [
    0, 256, 128, 384, 64, 320, 192, 448, 32, 288, 160, 416, 96, 352, 224, 480, 16, 272, 144, 400,
    80, 336, 208, 464, 48, 304, 176, 432, 112, 368, 240, 496, 8, 264, 136, 392, 72, 328, 200, 456,
    40, 296, 168, 424, 104, 360, 232, 488, 24, 280, 152, 408, 88, 344, 216, 472, 56, 312, 184, 440,
    120, 376, 248, 504, 4, 260, 132, 388, 68, 324, 196, 452, 36, 292, 164, 420, 100, 356, 228, 484,
    20, 276, 148, 404, 84, 340, 212, 468, 52, 308, 180, 436, 116, 372, 244, 500, 12, 268, 140, 396,
    76, 332, 204, 460, 44, 300, 172, 428, 108, 364, 236, 492, 28, 284, 156, 412, 92, 348, 220, 476,
    60, 316, 188, 444, 124, 380, 252, 508, 2, 258, 130, 386, 66, 322, 194, 450, 34, 290, 162, 418,
    98, 354, 226, 482, 18, 274, 146, 402, 82, 338, 210, 466, 50, 306, 178, 434, 114, 370, 242, 498,
    10, 266, 138, 394, 74, 330, 202, 458, 42, 298, 170, 426, 106, 362, 234, 490, 26, 282, 154, 410,
    90, 346, 218, 474, 58, 314, 186, 442, 122, 378, 250, 506, 6, 262, 134, 390, 70, 326, 198, 454,
    38, 294, 166, 422, 102, 358, 230, 486, 22, 278, 150, 406, 86, 342, 214, 470, 54, 310, 182, 438,
    118, 374, 246, 502, 14, 270, 142, 398, 78, 334, 206, 462, 46, 302, 174, 430, 110, 366, 238,
    494, 30, 286, 158, 414, 94, 350, 222, 478, 62, 318, 190, 446, 126, 382, 254, 510, 1, 257, 129,
    385, 65, 321, 193, 449, 33, 289, 161, 417, 97, 353, 225, 481, 17, 273, 145, 401, 81, 337, 209,
    465, 49, 305, 177, 433, 113, 369, 241, 497, 9, 265, 137, 393, 73, 329, 201, 457, 41, 297, 169,
    425, 105, 361, 233, 489, 25, 281, 153, 409, 89, 345, 217, 473, 57, 313, 185, 441, 121, 377,
    249, 505, 5, 261, 133, 389, 69, 325, 197, 453, 37, 293, 165, 421, 101, 357, 229, 485, 21, 277,
    149, 405, 85, 341, 213, 469, 53, 309, 181, 437, 117, 373, 245, 501, 13, 269, 141, 397, 77, 333,
    205, 461, 45, 301, 173, 429, 109, 365, 237, 493, 29, 285, 157, 413, 93, 349, 221, 477, 61, 317,
    189, 445, 125, 381, 253, 509, 3, 259, 131, 387, 67, 323, 195, 451, 35, 291, 163, 419, 99, 355,
    227, 483, 19, 275, 147, 403, 83, 339, 211, 467, 51, 307, 179, 435, 115, 371, 243, 499, 11, 267,
    139, 395, 75, 331, 203, 459, 43, 299, 171, 427, 107, 363, 235, 491, 27, 283, 155, 411, 91, 347,
    219, 475, 59, 315, 187, 443, 123, 379, 251, 507, 7, 1031, 519, 1543, 263, 1287, 775, 1799, 135,
    1159, 647, 1671, 391, 1415, 903, 1927, 71, 1095, 583, 1607, 327, 1351, 839, 1863, 199, 1223,
    711, 1735, 455, 1479, 967, 1991, 39, 1063, 551, 1575, 295, 1319, 807, 1831, 167, 1191, 679,
    1703, 423, 1447, 935, 1959, 103, 1127, 615, 1639, 359, 1383, 871, 1895, 231, 1255, 743, 1767,
    487, 1511, 999, 2023, 23, 1047, 535, 1559, 279, 1303, 791, 1815, 151, 1175, 663, 1687, 407,
    1431, 919, 1943, 87, 1111, 599, 1623, 343, 1367, 855, 1879, 215, 1239, 727, 1751, 471, 1495,
    983, 2007, 55, 1079, 567, 1591, 311, 1335, 823, 1847, 183, 1207, 695, 1719, 439, 1463, 951,
    1975, 119, 1143, 631, 1655, 375, 1399, 887, 1911, 247, 1271, 759, 1783, 503, 1527, 1015, 2039,
    15, 1039, 527, 1551, 271, 1295, 783, 1807, 143, 1167, 655, 1679, 399, 1423, 911, 1935, 79,
    1103, 591, 1615, 335, 1359, 847, 1871, 207, 1231, 719, 1743, 463, 1487, 975, 1999, 47, 1071,
    559, 1583, 303, 1327, 815, 1839, 175, 1199, 687, 1711, 431, 1455, 943, 1967, 111, 1135, 623,
    1647, 367, 1391, 879, 1903, 239, 1263, 751, 1775, 495, 1519, 1007, 2031, 31, 1055, 543, 1567,
    287, 1311, 799, 1823, 159, 1183, 671, 1695, 415, 1439, 927, 1951, 95, 1119, 607, 1631, 351,
    1375, 863, 1887, 223, 1247, 735, 1759, 479, 1503, 991, 2015, 63, 1087, 575, 1599, 319, 1343,
    831, 1855, 191, 1215, 703, 1727, 447, 1471, 959, 1983, 127, 1151, 639, 1663, 383, 1407, 895,
    1919, 255, 1279, 767, 1791, 511, 1535, 1023, 2047,
];
#[inline(always)]
extern "C" fn StoreStaticCommandHuffmanTree(mut storage_ix: *mut u64, mut storage: *mut u8) {
    unsafe {
        BrotliWriteBits(56, 0x926244 << 32 | 0x16307003, storage_ix, storage);
        BrotliWriteBits(3, 0, storage_ix, storage);
    }
}

static mut kStaticDistanceCodeBits: [u16; 64] = [
    0, 32, 16, 48, 8, 40, 24, 56, 4, 36, 20, 52, 12, 44, 28, 60, 2, 34, 18, 50, 10, 42, 26, 58, 6,
    38, 22, 54, 14, 46, 30, 62, 1, 33, 17, 49, 9, 41, 25, 57, 5, 37, 21, 53, 13, 45, 29, 61, 3, 35,
    19, 51, 11, 43, 27, 59, 7, 39, 23, 55, 15, 47, 31, 63,
];
#[inline(always)]
extern "C" fn StoreStaticDistanceHuffmanTree(mut storage_ix: *mut u64, mut storage: *mut u8) {
    unsafe {
        BrotliWriteBits(28, 0x369dc03, storage_ix, storage);
    }
}

#[inline(always)]
extern "C" fn BlockLengthPrefixCode(mut len: u32) -> u32 {
    let mut code = (if len >= 177 {
        if len >= 753 {
            20
        } else {
            14
        }
    } else if len >= 41 {
        7
    } else {
        0
    }) as u32;
    unsafe {
        while code < (26 - 1i32) as u32
            && len >= _kBrotliPrefixCodeRanges[code.wrapping_add(1) as usize].offset as u32
        {
            code = code.wrapping_add(1);
        }
    }
    return code;
}

#[inline(always)]
extern "C" fn GetBlockLengthPrefixCode(
    mut len: u32,
    mut code: *mut u64,
    mut n_extra: *mut u32,
    mut extra: *mut u32,
) {
    unsafe {
        *code = BlockLengthPrefixCode(len) as u64;
        *n_extra = _kBrotliPrefixCodeRanges[*code as usize].nbits as u32;
        *extra = len.wrapping_sub(_kBrotliPrefixCodeRanges[*code as usize].offset as u32);
    }
}

extern "C" fn InitBlockTypeCodeCalculator(mut self_0: *mut BlockTypeCodeCalculator) {
    unsafe {
        (*self_0).last_type = 1;
        (*self_0).second_last_type = 0;
    }
}

#[inline(always)]
extern "C" fn NextBlockTypeCode(
    mut calculator: *mut BlockTypeCodeCalculator,
    mut type_0: u8,
) -> u64 {
    unsafe {
        let mut type_code = (if type_0 as u64 == ((*calculator).last_type).wrapping_add(1) {
            1
        } else if type_0 as u64 == (*calculator).second_last_type {
            0
        } else {
            (type_0 as u32).wrapping_add(2)
        }) as u64;
        (*calculator).second_last_type = (*calculator).last_type;
        (*calculator).last_type = type_0 as u64;
        return type_code;
    }
}

extern "C" fn BrotliEncodeMlen(
    mut length: u64,
    mut bits: *mut u64,
    mut numbits: *mut u64,
    mut nibblesbits: *mut u64,
) {
    unsafe {
        let mut lg = (if length == 1 {
            1
        } else {
            (Log2FloorNonZero(length.wrapping_sub(1u64) as u64)).wrapping_add(1)
        }) as u64;
        let mut mnibbles = (if lg < 16u64 { 16 } else { lg.wrapping_add(3) }).wrapping_div(4);
        *nibblesbits = mnibbles.wrapping_sub(4);
        *numbits = mnibbles.wrapping_mul(4);
        *bits = length.wrapping_sub(1);
    }
}

#[inline(always)]
extern "C" fn StoreCommandExtra(
    mut cmd: *const Command,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut copylen_code = CommandCopyLenCode(cmd);
        let mut inscode = GetInsertLengthCode((*cmd).insert_len_ as u64);
        let mut copycode = GetCopyLengthCode(copylen_code as u64);
        let mut insnumextra = GetInsertExtra(inscode);
        let mut insextraval = ((*cmd).insert_len_).wrapping_sub(GetInsertBase(inscode)) as u64;
        let mut copyextraval = copylen_code.wrapping_sub(GetCopyBase(copycode)) as u64;
        let mut bits = copyextraval << insnumextra | insextraval;
        BrotliWriteBits(
            insnumextra.wrapping_add(GetCopyExtra(copycode)) as u64,
            bits,
            storage_ix,
            storage,
        );
    }
}

extern "C" fn StoreVarLenUint8(mut n: u64, mut storage_ix: *mut u64, mut storage: *mut u8) {
    unsafe {
        if n == 0 {
            BrotliWriteBits(1, 0, storage_ix, storage);
        } else {
            let mut nbits = Log2FloorNonZero(n) as u64;
            BrotliWriteBits(1, 1, storage_ix, storage);
            BrotliWriteBits(3, nbits, storage_ix, storage);
            BrotliWriteBits(nbits, n.wrapping_sub(1 << nbits), storage_ix, storage);
        };
    }
}

extern "C" fn StoreCompressedMetaBlockHeader(
    mut is_final_block: i32,
    mut length: u64,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut lenbits: u64 = 0;
        let mut nlenbits: u64 = 0;
        let mut nibblesbits: u64 = 0;
        BrotliWriteBits(1, is_final_block as u64, storage_ix, storage);
        if is_final_block != 0 {
            BrotliWriteBits(1, 0, storage_ix, storage);
        }
        BrotliEncodeMlen(length, &mut lenbits, &mut nlenbits, &mut nibblesbits);
        BrotliWriteBits(2, nibblesbits, storage_ix, storage);
        BrotliWriteBits(nlenbits, lenbits, storage_ix, storage);
        if is_final_block == 0 {
            BrotliWriteBits(1, 0, storage_ix, storage);
        }
    }
}

extern "C" fn BrotliStoreUncompressedMetaBlockHeader(
    mut length: u64,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut lenbits: u64 = 0;
        let mut nlenbits: u64 = 0;
        let mut nibblesbits: u64 = 0;
        BrotliWriteBits(1, 0, storage_ix, storage);
        BrotliEncodeMlen(length, &mut lenbits, &mut nlenbits, &mut nibblesbits);
        BrotliWriteBits(2, nibblesbits, storage_ix, storage);
        BrotliWriteBits(nlenbits, lenbits, storage_ix, storage);
        BrotliWriteBits(1, 1, storage_ix, storage);
    }
}

extern "C" fn BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
    num_codes: i32,
    mut code_length_bitdepth: *const u8,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        static mut kStorageOrder: [u8; 18] =
            [1, 2, 3, 4, 0, 5, 17, 6, 16, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        static mut kHuffmanBitLengthHuffmanCodeSymbols: [u8; 6] = [0, 7, 3, 2, 1, 15];
        static mut kHuffmanBitLengthHuffmanCodeBitLengths: [u8; 6] = [2, 4, 3, 2, 2, 4];
        let mut skip_some = 0;
        let mut codes_to_store = (17 + 1i32) as u64;
        if num_codes > 1 {
            while codes_to_store > 0 {
                if *code_length_bitdepth
                    .offset(kStorageOrder[codes_to_store.wrapping_sub(1) as usize] as isize)
                    as i32
                    != 0
                {
                    break;
                }
                codes_to_store = codes_to_store.wrapping_sub(1);
            }
        }
        if *code_length_bitdepth.offset(kStorageOrder[0 as usize] as isize) as i32 == 0
            && *code_length_bitdepth.offset(kStorageOrder[1 as usize] as isize) as i32 == 0
        {
            skip_some = 2;
            if *code_length_bitdepth.offset(kStorageOrder[2 as usize] as isize) as i32 == 0 {
                skip_some = 3;
            }
        }
        BrotliWriteBits(2, skip_some, storage_ix, storage);
        let mut i: u64 = 0;
        i = skip_some;
        while i < codes_to_store {
            let mut l = *code_length_bitdepth.offset(kStorageOrder[i as usize] as isize) as u64;
            BrotliWriteBits(
                kHuffmanBitLengthHuffmanCodeBitLengths[l as usize] as u64,
                kHuffmanBitLengthHuffmanCodeSymbols[l as usize] as u64,
                storage_ix,
                storage,
            );
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn BrotliStoreHuffmanTreeToBitMask(
    huffman_tree_size: u64,
    mut huffman_tree: *const u8,
    mut huffman_tree_extra_bits: *const u8,
    mut code_length_bitdepth: *const u8,
    mut code_length_bitdepth_symbols: *const u16,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i < huffman_tree_size {
            let mut ix = *huffman_tree.offset(i as isize) as u64;
            BrotliWriteBits(
                *code_length_bitdepth.offset(ix as isize) as u64,
                *code_length_bitdepth_symbols.offset(ix as isize) as u64,
                storage_ix,
                storage,
            );
            match ix {
                16 => {
                    BrotliWriteBits(
                        2,
                        *huffman_tree_extra_bits.offset(i as isize) as u64,
                        storage_ix,
                        storage,
                    );
                }
                17 => {
                    BrotliWriteBits(
                        3,
                        *huffman_tree_extra_bits.offset(i as isize) as u64,
                        storage_ix,
                        storage,
                    );
                }
                _ => {}
            }
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn StoreSimpleHuffmanTree(
    mut depths: *const u8,
    mut symbols: *mut u64,
    mut num_symbols: u64,
    mut max_bits: u64,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        BrotliWriteBits(2, 1, storage_ix, storage);
        BrotliWriteBits(2, num_symbols.wrapping_sub(1), storage_ix, storage);
        let mut i: u64 = 0;
        i = 0;
        while i < num_symbols {
            let mut j: u64 = 0;
            j = i.wrapping_add(1);
            while j < num_symbols {
                if (*depths.offset(*symbols.offset(j as isize) as isize) as i32)
                    < *depths.offset(*symbols.offset(i as isize) as isize) as i32
                {
                    let mut __brotli_swap_tmp = *symbols.offset(j as isize);
                    *symbols.offset(j as isize) = *symbols.offset(i as isize);
                    *symbols.offset(i as isize) = __brotli_swap_tmp;
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        if num_symbols == 2 {
            BrotliWriteBits(max_bits, *symbols.offset(0 as isize), storage_ix, storage);
            BrotliWriteBits(max_bits, *symbols.offset(1 as isize), storage_ix, storage);
        } else if num_symbols == 3 {
            BrotliWriteBits(max_bits, *symbols.offset(0 as isize), storage_ix, storage);
            BrotliWriteBits(max_bits, *symbols.offset(1 as isize), storage_ix, storage);
            BrotliWriteBits(max_bits, *symbols.offset(2 as isize), storage_ix, storage);
        } else {
            BrotliWriteBits(max_bits, *symbols.offset(0 as isize), storage_ix, storage);
            BrotliWriteBits(max_bits, *symbols.offset(1 as isize), storage_ix, storage);
            BrotliWriteBits(max_bits, *symbols.offset(2 as isize), storage_ix, storage);
            BrotliWriteBits(max_bits, *symbols.offset(3 as isize), storage_ix, storage);
            BrotliWriteBits(
                1,
                (if *depths.offset(*symbols.offset(0 as isize) as isize) as i32 == 1 {
                    1
                } else {
                    0
                }) as u64,
                storage_ix,
                storage,
            );
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliStoreHuffmanTree(
    mut depths: *const u8,
    mut num: u64,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut huffman_tree: [u8; 704] = [0; 704];
        let mut huffman_tree_extra_bits: [u8; 704] = [0; 704];
        let mut huffman_tree_size = 0;
        let mut code_length_bitdepth: [u8; 18] = [0; 18];
        let mut code_length_bitdepth_symbols: [u16; 18] = [0; 18];
        let mut huffman_tree_histogram: [u32; 18] = [0; 18];
        let mut i: u64 = 0;
        let mut num_codes = 0;
        let mut code = 0;
        BrotliWriteHuffmanTree(
            depths,
            num,
            &mut huffman_tree_size,
            huffman_tree.as_mut_ptr(),
            huffman_tree_extra_bits.as_mut_ptr(),
        );
        i = 0;
        while i < huffman_tree_size {
            huffman_tree_histogram[huffman_tree[i as usize] as usize] =
                (huffman_tree_histogram[huffman_tree[i as usize] as usize]).wrapping_add(1);
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < (17 + 1i32) as u64 {
            if huffman_tree_histogram[i as usize] != 0 {
                if num_codes == 0 {
                    code = i;
                    num_codes = 1;
                } else if num_codes == 1 {
                    num_codes = 2;
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        BrotliCreateHuffmanTree(
            huffman_tree_histogram.as_mut_ptr(),
            (17 + 1i32) as u64,
            5,
            tree,
            code_length_bitdepth.as_mut_ptr(),
        );
        BrotliConvertBitDepthsToSymbols(
            code_length_bitdepth.as_mut_ptr(),
            (17 + 1i32) as u64,
            code_length_bitdepth_symbols.as_mut_ptr(),
        );
        BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
            num_codes,
            code_length_bitdepth.as_mut_ptr(),
            storage_ix,
            storage,
        );
        if num_codes == 1 {
            code_length_bitdepth[code as usize] = 0;
        }
        BrotliStoreHuffmanTreeToBitMask(
            huffman_tree_size,
            huffman_tree.as_mut_ptr(),
            huffman_tree_extra_bits.as_mut_ptr(),
            code_length_bitdepth.as_mut_ptr(),
            code_length_bitdepth_symbols.as_mut_ptr(),
            storage_ix,
            storage,
        );
    }
}

extern "C" fn BuildAndStoreHuffmanTree(
    mut histogram: *const u32,
    histogram_length: u64,
    alphabet_size: u64,
    mut tree: *mut HuffmanTree,
    mut depth: *mut u8,
    mut bits: *mut u16,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut count = 0;
        let mut s4: [u64; 4] = [0; 4];
        let mut i: u64 = 0;
        let mut max_bits = 0;
        i = 0;
        while i < histogram_length {
            if *histogram.offset(i as isize) != 0 {
                if count < 4 {
                    s4[count as usize] = i;
                } else if count > 4 {
                    break;
                }
                count = count.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        let mut max_bits_counter = alphabet_size.wrapping_sub(1);
        while max_bits_counter != 0 {
            max_bits_counter >>= 1;
            max_bits = max_bits.wrapping_add(1);
        }
        if count <= 1 {
            BrotliWriteBits(4, 1, storage_ix, storage);
            BrotliWriteBits(max_bits, s4[0 as usize], storage_ix, storage);
            *depth.offset(s4[0 as usize] as isize) = 0;
            *bits.offset(s4[0 as usize] as isize) = 0;
            return;
        }
        memset(
            depth as *mut libc::c_void,
            0,
            histogram_length.wrapping_mul(::std::mem::size_of::<u8>() as u64),
        );
        BrotliCreateHuffmanTree(histogram, histogram_length, 15, tree, depth);
        BrotliConvertBitDepthsToSymbols(depth, histogram_length, bits);
        if count <= 4 {
            StoreSimpleHuffmanTree(depth, s4.as_mut_ptr(), count, max_bits, storage_ix, storage);
        } else {
            BrotliStoreHuffmanTree(depth, histogram_length, tree, storage_ix, storage);
        };
    }
}

#[inline(always)]
extern "C" fn SortHuffmanTree(mut v0: *const HuffmanTree, mut v1: *const HuffmanTree) -> i32 {
    unsafe {
        return if (*v0).total_count_ < (*v1).total_count_ {
            1
        } else {
            0
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliBuildAndStoreHuffmanTreeFast(
    mut m: *mut MemoryManager,
    mut histogram: *const u32,
    histogram_total: u64,
    max_bits: u64,
    mut depth: *mut u8,
    mut bits: *mut u16,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut count = 0;
        let mut symbols: [u64; 4] = [0; 4];
        let mut length = 0;
        let mut total = histogram_total;
        while total != 0 {
            if *histogram.offset(length as isize) != 0 {
                if count < 4 {
                    symbols[count as usize] = length;
                }
                count = count.wrapping_add(1);
                total =
                    (total as u64).wrapping_sub(*histogram.offset(length as isize) as u64) as u64;
            }
            length = length.wrapping_add(1);
        }
        if count <= 1 {
            BrotliWriteBits(4, 1, storage_ix, storage);
            BrotliWriteBits(max_bits, symbols[0 as usize], storage_ix, storage);
            *depth.offset(symbols[0 as usize] as isize) = 0;
            *bits.offset(symbols[0 as usize] as isize) = 0;
            return;
        }
        memset(
            depth as *mut libc::c_void,
            0,
            length.wrapping_mul(::std::mem::size_of::<u8>() as u64),
        );
        let max_tree_size = 2u64.wrapping_mul(length).wrapping_add(1);
        let mut tree = if max_tree_size > 0 {
            BrotliAllocate(
                m,
                max_tree_size.wrapping_mul(::std::mem::size_of::<HuffmanTree>() as u64),
            ) as *mut HuffmanTree
        } else {
            0 as *mut HuffmanTree
        };
        let mut count_limit: u32 = 0;
        if 0 != 0 || 0 != 0 {
            return;
        }
        count_limit = 1;
        loop {
            let mut node = tree;
            let mut l: u64 = 0;
            l = length;
            while l != 0 {
                l = l.wrapping_sub(1);
                if *histogram.offset(l as isize) != 0 {
                    if (*histogram.offset(l as isize) >= count_limit) as i64 != 0 {
                        InitHuffmanTree(node, *histogram.offset(l as isize), -1 as i16, l as i16);
                    } else {
                        InitHuffmanTree(node, count_limit, -1 as i16, l as i16);
                    }
                    node = node.offset(1);
                }
            }
            let n = node.offset_from(tree) as i32;
            let mut sentinel = HuffmanTree {
                total_count_: 0,
                index_left_: 0,
                index_right_or_value_: 0,
            };
            let mut i = 0;
            let mut j = n + 1;
            let mut k: i32 = 0;
            SortHuffmanTreeItems(
                tree,
                n as u64,
                Some(
                    SortHuffmanTree
                        as unsafe extern "C" fn(*const HuffmanTree, *const HuffmanTree) -> i32,
                ),
            );
            InitHuffmanTree(&mut sentinel, !0, -1 as i16, -1 as i16);
            let fresh7 = node;
            node = node.offset(1);
            *fresh7 = sentinel;
            let fresh8 = node;
            node = node.offset(1);
            *fresh8 = sentinel;
            k = n - 1;
            while k > 0 {
                let mut left: i32 = 0;
                let mut right: i32 = 0;
                if (*tree.offset(i as isize)).total_count_
                    <= (*tree.offset(j as isize)).total_count_
                {
                    left = i;
                    i += 1;
                } else {
                    left = j;
                    j += 1;
                }
                if (*tree.offset(i as isize)).total_count_
                    <= (*tree.offset(j as isize)).total_count_
                {
                    right = i;
                    i += 1;
                } else {
                    right = j;
                    j += 1;
                };
                (*node.offset(-1i32 as isize)).total_count_ = ((*tree.offset(left as isize))
                    .total_count_)
                    .wrapping_add((*tree.offset(right as isize)).total_count_);
                (*node.offset(-1i32 as isize)).index_left_ = left as i16;
                (*node.offset(-1i32 as isize)).index_right_or_value_ = right as i16;
                let fresh9 = node;
                node = node.offset(1);
                *fresh9 = sentinel;
                k -= 1;
            }
            if BrotliSetDepth(2 * n - 1, tree, depth, 14) != 0 {
                break;
            }
            count_limit = (count_limit).wrapping_mul(2) as u32;
        }
        BrotliFree(m, tree as *mut libc::c_void);
        tree = 0 as *mut HuffmanTree;
        BrotliConvertBitDepthsToSymbols(depth, length, bits);
        if count <= 4 {
            let mut i_0: u64 = 0;
            BrotliWriteBits(2, 1, storage_ix, storage);
            BrotliWriteBits(2, count.wrapping_sub(1), storage_ix, storage);
            i_0 = 0;
            while i_0 < count {
                let mut j_0: u64 = 0;
                j_0 = i_0.wrapping_add(1);
                while j_0 < count {
                    if (*depth.offset(symbols[j_0 as usize] as isize) as i32)
                        < *depth.offset(symbols[i_0 as usize] as isize) as i32
                    {
                        let mut __brotli_swap_tmp = symbols[j_0 as usize];
                        symbols[j_0 as usize] = symbols[i_0 as usize];
                        symbols[i_0 as usize] = __brotli_swap_tmp;
                    }
                    j_0 = j_0.wrapping_add(1);
                }
                i_0 = i_0.wrapping_add(1);
            }
            if count == 2 {
                BrotliWriteBits(max_bits, symbols[0 as usize], storage_ix, storage);
                BrotliWriteBits(max_bits, symbols[1 as usize], storage_ix, storage);
            } else if count == 3 {
                BrotliWriteBits(max_bits, symbols[0 as usize], storage_ix, storage);
                BrotliWriteBits(max_bits, symbols[1 as usize], storage_ix, storage);
                BrotliWriteBits(max_bits, symbols[2 as usize], storage_ix, storage);
            } else {
                BrotliWriteBits(max_bits, symbols[0 as usize], storage_ix, storage);
                BrotliWriteBits(max_bits, symbols[1 as usize], storage_ix, storage);
                BrotliWriteBits(max_bits, symbols[2 as usize], storage_ix, storage);
                BrotliWriteBits(max_bits, symbols[3 as usize], storage_ix, storage);
                BrotliWriteBits(
                    1,
                    (if *depth.offset(symbols[0 as usize] as isize) as i32 == 1 {
                        1
                    } else {
                        0
                    }) as u64,
                    storage_ix,
                    storage,
                );
            }
        } else {
            let mut previous_value = 8;
            let mut i_1: u64 = 0;
            StoreStaticCodeLengthCode(storage_ix, storage);
            i_1 = 0;
            while i_1 < length {
                let value = *depth.offset(i_1 as isize);
                let mut reps = 1;
                let mut k_0: u64 = 0;
                k_0 = i_1.wrapping_add(1);
                while k_0 < length && *depth.offset(k_0 as isize) as i32 == value as i32 {
                    reps = reps.wrapping_add(1);
                    k_0 = k_0.wrapping_add(1);
                }
                i_1 = (i_1).wrapping_add(reps) as u64;
                if value as i32 == 0 {
                    BrotliWriteBits(
                        kZeroRepsDepth[reps as usize] as u64,
                        kZeroRepsBits[reps as usize],
                        storage_ix,
                        storage,
                    );
                } else {
                    if previous_value as i32 != value as i32 {
                        BrotliWriteBits(
                            kCodeLengthDepth[value as usize] as u64,
                            kCodeLengthBits[value as usize] as u64,
                            storage_ix,
                            storage,
                        );
                        reps = reps.wrapping_sub(1);
                    }
                    if reps < 3 {
                        while reps != 0 {
                            reps = reps.wrapping_sub(1);
                            BrotliWriteBits(
                                kCodeLengthDepth[value as usize] as u64,
                                kCodeLengthBits[value as usize] as u64,
                                storage_ix,
                                storage,
                            );
                        }
                    } else {
                        reps = (reps as u64).wrapping_sub(3) as u64;
                        BrotliWriteBits(
                            kNonZeroRepsDepth[reps as usize] as u64,
                            kNonZeroRepsBits[reps as usize],
                            storage_ix,
                            storage,
                        );
                    }
                    previous_value = value;
                }
            }
        };
    }
}

extern "C" fn IndexOf(mut v: *const u8, mut v_size: u64, mut value: u8) -> u64 {
    unsafe {
        let mut i = 0;
        while i < v_size {
            if *v.offset(i as isize) as i32 == value as i32 {
                return i;
            }
            i = i.wrapping_add(1);
        }
        return i;
    }
}

extern "C" fn MoveToFront(mut v: *mut u8, mut index: u64) {
    unsafe {
        let mut value = *v.offset(index as isize);
        let mut i: u64 = 0;
        i = index;
        while i != 0 {
            *v.offset(i as isize) = *v.offset(i.wrapping_sub(1) as isize);
            i = i.wrapping_sub(1);
        }
        *v.offset(0 as isize) = value;
    }
}

extern "C" fn MoveToFrontTransform(mut v_in: *const u32, v_size: u64, mut v_out: *mut u32) {
    unsafe {
        let mut i: u64 = 0;
        let mut mtf: [u8; 256] = [0; 256];
        let mut max_value: u32 = 0;
        if v_size == 0 {
            return;
        }
        max_value = *v_in.offset(0 as isize);
        i = 1;
        while i < v_size {
            if *v_in.offset(i as isize) > max_value {
                max_value = *v_in.offset(i as isize);
            }
            i = i.wrapping_add(1);
        }
        i = 0;
        while i <= max_value as u64 {
            mtf[i as usize] = i as u8;
            i = i.wrapping_add(1);
        }
        let mut mtf_size = max_value.wrapping_add(1) as u64;
        i = 0;
        while i < v_size {
            let mut index = IndexOf(mtf.as_mut_ptr(), mtf_size, *v_in.offset(i as isize) as u8);
            *v_out.offset(i as isize) = index as u32;
            MoveToFront(mtf.as_mut_ptr(), index);
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn RunLengthCodeZeros(
    in_size: u64,
    mut v: *mut u32,
    mut out_size: *mut u64,
    mut max_run_length_prefix: *mut u32,
) {
    unsafe {
        let mut max_reps = 0;
        let mut i: u64 = 0;
        let mut max_prefix: u32 = 0;
        i = 0;
        while i < in_size {
            let mut reps = 0;
            while i < in_size && *v.offset(i as isize) != 0 {
                i = i.wrapping_add(1);
            }
            while i < in_size && *v.offset(i as isize) == 0 {
                reps = reps.wrapping_add(1);
                i = i.wrapping_add(1);
            }
            max_reps = brotli_max_uint32_t(reps, max_reps);
        }
        max_prefix = if max_reps > 0 {
            Log2FloorNonZero(max_reps as u64)
        } else {
            0
        };
        max_prefix = brotli_min_uint32_t(max_prefix, *max_run_length_prefix);
        *max_run_length_prefix = max_prefix;
        *out_size = 0;
        i = 0;
        while i < in_size {
            if *v.offset(i as isize) != 0 {
                *v.offset(*out_size as isize) =
                    (*v.offset(i as isize)).wrapping_add(*max_run_length_prefix);
                i = i.wrapping_add(1);
                *out_size = (*out_size).wrapping_add(1);
            } else {
                let mut reps_0 = 1;
                let mut k: u64 = 0;
                k = i.wrapping_add(1);
                while k < in_size && *v.offset(k as isize) == 0 {
                    reps_0 = reps_0.wrapping_add(1);
                    k = k.wrapping_add(1);
                }
                i = (i).wrapping_add(reps_0 as u64) as u64;
                while reps_0 != 0 {
                    if reps_0 < 2 << max_prefix {
                        let mut run_length_prefix = Log2FloorNonZero(reps_0 as u64);
                        let extra_bits = reps_0.wrapping_sub(1 << run_length_prefix);
                        *v.offset(*out_size as isize) =
                            run_length_prefix.wrapping_add(extra_bits << 9);
                        *out_size = (*out_size).wrapping_add(1);
                        break;
                    } else {
                        let extra_bits_0 = (1u32 << max_prefix).wrapping_sub(1);
                        *v.offset(*out_size as isize) = max_prefix.wrapping_add(extra_bits_0 << 9);
                        reps_0 = (reps_0 as u32).wrapping_sub((2u32 << max_prefix).wrapping_sub(1))
                            as u32;
                        *out_size = (*out_size).wrapping_add(1);
                    }
                }
            }
        }
    }
}

static mut kSymbolMask: u32 = 0;
extern "C" fn EncodeContextMap(
    mut m: *mut MemoryManager,
    mut context_map: *const u32,
    mut context_map_size: u64,
    mut num_clusters: u64,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut i: u64 = 0;
        let mut rle_symbols = 0 as *mut u32;
        let mut max_run_length_prefix = 6;
        let mut num_rle_symbols = 0;
        let mut histogram: [u32; 272] = [0; 272];
        let mut depths: [u8; 272] = [0; 272];
        let mut bits: [u16; 272] = [0; 272];
        StoreVarLenUint8(num_clusters.wrapping_sub(1), storage_ix, storage);
        if num_clusters == 1 {
            return;
        }
        rle_symbols = if context_map_size > 0 {
            BrotliAllocate(
                m,
                context_map_size.wrapping_mul(::std::mem::size_of::<u32>() as u64),
            ) as *mut u32
        } else {
            0 as *mut u32
        };
        if 0 != 0 || 0 != 0 {
            return;
        }
        MoveToFrontTransform(context_map, context_map_size, rle_symbols);
        RunLengthCodeZeros(
            context_map_size,
            rle_symbols,
            &mut num_rle_symbols,
            &mut max_run_length_prefix,
        );
        memset(
            histogram.as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u32; 272]>() as u64,
        );
        i = 0;
        while i < num_rle_symbols {
            histogram[(*rle_symbols.offset(i as isize) & kSymbolMask) as usize] = (histogram
                [(*rle_symbols.offset(i as isize) & kSymbolMask) as usize])
                .wrapping_add(1);
            i = i.wrapping_add(1);
        }
        let mut use_rle = if max_run_length_prefix > 0 { 1 } else { 0 };
        BrotliWriteBits(1, use_rle as u64, storage_ix, storage);
        if use_rle != 0 {
            BrotliWriteBits(
                4,
                max_run_length_prefix.wrapping_sub(1) as u64,
                storage_ix,
                storage,
            );
        }
        BuildAndStoreHuffmanTree(
            histogram.as_mut_ptr(),
            num_clusters.wrapping_add(max_run_length_prefix as u64),
            num_clusters.wrapping_add(max_run_length_prefix as u64),
            tree,
            depths.as_mut_ptr(),
            bits.as_mut_ptr(),
            storage_ix,
            storage,
        );
        i = 0;
        while i < num_rle_symbols {
            let rle_symbol = *rle_symbols.offset(i as isize) & kSymbolMask;
            let extra_bits_val = *rle_symbols.offset(i as isize) >> 9;
            BrotliWriteBits(
                depths[rle_symbol as usize] as u64,
                bits[rle_symbol as usize] as u64,
                storage_ix,
                storage,
            );
            if rle_symbol > 0 && rle_symbol <= max_run_length_prefix {
                BrotliWriteBits(
                    rle_symbol as u64,
                    extra_bits_val as u64,
                    storage_ix,
                    storage,
                );
            }
            i = i.wrapping_add(1);
        }
        BrotliWriteBits(1, 1, storage_ix, storage);
        BrotliFree(m, rle_symbols as *mut libc::c_void);
        rle_symbols = 0 as *mut u32;
    }
}

#[inline(always)]
extern "C" fn StoreBlockSwitch(
    mut code: *mut BlockSplitCode,
    block_len: u32,
    block_type: u8,
    mut is_first_block: i32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut typecode = NextBlockTypeCode(&mut (*code).type_code_calculator, block_type);
        let mut lencode: u64 = 0;
        let mut len_nextra: u32 = 0;
        let mut len_extra: u32 = 0;
        if is_first_block == 0 {
            BrotliWriteBits(
                (*code).type_depths[typecode as usize] as u64,
                (*code).type_bits[typecode as usize] as u64,
                storage_ix,
                storage,
            );
        }
        GetBlockLengthPrefixCode(block_len, &mut lencode, &mut len_nextra, &mut len_extra);
        BrotliWriteBits(
            (*code).length_depths[lencode as usize] as u64,
            (*code).length_bits[lencode as usize] as u64,
            storage_ix,
            storage,
        );
        BrotliWriteBits(len_nextra as u64, len_extra as u64, storage_ix, storage);
    }
}

extern "C" fn BuildAndStoreBlockSplitCode(
    mut types: *const u8,
    mut lengths: *const u32,
    num_blocks: u64,
    num_types: u64,
    mut tree: *mut HuffmanTree,
    mut code: *mut BlockSplitCode,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut type_histo: [u32; 258] = [0; 258];
        let mut length_histo: [u32; 26] = [0; 26];
        let mut i: u64 = 0;
        let mut type_code_calculator = BlockTypeCodeCalculator {
            last_type: 0,
            second_last_type: 0,
        };
        memset(
            type_histo.as_mut_ptr() as *mut libc::c_void,
            0,
            num_types
                .wrapping_add(2)
                .wrapping_mul(::std::mem::size_of::<u32>() as u64),
        );
        memset(
            length_histo.as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u32; 26]>() as u64,
        );
        InitBlockTypeCodeCalculator(&mut type_code_calculator);
        i = 0;
        while i < num_blocks {
            let mut type_code =
                NextBlockTypeCode(&mut type_code_calculator, *types.offset(i as isize));
            if i != 0 {
                type_histo[type_code as usize] = (type_histo[type_code as usize]).wrapping_add(1);
            }
            length_histo[BlockLengthPrefixCode(*lengths.offset(i as isize)) as usize] =
                (length_histo[BlockLengthPrefixCode(*lengths.offset(i as isize)) as usize])
                    .wrapping_add(1);
            i = i.wrapping_add(1);
        }
        StoreVarLenUint8(num_types.wrapping_sub(1), storage_ix, storage);
        if num_types > 1 {
            BuildAndStoreHuffmanTree(
                &mut *type_histo.as_mut_ptr().offset(0 as isize),
                num_types.wrapping_add(2),
                num_types.wrapping_add(2),
                tree,
                &mut *((*code).type_depths).as_mut_ptr().offset(0 as isize),
                &mut *((*code).type_bits).as_mut_ptr().offset(0 as isize),
                storage_ix,
                storage,
            );
            BuildAndStoreHuffmanTree(
                &mut *length_histo.as_mut_ptr().offset(0 as isize),
                26,
                26,
                tree,
                &mut *((*code).length_depths).as_mut_ptr().offset(0 as isize),
                &mut *((*code).length_bits).as_mut_ptr().offset(0 as isize),
                storage_ix,
                storage,
            );
            StoreBlockSwitch(
                code,
                *lengths.offset(0 as isize),
                *types.offset(0 as isize),
                1,
                storage_ix,
                storage,
            );
        }
    }
}

extern "C" fn StoreTrivialContextMap(
    mut num_types: u64,
    mut context_bits: u64,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        StoreVarLenUint8(num_types.wrapping_sub(1), storage_ix, storage);
        if num_types > 1 {
            let mut repeat_code = context_bits.wrapping_sub(1);
            let mut repeat_bits = (1u32 << repeat_code).wrapping_sub(1) as u64;
            let mut alphabet_size = num_types.wrapping_add(repeat_code);
            let mut histogram: [u32; 272] = [0; 272];
            let mut depths: [u8; 272] = [0; 272];
            let mut bits: [u16; 272] = [0; 272];
            let mut i: u64 = 0;
            memset(
                histogram.as_mut_ptr() as *mut libc::c_void,
                0,
                alphabet_size.wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
            BrotliWriteBits(1, 1, storage_ix, storage);
            BrotliWriteBits(4, repeat_code.wrapping_sub(1), storage_ix, storage);
            histogram[repeat_code as usize] = num_types as u32;
            histogram[0 as usize] = 1;
            i = context_bits;
            while i < alphabet_size {
                histogram[i as usize] = 1;
                i = i.wrapping_add(1);
            }
            BuildAndStoreHuffmanTree(
                histogram.as_mut_ptr(),
                alphabet_size,
                alphabet_size,
                tree,
                depths.as_mut_ptr(),
                bits.as_mut_ptr(),
                storage_ix,
                storage,
            );
            i = 0;
            while i < num_types {
                let mut code = if i == 0 {
                    0
                } else {
                    i.wrapping_add(context_bits).wrapping_sub(1)
                };
                BrotliWriteBits(
                    depths[code as usize] as u64,
                    bits[code as usize] as u64,
                    storage_ix,
                    storage,
                );
                BrotliWriteBits(
                    depths[repeat_code as usize] as u64,
                    bits[repeat_code as usize] as u64,
                    storage_ix,
                    storage,
                );
                BrotliWriteBits(repeat_code, repeat_bits, storage_ix, storage);
                i = i.wrapping_add(1);
            }
            BrotliWriteBits(1, 1, storage_ix, storage);
        }
    }
}

extern "C" fn InitBlockEncoder(
    mut self_0: *mut BlockEncoder,
    mut histogram_length: u64,
    mut num_block_types: u64,
    mut block_types: *const u8,
    mut block_lengths: *const u32,
    num_blocks: u64,
) {
    unsafe {
        (*self_0).histogram_length_ = histogram_length;
        (*self_0).num_block_types_ = num_block_types;
        let ref mut fresh10 = (*self_0).block_types_;
        *fresh10 = block_types;
        let ref mut fresh11 = (*self_0).block_lengths_;
        *fresh11 = block_lengths;
        (*self_0).num_blocks_ = num_blocks;
        InitBlockTypeCodeCalculator(&mut (*self_0).block_split_code_.type_code_calculator);
        (*self_0).block_ix_ = 0;
        (*self_0).block_len_ = (if num_blocks == 0 {
            0
        } else {
            *block_lengths.offset(0 as isize)
        }) as u64;
        (*self_0).entropy_ix_ = 0;
        let ref mut fresh12 = (*self_0).depths_;
        *fresh12 = 0 as *mut u8;
        let ref mut fresh13 = (*self_0).bits_;
        *fresh13 = 0 as *mut u16;
    }
}

extern "C" fn CleanupBlockEncoder(mut m: *mut MemoryManager, mut self_0: *mut BlockEncoder) {
    unsafe {
        BrotliFree(m, (*self_0).depths_ as *mut libc::c_void);
        let ref mut fresh14 = (*self_0).depths_;
        *fresh14 = 0 as *mut u8;
        BrotliFree(m, (*self_0).bits_ as *mut libc::c_void);
        let ref mut fresh15 = (*self_0).bits_;
        *fresh15 = 0 as *mut u16;
    }
}

extern "C" fn BuildAndStoreBlockSwitchEntropyCodes(
    mut self_0: *mut BlockEncoder,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        BuildAndStoreBlockSplitCode(
            (*self_0).block_types_,
            (*self_0).block_lengths_,
            (*self_0).num_blocks_,
            (*self_0).num_block_types_,
            tree,
            &mut (*self_0).block_split_code_,
            storage_ix,
            storage,
        );
    }
}

extern "C" fn StoreSymbol(
    mut self_0: *mut BlockEncoder,
    mut symbol: u64,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        if (*self_0).block_len_ == 0 {
            let ref mut fresh16 = (*self_0).block_ix_;
            *fresh16 = (*fresh16).wrapping_add(1);
            let mut block_ix = *fresh16;
            let mut block_len = *((*self_0).block_lengths_).offset(block_ix as isize);
            let mut block_type = *((*self_0).block_types_).offset(block_ix as isize);
            (*self_0).block_len_ = block_len as u64;
            (*self_0).entropy_ix_ = (block_type as u64).wrapping_mul((*self_0).histogram_length_);
            StoreBlockSwitch(
                &mut (*self_0).block_split_code_,
                block_len,
                block_type,
                0,
                storage_ix,
                storage,
            );
        }
        let ref mut fresh17 = (*self_0).block_len_;
        *fresh17 = (*fresh17).wrapping_sub(1);
        let mut ix = ((*self_0).entropy_ix_).wrapping_add(symbol);
        BrotliWriteBits(
            *((*self_0).depths_).offset(ix as isize) as u64,
            *((*self_0).bits_).offset(ix as isize) as u64,
            storage_ix,
            storage,
        );
    }
}

extern "C" fn StoreSymbolWithContext(
    mut self_0: *mut BlockEncoder,
    mut symbol: u64,
    mut context: u64,
    mut context_map: *const u32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
    context_bits: u64,
) {
    unsafe {
        if (*self_0).block_len_ == 0 {
            let ref mut fresh18 = (*self_0).block_ix_;
            *fresh18 = (*fresh18).wrapping_add(1);
            let mut block_ix = *fresh18;
            let mut block_len = *((*self_0).block_lengths_).offset(block_ix as isize);
            let mut block_type = *((*self_0).block_types_).offset(block_ix as isize);
            (*self_0).block_len_ = block_len as u64;
            (*self_0).entropy_ix_ = (block_type as u64) << context_bits;
            StoreBlockSwitch(
                &mut (*self_0).block_split_code_,
                block_len,
                block_type,
                0,
                storage_ix,
                storage,
            );
        }
        let ref mut fresh19 = (*self_0).block_len_;
        *fresh19 = (*fresh19).wrapping_sub(1);
        let mut histo_ix =
            *context_map.offset(((*self_0).entropy_ix_).wrapping_add(context) as isize) as u64;
        let mut ix = histo_ix
            .wrapping_mul((*self_0).histogram_length_)
            .wrapping_add(symbol);
        BrotliWriteBits(
            *((*self_0).depths_).offset(ix as isize) as u64,
            *((*self_0).bits_).offset(ix as isize) as u64,
            storage_ix,
            storage,
        );
    }
}

extern "C" fn BuildAndStoreEntropyCodesCommand(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockEncoder,
    mut histograms: *const HistogramCommand,
    histograms_size: u64,
    alphabet_size: u64,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let table_size = histograms_size.wrapping_mul((*self_0).histogram_length_);
        let ref mut fresh20 = (*self_0).depths_;
        *fresh20 = if table_size > 0 {
            BrotliAllocate(
                m,
                table_size.wrapping_mul(::std::mem::size_of::<u8>() as u64),
            ) as *mut u8
        } else {
            0 as *mut u8
        };
        let ref mut fresh21 = (*self_0).bits_;
        *fresh21 = if table_size > 0 {
            BrotliAllocate(
                m,
                table_size.wrapping_mul(::std::mem::size_of::<u16>() as u64),
            ) as *mut u16
        } else {
            0 as *mut u16
        };
        if 0 != 0 {
            return;
        }
        let mut i: u64 = 0;
        i = 0;
        while i < histograms_size {
            let mut ix = i.wrapping_mul((*self_0).histogram_length_);
            BuildAndStoreHuffmanTree(
                &*((*histograms.offset(i as isize)).data_)
                    .as_ptr()
                    .offset(0 as isize),
                (*self_0).histogram_length_,
                alphabet_size,
                tree,
                &mut *((*self_0).depths_).offset(ix as isize),
                &mut *((*self_0).bits_).offset(ix as isize),
                storage_ix,
                storage,
            );
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn BuildAndStoreEntropyCodesLiteral(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockEncoder,
    mut histograms: *const HistogramLiteral,
    histograms_size: u64,
    alphabet_size: u64,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let table_size = histograms_size.wrapping_mul((*self_0).histogram_length_);
        let ref mut fresh22 = (*self_0).depths_;
        *fresh22 = if table_size > 0 {
            BrotliAllocate(
                m,
                table_size.wrapping_mul(::std::mem::size_of::<u8>() as u64),
            ) as *mut u8
        } else {
            0 as *mut u8
        };
        let ref mut fresh23 = (*self_0).bits_;
        *fresh23 = if table_size > 0 {
            BrotliAllocate(
                m,
                table_size.wrapping_mul(::std::mem::size_of::<u16>() as u64),
            ) as *mut u16
        } else {
            0 as *mut u16
        };
        if 0 != 0 {
            return;
        }
        let mut i: u64 = 0;
        i = 0;
        while i < histograms_size {
            let mut ix = i.wrapping_mul((*self_0).histogram_length_);
            BuildAndStoreHuffmanTree(
                &*((*histograms.offset(i as isize)).data_)
                    .as_ptr()
                    .offset(0 as isize),
                (*self_0).histogram_length_,
                alphabet_size,
                tree,
                &mut *((*self_0).depths_).offset(ix as isize),
                &mut *((*self_0).bits_).offset(ix as isize),
                storage_ix,
                storage,
            );
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn BuildAndStoreEntropyCodesDistance(
    mut m: *mut MemoryManager,
    mut self_0: *mut BlockEncoder,
    mut histograms: *const HistogramDistance,
    histograms_size: u64,
    alphabet_size: u64,
    mut tree: *mut HuffmanTree,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let table_size = histograms_size.wrapping_mul((*self_0).histogram_length_);
        let ref mut fresh24 = (*self_0).depths_;
        *fresh24 = if table_size > 0 {
            BrotliAllocate(
                m,
                table_size.wrapping_mul(::std::mem::size_of::<u8>() as u64),
            ) as *mut u8
        } else {
            0 as *mut u8
        };
        let ref mut fresh25 = (*self_0).bits_;
        *fresh25 = if table_size > 0 {
            BrotliAllocate(
                m,
                table_size.wrapping_mul(::std::mem::size_of::<u16>() as u64),
            ) as *mut u16
        } else {
            0 as *mut u16
        };
        if 0 != 0 {
            return;
        }
        let mut i: u64 = 0;
        i = 0;
        while i < histograms_size {
            let mut ix = i.wrapping_mul((*self_0).histogram_length_);
            BuildAndStoreHuffmanTree(
                &*((*histograms.offset(i as isize)).data_)
                    .as_ptr()
                    .offset(0 as isize),
                (*self_0).histogram_length_,
                alphabet_size,
                tree,
                &mut *((*self_0).depths_).offset(ix as isize),
                &mut *((*self_0).bits_).offset(ix as isize),
                storage_ix,
                storage,
            );
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn JumpToByteBoundary(mut storage_ix: *mut u64, mut storage: *mut u8) {
    unsafe {
        *storage_ix = (*storage_ix).wrapping_add(7) & !7 as u64;
        *storage.offset((*storage_ix >> 3i32) as isize) = 0;
    }
}

#[no_mangle]
pub extern "C" fn BrotliStoreMetaBlock(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut start_pos: u64,
    mut length: u64,
    mut mask: u64,
    mut prev_byte: u8,
    mut prev_byte2: u8,
    mut is_last: i32,
    mut params: *const BrotliEncoderParams,
    mut literal_context_mode: u32,
    mut commands: *const Command,
    mut n_commands: u64,
    mut mb: *const MetaBlockSplit,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut pos = start_pos;
        let mut i: u64 = 0;
        let mut num_distance_symbols = (*params).dist.alphabet_size_max;
        let mut num_effective_distance_symbols = (*params).dist.alphabet_size_limit;
        let mut tree = 0 as *mut HuffmanTree;
        let mut literal_context_lut: ContextLut = &*_kBrotliContextLookupTable
            .as_ptr()
            .offset(((literal_context_mode as u32) << 9i32) as isize)
            as *const u8;
        let mut literal_enc = BlockEncoder {
            histogram_length_: 0,
            num_block_types_: 0,
            block_types_: 0 as *const u8,
            block_lengths_: 0 as *const u32,
            num_blocks_: 0,
            block_split_code_: BlockSplitCode {
                type_code_calculator: BlockTypeCodeCalculator {
                    last_type: 0,
                    second_last_type: 0,
                },
                type_depths: [0; 258],
                type_bits: [0; 258],
                length_depths: [0; 26],
                length_bits: [0; 26],
            },
            block_ix_: 0,
            block_len_: 0,
            entropy_ix_: 0,
            depths_: 0 as *mut u8,
            bits_: 0 as *mut u16,
        };
        let mut command_enc = BlockEncoder {
            histogram_length_: 0,
            num_block_types_: 0,
            block_types_: 0 as *const u8,
            block_lengths_: 0 as *const u32,
            num_blocks_: 0,
            block_split_code_: BlockSplitCode {
                type_code_calculator: BlockTypeCodeCalculator {
                    last_type: 0,
                    second_last_type: 0,
                },
                type_depths: [0; 258],
                type_bits: [0; 258],
                length_depths: [0; 26],
                length_bits: [0; 26],
            },
            block_ix_: 0,
            block_len_: 0,
            entropy_ix_: 0,
            depths_: 0 as *mut u8,
            bits_: 0 as *mut u16,
        };
        let mut distance_enc = BlockEncoder {
            histogram_length_: 0,
            num_block_types_: 0,
            block_types_: 0 as *const u8,
            block_lengths_: 0 as *const u32,
            num_blocks_: 0,
            block_split_code_: BlockSplitCode {
                type_code_calculator: BlockTypeCodeCalculator {
                    last_type: 0,
                    second_last_type: 0,
                },
                type_depths: [0; 258],
                type_bits: [0; 258],
                length_depths: [0; 26],
                length_bits: [0; 26],
            },
            block_ix_: 0,
            block_len_: 0,
            entropy_ix_: 0,
            depths_: 0 as *mut u8,
            bits_: 0 as *mut u16,
        };
        let mut dist: *const BrotliDistanceParams = &(*params).dist;
        StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
        tree = if 2 * 704 + 1 > 0 {
            BrotliAllocate(
                m,
                ((2 * 704 + 1i32) as u64).wrapping_mul(::std::mem::size_of::<HuffmanTree>() as u64),
            ) as *mut HuffmanTree
        } else {
            0 as *mut HuffmanTree
        };
        if 0 != 0 || 0 != 0 {
            return;
        }
        InitBlockEncoder(
            &mut literal_enc,
            256,
            (*mb).literal_split.num_types,
            (*mb).literal_split.types,
            (*mb).literal_split.lengths,
            (*mb).literal_split.num_blocks,
        );
        InitBlockEncoder(
            &mut command_enc,
            704,
            (*mb).command_split.num_types,
            (*mb).command_split.types,
            (*mb).command_split.lengths,
            (*mb).command_split.num_blocks,
        );
        InitBlockEncoder(
            &mut distance_enc,
            num_effective_distance_symbols as u64,
            (*mb).distance_split.num_types,
            (*mb).distance_split.types,
            (*mb).distance_split.lengths,
            (*mb).distance_split.num_blocks,
        );
        BuildAndStoreBlockSwitchEntropyCodes(&mut literal_enc, tree, storage_ix, storage);
        BuildAndStoreBlockSwitchEntropyCodes(&mut command_enc, tree, storage_ix, storage);
        BuildAndStoreBlockSwitchEntropyCodes(&mut distance_enc, tree, storage_ix, storage);
        BrotliWriteBits(2, (*dist).distance_postfix_bits as u64, storage_ix, storage);
        BrotliWriteBits(
            4,
            ((*dist).num_direct_distance_codes >> (*dist).distance_postfix_bits) as u64,
            storage_ix,
            storage,
        );
        i = 0;
        while i < (*mb).literal_split.num_types {
            BrotliWriteBits(2, literal_context_mode as u64, storage_ix, storage);
            i = i.wrapping_add(1);
        }
        if (*mb).literal_context_map_size == 0 {
            StoreTrivialContextMap((*mb).literal_histograms_size, 6, tree, storage_ix, storage);
        } else {
            EncodeContextMap(
                m,
                (*mb).literal_context_map,
                (*mb).literal_context_map_size,
                (*mb).literal_histograms_size,
                tree,
                storage_ix,
                storage,
            );
            if 0 != 0 {
                return;
            }
        }
        if (*mb).distance_context_map_size == 0 {
            StoreTrivialContextMap((*mb).distance_histograms_size, 2, tree, storage_ix, storage);
        } else {
            EncodeContextMap(
                m,
                (*mb).distance_context_map,
                (*mb).distance_context_map_size,
                (*mb).distance_histograms_size,
                tree,
                storage_ix,
                storage,
            );
            if 0 != 0 {
                return;
            }
        }
        BuildAndStoreEntropyCodesLiteral(
            m,
            &mut literal_enc,
            (*mb).literal_histograms,
            (*mb).literal_histograms_size,
            256,
            tree,
            storage_ix,
            storage,
        );
        if 0 != 0 {
            return;
        }
        BuildAndStoreEntropyCodesCommand(
            m,
            &mut command_enc,
            (*mb).command_histograms,
            (*mb).command_histograms_size,
            704,
            tree,
            storage_ix,
            storage,
        );
        if 0 != 0 {
            return;
        }
        BuildAndStoreEntropyCodesDistance(
            m,
            &mut distance_enc,
            (*mb).distance_histograms,
            (*mb).distance_histograms_size,
            num_distance_symbols as u64,
            tree,
            storage_ix,
            storage,
        );
        if 0 != 0 {
            return;
        }
        BrotliFree(m, tree as *mut libc::c_void);
        tree = 0 as *mut HuffmanTree;
        i = 0;
        while i < n_commands {
            let cmd = *commands.offset(i as isize);
            let mut cmd_code = cmd.cmd_prefix_ as u64;
            StoreSymbol(&mut command_enc, cmd_code, storage_ix, storage);
            StoreCommandExtra(&cmd, storage_ix, storage);
            if (*mb).literal_context_map_size == 0 {
                let mut j: u64 = 0;
                j = cmd.insert_len_ as u64;
                while j != 0 {
                    StoreSymbol(
                        &mut literal_enc,
                        *input.offset((pos & mask) as isize) as u64,
                        storage_ix,
                        storage,
                    );
                    pos = pos.wrapping_add(1);
                    j = j.wrapping_sub(1);
                }
            } else {
                let mut j_0: u64 = 0;
                j_0 = cmd.insert_len_ as u64;
                while j_0 != 0 {
                    let mut context = (*literal_context_lut.offset(prev_byte as isize) as i32
                        | *literal_context_lut
                            .offset(256 as isize)
                            .offset(prev_byte2 as isize) as i32)
                        as u64;
                    let mut literal = *input.offset((pos & mask) as isize);
                    StoreSymbolWithContext(
                        &mut literal_enc,
                        literal as u64,
                        context,
                        (*mb).literal_context_map,
                        storage_ix,
                        storage,
                        6,
                    );
                    prev_byte2 = prev_byte;
                    prev_byte = literal;
                    pos = pos.wrapping_add(1);
                    j_0 = j_0.wrapping_sub(1);
                }
            }
            pos = (pos as u64).wrapping_add(CommandCopyLen(&cmd) as u64) as u64;
            if CommandCopyLen(&cmd) != 0 {
                prev_byte2 = *input.offset((pos.wrapping_sub(2u64) & mask) as isize);
                prev_byte = *input.offset((pos.wrapping_sub(1u64) & mask) as isize);
                if cmd.cmd_prefix_ as i32 >= 128 {
                    let mut dist_code = (cmd.dist_prefix_ as i32 & 0x3ffi32) as u64;
                    let mut distnumextra = (cmd.dist_prefix_ as i32 >> 10i32) as u32;
                    let mut distextra = cmd.dist_extra_ as u64;
                    if (*mb).distance_context_map_size == 0 {
                        StoreSymbol(&mut distance_enc, dist_code, storage_ix, storage);
                    } else {
                        let mut context_0 = CommandDistanceContext(&cmd) as u64;
                        StoreSymbolWithContext(
                            &mut distance_enc,
                            dist_code,
                            context_0,
                            (*mb).distance_context_map,
                            storage_ix,
                            storage,
                            2,
                        );
                    }
                    BrotliWriteBits(distnumextra as u64, distextra, storage_ix, storage);
                }
            }
            i = i.wrapping_add(1);
        }
        CleanupBlockEncoder(m, &mut distance_enc);
        CleanupBlockEncoder(m, &mut command_enc);
        CleanupBlockEncoder(m, &mut literal_enc);
        if is_last != 0 {
            JumpToByteBoundary(storage_ix, storage);
        }
    }
}

extern "C" fn BuildHistograms(
    mut input: *const u8,
    mut start_pos: u64,
    mut mask: u64,
    mut commands: *const Command,
    mut n_commands: u64,
    mut lit_histo: *mut HistogramLiteral,
    mut cmd_histo: *mut HistogramCommand,
    mut dist_histo: *mut HistogramDistance,
) {
    unsafe {
        let mut pos = start_pos;
        let mut i: u64 = 0;
        i = 0;
        while i < n_commands {
            let cmd = *commands.offset(i as isize);
            let mut j: u64 = 0;
            HistogramAddCommand(cmd_histo, cmd.cmd_prefix_ as u64);
            j = cmd.insert_len_ as u64;
            while j != 0 {
                HistogramAddLiteral(lit_histo, *input.offset((pos & mask) as isize) as u64);
                pos = pos.wrapping_add(1);
                j = j.wrapping_sub(1);
            }
            pos = (pos as u64).wrapping_add(CommandCopyLen(&cmd) as u64) as u64;
            if CommandCopyLen(&cmd) != 0 && cmd.cmd_prefix_ as i32 >= 128 {
                HistogramAddDistance(dist_histo, (cmd.dist_prefix_ as i32 & 0x3ffi32) as u64);
            }
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn StoreDataWithHuffmanCodes(
    mut input: *const u8,
    mut start_pos: u64,
    mut mask: u64,
    mut commands: *const Command,
    mut n_commands: u64,
    mut lit_depth: *const u8,
    mut lit_bits: *const u16,
    mut cmd_depth: *const u8,
    mut cmd_bits: *const u16,
    mut dist_depth: *const u8,
    mut dist_bits: *const u16,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut pos = start_pos;
        let mut i: u64 = 0;
        i = 0;
        while i < n_commands {
            let cmd = *commands.offset(i as isize);
            let cmd_code = cmd.cmd_prefix_ as u64;
            let mut j: u64 = 0;
            BrotliWriteBits(
                *cmd_depth.offset(cmd_code as isize) as u64,
                *cmd_bits.offset(cmd_code as isize) as u64,
                storage_ix,
                storage,
            );
            StoreCommandExtra(&cmd, storage_ix, storage);
            j = cmd.insert_len_ as u64;
            while j != 0 {
                let literal = *input.offset((pos & mask) as isize);
                BrotliWriteBits(
                    *lit_depth.offset(literal as isize) as u64,
                    *lit_bits.offset(literal as isize) as u64,
                    storage_ix,
                    storage,
                );
                pos = pos.wrapping_add(1);
                j = j.wrapping_sub(1);
            }
            pos = (pos as u64).wrapping_add(CommandCopyLen(&cmd) as u64) as u64;
            if CommandCopyLen(&cmd) != 0 && cmd.cmd_prefix_ as i32 >= 128 {
                let dist_code = (cmd.dist_prefix_ as i32 & 0x3ffi32) as u64;
                let distnumextra = (cmd.dist_prefix_ as i32 >> 10i32) as u32;
                let distextra = cmd.dist_extra_;
                BrotliWriteBits(
                    *dist_depth.offset(dist_code as isize) as u64,
                    *dist_bits.offset(dist_code as isize) as u64,
                    storage_ix,
                    storage,
                );
                BrotliWriteBits(distnumextra as u64, distextra as u64, storage_ix, storage);
            }
            i = i.wrapping_add(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliStoreMetaBlockTrivial(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut start_pos: u64,
    mut length: u64,
    mut mask: u64,
    mut is_last: i32,
    mut params: *const BrotliEncoderParams,
    mut commands: *const Command,
    mut n_commands: u64,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut lit_histo = HistogramLiteral {
            data_: [0; 256],
            total_count_: 0,
            bit_cost_: 0.,
        };
        let mut cmd_histo = HistogramCommand {
            data_: [0; 704],
            total_count_: 0,
            bit_cost_: 0.,
        };
        let mut dist_histo = HistogramDistance {
            data_: [0; 544],
            total_count_: 0,
            bit_cost_: 0.,
        };
        let mut lit_depth: [u8; 256] = [0; 256];
        let mut lit_bits: [u16; 256] = [0; 256];
        let mut cmd_depth: [u8; 704] = [0; 704];
        let mut cmd_bits: [u16; 704] = [0; 704];
        let mut dist_depth: [u8; 140] = [0; 140];
        let mut dist_bits: [u16; 140] = [0; 140];
        let mut tree = 0 as *mut HuffmanTree;
        let mut num_distance_symbols = (*params).dist.alphabet_size_max;
        StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
        HistogramClearLiteral(&mut lit_histo);
        HistogramClearCommand(&mut cmd_histo);
        HistogramClearDistance(&mut dist_histo);
        BuildHistograms(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            &mut lit_histo,
            &mut cmd_histo,
            &mut dist_histo,
        );
        BrotliWriteBits(13, 0, storage_ix, storage);
        tree = if 2 * 704 + 1 > 0 {
            BrotliAllocate(
                m,
                ((2 * 704 + 1i32) as u64).wrapping_mul(::std::mem::size_of::<HuffmanTree>() as u64),
            ) as *mut HuffmanTree
        } else {
            0 as *mut HuffmanTree
        };
        if 0 != 0 || 0 != 0 {
            return;
        }
        BuildAndStoreHuffmanTree(
            (lit_histo.data_).as_mut_ptr(),
            256,
            256,
            tree,
            lit_depth.as_mut_ptr(),
            lit_bits.as_mut_ptr(),
            storage_ix,
            storage,
        );
        BuildAndStoreHuffmanTree(
            (cmd_histo.data_).as_mut_ptr(),
            704,
            704,
            tree,
            cmd_depth.as_mut_ptr(),
            cmd_bits.as_mut_ptr(),
            storage_ix,
            storage,
        );
        BuildAndStoreHuffmanTree(
            (dist_histo.data_).as_mut_ptr(),
            ((16 + 0i32) as u32).wrapping_add(62 << 0 + 1) as u64,
            num_distance_symbols as u64,
            tree,
            dist_depth.as_mut_ptr(),
            dist_bits.as_mut_ptr(),
            storage_ix,
            storage,
        );
        BrotliFree(m, tree as *mut libc::c_void);
        tree = 0 as *mut HuffmanTree;
        StoreDataWithHuffmanCodes(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            lit_depth.as_mut_ptr(),
            lit_bits.as_mut_ptr(),
            cmd_depth.as_mut_ptr(),
            cmd_bits.as_mut_ptr(),
            dist_depth.as_mut_ptr(),
            dist_bits.as_mut_ptr(),
            storage_ix,
            storage,
        );
        if is_last != 0 {
            JumpToByteBoundary(storage_ix, storage);
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliStoreMetaBlockFast(
    mut m: *mut MemoryManager,
    mut input: *const u8,
    mut start_pos: u64,
    mut length: u64,
    mut mask: u64,
    mut is_last: i32,
    mut params: *const BrotliEncoderParams,
    mut commands: *const Command,
    mut n_commands: u64,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut num_distance_symbols = (*params).dist.alphabet_size_max;
        let mut distance_alphabet_bits =
            (Log2FloorNonZero(num_distance_symbols.wrapping_sub(1u32) as u64)).wrapping_add(1);
        StoreCompressedMetaBlockHeader(is_last, length, storage_ix, storage);
        BrotliWriteBits(13, 0, storage_ix, storage);
        if n_commands <= 128 {
            let mut histogram: [u32; 256] = [0; 256];
            let mut pos = start_pos;
            let mut num_literals = 0;
            let mut i: u64 = 0;
            let mut lit_depth: [u8; 256] = [0; 256];
            let mut lit_bits: [u16; 256] = [0; 256];
            i = 0;
            while i < n_commands {
                let cmd = *commands.offset(i as isize);
                let mut j: u64 = 0;
                j = cmd.insert_len_ as u64;
                while j != 0 {
                    histogram[*input.offset((pos & mask) as isize) as usize] =
                        (histogram[*input.offset((pos & mask) as isize) as usize]).wrapping_add(1);
                    pos = pos.wrapping_add(1);
                    j = j.wrapping_sub(1);
                }
                num_literals = (num_literals as u64).wrapping_add(cmd.insert_len_ as u64) as u64;
                pos = (pos as u64).wrapping_add(CommandCopyLen(&cmd) as u64) as u64;
                i = i.wrapping_add(1);
            }
            BrotliBuildAndStoreHuffmanTreeFast(
                m,
                histogram.as_mut_ptr(),
                num_literals,
                8,
                lit_depth.as_mut_ptr(),
                lit_bits.as_mut_ptr(),
                storage_ix,
                storage,
            );
            if 0 != 0 {
                return;
            }
            StoreStaticCommandHuffmanTree(storage_ix, storage);
            StoreStaticDistanceHuffmanTree(storage_ix, storage);
            StoreDataWithHuffmanCodes(
                input,
                start_pos,
                mask,
                commands,
                n_commands,
                lit_depth.as_mut_ptr(),
                lit_bits.as_mut_ptr(),
                kStaticCommandCodeDepth.as_ptr(),
                kStaticCommandCodeBits.as_ptr(),
                kStaticDistanceCodeDepth.as_ptr(),
                kStaticDistanceCodeBits.as_ptr(),
                storage_ix,
                storage,
            );
        } else {
            let mut lit_histo = HistogramLiteral {
                data_: [0; 256],
                total_count_: 0,
                bit_cost_: 0.,
            };
            let mut cmd_histo = HistogramCommand {
                data_: [0; 704],
                total_count_: 0,
                bit_cost_: 0.,
            };
            let mut dist_histo = HistogramDistance {
                data_: [0; 544],
                total_count_: 0,
                bit_cost_: 0.,
            };
            let mut lit_depth_0: [u8; 256] = [0; 256];
            let mut lit_bits_0: [u16; 256] = [0; 256];
            let mut cmd_depth: [u8; 704] = [0; 704];
            let mut cmd_bits: [u16; 704] = [0; 704];
            let mut dist_depth: [u8; 140] = [0; 140];
            let mut dist_bits: [u16; 140] = [0; 140];
            HistogramClearLiteral(&mut lit_histo);
            HistogramClearCommand(&mut cmd_histo);
            HistogramClearDistance(&mut dist_histo);
            BuildHistograms(
                input,
                start_pos,
                mask,
                commands,
                n_commands,
                &mut lit_histo,
                &mut cmd_histo,
                &mut dist_histo,
            );
            BrotliBuildAndStoreHuffmanTreeFast(
                m,
                (lit_histo.data_).as_mut_ptr(),
                lit_histo.total_count_,
                8,
                lit_depth_0.as_mut_ptr(),
                lit_bits_0.as_mut_ptr(),
                storage_ix,
                storage,
            );
            if 0 != 0 {
                return;
            }
            BrotliBuildAndStoreHuffmanTreeFast(
                m,
                (cmd_histo.data_).as_mut_ptr(),
                cmd_histo.total_count_,
                10,
                cmd_depth.as_mut_ptr(),
                cmd_bits.as_mut_ptr(),
                storage_ix,
                storage,
            );
            if 0 != 0 {
                return;
            }
            BrotliBuildAndStoreHuffmanTreeFast(
                m,
                (dist_histo.data_).as_mut_ptr(),
                dist_histo.total_count_,
                distance_alphabet_bits as u64,
                dist_depth.as_mut_ptr(),
                dist_bits.as_mut_ptr(),
                storage_ix,
                storage,
            );
            if 0 != 0 {
                return;
            }
            StoreDataWithHuffmanCodes(
                input,
                start_pos,
                mask,
                commands,
                n_commands,
                lit_depth_0.as_mut_ptr(),
                lit_bits_0.as_mut_ptr(),
                cmd_depth.as_mut_ptr(),
                cmd_bits.as_mut_ptr(),
                dist_depth.as_mut_ptr(),
                dist_bits.as_mut_ptr(),
                storage_ix,
                storage,
            );
        }
        if is_last != 0 {
            JumpToByteBoundary(storage_ix, storage);
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliStoreUncompressedMetaBlock(
    mut is_final_block: i32,
    mut input: *const u8,
    mut position: u64,
    mut mask: u64,
    mut len: u64,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let mut masked_pos = position & mask;
        BrotliStoreUncompressedMetaBlockHeader(len, storage_ix, storage);
        JumpToByteBoundary(storage_ix, storage);
        if masked_pos.wrapping_add(len) > mask.wrapping_add(1) {
            let mut len1 = mask.wrapping_add(1).wrapping_sub(masked_pos);
            memcpy(
                &mut *storage.offset((*storage_ix >> 3i32) as isize) as *mut u8
                    as *mut libc::c_void,
                &*input.offset(masked_pos as isize) as *const u8 as *const libc::c_void,
                len1,
            );
            *storage_ix = (*storage_ix as u64).wrapping_add(len1 << 3) as u64;
            len = (len as u64).wrapping_sub(len1) as u64;
            masked_pos = 0;
        }
        memcpy(
            &mut *storage.offset((*storage_ix >> 3i32) as isize) as *mut u8 as *mut libc::c_void,
            &*input.offset(masked_pos as isize) as *const u8 as *const libc::c_void,
            len,
        );
        *storage_ix = (*storage_ix as u64).wrapping_add(len << 3) as u64;
        BrotliWriteBitsPrepareStorage(*storage_ix, storage);
        if is_final_block != 0 {
            BrotliWriteBits(1, 1, storage_ix, storage);
            BrotliWriteBits(1, 1, storage_ix, storage);
            JumpToByteBoundary(storage_ix, storage);
        }
    }
}

extern "C" fn run_static_initializers() {
    unsafe {
        kSymbolMask = (1u32 << 9).wrapping_sub(1);
    }
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
