pub const NULL: i32 = 0;
use c2rust_bitfields;
use libc;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    static _kBrotliPrefixCodeRanges: [BrotliPrefixCodeRange; 26];
    static _kBrotliContextLookupTable: [u8; 2048];
    fn BrotliTransformDictionaryWord(
        dst: *mut u8,
        word: *const u8,
        len: i32,
        transforms: *const BrotliTransforms,
        transform_idx: i32,
    ) -> i32;
    static kBrotliBitMask: [u32; 33];
    fn BrotliWarmupBitReader(br: *mut BrotliBitReader) -> i32;
    fn BrotliSafeReadBits32Slow(br: *mut BrotliBitReader, n_bits: u32, val: *mut u32) -> i32;
    fn BrotliBuildCodeLengthsHuffmanTable(
        root_table: *mut HuffmanCode,
        code_lengths: *const u8,
        count: *mut u16,
    );
    fn BrotliBuildHuffmanTable(
        root_table: *mut HuffmanCode,
        root_bits: i32,
        symbol_lists: *const u16,
        count: *mut u16,
    ) -> u32;
    fn BrotliBuildSimpleHuffmanTable(
        table: *mut HuffmanCode,
        root_bits: i32,
        symbols: *mut u16,
        num_symbols: u32,
    ) -> u32;
    fn BrotliDecoderStateInit(
        s: *mut BrotliDecoderStateInternal,
        alloc_func: brotli_alloc_func,
        free_func: brotli_free_func,
        opaque: *mut libc::c_void,
    ) -> i32;
    fn BrotliDecoderStateCleanup(s: *mut BrotliDecoderStateInternal);
    fn BrotliDecoderStateMetablockBegin(s: *mut BrotliDecoderStateInternal);
    fn BrotliDecoderStateCleanupAfterMetablock(s: *mut BrotliDecoderStateInternal);
    fn BrotliDecoderHuffmanTreeGroupInit(
        s: *mut BrotliDecoderStateInternal,
        group: *mut HuffmanTreeGroup,
        alphabet_size_max: u32,
        alphabet_size_limit: u32,
        ntrees: u32,
    ) -> i32;
}
pub type brotli_alloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct BrotliDecoderStateStruct {
    pub state: u32,
    pub loop_counter: i32,
    pub br: BrotliBitReader,
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub memory_manager_opaque: *mut libc::c_void,
    pub buffer: C2RustUnnamed_0,
    pub buffer_length: u32,
    pub pos: i32,
    pub max_backward_distance: i32,
    pub max_distance: i32,
    pub ringbuffer_size: i32,
    pub ringbuffer_mask: i32,
    pub dist_rb_idx: i32,
    pub dist_rb: [i32; 4],
    pub error_code: i32,
    pub ringbuffer: *mut u8,
    pub ringbuffer_end: *mut u8,
    pub htree_command: *mut HuffmanCode,
    pub context_lookup: *const u8,
    pub context_map_slice: *mut u8,
    pub dist_context_map_slice: *mut u8,
    pub literal_hgroup: HuffmanTreeGroup,
    pub insert_copy_hgroup: HuffmanTreeGroup,
    pub distance_hgroup: HuffmanTreeGroup,
    pub block_type_trees: *mut HuffmanCode,
    pub block_len_trees: *mut HuffmanCode,
    pub trivial_literal_context: i32,
    pub distance_context: i32,
    pub meta_block_remaining_len: i32,
    pub block_length_index: u32,
    pub block_length: [u32; 3],
    pub num_block_types: [u32; 3],
    pub block_type_rb: [u32; 6],
    pub distance_postfix_bits: u32,
    pub num_direct_distance_codes: u32,
    pub num_dist_htrees: u32,
    pub dist_context_map: *mut u8,
    pub literal_htree: *mut HuffmanCode,
    pub dist_htree_index: u8,
    pub copy_length: i32,
    pub distance_code: i32,
    pub rb_roundtrips: u64,
    pub partial_pos_out: u64,
    pub mtf_upper_bound: u32,
    pub mtf: [u32; 65],
    pub substate_metablock_header: u32,
    pub substate_uncompressed: u32,
    pub substate_decode_uint8: u32,
    pub substate_read_block_length: u32,
    #[bitfield(name = "is_last_metablock", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "is_uncompressed", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_metadata", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "should_wrap_ringbuffer", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(
        name = "canny_ringbuffer_allocation",
        ty = "libc::c_uint",
        bits = "4..=4"
    )]
    #[bitfield(name = "large_window", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "size_nibbles", ty = "libc::c_uint", bits = "6..=13")]
    pub is_last_metablock_is_uncompressed_is_metadata_should_wrap_ringbuffer_canny_ringbuffer_allocation_large_window_size_nibbles:
        [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub window_bits: u32,
    pub new_ringbuffer_size: i32,
    pub num_literal_htrees: u32,
    pub context_map: *mut u8,
    pub context_modes: *mut u8,
    pub dictionary: *const BrotliDictionary,
    pub transforms: *const BrotliTransforms,
    pub trivial_literal_contexts: [u32; 8],
    pub arena: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub header: BrotliMetablockHeaderArena,
    pub body: BrotliMetablockBodyArena,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliMetablockBodyArena {
    pub dist_extra_bits: [u8; 544],
    pub dist_offset: [u32; 544],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliMetablockHeaderArena {
    pub substate_tree_group: u32,
    pub substate_context_map: u32,
    pub substate_huffman: u32,
    pub sub_loop_counter: u32,
    pub repeat_code_len: u32,
    pub prev_code_len: u32,
    pub symbol: u32,
    pub repeat: u32,
    pub space: u32,
    pub table: [HuffmanCode; 32],
    pub symbol_lists: *mut u16,
    pub symbols_lists_array: [u16; 720],
    pub next_symbol: [i32; 32],
    pub code_length_code_lengths: [u8; 18],
    pub code_length_histo: [u16; 16],
    pub htree_index: i32,
    pub next: *mut HuffmanCode,
    pub context_index: u32,
    pub max_run_length_prefix: u32,
    pub code: u32,
    pub context_map_table: [HuffmanCode; 646],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode {
    pub bits: u8,
    pub value: u16,
}
pub const BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS: u32 = 5;
pub const BROTLI_STATE_HUFFMAN_COMPLEX: u32 = 4;
pub const BROTLI_STATE_HUFFMAN_SIMPLE_BUILD: u32 = 3;
pub const BROTLI_STATE_HUFFMAN_SIMPLE_READ: u32 = 2;
pub const BROTLI_STATE_HUFFMAN_SIMPLE_SIZE: u32 = 1;
pub const BROTLI_STATE_HUFFMAN_NONE: u32 = 0;
pub const BROTLI_STATE_CONTEXT_MAP_TRANSFORM: u32 = 4;
pub const BROTLI_STATE_CONTEXT_MAP_DECODE: u32 = 3;
pub const BROTLI_STATE_CONTEXT_MAP_HUFFMAN: u32 = 2;
pub const BROTLI_STATE_CONTEXT_MAP_READ_PREFIX: u32 = 1;
pub const BROTLI_STATE_CONTEXT_MAP_NONE: u32 = 0;
pub const BROTLI_STATE_TREE_GROUP_LOOP: u32 = 1;
pub const BROTLI_STATE_TREE_GROUP_NONE: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliTransforms {
    pub prefix_suffix_size: u16,
    pub prefix_suffix: *const u8,
    pub prefix_suffix_map: *const u16,
    pub num_transforms: u32,
    pub transforms: *const u8,
    pub params: *const u8,
    pub cutOffTransforms: [i16; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDictionary {
    pub size_bits_by_length: [u8; 32],
    pub offsets_by_length: [u32; 32],
    pub data_size: u64,
    pub data: *const u8,
}
pub const BROTLI_STATE_READ_BLOCK_LENGTH_SUFFIX: u32 = 1;
pub const BROTLI_STATE_READ_BLOCK_LENGTH_NONE: u32 = 0;
pub const BROTLI_STATE_DECODE_UINT8_LONG: u32 = 2;
pub const BROTLI_STATE_DECODE_UINT8_SHORT: u32 = 1;
pub const BROTLI_STATE_DECODE_UINT8_NONE: u32 = 0;
pub const BROTLI_STATE_UNCOMPRESSED_WRITE: u32 = 1;
pub const BROTLI_STATE_UNCOMPRESSED_NONE: u32 = 0;
pub const BROTLI_STATE_METABLOCK_HEADER_METADATA: u32 = 7;
pub const BROTLI_STATE_METABLOCK_HEADER_BYTES: u32 = 6;
pub const BROTLI_STATE_METABLOCK_HEADER_RESERVED: u32 = 5;
pub const BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED: u32 = 4;
pub const BROTLI_STATE_METABLOCK_HEADER_SIZE: u32 = 3;
pub const BROTLI_STATE_METABLOCK_HEADER_NIBBLES: u32 = 2;
pub const BROTLI_STATE_METABLOCK_HEADER_EMPTY: u32 = 1;
pub const BROTLI_STATE_METABLOCK_HEADER_NONE: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTreeGroup {
    pub htrees: *mut *mut HuffmanCode,
    pub codes: *mut HuffmanCode,
    pub alphabet_size_max: u16,
    pub alphabet_size_limit: u16,
    pub num_htrees: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub u64_0: u64,
    pub u8_0: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliBitReader {
    pub val_: u64,
    pub bit_pos_: u32,
    pub next_in: *const u8,
    pub avail_in: u64,
}
pub const BROTLI_STATE_DONE: u32 = 26;
pub const BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_BODY: u32 = 25;
pub const BROTLI_STATE_TREE_GROUP: u32 = 24;
pub const BROTLI_STATE_CONTEXT_MAP_2: u32 = 23;
pub const BROTLI_STATE_CONTEXT_MAP_1: u32 = 22;
pub const BROTLI_STATE_HUFFMAN_CODE_3: u32 = 21;
pub const BROTLI_STATE_HUFFMAN_CODE_2: u32 = 20;
pub const BROTLI_STATE_HUFFMAN_CODE_1: u32 = 19;
pub const BROTLI_STATE_HUFFMAN_CODE_0: u32 = 18;
pub const BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_HEADER: u32 = 17;
pub const BROTLI_STATE_COMMAND_POST_WRITE_2: u32 = 16;
pub const BROTLI_STATE_COMMAND_POST_WRITE_1: u32 = 15;
pub const BROTLI_STATE_METABLOCK_DONE: u32 = 14;
pub const BROTLI_STATE_COMMAND_INNER_WRITE: u32 = 13;
pub const BROTLI_STATE_METADATA: u32 = 12;
pub const BROTLI_STATE_UNCOMPRESSED: u32 = 11;
pub const BROTLI_STATE_COMMAND_POST_WRAP_COPY: u32 = 10;
pub const BROTLI_STATE_COMMAND_POST_DECODE_LITERALS: u32 = 9;
pub const BROTLI_STATE_COMMAND_INNER: u32 = 8;
pub const BROTLI_STATE_COMMAND_BEGIN: u32 = 7;
pub const BROTLI_STATE_CONTEXT_MODES: u32 = 6;
pub const BROTLI_STATE_METABLOCK_HEADER_2: u32 = 5;
pub const BROTLI_STATE_METABLOCK_HEADER: u32 = 4;
pub const BROTLI_STATE_METABLOCK_BEGIN: u32 = 3;
pub const BROTLI_STATE_INITIALIZE: u32 = 2;
pub const BROTLI_STATE_LARGE_WINDOW_BITS: u32 = 1;
pub const BROTLI_STATE_UNINITED: u32 = 0;
pub type BrotliDecoderState = BrotliDecoderStateStruct;
pub const BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT: u32 = 3;
pub const BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT: u32 = 2;
pub const BROTLI_DECODER_RESULT_SUCCESS: u32 = 1;
pub const BROTLI_DECODER_RESULT_ERROR: u32 = 0;
pub const BROTLI_DECODER_ERROR_UNREACHABLE: i32 = -31;
pub const BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES: i32 = -30;
pub const BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2: i32 = -27;
pub const BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1: i32 = -26;
pub const BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP: i32 = -25;
pub const BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS: i32 = -22;
pub const BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES: i32 = -21;
pub const BROTLI_DECODER_ERROR_INVALID_ARGUMENTS: i32 = -20;
pub const BROTLI_DECODER_ERROR_DICTIONARY_NOT_SET: i32 = -19;
pub const BROTLI_DECODER_ERROR_FORMAT_DISTANCE: i32 = -16;
pub const BROTLI_DECODER_ERROR_FORMAT_PADDING_2: i32 = -15;
pub const BROTLI_DECODER_ERROR_FORMAT_PADDING_1: i32 = -14;
pub const BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS: i32 = -13;
pub const BROTLI_DECODER_ERROR_FORMAT_DICTIONARY: i32 = -12;
pub const BROTLI_DECODER_ERROR_FORMAT_TRANSFORM: i32 = -11;
pub const BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2: i32 = -10;
pub const BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1: i32 = -9;
pub const BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT: i32 = -8;
pub const BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE: i32 = -7;
pub const BROTLI_DECODER_ERROR_FORMAT_CL_SPACE: i32 = -6;
pub const BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME: i32 = -5;
pub const BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET: i32 = -4;
pub const BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE: i32 = -3;
pub const BROTLI_DECODER_ERROR_FORMAT_RESERVED: i32 = -2;
pub const BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE: i32 = -1;
pub const BROTLI_DECODER_NEEDS_MORE_OUTPUT: i32 = 3;
pub const BROTLI_DECODER_NEEDS_MORE_INPUT: i32 = 2;
pub const BROTLI_DECODER_SUCCESS: i32 = 1;
pub const BROTLI_DECODER_PARAM_LARGE_WINDOW: u32 = 1;
pub const BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION: u32 = 0;
pub type BrotliDecoderStateInternal = BrotliDecoderStateStruct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliBitReaderState {
    pub val_: u64,
    pub bit_pos_: u32,
    pub next_in: *const u8,
    pub avail_in: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliPrefixCodeRange {
    pub offset: u16,
    pub nbits: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CmdLutElement {
    pub insert_len_extra_bits: u8,
    pub copy_len_extra_bits: u8,
    pub distance_code: i8,
    pub context: u8,
    pub insert_len_offset: u16,
    pub copy_len_offset: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliDistanceCodeLimit {
    pub max_alphabet_size: u32,
    pub max_distance: u32,
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
extern "C" fn BrotliCalculateDistanceCodeLimit(
    mut max_distance: u32,
    mut npostfix: u32,
    mut ndirect: u32,
) -> BrotliDistanceCodeLimit {
    let mut result = BrotliDistanceCodeLimit {
        max_alphabet_size: 0,
        max_distance: 0,
    };
    unsafe {
        if max_distance <= ndirect {
            result.max_alphabet_size = max_distance.wrapping_add(16);
            result.max_distance = max_distance;
            return result;
        } else {
            let mut forbidden_distance = max_distance.wrapping_add(1);
            let mut offset = forbidden_distance.wrapping_sub(ndirect).wrapping_sub(1);
            let mut ndistbits = 0;
            let mut tmp: u32 = 0;
            let mut half: u32 = 0;
            let mut group: u32 = 0;
            let mut postfix = (1u32 << npostfix).wrapping_sub(1);
            let mut extra: u32 = 0;
            let mut start: u32 = 0;
            offset = (offset >> npostfix).wrapping_add(4);
            tmp = offset.wrapping_div(2);
            while tmp != 0 {
                ndistbits = ndistbits.wrapping_add(1);
                tmp = tmp >> 1;
            }
            ndistbits = ndistbits.wrapping_sub(1);
            half = offset >> ndistbits & 1;
            group = ndistbits.wrapping_sub(1) << 1 | half;
            if group == 0 {
                result.max_alphabet_size = ndirect.wrapping_add(16);
                result.max_distance = ndirect;
                return result;
            }
            group = group.wrapping_sub(1);
            ndistbits = (group >> 1i32).wrapping_add(1);
            extra = (1u32 << ndistbits).wrapping_sub(1);
            start = (1u32 << ndistbits.wrapping_add(1)).wrapping_sub(4);
            start = (start).wrapping_add((group & 1) << ndistbits) as u32;
            result.max_alphabet_size = (group << npostfix | postfix)
                .wrapping_add(ndirect)
                .wrapping_add(16)
                .wrapping_add(1);
            result.max_distance = (start.wrapping_add(extra) << npostfix)
                .wrapping_add(postfix)
                .wrapping_add(ndirect)
                .wrapping_add(1);
            return result;
        };
    }
}

#[inline(always)]
extern "C" fn BitMask(mut n: u32) -> u32 {
    unsafe {
        if 0 != 0 || 0 != 0 {
            return !(0xffffffff << n);
        } else {
            return kBrotliBitMask[n as usize];
        };
    }
}

#[inline(always)]
extern "C" fn BrotliBitReaderSaveState(
    from: *mut BrotliBitReader,
    mut to: *mut BrotliBitReaderState,
) {
    unsafe {
        (*to).val_ = (*from).val_;
        (*to).bit_pos_ = (*from).bit_pos_;
        let ref mut fresh0 = (*to).next_in;
        *fresh0 = (*from).next_in;
        (*to).avail_in = (*from).avail_in;
    }
}

#[inline(always)]
extern "C" fn BrotliBitReaderRestoreState(
    to: *mut BrotliBitReader,
    mut from: *mut BrotliBitReaderState,
) {
    unsafe {
        (*to).val_ = (*from).val_;
        (*to).bit_pos_ = (*from).bit_pos_;
        let ref mut fresh1 = (*to).next_in;
        *fresh1 = (*from).next_in;
        (*to).avail_in = (*from).avail_in;
    }
}

#[inline(always)]
extern "C" fn BrotliGetAvailableBits(mut br: *const BrotliBitReader) -> u32 {
    unsafe {
        return ((if 1 != 0 { 64i32 } else { 32 }) as u32).wrapping_sub((*br).bit_pos_);
    }
}

#[inline(always)]
extern "C" fn BrotliGetRemainingBytes(mut br: *mut BrotliBitReader) -> u64 {
    unsafe {
        static mut kCap: u64 = 1 << 30;
        if (*br).avail_in > kCap {
            return kCap;
        }
        return ((*br).avail_in).wrapping_add((BrotliGetAvailableBits(br) >> 3i32) as u64);
    }
}

#[inline(always)]
extern "C" fn BrotliCheckInputAmount(br: *mut BrotliBitReader, mut num: u64) -> i32 {
    unsafe {
        return if (*br).avail_in >= num { 1 } else { 0 };
    }
}

#[inline(always)]
extern "C" fn BrotliFillBitWindow(br: *mut BrotliBitReader, mut n_bits: u32) {
    unsafe {
        if 0 == 0 && 0 != 0 && n_bits <= 8 {
            if (*br).bit_pos_ >= 56 {
                (*br).val_ >>= 56;
                let ref mut fresh2 = (*br).bit_pos_;
                *fresh2 ^= 56;
                let ref mut fresh3 = (*br).val_;
                *fresh3 |= BrotliUnalignedRead64((*br).next_in as *const libc::c_void) << 8;
                let ref mut fresh4 = (*br).avail_in;
                *fresh4 = (*fresh4 as u64).wrapping_sub(7) as u64;
                let ref mut fresh5 = (*br).next_in;
                *fresh5 = (*fresh5).offset(7 as isize);
            }
        } else if 0 == 0 && 0 != 0 && n_bits <= 16 {
            if (*br).bit_pos_ >= 48 {
                (*br).val_ >>= 48;
                let ref mut fresh6 = (*br).bit_pos_;
                *fresh6 ^= 48;
                let ref mut fresh7 = (*br).val_;
                *fresh7 |= BrotliUnalignedRead64((*br).next_in as *const libc::c_void) << 16;
                let ref mut fresh8 = (*br).avail_in;
                *fresh8 = (*fresh8 as u64).wrapping_sub(6) as u64;
                let ref mut fresh9 = (*br).next_in;
                *fresh9 = (*fresh9).offset(6 as isize);
            }
        } else if (*br).bit_pos_ >= 32 {
            (*br).val_ >>= 32;
            let ref mut fresh10 = (*br).bit_pos_;
            *fresh10 ^= 32;
            let ref mut fresh11 = (*br).val_;
            *fresh11 |= (BrotliUnalignedRead32((*br).next_in as *const libc::c_void) as u64) << 32;
            let ref mut fresh12 = (*br).avail_in;
            *fresh12 =
                (*fresh12 as u64).wrapping_sub(::std::mem::size_of::<u64>() as u64 >> 1) as u64;
            let ref mut fresh13 = (*br).next_in;
            *fresh13 = (*fresh13).offset((::std::mem::size_of::<u64>() as u64 >> 1i32) as isize);
        }
    }
}

#[inline(always)]
extern "C" fn BrotliFillBitWindow16(br: *mut BrotliBitReader) {
    unsafe {
        BrotliFillBitWindow(br, 17);
    }
}

#[inline(always)]
extern "C" fn BrotliPullByte(br: *mut BrotliBitReader) -> i32 {
    unsafe {
        if (*br).avail_in == 0 {
            return 0;
        };
        (*br).val_ >>= 8;
        let ref mut fresh14 = (*br).val_;
        *fresh14 |= (*(*br).next_in as u64) << 56;
        let ref mut fresh15 = (*br).bit_pos_;
        *fresh15 = (*fresh15 as u32).wrapping_sub(8) as u32;
        let ref mut fresh16 = (*br).avail_in;
        *fresh16 = (*fresh16).wrapping_sub(1);
        let ref mut fresh17 = (*br).next_in;
        *fresh17 = (*fresh17).offset(1);
        return 1;
    }
}

#[inline(always)]
extern "C" fn BrotliGetBitsUnmasked(br: *mut BrotliBitReader) -> u64 {
    unsafe {
        return (*br).val_ >> (*br).bit_pos_;
    }
}

#[inline(always)]
extern "C" fn BrotliGet16BitsUnmasked(br: *mut BrotliBitReader) -> u32 {
    unsafe {
        BrotliFillBitWindow(br, 16);
        return BrotliGetBitsUnmasked(br) as u32;
    }
}

#[inline(always)]
extern "C" fn BrotliGetBits(br: *mut BrotliBitReader, mut n_bits: u32) -> u32 {
    unsafe {
        BrotliFillBitWindow(br, n_bits);
        return BrotliGetBitsUnmasked(br) as u32 & BitMask(n_bits);
    }
}

#[inline(always)]
extern "C" fn BrotliSafeGetBits(
    br: *mut BrotliBitReader,
    mut n_bits: u32,
    mut val: *mut u32,
) -> i32 {
    unsafe {
        while BrotliGetAvailableBits(br) < n_bits {
            if BrotliPullByte(br) == 0 {
                return 0;
            }
        }
        *val = BrotliGetBitsUnmasked(br) as u32 & BitMask(n_bits);
        return 1;
    }
}

#[inline(always)]
extern "C" fn BrotliDropBits(br: *mut BrotliBitReader, mut n_bits: u32) {
    unsafe {
        let ref mut fresh18 = (*br).bit_pos_;
        *fresh18 = (*fresh18 as u32).wrapping_add(n_bits) as u32;
    }
}

#[inline(always)]
extern "C" fn BrotliBitReaderUnload(mut br: *mut BrotliBitReader) {
    unsafe {
        let mut unused_bytes = BrotliGetAvailableBits(br) >> 3;
        let mut unused_bits = unused_bytes << 3;
        let ref mut fresh19 = (*br).avail_in;
        *fresh19 = (*fresh19 as u64).wrapping_add(unused_bytes as u64) as u64;
        let ref mut fresh20 = (*br).next_in;
        *fresh20 = (*fresh20).offset(-(unused_bytes as isize));
        if unused_bits as u64 == (::std::mem::size_of::<u64>() as u64) << 3 {
            (*br).val_ = 0;
        } else {
            (*br).val_ <<= unused_bits;
        }
        let ref mut fresh21 = (*br).bit_pos_;
        *fresh21 = (*fresh21 as u32).wrapping_add(unused_bits) as u32;
    }
}

#[inline(always)]
extern "C" fn BrotliTakeBits(br: *mut BrotliBitReader, mut n_bits: u32, mut val: *mut u32) {
    unsafe {
        *val = BrotliGetBitsUnmasked(br) as u32 & BitMask(n_bits);
        BrotliDropBits(br, n_bits);
    }
}

#[inline(always)]
extern "C" fn BrotliReadBits24(br: *mut BrotliBitReader, mut n_bits: u32) -> u32 {
    unsafe {
        if 1 != 0 || n_bits <= 16 {
            let mut val: u32 = 0;
            BrotliFillBitWindow(br, n_bits);
            BrotliTakeBits(br, n_bits, &mut val);
            return val;
        } else {
            let mut low_val: u32 = 0;
            let mut high_val: u32 = 0;
            BrotliFillBitWindow(br, 16);
            BrotliTakeBits(br, 16, &mut low_val);
            BrotliFillBitWindow(br, 8);
            BrotliTakeBits(br, n_bits.wrapping_sub(16), &mut high_val);
            return low_val | high_val << 16;
        };
    }
}

#[inline(always)]
extern "C" fn BrotliReadBits32(br: *mut BrotliBitReader, mut n_bits: u32) -> u32 {
    unsafe {
        if 1 != 0 || n_bits <= 16 {
            let mut val: u32 = 0;
            BrotliFillBitWindow(br, n_bits);
            BrotliTakeBits(br, n_bits, &mut val);
            return val;
        } else {
            let mut low_val: u32 = 0;
            let mut high_val: u32 = 0;
            BrotliFillBitWindow(br, 16);
            BrotliTakeBits(br, 16, &mut low_val);
            BrotliFillBitWindow(br, 16);
            BrotliTakeBits(br, n_bits.wrapping_sub(16), &mut high_val);
            return low_val | high_val << 16;
        };
    }
}

#[inline(always)]
extern "C" fn BrotliSafeReadBits(
    br: *mut BrotliBitReader,
    mut n_bits: u32,
    mut val: *mut u32,
) -> i32 {
    unsafe {
        while BrotliGetAvailableBits(br) < n_bits {
            if BrotliPullByte(br) == 0 {
                return 0;
            }
        }
        BrotliTakeBits(br, n_bits, val);
        return 1;
    }
}

#[inline(always)]
extern "C" fn BrotliSafeReadBits32(
    br: *mut BrotliBitReader,
    mut n_bits: u32,
    mut val: *mut u32,
) -> i32 {
    unsafe {
        if 1 != 0 || n_bits <= 24 {
            while BrotliGetAvailableBits(br) < n_bits {
                if BrotliPullByte(br) == 0 {
                    return 0;
                }
            }
            BrotliTakeBits(br, n_bits, val);
            return 1;
        } else {
            return BrotliSafeReadBits32Slow(br, n_bits, val);
        };
    }
}

#[inline(always)]
extern "C" fn BrotliJumpToByteBoundary(mut br: *mut BrotliBitReader) -> i32 {
    unsafe {
        let mut pad_bits_count = BrotliGetAvailableBits(br) & 0x7;
        let mut pad_bits = 0;
        if pad_bits_count != 0 {
            BrotliTakeBits(br, pad_bits_count, &mut pad_bits);
        }
        return if pad_bits == 0 { 1 } else { 0 };
    }
}

#[inline(always)]
extern "C" fn BrotliCopyBytes(mut dest: *mut u8, mut br: *mut BrotliBitReader, mut num: u64) {
    unsafe {
        while BrotliGetAvailableBits(br) >= 8 && num > 0 {
            *dest = BrotliGetBitsUnmasked(br) as u8;
            BrotliDropBits(br, 8);
            dest = dest.offset(1);
            num = num.wrapping_sub(1);
        }
        memcpy(
            dest as *mut libc::c_void,
            (*br).next_in as *const libc::c_void,
            num,
        );
        let ref mut fresh22 = (*br).avail_in;
        *fresh22 = (*fresh22 as u64).wrapping_sub(num) as u64;
        let ref mut fresh23 = (*br).next_in;
        *fresh23 = (*fresh23).offset(num as isize);
    }
}

static mut kCmdLut: [CmdLutElement; 704] = [
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0,
            insert_len_offset: 0,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x1,
            insert_len_offset: 0,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x2,
            insert_len_offset: 0,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0,
            insert_len_offset: 0x1,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x1,
            insert_len_offset: 0x1,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x2,
            insert_len_offset: 0x1,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0,
            insert_len_offset: 0x2,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x1,
            insert_len_offset: 0x2,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x2,
            insert_len_offset: 0x2,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0,
            insert_len_offset: 0x3,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x1,
            insert_len_offset: 0x3,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x2,
            insert_len_offset: 0x3,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0,
            insert_len_offset: 0x4,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x1,
            insert_len_offset: 0x4,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x2,
            insert_len_offset: 0x4,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0,
            insert_len_offset: 0x5,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x1,
            insert_len_offset: 0x5,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x2,
            insert_len_offset: 0x5,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0,
            insert_len_offset: 0x6,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x1,
            insert_len_offset: 0x6,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x2,
            insert_len_offset: 0x6,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0,
            insert_len_offset: 0x8,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x1,
            insert_len_offset: 0x8,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x2,
            insert_len_offset: 0x8,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x1,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x2,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x3,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x4,
            distance_code: 0,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x1,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x1,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x1,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x2,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x2,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x2,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x3,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x3,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x3,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x4,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x4,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x4,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x5,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x5,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x5,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x6,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x6,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x6,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x8,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x8,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x8,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0xa,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0xa,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0xa,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0xe,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0xe,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0xe,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x12,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x12,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x12,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x22,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x22,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x22,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x32,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x32,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x32,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x42,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x42,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x42,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x62,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x62,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x62,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x2,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x3,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x4,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x6,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x1,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x8,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x82,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x82,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x82,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x142,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x142,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x142,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x242,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x242,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x242,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x442,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x442,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x442,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x842,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x842,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x842,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x2,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x1,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x3,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x2,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x4,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x5,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x7,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x8,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x9,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xa,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x2,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xe,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x12,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x3,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1a,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x22,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x4,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x32,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x42,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x5,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x62,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0xa,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x1,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0xc,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0xe,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x2,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x12,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x16,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x3,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x1e,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x26,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x4,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x36,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x6,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x82,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x7,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0xc2,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x8,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x142,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x9,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x242,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xa,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x442,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xc,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x842,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0xe,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x1842,
            copy_len_offset: 0x846,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x46,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x5,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x66,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x6,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x86,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x7,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0xc6,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x8,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x146,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x9,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x246,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0xa,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x446,
        };
        init
    },
    {
        let mut init = CmdLutElement {
            insert_len_extra_bits: 0x18,
            copy_len_extra_bits: 0x18,
            distance_code: -1 as i8,
            context: 0x3,
            insert_len_offset: 0x5842,
            copy_len_offset: 0x846,
        };
        init
    },
];
static mut kRingBufferWriteAheadSlack: u32 = 42;
static mut kCodeLengthCodeOrder: [u8; 18] =
    [1, 2, 3, 4, 0, 5, 17, 6, 16, 7, 8, 9, 10, 11, 12, 13, 14, 15];
static mut kCodeLengthPrefixLength: [u8; 16] = [2, 2, 2, 3, 2, 2, 2, 4, 2, 2, 2, 3, 2, 2, 2, 4];
static mut kCodeLengthPrefixValue: [u8; 16] = [0, 4, 3, 2, 0, 4, 3, 1, 0, 4, 3, 2, 0, 4, 3, 5];
#[no_mangle]
pub extern "C" fn BrotliDecoderSetParameter(
    mut state: *mut BrotliDecoderStateInternal,
    mut p: u32,
    mut value: u32,
) -> i32 {
    unsafe {
        if (*state).state as u32 != BROTLI_STATE_UNINITED as u32 {
            return 0;
        }
        match p as u32 {
            0 => {
                (*state).set_canny_ringbuffer_allocation((if value != 0 { 0 } else { 1 }) as u32);
                return 1;
            }
            1 => {
                (*state).set_large_window((if value != 0 { 1 } else { 0 }) as u32);
                return 1;
            }
            _ => return 0,
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderCreateInstance(
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut libc::c_void,
) -> *mut BrotliDecoderState {
    unsafe {
        let mut state = 0 as *mut BrotliDecoderStateInternal;
        if alloc_func.is_none() && free_func.is_none() {
            state = malloc(::std::mem::size_of::<BrotliDecoderStateInternal>() as u64)
                as *mut BrotliDecoderStateInternal;
        } else if alloc_func.is_some() && free_func.is_some() {
            state = alloc_func.expect("non-null function pointer")(
                opaque,
                ::std::mem::size_of::<BrotliDecoderStateInternal>() as u64,
            ) as *mut BrotliDecoderStateInternal;
        }
        if state.is_null() {
            return 0 as *mut BrotliDecoderState;
        }
        if BrotliDecoderStateInit(state, alloc_func, free_func, opaque) == 0 {
            if alloc_func.is_none() && free_func.is_none() {
                free(state as *mut libc::c_void);
            } else if alloc_func.is_some() && free_func.is_some() {
                free_func.expect("non-null function pointer")(opaque, state as *mut libc::c_void);
            }
            return 0 as *mut BrotliDecoderState;
        }
        return state;
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderDestroyInstance(mut state: *mut BrotliDecoderStateInternal) {
    unsafe {
        if state.is_null() {
            return;
        } else {
            let mut free_func: brotli_free_func = (*state).free_func;
            let mut opaque = (*state).memory_manager_opaque;
            BrotliDecoderStateCleanup(state);
            free_func.expect("non-null function pointer")(opaque, state as *mut libc::c_void);
        };
    }
}

#[inline(never)]
extern "C" fn SaveErrorCode(mut s: *mut BrotliDecoderStateInternal, mut e: i32) -> u32 {
    unsafe {
        (*s).error_code = e as i32;
        match e as i32 {
            1 => return BROTLI_DECODER_RESULT_SUCCESS,
            2 => return BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT,
            3 => return BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT,
            _ => return BROTLI_DECODER_RESULT_ERROR,
        };
    }
}

extern "C" fn DecodeWindowBits(
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
) -> i32 {
    unsafe {
        let mut n: u32 = 0;
        let mut large_window = (*s).large_window() as i32;
        (*s).set_large_window(0);
        BrotliTakeBits(br, 1, &mut n);
        if n == 0 {
            (*s).window_bits = 16;
            return BROTLI_DECODER_SUCCESS;
        }
        BrotliTakeBits(br, 3, &mut n);
        if n != 0 {
            (*s).window_bits = 17u32.wrapping_add(n);
            return BROTLI_DECODER_SUCCESS;
        }
        BrotliTakeBits(br, 3, &mut n);
        if n == 1 {
            if large_window != 0 {
                BrotliTakeBits(br, 1, &mut n);
                if n == 1 {
                    return BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS as i32;
                };
                (*s).set_large_window(1);
                return BROTLI_DECODER_SUCCESS;
            } else {
                return BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS as i32;
            }
        }
        if n != 0 {
            (*s).window_bits = 8u32.wrapping_add(n);
            return BROTLI_DECODER_SUCCESS;
        };
        (*s).window_bits = 17;
        return BROTLI_DECODER_SUCCESS;
    }
}

#[inline(always)]
extern "C" fn memmove16(mut dst: *mut u8, mut src: *mut u8) {
    unsafe {
        let mut buffer: [u32; 4] = [0; 4];
        memcpy(
            buffer.as_mut_ptr() as *mut libc::c_void,
            src as *const libc::c_void,
            16,
        );
        memcpy(
            dst as *mut libc::c_void,
            buffer.as_mut_ptr() as *const libc::c_void,
            16,
        );
    }
}

#[inline(never)]
extern "C" fn DecodeVarLenUint8(
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
    mut value: *mut u32,
) -> i32 {
    unsafe {
        let mut bits: u32 = 0;
        let mut current_block_24: u64;
        match (*s).substate_decode_uint8 as u32 {
            0 => {
                if (BrotliSafeReadBits(br, 1, &mut bits) == 0) as i64 != 0 {
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                if bits == 0 {
                    *value = 0;
                    return BROTLI_DECODER_SUCCESS;
                }
                current_block_24 = 8496924045286093097;
            }
            1 => {
                current_block_24 = 8496924045286093097;
            }
            2 => {
                current_block_24 = 2162225893322838330;
            }
            _ => {
                return BROTLI_DECODER_ERROR_UNREACHABLE as i32;
            }
        }
        match current_block_24 {
            8496924045286093097 => {
                if (BrotliSafeReadBits(br, 3, &mut bits) == 0) as i64 != 0 {
                    (*s).substate_decode_uint8 = BROTLI_STATE_DECODE_UINT8_SHORT;
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                if bits == 0 {
                    *value = 1;
                    (*s).substate_decode_uint8 = BROTLI_STATE_DECODE_UINT8_NONE;
                    return BROTLI_DECODER_SUCCESS;
                }
                *value = bits;
            }
            _ => {}
        }
        if (BrotliSafeReadBits(br, *value, &mut bits) == 0) as i64 != 0 {
            (*s).substate_decode_uint8 = BROTLI_STATE_DECODE_UINT8_LONG;
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        *value = (1u32 << *value).wrapping_add(bits);
        (*s).substate_decode_uint8 = BROTLI_STATE_DECODE_UINT8_NONE;
        return BROTLI_DECODER_SUCCESS;
    }
}

#[inline(never)]
extern "C" fn DecodeMetaBlockLength(
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
) -> i32 {
    unsafe {
        let mut bits: u32 = 0;
        let mut i: i32 = 0;
        loop {
            's_372: {
                let mut current_block_76: u64;
                match (*s).substate_metablock_header as u32 {
                    0 => {
                        if BrotliSafeReadBits(br, 1, &mut bits) == 0 {
                            return BROTLI_DECODER_NEEDS_MORE_INPUT;
                        };
                        (*s).set_is_last_metablock((if bits != 0 { 1 } else { 0 }) as u32);
                        (*s).meta_block_remaining_len = 0;
                        (*s).set_is_uncompressed(0);
                        (*s).set_is_metadata(0);
                        if (*s).is_last_metablock() == 0 {
                            (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NIBBLES;
                            current_block_76 = 3229571381435211107;
                        } else {
                            (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_EMPTY;
                            current_block_76 = 4247893664202276398;
                        }
                    }
                    1 => {
                        current_block_76 = 4247893664202276398;
                    }
                    2 => {
                        current_block_76 = 3591041403858572591;
                    }
                    3 => {
                        current_block_76 = 17332659574749544158;
                    }
                    4 => {
                        current_block_76 = 15076157755807638962;
                    }
                    5 => {
                        if BrotliSafeReadBits(br, 1, &mut bits) == 0 {
                            return BROTLI_DECODER_NEEDS_MORE_INPUT;
                        }
                        if bits != 0 {
                            return BROTLI_DECODER_ERROR_FORMAT_RESERVED as i32;
                        };
                        (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_BYTES;
                        current_block_76 = 9322584641567792192;
                    }
                    6 => {
                        current_block_76 = 9322584641567792192;
                    }
                    7 => {
                        current_block_76 = 7544172152923346656;
                    }
                    _ => {
                        return BROTLI_DECODER_ERROR_UNREACHABLE as i32;
                    }
                }
                match current_block_76 {
                    9322584641567792192 => {
                        if BrotliSafeReadBits(br, 2, &mut bits) == 0 {
                            return BROTLI_DECODER_NEEDS_MORE_INPUT;
                        }
                        if bits == 0 {
                            (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NONE;
                            return BROTLI_DECODER_SUCCESS;
                        };
                        (*s).set_size_nibbles(bits);
                        (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_METADATA;
                        current_block_76 = 7544172152923346656;
                    }
                    4247893664202276398 => {
                        if BrotliSafeReadBits(br, 1, &mut bits) == 0 {
                            return BROTLI_DECODER_NEEDS_MORE_INPUT;
                        }
                        if bits != 0 {
                            (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NONE;
                            return BROTLI_DECODER_SUCCESS;
                        };
                        (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NIBBLES;
                        current_block_76 = 3591041403858572591;
                    }
                    _ => {}
                }
                match current_block_76 {
                    7544172152923346656 => {
                        i = (*s).loop_counter;
                        while i < (*s).size_nibbles() as i32 {
                            if BrotliSafeReadBits(br, 8, &mut bits) == 0 {
                                (*s).loop_counter = i;
                                return BROTLI_DECODER_NEEDS_MORE_INPUT;
                            }
                            if i + 1 == (*s).size_nibbles() as i32
                                && (*s).size_nibbles() as i32 > 1
                                && bits == 0
                            {
                                return BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE as i32;
                            };
                            (*s).meta_block_remaining_len |= (bits << i * 8i32) as i32;
                            i += 1;
                        }
                        let ref mut fresh25 = (*s).meta_block_remaining_len;
                        *fresh25 += 1;
                        (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NONE;
                        return BROTLI_DECODER_SUCCESS;
                    }
                    3591041403858572591 => {
                        if BrotliSafeReadBits(br, 2, &mut bits) == 0 {
                            return BROTLI_DECODER_NEEDS_MORE_INPUT;
                        };
                        (*s).set_size_nibbles(bits.wrapping_add(4) as u32);
                        (*s).loop_counter = 0;
                        if bits == 3 {
                            (*s).set_is_metadata(1);
                            (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_RESERVED;
                            current_block_76 = 3229571381435211107;
                        } else {
                            (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_SIZE;
                            current_block_76 = 17332659574749544158;
                        }
                    }
                    _ => {}
                }
                match current_block_76 {
                    17332659574749544158 => {
                        i = (*s).loop_counter;
                        while i < (*s).size_nibbles() as i32 {
                            if BrotliSafeReadBits(br, 4, &mut bits) == 0 {
                                (*s).loop_counter = i;
                                return BROTLI_DECODER_NEEDS_MORE_INPUT;
                            }
                            if i + 1 == (*s).size_nibbles() as i32
                                && (*s).size_nibbles() as i32 > 4
                                && bits == 0
                            {
                                return BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE as i32;
                            };
                            (*s).meta_block_remaining_len |= (bits << i * 4i32) as i32;
                            i += 1;
                        }
                        (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED;
                    }
                    3229571381435211107 => {
                        break 's_372;
                    }
                    _ => {}
                }
                if (*s).is_last_metablock() == 0 {
                    if BrotliSafeReadBits(br, 1, &mut bits) == 0 {
                        return BROTLI_DECODER_NEEDS_MORE_INPUT;
                    };
                    (*s).set_is_uncompressed((if bits != 0 { 1 } else { 0 }) as u32);
                }
                let ref mut fresh24 = (*s).meta_block_remaining_len;
                *fresh24 += 1;
                (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NONE;
                return BROTLI_DECODER_SUCCESS;
            }
        }
    }
}

#[inline(always)]
extern "C" fn DecodeSymbol(
    mut bits: u32,
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
) -> u32 {
    unsafe {
        table = table.offset((bits & 0xffu32) as isize);
        if (*table).bits as u32 > 8 {
            let mut nbits = ((*table).bits as u32).wrapping_sub(8);
            BrotliDropBits(br, 8);
            table = table
                .offset(((*table).value as u32).wrapping_add(bits >> 8 & BitMask(nbits)) as isize);
        }
        BrotliDropBits(br, (*table).bits as u32);
        return (*table).value as u32;
    }
}

#[inline(always)]
extern "C" fn ReadSymbol(mut table: *const HuffmanCode, mut br: *mut BrotliBitReader) -> u32 {
    unsafe {
        return DecodeSymbol(BrotliGet16BitsUnmasked(br), table, br);
    }
}

#[inline(never)]
extern "C" fn SafeDecodeSymbol(
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
    mut result: *mut u32,
) -> i32 {
    unsafe {
        let mut val: u32 = 0;
        let mut available_bits = BrotliGetAvailableBits(br);
        if available_bits == 0 {
            if (*table).bits as i32 == 0 {
                *result = (*table).value as u32;
                return 1;
            }
            return 0;
        }
        val = BrotliGetBitsUnmasked(br) as u32;
        table = table.offset((val & 0xffu32) as isize);
        if (*table).bits as u32 <= 8 {
            if (*table).bits as u32 <= available_bits {
                BrotliDropBits(br, (*table).bits as u32);
                *result = (*table).value as u32;
                return 1;
            } else {
                return 0;
            }
        }
        if available_bits <= 8 {
            return 0;
        }
        val = (val & BitMask((*table).bits as u32)) >> 8;
        available_bits = (available_bits as u32).wrapping_sub(8) as u32;
        table = table.offset(((*table).value as u32).wrapping_add(val) as isize);
        if available_bits < (*table).bits as u32 {
            return 0;
        }
        BrotliDropBits(br, 8u32.wrapping_add((*table).bits as u32));
        *result = (*table).value as u32;
        return 1;
    }
}

#[inline(always)]
extern "C" fn SafeReadSymbol(
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
    mut result: *mut u32,
) -> i32 {
    unsafe {
        let mut val: u32 = 0;
        if (BrotliSafeGetBits(br, 15, &mut val) != 0) as i64 != 0 {
            *result = DecodeSymbol(val, table, br);
            return 1;
        }
        return SafeDecodeSymbol(table, br, result);
    }
}

#[inline(always)]
extern "C" fn PreloadSymbol(
    mut safe: i32,
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
    mut bits: *mut u32,
    mut value: *mut u32,
) {
    unsafe {
        if safe != 0 {
            return;
        }
        table = table.offset(BrotliGetBits(br, 8) as isize);
        *bits = (*table).bits as u32;
        *value = (*table).value as u32;
    }
}

#[inline(always)]
extern "C" fn ReadPreloadedSymbol(
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
    mut bits: *mut u32,
    mut value: *mut u32,
) -> u32 {
    unsafe {
        let mut result = *value;
        if (*bits > 8) as i64 != 0 {
            let mut val = BrotliGet16BitsUnmasked(br);
            let mut ext = table
                .offset((val & 0xffu32) as isize)
                .offset(*value as isize);
            let mut mask = BitMask((*bits).wrapping_sub(8));
            BrotliDropBits(br, 8);
            ext = ext.offset((val >> 8 & mask) as isize);
            BrotliDropBits(br, (*ext).bits as u32);
            result = (*ext).value as u32;
        } else {
            BrotliDropBits(br, *bits);
        }
        PreloadSymbol(0, table, br, bits, value);
        return result;
    }
}

#[inline(always)]
extern "C" fn Log2Floor(mut x: u32) -> u32 {
    let mut result = 0;
    while x != 0 {
        x >>= 1;
        result = result.wrapping_add(1);
    }
    return result;
}

extern "C" fn ReadSimpleHuffmanSymbols(
    mut alphabet_size_max: u32,
    mut alphabet_size_limit: u32,
    mut s: *mut BrotliDecoderStateInternal,
) -> i32 {
    unsafe {
        let mut br: *mut BrotliBitReader = &mut (*s).br;
        let mut h: *mut BrotliMetablockHeaderArena = &mut (*s).arena.header;
        let mut max_bits = Log2Floor(alphabet_size_max.wrapping_sub(1));
        let mut i = (*h).sub_loop_counter;
        let mut num_symbols = (*h).symbol;
        while i <= num_symbols {
            let mut v: u32 = 0;
            if (BrotliSafeReadBits(br, max_bits, &mut v) == 0) as i64 != 0 {
                (*h).sub_loop_counter = i;
                (*h).substate_huffman = BROTLI_STATE_HUFFMAN_SIMPLE_READ;
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            if v >= alphabet_size_limit {
                return BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET as i32;
            };
            (*h).symbols_lists_array[i as usize] = v as u16;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < num_symbols {
            let mut k = i.wrapping_add(1);
            while k <= num_symbols {
                if (*h).symbols_lists_array[i as usize] as i32
                    == (*h).symbols_lists_array[k as usize] as i32
                {
                    return BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME as i32;
                }
                k = k.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        return BROTLI_DECODER_SUCCESS;
    }
}

#[inline(always)]
extern "C" fn ProcessSingleCodeLength(
    mut code_len: u32,
    mut symbol: *mut u32,
    mut repeat: *mut u32,
    mut space: *mut u32,
    mut prev_code_len: *mut u32,
    mut symbol_lists: *mut u16,
    mut code_length_histo: *mut u16,
    mut next_symbol: *mut i32,
) {
    unsafe {
        *repeat = 0;
        if code_len != 0 {
            *symbol_lists.offset(*next_symbol.offset(code_len as isize) as isize) = *symbol as u16;
            *next_symbol.offset(code_len as isize) = *symbol as i32;
            *prev_code_len = code_len;
            *space = (*space as u32).wrapping_sub(32768 >> code_len) as u32;
            let ref mut fresh26 = *code_length_histo.offset(code_len as isize);
            *fresh26 = (*fresh26).wrapping_add(1);
        }
        *symbol = (*symbol).wrapping_add(1);
    }
}

#[inline(always)]
extern "C" fn ProcessRepeatedCodeLength(
    mut code_len: u32,
    mut repeat_delta: u32,
    mut alphabet_size: u32,
    mut symbol: *mut u32,
    mut repeat: *mut u32,
    mut space: *mut u32,
    mut prev_code_len: *mut u32,
    mut repeat_code_len: *mut u32,
    mut symbol_lists: *mut u16,
    mut code_length_histo: *mut u16,
    mut next_symbol: *mut i32,
) {
    unsafe {
        let mut old_repeat: u32 = 0;
        let mut extra_bits = 3;
        let mut new_len = 0;
        if code_len == 16 {
            new_len = *prev_code_len;
            extra_bits = 2;
        }
        if *repeat_code_len != new_len {
            *repeat = 0;
            *repeat_code_len = new_len;
        }
        old_repeat = *repeat;
        if *repeat > 0 {
            *repeat = (*repeat as u32).wrapping_sub(2) as u32;
            *repeat <<= extra_bits;
        }
        *repeat = (*repeat as u32).wrapping_add(repeat_delta.wrapping_add(3)) as u32;
        repeat_delta = (*repeat).wrapping_sub(old_repeat);
        if (*symbol).wrapping_add(repeat_delta) > alphabet_size {
            *symbol = alphabet_size;
            *space = 0xfffff;
            return;
        }
        if *repeat_code_len != 0 {
            let mut last = (*symbol).wrapping_add(repeat_delta);
            let mut next = *next_symbol.offset(*repeat_code_len as isize);
            loop {
                *symbol_lists.offset(next as isize) = *symbol as u16;
                next = *symbol as i32;
                *symbol = (*symbol).wrapping_add(1);
                if !(*symbol != last) {
                    break;
                }
            }
            *next_symbol.offset(*repeat_code_len as isize) = next;
            *space = (*space as u32)
                .wrapping_sub(repeat_delta << 15u32.wrapping_sub(*repeat_code_len))
                as u32;
            *code_length_histo.offset(*repeat_code_len as isize) =
                (*code_length_histo.offset(*repeat_code_len as isize) as u32)
                    .wrapping_add(repeat_delta) as u16;
        } else {
            *symbol = (*symbol as u32).wrapping_add(repeat_delta) as u32;
        };
    }
}

extern "C" fn ReadSymbolCodeLengths(
    mut alphabet_size: u32,
    mut s: *mut BrotliDecoderStateInternal,
) -> i32 {
    unsafe {
        let mut br: *mut BrotliBitReader = &mut (*s).br;
        let mut h: *mut BrotliMetablockHeaderArena = &mut (*s).arena.header;
        let mut symbol = (*h).symbol;
        let mut repeat = (*h).repeat;
        let mut space = (*h).space;
        let mut prev_code_len = (*h).prev_code_len;
        let mut repeat_code_len = (*h).repeat_code_len;
        let mut symbol_lists = (*h).symbol_lists;
        let mut code_length_histo = ((*h).code_length_histo).as_mut_ptr();
        let mut next_symbol = ((*h).next_symbol).as_mut_ptr();
        if BrotliWarmupBitReader(br) == 0 {
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        while symbol < alphabet_size && space > 0 {
            let mut p: *const HuffmanCode = ((*h).table).as_mut_ptr();
            let mut code_len: u32 = 0;
            if BrotliCheckInputAmount(br, ::std::mem::size_of::<u64>() as u64 >> 1) == 0 {
                (*h).symbol = symbol;
                (*h).repeat = repeat;
                (*h).prev_code_len = prev_code_len;
                (*h).repeat_code_len = repeat_code_len;
                (*h).space = space;
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            BrotliFillBitWindow16(br);
            p = p.offset((BrotliGetBitsUnmasked(br) & BitMask(5u32) as u64) as isize);
            BrotliDropBits(br, (*p).bits as u32);
            code_len = (*p).value as u32;
            if code_len < 16 {
                ProcessSingleCodeLength(
                    code_len,
                    &mut symbol,
                    &mut repeat,
                    &mut space,
                    &mut prev_code_len,
                    symbol_lists,
                    code_length_histo,
                    next_symbol,
                );
            } else {
                let mut extra_bits = (if code_len == 16 { 2 } else { 3 }) as u32;
                let mut repeat_delta = BrotliGetBitsUnmasked(br) as u32 & BitMask(extra_bits);
                BrotliDropBits(br, extra_bits);
                ProcessRepeatedCodeLength(
                    code_len,
                    repeat_delta,
                    alphabet_size,
                    &mut symbol,
                    &mut repeat,
                    &mut space,
                    &mut prev_code_len,
                    &mut repeat_code_len,
                    symbol_lists,
                    code_length_histo,
                    next_symbol,
                );
            }
        }
        (*h).space = space;
        return BROTLI_DECODER_SUCCESS;
    }
}

extern "C" fn SafeReadSymbolCodeLengths(
    mut alphabet_size: u32,
    mut s: *mut BrotliDecoderStateInternal,
) -> i32 {
    unsafe {
        let mut br: *mut BrotliBitReader = &mut (*s).br;
        let mut h: *mut BrotliMetablockHeaderArena = &mut (*s).arena.header;
        let mut get_byte = 0;
        while (*h).symbol < alphabet_size && (*h).space > 0 {
            let mut p: *const HuffmanCode = ((*h).table).as_mut_ptr();
            let mut code_len: u32 = 0;
            let mut available_bits: u32 = 0;
            let mut bits = 0;
            if get_byte != 0 && BrotliPullByte(br) == 0 {
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            get_byte = 0;
            available_bits = BrotliGetAvailableBits(br);
            if available_bits != 0 {
                bits = BrotliGetBitsUnmasked(br) as u32;
            }
            p = p.offset((bits & BitMask(5u32)) as isize);
            if (*p).bits as u32 > available_bits {
                get_byte = 1;
            } else {
                code_len = (*p).value as u32;
                if code_len < 16 {
                    BrotliDropBits(br, (*p).bits as u32);
                    ProcessSingleCodeLength(
                        code_len,
                        &mut (*h).symbol,
                        &mut (*h).repeat,
                        &mut (*h).space,
                        &mut (*h).prev_code_len,
                        (*h).symbol_lists,
                        ((*h).code_length_histo).as_mut_ptr(),
                        ((*h).next_symbol).as_mut_ptr(),
                    );
                } else {
                    let mut extra_bits = code_len.wrapping_sub(14);
                    let mut repeat_delta = bits >> (*p).bits as i32 & BitMask(extra_bits);
                    if available_bits < ((*p).bits as u32).wrapping_add(extra_bits) {
                        get_byte = 1;
                    } else {
                        BrotliDropBits(br, ((*p).bits as u32).wrapping_add(extra_bits));
                        ProcessRepeatedCodeLength(
                            code_len,
                            repeat_delta,
                            alphabet_size,
                            &mut (*h).symbol,
                            &mut (*h).repeat,
                            &mut (*h).space,
                            &mut (*h).prev_code_len,
                            &mut (*h).repeat_code_len,
                            (*h).symbol_lists,
                            ((*h).code_length_histo).as_mut_ptr(),
                            ((*h).next_symbol).as_mut_ptr(),
                        );
                    }
                }
            }
        }
        return BROTLI_DECODER_SUCCESS;
    }
}

extern "C" fn ReadCodeLengthCodeLengths(mut s: *mut BrotliDecoderStateInternal) -> i32 {
    unsafe {
        let mut br: *mut BrotliBitReader = &mut (*s).br;
        let mut h: *mut BrotliMetablockHeaderArena = &mut (*s).arena.header;
        let mut num_codes = (*h).repeat;
        let mut space = (*h).space;
        let mut i = (*h).sub_loop_counter;
        while i < (17 + 1i32) as u32 {
            let code_len_idx = kCodeLengthCodeOrder[i as usize];
            let mut ix: u32 = 0;
            let mut v: u32 = 0;
            if (BrotliSafeGetBits(br, 4, &mut ix) == 0) as i64 != 0 {
                let mut available_bits = BrotliGetAvailableBits(br);
                if available_bits != 0 {
                    ix = (BrotliGetBitsUnmasked(br) & 0xfu64) as u32;
                } else {
                    ix = 0;
                }
                if kCodeLengthPrefixLength[ix as usize] as u32 > available_bits {
                    (*h).sub_loop_counter = i;
                    (*h).repeat = num_codes;
                    (*h).space = space;
                    (*h).substate_huffman = BROTLI_STATE_HUFFMAN_COMPLEX;
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
            }
            v = kCodeLengthPrefixValue[ix as usize] as u32;
            BrotliDropBits(br, kCodeLengthPrefixLength[ix as usize] as u32);
            (*h).code_length_code_lengths[code_len_idx as usize] = v as u8;
            if v != 0 {
                space = space.wrapping_sub(32 >> v);
                num_codes = num_codes.wrapping_add(1);
                let ref mut fresh27 = (*h).code_length_histo[v as usize];
                *fresh27 = (*fresh27).wrapping_add(1);
                if space.wrapping_sub(1) >= 32 {
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        if !(num_codes == 1 || space == 0) {
            return BROTLI_DECODER_ERROR_FORMAT_CL_SPACE as i32;
        }
        return BROTLI_DECODER_SUCCESS;
    }
}

extern "C" fn ReadHuffmanCode(
    mut alphabet_size_max: u32,
    mut alphabet_size_limit: u32,
    mut table: *mut HuffmanCode,
    mut opt_table_size: *mut u32,
    mut s: *mut BrotliDecoderStateInternal,
) -> i32 {
    unsafe {
        let mut br: *mut BrotliBitReader = &mut (*s).br;
        let mut h: *mut BrotliMetablockHeaderArena = &mut (*s).arena.header;
        let mut current_block_67: u64;
        loop {
            match (*h).substate_huffman as u32 {
                0 => {
                    if BrotliSafeReadBits(br, 2, &mut (*h).sub_loop_counter) == 0 {
                        return BROTLI_DECODER_NEEDS_MORE_INPUT;
                    }
                    if (*h).sub_loop_counter != 1 {
                        (*h).space = 32;
                        (*h).repeat = 0;
                        memset(
                            &mut *((*h).code_length_histo).as_mut_ptr().offset(0 as isize)
                                as *mut u16 as *mut libc::c_void,
                            0,
                            (::std::mem::size_of::<u16>() as u64).wrapping_mul((5 + 1i32) as u64),
                        );
                        memset(
                            &mut *((*h).code_length_code_lengths)
                                .as_mut_ptr()
                                .offset(0 as isize) as *mut u8
                                as *mut libc::c_void,
                            0,
                            ::std::mem::size_of::<[u8; 18]>() as u64,
                        );
                        (*h).substate_huffman = BROTLI_STATE_HUFFMAN_COMPLEX;
                        continue;
                    } else {
                        current_block_67 = 18212719600168532651;
                    }
                }
                1 => {
                    current_block_67 = 18212719600168532651;
                }
                2 => {
                    current_block_67 = 5459849341776515350;
                }
                3 => {
                    current_block_67 = 16287337601645855029;
                }
                4 => {
                    let mut i: u32 = 0;
                    let mut result_0 = ReadCodeLengthCodeLengths(s);
                    if result_0 as i32 != BROTLI_DECODER_SUCCESS as i32 {
                        return result_0;
                    }
                    BrotliBuildCodeLengthsHuffmanTable(
                        ((*h).table).as_mut_ptr(),
                        ((*h).code_length_code_lengths).as_mut_ptr(),
                        ((*h).code_length_histo).as_mut_ptr(),
                    );
                    memset(
                        &mut *((*h).code_length_histo).as_mut_ptr().offset(0 as isize) as *mut u16
                            as *mut libc::c_void,
                        0,
                        ::std::mem::size_of::<[u16; 16]>() as u64,
                    );
                    i = 0;
                    while i <= 15 {
                        (*h).next_symbol[i as usize] = i as i32 - (15 + 1);
                        *((*h).symbol_lists).offset((*h).next_symbol[i as usize] as isize) = 0xffff;
                        i = i.wrapping_add(1);
                    }
                    (*h).symbol = 0;
                    (*h).prev_code_len = 8;
                    (*h).repeat = 0;
                    (*h).repeat_code_len = 0;
                    (*h).space = 32768;
                    (*h).substate_huffman = BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS;
                    current_block_67 = 13094840349067953292;
                }
                5 => {
                    current_block_67 = 13094840349067953292;
                }
                _ => {
                    return BROTLI_DECODER_ERROR_UNREACHABLE as i32;
                }
            }
            match current_block_67 {
                18212719600168532651 => {
                    if BrotliSafeReadBits(br, 2, &mut (*h).symbol) == 0 {
                        (*h).substate_huffman = BROTLI_STATE_HUFFMAN_SIMPLE_SIZE;
                        return BROTLI_DECODER_NEEDS_MORE_INPUT;
                    };
                    (*h).sub_loop_counter = 0;
                    current_block_67 = 5459849341776515350;
                }
                13094840349067953292 => {
                    let mut table_size_0: u32 = 0;
                    let mut result_1 = ReadSymbolCodeLengths(alphabet_size_limit, s);
                    if result_1 as i32 == BROTLI_DECODER_NEEDS_MORE_INPUT as i32 {
                        result_1 = SafeReadSymbolCodeLengths(alphabet_size_limit, s);
                    }
                    if result_1 as i32 != BROTLI_DECODER_SUCCESS as i32 {
                        return result_1;
                    }
                    if (*h).space != 0 {
                        return BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE as i32;
                    }
                    table_size_0 = BrotliBuildHuffmanTable(
                        table,
                        8,
                        (*h).symbol_lists,
                        ((*h).code_length_histo).as_mut_ptr(),
                    );
                    if !opt_table_size.is_null() {
                        *opt_table_size = table_size_0;
                    };
                    (*h).substate_huffman = BROTLI_STATE_HUFFMAN_NONE;
                    return BROTLI_DECODER_SUCCESS;
                }
                _ => {}
            }
            match current_block_67 {
                5459849341776515350 => {
                    let mut result =
                        ReadSimpleHuffmanSymbols(alphabet_size_max, alphabet_size_limit, s);
                    if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                        return result;
                    }
                }
                _ => {}
            }
            let mut table_size: u32 = 0;
            if (*h).symbol == 3 {
                let mut bits: u32 = 0;
                if BrotliSafeReadBits(br, 1, &mut bits) == 0 {
                    (*h).substate_huffman = BROTLI_STATE_HUFFMAN_SIMPLE_BUILD;
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                let ref mut fresh28 = (*h).symbol;
                *fresh28 = (*fresh28 as u32).wrapping_add(bits) as u32;
            }
            table_size = BrotliBuildSimpleHuffmanTable(
                table,
                8,
                ((*h).symbols_lists_array).as_mut_ptr(),
                (*h).symbol,
            );
            if !opt_table_size.is_null() {
                *opt_table_size = table_size;
            };
            (*h).substate_huffman = BROTLI_STATE_HUFFMAN_NONE;
            return BROTLI_DECODER_SUCCESS;
        }
    }
}

#[inline(always)]
extern "C" fn ReadBlockLength(mut table: *const HuffmanCode, mut br: *mut BrotliBitReader) -> u32 {
    unsafe {
        let mut code: u32 = 0;
        let mut nbits: u32 = 0;
        code = ReadSymbol(table, br);
        nbits = _kBrotliPrefixCodeRanges[code as usize].nbits as u32;
        return (_kBrotliPrefixCodeRanges[code as usize].offset as u32)
            .wrapping_add(BrotliReadBits24(br, nbits));
    }
}

#[inline(always)]
extern "C" fn SafeReadBlockLength(
    mut s: *mut BrotliDecoderStateInternal,
    mut result: *mut u32,
    mut table: *const HuffmanCode,
    mut br: *mut BrotliBitReader,
) -> i32 {
    unsafe {
        let mut index: u32 = 0;
        if (*s).substate_read_block_length as u32 == BROTLI_STATE_READ_BLOCK_LENGTH_NONE as u32 {
            if SafeReadSymbol(table, br, &mut index) == 0 {
                return 0;
            }
        } else {
            index = (*s).block_length_index;
        }
        let mut bits: u32 = 0;
        let mut nbits = _kBrotliPrefixCodeRanges[index as usize].nbits as u32;
        let mut offset = _kBrotliPrefixCodeRanges[index as usize].offset as u32;
        if BrotliSafeReadBits(br, nbits, &mut bits) == 0 {
            (*s).block_length_index = index;
            (*s).substate_read_block_length = BROTLI_STATE_READ_BLOCK_LENGTH_SUFFIX;
            return 0;
        }
        *result = offset.wrapping_add(bits);
        (*s).substate_read_block_length = BROTLI_STATE_READ_BLOCK_LENGTH_NONE;
        return 1;
    }
}

#[inline(never)]
extern "C" fn InverseMoveToFrontTransform(
    mut v: *mut u8,
    mut v_len: u32,
    mut state: *mut BrotliDecoderStateInternal,
) {
    unsafe {
        let mut i = 1;
        let mut upper_bound = (*state).mtf_upper_bound;
        let mut mtf: *mut u32 = &mut *((*state).mtf).as_mut_ptr().offset(1 as isize) as *mut u32;
        let mut mtf_u8 = mtf as *mut u8;
        let b0123: [u8; 4] = [0, 1, 2, 3];
        let mut pattern: u32 = 0;
        memcpy(
            &mut pattern as *mut u32 as *mut libc::c_void,
            &b0123 as *const [u8; 4] as *const libc::c_void,
            4,
        );
        *mtf.offset(0 as isize) = pattern;
        loop {
            pattern = (pattern).wrapping_add(0x4040404) as u32;
            *mtf.offset(i as isize) = pattern;
            i = i.wrapping_add(1);
            if !(i <= upper_bound) {
                break;
            }
        }
        upper_bound = 0;
        i = 0;
        while i < v_len {
            let mut index = *v.offset(i as isize) as i32;
            let mut value = *mtf_u8.offset(index as isize);
            upper_bound |= *v.offset(i as isize) as u32;
            *v.offset(i as isize) = value;
            *mtf_u8.offset(-1i32 as isize) = value;
            loop {
                index -= 1;
                *mtf_u8.offset((index + 1i32) as isize) = *mtf_u8.offset(index as isize);
                if !(index >= 0) {
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        (*state).mtf_upper_bound = upper_bound >> 2;
    }
}

extern "C" fn HuffmanTreeGroupDecode(
    mut group: *mut HuffmanTreeGroup,
    mut s: *mut BrotliDecoderStateInternal,
) -> i32 {
    unsafe {
        let mut h: *mut BrotliMetablockHeaderArena = &mut (*s).arena.header;
        if (*h).substate_tree_group as u32 != BROTLI_STATE_TREE_GROUP_LOOP as u32 {
            let ref mut fresh29 = (*h).next;
            *fresh29 = (*group).codes;
            (*h).htree_index = 0;
            (*h).substate_tree_group = BROTLI_STATE_TREE_GROUP_LOOP;
        }
        while (*h).htree_index < (*group).num_htrees as i32 {
            let mut table_size: u32 = 0;
            let mut result = ReadHuffmanCode(
                (*group).alphabet_size_max as u32,
                (*group).alphabet_size_limit as u32,
                (*h).next,
                &mut table_size,
                s,
            );
            if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                return result;
            }
            let ref mut fresh30 = *((*group).htrees).offset((*h).htree_index as isize);
            *fresh30 = (*h).next;
            let ref mut fresh31 = (*h).next;
            *fresh31 = (*fresh31).offset(table_size as isize);
            let ref mut fresh32 = (*h).htree_index;
            *fresh32 += 1;
        }
        (*h).substate_tree_group = BROTLI_STATE_TREE_GROUP_NONE;
        return BROTLI_DECODER_SUCCESS;
    }
}

extern "C" fn DecodeContextMap(
    mut context_map_size: u32,
    mut num_htrees: *mut u32,
    mut context_map_arg: *mut *mut u8,
    mut s: *mut BrotliDecoderStateInternal,
) -> i32 {
    unsafe {
        let mut br: *mut BrotliBitReader = &mut (*s).br;
        let mut result = BROTLI_DECODER_SUCCESS;
        let mut h: *mut BrotliMetablockHeaderArena = &mut (*s).arena.header;
        let mut current_block_72: u64;
        match (*h).substate_context_map as i32 {
            0 => {
                result = DecodeVarLenUint8(s, br, num_htrees);
                if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                    return result;
                }
                *num_htrees = (*num_htrees).wrapping_add(1);
                (*h).context_index = 0;
                *context_map_arg = ((*s).alloc_func).expect("non-null function pointer")(
                    (*s).memory_manager_opaque,
                    context_map_size as u64,
                ) as *mut u8;
                if (*context_map_arg).is_null() {
                    return BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP as i32;
                }
                if *num_htrees <= 1 {
                    memset(
                        *context_map_arg as *mut libc::c_void,
                        0,
                        context_map_size as u64,
                    );
                    return BROTLI_DECODER_SUCCESS;
                };
                (*h).substate_context_map = BROTLI_STATE_CONTEXT_MAP_READ_PREFIX;
                current_block_72 = 691213823176220084;
            }
            1 => {
                current_block_72 = 691213823176220084;
            }
            2 => {
                current_block_72 = 1177961044263080468;
            }
            3 => {
                current_block_72 = 3557707350680000928;
            }
            4 => {
                current_block_72 = 13829011953889633999;
            }
            _ => {
                return BROTLI_DECODER_ERROR_UNREACHABLE as i32;
            }
        }
        match current_block_72 {
            691213823176220084 => {
                let mut bits: u32 = 0;
                if BrotliSafeGetBits(br, 5, &mut bits) == 0 {
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                if bits & 1 != 0 {
                    (*h).max_run_length_prefix = (bits >> 1i32).wrapping_add(1);
                    BrotliDropBits(br, 5);
                } else {
                    (*h).max_run_length_prefix = 0;
                    BrotliDropBits(br, 1);
                };
                (*h).substate_context_map = BROTLI_STATE_CONTEXT_MAP_HUFFMAN;
                current_block_72 = 1177961044263080468;
            }
            _ => {}
        }
        match current_block_72 {
            1177961044263080468 => {
                let mut alphabet_size = (*num_htrees).wrapping_add((*h).max_run_length_prefix);
                result = ReadHuffmanCode(
                    alphabet_size,
                    alphabet_size,
                    ((*h).context_map_table).as_mut_ptr(),
                    0 as *mut u32,
                    s,
                );
                if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                    return result;
                };
                (*h).code = 0xffff;
                (*h).substate_context_map = BROTLI_STATE_CONTEXT_MAP_DECODE;
                current_block_72 = 3557707350680000928;
            }
            _ => {}
        }
        match current_block_72 {
            3557707350680000928 => {
                let mut context_index = (*h).context_index;
                let mut max_run_length_prefix = (*h).max_run_length_prefix;
                let mut context_map = *context_map_arg;
                let mut code = (*h).code;
                let mut skip_preamble = (code != 0xffff) as i32;
                while context_index < context_map_size || skip_preamble != 0 {
                    if skip_preamble == 0 {
                        if SafeReadSymbol(((*h).context_map_table).as_mut_ptr(), br, &mut code) == 0
                        {
                            (*h).code = 0xffff;
                            (*h).context_index = context_index;
                            return BROTLI_DECODER_NEEDS_MORE_INPUT;
                        }
                        if code == 0 {
                            let fresh33 = context_index;
                            context_index = context_index.wrapping_add(1);
                            *context_map.offset(fresh33 as isize) = 0;
                            continue;
                        } else if code > max_run_length_prefix {
                            let fresh34 = context_index;
                            context_index = context_index.wrapping_add(1);
                            *context_map.offset(fresh34 as isize) =
                                code.wrapping_sub(max_run_length_prefix) as u8;
                            continue;
                        }
                    } else {
                        skip_preamble = 0;
                    }
                    let mut reps: u32 = 0;
                    if BrotliSafeReadBits(br, code, &mut reps) == 0 {
                        (*h).code = code;
                        (*h).context_index = context_index;
                        return BROTLI_DECODER_NEEDS_MORE_INPUT;
                    }
                    reps = (reps).wrapping_add(1 << code) as u32;
                    if context_index.wrapping_add(reps) > context_map_size {
                        return BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT as i32;
                    }
                    loop {
                        let fresh35 = context_index;
                        context_index = context_index.wrapping_add(1);
                        *context_map.offset(fresh35 as isize) = 0;
                        reps = reps.wrapping_sub(1);
                        if !(reps != 0) {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
        let mut bits_0: u32 = 0;
        if BrotliSafeReadBits(br, 1, &mut bits_0) == 0 {
            (*h).substate_context_map = BROTLI_STATE_CONTEXT_MAP_TRANSFORM;
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        if bits_0 != 0 {
            InverseMoveToFrontTransform(*context_map_arg, context_map_size, s);
        };
        (*h).substate_context_map = BROTLI_STATE_CONTEXT_MAP_NONE;
        return BROTLI_DECODER_SUCCESS;
    }
}

#[inline(always)]
extern "C" fn DecodeBlockTypeAndLength(
    mut safe: i32,
    mut s: *mut BrotliDecoderStateInternal,
    mut tree_type: i32,
) -> i32 {
    unsafe {
        let mut max_block_type = (*s).num_block_types[tree_type as usize];
        let mut type_tree: *const HuffmanCode =
            &mut *((*s).block_type_trees).offset((tree_type * 632i32) as isize) as *mut HuffmanCode;
        let mut len_tree: *const HuffmanCode =
            &mut *((*s).block_len_trees).offset((tree_type * 396i32) as isize) as *mut HuffmanCode;
        let mut br: *mut BrotliBitReader = &mut (*s).br;
        let mut ringbuffer: *mut u32 = &mut *((*s).block_type_rb)
            .as_mut_ptr()
            .offset((tree_type * 2i32) as isize) as *mut u32;
        let mut block_type: u32 = 0;
        if max_block_type <= 1 {
            return 0;
        }
        if safe == 0 {
            block_type = ReadSymbol(type_tree, br);
            (*s).block_length[tree_type as usize] = ReadBlockLength(len_tree, br);
        } else {
            let mut memento = BrotliBitReaderState {
                val_: 0,
                bit_pos_: 0,
                next_in: 0 as *const u8,
                avail_in: 0,
            };
            BrotliBitReaderSaveState(br, &mut memento);
            if SafeReadSymbol(type_tree, br, &mut block_type) == 0 {
                return 0;
            }
            if SafeReadBlockLength(
                s,
                &mut *((*s).block_length).as_mut_ptr().offset(tree_type as isize),
                len_tree,
                br,
            ) == 0
            {
                (*s).substate_read_block_length = BROTLI_STATE_READ_BLOCK_LENGTH_NONE;
                BrotliBitReaderRestoreState(br, &mut memento);
                return 0;
            }
        }
        if block_type == 1 {
            block_type = (*ringbuffer.offset(1 as isize)).wrapping_add(1);
        } else if block_type == 0 {
            block_type = *ringbuffer.offset(0 as isize);
        } else {
            block_type = (block_type).wrapping_sub(2) as u32;
        }
        if block_type >= max_block_type {
            block_type = (block_type).wrapping_sub(max_block_type) as u32;
        };
        *ringbuffer.offset(0 as isize) = *ringbuffer.offset(1 as isize);
        *ringbuffer.offset(1 as isize) = block_type;
        return 1;
    }
}

#[inline(always)]
extern "C" fn DetectTrivialLiteralBlockTypes(mut s: *mut BrotliDecoderStateInternal) {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i < 8 {
            (*s).trivial_literal_contexts[i as usize] = 0;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i < (*s).num_block_types[0 as usize] as u64 {
            let mut offset = i << 6;
            let mut error = 0;
            let mut sample = *((*s).context_map).offset(offset as isize) as u64;
            let mut j: u64 = 0;
            j = 0;
            while j < (1u32 << 6) as u64 {
                if 4 & 4 != 0 {
                    let fresh39 = j;
                    j = j.wrapping_add(1);
                    error |= *((*s).context_map).offset(offset.wrapping_add(fresh39) as isize)
                        as u64
                        ^ sample;
                    let fresh40 = j;
                    j = j.wrapping_add(1);
                    error |= *((*s).context_map).offset(offset.wrapping_add(fresh40) as isize)
                        as u64
                        ^ sample;
                    let fresh41 = j;
                    j = j.wrapping_add(1);
                    error |= *((*s).context_map).offset(offset.wrapping_add(fresh41) as isize)
                        as u64
                        ^ sample;
                    let fresh42 = j;
                    j = j.wrapping_add(1);
                    error |= *((*s).context_map).offset(offset.wrapping_add(fresh42) as isize)
                        as u64
                        ^ sample;
                }
            }
            if error == 0 {
                let ref mut fresh43 = (*s).trivial_literal_contexts[(i >> 5i32) as usize];
                *fresh43 |= 1 << (i & 31);
            }
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn PrepareLiteralDecoding(mut s: *mut BrotliDecoderStateInternal) {
    unsafe {
        let mut context_mode: u8 = 0;
        let mut trivial: u64 = 0;
        let mut block_type = (*s).block_type_rb[1 as usize];
        let mut context_offset = block_type << 6;
        let ref mut fresh44 = (*s).context_map_slice;
        *fresh44 = ((*s).context_map).offset(context_offset as isize);
        trivial = (*s).trivial_literal_contexts[(block_type >> 5i32) as usize] as u64;
        (*s).trivial_literal_context = (trivial >> (block_type & 31u32) & 1) as i32;
        let ref mut fresh45 = (*s).literal_htree;
        *fresh45 = *((*s).literal_hgroup.htrees)
            .offset(*((*s).context_map_slice).offset(0 as isize) as isize);
        context_mode = (*((*s).context_modes).offset(block_type as isize) as i32 & 3i32) as u8;
        let ref mut fresh46 = (*s).context_lookup;
        *fresh46 = &*_kBrotliContextLookupTable
            .as_ptr()
            .offset(((context_mode as i32) << 9i32) as isize) as *const u8;
    }
}

#[inline(always)]
extern "C" fn DecodeLiteralBlockSwitchInternal(
    mut safe: i32,
    mut s: *mut BrotliDecoderStateInternal,
) -> i32 {
    unsafe {
        if DecodeBlockTypeAndLength(safe, s, 0) == 0 {
            return 0;
        }
        PrepareLiteralDecoding(s);
        return 1;
    }
}

#[inline(never)]
extern "C" fn DecodeLiteralBlockSwitch(mut s: *mut BrotliDecoderStateInternal) {
    unsafe {
        DecodeLiteralBlockSwitchInternal(0, s);
    }
}

#[inline(never)]
extern "C" fn SafeDecodeLiteralBlockSwitch(mut s: *mut BrotliDecoderStateInternal) -> i32 {
    unsafe {
        return DecodeLiteralBlockSwitchInternal(1, s);
    }
}

#[inline(always)]
extern "C" fn DecodeCommandBlockSwitchInternal(
    mut safe: i32,
    mut s: *mut BrotliDecoderStateInternal,
) -> i32 {
    unsafe {
        if DecodeBlockTypeAndLength(safe, s, 1) == 0 {
            return 0;
        }
        let ref mut fresh47 = (*s).htree_command;
        *fresh47 =
            *((*s).insert_copy_hgroup.htrees).offset((*s).block_type_rb[3 as usize] as isize);
        return 1;
    }
}

#[inline(never)]
extern "C" fn DecodeCommandBlockSwitch(mut s: *mut BrotliDecoderStateInternal) {
    unsafe {
        DecodeCommandBlockSwitchInternal(0, s);
    }
}

#[inline(never)]
extern "C" fn SafeDecodeCommandBlockSwitch(mut s: *mut BrotliDecoderStateInternal) -> i32 {
    unsafe {
        return DecodeCommandBlockSwitchInternal(1, s);
    }
}

#[inline(always)]
extern "C" fn DecodeDistanceBlockSwitchInternal(
    mut safe: i32,
    mut s: *mut BrotliDecoderStateInternal,
) -> i32 {
    unsafe {
        if DecodeBlockTypeAndLength(safe, s, 2) == 0 {
            return 0;
        }
        let ref mut fresh48 = (*s).dist_context_map_slice;
        *fresh48 =
            ((*s).dist_context_map).offset(((*s).block_type_rb[5 as usize] << 2i32) as isize);
        (*s).dist_htree_index =
            *((*s).dist_context_map_slice).offset((*s).distance_context as isize);
        return 1;
    }
}

#[inline(never)]
extern "C" fn DecodeDistanceBlockSwitch(mut s: *mut BrotliDecoderStateInternal) {
    unsafe {
        DecodeDistanceBlockSwitchInternal(0, s);
    }
}

#[inline(never)]
extern "C" fn SafeDecodeDistanceBlockSwitch(mut s: *mut BrotliDecoderStateInternal) -> i32 {
    unsafe {
        return DecodeDistanceBlockSwitchInternal(1, s);
    }
}

extern "C" fn UnwrittenBytes(mut s: *const BrotliDecoderStateInternal, mut wrap: i32) -> u64 {
    unsafe {
        let mut pos = if wrap != 0 && (*s).pos > (*s).ringbuffer_size {
            (*s).ringbuffer_size as u64
        } else {
            (*s).pos as u64
        };
        let mut partial_pos_rb = ((*s).rb_roundtrips)
            .wrapping_mul((*s).ringbuffer_size as u64)
            .wrapping_add(pos);
        return partial_pos_rb.wrapping_sub((*s).partial_pos_out);
    }
}

#[inline(never)]
extern "C" fn WriteRingBuffer(
    mut s: *mut BrotliDecoderStateInternal,
    mut available_out: *mut u64,
    mut next_out: *mut *mut u8,
    mut total_out: *mut u64,
    mut force: i32,
) -> i32 {
    unsafe {
        let mut start =
            ((*s).ringbuffer).offset(((*s).partial_pos_out & (*s).ringbuffer_mask as u64) as isize);
        let mut to_write = UnwrittenBytes(s, 1);
        let mut num_written = *available_out;
        if num_written > to_write {
            num_written = to_write;
        }
        if (*s).meta_block_remaining_len < 0 {
            return BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1 as i32;
        }
        if !next_out.is_null() && (*next_out).is_null() {
            *next_out = start;
        } else if !next_out.is_null() {
            memcpy(
                *next_out as *mut libc::c_void,
                start as *const libc::c_void,
                num_written,
            );
            *next_out = (*next_out).offset(num_written as isize);
        }
        *available_out = (*available_out as u64).wrapping_sub(num_written) as u64;
        let ref mut fresh49 = (*s).partial_pos_out;
        *fresh49 = (*fresh49 as u64).wrapping_add(num_written) as u64;
        if !total_out.is_null() {
            *total_out = (*s).partial_pos_out;
        }
        if num_written < to_write {
            if (*s).ringbuffer_size == 1 << (*s).window_bits || force != 0 {
                return BROTLI_DECODER_NEEDS_MORE_OUTPUT;
            } else {
                return BROTLI_DECODER_SUCCESS;
            }
        }
        if (*s).ringbuffer_size == 1 << (*s).window_bits && (*s).pos >= (*s).ringbuffer_size {
            (*s).pos -= (*s).ringbuffer_size;
            let ref mut fresh50 = (*s).rb_roundtrips;
            *fresh50 = (*fresh50).wrapping_add(1);
            (*s).set_should_wrap_ringbuffer((if (*s).pos as u64 != 0 { 1 } else { 0 }) as u32);
        }
        return BROTLI_DECODER_SUCCESS;
    }
}

#[inline(never)]
extern "C" fn WrapRingBuffer(mut s: *mut BrotliDecoderStateInternal) {
    unsafe {
        if (*s).should_wrap_ringbuffer() != 0 {
            memcpy(
                (*s).ringbuffer as *mut libc::c_void,
                (*s).ringbuffer_end as *const libc::c_void,
                (*s).pos as u64,
            );
            (*s).set_should_wrap_ringbuffer(0);
        }
    }
}

#[inline(never)]
extern "C" fn BrotliEnsureRingBuffer(mut s: *mut BrotliDecoderStateInternal) -> i32 {
    unsafe {
        let mut old_ringbuffer = (*s).ringbuffer;
        if (*s).ringbuffer_size == (*s).new_ringbuffer_size {
            return 1;
        }
        let ref mut fresh51 = (*s).ringbuffer;
        *fresh51 = ((*s).alloc_func).expect("non-null function pointer")(
            (*s).memory_manager_opaque,
            ((*s).new_ringbuffer_size as u64).wrapping_add(kRingBufferWriteAheadSlack as u64),
        ) as *mut u8;
        if ((*s).ringbuffer).is_null() {
            let ref mut fresh52 = (*s).ringbuffer;
            *fresh52 = old_ringbuffer;
            return 0;
        };
        *((*s).ringbuffer).offset(((*s).new_ringbuffer_size - 2i32) as isize) = 0;
        *((*s).ringbuffer).offset(((*s).new_ringbuffer_size - 1i32) as isize) = 0;
        if !old_ringbuffer.is_null() {
            memcpy(
                (*s).ringbuffer as *mut libc::c_void,
                old_ringbuffer as *const libc::c_void,
                (*s).pos as u64,
            );
            ((*s).free_func).expect("non-null function pointer")(
                (*s).memory_manager_opaque,
                old_ringbuffer as *mut libc::c_void,
            );
            old_ringbuffer = 0 as *mut u8;
        };
        (*s).ringbuffer_size = (*s).new_ringbuffer_size;
        (*s).ringbuffer_mask = (*s).new_ringbuffer_size - 1;
        let ref mut fresh53 = (*s).ringbuffer_end;
        *fresh53 = ((*s).ringbuffer).offset((*s).ringbuffer_size as isize);
        return 1;
    }
}

#[inline(never)]
extern "C" fn CopyUncompressedBlockToOutput(
    mut available_out: *mut u64,
    mut next_out: *mut *mut u8,
    mut total_out: *mut u64,
    mut s: *mut BrotliDecoderStateInternal,
) -> i32 {
    unsafe {
        if BrotliEnsureRingBuffer(s) == 0 {
            return BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1 as i32;
        }
        loop {
            let mut current_block_27: u64;
            match (*s).substate_uncompressed as u32 {
                0 => {
                    let mut nbytes = BrotliGetRemainingBytes(&mut (*s).br) as i32;
                    if nbytes > (*s).meta_block_remaining_len {
                        nbytes = (*s).meta_block_remaining_len;
                    }
                    if (*s).pos + nbytes > (*s).ringbuffer_size {
                        nbytes = (*s).ringbuffer_size - (*s).pos;
                    }
                    BrotliCopyBytes(
                        &mut *((*s).ringbuffer).offset((*s).pos as isize),
                        &mut (*s).br,
                        nbytes as u64,
                    );
                    (*s).pos += nbytes;
                    (*s).meta_block_remaining_len -= nbytes;
                    if (*s).pos < 1 << (*s).window_bits {
                        if (*s).meta_block_remaining_len == 0 {
                            return BROTLI_DECODER_SUCCESS;
                        }
                        return BROTLI_DECODER_NEEDS_MORE_INPUT;
                    };
                    (*s).substate_uncompressed = BROTLI_STATE_UNCOMPRESSED_WRITE;
                    current_block_27 = 26972500619410423;
                }
                1 => {
                    current_block_27 = 26972500619410423;
                }
                _ => {
                    current_block_27 = 16924917904204750491;
                }
            }
            match current_block_27 {
                26972500619410423 => {
                    let mut result = NULL;
                    result = WriteRingBuffer(s, available_out, next_out, total_out, 0);
                    if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                        return result;
                    }
                    if (*s).ringbuffer_size == 1 << (*s).window_bits {
                        (*s).max_distance = (*s).max_backward_distance;
                    };
                    (*s).substate_uncompressed = BROTLI_STATE_UNCOMPRESSED_NONE;
                }
                _ => {}
            }
        }
    }
}

#[inline(never)]
extern "C" fn BrotliCalculateRingBufferSize(mut s: *mut BrotliDecoderStateInternal) {
    unsafe {
        let mut window_size = 1 << (*s).window_bits;
        let mut new_ringbuffer_size = window_size;
        let mut min_size = if (*s).ringbuffer_size != 0 {
            (*s).ringbuffer_size
        } else {
            1024
        };
        let mut output_size: i32 = 0;
        if (*s).ringbuffer_size == window_size {
            return;
        }
        if (*s).is_metadata() != 0 {
            return;
        }
        if ((*s).ringbuffer).is_null() {
            output_size = 0;
        } else {
            output_size = (*s).pos;
        }
        output_size += (*s).meta_block_remaining_len;
        min_size = if min_size < output_size {
            output_size
        } else {
            min_size
        };
        if (*s).canny_ringbuffer_allocation() != 0 {
            while new_ringbuffer_size >> 1 >= min_size {
                new_ringbuffer_size >>= 1;
            }
        };
        (*s).new_ringbuffer_size = new_ringbuffer_size;
    }
}

extern "C" fn ReadContextModes(mut s: *mut BrotliDecoderStateInternal) -> i32 {
    unsafe {
        let mut br: *mut BrotliBitReader = &mut (*s).br;
        let mut i = (*s).loop_counter;
        while i < (*s).num_block_types[0 as usize] as i32 {
            let mut bits: u32 = 0;
            if BrotliSafeReadBits(br, 2, &mut bits) == 0 {
                (*s).loop_counter = i;
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            };
            *((*s).context_modes).offset(i as isize) = bits as u8;
            i += 1;
        }
        return BROTLI_DECODER_SUCCESS;
    }
}

#[inline(always)]
extern "C" fn TakeDistanceFromRingBuffer(mut s: *mut BrotliDecoderStateInternal) {
    unsafe {
        let mut offset = (*s).distance_code - 3;
        if (*s).distance_code <= 3 {
            (*s).distance_context = 1 >> (*s).distance_code;
            (*s).distance_code = (*s).dist_rb[((*s).dist_rb_idx - offset & 3i32) as usize];
            (*s).dist_rb_idx -= (*s).distance_context;
        } else {
            let mut index_delta = 3;
            let mut delta: i32 = 0;
            let mut base = (*s).distance_code - 10;
            if (*s).distance_code < 10 {
                base = (*s).distance_code - 4;
            } else {
                index_delta = 2;
            }
            delta = (0x605142 >> 4 * base & 0xf) - 3;
            (*s).distance_code =
                (*s).dist_rb[((*s).dist_rb_idx + index_delta & 0x3i32) as usize] + delta;
            if (*s).distance_code <= 0 {
                (*s).distance_code = 0x7fffffff;
            }
        };
    }
}

#[inline(always)]
extern "C" fn SafeReadBits(br: *mut BrotliBitReader, mut n_bits: u32, mut val: *mut u32) -> i32 {
    unsafe {
        if n_bits != 0 {
            return BrotliSafeReadBits(br, n_bits, val);
        } else {
            *val = 0;
            return 1;
        };
    }
}

#[inline(always)]
extern "C" fn SafeReadBits32(br: *mut BrotliBitReader, mut n_bits: u32, mut val: *mut u32) -> i32 {
    unsafe {
        if n_bits != 0 {
            return BrotliSafeReadBits32(br, n_bits, val);
        } else {
            *val = 0;
            return 1;
        };
    }
}

extern "C" fn CalculateDistanceLut(mut s: *mut BrotliDecoderStateInternal) {
    unsafe {
        let mut b: *mut BrotliMetablockBodyArena = &mut (*s).arena.body;
        let mut npostfix = (*s).distance_postfix_bits;
        let mut ndirect = (*s).num_direct_distance_codes;
        let mut alphabet_size_limit = (*s).distance_hgroup.alphabet_size_limit as u32;
        let mut postfix = 1 << npostfix;
        let mut j: u32 = 0;
        let mut bits = 1;
        let mut half = 0;
        let mut i = 16;
        j = 0;
        while j < ndirect {
            (*b).dist_extra_bits[i as usize] = 0;
            (*b).dist_offset[i as usize] = j.wrapping_add(1);
            i = i.wrapping_add(1);
            j = j.wrapping_add(1);
        }
        while i < alphabet_size_limit {
            let mut base = ndirect
                .wrapping_add((2u32.wrapping_add(half) << bits).wrapping_sub(4) << npostfix)
                .wrapping_add(1);
            j = 0;
            while j < postfix {
                (*b).dist_extra_bits[i as usize] = bits as u8;
                (*b).dist_offset[i as usize] = base.wrapping_add(j);
                i = i.wrapping_add(1);
                j = j.wrapping_add(1);
            }
            bits = bits.wrapping_add(half);
            half = half ^ 1;
        }
    }
}

#[inline(always)]
extern "C" fn ReadDistanceInternal(
    mut safe: i32,
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
) -> i32 {
    unsafe {
        let mut b: *mut BrotliMetablockBodyArena = &mut (*s).arena.body;
        let mut code: u32 = 0;
        let mut bits: u32 = 0;
        let mut memento = BrotliBitReaderState {
            val_: 0,
            bit_pos_: 0,
            next_in: 0 as *const u8,
            avail_in: 0,
        };
        let mut distance_tree =
            *((*s).distance_hgroup.htrees).offset((*s).dist_htree_index as isize);
        if safe == 0 {
            code = ReadSymbol(distance_tree, br);
        } else {
            BrotliBitReaderSaveState(br, &mut memento);
            if SafeReadSymbol(distance_tree, br, &mut code) == 0 {
                return 0;
            }
        }
        let ref mut fresh54 = (*s).block_length[2 as usize];
        *fresh54 = (*fresh54).wrapping_sub(1);
        (*s).distance_context = 0;
        if code & !0xf == 0 {
            (*s).distance_code = code as i32;
            TakeDistanceFromRingBuffer(s);
            return 1;
        }
        if safe == 0 {
            bits = BrotliReadBits32(br, (*b).dist_extra_bits[code as usize] as u32);
        } else if SafeReadBits32(br, (*b).dist_extra_bits[code as usize] as u32, &mut bits) == 0 {
            let ref mut fresh55 = (*s).block_length[2 as usize];
            *fresh55 = (*fresh55).wrapping_add(1);
            BrotliBitReaderRestoreState(br, &mut memento);
            return 0;
        };
        (*s).distance_code = ((*b).dist_offset[code as usize])
            .wrapping_add(bits << (*s).distance_postfix_bits) as i32;
        return 1;
    }
}

#[inline(always)]
extern "C" fn ReadDistance(mut s: *mut BrotliDecoderStateInternal, mut br: *mut BrotliBitReader) {
    unsafe {
        ReadDistanceInternal(0, s, br);
    }
}

#[inline(always)]
extern "C" fn SafeReadDistance(
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
) -> i32 {
    unsafe {
        return ReadDistanceInternal(1, s, br);
    }
}

#[inline(always)]
extern "C" fn ReadCommandInternal(
    mut safe: i32,
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
    mut insert_length: *mut i32,
) -> i32 {
    unsafe {
        let mut cmd_code: u32 = 0;
        let mut insert_len_extra = 0;
        let mut copy_length: u32 = 0;
        let mut v = CmdLutElement {
            insert_len_extra_bits: 0,
            copy_len_extra_bits: 0,
            distance_code: 0,
            context: 0,
            insert_len_offset: 0,
            copy_len_offset: 0,
        };
        let mut memento = BrotliBitReaderState {
            val_: 0,
            bit_pos_: 0,
            next_in: 0 as *const u8,
            avail_in: 0,
        };
        if safe == 0 {
            cmd_code = ReadSymbol((*s).htree_command, br);
        } else {
            BrotliBitReaderSaveState(br, &mut memento);
            if SafeReadSymbol((*s).htree_command, br, &mut cmd_code) == 0 {
                return 0;
            }
        }
        v = kCmdLut[cmd_code as usize];
        (*s).distance_code = v.distance_code as i32;
        (*s).distance_context = v.context as i32;
        (*s).dist_htree_index =
            *((*s).dist_context_map_slice).offset((*s).distance_context as isize);
        *insert_length = v.insert_len_offset as i32;
        if safe == 0 {
            if (v.insert_len_extra_bits as i32 != 0) as i64 != 0 {
                insert_len_extra = BrotliReadBits24(br, v.insert_len_extra_bits as u32);
            }
            copy_length = BrotliReadBits24(br, v.copy_len_extra_bits as u32);
        } else if SafeReadBits(br, v.insert_len_extra_bits as u32, &mut insert_len_extra) == 0
            || SafeReadBits(br, v.copy_len_extra_bits as u32, &mut copy_length) == 0
        {
            BrotliBitReaderRestoreState(br, &mut memento);
            return 0;
        };
        (*s).copy_length = copy_length as i32 + v.copy_len_offset as i32;
        let ref mut fresh56 = (*s).block_length[1 as usize];
        *fresh56 = (*fresh56).wrapping_sub(1);
        *insert_length += insert_len_extra as i32;
        return 1;
    }
}

#[inline(always)]
extern "C" fn ReadCommand(
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
    mut insert_length: *mut i32,
) {
    unsafe {
        ReadCommandInternal(0, s, br, insert_length);
    }
}

#[inline(always)]
extern "C" fn SafeReadCommand(
    mut s: *mut BrotliDecoderStateInternal,
    mut br: *mut BrotliBitReader,
    mut insert_length: *mut i32,
) -> i32 {
    unsafe {
        return ReadCommandInternal(1, s, br, insert_length);
    }
}

#[inline(always)]
extern "C" fn CheckInputAmount(mut safe: i32, br: *mut BrotliBitReader, mut num: u64) -> i32 {
    unsafe {
        if safe != 0 {
            return 1;
        }
        return BrotliCheckInputAmount(br, num);
    }
}

#[inline(always)]
extern "C" fn ProcessCommandsInternal(
    mut safe: i32,
    mut s: *mut BrotliDecoderStateInternal,
) -> i32 {
    unsafe {
        let mut current_block: u64;
        let mut pos = (*s).pos;
        let mut i = (*s).loop_counter;
        let mut result = BROTLI_DECODER_SUCCESS;
        let mut br: *mut BrotliBitReader = &mut (*s).br;
        if CheckInputAmount(safe, br, 28) == 0 {
            result = BROTLI_DECODER_NEEDS_MORE_INPUT;
        } else {
            if safe == 0 {
                BrotliWarmupBitReader(br);
            }
            if (*s).state as u32 == BROTLI_STATE_COMMAND_BEGIN as u32 {
                current_block = 2255223553483703607;
            } else if (*s).state as u32 == BROTLI_STATE_COMMAND_INNER as u32 {
                current_block = 10079663316037948169;
            } else if (*s).state as u32 == BROTLI_STATE_COMMAND_POST_DECODE_LITERALS as u32 {
                current_block = 9270490131672724345;
            } else if (*s).state as u32 == BROTLI_STATE_COMMAND_POST_WRAP_COPY as u32 {
                current_block = 3569141194949357899;
            } else {
                return BROTLI_DECODER_ERROR_UNREACHABLE as i32;
            }
            'c_1900: loop {
                match current_block {
                    9270490131672724345 => {
                        if safe != 0 {
                            (*s).state = BROTLI_STATE_COMMAND_POST_DECODE_LITERALS;
                        }
                        if (*s).distance_code >= 0 {
                            (*s).distance_context = if (*s).distance_code != 0 { 0 } else { 1 };
                            let ref mut fresh59 = (*s).dist_rb_idx;
                            *fresh59 -= 1;
                            (*s).distance_code = (*s).dist_rb[((*s).dist_rb_idx & 3i32) as usize];
                        } else {
                            if ((*s).block_length[2 as usize] == 0) as i64 != 0 {
                                if safe != 0 {
                                    if SafeDecodeDistanceBlockSwitch(s) == 0 {
                                        result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                        break;
                                    }
                                } else {
                                    DecodeDistanceBlockSwitch(s);
                                }
                            }
                            if safe != 0 {
                                if SafeReadDistance(s, br) == 0 {
                                    result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                    break;
                                }
                            } else {
                                ReadDistance(s, br);
                            }
                        }
                        if (*s).max_distance != (*s).max_backward_distance {
                            (*s).max_distance = if pos < (*s).max_backward_distance {
                                pos
                            } else {
                                (*s).max_backward_distance
                            };
                        }
                        i = (*s).copy_length;
                        if (*s).distance_code > (*s).max_distance {
                            if (*s).distance_code > 0x7ffffffc {
                                return BROTLI_DECODER_ERROR_FORMAT_DISTANCE as i32;
                            }
                            if i >= 4 && i <= 24 {
                                let mut address = (*s).distance_code - (*s).max_distance - 1;
                                let mut words = (*s).dictionary;
                                let mut transforms = (*s).transforms;
                                let mut offset =
                                    (*(*s).dictionary).offsets_by_length[i as usize] as i32;
                                let mut shift =
                                    (*(*s).dictionary).size_bits_by_length[i as usize] as u32;
                                let mut mask = BitMask(shift) as i32;
                                let mut word_idx = address & mask;
                                let mut transform_idx = address >> shift;
                                (*s).dist_rb_idx += (*s).distance_context;
                                offset += word_idx * i;
                                if ((*words).data).is_null() as i64 != 0 {
                                    return BROTLI_DECODER_ERROR_DICTIONARY_NOT_SET as i32;
                                }
                                if transform_idx < (*transforms).num_transforms as i32 {
                                    let mut word: *const u8 =
                                        &*((*words).data).offset(offset as isize) as *const u8;
                                    let mut len = i;
                                    if transform_idx
                                        == (*transforms).cutOffTransforms[0 as usize] as i32
                                    {
                                        memcpy(
                                            &mut *((*s).ringbuffer).offset(pos as isize) as *mut u8
                                                as *mut libc::c_void,
                                            word as *const libc::c_void,
                                            len as u64,
                                        );
                                    } else {
                                        len = BrotliTransformDictionaryWord(
                                            &mut *((*s).ringbuffer).offset(pos as isize),
                                            word,
                                            len,
                                            transforms,
                                            transform_idx,
                                        );
                                    }
                                    pos += len;
                                    (*s).meta_block_remaining_len -= len;
                                    if pos >= (*s).ringbuffer_size {
                                        (*s).state = BROTLI_STATE_COMMAND_POST_WRITE_1;
                                        break;
                                    }
                                } else {
                                    return BROTLI_DECODER_ERROR_FORMAT_TRANSFORM as i32;
                                }
                            } else {
                                return BROTLI_DECODER_ERROR_FORMAT_DICTIONARY as i32;
                            }
                        } else {
                            let mut src_start = pos - (*s).distance_code & (*s).ringbuffer_mask;
                            let mut copy_dst: *mut u8 =
                                &mut *((*s).ringbuffer).offset(pos as isize) as *mut u8;
                            let mut copy_src: *mut u8 =
                                &mut *((*s).ringbuffer).offset(src_start as isize) as *mut u8;
                            let mut dst_end = pos + i;
                            let mut src_end = src_start + i;
                            (*s).dist_rb[((*s).dist_rb_idx & 3i32) as usize] = (*s).distance_code;
                            let ref mut fresh60 = (*s).dist_rb_idx;
                            *fresh60 += 1;
                            (*s).meta_block_remaining_len -= i;
                            memmove16(copy_dst, copy_src);
                            if src_end > pos && dst_end > src_start {
                                current_block = 3569141194949357899;
                                continue;
                            } else {
                                if dst_end >= (*s).ringbuffer_size
                                    || src_end >= (*s).ringbuffer_size
                                {
                                    current_block = 3569141194949357899;
                                    continue;
                                }
                                pos += i;
                                if i > 16 {
                                    if i > 32 {
                                        memcpy(
                                            copy_dst.offset(16 as isize) as *mut libc::c_void,
                                            copy_src.offset(16 as isize) as *const libc::c_void,
                                            (i - 16i32) as u64,
                                        );
                                    } else {
                                        memmove16(
                                            copy_dst.offset(16 as isize),
                                            copy_src.offset(16 as isize),
                                        );
                                    }
                                }
                            }
                        }
                        if !((*s).meta_block_remaining_len <= 0) {
                            current_block = 2255223553483703607;
                            continue;
                        };
                        (*s).state = BROTLI_STATE_METABLOCK_DONE;
                        break;
                    }
                    2255223553483703607 => {
                        if safe != 0 {
                            (*s).state = BROTLI_STATE_COMMAND_BEGIN;
                        }
                        if CheckInputAmount(safe, br, 28) == 0 {
                            (*s).state = BROTLI_STATE_COMMAND_BEGIN;
                            result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                            break;
                        } else if ((*s).block_length[1 as usize] == 0) as i64 != 0 {
                            if safe != 0 {
                                if !(SafeDecodeCommandBlockSwitch(s) == 0) {
                                    current_block = 2255223553483703607;
                                    continue;
                                }
                                result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                break;
                            } else {
                                DecodeCommandBlockSwitch(s);
                                current_block = 2255223553483703607;
                            }
                        } else {
                            if safe != 0 {
                                if SafeReadCommand(s, br, &mut i) == 0 {
                                    result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                    break;
                                }
                            } else {
                                ReadCommand(s, br, &mut i);
                            }
                            if i == 0 {
                                current_block = 9270490131672724345;
                                continue;
                            };
                            (*s).meta_block_remaining_len -= i;
                            current_block = 10079663316037948169;
                        }
                    }
                    3569141194949357899 => {
                        let mut wrap_guard = (*s).ringbuffer_size - pos;
                        loop {
                            i -= 1;
                            if !(i >= 0) {
                                break;
                            };
                            *((*s).ringbuffer).offset(pos as isize) = *((*s).ringbuffer)
                                .offset((pos - (*s).distance_code & (*s).ringbuffer_mask) as isize);
                            pos += 1;
                            wrap_guard -= 1;
                            if !((wrap_guard == 0) as i64 != 0) {
                                continue;
                            };
                            (*s).state = BROTLI_STATE_COMMAND_POST_WRITE_2;
                            break 'c_1900;
                        }
                        if !((*s).meta_block_remaining_len <= 0) {
                            current_block = 2255223553483703607;
                            continue;
                        };
                        (*s).state = BROTLI_STATE_METABLOCK_DONE;
                        break;
                    }
                    _ => {
                        if safe != 0 {
                            (*s).state = BROTLI_STATE_COMMAND_INNER;
                        }
                        if (*s).trivial_literal_context != 0 {
                            let mut bits: u32 = 0;
                            let mut value: u32 = 0;
                            PreloadSymbol(safe, (*s).literal_htree, br, &mut bits, &mut value);
                            loop {
                                if CheckInputAmount(safe, br, 28) == 0 {
                                    (*s).state = BROTLI_STATE_COMMAND_INNER;
                                    result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                    break 'c_1900;
                                } else {
                                    if ((*s).block_length[0 as usize] == 0) as i64 != 0 {
                                        if safe != 0 {
                                            if SafeDecodeLiteralBlockSwitch(s) == 0 {
                                                result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                                break 'c_1900;
                                            }
                                        } else {
                                            DecodeLiteralBlockSwitch(s);
                                        }
                                        PreloadSymbol(
                                            safe,
                                            (*s).literal_htree,
                                            br,
                                            &mut bits,
                                            &mut value,
                                        );
                                        if (*s).trivial_literal_context == 0 {
                                            current_block = 10079663316037948169;
                                            continue 'c_1900;
                                        }
                                    }
                                    if safe == 0 {
                                        *((*s).ringbuffer).offset(pos as isize) =
                                            ReadPreloadedSymbol(
                                                (*s).literal_htree,
                                                br,
                                                &mut bits,
                                                &mut value,
                                            ) as u8;
                                    } else {
                                        let mut literal: u32 = 0;
                                        if SafeReadSymbol((*s).literal_htree, br, &mut literal) == 0
                                        {
                                            result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                            break 'c_1900;
                                        } else {
                                            *((*s).ringbuffer).offset(pos as isize) = literal as u8;
                                        }
                                    }
                                    let ref mut fresh57 = (*s).block_length[0 as usize];
                                    *fresh57 = (*fresh57).wrapping_sub(1);
                                    pos += 1;
                                    if (pos == (*s).ringbuffer_size) as i64 != 0 {
                                        (*s).state = BROTLI_STATE_COMMAND_INNER_WRITE;
                                        i -= 1;
                                        break 'c_1900;
                                    } else {
                                        i -= 1;
                                        if !(i != 0) {
                                            break;
                                        }
                                    }
                                }
                            }
                        } else {
                            let mut p1 = *((*s).ringbuffer)
                                .offset((pos - 1 & (*s).ringbuffer_mask) as isize);
                            let mut p2 = *((*s).ringbuffer)
                                .offset((pos - 2 & (*s).ringbuffer_mask) as isize);
                            loop {
                                let mut hc = 0 as *const HuffmanCode;
                                let mut context: u8 = 0;
                                if CheckInputAmount(safe, br, 28) == 0 {
                                    (*s).state = BROTLI_STATE_COMMAND_INNER;
                                    result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                    break 'c_1900;
                                } else {
                                    if ((*s).block_length[0 as usize] == 0) as i64 != 0 {
                                        if safe != 0 {
                                            if SafeDecodeLiteralBlockSwitch(s) == 0 {
                                                result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                                break 'c_1900;
                                            }
                                        } else {
                                            DecodeLiteralBlockSwitch(s);
                                        }
                                        if (*s).trivial_literal_context != 0 {
                                            current_block = 10079663316037948169;
                                            continue 'c_1900;
                                        }
                                    }
                                    context = (*((*s).context_lookup).offset(p1 as isize) as i32
                                        | *((*s).context_lookup)
                                            .offset(256 as isize)
                                            .offset(p2 as isize)
                                            as i32)
                                        as u8;
                                    hc = *((*s).literal_hgroup.htrees)
                                        .offset(*((*s).context_map_slice).offset(context as isize)
                                            as isize);
                                    p2 = p1;
                                    if safe == 0 {
                                        p1 = ReadSymbol(hc, br) as u8;
                                    } else {
                                        let mut literal_0: u32 = 0;
                                        if SafeReadSymbol(hc, br, &mut literal_0) == 0 {
                                            result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                            break 'c_1900;
                                        } else {
                                            p1 = literal_0 as u8;
                                        }
                                    };
                                    *((*s).ringbuffer).offset(pos as isize) = p1;
                                    let ref mut fresh58 = (*s).block_length[0 as usize];
                                    *fresh58 = (*fresh58).wrapping_sub(1);
                                    pos += 1;
                                    if (pos == (*s).ringbuffer_size) as i64 != 0 {
                                        (*s).state = BROTLI_STATE_COMMAND_INNER_WRITE;
                                        i -= 1;
                                        break 'c_1900;
                                    } else {
                                        i -= 1;
                                        if !(i != 0) {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        if !(((*s).meta_block_remaining_len <= 0) as i64 != 0) {
                            current_block = 9270490131672724345;
                            continue;
                        };
                        (*s).state = BROTLI_STATE_METABLOCK_DONE;
                        break;
                    }
                }
            }
        };
        (*s).pos = pos;
        (*s).loop_counter = i;
        return result;
    }
}

#[inline(never)]
extern "C" fn ProcessCommands(mut s: *mut BrotliDecoderStateInternal) -> i32 {
    unsafe {
        return ProcessCommandsInternal(0, s);
    }
}

#[inline(never)]
extern "C" fn SafeProcessCommands(mut s: *mut BrotliDecoderStateInternal) -> i32 {
    unsafe {
        return ProcessCommandsInternal(1, s);
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderDecompress(
    mut encoded_size: u64,
    mut encoded_buffer: *const u8,
    mut decoded_size: *mut u64,
    mut decoded_buffer: *mut u8,
) -> u32 {
    unsafe {
        let mut s = BrotliDecoderStateInternal {
            state : BROTLI_STATE_UNINITED,
            loop_counter : 0,
            br : BrotliBitReader {
                val_ : 0,
                bit_pos_ : 0,
                next_in : 0 as * const u8,
                avail_in : 0,
            },
            alloc_func : None,
            free_func : None,
            memory_manager_opaque : 0 as * mut libc :: c_void,
            buffer : C2RustUnnamed_0 {
                u64_0 : 0
            },
            buffer_length : 0,
            pos : 0,
            max_backward_distance : 0,
            max_distance : 0,
            ringbuffer_size : 0,
            ringbuffer_mask : 0,
            dist_rb_idx : 0,
            dist_rb : [0; 4],
            error_code : 0,
            ringbuffer : 0 as * mut u8,
            ringbuffer_end : 0 as * mut u8,
            htree_command : 0 as * mut HuffmanCode,
            context_lookup : 0 as * const u8,
            context_map_slice : 0 as * mut u8,
            dist_context_map_slice : 0 as * mut u8,
            literal_hgroup : HuffmanTreeGroup {
                htrees : 0 as * mut * mut HuffmanCode,
                codes : 0 as * mut HuffmanCode,
                alphabet_size_max : 0,
                alphabet_size_limit : 0,
                num_htrees : 0,
            },
            insert_copy_hgroup : HuffmanTreeGroup {
                htrees : 0 as * mut * mut HuffmanCode,
                codes : 0 as * mut HuffmanCode,
                alphabet_size_max : 0,
                alphabet_size_limit : 0,
                num_htrees : 0,
            },
            distance_hgroup : HuffmanTreeGroup {
                htrees : 0 as * mut * mut HuffmanCode,
                codes : 0 as * mut HuffmanCode,
                alphabet_size_max : 0,
                alphabet_size_limit : 0,
                num_htrees : 0,
            },
            block_type_trees : 0 as * mut HuffmanCode,
            block_len_trees : 0 as * mut HuffmanCode,
            trivial_literal_context : 0,
            distance_context : 0,
            meta_block_remaining_len : 0,
            block_length_index : 0,
            block_length : [0; 3],
            num_block_types : [0; 3],
            block_type_rb : [0; 6],
            distance_postfix_bits : 0,
            num_direct_distance_codes : 0,
            num_dist_htrees : 0,
            dist_context_map : 0 as * mut u8,
            literal_htree : 0 as * mut HuffmanCode,
            dist_htree_index : 0,
            copy_length : 0,
            distance_code : 0,
            rb_roundtrips : 0,
            partial_pos_out : 0,
            mtf_upper_bound : 0,
            mtf : [0; 65],
            substate_metablock_header : BROTLI_STATE_METABLOCK_HEADER_NONE,
            substate_uncompressed : BROTLI_STATE_UNCOMPRESSED_NONE,
            substate_decode_uint8 : BROTLI_STATE_DECODE_UINT8_NONE,
            substate_read_block_length : BROTLI_STATE_READ_BLOCK_LENGTH_NONE,
            is_last_metablock_is_uncompressed_is_metadata_should_wrap_ringbuffer_canny_ringbuffer_allocation_large_window_size_nibbles : [0; 2],
            c2rust_padding : [0; 2],
            window_bits : 0,
            new_ringbuffer_size : 0,
            num_literal_htrees : 0,
            context_map : 0 as * mut u8,
            context_modes : 0 as * mut u8,
            dictionary : 0 as * const BrotliDictionary,
            transforms : 0 as * const BrotliTransforms,
            trivial_literal_contexts : [0; 8],
            arena : C2RustUnnamed {
                header : BrotliMetablockHeaderArena {
                    substate_tree_group : BROTLI_STATE_TREE_GROUP_NONE,
                    substate_context_map : BROTLI_STATE_CONTEXT_MAP_NONE,
                    substate_huffman : BROTLI_STATE_HUFFMAN_NONE,
                    sub_loop_counter : 0,
                    repeat_code_len : 0,
                    prev_code_len : 0,
                    symbol : 0,
                    repeat : 0,
                    space : 0,
                    table : [HuffmanCode {
                        bits : 0,
                        value : 0
                    }; 32],
                    symbol_lists : 0 as * mut u16,
                    symbols_lists_array : [0; 720],
                    next_symbol : [0; 32],
                    code_length_code_lengths : [0; 18],
                    code_length_histo : [0; 16],
                    htree_index : 0,
                    next : 0 as * mut HuffmanCode,
                    context_index : 0,
                    max_run_length_prefix : 0,
                    code : 0,
                    context_map_table : [HuffmanCode {
                        bits : 0,
                        value : 0
                    }; 646],
                },
            },
        };
        let mut result = BROTLI_DECODER_RESULT_ERROR;
        let mut total_out = 0;
        let mut available_in = encoded_size;
        let mut next_in = encoded_buffer;
        let mut available_out = *decoded_size;
        let mut next_out = decoded_buffer;
        if BrotliDecoderStateInit(&mut s, None, None, 0 as *mut libc::c_void) == 0 {
            return BROTLI_DECODER_RESULT_ERROR;
        }
        result = BrotliDecoderDecompressStream(
            &mut s,
            &mut available_in,
            &mut next_in,
            &mut available_out,
            &mut next_out,
            &mut total_out,
        );
        *decoded_size = total_out;
        BrotliDecoderStateCleanup(&mut s);
        if result as u32 != BROTLI_DECODER_RESULT_SUCCESS as u32 {
            result = BROTLI_DECODER_RESULT_ERROR;
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderDecompressStream(
    mut s: *mut BrotliDecoderStateInternal,
    mut available_in: *mut u64,
    mut next_in: *mut *const u8,
    mut available_out: *mut u64,
    mut next_out: *mut *mut u8,
    mut total_out: *mut u64,
) -> u32 {
    unsafe {
        let mut result = BROTLI_DECODER_SUCCESS;
        let mut br: *mut BrotliBitReader = &mut (*s).br;
        if !total_out.is_null() {
            *total_out = (*s).partial_pos_out;
        }
        if (*s).error_code < 0 {
            return BROTLI_DECODER_RESULT_ERROR;
        }
        if *available_out != 0 && (next_out.is_null() || (*next_out).is_null()) {
            return SaveErrorCode(s, BROTLI_DECODER_ERROR_INVALID_ARGUMENTS as i32);
        }
        if *available_out == 0 {
            next_out = 0 as *mut *mut u8;
        }
        if (*s).buffer_length == 0 {
            (*br).avail_in = *available_in;
            let ref mut fresh61 = (*br).next_in;
            *fresh61 = *next_in;
        } else {
            result = BROTLI_DECODER_NEEDS_MORE_INPUT;
            let ref mut fresh62 = (*br).next_in;
            *fresh62 = &mut *((*s).buffer.u8_0).as_mut_ptr().offset(0 as isize) as *mut u8;
        }
        loop {
            if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                if result as i32 == BROTLI_DECODER_NEEDS_MORE_INPUT as i32 {
                    if !((*s).ringbuffer).is_null() {
                        let mut intermediate_result =
                            WriteRingBuffer(s, available_out, next_out, total_out, 1);
                        if (intermediate_result as i32) < 0 {
                            result = intermediate_result;
                            break;
                        }
                    }
                    if (*s).buffer_length != 0 {
                        if (*br).avail_in == 0 {
                            (*s).buffer_length = 0;
                            result = BROTLI_DECODER_SUCCESS;
                            (*br).avail_in = *available_in;
                            let ref mut fresh63 = (*br).next_in;
                            *fresh63 = *next_in;
                        } else {
                            if !(*available_in != 0) {
                                break;
                            }
                            result = BROTLI_DECODER_SUCCESS;
                            (*s).buffer.u8_0[(*s).buffer_length as usize] = **next_in;
                            let ref mut fresh64 = (*s).buffer_length;
                            *fresh64 = (*fresh64).wrapping_add(1);
                            (*br).avail_in = (*s).buffer_length as u64;
                            *next_in = (*next_in).offset(1);
                            *available_in = (*available_in).wrapping_sub(1);
                        }
                    } else {
                        *next_in = (*br).next_in;
                        *available_in = (*br).avail_in;
                        while *available_in != 0 {
                            (*s).buffer.u8_0[(*s).buffer_length as usize] = **next_in;
                            let ref mut fresh65 = (*s).buffer_length;
                            *fresh65 = (*fresh65).wrapping_add(1);
                            *next_in = (*next_in).offset(1);
                            *available_in = (*available_in).wrapping_sub(1);
                        }
                        break;
                    }
                } else {
                    if (*s).buffer_length != 0 {
                        (*s).buffer_length = 0;
                    } else {
                        BrotliBitReaderUnload(br);
                        *available_in = (*br).avail_in;
                        *next_in = (*br).next_in;
                    }
                    break;
                }
            } else {
                let mut current_block_175: u64;
                match (*s).state as u32 {
                    0 => {
                        if BrotliWarmupBitReader(br) == 0 {
                            result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                        } else {
                            result = DecodeWindowBits(s, br);
                            if !(result as i32 != BROTLI_DECODER_SUCCESS as i32) {
                                if (*s).large_window() != 0 {
                                    (*s).state = BROTLI_STATE_LARGE_WINDOW_BITS;
                                } else {
                                    (*s).state = BROTLI_STATE_INITIALIZE;
                                }
                            }
                        }
                        current_block_175 = 17836586256320518600;
                    }
                    1 => {
                        if BrotliSafeReadBits(br, 6, &mut (*s).window_bits) == 0 {
                            result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                            current_block_175 = 17836586256320518600;
                        } else if (*s).window_bits < 10 || (*s).window_bits > 30 {
                            result = BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS as i32;
                            current_block_175 = 17836586256320518600;
                        } else {
                            (*s).state = BROTLI_STATE_INITIALIZE;
                            current_block_175 = 7178192492338286402;
                        }
                    }
                    2 => {
                        current_block_175 = 7178192492338286402;
                    }
                    3 => {
                        current_block_175 = 902756599785946114;
                    }
                    4 => {
                        current_block_175 = 9738974198234964671;
                    }
                    17 => {
                        current_block_175 = 2471697540297962509;
                    }
                    18 => {
                        current_block_175 = 6909244940670113449;
                    }
                    19 => {
                        current_block_175 = 13253659531982233645;
                    }
                    20 => {
                        current_block_175 = 10241167629170301496;
                    }
                    21 => {
                        current_block_175 = 13256895345714485905;
                    }
                    11 => {
                        result =
                            CopyUncompressedBlockToOutput(available_out, next_out, total_out, s);
                        if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                            current_block_175 = 17836586256320518600;
                        } else {
                            (*s).state = BROTLI_STATE_METABLOCK_DONE;
                            current_block_175 = 17836586256320518600;
                        }
                    }
                    12 => {
                        while (*s).meta_block_remaining_len > 0 {
                            let mut bits: u32 = 0;
                            if BrotliSafeReadBits(br, 8, &mut bits) == 0 {
                                result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                break;
                            } else {
                                let ref mut fresh72 = (*s).meta_block_remaining_len;
                                *fresh72 -= 1;
                            }
                        }
                        if result as i32 == BROTLI_DECODER_SUCCESS as i32 {
                            (*s).state = BROTLI_STATE_METABLOCK_DONE;
                        }
                        current_block_175 = 17836586256320518600;
                    }
                    5 => {
                        let mut bits_0: u32 = 0;
                        if BrotliSafeReadBits(br, 6, &mut bits_0) == 0 {
                            result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                            current_block_175 = 17836586256320518600;
                        } else {
                            (*s).distance_postfix_bits = bits_0 & BitMask(2);
                            bits_0 >>= 2;
                            (*s).num_direct_distance_codes = bits_0 << (*s).distance_postfix_bits;
                            let ref mut fresh73 = (*s).context_modes;
                            *fresh73 = ((*s).alloc_func).expect("non-null function pointer")(
                                (*s).memory_manager_opaque,
                                (*s).num_block_types[0 as usize] as u64,
                            ) as *mut u8;
                            if ((*s).context_modes).is_null() {
                                result = BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES as i32;
                                current_block_175 = 17836586256320518600;
                            } else {
                                (*s).loop_counter = 0;
                                (*s).state = BROTLI_STATE_CONTEXT_MODES;
                                current_block_175 = 2839988477215445302;
                            }
                        }
                    }
                    6 => {
                        current_block_175 = 2839988477215445302;
                    }
                    22 => {
                        current_block_175 = 8539919456974388356;
                    }
                    23 => {
                        current_block_175 = 5482373152242628851;
                    }
                    24 => {
                        current_block_175 = 13391418783698890455;
                    }
                    25 => {
                        current_block_175 = 10480376545642380787;
                    }
                    7 => {
                        current_block_175 = 14745051867202743506;
                    }
                    8 => {
                        current_block_175 = 581520119535762775;
                    }
                    9 | 10 => {
                        current_block_175 = 16861845670956576353;
                    }
                    13 => {
                        current_block_175 = 5476895363554313633;
                    }
                    15 | 16 => {
                        current_block_175 = 5476895363554313633;
                    }
                    14 => {
                        if (*s).meta_block_remaining_len < 0 {
                            result = BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2 as i32;
                            current_block_175 = 17836586256320518600;
                        } else {
                            BrotliDecoderStateCleanupAfterMetablock(s);
                            if (*s).is_last_metablock() == 0 {
                                (*s).state = BROTLI_STATE_METABLOCK_BEGIN;
                                current_block_175 = 17836586256320518600;
                            } else if BrotliJumpToByteBoundary(br) == 0 {
                                result = BROTLI_DECODER_ERROR_FORMAT_PADDING_2 as i32;
                                current_block_175 = 17836586256320518600;
                            } else {
                                if (*s).buffer_length == 0 {
                                    BrotliBitReaderUnload(br);
                                    *available_in = (*br).avail_in;
                                    *next_in = (*br).next_in;
                                };
                                (*s).state = BROTLI_STATE_DONE;
                                current_block_175 = 8580127841665746230;
                            }
                        }
                    }
                    26 => {
                        current_block_175 = 8580127841665746230;
                    }
                    _ => {
                        current_block_175 = 17836586256320518600;
                    }
                }
                match current_block_175 {
                    8580127841665746230 => {
                        if !((*s).ringbuffer).is_null() {
                            result = WriteRingBuffer(s, available_out, next_out, total_out, 1);
                            if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                                current_block_175 = 17836586256320518600;
                            } else {
                                current_block_175 = 3677748771821733454;
                            }
                        } else {
                            current_block_175 = 3677748771821733454;
                        }
                        match current_block_175 {
                            17836586256320518600 => {}
                            _ => return SaveErrorCode(s, result),
                        }
                    }
                    5476895363554313633 => {
                        result = WriteRingBuffer(s, available_out, next_out, total_out, 0);
                        if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                            current_block_175 = 17836586256320518600;
                        } else {
                            WrapRingBuffer(s);
                            if (*s).ringbuffer_size == 1 << (*s).window_bits {
                                (*s).max_distance = (*s).max_backward_distance;
                            }
                            if (*s).state as u32 == BROTLI_STATE_COMMAND_POST_WRITE_1 as u32 {
                                if (*s).meta_block_remaining_len == 0 {
                                    (*s).state = BROTLI_STATE_METABLOCK_DONE;
                                } else {
                                    (*s).state = BROTLI_STATE_COMMAND_BEGIN;
                                }
                            } else if (*s).state as u32 == BROTLI_STATE_COMMAND_POST_WRITE_2 as u32
                            {
                                (*s).state = BROTLI_STATE_COMMAND_POST_WRAP_COPY;
                            } else if (*s).loop_counter == 0 {
                                if (*s).meta_block_remaining_len == 0 {
                                    (*s).state = BROTLI_STATE_METABLOCK_DONE;
                                } else {
                                    (*s).state = BROTLI_STATE_COMMAND_POST_DECODE_LITERALS;
                                }
                            } else {
                                (*s).state = BROTLI_STATE_COMMAND_INNER;
                            }
                            current_block_175 = 17836586256320518600;
                        }
                    }
                    2839988477215445302 => {
                        result = ReadContextModes(s);
                        if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                            current_block_175 = 17836586256320518600;
                        } else {
                            (*s).state = BROTLI_STATE_CONTEXT_MAP_1;
                            current_block_175 = 8539919456974388356;
                        }
                    }
                    7178192492338286402 => {
                        (*s).max_backward_distance = (1 << (*s).window_bits) - 16;
                        let ref mut fresh66 = (*s).block_type_trees;
                        *fresh66 = ((*s).alloc_func).expect("non-null function pointer")(
                            (*s).memory_manager_opaque,
                            (::std::mem::size_of::<HuffmanCode>() as u64)
                                .wrapping_mul(3)
                                .wrapping_mul((632 + 396i32) as u64),
                        ) as *mut HuffmanCode;
                        if ((*s).block_type_trees).is_null() {
                            result = BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES as i32;
                            current_block_175 = 17836586256320518600;
                        } else {
                            let ref mut fresh67 = (*s).block_len_trees;
                            *fresh67 = ((*s).block_type_trees).offset((3 * 632i32) as isize);
                            (*s).state = BROTLI_STATE_METABLOCK_BEGIN;
                            current_block_175 = 902756599785946114;
                        }
                    }
                    _ => {}
                }
                match current_block_175 {
                    8539919456974388356 => {
                        result = DecodeContextMap(
                            (*s).num_block_types[0 as usize] << 6,
                            &mut (*s).num_literal_htrees,
                            &mut (*s).context_map,
                            s,
                        );
                        if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                            current_block_175 = 17836586256320518600;
                        } else {
                            DetectTrivialLiteralBlockTypes(s);
                            (*s).state = BROTLI_STATE_CONTEXT_MAP_2;
                            current_block_175 = 5482373152242628851;
                        }
                    }
                    902756599785946114 => {
                        BrotliDecoderStateMetablockBegin(s);
                        (*s).state = BROTLI_STATE_METABLOCK_HEADER;
                        current_block_175 = 9738974198234964671;
                    }
                    _ => {}
                }
                match current_block_175 {
                    5482373152242628851 => {
                        let mut npostfix = (*s).distance_postfix_bits;
                        let mut ndirect = (*s).num_direct_distance_codes;
                        let mut distance_alphabet_size_max = 16u32
                            .wrapping_add(ndirect)
                            .wrapping_add(24u32 << npostfix.wrapping_add(1));
                        let mut distance_alphabet_size_limit = distance_alphabet_size_max;
                        let mut allocation_success = 1;
                        if (*s).large_window() != 0 {
                            let mut limit =
                                BrotliCalculateDistanceCodeLimit(0x7ffffffc, npostfix, ndirect);
                            distance_alphabet_size_max = 16u32
                                .wrapping_add(ndirect)
                                .wrapping_add(62u32 << npostfix.wrapping_add(1));
                            distance_alphabet_size_limit = limit.max_alphabet_size;
                        }
                        result = DecodeContextMap(
                            (*s).num_block_types[2 as usize] << 2,
                            &mut (*s).num_dist_htrees,
                            &mut (*s).dist_context_map,
                            s,
                        );
                        if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                            current_block_175 = 17836586256320518600;
                        } else {
                            allocation_success &= BrotliDecoderHuffmanTreeGroupInit(
                                s,
                                &mut (*s).literal_hgroup,
                                256,
                                256,
                                (*s).num_literal_htrees,
                            );
                            allocation_success &= BrotliDecoderHuffmanTreeGroupInit(
                                s,
                                &mut (*s).insert_copy_hgroup,
                                704,
                                704,
                                (*s).num_block_types[1 as usize],
                            );
                            allocation_success &= BrotliDecoderHuffmanTreeGroupInit(
                                s,
                                &mut (*s).distance_hgroup,
                                distance_alphabet_size_max,
                                distance_alphabet_size_limit,
                                (*s).num_dist_htrees,
                            );
                            if allocation_success == 0 {
                                return SaveErrorCode(
                                    s,
                                    BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS as i32,
                                );
                            };
                            (*s).loop_counter = 0;
                            (*s).state = BROTLI_STATE_TREE_GROUP;
                            current_block_175 = 13391418783698890455;
                        }
                    }
                    9738974198234964671 => {
                        result = DecodeMetaBlockLength(s, br);
                        if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                            current_block_175 = 17836586256320518600;
                        } else {
                            if (*s).is_metadata() as i32 != 0 || (*s).is_uncompressed() as i32 != 0
                            {
                                if BrotliJumpToByteBoundary(br) == 0 {
                                    result = BROTLI_DECODER_ERROR_FORMAT_PADDING_1 as i32;
                                    current_block_175 = 17836586256320518600;
                                } else {
                                    current_block_175 = 16037123508100270995;
                                }
                            } else {
                                current_block_175 = 16037123508100270995;
                            }
                            match current_block_175 {
                                17836586256320518600 => {}
                                _ => {
                                    if (*s).is_metadata() != 0 {
                                        (*s).state = BROTLI_STATE_METADATA;
                                        current_block_175 = 17836586256320518600;
                                    } else if (*s).meta_block_remaining_len == 0 {
                                        (*s).state = BROTLI_STATE_METABLOCK_DONE;
                                        current_block_175 = 17836586256320518600;
                                    } else {
                                        BrotliCalculateRingBufferSize(s);
                                        if (*s).is_uncompressed() != 0 {
                                            (*s).state = BROTLI_STATE_UNCOMPRESSED;
                                            current_block_175 = 17836586256320518600;
                                        } else {
                                            (*s).state =
                                                BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_HEADER;
                                            current_block_175 = 2471697540297962509;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                match current_block_175 {
                    13391418783698890455 => {
                        let mut hgroup = 0 as *mut HuffmanTreeGroup;
                        match (*s).loop_counter {
                            0 => {
                                hgroup = &mut (*s).literal_hgroup;
                            }
                            1 => {
                                hgroup = &mut (*s).insert_copy_hgroup;
                            }
                            2 => {
                                hgroup = &mut (*s).distance_hgroup;
                            }
                            _ => {
                                return SaveErrorCode(s, BROTLI_DECODER_ERROR_UNREACHABLE as i32);
                            }
                        }
                        result = HuffmanTreeGroupDecode(hgroup, s);
                        if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                            current_block_175 = 17836586256320518600;
                        } else {
                            let ref mut fresh74 = (*s).loop_counter;
                            *fresh74 += 1;
                            if (*s).loop_counter < 3 {
                                current_block_175 = 17836586256320518600;
                            } else {
                                (*s).state = BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_BODY;
                                current_block_175 = 10480376545642380787;
                            }
                        }
                    }
                    2471697540297962509 => {
                        let mut h: *mut BrotliMetablockHeaderArena = &mut (*s).arena.header;
                        (*s).loop_counter = 0;
                        (*h).sub_loop_counter = 0;
                        let ref mut fresh68 = (*h).symbol_lists;
                        *fresh68 = &mut *((*h).symbols_lists_array)
                            .as_mut_ptr()
                            .offset((15 + 1i32) as isize)
                            as *mut u16;
                        (*h).substate_huffman = BROTLI_STATE_HUFFMAN_NONE;
                        (*h).substate_tree_group = BROTLI_STATE_TREE_GROUP_NONE;
                        (*h).substate_context_map = BROTLI_STATE_CONTEXT_MAP_NONE;
                        (*s).state = BROTLI_STATE_HUFFMAN_CODE_0;
                        current_block_175 = 6909244940670113449;
                    }
                    _ => {}
                }
                match current_block_175 {
                    10480376545642380787 => {
                        PrepareLiteralDecoding(s);
                        let ref mut fresh75 = (*s).dist_context_map_slice;
                        *fresh75 = (*s).dist_context_map;
                        let ref mut fresh76 = (*s).htree_command;
                        *fresh76 = *((*s).insert_copy_hgroup.htrees).offset(0 as isize);
                        if BrotliEnsureRingBuffer(s) == 0 {
                            result = BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2 as i32;
                            current_block_175 = 17836586256320518600;
                        } else {
                            CalculateDistanceLut(s);
                            (*s).state = BROTLI_STATE_COMMAND_BEGIN;
                            current_block_175 = 14745051867202743506;
                        }
                    }
                    6909244940670113449 => {
                        if (*s).loop_counter >= 3 {
                            (*s).state = BROTLI_STATE_METABLOCK_HEADER_2;
                            current_block_175 = 17836586256320518600;
                        } else {
                            result = DecodeVarLenUint8(
                                s,
                                br,
                                &mut *((*s).num_block_types)
                                    .as_mut_ptr()
                                    .offset((*s).loop_counter as isize),
                            );
                            if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                                current_block_175 = 17836586256320518600;
                            } else {
                                let ref mut fresh69 =
                                    (*s).num_block_types[(*s).loop_counter as usize];
                                *fresh69 = (*fresh69).wrapping_add(1);
                                if (*s).num_block_types[(*s).loop_counter as usize] < 2 {
                                    let ref mut fresh70 = (*s).loop_counter;
                                    *fresh70 += 1;
                                    current_block_175 = 17836586256320518600;
                                } else {
                                    (*s).state = BROTLI_STATE_HUFFMAN_CODE_1;
                                    current_block_175 = 13253659531982233645;
                                }
                            }
                        }
                    }
                    _ => {}
                }
                match current_block_175 {
                    13253659531982233645 => {
                        let mut alphabet_size =
                            ((*s).num_block_types[(*s).loop_counter as usize]).wrapping_add(2);
                        let mut tree_offset = (*s).loop_counter * 632;
                        result = ReadHuffmanCode(
                            alphabet_size,
                            alphabet_size,
                            &mut *((*s).block_type_trees).offset(tree_offset as isize),
                            0 as *mut u32,
                            s,
                        );
                        if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                            current_block_175 = 17836586256320518600;
                        } else {
                            (*s).state = BROTLI_STATE_HUFFMAN_CODE_2;
                            current_block_175 = 10241167629170301496;
                        }
                    }
                    14745051867202743506 => {
                        current_block_175 = 581520119535762775;
                    }
                    _ => {}
                }
                match current_block_175 {
                    10241167629170301496 => {
                        let mut alphabet_size_0 = 26;
                        let mut tree_offset_0 = (*s).loop_counter * 396;
                        result = ReadHuffmanCode(
                            alphabet_size_0,
                            alphabet_size_0,
                            &mut *((*s).block_len_trees).offset(tree_offset_0 as isize),
                            0 as *mut u32,
                            s,
                        );
                        if result as i32 != BROTLI_DECODER_SUCCESS as i32 {
                            current_block_175 = 17836586256320518600;
                        } else {
                            (*s).state = BROTLI_STATE_HUFFMAN_CODE_3;
                            current_block_175 = 13256895345714485905;
                        }
                    }
                    581520119535762775 => {
                        current_block_175 = 16861845670956576353;
                    }
                    _ => {}
                }
                match current_block_175 {
                    16861845670956576353 => {
                        result = ProcessCommands(s);
                        if result as i32 == BROTLI_DECODER_NEEDS_MORE_INPUT as i32 {
                            result = SafeProcessCommands(s);
                        }
                    }
                    13256895345714485905 => {
                        let mut tree_offset_1 = (*s).loop_counter * 396;
                        if SafeReadBlockLength(
                            s,
                            &mut *((*s).block_length)
                                .as_mut_ptr()
                                .offset((*s).loop_counter as isize),
                            &mut *((*s).block_len_trees).offset(tree_offset_1 as isize),
                            br,
                        ) == 0
                        {
                            result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                        } else {
                            let ref mut fresh71 = (*s).loop_counter;
                            *fresh71 += 1;
                            (*s).state = BROTLI_STATE_HUFFMAN_CODE_0;
                        }
                    }
                    _ => {}
                }
            }
        }
        return SaveErrorCode(s, result);
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderHasMoreOutput(mut s: *const BrotliDecoderStateInternal) -> i32 {
    unsafe {
        if (*s).error_code < 0 {
            return 0;
        }
        return if !((*s).ringbuffer).is_null() && UnwrittenBytes(s, 0) != 0 {
            1
        } else {
            0
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderTakeOutput(
    mut s: *mut BrotliDecoderStateInternal,
    mut size: *mut u64,
) -> *const u8 {
    unsafe {
        let mut result = 0 as *mut u8;
        let mut available_out = if *size != 0 {
            *size
        } else {
            (1u32 << 24) as u64
        };
        let mut requested_out = available_out;
        let mut status = NULL;
        if ((*s).ringbuffer).is_null() || (*s).error_code < 0 {
            *size = 0;
            return 0 as *const u8;
        }
        WrapRingBuffer(s);
        status = WriteRingBuffer(s, &mut available_out, &mut result, 0 as *mut u64, 1);
        if status as i32 == BROTLI_DECODER_SUCCESS as i32
            || status as i32 == BROTLI_DECODER_NEEDS_MORE_OUTPUT as i32
        {
            *size = requested_out.wrapping_sub(available_out);
        } else {
            if (status as i32) < 0 {
                SaveErrorCode(s, status);
            }
            *size = 0;
            result = 0 as *mut u8;
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderIsUsed(mut s: *const BrotliDecoderStateInternal) -> i32 {
    unsafe {
        return if (*s).state as u32 != BROTLI_STATE_UNINITED as u32
            || BrotliGetAvailableBits(&(*s).br) != 0
        {
            1
        } else {
            0
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderIsFinished(mut s: *const BrotliDecoderStateInternal) -> i32 {
    unsafe {
        return ((if (*s).state as u32 == BROTLI_STATE_DONE as u32 {
            1
        } else {
            0
        }) != 0
            && BrotliDecoderHasMoreOutput(s) == 0) as i32;
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderGetErrorCode(mut s: *const BrotliDecoderStateInternal) -> i32 {
    unsafe {
        return (*s).error_code as i32;
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderErrorString(mut c: i32) -> *const i8 {
    match c as i32 {
        0 => return b"NO_ERROR\0" as *const u8 as *const i8,
        1 => return b"SUCCESS\0" as *const u8 as *const i8,
        2 => return b"NEEDS_MORE_INPUT\0" as *const u8 as *const i8,
        3 => return b"NEEDS_MORE_OUTPUT\0" as *const u8 as *const i8,
        -1 => return b"EXUBERANT_NIBBLE\0" as *const u8 as *const i8,
        -2 => return b"RESERVED\0" as *const u8 as *const i8,
        -3 => return b"EXUBERANT_META_NIBBLE\0" as *const u8 as *const i8,
        -4 => return b"SIMPLE_HUFFMAN_ALPHABET\0" as *const u8 as *const i8,
        -5 => return b"SIMPLE_HUFFMAN_SAME\0" as *const u8 as *const i8,
        -6 => return b"CL_SPACE\0" as *const u8 as *const i8,
        -7 => return b"HUFFMAN_SPACE\0" as *const u8 as *const i8,
        -8 => return b"CONTEXT_MAP_REPEAT\0" as *const u8 as *const i8,
        -9 => return b"BLOCK_LENGTH_1\0" as *const u8 as *const i8,
        -10 => return b"BLOCK_LENGTH_2\0" as *const u8 as *const i8,
        -11 => return b"TRANSFORM\0" as *const u8 as *const i8,
        -12 => return b"DICTIONARY\0" as *const u8 as *const i8,
        -13 => return b"WINDOW_BITS\0" as *const u8 as *const i8,
        -14 => return b"PADDING_1\0" as *const u8 as *const i8,
        -15 => return b"PADDING_2\0" as *const u8 as *const i8,
        -16 => return b"DISTANCE\0" as *const u8 as *const i8,
        -19 => return b"DICTIONARY_NOT_SET\0" as *const u8 as *const i8,
        -20 => return b"INVALID_ARGUMENTS\0" as *const u8 as *const i8,
        -21 => return b"CONTEXT_MODES\0" as *const u8 as *const i8,
        -22 => return b"TREE_GROUPS\0" as *const u8 as *const i8,
        -25 => return b"CONTEXT_MAP\0" as *const u8 as *const i8,
        -26 => return b"RING_BUFFER_1\0" as *const u8 as *const i8,
        -27 => return b"RING_BUFFER_2\0" as *const u8 as *const i8,
        -30 => return b"BLOCK_TYPE_TREES\0" as *const u8 as *const i8,
        -31 => return b"UNREACHABLE\0" as *const u8 as *const i8,
        _ => return b"INVALID\0" as *const u8 as *const i8,
    };
}

#[no_mangle]
pub extern "C" fn BrotliDecoderVersion() -> u32 {
    return 0x1000009;
}
