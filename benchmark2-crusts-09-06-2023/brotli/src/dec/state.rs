use c2rust_bitfields;
use libc;
extern "C" {
    fn BrotliInitBitReader(br: *mut BrotliBitReader);
    fn BrotliDefaultAllocFunc(opaque: *mut libc::c_void, size: u64) -> *mut libc::c_void;
    fn BrotliDefaultFreeFunc(opaque: *mut libc::c_void, address: *mut libc::c_void);
    fn BrotliGetDictionary() -> *const BrotliDictionary;
    fn BrotliGetTransforms() -> *const BrotliTransforms;
}
pub type brotli_alloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
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
pub struct BrotliBitReader {
    pub val_: u64,
    pub bit_pos_: u32,
    pub next_in: *const u8,
    pub avail_in: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanCode {
    pub bits: u8,
    pub value: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTreeGroup {
    pub htrees: *mut *mut HuffmanCode,
    pub codes: *mut HuffmanCode,
    pub alphabet_size_max: u16,
    pub alphabet_size_limit: u16,
    pub num_htrees: u16,
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
pub const BROTLI_STATE_METABLOCK_HEADER_METADATA: u32 = 7;
pub const BROTLI_STATE_METABLOCK_HEADER_BYTES: u32 = 6;
pub const BROTLI_STATE_METABLOCK_HEADER_RESERVED: u32 = 5;
pub const BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED: u32 = 4;
pub const BROTLI_STATE_METABLOCK_HEADER_SIZE: u32 = 3;
pub const BROTLI_STATE_METABLOCK_HEADER_NIBBLES: u32 = 2;
pub const BROTLI_STATE_METABLOCK_HEADER_EMPTY: u32 = 1;
pub const BROTLI_STATE_METABLOCK_HEADER_NONE: u32 = 0;
pub const BROTLI_STATE_UNCOMPRESSED_WRITE: u32 = 1;
pub const BROTLI_STATE_UNCOMPRESSED_NONE: u32 = 0;
pub const BROTLI_STATE_TREE_GROUP_LOOP: u32 = 1;
pub const BROTLI_STATE_TREE_GROUP_NONE: u32 = 0;
pub const BROTLI_STATE_CONTEXT_MAP_TRANSFORM: u32 = 4;
pub const BROTLI_STATE_CONTEXT_MAP_DECODE: u32 = 3;
pub const BROTLI_STATE_CONTEXT_MAP_HUFFMAN: u32 = 2;
pub const BROTLI_STATE_CONTEXT_MAP_READ_PREFIX: u32 = 1;
pub const BROTLI_STATE_CONTEXT_MAP_NONE: u32 = 0;
pub const BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS: u32 = 5;
pub const BROTLI_STATE_HUFFMAN_COMPLEX: u32 = 4;
pub const BROTLI_STATE_HUFFMAN_SIMPLE_BUILD: u32 = 3;
pub const BROTLI_STATE_HUFFMAN_SIMPLE_READ: u32 = 2;
pub const BROTLI_STATE_HUFFMAN_SIMPLE_SIZE: u32 = 1;
pub const BROTLI_STATE_HUFFMAN_NONE: u32 = 0;
pub const BROTLI_STATE_DECODE_UINT8_LONG: u32 = 2;
pub const BROTLI_STATE_DECODE_UINT8_SHORT: u32 = 1;
pub const BROTLI_STATE_DECODE_UINT8_NONE: u32 = 0;
pub const BROTLI_STATE_READ_BLOCK_LENGTH_SUFFIX: u32 = 1;
pub const BROTLI_STATE_READ_BLOCK_LENGTH_NONE: u32 = 0;
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
pub struct BrotliMetablockBodyArena {
    pub dist_extra_bits: [u8; 544],
    pub dist_offset: [u32; 544],
}
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
pub union C2RustUnnamed_0 {
    pub u64_0: u64,
    pub u8_0: [u8; 8],
}
pub type BrotliDecoderStateInternal = BrotliDecoderStateStruct;
#[no_mangle]
pub extern "C" fn BrotliDecoderStateInit(
    mut s: *mut BrotliDecoderStateInternal,
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut libc::c_void,
) -> i32 {
    unsafe {
        if alloc_func.is_none() {
            let ref mut fresh0 = (*s).alloc_func;
            *fresh0 = Some(
                BrotliDefaultAllocFunc
                    as unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void,
            );
            let ref mut fresh1 = (*s).free_func;
            *fresh1 = Some(
                BrotliDefaultFreeFunc
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            );
            let ref mut fresh2 = (*s).memory_manager_opaque;
            *fresh2 = 0 as *mut libc::c_void;
        } else {
            let ref mut fresh3 = (*s).alloc_func;
            *fresh3 = alloc_func;
            let ref mut fresh4 = (*s).free_func;
            *fresh4 = free_func;
            let ref mut fresh5 = (*s).memory_manager_opaque;
            *fresh5 = opaque;
        };
        (*s).error_code = 0;
        BrotliInitBitReader(&mut (*s).br);
        (*s).state = BROTLI_STATE_UNINITED;
        (*s).set_large_window(0);
        (*s).substate_metablock_header = BROTLI_STATE_METABLOCK_HEADER_NONE;
        (*s).substate_uncompressed = BROTLI_STATE_UNCOMPRESSED_NONE;
        (*s).substate_decode_uint8 = BROTLI_STATE_DECODE_UINT8_NONE;
        (*s).substate_read_block_length = BROTLI_STATE_READ_BLOCK_LENGTH_NONE;
        (*s).buffer_length = 0;
        (*s).loop_counter = 0;
        (*s).pos = 0;
        (*s).rb_roundtrips = 0;
        (*s).partial_pos_out = 0;
        let ref mut fresh6 = (*s).block_type_trees;
        *fresh6 = 0 as *mut HuffmanCode;
        let ref mut fresh7 = (*s).block_len_trees;
        *fresh7 = 0 as *mut HuffmanCode;
        let ref mut fresh8 = (*s).ringbuffer;
        *fresh8 = 0 as *mut u8;
        (*s).ringbuffer_size = 0;
        (*s).new_ringbuffer_size = 0;
        (*s).ringbuffer_mask = 0;
        let ref mut fresh9 = (*s).context_map;
        *fresh9 = 0 as *mut u8;
        let ref mut fresh10 = (*s).context_modes;
        *fresh10 = 0 as *mut u8;
        let ref mut fresh11 = (*s).dist_context_map;
        *fresh11 = 0 as *mut u8;
        let ref mut fresh12 = (*s).context_map_slice;
        *fresh12 = 0 as *mut u8;
        let ref mut fresh13 = (*s).dist_context_map_slice;
        *fresh13 = 0 as *mut u8;
        let ref mut fresh14 = (*s).literal_hgroup.codes;
        *fresh14 = 0 as *mut HuffmanCode;
        let ref mut fresh15 = (*s).literal_hgroup.htrees;
        *fresh15 = 0 as *mut *mut HuffmanCode;
        let ref mut fresh16 = (*s).insert_copy_hgroup.codes;
        *fresh16 = 0 as *mut HuffmanCode;
        let ref mut fresh17 = (*s).insert_copy_hgroup.htrees;
        *fresh17 = 0 as *mut *mut HuffmanCode;
        let ref mut fresh18 = (*s).distance_hgroup.codes;
        *fresh18 = 0 as *mut HuffmanCode;
        let ref mut fresh19 = (*s).distance_hgroup.htrees;
        *fresh19 = 0 as *mut *mut HuffmanCode;
        (*s).set_is_last_metablock(0);
        (*s).set_is_uncompressed(0);
        (*s).set_is_metadata(0);
        (*s).set_should_wrap_ringbuffer(0);
        (*s).set_canny_ringbuffer_allocation(1);
        (*s).window_bits = 0;
        (*s).max_distance = 0;
        (*s).dist_rb[0 as usize] = 16;
        (*s).dist_rb[1 as usize] = 15;
        (*s).dist_rb[2 as usize] = 11;
        (*s).dist_rb[3 as usize] = 4;
        (*s).dist_rb_idx = 0;
        let ref mut fresh20 = (*s).block_type_trees;
        *fresh20 = 0 as *mut HuffmanCode;
        let ref mut fresh21 = (*s).block_len_trees;
        *fresh21 = 0 as *mut HuffmanCode;
        (*s).mtf_upper_bound = 63;
        let ref mut fresh22 = (*s).dictionary;
        *fresh22 = BrotliGetDictionary();
        let ref mut fresh23 = (*s).transforms;
        *fresh23 = BrotliGetTransforms();
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderStateMetablockBegin(mut s: *mut BrotliDecoderStateInternal) {
    unsafe {
        (*s).meta_block_remaining_len = 0;
        (*s).block_length[0 as usize] = 1 << 24;
        (*s).block_length[1 as usize] = 1 << 24;
        (*s).block_length[2 as usize] = 1 << 24;
        (*s).num_block_types[0 as usize] = 1;
        (*s).num_block_types[1 as usize] = 1;
        (*s).num_block_types[2 as usize] = 1;
        (*s).block_type_rb[0 as usize] = 1;
        (*s).block_type_rb[1 as usize] = 0;
        (*s).block_type_rb[2 as usize] = 1;
        (*s).block_type_rb[3 as usize] = 0;
        (*s).block_type_rb[4 as usize] = 1;
        (*s).block_type_rb[5 as usize] = 0;
        let ref mut fresh24 = (*s).context_map;
        *fresh24 = 0 as *mut u8;
        let ref mut fresh25 = (*s).context_modes;
        *fresh25 = 0 as *mut u8;
        let ref mut fresh26 = (*s).dist_context_map;
        *fresh26 = 0 as *mut u8;
        let ref mut fresh27 = (*s).context_map_slice;
        *fresh27 = 0 as *mut u8;
        let ref mut fresh28 = (*s).literal_htree;
        *fresh28 = 0 as *mut HuffmanCode;
        let ref mut fresh29 = (*s).dist_context_map_slice;
        *fresh29 = 0 as *mut u8;
        (*s).dist_htree_index = 0;
        let ref mut fresh30 = (*s).context_lookup;
        *fresh30 = 0 as *const u8;
        let ref mut fresh31 = (*s).literal_hgroup.codes;
        *fresh31 = 0 as *mut HuffmanCode;
        let ref mut fresh32 = (*s).literal_hgroup.htrees;
        *fresh32 = 0 as *mut *mut HuffmanCode;
        let ref mut fresh33 = (*s).insert_copy_hgroup.codes;
        *fresh33 = 0 as *mut HuffmanCode;
        let ref mut fresh34 = (*s).insert_copy_hgroup.htrees;
        *fresh34 = 0 as *mut *mut HuffmanCode;
        let ref mut fresh35 = (*s).distance_hgroup.codes;
        *fresh35 = 0 as *mut HuffmanCode;
        let ref mut fresh36 = (*s).distance_hgroup.htrees;
        *fresh36 = 0 as *mut *mut HuffmanCode;
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderStateCleanupAfterMetablock(mut s: *mut BrotliDecoderStateInternal) {
    unsafe {
        ((*s).free_func).expect("non-null function pointer")(
            (*s).memory_manager_opaque,
            (*s).context_modes as *mut libc::c_void,
        );
        let ref mut fresh37 = (*s).context_modes;
        *fresh37 = 0 as *mut u8;
        ((*s).free_func).expect("non-null function pointer")(
            (*s).memory_manager_opaque,
            (*s).context_map as *mut libc::c_void,
        );
        let ref mut fresh38 = (*s).context_map;
        *fresh38 = 0 as *mut u8;
        ((*s).free_func).expect("non-null function pointer")(
            (*s).memory_manager_opaque,
            (*s).dist_context_map as *mut libc::c_void,
        );
        let ref mut fresh39 = (*s).dist_context_map;
        *fresh39 = 0 as *mut u8;
        ((*s).free_func).expect("non-null function pointer")(
            (*s).memory_manager_opaque,
            (*s).literal_hgroup.htrees as *mut libc::c_void,
        );
        let ref mut fresh40 = (*s).literal_hgroup.htrees;
        *fresh40 = 0 as *mut *mut HuffmanCode;
        ((*s).free_func).expect("non-null function pointer")(
            (*s).memory_manager_opaque,
            (*s).insert_copy_hgroup.htrees as *mut libc::c_void,
        );
        let ref mut fresh41 = (*s).insert_copy_hgroup.htrees;
        *fresh41 = 0 as *mut *mut HuffmanCode;
        ((*s).free_func).expect("non-null function pointer")(
            (*s).memory_manager_opaque,
            (*s).distance_hgroup.htrees as *mut libc::c_void,
        );
        let ref mut fresh42 = (*s).distance_hgroup.htrees;
        *fresh42 = 0 as *mut *mut HuffmanCode;
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderStateCleanup(mut s: *mut BrotliDecoderStateInternal) {
    unsafe {
        BrotliDecoderStateCleanupAfterMetablock(s);
        ((*s).free_func).expect("non-null function pointer")(
            (*s).memory_manager_opaque,
            (*s).ringbuffer as *mut libc::c_void,
        );
        let ref mut fresh43 = (*s).ringbuffer;
        *fresh43 = 0 as *mut u8;
        ((*s).free_func).expect("non-null function pointer")(
            (*s).memory_manager_opaque,
            (*s).block_type_trees as *mut libc::c_void,
        );
        let ref mut fresh44 = (*s).block_type_trees;
        *fresh44 = 0 as *mut HuffmanCode;
    }
}

#[no_mangle]
pub extern "C" fn BrotliDecoderHuffmanTreeGroupInit(
    mut s: *mut BrotliDecoderStateInternal,
    mut group: *mut HuffmanTreeGroup,
    mut alphabet_size_max: u32,
    mut alphabet_size_limit: u32,
    mut ntrees: u32,
) -> i32 {
    unsafe {
        let max_table_size = alphabet_size_limit.wrapping_add(376) as u64;
        let code_size = (::std::mem::size_of::<HuffmanCode>() as u64)
            .wrapping_mul(ntrees as u64)
            .wrapping_mul(max_table_size);
        let htree_size =
            (::std::mem::size_of::<*mut HuffmanCode>() as u64).wrapping_mul(ntrees as u64);
        let mut p = ((*s).alloc_func).expect("non-null function pointer")(
            (*s).memory_manager_opaque,
            code_size.wrapping_add(htree_size),
        ) as *mut *mut HuffmanCode;
        (*group).alphabet_size_max = alphabet_size_max as u16;
        (*group).alphabet_size_limit = alphabet_size_limit as u16;
        (*group).num_htrees = ntrees as u16;
        let ref mut fresh45 = (*group).htrees;
        *fresh45 = p;
        let ref mut fresh46 = (*group).codes;
        *fresh46 = &mut *p.offset(ntrees as isize) as *mut *mut HuffmanCode as *mut HuffmanCode;
        return !p.is_null() as i32;
    }
}
