pub const NULL: i32 = 0;
use libc;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    static _kBrotliContextLookupTable: [u8; 2048];
    static kBrotliLog2Table: [f64; 256];
    fn BrotliCreateBackwardReferences(
        num_bytes: u64,
        position: u64,
        ringbuffer: *const u8,
        ringbuffer_mask: u64,
        literal_context_lut: ContextLut,
        params: *const BrotliEncoderParams,
        hasher: *mut Hasher,
        dist_cache: *mut i32,
        last_insert_len: *mut u64,
        commands: *mut Command,
        num_commands: *mut u64,
        num_literals: *mut u64,
    );
    fn log2(_: f64) -> f64;
    fn BrotliAllocate(m: *mut MemoryManager, n: u64) -> *mut libc::c_void;
    fn BrotliWipeOutMemoryManager(m: *mut MemoryManager);
    fn BrotliFree(m: *mut MemoryManager, p: *mut libc::c_void);
    fn BrotliInitMemoryManager(
        m: *mut MemoryManager,
        alloc_func: brotli_alloc_func,
        free_func: brotli_free_func,
        opaque: *mut libc::c_void,
    );
    fn BrotliInitEncoderDictionary(dict: *mut BrotliEncoderDictionary);
    fn BrotliCreateZopfliBackwardReferences(
        m: *mut MemoryManager,
        num_bytes: u64,
        position: u64,
        ringbuffer: *const u8,
        ringbuffer_mask: u64,
        literal_context_lut: ContextLut,
        params: *const BrotliEncoderParams,
        hasher: *mut Hasher,
        dist_cache: *mut i32,
        last_insert_len: *mut u64,
        commands: *mut Command,
        num_commands: *mut u64,
        num_literals: *mut u64,
    );
    fn BrotliCreateHqZopfliBackwardReferences(
        m: *mut MemoryManager,
        num_bytes: u64,
        position: u64,
        ringbuffer: *const u8,
        ringbuffer_mask: u64,
        literal_context_lut: ContextLut,
        params: *const BrotliEncoderParams,
        hasher: *mut Hasher,
        dist_cache: *mut i32,
        last_insert_len: *mut u64,
        commands: *mut Command,
        num_commands: *mut u64,
        num_literals: *mut u64,
    );
    fn BrotliInitZopfliNodes(array: *mut ZopfliNode, length: u64);
    fn BrotliZopfliComputeShortestPath(
        m: *mut MemoryManager,
        num_bytes: u64,
        position: u64,
        ringbuffer: *const u8,
        ringbuffer_mask: u64,
        literal_context_lut: ContextLut,
        params: *const BrotliEncoderParams,
        dist_cache: *const i32,
        hasher: *mut Hasher,
        nodes: *mut ZopfliNode,
    ) -> u64;
    fn BrotliZopfliCreateCommands(
        num_bytes: u64,
        block_start: u64,
        nodes: *const ZopfliNode,
        dist_cache: *mut i32,
        last_insert_len: *mut u64,
        params: *const BrotliEncoderParams,
        commands: *mut Command,
        num_literals: *mut u64,
    );
    fn BrotliInitBlockSplit(self_0: *mut BlockSplit);
    fn BrotliDestroyBlockSplit(m: *mut MemoryManager, self_0: *mut BlockSplit);
    fn BrotliBuildMetaBlock(
        m: *mut MemoryManager,
        ringbuffer: *const u8,
        pos: u64,
        mask: u64,
        params: *mut BrotliEncoderParams,
        prev_byte: u8,
        prev_byte2: u8,
        cmds: *mut Command,
        num_commands: u64,
        literal_context_mode: u32,
        mb: *mut MetaBlockSplit,
    );
    fn BrotliStoreMetaBlockFast(
        m: *mut MemoryManager,
        input: *const u8,
        start_pos: u64,
        length: u64,
        mask: u64,
        is_last: i32,
        params: *const BrotliEncoderParams,
        commands: *const Command,
        n_commands: u64,
        storage_ix: *mut u64,
        storage: *mut u8,
    );
    fn BrotliStoreMetaBlock(
        m: *mut MemoryManager,
        input: *const u8,
        start_pos: u64,
        length: u64,
        mask: u64,
        prev_byte: u8,
        prev_byte2: u8,
        is_last: i32,
        params: *const BrotliEncoderParams,
        literal_context_mode: u32,
        commands: *const Command,
        n_commands: u64,
        mb: *const MetaBlockSplit,
        storage_ix: *mut u64,
        storage: *mut u8,
    );
    fn BrotliOptimizeHistograms(num_distance_codes: u32, mb: *mut MetaBlockSplit);
    fn BrotliBuildMetaBlockGreedy(
        m: *mut MemoryManager,
        ringbuffer: *const u8,
        pos: u64,
        mask: u64,
        prev_byte: u8,
        prev_byte2: u8,
        literal_context_lut: ContextLut,
        num_contexts: u64,
        static_context_map: *const u32,
        commands: *const Command,
        n_commands: u64,
        mb: *mut MetaBlockSplit,
    );
    fn BrotliStoreMetaBlockTrivial(
        m: *mut MemoryManager,
        input: *const u8,
        start_pos: u64,
        length: u64,
        mask: u64,
        is_last: i32,
        params: *const BrotliEncoderParams,
        commands: *const Command,
        n_commands: u64,
        storage_ix: *mut u64,
        storage: *mut u8,
    );
    fn BrotliInitDistanceParams(params: *mut BrotliEncoderParams, npostfix: u32, ndirect: u32);
    fn BrotliStoreUncompressedMetaBlock(
        is_final_block: i32,
        input: *const u8,
        position: u64,
        mask: u64,
        len: u64,
        storage_ix: *mut u64,
        storage: *mut u8,
    );
    fn BrotliCompressFragmentFast(
        m: *mut MemoryManager,
        input: *const u8,
        input_size: u64,
        is_last: i32,
        table: *mut i32,
        table_size: u64,
        cmd_depth: *mut u8,
        cmd_bits: *mut u16,
        cmd_code_numbits: *mut u64,
        cmd_code: *mut u8,
        storage_ix: *mut u64,
        storage: *mut u8,
    );
    fn BrotliCompressFragmentTwoPass(
        m: *mut MemoryManager,
        input: *const u8,
        input_size: u64,
        is_last: i32,
        command_buf: *mut u32,
        literal_buf: *mut u8,
        table: *mut i32,
        table_size: u64,
        storage_ix: *mut u64,
        storage: *mut u8,
    );
    fn BrotliIsMostlyUTF8(
        data: *const u8,
        pos: u64,
        mask: u64,
        length: u64,
        min_fraction: f64,
    ) -> i32;
}
pub type brotli_alloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
pub const BROTLI_MODE_FONT: u32 = 2;
pub const BROTLI_MODE_TEXT: u32 = 1;
pub const BROTLI_MODE_GENERIC: u32 = 0;
pub const BROTLI_OPERATION_EMIT_METADATA: u32 = 3;
pub const BROTLI_OPERATION_FINISH: u32 = 2;
pub const BROTLI_OPERATION_FLUSH: u32 = 1;
pub const BROTLI_OPERATION_PROCESS: u32 = 0;
pub const BROTLI_PARAM_STREAM_OFFSET: u32 = 9;
pub const BROTLI_PARAM_NDIRECT: u32 = 8;
pub const BROTLI_PARAM_NPOSTFIX: u32 = 7;
pub const BROTLI_PARAM_LARGE_WINDOW: u32 = 6;
pub const BROTLI_PARAM_SIZE_HINT: u32 = 5;
pub const BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING: u32 = 4;
pub const BROTLI_PARAM_LGBLOCK: u32 = 3;
pub const BROTLI_PARAM_LGWIN: u32 = 2;
pub const BROTLI_PARAM_QUALITY: u32 = 1;
pub const BROTLI_PARAM_MODE: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BrotliEncoderStateStruct {
    pub params: BrotliEncoderParams,
    pub memory_manager_: MemoryManager,
    pub input_pos_: u64,
    pub ringbuffer_: RingBuffer,
    pub cmd_alloc_size_: u64,
    pub commands_: *mut Command,
    pub num_commands_: u64,
    pub num_literals_: u64,
    pub last_insert_len_: u64,
    pub last_flush_pos_: u64,
    pub last_processed_pos_: u64,
    pub dist_cache_: [i32; 16],
    pub saved_dist_cache_: [i32; 4],
    pub last_bytes_: u16,
    pub last_bytes_bits_: u8,
    pub flint_: i8,
    pub prev_byte_: u8,
    pub prev_byte2_: u8,
    pub storage_size_: u64,
    pub storage_: *mut u8,
    pub hasher_: Hasher,
    pub small_table_: [i32; 1024],
    pub large_table_: *mut i32,
    pub large_table_size_: u64,
    pub cmd_depths_: [u8; 128],
    pub cmd_bits_: [u16; 128],
    pub cmd_code_: [u8; 512],
    pub cmd_code_numbits_: u64,
    pub command_buf_: *mut u32,
    pub literal_buf_: *mut u8,
    pub next_out_: *mut u8,
    pub available_out_: u64,
    pub total_out_: u64,
    pub tiny_buf_: C2RustUnnamed,
    pub remaining_metadata_bytes_: u32,
    pub stream_state_: u32,
    pub is_last_block_emitted_: i32,
    pub is_initialized_: i32,
}
pub const BROTLI_STREAM_METADATA_BODY: u32 = 4;
pub const BROTLI_STREAM_METADATA_HEAD: u32 = 3;
pub const BROTLI_STREAM_FINISHED: u32 = 2;
pub const BROTLI_STREAM_FLUSH_REQUESTED: u32 = 1;
pub const BROTLI_STREAM_PROCESSING: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u64_0: [u64; 2],
    pub u8_0: [u8; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hasher {
    pub common: HasherCommon,
    pub privat: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub struct H10 {
    pub window_mask_: u64,
    pub buckets_: *mut u32,
    pub invalid_pos_: u32,
    pub forest_: *mut u32,
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
pub struct DictWord {
    pub len: u8,
    pub transform: u8,
    pub idx: u16,
}
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
pub struct BrotliDistanceParams {
    pub distance_postfix_bits: u32,
    pub num_direct_distance_codes: u32,
    pub alphabet_size_max: u32,
    pub alphabet_size_limit: u32,
    pub max_distance: u64,
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
pub struct HasherCommon {
    pub extra: *mut libc::c_void,
    pub dict_num_lookups: u64,
    pub dict_num_matches: u64,
    pub params: BrotliHasherParams,
    pub is_prepared_: i32,
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
pub struct H54 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut u32,
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
pub struct H3 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut u32,
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
pub struct H41 {
    pub free_slot_idx: [u16; 1],
    pub max_hops: u64,
    pub extra: *mut libc::c_void,
    pub common: *mut HasherCommon,
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
pub struct H4 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2 {
    pub common: *mut HasherCommon,
    pub buckets_: *mut u32,
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
pub struct RingBuffer {
    pub size_: u32,
    pub mask_: u32,
    pub tail_size_: u32,
    pub total_size_: u32,
    pub cur_size_: u32,
    pub pos_: u32,
    pub data_: *mut u8,
    pub buffer_: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub opaque: *mut libc::c_void,
}
pub type BrotliEncoderState = BrotliEncoderStateStruct;
pub const CONTEXT_SIGNED: u32 = 3;
pub const CONTEXT_UTF8: u32 = 2;
pub const CONTEXT_MSB6: u32 = 1;
pub const CONTEXT_LSB6: u32 = 0;
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
pub struct HistogramDistance {
    pub data_: [u32; 544],
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
pub struct HistogramLiteral {
    pub data_: [u32; 256],
    pub total_count_: u64,
    pub bit_cost_: f64,
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
pub type ContextLut = *const u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BackwardMatch {
    pub distance: u32,
    pub length_and_code: u32,
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
pub struct SlotH40 {
    pub delta: u16,
    pub next: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BankH40 {
    pub slots: [SlotH40; 65536],
}
pub const BROTLI_FLINT_WAITING_FOR_FLUSHING: i32 = -1;
pub const BROTLI_FLINT_DONE: i32 = -2;
pub const BROTLI_FLINT_NEEDS_2_BYTES: i32 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZopfliNode {
    pub length: u32,
    pub distance: u32,
    pub dcode_insert_length: u32,
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub cost: libc::c_float,
    pub next: u32,
    pub shortcut: u32,
}
pub const BROTLI_FLINT_NEEDS_1_BYTE: i32 = 1;
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
extern "C" fn brotli_max_int(mut a: i32, mut b: i32) -> i32 {
    return if a > b { a } else { b };
}

#[inline(always)]
extern "C" fn brotli_min_int(mut a: i32, mut b: i32) -> i32 {
    return if a < b { a } else { b };
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
extern "C" fn brotli_min_uint32_t(mut a: u32, mut b: u32) -> u32 {
    return if a < b { a } else { b };
}

#[inline(always)]
extern "C" fn HashTypeLengthH4() -> u64 {
    return 8;
}

#[inline(always)]
extern "C" fn ComputeLgBlock(mut params: *const BrotliEncoderParams) -> i32 {
    unsafe {
        let mut lgblock = (*params).lgblock;
        if (*params).quality == 0 || (*params).quality == 1 {
            lgblock = (*params).lgwin;
        } else if (*params).quality < 4 {
            lgblock = 14;
        } else if lgblock == 0 {
            lgblock = 16;
            if (*params).quality >= 9 && (*params).lgwin > lgblock {
                lgblock = brotli_min_int(18, (*params).lgwin);
            }
        } else {
            lgblock = brotli_min_int(24, brotli_max_int(16, lgblock));
        }
        return lgblock;
    }
}

#[inline(always)]
extern "C" fn SanitizeParams(mut params: *mut BrotliEncoderParams) {
    unsafe {
        (*params).quality = brotli_min_int(11, brotli_max_int(0, (*params).quality));
        if (*params).quality <= 2 {
            (*params).large_window = 0;
        }
        if (*params).lgwin < 10 {
            (*params).lgwin = 10;
        } else {
            let mut max_lgwin = if (*params).large_window != 0 { 30 } else { 24 };
            if (*params).lgwin > max_lgwin {
                (*params).lgwin = max_lgwin;
            }
        };
    }
}

#[inline(always)]
extern "C" fn HashBytesH40(mut data: *const u8) -> u64 {
    unsafe {
        let h = (BrotliUnalignedRead32(data as *const libc::c_void)).wrapping_mul(kHashMul32);
        return (h >> 32 - 15i32) as u64;
    }
}

#[inline(always)]
extern "C" fn MaxHashTableSize(mut quality: i32) -> u64 {
    return (if quality == 0i32 { 1 << 15 } else { 1 << 17 }) as u64;
}

#[inline(always)]
extern "C" fn ChooseHasher(
    mut params: *const BrotliEncoderParams,
    mut hparams: *mut BrotliHasherParams,
) {
    unsafe {
        if (*params).quality > 9 {
            (*hparams).type_0 = 10;
        } else if (*params).quality == 4 && (*params).size_hint >= (1i32 << 20) as u64 {
            (*hparams).type_0 = 54;
        } else if (*params).quality < 5 {
            (*hparams).type_0 = (*params).quality;
        } else if (*params).lgwin <= 16 {
            (*hparams).type_0 = if (*params).quality < 7 {
                40
            } else if (*params).quality < 9 {
                41
            } else {
                42
            };
        } else if (*params).size_hint >= (1i32 << 20) as u64 && (*params).lgwin >= 19 {
            (*hparams).type_0 = 6;
            (*hparams).block_bits = (*params).quality - 1;
            (*hparams).bucket_bits = 15;
            (*hparams).hash_len = 5;
            (*hparams).num_last_distances_to_check = if (*params).quality < 7 {
                4
            } else if (*params).quality < 9 {
                10
            } else {
                16
            };
        } else {
            (*hparams).type_0 = 5;
            (*hparams).block_bits = (*params).quality - 1;
            (*hparams).bucket_bits = if (*params).quality < 7 { 14 } else { 15 };
            (*hparams).num_last_distances_to_check = if (*params).quality < 7 {
                4
            } else if (*params).quality < 9 {
                10
            } else {
                16
            };
        }
        if (*params).lgwin > 24 {
            if (*hparams).type_0 == 3 {
                (*hparams).type_0 = 35;
            }
            if (*hparams).type_0 == 54 {
                (*hparams).type_0 = 55;
            }
            if (*hparams).type_0 == 6 {
                (*hparams).type_0 = 65;
            }
        }
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesH2(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        return (::std::mem::size_of::<u32>() as u64).wrapping_mul((1i32 << 16) as u64);
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesH4(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        return (::std::mem::size_of::<u32>() as u64).wrapping_mul((1i32 << 17) as u64);
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesH5(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        let mut bucket_size = 1 << (*params).hasher.bucket_bits;
        let mut block_size = 1 << (*params).hasher.block_bits;
        return (::std::mem::size_of::<u16>() as u64)
            .wrapping_mul(bucket_size)
            .wrapping_add(
                (::std::mem::size_of::<u32>() as u64)
                    .wrapping_mul(bucket_size)
                    .wrapping_mul(block_size),
            );
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesH40(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        return (::std::mem::size_of::<u32>() as u64)
            .wrapping_mul((1i32 << 15) as u64)
            .wrapping_add((::std::mem::size_of::<u16>() as u64).wrapping_mul((1i32 << 15) as u64))
            .wrapping_add((::std::mem::size_of::<u8>() as u64).wrapping_mul(65536))
            .wrapping_add((::std::mem::size_of::<BankH40>() as u64).wrapping_mul(1));
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesH41(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        return (::std::mem::size_of::<u32>() as u64)
            .wrapping_mul((1i32 << 15) as u64)
            .wrapping_add((::std::mem::size_of::<u16>() as u64).wrapping_mul((1i32 << 15) as u64))
            .wrapping_add((::std::mem::size_of::<u8>() as u64).wrapping_mul(65536))
            .wrapping_add((::std::mem::size_of::<BankH41>() as u64).wrapping_mul(1));
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesH42(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        return (::std::mem::size_of::<u32>() as u64)
            .wrapping_mul((1i32 << 15) as u64)
            .wrapping_add((::std::mem::size_of::<u16>() as u64).wrapping_mul((1i32 << 15) as u64))
            .wrapping_add((::std::mem::size_of::<u8>() as u64).wrapping_mul(65536))
            .wrapping_add((::std::mem::size_of::<BankH42>() as u64).wrapping_mul(512));
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesH35(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        return (HashMemAllocInBytesH3(params, one_shot, input_size)).wrapping_add(
            HashMemAllocInBytesHROLLING_FAST(params, one_shot, input_size),
        );
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesHROLLING_FAST(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        return 16777216u64.wrapping_mul(::std::mem::size_of::<u32>() as u64);
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesH55(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        return (HashMemAllocInBytesH54(params, one_shot, input_size)).wrapping_add(
            HashMemAllocInBytesHROLLING_FAST(params, one_shot, input_size),
        );
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesHROLLING(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        return 16777216u64.wrapping_mul(::std::mem::size_of::<u32>() as u64);
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesH65(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        return (HashMemAllocInBytesH6(params, one_shot, input_size))
            .wrapping_add(HashMemAllocInBytesHROLLING(params, one_shot, input_size));
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesH10(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        let mut num_nodes = 1 << (*params).lgwin;
        if one_shot != 0 && input_size < num_nodes {
            num_nodes = input_size;
        }
        return (::std::mem::size_of::<u32>() as u64)
            .wrapping_mul((1i32 << 17) as u64)
            .wrapping_add(
                2u64.wrapping_mul(::std::mem::size_of::<u32>() as u64)
                    .wrapping_mul(num_nodes),
            );
    }
}

#[inline(always)]
extern "C" fn HasherSize(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    input_size: u64,
) -> u64 {
    unsafe {
        match (*params).hasher.type_0 {
            2 => return HashMemAllocInBytesH2(params, one_shot, input_size),
            3 => return HashMemAllocInBytesH3(params, one_shot, input_size),
            4 => return HashMemAllocInBytesH4(params, one_shot, input_size),
            5 => return HashMemAllocInBytesH5(params, one_shot, input_size),
            6 => return HashMemAllocInBytesH6(params, one_shot, input_size),
            40 => return HashMemAllocInBytesH40(params, one_shot, input_size),
            41 => return HashMemAllocInBytesH41(params, one_shot, input_size),
            42 => return HashMemAllocInBytesH42(params, one_shot, input_size),
            54 => return HashMemAllocInBytesH54(params, one_shot, input_size),
            35 => return HashMemAllocInBytesH35(params, one_shot, input_size),
            55 => return HashMemAllocInBytesH55(params, one_shot, input_size),
            65 => return HashMemAllocInBytesH65(params, one_shot, input_size),
            10 => return HashMemAllocInBytesH10(params, one_shot, input_size),
            _ => {}
        }
        return 0;
    }
}

extern "C" fn InitializeH2(
    mut common: *mut HasherCommon,
    mut self_0: *mut H2,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let ref mut fresh0 = (*self_0).common;
        *fresh0 = common;
        let ref mut fresh1 = (*self_0).buckets_;
        *fresh1 = (*common).extra as *mut u32;
    }
}

extern "C" fn InitializeH4(
    mut common: *mut HasherCommon,
    mut self_0: *mut H4,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let ref mut fresh2 = (*self_0).common;
        *fresh2 = common;
        let ref mut fresh3 = (*self_0).buckets_;
        *fresh3 = (*common).extra as *mut u32;
    }
}

extern "C" fn InitializeH5(
    mut common: *mut HasherCommon,
    mut self_0: *mut H5,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let ref mut fresh4 = (*self_0).common_;
        *fresh4 = common;
        (*self_0).hash_shift_ = 32 - (*common).params.bucket_bits;
        (*self_0).bucket_size_ = 1 << (*common).params.bucket_bits;
        (*self_0).block_size_ = 1 << (*common).params.block_bits;
        (*self_0).block_mask_ = ((*self_0).block_size_).wrapping_sub(1) as u32;
        let ref mut fresh5 = (*self_0).num_;
        *fresh5 = (*common).extra as *mut u16;
        let ref mut fresh6 = (*self_0).buckets_;
        *fresh6 =
            &mut *((*self_0).num_).offset((*self_0).bucket_size_ as isize) as *mut u16 as *mut u32;
        (*self_0).block_bits_ = (*common).params.block_bits;
        (*self_0).num_last_distances_to_check_ = (*common).params.num_last_distances_to_check;
    }
}

extern "C" fn InitializeH40(
    mut common: *mut HasherCommon,
    mut self_0: *mut H40,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let ref mut fresh7 = (*self_0).common;
        *fresh7 = common;
        let ref mut fresh8 = (*self_0).extra;
        *fresh8 = (*common).extra;
        (*self_0).max_hops =
            ((if (*params).quality > 6i32 { 7 } else { 8 }) << (*params).quality - 4) as u64;
    }
}

extern "C" fn InitializeH41(
    mut common: *mut HasherCommon,
    mut self_0: *mut H41,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let ref mut fresh9 = (*self_0).common;
        *fresh9 = common;
        let ref mut fresh10 = (*self_0).extra;
        *fresh10 = (*common).extra;
        (*self_0).max_hops =
            ((if (*params).quality > 6i32 { 7 } else { 8 }) << (*params).quality - 4) as u64;
    }
}

extern "C" fn InitializeH42(
    mut common: *mut HasherCommon,
    mut self_0: *mut H42,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let ref mut fresh11 = (*self_0).common;
        *fresh11 = common;
        let ref mut fresh12 = (*self_0).extra;
        *fresh12 = (*common).extra;
        (*self_0).max_hops =
            ((if (*params).quality > 6i32 { 7 } else { 8 }) << (*params).quality - 4) as u64;
    }
}

extern "C" fn InitializeH35(
    mut common: *mut HasherCommon,
    mut self_0: *mut H35,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let ref mut fresh13 = (*self_0).common;
        *fresh13 = common;
        let ref mut fresh14 = (*self_0).extra;
        *fresh14 = (*common).extra;
        (*self_0).hb_common = *(*self_0).common;
        (*self_0).fresh = 1;
        let ref mut fresh15 = (*self_0).params;
        *fresh15 = params;
    }
}

extern "C" fn InitializeH55(
    mut common: *mut HasherCommon,
    mut self_0: *mut H55,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let ref mut fresh16 = (*self_0).common;
        *fresh16 = common;
        let ref mut fresh17 = (*self_0).extra;
        *fresh17 = (*common).extra;
        (*self_0).hb_common = *(*self_0).common;
        (*self_0).fresh = 1;
        let ref mut fresh18 = (*self_0).params;
        *fresh18 = params;
    }
}

extern "C" fn InitializeH65(
    mut common: *mut HasherCommon,
    mut self_0: *mut H65,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let ref mut fresh19 = (*self_0).common;
        *fresh19 = common;
        let ref mut fresh20 = (*self_0).extra;
        *fresh20 = (*common).extra;
        (*self_0).hb_common = *(*self_0).common;
        (*self_0).fresh = 1;
        let ref mut fresh21 = (*self_0).params;
        *fresh21 = params;
    }
}

extern "C" fn InitializeH10(
    mut common: *mut HasherCommon,
    mut self_0: *mut H10,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let ref mut fresh22 = (*self_0).buckets_;
        *fresh22 = (*common).extra as *mut u32;
        let ref mut fresh23 = (*self_0).forest_;
        *fresh23 = &mut *((*self_0).buckets_).offset((1i32 << 17i32) as isize) as *mut u32;
        (*self_0).window_mask_ = (1u32 << (*params).lgwin).wrapping_sub(1) as u64;
        (*self_0).invalid_pos_ = 0u64.wrapping_sub((*self_0).window_mask_) as u32;
    }
}

extern "C" fn PrepareH2(
    mut self_0: *mut H2,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        let mut buckets = (*self_0).buckets_;
        let mut partial_prepare_threshold = (1i32 << 16 >> 5) as u64;
        if one_shot != 0 && input_size <= partial_prepare_threshold {
            let mut i: u64 = 0;
            i = 0;
            while i < input_size {
                let key = HashBytesH2(&*data.offset(i as isize));
                if 1 << 0 == 1 {
                    *buckets.offset(key as isize) = 0;
                } else {
                    let mut j: u32 = 0;
                    j = 0;
                    while j < (1i32 << 0) as u32 {
                        *buckets.offset(
                            (key.wrapping_add(j << 3i32) & ((1i32 << 16i32) - 1) as u32) as isize,
                        ) = 0;
                        j = j.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        } else {
            memset(
                buckets as *mut libc::c_void,
                0,
                (::std::mem::size_of::<u32>() as u64).wrapping_mul((1i32 << 16) as u64),
            );
        };
    }
}

extern "C" fn PrepareH4(
    mut self_0: *mut H4,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        let mut buckets = (*self_0).buckets_;
        let mut partial_prepare_threshold = (1i32 << 17 >> 5) as u64;
        if one_shot != 0 && input_size <= partial_prepare_threshold {
            let mut i: u64 = 0;
            i = 0;
            while i < input_size {
                let key = HashBytesH4(&*data.offset(i as isize));
                if 1 << 2 == 1 {
                    *buckets.offset(key as isize) = 0;
                } else {
                    let mut j: u32 = 0;
                    j = 0;
                    while j < (1i32 << 2) as u32 {
                        *buckets.offset(
                            (key.wrapping_add(j << 3i32) & ((1i32 << 17i32) - 1) as u32) as isize,
                        ) = 0;
                        j = j.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        } else {
            memset(
                buckets as *mut libc::c_void,
                0,
                (::std::mem::size_of::<u32>() as u64).wrapping_mul((1i32 << 17) as u64),
            );
        };
    }
}

extern "C" fn PrepareH5(
    mut self_0: *mut H5,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        let mut num = (*self_0).num_;
        let mut partial_prepare_threshold = (*self_0).bucket_size_ >> 6;
        if one_shot != 0 && input_size <= partial_prepare_threshold {
            let mut i: u64 = 0;
            i = 0;
            while i < input_size {
                let key = HashBytesH5(&*data.offset(i as isize), (*self_0).hash_shift_);
                *num.offset(key as isize) = 0;
                i = i.wrapping_add(1);
            }
        } else {
            memset(
                num as *mut libc::c_void,
                0,
                ((*self_0).bucket_size_).wrapping_mul(::std::mem::size_of::<u16>() as u64),
            );
        };
    }
}

extern "C" fn PrepareH40(
    mut self_0: *mut H40,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        let mut addr = AddrH40((*self_0).extra);
        let mut head = HeadH40((*self_0).extra);
        let mut tiny_hash = TinyHashH40((*self_0).extra);
        let mut partial_prepare_threshold = (1i32 << 15 >> 6) as u64;
        if one_shot != 0 && input_size <= partial_prepare_threshold {
            let mut i: u64 = 0;
            i = 0;
            while i < input_size {
                let mut bucket = HashBytesH40(&*data.offset(i as isize));
                *addr.offset(bucket as isize) = 0xcccccccc;
                *head.offset(bucket as isize) = 0xcccc;
                i = i.wrapping_add(1);
            }
        } else {
            memset(
                addr as *mut libc::c_void,
                0xcc,
                (::std::mem::size_of::<u32>() as u64).wrapping_mul((1i32 << 15) as u64),
            );
            memset(
                head as *mut libc::c_void,
                0,
                (::std::mem::size_of::<u16>() as u64).wrapping_mul((1i32 << 15) as u64),
            );
        }
        memset(
            tiny_hash as *mut libc::c_void,
            0,
            (::std::mem::size_of::<u8>() as u64).wrapping_mul(65536),
        );
        memset(
            ((*self_0).free_slot_idx).as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u16; 1]>() as u64,
        );
    }
}

extern "C" fn PrepareH41(
    mut self_0: *mut H41,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        let mut addr = AddrH41((*self_0).extra);
        let mut head = HeadH41((*self_0).extra);
        let mut tiny_hash = TinyHashH41((*self_0).extra);
        let mut partial_prepare_threshold = (1i32 << 15 >> 6) as u64;
        if one_shot != 0 && input_size <= partial_prepare_threshold {
            let mut i: u64 = 0;
            i = 0;
            while i < input_size {
                let mut bucket = HashBytesH41(&*data.offset(i as isize));
                *addr.offset(bucket as isize) = 0xcccccccc;
                *head.offset(bucket as isize) = 0xcccc;
                i = i.wrapping_add(1);
            }
        } else {
            memset(
                addr as *mut libc::c_void,
                0xcc,
                (::std::mem::size_of::<u32>() as u64).wrapping_mul((1i32 << 15) as u64),
            );
            memset(
                head as *mut libc::c_void,
                0,
                (::std::mem::size_of::<u16>() as u64).wrapping_mul((1i32 << 15) as u64),
            );
        }
        memset(
            tiny_hash as *mut libc::c_void,
            0,
            (::std::mem::size_of::<u8>() as u64).wrapping_mul(65536),
        );
        memset(
            ((*self_0).free_slot_idx).as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u16; 1]>() as u64,
        );
    }
}

extern "C" fn PrepareH42(
    mut self_0: *mut H42,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        let mut addr = AddrH42((*self_0).extra);
        let mut head = HeadH42((*self_0).extra);
        let mut tiny_hash = TinyHashH42((*self_0).extra);
        let mut partial_prepare_threshold = (1i32 << 15 >> 6) as u64;
        if one_shot != 0 && input_size <= partial_prepare_threshold {
            let mut i: u64 = 0;
            i = 0;
            while i < input_size {
                let mut bucket = HashBytesH42(&*data.offset(i as isize));
                *addr.offset(bucket as isize) = 0xcccccccc;
                *head.offset(bucket as isize) = 0xcccc;
                i = i.wrapping_add(1);
            }
        } else {
            memset(
                addr as *mut libc::c_void,
                0xcc,
                (::std::mem::size_of::<u32>() as u64).wrapping_mul((1i32 << 15) as u64),
            );
            memset(
                head as *mut libc::c_void,
                0,
                (::std::mem::size_of::<u16>() as u64).wrapping_mul((1i32 << 15) as u64),
            );
        }
        memset(
            tiny_hash as *mut libc::c_void,
            0,
            (::std::mem::size_of::<u8>() as u64).wrapping_mul(65536),
        );
        memset(
            ((*self_0).free_slot_idx).as_mut_ptr() as *mut libc::c_void,
            0,
            ::std::mem::size_of::<[u16; 512]>() as u64,
        );
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesH3(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        return (::std::mem::size_of::<u32>() as u64).wrapping_mul((1i32 << 16) as u64);
    }
}

extern "C" fn InitializeH3(
    mut common: *mut HasherCommon,
    mut self_0: *mut H3,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let ref mut fresh24 = (*self_0).common;
        *fresh24 = common;
        let ref mut fresh25 = (*self_0).buckets_;
        *fresh25 = (*common).extra as *mut u32;
    }
}

extern "C" fn PrepareH3(
    mut self_0: *mut H3,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        let mut buckets = (*self_0).buckets_;
        let mut partial_prepare_threshold = (1i32 << 16 >> 5) as u64;
        if one_shot != 0 && input_size <= partial_prepare_threshold {
            let mut i: u64 = 0;
            i = 0;
            while i < input_size {
                let key = HashBytesH3(&*data.offset(i as isize));
                if 1 << 1 == 1 {
                    *buckets.offset(key as isize) = 0;
                } else {
                    let mut j: u32 = 0;
                    j = 0;
                    while j < (1i32 << 1) as u32 {
                        *buckets.offset(
                            (key.wrapping_add(j << 3i32) & ((1i32 << 16i32) - 1) as u32) as isize,
                        ) = 0;
                        j = j.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        } else {
            memset(
                buckets as *mut libc::c_void,
                0,
                (::std::mem::size_of::<u32>() as u64).wrapping_mul((1i32 << 16) as u64),
            );
        };
    }
}

extern "C" fn PrepareH35(
    mut self_0: *mut H35,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        if (*self_0).fresh != 0 {
            (*self_0).fresh = 0;
            let ref mut fresh26 = (*self_0).hb_common.extra;
            *fresh26 = ((*self_0).extra as *mut u8).offset(HashMemAllocInBytesH3(
                (*self_0).params,
                one_shot,
                input_size,
            ) as isize) as *mut libc::c_void;
            InitializeH3((*self_0).common, &mut (*self_0).ha, (*self_0).params);
            InitializeHROLLING_FAST(
                &mut (*self_0).hb_common,
                &mut (*self_0).hb,
                (*self_0).params,
            );
        }
        PrepareH3(&mut (*self_0).ha, one_shot, input_size, data);
        PrepareHROLLING_FAST(&mut (*self_0).hb, one_shot, input_size, data);
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesH54(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        return (::std::mem::size_of::<u32>() as u64).wrapping_mul((1i32 << 20) as u64);
    }
}

extern "C" fn InitializeH54(
    mut common: *mut HasherCommon,
    mut self_0: *mut H54,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let ref mut fresh27 = (*self_0).common;
        *fresh27 = common;
        let ref mut fresh28 = (*self_0).buckets_;
        *fresh28 = (*common).extra as *mut u32;
    }
}

static mut kRollingHashMul32HROLLING_FAST: u32 = 69069;
static mut kInvalidPosHROLLING_FAST: u32 = 0xffffffff;
extern "C" fn InitializeHROLLING_FAST(
    mut common: *mut HasherCommon,
    mut self_0: *mut HROLLING_FAST,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let mut i: u64 = 0;
        (*self_0).state = 0;
        (*self_0).next_ix = 0;
        (*self_0).factor = kRollingHashMul32HROLLING_FAST;
        (*self_0).factor_remove = 1;
        i = 0;
        while i < 32 {
            let ref mut fresh29 = (*self_0).factor_remove;
            *fresh29 = (*fresh29 as u32).wrapping_mul((*self_0).factor) as u32;
            i = (i).wrapping_add(4) as u64;
        }
        let ref mut fresh30 = (*self_0).table;
        *fresh30 = (*common).extra as *mut u32;
        i = 0;
        while i < 16777216 {
            *((*self_0).table).offset(i as isize) = kInvalidPosHROLLING_FAST;
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn PrepareH54(
    mut self_0: *mut H54,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        let mut buckets = (*self_0).buckets_;
        let mut partial_prepare_threshold = (1i32 << 20 >> 5) as u64;
        if one_shot != 0 && input_size <= partial_prepare_threshold {
            let mut i: u64 = 0;
            i = 0;
            while i < input_size {
                let key = HashBytesH54(&*data.offset(i as isize));
                if 1 << 2 == 1 {
                    *buckets.offset(key as isize) = 0;
                } else {
                    let mut j: u32 = 0;
                    j = 0;
                    while j < (1i32 << 2) as u32 {
                        *buckets.offset(
                            (key.wrapping_add(j << 3i32) & ((1i32 << 20i32) - 1) as u32) as isize,
                        ) = 0;
                        j = j.wrapping_add(1);
                    }
                }
                i = i.wrapping_add(1);
            }
        } else {
            memset(
                buckets as *mut libc::c_void,
                0,
                (::std::mem::size_of::<u32>() as u64).wrapping_mul((1i32 << 20) as u64),
            );
        };
    }
}

extern "C" fn PrepareH55(
    mut self_0: *mut H55,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        if (*self_0).fresh != 0 {
            (*self_0).fresh = 0;
            let ref mut fresh31 = (*self_0).hb_common.extra;
            *fresh31 = ((*self_0).extra as *mut u8).offset(HashMemAllocInBytesH54(
                (*self_0).params,
                one_shot,
                input_size,
            ) as isize) as *mut libc::c_void;
            InitializeH54((*self_0).common, &mut (*self_0).ha, (*self_0).params);
            InitializeHROLLING_FAST(
                &mut (*self_0).hb_common,
                &mut (*self_0).hb,
                (*self_0).params,
            );
        }
        PrepareH54(&mut (*self_0).ha, one_shot, input_size, data);
        PrepareHROLLING_FAST(&mut (*self_0).hb, one_shot, input_size, data);
    }
}

#[inline(always)]
extern "C" fn HashMemAllocInBytesH6(
    mut params: *const BrotliEncoderParams,
    mut one_shot: i32,
    mut input_size: u64,
) -> u64 {
    unsafe {
        let mut bucket_size = 1 << (*params).hasher.bucket_bits;
        let mut block_size = 1 << (*params).hasher.block_bits;
        return (::std::mem::size_of::<u16>() as u64)
            .wrapping_mul(bucket_size)
            .wrapping_add(
                (::std::mem::size_of::<u32>() as u64)
                    .wrapping_mul(bucket_size)
                    .wrapping_mul(block_size),
            );
    }
}

extern "C" fn InitializeH6(
    mut common: *mut HasherCommon,
    mut self_0: *mut H6,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let ref mut fresh32 = (*self_0).common_;
        *fresh32 = common;
        (*self_0).hash_shift_ = 64 - (*common).params.bucket_bits;
        (*self_0).hash_mask_ = !0 >> 64 - 8 * (*common).params.hash_len;
        (*self_0).bucket_size_ = 1 << (*common).params.bucket_bits;
        (*self_0).block_bits_ = (*common).params.block_bits;
        (*self_0).block_size_ = 1 << (*common).params.block_bits;
        (*self_0).block_mask_ = ((*self_0).block_size_).wrapping_sub(1) as u32;
        (*self_0).num_last_distances_to_check_ = (*common).params.num_last_distances_to_check;
        let ref mut fresh33 = (*self_0).num_;
        *fresh33 = (*common).extra as *mut u16;
        let ref mut fresh34 = (*self_0).buckets_;
        *fresh34 =
            &mut *((*self_0).num_).offset((*self_0).bucket_size_ as isize) as *mut u16 as *mut u32;
    }
}

static mut kRollingHashMul32HROLLING: u32 = 69069;
static mut kInvalidPosHROLLING: u32 = 0xffffffff;
extern "C" fn InitializeHROLLING(
    mut common: *mut HasherCommon,
    mut self_0: *mut HROLLING,
    mut params: *const BrotliEncoderParams,
) {
    unsafe {
        let mut i: u64 = 0;
        (*self_0).state = 0;
        (*self_0).next_ix = 0;
        (*self_0).factor = kRollingHashMul32HROLLING;
        (*self_0).factor_remove = 1;
        i = 0;
        while i < 32 {
            let ref mut fresh35 = (*self_0).factor_remove;
            *fresh35 = (*fresh35 as u32).wrapping_mul((*self_0).factor) as u32;
            i = (i).wrapping_add(1) as u64;
        }
        let ref mut fresh36 = (*self_0).table;
        *fresh36 = (*common).extra as *mut u32;
        i = 0;
        while i < 16777216 {
            *((*self_0).table).offset(i as isize) = kInvalidPosHROLLING;
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn PrepareH6(
    mut self_0: *mut H6,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        let mut num = (*self_0).num_;
        let mut partial_prepare_threshold = (*self_0).bucket_size_ >> 6;
        if one_shot != 0 && input_size <= partial_prepare_threshold {
            let mut i: u64 = 0;
            i = 0;
            while i < input_size {
                let key = HashBytesH6(
                    &*data.offset(i as isize),
                    (*self_0).hash_mask_,
                    (*self_0).hash_shift_,
                );
                *num.offset(key as isize) = 0;
                i = i.wrapping_add(1);
            }
        } else {
            memset(
                num as *mut libc::c_void,
                0,
                ((*self_0).bucket_size_).wrapping_mul(::std::mem::size_of::<u16>() as u64),
            );
        };
    }
}

extern "C" fn PrepareH65(
    mut self_0: *mut H65,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        if (*self_0).fresh != 0 {
            (*self_0).fresh = 0;
            let ref mut fresh37 = (*self_0).hb_common.extra;
            *fresh37 = ((*self_0).extra as *mut u8).offset(HashMemAllocInBytesH6(
                (*self_0).params,
                one_shot,
                input_size,
            ) as isize) as *mut libc::c_void;
            InitializeH6((*self_0).common, &mut (*self_0).ha, (*self_0).params);
            InitializeHROLLING(
                &mut (*self_0).hb_common,
                &mut (*self_0).hb,
                (*self_0).params,
            );
        }
        PrepareH6(&mut (*self_0).ha, one_shot, input_size, data);
        PrepareHROLLING(&mut (*self_0).hb, one_shot, input_size, data);
    }
}

extern "C" fn PrepareH10(
    mut self_0: *mut H10,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        let mut invalid_pos = (*self_0).invalid_pos_;
        let mut i: u32 = 0;
        let mut buckets = (*self_0).buckets_;
        i = 0;
        while i < (1i32 << 17) as u32 {
            *buckets.offset(i as isize) = invalid_pos;
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn HasherSetup(
    mut m: *mut MemoryManager,
    mut hasher: *mut Hasher,
    mut params: *mut BrotliEncoderParams,
    mut data: *const u8,
    mut position: u64,
    mut input_size: u64,
    mut is_last: i32,
) {
    unsafe {
        let mut one_shot = (position == 0 && is_last != 0) as i32;
        if ((*hasher).common.extra).is_null() {
            let mut alloc_size: u64 = 0;
            ChooseHasher(params, &mut (*params).hasher);
            alloc_size = HasherSize(params, one_shot, input_size);
            let ref mut fresh38 = (*hasher).common.extra;
            *fresh38 = (if alloc_size > 0u64 {
                BrotliAllocate(
                    m,
                    alloc_size.wrapping_mul(::std::mem::size_of::<u8>() as u64),
                ) as *mut u8
            } else {
                0 as *mut u8
            }) as *mut libc::c_void;
            if 0 != 0 || 0 != 0 {
                return;
            };
            (*hasher).common.params = (*params).hasher;
            match (*hasher).common.params.type_0 {
                2 => {
                    InitializeH2(&mut (*hasher).common, &mut (*hasher).privat._H2, params);
                }
                3 => {
                    InitializeH3(&mut (*hasher).common, &mut (*hasher).privat._H3, params);
                }
                4 => {
                    InitializeH4(&mut (*hasher).common, &mut (*hasher).privat._H4, params);
                }
                5 => {
                    InitializeH5(&mut (*hasher).common, &mut (*hasher).privat._H5, params);
                }
                6 => {
                    InitializeH6(&mut (*hasher).common, &mut (*hasher).privat._H6, params);
                }
                40 => {
                    InitializeH40(&mut (*hasher).common, &mut (*hasher).privat._H40, params);
                }
                41 => {
                    InitializeH41(&mut (*hasher).common, &mut (*hasher).privat._H41, params);
                }
                42 => {
                    InitializeH42(&mut (*hasher).common, &mut (*hasher).privat._H42, params);
                }
                54 => {
                    InitializeH54(&mut (*hasher).common, &mut (*hasher).privat._H54, params);
                }
                35 => {
                    InitializeH35(&mut (*hasher).common, &mut (*hasher).privat._H35, params);
                }
                55 => {
                    InitializeH55(&mut (*hasher).common, &mut (*hasher).privat._H55, params);
                }
                65 => {
                    InitializeH65(&mut (*hasher).common, &mut (*hasher).privat._H65, params);
                }
                10 => {
                    InitializeH10(&mut (*hasher).common, &mut (*hasher).privat._H10, params);
                }
                _ => {}
            }
            HasherReset(hasher);
        }
        if (*hasher).common.is_prepared_ == 0 {
            match (*hasher).common.params.type_0 {
                2 => {
                    PrepareH2(&mut (*hasher).privat._H2, one_shot, input_size, data);
                }
                3 => {
                    PrepareH3(&mut (*hasher).privat._H3, one_shot, input_size, data);
                }
                4 => {
                    PrepareH4(&mut (*hasher).privat._H4, one_shot, input_size, data);
                }
                5 => {
                    PrepareH5(&mut (*hasher).privat._H5, one_shot, input_size, data);
                }
                6 => {
                    PrepareH6(&mut (*hasher).privat._H6, one_shot, input_size, data);
                }
                40 => {
                    PrepareH40(&mut (*hasher).privat._H40, one_shot, input_size, data);
                }
                41 => {
                    PrepareH41(&mut (*hasher).privat._H41, one_shot, input_size, data);
                }
                42 => {
                    PrepareH42(&mut (*hasher).privat._H42, one_shot, input_size, data);
                }
                54 => {
                    PrepareH54(&mut (*hasher).privat._H54, one_shot, input_size, data);
                }
                35 => {
                    PrepareH35(&mut (*hasher).privat._H35, one_shot, input_size, data);
                }
                55 => {
                    PrepareH55(&mut (*hasher).privat._H55, one_shot, input_size, data);
                }
                65 => {
                    PrepareH65(&mut (*hasher).privat._H65, one_shot, input_size, data);
                }
                10 => {
                    PrepareH10(&mut (*hasher).privat._H10, one_shot, input_size, data);
                }
                _ => {}
            }
            if position == 0 {
                (*hasher).common.dict_num_lookups = 0;
                (*hasher).common.dict_num_matches = 0;
            };
            (*hasher).common.is_prepared_ = 1;
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH2() -> u64 {
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
extern "C" fn StitchToPreviousBlockH2(
    mut self_0: *mut H2,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
) {
    unsafe {
        if num_bytes >= (HashTypeLengthH2()).wrapping_sub(1) && position >= 3 {
            StoreH2(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(3),
            );
            StoreH2(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(2),
            );
            StoreH2(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(1),
            );
        }
    }
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
extern "C" fn StitchToPreviousBlockH4(
    mut self_0: *mut H4,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
) {
    unsafe {
        if num_bytes >= (HashTypeLengthH4()).wrapping_sub(1) && position >= 3 {
            StoreH4(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(3),
            );
            StoreH4(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(2),
            );
            StoreH4(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(1),
            );
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH5() -> u64 {
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
        let ref mut fresh39 = *((*self_0).num_).offset(key as isize);
        *fresh39 = (*fresh39).wrapping_add(1);
    }
}

#[inline(always)]
extern "C" fn StitchToPreviousBlockH5(
    mut self_0: *mut H5,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
) {
    unsafe {
        if num_bytes >= (HashTypeLengthH5()).wrapping_sub(1) && position >= 3 {
            StoreH5(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(3),
            );
            StoreH5(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(2),
            );
            StoreH5(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(1),
            );
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH40() -> u64 {
    return 4;
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

extern "C" fn TinyHashH42(mut extra: *mut libc::c_void) -> *mut u8 {
    unsafe {
        return &mut *((HeadH42 as unsafe extern "C" fn(*mut libc::c_void) -> *mut u16)(extra))
            .offset((1i32 << 15i32) as isize) as *mut u16 as *mut u8;
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
        let ref mut fresh40 = (*self_0).free_slot_idx[bank as usize];
        let fresh41 = *fresh40;
        *fresh40 = (*fresh40).wrapping_add(1);
        let idx = (fresh41 as i32 & (1i32 << 16) - 1) as u64;
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
extern "C" fn StitchToPreviousBlockH40(
    mut self_0: *mut H40,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ring_buffer_mask: u64,
) {
    unsafe {
        if num_bytes >= (HashTypeLengthH40()).wrapping_sub(1) && position >= 3 {
            StoreH40(
                self_0,
                ringbuffer,
                ring_buffer_mask,
                position.wrapping_sub(3),
            );
            StoreH40(
                self_0,
                ringbuffer,
                ring_buffer_mask,
                position.wrapping_sub(2),
            );
            StoreH40(
                self_0,
                ringbuffer,
                ring_buffer_mask,
                position.wrapping_sub(1),
            );
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH41() -> u64 {
    return 4;
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

#[inline(always)]
extern "C" fn HashBytesH41(mut data: *const u8) -> u64 {
    unsafe {
        let h = (BrotliUnalignedRead32(data as *const libc::c_void)).wrapping_mul(kHashMul32);
        return (h >> 32 - 15i32) as u64;
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
        let ref mut fresh42 = (*self_0).free_slot_idx[bank as usize];
        let fresh43 = *fresh42;
        *fresh42 = (*fresh42).wrapping_add(1);
        let idx = (fresh43 as i32 & (1i32 << 16) - 1) as u64;
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
extern "C" fn StitchToPreviousBlockH41(
    mut self_0: *mut H41,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ring_buffer_mask: u64,
) {
    unsafe {
        if num_bytes >= (HashTypeLengthH41()).wrapping_sub(1) && position >= 3 {
            StoreH41(
                self_0,
                ringbuffer,
                ring_buffer_mask,
                position.wrapping_sub(3),
            );
            StoreH41(
                self_0,
                ringbuffer,
                ring_buffer_mask,
                position.wrapping_sub(2),
            );
            StoreH41(
                self_0,
                ringbuffer,
                ring_buffer_mask,
                position.wrapping_sub(1),
            );
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH42() -> u64 {
    return 4;
}

extern "C" fn BanksH42(mut extra: *mut libc::c_void) -> *mut BankH42 {
    unsafe {
        return &mut *((TinyHashH42 as unsafe extern "C" fn(*mut libc::c_void) -> *mut u8)(extra))
            .offset(65536 as isize) as *mut u8 as *mut BankH42;
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

#[inline(always)]
extern "C" fn HashBytesH42(mut data: *const u8) -> u64 {
    unsafe {
        let h = (BrotliUnalignedRead32(data as *const libc::c_void)).wrapping_mul(kHashMul32);
        return (h >> 32 - 15i32) as u64;
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
        let ref mut fresh44 = (*self_0).free_slot_idx[bank as usize];
        let fresh45 = *fresh44;
        *fresh44 = (*fresh44).wrapping_add(1);
        let idx = (fresh45 as i32 & (1i32 << 9) - 1) as u64;
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
extern "C" fn StitchToPreviousBlockH42(
    mut self_0: *mut H42,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ring_buffer_mask: u64,
) {
    unsafe {
        if num_bytes >= (HashTypeLengthH42()).wrapping_sub(1) && position >= 3 {
            StoreH42(
                self_0,
                ringbuffer,
                ring_buffer_mask,
                position.wrapping_sub(3),
            );
            StoreH42(
                self_0,
                ringbuffer,
                ring_buffer_mask,
                position.wrapping_sub(2),
            );
            StoreH42(
                self_0,
                ringbuffer,
                ring_buffer_mask,
                position.wrapping_sub(1),
            );
        }
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH3() -> u64 {
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
extern "C" fn StitchToPreviousBlockH3(
    mut self_0: *mut H3,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
) {
    unsafe {
        if num_bytes >= (HashTypeLengthH3()).wrapping_sub(1) && position >= 3 {
            StoreH3(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(3),
            );
            StoreH3(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(2),
            );
            StoreH3(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(1),
            );
        }
    }
}

#[inline(always)]
extern "C" fn StitchToPreviousBlockH35(
    mut self_0: *mut H35,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ring_buffer_mask: u64,
) {
    unsafe {
        StitchToPreviousBlockH3(
            &mut (*self_0).ha,
            num_bytes,
            position,
            ringbuffer,
            ring_buffer_mask,
        );
        StitchToPreviousBlockHROLLING_FAST(
            &mut (*self_0).hb,
            num_bytes,
            position,
            ringbuffer,
            ring_buffer_mask,
        );
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH54() -> u64 {
    return 8;
}

static mut kHashMul64: u64 = 0x1e35a7bd << 32 | 0x1e35a7bd;
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
extern "C" fn StitchToPreviousBlockH54(
    mut self_0: *mut H54,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
) {
    unsafe {
        if num_bytes >= (HashTypeLengthH54()).wrapping_sub(1) && position >= 3 {
            StoreH54(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(3),
            );
            StoreH54(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(2),
            );
            StoreH54(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(1),
            );
        }
    }
}

extern "C" fn HashByteHROLLING_FAST(mut byte: u8) -> u32 {
    return (byte as u32).wrapping_add(1);
}

extern "C" fn HashRollingFunctionInitialHROLLING_FAST(
    mut state: u32,
    mut add: u8,
    mut factor: u32,
) -> u32 {
    return factor
        .wrapping_mul(state)
        .wrapping_add(HashByteHROLLING_FAST(add));
}

extern "C" fn PrepareHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        let mut i: u64 = 0;
        if input_size < 32 {
            return;
        };
        (*self_0).state = 0;
        i = 0;
        while i < 32 {
            (*self_0).state = HashRollingFunctionInitialHROLLING_FAST(
                (*self_0).state,
                *data.offset(i as isize),
                (*self_0).factor,
            );
            i = (i).wrapping_add(4) as u64;
        }
    }
}

#[inline(always)]
extern "C" fn StitchToPreviousBlockHROLLING_FAST(
    mut self_0: *mut HROLLING_FAST,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ring_buffer_mask: u64,
) {
    unsafe {
        let mut position_masked: u64 = 0;
        let mut available = num_bytes;
        if position & (4 - 1i32) as u64 != 0 {
            let mut diff = 4u64.wrapping_sub(position & (4 - 1i32) as u64);
            available = if diff > available {
                0
            } else {
                available.wrapping_sub(diff)
            };
            position = (position as u64).wrapping_add(diff) as u64;
        }
        position_masked = position & ring_buffer_mask;
        if available > ring_buffer_mask.wrapping_sub(position_masked) {
            available = ring_buffer_mask.wrapping_sub(position_masked);
        }
        PrepareHROLLING_FAST(
            self_0,
            0,
            available,
            ringbuffer.offset((position & ring_buffer_mask) as isize),
        );
        (*self_0).next_ix = position;
    }
}

#[inline(always)]
extern "C" fn StitchToPreviousBlockH55(
    mut self_0: *mut H55,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ring_buffer_mask: u64,
) {
    unsafe {
        StitchToPreviousBlockH54(
            &mut (*self_0).ha,
            num_bytes,
            position,
            ringbuffer,
            ring_buffer_mask,
        );
        StitchToPreviousBlockHROLLING_FAST(
            &mut (*self_0).hb,
            num_bytes,
            position,
            ringbuffer,
            ring_buffer_mask,
        );
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH6() -> u64 {
    return 8;
}

static mut kHashMul64Long: u64 = 0x1fe35a7b << 32 | 0xd3579bd3;
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
        let ref mut fresh46 = *num.offset(key as isize);
        *fresh46 = (*fresh46).wrapping_add(1);
        *buckets.offset(offset as isize) = ix as u32;
    }
}

#[inline(always)]
extern "C" fn StitchToPreviousBlockH6(
    mut self_0: *mut H6,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
) {
    unsafe {
        if num_bytes >= (HashTypeLengthH6()).wrapping_sub(1) && position >= 3 {
            StoreH6(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(3),
            );
            StoreH6(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(2),
            );
            StoreH6(
                self_0,
                ringbuffer,
                ringbuffer_mask,
                position.wrapping_sub(1),
            );
        }
    }
}

extern "C" fn HashByteHROLLING(mut byte: u8) -> u32 {
    return (byte as u32).wrapping_add(1);
}

extern "C" fn HashRollingFunctionInitialHROLLING(
    mut state: u32,
    mut add: u8,
    mut factor: u32,
) -> u32 {
    return factor
        .wrapping_mul(state)
        .wrapping_add(HashByteHROLLING(add));
}

extern "C" fn PrepareHROLLING(
    mut self_0: *mut HROLLING,
    mut one_shot: i32,
    mut input_size: u64,
    mut data: *const u8,
) {
    unsafe {
        let mut i: u64 = 0;
        if input_size < 32 {
            return;
        };
        (*self_0).state = 0;
        i = 0;
        while i < 32 {
            (*self_0).state = HashRollingFunctionInitialHROLLING(
                (*self_0).state,
                *data.offset(i as isize),
                (*self_0).factor,
            );
            i = (i).wrapping_add(1) as u64;
        }
    }
}

#[inline(always)]
extern "C" fn StitchToPreviousBlockHROLLING(
    mut self_0: *mut HROLLING,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ring_buffer_mask: u64,
) {
    unsafe {
        let mut position_masked: u64 = 0;
        let mut available = num_bytes;
        if position & (1 - 1i32) as u64 != 0 {
            let mut diff = 1u64.wrapping_sub(position & (1 - 1i32) as u64);
            available = if diff > available {
                0
            } else {
                available.wrapping_sub(diff)
            };
            position = (position as u64).wrapping_add(diff) as u64;
        }
        position_masked = position & ring_buffer_mask;
        if available > ring_buffer_mask.wrapping_sub(position_masked) {
            available = ring_buffer_mask.wrapping_sub(position_masked);
        }
        PrepareHROLLING(
            self_0,
            0,
            available,
            ringbuffer.offset((position & ring_buffer_mask) as isize),
        );
        (*self_0).next_ix = position;
    }
}

#[inline(always)]
extern "C" fn StitchToPreviousBlockH65(
    mut self_0: *mut H65,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ring_buffer_mask: u64,
) {
    unsafe {
        StitchToPreviousBlockH6(
            &mut (*self_0).ha,
            num_bytes,
            position,
            ringbuffer,
            ring_buffer_mask,
        );
        StitchToPreviousBlockHROLLING(
            &mut (*self_0).hb,
            num_bytes,
            position,
            ringbuffer,
            ring_buffer_mask,
        );
    }
}

#[inline(always)]
extern "C" fn HashTypeLengthH10() -> u64 {
    return 4;
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
extern "C" fn InitBackwardMatch(mut self_0: *mut BackwardMatch, mut dist: u64, mut len: u64) {
    unsafe {
        (*self_0).distance = dist as u32;
        (*self_0).length_and_code = (len << 5i32) as u32;
    }
}

#[inline(always)]
extern "C" fn LeftChildIndexH10(mut self_0: *mut H10, pos: u64) -> u64 {
    unsafe {
        return 2u64.wrapping_mul(pos & (*self_0).window_mask_);
    }
}

static mut kHashMul32: u32 = 0x1e35a7bd;
extern "C" fn HashBytesH10(mut data: *const u8) -> u32 {
    unsafe {
        let mut h = (BrotliUnalignedRead32(data as *const libc::c_void)).wrapping_mul(kHashMul32);
        return h >> 32 - 17;
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
                    let fresh47 = matches;
                    matches = matches.offset(1);
                    InitBackwardMatch(fresh47, backward, len);
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
extern "C" fn StitchToPreviousBlockH10(
    mut self_0: *mut H10,
    mut num_bytes: u64,
    mut position: u64,
    mut ringbuffer: *const u8,
    mut ringbuffer_mask: u64,
) {
    unsafe {
        if num_bytes >= (HashTypeLengthH10()).wrapping_sub(1) && position >= 128 {
            let i_start = position.wrapping_sub(128).wrapping_add(1);
            let i_end = brotli_min_size_t(position, i_start.wrapping_add(num_bytes));
            let mut i: u64 = 0;
            i = i_start;
            while i < i_end {
                let max_backward = ((*self_0).window_mask_).wrapping_sub(brotli_max_size_t(
                    (16 - 1i32) as u64,
                    position.wrapping_sub(i),
                ));
                StoreAndFindMatchesH10(
                    self_0,
                    ringbuffer,
                    i,
                    ringbuffer_mask,
                    128,
                    max_backward,
                    0 as *mut u64,
                    0 as *mut BackwardMatch,
                );
                i = i.wrapping_add(1);
            }
        }
    }
}

#[inline(always)]
extern "C" fn InitOrStitchToPreviousBlock(
    mut m: *mut MemoryManager,
    mut hasher: *mut Hasher,
    mut data: *const u8,
    mut mask: u64,
    mut params: *mut BrotliEncoderParams,
    mut position: u64,
    mut input_size: u64,
    mut is_last: i32,
) {
    unsafe {
        HasherSetup(m, hasher, params, data, position, input_size, is_last);
        if 0 != 0 {
            return;
        }
        match (*hasher).common.params.type_0 {
            2 => {
                StitchToPreviousBlockH2(
                    &mut (*hasher).privat._H2,
                    input_size,
                    position,
                    data,
                    mask,
                );
            }
            3 => {
                StitchToPreviousBlockH3(
                    &mut (*hasher).privat._H3,
                    input_size,
                    position,
                    data,
                    mask,
                );
            }
            4 => {
                StitchToPreviousBlockH4(
                    &mut (*hasher).privat._H4,
                    input_size,
                    position,
                    data,
                    mask,
                );
            }
            5 => {
                StitchToPreviousBlockH5(
                    &mut (*hasher).privat._H5,
                    input_size,
                    position,
                    data,
                    mask,
                );
            }
            6 => {
                StitchToPreviousBlockH6(
                    &mut (*hasher).privat._H6,
                    input_size,
                    position,
                    data,
                    mask,
                );
            }
            40 => {
                StitchToPreviousBlockH40(
                    &mut (*hasher).privat._H40,
                    input_size,
                    position,
                    data,
                    mask,
                );
            }
            41 => {
                StitchToPreviousBlockH41(
                    &mut (*hasher).privat._H41,
                    input_size,
                    position,
                    data,
                    mask,
                );
            }
            42 => {
                StitchToPreviousBlockH42(
                    &mut (*hasher).privat._H42,
                    input_size,
                    position,
                    data,
                    mask,
                );
            }
            54 => {
                StitchToPreviousBlockH54(
                    &mut (*hasher).privat._H54,
                    input_size,
                    position,
                    data,
                    mask,
                );
            }
            35 => {
                StitchToPreviousBlockH35(
                    &mut (*hasher).privat._H35,
                    input_size,
                    position,
                    data,
                    mask,
                );
            }
            55 => {
                StitchToPreviousBlockH55(
                    &mut (*hasher).privat._H55,
                    input_size,
                    position,
                    data,
                    mask,
                );
            }
            65 => {
                StitchToPreviousBlockH65(
                    &mut (*hasher).privat._H65,
                    input_size,
                    position,
                    data,
                    mask,
                );
            }
            10 => {
                StitchToPreviousBlockH10(
                    &mut (*hasher).privat._H10,
                    input_size,
                    position,
                    data,
                    mask,
                );
            }
            _ => {}
        };
    }
}

#[inline(always)]
extern "C" fn CommandRestoreDistanceCode(
    mut self_0: *const Command,
    mut dist: *const BrotliDistanceParams,
) -> u32 {
    unsafe {
        if ((*self_0).dist_prefix_ as u32 & 0x3ffu32)
            < 16u32.wrapping_add((*dist).num_direct_distance_codes)
        {
            return (*self_0).dist_prefix_ as u32 & 0x3ff;
        } else {
            let mut dcode = (*self_0).dist_prefix_ as u32 & 0x3ff;
            let mut nbits = ((*self_0).dist_prefix_ as i32 >> 10i32) as u32;
            let mut extra = (*self_0).dist_extra_;
            let mut postfix_mask = (1u32 << (*dist).distance_postfix_bits).wrapping_sub(1);
            let mut hcode = dcode
                .wrapping_sub((*dist).num_direct_distance_codes)
                .wrapping_sub(16)
                >> (*dist).distance_postfix_bits;
            let mut lcode = dcode
                .wrapping_sub((*dist).num_direct_distance_codes)
                .wrapping_sub(16)
                & postfix_mask;
            let mut offset = (2u32.wrapping_add(hcode & 1) << nbits).wrapping_sub(4);
            return (offset.wrapping_add(extra) << (*dist).distance_postfix_bits)
                .wrapping_add(lcode)
                .wrapping_add((*dist).num_direct_distance_codes)
                .wrapping_add(16);
        };
    }
}

#[inline(always)]
extern "C" fn ComputeRbBits(mut params: *const BrotliEncoderParams) -> i32 {
    unsafe {
        return 1 + brotli_max_int((*params).lgwin, (*params).lgblock);
    }
}

#[inline(always)]
extern "C" fn MaxMetablockSize(mut params: *const BrotliEncoderParams) -> u64 {
    unsafe {
        let mut bits = brotli_min_int(ComputeRbBits(params), 24);
        return 1 << bits;
    }
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
extern "C" fn Log2FloorNonZero(mut n: u64) -> u32 {
    return 31 ^ (n as u32).leading_zeros() as u32;
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
extern "C" fn InitInsertCommand(mut self_0: *mut Command, mut insertlen: u64) {
    unsafe {
        (*self_0).insert_len_ = insertlen as u32;
        (*self_0).copy_len_ = (4i32 << 25) as u32;
        (*self_0).dist_extra_ = 0;
        (*self_0).dist_prefix_ = 16;
        GetLengthCode(insertlen, 4, 0, &mut (*self_0).cmd_prefix_);
    }
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
extern "C" fn HasherReset(mut hasher: *mut Hasher) {
    unsafe {
        (*hasher).common.is_prepared_ = 0;
    }
}

#[inline(always)]
extern "C" fn DestroyHasher(mut m: *mut MemoryManager, mut hasher: *mut Hasher) {
    unsafe {
        if ((*hasher).common.extra).is_null() {
            return;
        }
        BrotliFree(m, (*hasher).common.extra);
        let ref mut fresh48 = (*hasher).common.extra;
        *fresh48 = 0 as *mut libc::c_void;
    }
}

#[inline(always)]
extern "C" fn HasherInit(mut hasher: *mut Hasher) {
    unsafe {
        let ref mut fresh49 = (*hasher).common.extra;
        *fresh49 = 0 as *mut libc::c_void;
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
            current_block = 7572130911416793741;
        } else {
            current_block = 715039052867723359;
        }
        loop {
            match current_block {
                715039052867723359 => {
                    if !(population < population_end) {
                        break;
                    }
                    let fresh50 = population;
                    population = population.offset(1);
                    p = *fresh50 as u64;
                    sum = (sum as u64).wrapping_add(p) as u64;
                    retval -= p as f64 * FastLog2(p);
                    current_block = 7572130911416793741;
                }
                _ => {
                    let fresh51 = population;
                    population = population.offset(1);
                    p = *fresh51 as u64;
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
extern "C" fn InitMetaBlockSplit(mut mb: *mut MetaBlockSplit) {
    unsafe {
        BrotliInitBlockSplit(&mut (*mb).literal_split);
        BrotliInitBlockSplit(&mut (*mb).command_split);
        BrotliInitBlockSplit(&mut (*mb).distance_split);
        let ref mut fresh52 = (*mb).literal_context_map;
        *fresh52 = 0 as *mut u32;
        (*mb).literal_context_map_size = 0;
        let ref mut fresh53 = (*mb).distance_context_map;
        *fresh53 = 0 as *mut u32;
        (*mb).distance_context_map_size = 0;
        let ref mut fresh54 = (*mb).literal_histograms;
        *fresh54 = 0 as *mut HistogramLiteral;
        (*mb).literal_histograms_size = 0;
        let ref mut fresh55 = (*mb).command_histograms;
        *fresh55 = 0 as *mut HistogramCommand;
        (*mb).command_histograms_size = 0;
        let ref mut fresh56 = (*mb).distance_histograms;
        *fresh56 = 0 as *mut HistogramDistance;
        (*mb).distance_histograms_size = 0;
    }
}

#[inline(always)]
extern "C" fn DestroyMetaBlockSplit(mut m: *mut MemoryManager, mut mb: *mut MetaBlockSplit) {
    unsafe {
        BrotliDestroyBlockSplit(m, &mut (*mb).literal_split);
        BrotliDestroyBlockSplit(m, &mut (*mb).command_split);
        BrotliDestroyBlockSplit(m, &mut (*mb).distance_split);
        BrotliFree(m, (*mb).literal_context_map as *mut libc::c_void);
        let ref mut fresh57 = (*mb).literal_context_map;
        *fresh57 = 0 as *mut u32;
        BrotliFree(m, (*mb).distance_context_map as *mut libc::c_void);
        let ref mut fresh58 = (*mb).distance_context_map;
        *fresh58 = 0 as *mut u32;
        BrotliFree(m, (*mb).literal_histograms as *mut libc::c_void);
        let ref mut fresh59 = (*mb).literal_histograms;
        *fresh59 = 0 as *mut HistogramLiteral;
        BrotliFree(m, (*mb).command_histograms as *mut libc::c_void);
        let ref mut fresh60 = (*mb).command_histograms;
        *fresh60 = 0 as *mut HistogramCommand;
        BrotliFree(m, (*mb).distance_histograms as *mut libc::c_void);
        let ref mut fresh61 = (*mb).distance_histograms;
        *fresh61 = 0 as *mut HistogramDistance;
    }
}

static mut kCompressFragmentTwoPassBlockSize: u64 = (1i32 << 17) as u64;
#[inline(always)]
extern "C" fn RingBufferInit(mut rb: *mut RingBuffer) {
    unsafe {
        (*rb).cur_size_ = 0;
        (*rb).pos_ = 0;
        let ref mut fresh62 = (*rb).data_;
        *fresh62 = 0 as *mut u8;
        let ref mut fresh63 = (*rb).buffer_;
        *fresh63 = 0 as *mut u8;
    }
}

#[inline(always)]
extern "C" fn RingBufferSetup(mut params: *const BrotliEncoderParams, mut rb: *mut RingBuffer) {
    unsafe {
        let mut window_bits = ComputeRbBits(params);
        let mut tail_bits = (*params).lgblock;
        *(&(*rb).size_ as *const u32 as *mut u32) = 1 << window_bits;
        *(&(*rb).mask_ as *const u32 as *mut u32) = (1u32 << window_bits).wrapping_sub(1);
        *(&(*rb).tail_size_ as *const u32 as *mut u32) = 1 << tail_bits;
        *(&(*rb).total_size_ as *const u32 as *mut u32) =
            ((*rb).size_).wrapping_add((*rb).tail_size_);
    }
}

#[inline(always)]
extern "C" fn RingBufferFree(mut m: *mut MemoryManager, mut rb: *mut RingBuffer) {
    unsafe {
        BrotliFree(m, (*rb).data_ as *mut libc::c_void);
        let ref mut fresh64 = (*rb).data_;
        *fresh64 = 0 as *mut u8;
    }
}

#[inline(always)]
extern "C" fn RingBufferInitBuffer(
    mut m: *mut MemoryManager,
    buflen: u32,
    mut rb: *mut RingBuffer,
) {
    unsafe {
        static mut kSlackForEightByteHashingEverywhere: u64 = 7;
        let mut new_data = if (2u32.wrapping_add(buflen) as u64)
            .wrapping_add(kSlackForEightByteHashingEverywhere)
            > 0
        {
            BrotliAllocate(
                m,
                (2u32.wrapping_add(buflen) as u64)
                    .wrapping_add(kSlackForEightByteHashingEverywhere)
                    .wrapping_mul(::std::mem::size_of::<u8>() as u64),
            ) as *mut u8
        } else {
            0 as *mut u8
        };
        let mut i: u64 = 0;
        if 0 != 0 || 0 != 0 {
            return;
        }
        if !((*rb).data_).is_null() {
            memcpy(
                new_data as *mut libc::c_void,
                (*rb).data_ as *const libc::c_void,
                (2u32.wrapping_add((*rb).cur_size_) as u64)
                    .wrapping_add(kSlackForEightByteHashingEverywhere),
            );
            BrotliFree(m, (*rb).data_ as *mut libc::c_void);
            let ref mut fresh65 = (*rb).data_;
            *fresh65 = 0 as *mut u8;
        }
        let ref mut fresh66 = (*rb).data_;
        *fresh66 = new_data;
        (*rb).cur_size_ = buflen;
        let ref mut fresh67 = (*rb).buffer_;
        *fresh67 = ((*rb).data_).offset(2 as isize);
        let ref mut fresh68 = *((*rb).buffer_).offset(-1i32 as isize);
        *fresh68 = 0;
        *((*rb).buffer_).offset(-2i32 as isize) = *fresh68;
        i = 0;
        while i < kSlackForEightByteHashingEverywhere {
            *((*rb).buffer_).offset(((*rb).cur_size_ as u64).wrapping_add(i) as isize) = 0;
            i = i.wrapping_add(1);
        }
    }
}

#[inline(always)]
extern "C" fn RingBufferWriteTail(mut bytes: *const u8, mut n: u64, mut rb: *mut RingBuffer) {
    unsafe {
        let masked_pos = ((*rb).pos_ & (*rb).mask_) as u64;
        if (masked_pos < (*rb).tail_size_ as u64) as i64 != 0 {
            let p = ((*rb).size_ as u64).wrapping_add(masked_pos);
            memcpy(
                &mut *((*rb).buffer_).offset(p as isize) as *mut u8 as *mut libc::c_void,
                bytes as *const libc::c_void,
                brotli_min_size_t(n, ((*rb).tail_size_ as u64).wrapping_sub(masked_pos)),
            );
        }
    }
}

#[inline(always)]
extern "C" fn RingBufferWrite(
    mut m: *mut MemoryManager,
    mut bytes: *const u8,
    mut n: u64,
    mut rb: *mut RingBuffer,
) {
    unsafe {
        if (*rb).pos_ == 0 && n < (*rb).tail_size_ as u64 {
            (*rb).pos_ = n as u32;
            RingBufferInitBuffer(m, (*rb).pos_, rb);
            if 0 != 0 {
                return;
            }
            memcpy(
                (*rb).buffer_ as *mut libc::c_void,
                bytes as *const libc::c_void,
                n,
            );
            return;
        }
        if (*rb).cur_size_ < (*rb).total_size_ {
            RingBufferInitBuffer(m, (*rb).total_size_, rb);
            if 0 != 0 {
                return;
            };
            *((*rb).buffer_).offset(((*rb).size_).wrapping_sub(2) as isize) = 0;
            *((*rb).buffer_).offset(((*rb).size_).wrapping_sub(1) as isize) = 0;
            *((*rb).buffer_).offset((*rb).size_ as isize) = 241;
        }
        let masked_pos = ((*rb).pos_ & (*rb).mask_) as u64;
        RingBufferWriteTail(bytes, n, rb);
        if (masked_pos.wrapping_add(n) <= (*rb).size_ as u64) as i64 != 0 {
            memcpy(
                &mut *((*rb).buffer_).offset(masked_pos as isize) as *mut u8 as *mut libc::c_void,
                bytes as *const libc::c_void,
                n,
            );
        } else {
            memcpy(
                &mut *((*rb).buffer_).offset(masked_pos as isize) as *mut u8 as *mut libc::c_void,
                bytes as *const libc::c_void,
                brotli_min_size_t(n, ((*rb).total_size_ as u64).wrapping_sub(masked_pos)),
            );
            memcpy(
                &mut *((*rb).buffer_).offset(0 as isize) as *mut u8 as *mut libc::c_void,
                bytes.offset(((*rb).size_ as u64).wrapping_sub(masked_pos) as isize)
                    as *const libc::c_void,
                n.wrapping_sub(((*rb).size_ as u64).wrapping_sub(masked_pos)),
            );
        }
        let mut not_first_lap = ((*rb).pos_ & 1u32 << 31 != 0) as i32;
        let mut rb_pos_mask = (1u32 << 31).wrapping_sub(1);
        *((*rb).buffer_).offset(-2i32 as isize) =
            *((*rb).buffer_).offset(((*rb).size_).wrapping_sub(2) as isize);
        *((*rb).buffer_).offset(-1i32 as isize) =
            *((*rb).buffer_).offset(((*rb).size_).wrapping_sub(1) as isize);
        (*rb).pos_ = ((*rb).pos_ & rb_pos_mask).wrapping_add((n & rb_pos_mask as u64) as u32);
        if not_first_lap != 0 {
            let ref mut fresh69 = (*rb).pos_;
            *fresh69 |= 1 << 31;
        }
    }
}

static mut kMinUTF8Ratio: f64 = 0.75f64;
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

extern "C" fn InputBlockSize(mut s: *mut BrotliEncoderState) -> u64 {
    unsafe {
        return 1 << (*s).params.lgblock;
    }
}

extern "C" fn UnprocessedInputSize(mut s: *mut BrotliEncoderState) -> u64 {
    unsafe {
        return ((*s).input_pos_).wrapping_sub((*s).last_processed_pos_);
    }
}

extern "C" fn RemainingInputBlockSize(mut s: *mut BrotliEncoderState) -> u64 {
    unsafe {
        let delta = UnprocessedInputSize(s);
        let mut block_size = InputBlockSize(s);
        if delta >= block_size {
            return 0;
        }
        return block_size.wrapping_sub(delta);
    }
}

#[no_mangle]
pub extern "C" fn BrotliEncoderSetParameter(
    mut state: *mut BrotliEncoderState,
    mut p: u32,
    mut value: u32,
) -> i32 {
    unsafe {
        if (*state).is_initialized_ != 0 {
            return 0;
        }
        match p as u32 {
            0 => {
                (*state).params.mode = value as u32;
                return 1;
            }
            1 => {
                (*state).params.quality = value as i32;
                return 1;
            }
            2 => {
                (*state).params.lgwin = value as i32;
                return 1;
            }
            3 => {
                (*state).params.lgblock = value as i32;
                return 1;
            }
            4 => {
                if value != 0 && value != 1 {
                    return 0;
                };
                (*state).params.disable_literal_context_modeling = if value != 0 { 1 } else { 0 };
                return 1;
            }
            5 => {
                (*state).params.size_hint = value as u64;
                return 1;
            }
            6 => {
                (*state).params.large_window = if value != 0 { 1 } else { 0 };
                return 1;
            }
            7 => {
                (*state).params.dist.distance_postfix_bits = value;
                return 1;
            }
            8 => {
                (*state).params.dist.num_direct_distance_codes = value;
                return 1;
            }
            9 => {
                if value > 1 << 30 {
                    return 0;
                };
                (*state).params.stream_offset = value as u64;
                return 1;
            }
            _ => return 0,
        };
    }
}

extern "C" fn WrapPosition(mut position: u64) -> u32 {
    let mut result = position as u32;
    let mut gb = position >> 30;
    if gb > 2 {
        result = result & (1u32 << 30).wrapping_sub(1)
            | ((gb.wrapping_sub(1u64) & 1u64) as u32).wrapping_add(1) << 30;
    }
    return result;
}

extern "C" fn GetBrotliStorage(mut s: *mut BrotliEncoderState, mut size: u64) -> *mut u8 {
    unsafe {
        let mut m: *mut MemoryManager = &mut (*s).memory_manager_;
        if (*s).storage_size_ < size {
            BrotliFree(m, (*s).storage_ as *mut libc::c_void);
            let ref mut fresh70 = (*s).storage_;
            *fresh70 = 0 as *mut u8;
            let ref mut fresh71 = (*s).storage_;
            *fresh71 = if size > 0 {
                BrotliAllocate(m, size.wrapping_mul(::std::mem::size_of::<u8>() as u64)) as *mut u8
            } else {
                0 as *mut u8
            };
            if 0 != 0 || 0 != 0 {
                return 0 as *mut u8;
            };
            (*s).storage_size_ = size;
        }
        return (*s).storage_;
    }
}

extern "C" fn HashTableSize(mut max_table_size: u64, mut input_size: u64) -> u64 {
    let mut htsize = 256;
    while htsize < max_table_size && htsize < input_size {
        htsize <<= 1;
    }
    return htsize;
}

extern "C" fn GetHashTable(
    mut s: *mut BrotliEncoderState,
    mut quality: i32,
    mut input_size: u64,
    mut table_size: *mut u64,
) -> *mut i32 {
    unsafe {
        let mut m: *mut MemoryManager = &mut (*s).memory_manager_;
        let max_table_size = MaxHashTableSize(quality);
        let mut htsize = HashTableSize(max_table_size, input_size);
        let mut table = 0 as *mut i32;
        if quality == 0 {
            if htsize & 0xaaaaa == 0 {
                htsize <<= 1;
            }
        }
        if htsize
            <= (::std::mem::size_of::<[i32; 1024]>() as u64)
                .wrapping_div(::std::mem::size_of::<i32>() as u64)
        {
            table = ((*s).small_table_).as_mut_ptr();
        } else {
            if htsize > (*s).large_table_size_ {
                (*s).large_table_size_ = htsize;
                BrotliFree(m, (*s).large_table_ as *mut libc::c_void);
                let ref mut fresh72 = (*s).large_table_;
                *fresh72 = 0 as *mut i32;
                let ref mut fresh73 = (*s).large_table_;
                *fresh73 = if htsize > 0 {
                    BrotliAllocate(m, htsize.wrapping_mul(::std::mem::size_of::<i32>() as u64))
                        as *mut i32
                } else {
                    0 as *mut i32
                };
                if 0 != 0 || 0 != 0 {
                    return 0 as *mut i32;
                }
            }
            table = (*s).large_table_;
        }
        *table_size = htsize;
        memset(
            table as *mut libc::c_void,
            0,
            htsize.wrapping_mul(::std::mem::size_of::<i32>() as u64),
        );
        return table;
    }
}

extern "C" fn EncodeWindowBits(
    mut lgwin: i32,
    mut large_window: i32,
    mut last_bytes: *mut u16,
    mut last_bytes_bits: *mut u8,
) {
    unsafe {
        if large_window != 0 {
            *last_bytes = ((lgwin & 0x3fi32) << 8 | 0x11) as u16;
            *last_bytes_bits = 14;
        } else if lgwin == 16 {
            *last_bytes = 0;
            *last_bytes_bits = 1;
        } else if lgwin == 17 {
            *last_bytes = 1;
            *last_bytes_bits = 7;
        } else if lgwin > 17 {
            *last_bytes = ((lgwin - 17i32) << 1 | 0x1) as u16;
            *last_bytes_bits = 4;
        } else {
            *last_bytes = ((lgwin - 8i32) << 4 | 0x1) as u16;
            *last_bytes_bits = 7;
        };
    }
}

extern "C" fn InitCommandPrefixCodes(
    mut cmd_depths: *mut u8,
    mut cmd_bits: *mut u16,
    mut cmd_code: *mut u8,
    mut cmd_code_numbits: *mut u64,
) {
    unsafe {
        static mut kDefaultCommandDepths: [u8; 128] = [
            0, 4, 4, 5, 6, 6, 7, 7, 7, 7, 7, 8, 8, 8, 8, 8, 0, 0, 0, 4, 4, 4, 4, 4, 5, 5, 6, 6, 6,
            6, 7, 7, 7, 7, 10, 10, 10, 10, 10, 10, 0, 4, 4, 5, 5, 5, 6, 6, 7, 8, 8, 9, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
            6, 6, 6, 6, 6, 5, 5, 5, 5, 5, 5, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 7, 7, 7,
            8, 10, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 0, 0, 0, 0,
        ];
        static mut kDefaultCommandBits: [u16; 128] = [
            0, 0, 8, 9, 3, 35, 7, 71, 39, 103, 23, 47, 175, 111, 239, 31, 0, 0, 0, 4, 12, 2, 10, 6,
            13, 29, 11, 43, 27, 59, 87, 55, 15, 79, 319, 831, 191, 703, 447, 959, 0, 14, 1, 25, 5,
            21, 19, 51, 119, 159, 95, 223, 479, 991, 63, 575, 127, 639, 383, 895, 255, 767, 511,
            1023, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 59, 7, 39, 23, 55, 30, 1,
            17, 9, 25, 5, 0, 8, 4, 12, 2, 10, 6, 21, 13, 29, 3, 19, 11, 15, 47, 31, 95, 63, 127,
            255, 767, 2815, 1791, 3839, 511, 2559, 1535, 3583, 1023, 3071, 2047, 4095, 0, 0, 0, 0,
        ];
        static mut kDefaultCommandCode: [u8; 57] = [
            0xff, 0x77, 0xd5, 0xbf, 0xe7, 0xde, 0xea, 0x9e, 0x51, 0x5d, 0xde, 0xc6, 0x70, 0x57,
            0xbc, 0x58, 0x58, 0x58, 0xd8, 0xd8, 0x58, 0xd5, 0xcb, 0x8c, 0xea, 0xe0, 0xc3, 0x87,
            0x1f, 0x83, 0xc1, 0x60, 0x1c, 0x67, 0xb2, 0xaa, 0x6, 0x83, 0xc1, 0x60, 0x30, 0x18,
            0xcc, 0xa1, 0xce, 0x88, 0x54, 0x94, 0x46, 0xe1, 0xb0, 0xd0, 0x4e, 0xb2, 0xf7, 0x4, 0,
        ];
        static mut kDefaultCommandCodeNumBits: u64 = 448;
        memcpy(
            cmd_depths as *mut libc::c_void,
            kDefaultCommandDepths.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[u8; 128]>() as u64,
        );
        memcpy(
            cmd_bits as *mut libc::c_void,
            kDefaultCommandBits.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[u16; 128]>() as u64,
        );
        memcpy(
            cmd_code as *mut libc::c_void,
            kDefaultCommandCode.as_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[u8; 57]>() as u64,
        );
        *cmd_code_numbits = kDefaultCommandCodeNumBits;
    }
}

extern "C" fn ChooseContextMap(
    mut quality: i32,
    mut bigram_histo: *mut u32,
    mut num_literal_contexts: *mut u64,
    mut literal_context_map: *mut *const u32,
) {
    unsafe {
        static mut kStaticContextMapContinuation: [u32; 64] = [
            1, 1, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        static mut kStaticContextMapSimpleUTF8: [u32; 64] = [
            0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ];
        let mut monogram_histo: [u32; 3] = [0; 3];
        let mut two_prefix_histo: [u32; 6] = [0; 6];
        let mut total: u64 = 0;
        let mut i: u64 = 0;
        let mut dummy: u64 = 0;
        let mut entropy: [f64; 4] = [0.; 4];
        i = 0;
        while i < 9 {
            monogram_histo[i.wrapping_rem(3) as usize] =
                (monogram_histo[i.wrapping_rem(3u64) as usize] as u32)
                    .wrapping_add(*bigram_histo.offset(i as isize)) as u32;
            two_prefix_histo[i.wrapping_rem(6) as usize] =
                (two_prefix_histo[i.wrapping_rem(6u64) as usize] as u32)
                    .wrapping_add(*bigram_histo.offset(i as isize)) as u32;
            i = i.wrapping_add(1);
        }
        entropy[1 as usize] = ShannonEntropy(monogram_histo.as_mut_ptr(), 3, &mut dummy);
        entropy[2 as usize] = ShannonEntropy(two_prefix_histo.as_mut_ptr(), 3, &mut dummy)
            + ShannonEntropy(
                two_prefix_histo.as_mut_ptr().offset(3 as isize),
                3,
                &mut dummy,
            );
        entropy[3 as usize] = 0 as f64;
        i = 0;
        while i < 3 {
            entropy[3 as usize] += ShannonEntropy(
                bigram_histo.offset(3u64.wrapping_mul(i) as isize),
                3,
                &mut dummy,
            );
            i = i.wrapping_add(1);
        }
        total = (monogram_histo[0 as usize])
            .wrapping_add(monogram_histo[1 as usize])
            .wrapping_add(monogram_histo[2 as usize]) as u64;
        entropy[0 as usize] = 1.0f64 / total as f64;
        entropy[1 as usize] *= entropy[0 as usize];
        entropy[2 as usize] *= entropy[0 as usize];
        entropy[3 as usize] *= entropy[0 as usize];
        if quality < 7 {
            entropy[3 as usize] = entropy[1 as usize] * 10 as f64;
        }
        if entropy[1 as usize] - entropy[2 as usize] < 0.2f64
            && entropy[1 as usize] - entropy[3 as usize] < 0.2f64
        {
            *num_literal_contexts = 1;
        } else if entropy[2 as usize] - entropy[3 as usize] < 0.02f64 {
            *num_literal_contexts = 2;
            *literal_context_map = kStaticContextMapSimpleUTF8.as_ptr();
        } else {
            *num_literal_contexts = 3;
            *literal_context_map = kStaticContextMapContinuation.as_ptr();
        };
    }
}

extern "C" fn ShouldUseComplexStaticContextMap(
    mut input: *const u8,
    mut start_pos: u64,
    mut length: u64,
    mut mask: u64,
    mut quality: i32,
    mut size_hint: u64,
    mut num_literal_contexts: *mut u64,
    mut literal_context_map: *mut *const u32,
) -> i32 {
    unsafe {
        static mut kStaticContextMapComplexUTF8: [u32; 64] = [
            11, 11, 12, 12, 0, 0, 0, 0, 1, 1, 9, 9, 2, 2, 2, 2, 1, 1, 1, 1, 8, 3, 3, 3, 1, 1, 1, 1,
            2, 2, 2, 2, 8, 4, 4, 4, 8, 7, 4, 4, 8, 0, 0, 0, 3, 3, 3, 3, 5, 5, 10, 5, 5, 5, 10, 5,
            6, 6, 6, 6, 6, 6, 6, 6,
        ];
        if size_hint < (1i32 << 20) as u64 {
            return 0;
        } else {
            let end_pos = start_pos.wrapping_add(length);
            let mut combined_histo: [u32; 32] = [0; 32];
            let mut context_histo: [[u32; 32]; 13] = [
                [0; 32], [0; 32], [0; 32], [0; 32], [0; 32], [0; 32], [0; 32], [0; 32], [0; 32],
                [0; 32], [0; 32], [0; 32], [0; 32],
            ];
            let mut total = 0;
            let mut entropy: [f64; 3] = [0.; 3];
            let mut dummy: u64 = 0;
            let mut i: u64 = 0;
            let mut utf8_lut: ContextLut = &*_kBrotliContextLookupTable
                .as_ptr()
                .offset(((CONTEXT_UTF8 as i32) << 9i32) as isize)
                as *const u8;
            while start_pos.wrapping_add(64) <= end_pos {
                let stride_end_pos = start_pos.wrapping_add(64);
                let mut prev2 = *input.offset((start_pos & mask) as isize);
                let mut prev1 = *input.offset((start_pos.wrapping_add(1u64) & mask) as isize);
                let mut pos: u64 = 0;
                pos = start_pos.wrapping_add(2);
                while pos < stride_end_pos {
                    let literal = *input.offset((pos & mask) as isize);
                    let context = kStaticContextMapComplexUTF8[(*utf8_lut.offset(prev1 as isize)
                        as i32
                        | *utf8_lut.offset(256 as isize).offset(prev2 as isize) as i32)
                        as usize] as u8;
                    total = total.wrapping_add(1);
                    combined_histo[(literal as i32 >> 3i32) as usize] =
                        (combined_histo[(literal as i32 >> 3i32) as usize]).wrapping_add(1);
                    context_histo[context as usize][(literal as i32 >> 3i32) as usize] =
                        (context_histo[context as usize][(literal as i32 >> 3i32) as usize])
                            .wrapping_add(1);
                    prev2 = prev1;
                    prev1 = literal;
                    pos = pos.wrapping_add(1);
                }
                start_pos = (start_pos as u64).wrapping_add(4096) as u64;
            }
            entropy[1 as usize] = ShannonEntropy(combined_histo.as_mut_ptr(), 32, &mut dummy);
            entropy[2 as usize] = 0 as f64;
            i = 0;
            while i < 13 {
                entropy[2 as usize] += ShannonEntropy(
                    &mut *(*context_histo.as_mut_ptr().offset(i as isize))
                        .as_mut_ptr()
                        .offset(0 as isize),
                    32,
                    &mut dummy,
                );
                i = i.wrapping_add(1);
            }
            entropy[0 as usize] = 1.0f64 / total as f64;
            entropy[1 as usize] *= entropy[0 as usize];
            entropy[2 as usize] *= entropy[0 as usize];
            if entropy[2 as usize] > 3.0f64 || entropy[1 as usize] - entropy[2 as usize] < 0.2f64 {
                return 0;
            } else {
                *num_literal_contexts = 13;
                *literal_context_map = kStaticContextMapComplexUTF8.as_ptr();
                return 1;
            }
        };
    }
}

extern "C" fn DecideOverLiteralContextModeling(
    mut input: *const u8,
    mut start_pos: u64,
    mut length: u64,
    mut mask: u64,
    mut quality: i32,
    mut size_hint: u64,
    mut num_literal_contexts: *mut u64,
    mut literal_context_map: *mut *const u32,
) {
    unsafe {
        if quality < 5 || length < 64 {
            return;
        } else {
            if !(ShouldUseComplexStaticContextMap(
                input,
                start_pos,
                length,
                mask,
                quality,
                size_hint,
                num_literal_contexts,
                literal_context_map,
            ) != 0)
            {
                let end_pos = start_pos.wrapping_add(length);
                let mut bigram_prefix_histo: [u32; 9] = [0; 9];
                while start_pos.wrapping_add(64) <= end_pos {
                    static mut lut: [i32; 4] = [0, 0, 1, 2];
                    let stride_end_pos = start_pos.wrapping_add(64);
                    let mut prev = lut
                        [(*input.offset((start_pos & mask) as isize) as i32 >> 6i32) as usize]
                        * 3;
                    let mut pos: u64 = 0;
                    pos = start_pos.wrapping_add(1);
                    while pos < stride_end_pos {
                        let literal = *input.offset((pos & mask) as isize);
                        bigram_prefix_histo
                            [(prev + lut[(literal as i32 >> 6i32) as usize]) as usize] =
                            (bigram_prefix_histo
                                [(prev + lut[(literal as i32 >> 6i32) as usize]) as usize])
                                .wrapping_add(1);
                        prev = lut[(literal as i32 >> 6i32) as usize] * 3;
                        pos = pos.wrapping_add(1);
                    }
                    start_pos = (start_pos as u64).wrapping_add(4096) as u64;
                }
                ChooseContextMap(
                    quality,
                    &mut *bigram_prefix_histo.as_mut_ptr().offset(0 as isize),
                    num_literal_contexts,
                    literal_context_map,
                );
            }
        };
    }
}

extern "C" fn ShouldCompress(
    mut data: *const u8,
    mask: u64,
    last_flush_pos: u64,
    bytes: u64,
    num_literals: u64,
    num_commands: u64,
) -> i32 {
    unsafe {
        if bytes <= 2 {
            return 0;
        }
        if num_commands < (bytes >> 8i32).wrapping_add(2) {
            if num_literals as f64 > 0.99f64 * bytes as f64 {
                let mut literal_histo: [u32; 256] = [0; 256];
                static mut kSampleRate: u32 = 13;
                static mut kMinEntropy: f64 = 7.92f64;
                let bit_cost_threshold = bytes as f64 * kMinEntropy / kSampleRate as f64;
                let mut t = bytes
                    .wrapping_add(kSampleRate as u64)
                    .wrapping_sub(1)
                    .wrapping_div(kSampleRate as u64);
                let mut pos = last_flush_pos as u32;
                let mut i: u64 = 0;
                i = 0;
                while i < t {
                    literal_histo[*data.offset((pos as u64 & mask) as isize) as usize] =
                        (literal_histo[*data.offset((pos as u64 & mask) as isize) as usize])
                            .wrapping_add(1);
                    pos = (pos as u32).wrapping_add(kSampleRate) as u32;
                    i = i.wrapping_add(1);
                }
                if BitsEntropy(literal_histo.as_mut_ptr(), 256) > bit_cost_threshold {
                    return 0;
                }
            }
        }
        return 1;
    }
}

extern "C" fn ChooseContextMode(
    mut params: *const BrotliEncoderParams,
    mut data: *const u8,
    pos: u64,
    mask: u64,
    length: u64,
) -> u32 {
    unsafe {
        if (*params).quality >= 10
            && BrotliIsMostlyUTF8(data, pos, mask, length, kMinUTF8Ratio) == 0
        {
            return CONTEXT_SIGNED;
        }
        return CONTEXT_UTF8;
    }
}

extern "C" fn WriteMetaBlockInternal(
    mut m: *mut MemoryManager,
    mut data: *const u8,
    mask: u64,
    last_flush_pos: u64,
    bytes: u64,
    is_last: i32,
    mut literal_context_mode: u32,
    mut params: *const BrotliEncoderParams,
    prev_byte: u8,
    prev_byte2: u8,
    num_literals: u64,
    num_commands: u64,
    mut commands: *mut Command,
    mut saved_dist_cache: *const i32,
    mut dist_cache: *mut i32,
    mut storage_ix: *mut u64,
    mut storage: *mut u8,
) {
    unsafe {
        let wrapped_last_flush_pos = WrapPosition(last_flush_pos);
        let mut last_bytes: u16 = 0;
        let mut last_bytes_bits: u8 = 0;
        let mut literal_context_lut: ContextLut = &*_kBrotliContextLookupTable
            .as_ptr()
            .offset(((literal_context_mode as u32) << 9i32) as isize)
            as *const u8;
        let mut block_params = *params;
        if bytes == 0 {
            BrotliWriteBits(2, 3, storage_ix, storage);
            *storage_ix = (*storage_ix).wrapping_add(7) & !7 as u64;
            return;
        }
        if ShouldCompress(
            data,
            mask,
            last_flush_pos,
            bytes,
            num_literals,
            num_commands,
        ) == 0
        {
            memcpy(
                dist_cache as *mut libc::c_void,
                saved_dist_cache as *const libc::c_void,
                4u64.wrapping_mul(::std::mem::size_of::<i32>() as u64),
            );
            BrotliStoreUncompressedMetaBlock(
                is_last,
                data,
                wrapped_last_flush_pos as u64,
                mask,
                bytes,
                storage_ix,
                storage,
            );
            return;
        }
        last_bytes =
            ((*storage.offset(1 as isize) as i32) << 8 | *storage.offset(0 as isize) as i32) as u16;
        last_bytes_bits = *storage_ix as u8;
        if (*params).quality <= 2 {
            BrotliStoreMetaBlockFast(
                m,
                data,
                wrapped_last_flush_pos as u64,
                bytes,
                mask,
                is_last,
                params,
                commands,
                num_commands,
                storage_ix,
                storage,
            );
            if 0 != 0 {
                return;
            }
        } else if (*params).quality < 4 {
            BrotliStoreMetaBlockTrivial(
                m,
                data,
                wrapped_last_flush_pos as u64,
                bytes,
                mask,
                is_last,
                params,
                commands,
                num_commands,
                storage_ix,
                storage,
            );
            if 0 != 0 {
                return;
            }
        } else {
            let mut mb = MetaBlockSplit {
                literal_split: BlockSplit {
                    num_types: 0,
                    num_blocks: 0,
                    types: 0 as *mut u8,
                    lengths: 0 as *mut u32,
                    types_alloc_size: 0,
                    lengths_alloc_size: 0,
                },
                command_split: BlockSplit {
                    num_types: 0,
                    num_blocks: 0,
                    types: 0 as *mut u8,
                    lengths: 0 as *mut u32,
                    types_alloc_size: 0,
                    lengths_alloc_size: 0,
                },
                distance_split: BlockSplit {
                    num_types: 0,
                    num_blocks: 0,
                    types: 0 as *mut u8,
                    lengths: 0 as *mut u32,
                    types_alloc_size: 0,
                    lengths_alloc_size: 0,
                },
                literal_context_map: 0 as *mut u32,
                literal_context_map_size: 0,
                distance_context_map: 0 as *mut u32,
                distance_context_map_size: 0,
                literal_histograms: 0 as *mut HistogramLiteral,
                literal_histograms_size: 0,
                command_histograms: 0 as *mut HistogramCommand,
                command_histograms_size: 0,
                distance_histograms: 0 as *mut HistogramDistance,
                distance_histograms_size: 0,
            };
            InitMetaBlockSplit(&mut mb);
            if (*params).quality < 10 {
                let mut num_literal_contexts = 1;
                let mut literal_context_map = 0 as *const u32;
                if (*params).disable_literal_context_modeling == 0 {
                    DecideOverLiteralContextModeling(
                        data,
                        wrapped_last_flush_pos as u64,
                        bytes,
                        mask,
                        (*params).quality,
                        (*params).size_hint,
                        &mut num_literal_contexts,
                        &mut literal_context_map,
                    );
                }
                BrotliBuildMetaBlockGreedy(
                    m,
                    data,
                    wrapped_last_flush_pos as u64,
                    mask,
                    prev_byte,
                    prev_byte2,
                    literal_context_lut,
                    num_literal_contexts,
                    literal_context_map,
                    commands,
                    num_commands,
                    &mut mb,
                );
                if 0 != 0 {
                    return;
                }
            } else {
                BrotliBuildMetaBlock(
                    m,
                    data,
                    wrapped_last_flush_pos as u64,
                    mask,
                    &mut block_params,
                    prev_byte,
                    prev_byte2,
                    commands,
                    num_commands,
                    literal_context_mode,
                    &mut mb,
                );
                if 0 != 0 {
                    return;
                }
            }
            if (*params).quality >= 4 {
                BrotliOptimizeHistograms(block_params.dist.alphabet_size_limit, &mut mb);
            }
            BrotliStoreMetaBlock(
                m,
                data,
                wrapped_last_flush_pos as u64,
                bytes,
                mask,
                prev_byte,
                prev_byte2,
                is_last,
                &mut block_params,
                literal_context_mode,
                commands,
                num_commands,
                &mut mb,
                storage_ix,
                storage,
            );
            if 0 != 0 {
                return;
            }
            DestroyMetaBlockSplit(m, &mut mb);
        }
        if bytes.wrapping_add(4) < *storage_ix >> 3 {
            memcpy(
                dist_cache as *mut libc::c_void,
                saved_dist_cache as *const libc::c_void,
                4u64.wrapping_mul(::std::mem::size_of::<i32>() as u64),
            );
            *storage.offset(0 as isize) = last_bytes as u8;
            *storage.offset(1 as isize) = (last_bytes as i32 >> 8i32) as u8;
            *storage_ix = last_bytes_bits as u64;
            BrotliStoreUncompressedMetaBlock(
                is_last,
                data,
                wrapped_last_flush_pos as u64,
                mask,
                bytes,
                storage_ix,
                storage,
            );
        }
    }
}

extern "C" fn ChooseDistanceParams(mut params: *mut BrotliEncoderParams) {
    unsafe {
        let mut distance_postfix_bits = 0;
        let mut num_direct_distance_codes = 0;
        if (*params).quality >= 4 {
            let mut ndirect_msb: u32 = 0;
            if (*params).mode as u32 == BROTLI_MODE_FONT as u32 {
                distance_postfix_bits = 1;
                num_direct_distance_codes = 12;
            } else {
                distance_postfix_bits = (*params).dist.distance_postfix_bits;
                num_direct_distance_codes = (*params).dist.num_direct_distance_codes;
            }
            ndirect_msb = num_direct_distance_codes >> distance_postfix_bits & 0xf;
            if distance_postfix_bits > 3
                || num_direct_distance_codes > 120
                || ndirect_msb << distance_postfix_bits != num_direct_distance_codes
            {
                distance_postfix_bits = 0;
                num_direct_distance_codes = 0;
            }
        }
        BrotliInitDistanceParams(params, distance_postfix_bits, num_direct_distance_codes);
    }
}

extern "C" fn EnsureInitialized(mut s: *mut BrotliEncoderState) -> i32 {
    unsafe {
        if 0 != 0 {
            return 0;
        }
        if (*s).is_initialized_ != 0 {
            return 1;
        };
        (*s).last_bytes_bits_ = 0;
        (*s).last_bytes_ = 0;
        (*s).flint_ = BROTLI_FLINT_DONE as i8;
        (*s).remaining_metadata_bytes_ = !0;
        SanitizeParams(&mut (*s).params);
        (*s).params.lgblock = ComputeLgBlock(&mut (*s).params);
        ChooseDistanceParams(&mut (*s).params);
        if (*s).params.stream_offset != 0 {
            (*s).flint_ = BROTLI_FLINT_NEEDS_2_BYTES as i8;
            (*s).dist_cache_[0 as usize] = -16;
            (*s).dist_cache_[1 as usize] = -16;
            (*s).dist_cache_[2 as usize] = -16;
            (*s).dist_cache_[3 as usize] = -16;
            memcpy(
                ((*s).saved_dist_cache_).as_mut_ptr() as *mut libc::c_void,
                ((*s).dist_cache_).as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[i32; 4]>() as u64,
            );
        }
        RingBufferSetup(&mut (*s).params, &mut (*s).ringbuffer_);
        let mut lgwin = (*s).params.lgwin;
        if (*s).params.quality == 0 || (*s).params.quality == 1 {
            lgwin = brotli_max_int(lgwin, 18);
        }
        if (*s).params.stream_offset == 0 {
            EncodeWindowBits(
                lgwin,
                (*s).params.large_window,
                &mut (*s).last_bytes_,
                &mut (*s).last_bytes_bits_,
            );
        } else {
            (*s).params.stream_offset =
                brotli_min_size_t((*s).params.stream_offset, (1u64 << lgwin).wrapping_sub(16));
        }
        if (*s).params.quality == 0 {
            InitCommandPrefixCodes(
                ((*s).cmd_depths_).as_mut_ptr(),
                ((*s).cmd_bits_).as_mut_ptr(),
                ((*s).cmd_code_).as_mut_ptr(),
                &mut (*s).cmd_code_numbits_,
            );
        };
        (*s).is_initialized_ = 1;
        return 1;
    }
}

extern "C" fn BrotliEncoderInitParams(mut params: *mut BrotliEncoderParams) {
    unsafe {
        (*params).mode = BROTLI_MODE_GENERIC;
        (*params).large_window = 0;
        (*params).quality = 11;
        (*params).lgwin = 22;
        (*params).lgblock = 0;
        (*params).stream_offset = 0;
        (*params).size_hint = 0;
        (*params).disable_literal_context_modeling = 0;
        BrotliInitEncoderDictionary(&mut (*params).dictionary);
        (*params).dist.distance_postfix_bits = 0;
        (*params).dist.num_direct_distance_codes = 0;
        (*params).dist.alphabet_size_max = ((16 + 0i32) as u32).wrapping_add(24 << 0 + 1);
        (*params).dist.alphabet_size_limit = (*params).dist.alphabet_size_max;
        (*params).dist.max_distance = 0x3fffffc;
    }
}

extern "C" fn BrotliEncoderInitState(mut s: *mut BrotliEncoderState) {
    unsafe {
        BrotliEncoderInitParams(&mut (*s).params);
        (*s).input_pos_ = 0;
        (*s).num_commands_ = 0;
        (*s).num_literals_ = 0;
        (*s).last_insert_len_ = 0;
        (*s).last_flush_pos_ = 0;
        (*s).last_processed_pos_ = 0;
        (*s).prev_byte_ = 0;
        (*s).prev_byte2_ = 0;
        (*s).storage_size_ = 0;
        let ref mut fresh74 = (*s).storage_;
        *fresh74 = 0 as *mut u8;
        HasherInit(&mut (*s).hasher_);
        let ref mut fresh75 = (*s).large_table_;
        *fresh75 = 0 as *mut i32;
        (*s).large_table_size_ = 0;
        (*s).cmd_code_numbits_ = 0;
        let ref mut fresh76 = (*s).command_buf_;
        *fresh76 = 0 as *mut u32;
        let ref mut fresh77 = (*s).literal_buf_;
        *fresh77 = 0 as *mut u8;
        let ref mut fresh78 = (*s).next_out_;
        *fresh78 = 0 as *mut u8;
        (*s).available_out_ = 0;
        (*s).total_out_ = 0;
        (*s).stream_state_ = BROTLI_STREAM_PROCESSING;
        (*s).is_last_block_emitted_ = 0;
        (*s).is_initialized_ = 0;
        RingBufferInit(&mut (*s).ringbuffer_);
        let ref mut fresh79 = (*s).commands_;
        *fresh79 = 0 as *mut Command;
        (*s).cmd_alloc_size_ = 0;
        (*s).dist_cache_[0 as usize] = 4;
        (*s).dist_cache_[1 as usize] = 11;
        (*s).dist_cache_[2 as usize] = 15;
        (*s).dist_cache_[3 as usize] = 16;
        memcpy(
            ((*s).saved_dist_cache_).as_mut_ptr() as *mut libc::c_void,
            ((*s).dist_cache_).as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[i32; 4]>() as u64,
        );
    }
}

#[no_mangle]
pub extern "C" fn BrotliEncoderCreateInstance(
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut libc::c_void,
) -> *mut BrotliEncoderState {
    unsafe {
        let mut state = 0 as *mut BrotliEncoderState;
        if alloc_func.is_none() && free_func.is_none() {
            state = malloc(::std::mem::size_of::<BrotliEncoderState>() as u64)
                as *mut BrotliEncoderState;
        } else if alloc_func.is_some() && free_func.is_some() {
            state = alloc_func.expect("non-null function pointer")(
                opaque,
                ::std::mem::size_of::<BrotliEncoderState>() as u64,
            ) as *mut BrotliEncoderState;
        }
        if state.is_null() {
            return 0 as *mut BrotliEncoderState;
        }
        BrotliInitMemoryManager(&mut (*state).memory_manager_, alloc_func, free_func, opaque);
        BrotliEncoderInitState(state);
        return state;
    }
}

extern "C" fn BrotliEncoderCleanupState(mut s: *mut BrotliEncoderState) {
    unsafe {
        let mut m: *mut MemoryManager = &mut (*s).memory_manager_;
        if 0 != 0 {
            BrotliWipeOutMemoryManager(m);
            return;
        }
        BrotliFree(m, (*s).storage_ as *mut libc::c_void);
        let ref mut fresh80 = (*s).storage_;
        *fresh80 = 0 as *mut u8;
        BrotliFree(m, (*s).commands_ as *mut libc::c_void);
        let ref mut fresh81 = (*s).commands_;
        *fresh81 = 0 as *mut Command;
        RingBufferFree(m, &mut (*s).ringbuffer_);
        DestroyHasher(m, &mut (*s).hasher_);
        BrotliFree(m, (*s).large_table_ as *mut libc::c_void);
        let ref mut fresh82 = (*s).large_table_;
        *fresh82 = 0 as *mut i32;
        BrotliFree(m, (*s).command_buf_ as *mut libc::c_void);
        let ref mut fresh83 = (*s).command_buf_;
        *fresh83 = 0 as *mut u32;
        BrotliFree(m, (*s).literal_buf_ as *mut libc::c_void);
        let ref mut fresh84 = (*s).literal_buf_;
        *fresh84 = 0 as *mut u8;
    }
}

#[no_mangle]
pub extern "C" fn BrotliEncoderDestroyInstance(mut state: *mut BrotliEncoderState) {
    unsafe {
        if state.is_null() {
            return;
        } else {
            let mut m: *mut MemoryManager = &mut (*state).memory_manager_;
            let mut free_func: brotli_free_func = (*m).free_func;
            let mut opaque = (*m).opaque;
            BrotliEncoderCleanupState(state);
            free_func.expect("non-null function pointer")(opaque, state as *mut libc::c_void);
        };
    }
}

extern "C" fn CopyInputToRingBuffer(
    mut s: *mut BrotliEncoderState,
    input_size: u64,
    mut input_buffer: *const u8,
) {
    unsafe {
        let mut ringbuffer_: *mut RingBuffer = &mut (*s).ringbuffer_;
        let mut m: *mut MemoryManager = &mut (*s).memory_manager_;
        RingBufferWrite(m, input_buffer, input_size, ringbuffer_);
        if 0 != 0 {
            return;
        }
        let ref mut fresh85 = (*s).input_pos_;
        *fresh85 = (*fresh85 as u64).wrapping_add(input_size) as u64;
        if (*ringbuffer_).pos_ <= (*ringbuffer_).mask_ {
            memset(
                ((*ringbuffer_).buffer_).offset((*ringbuffer_).pos_ as isize) as *mut libc::c_void,
                0,
                7,
            );
        }
    }
}

extern "C" fn UpdateLastProcessedPos(mut s: *mut BrotliEncoderState) -> i32 {
    unsafe {
        let mut wrapped_last_processed_pos = WrapPosition((*s).last_processed_pos_);
        let mut wrapped_input_pos = WrapPosition((*s).input_pos_);
        (*s).last_processed_pos_ = (*s).input_pos_;
        return if wrapped_input_pos < wrapped_last_processed_pos {
            1
        } else {
            0
        };
    }
}

extern "C" fn ExtendLastCommand(
    mut s: *mut BrotliEncoderState,
    mut bytes: *mut u32,
    mut wrapped_last_processed_pos: *mut u32,
) {
    unsafe {
        let mut last_command: *mut Command = &mut *((*s).commands_)
            .offset(((*s).num_commands_).wrapping_sub(1) as isize)
            as *mut Command;
        let mut data: *const u8 = (*s).ringbuffer_.buffer_;
        let mask = (*s).ringbuffer_.mask_;
        let mut max_backward_distance = (1u64 << (*s).params.lgwin).wrapping_sub(16);
        let mut last_copy_len = ((*last_command).copy_len_ & 0x1ffffffu32) as u64;
        let mut last_processed_pos = ((*s).last_processed_pos_).wrapping_sub(last_copy_len);
        let mut max_distance = if last_processed_pos < max_backward_distance {
            last_processed_pos
        } else {
            max_backward_distance
        };
        let mut cmd_dist = (*s).dist_cache_[0 as usize] as u64;
        let mut distance_code = CommandRestoreDistanceCode(last_command, &mut (*s).params.dist);
        if distance_code < 16 || distance_code.wrapping_sub((16 - 1i32) as u32) as u64 == cmd_dist {
            if cmd_dist <= max_distance {
                while *bytes != 0
                    && *data.offset((*wrapped_last_processed_pos & mask) as isize) as i32
                        == *data.offset(
                            ((*wrapped_last_processed_pos as u64).wrapping_sub(cmd_dist)
                                & mask as u64) as isize,
                        ) as i32
                {
                    let ref mut fresh86 = (*last_command).copy_len_;
                    *fresh86 = (*fresh86).wrapping_add(1);
                    *bytes = (*bytes).wrapping_sub(1);
                    *wrapped_last_processed_pos = (*wrapped_last_processed_pos).wrapping_add(1);
                }
            }
            GetLengthCode(
                (*last_command).insert_len_ as u64,
                (((*last_command).copy_len_ & 0x1ffffffu32) as i32
                    + ((*last_command).copy_len_ >> 25i32) as i32) as u64,
                if (*last_command).dist_prefix_ as i32 & 0x3ff == 0 {
                    1
                } else {
                    0
                },
                &mut (*last_command).cmd_prefix_,
            );
        }
    }
}

extern "C" fn EncodeData(
    mut s: *mut BrotliEncoderState,
    is_last: i32,
    force_flush: i32,
    mut out_size: *mut u64,
    mut output: *mut *mut u8,
) -> i32 {
    unsafe {
        let delta = UnprocessedInputSize(s);
        let mut bytes = delta as u32;
        let mut wrapped_last_processed_pos = WrapPosition((*s).last_processed_pos_);
        let mut data = 0 as *mut u8;
        let mut mask: u32 = 0;
        let mut m: *mut MemoryManager = &mut (*s).memory_manager_;
        let mut literal_context_mode = CONTEXT_LSB6;
        let mut literal_context_lut = 0 as *const u8;
        data = (*s).ringbuffer_.buffer_;
        mask = (*s).ringbuffer_.mask_;
        if (*s).is_last_block_emitted_ != 0 {
            return 0;
        }
        if is_last != 0 {
            (*s).is_last_block_emitted_ = 1;
        }
        if delta > InputBlockSize(s) {
            return 0;
        }
        if (*s).params.quality == 1 && ((*s).command_buf_).is_null() {
            let ref mut fresh87 = (*s).command_buf_;
            *fresh87 = if kCompressFragmentTwoPassBlockSize > 0 {
                BrotliAllocate(
                    m,
                    kCompressFragmentTwoPassBlockSize
                        .wrapping_mul(::std::mem::size_of::<u32>() as u64),
                ) as *mut u32
            } else {
                0 as *mut u32
            };
            let ref mut fresh88 = (*s).literal_buf_;
            *fresh88 = if kCompressFragmentTwoPassBlockSize > 0 {
                BrotliAllocate(
                    m,
                    kCompressFragmentTwoPassBlockSize
                        .wrapping_mul(::std::mem::size_of::<u8>() as u64),
                ) as *mut u8
            } else {
                0 as *mut u8
            };
            if 0 != 0 || 0 != 0 || 0 != 0 {
                return 0;
            }
        }
        if (*s).params.quality == 0 || (*s).params.quality == 1 {
            let mut storage = 0 as *mut u8;
            let mut storage_ix = (*s).last_bytes_bits_ as u64;
            let mut table_size: u64 = 0;
            let mut table = 0 as *mut i32;
            if delta == 0 && is_last == 0 {
                *out_size = 0;
                return 1;
            }
            storage = GetBrotliStorage(s, 2u32.wrapping_mul(bytes).wrapping_add(503) as u64);
            if 0 != 0 {
                return 0;
            };
            *storage.offset(0 as isize) = (*s).last_bytes_ as u8;
            *storage.offset(1 as isize) = ((*s).last_bytes_ as i32 >> 8i32) as u8;
            table = GetHashTable(s, (*s).params.quality, bytes as u64, &mut table_size);
            if 0 != 0 {
                return 0;
            }
            if (*s).params.quality == 0 {
                BrotliCompressFragmentFast(
                    m,
                    &mut *data.offset((wrapped_last_processed_pos & mask) as isize),
                    bytes as u64,
                    is_last,
                    table,
                    table_size,
                    ((*s).cmd_depths_).as_mut_ptr(),
                    ((*s).cmd_bits_).as_mut_ptr(),
                    &mut (*s).cmd_code_numbits_,
                    ((*s).cmd_code_).as_mut_ptr(),
                    &mut storage_ix,
                    storage,
                );
                if 0 != 0 {
                    return 0;
                }
            } else {
                BrotliCompressFragmentTwoPass(
                    m,
                    &mut *data.offset((wrapped_last_processed_pos & mask) as isize),
                    bytes as u64,
                    is_last,
                    (*s).command_buf_,
                    (*s).literal_buf_,
                    table,
                    table_size,
                    &mut storage_ix,
                    storage,
                );
                if 0 != 0 {
                    return 0;
                }
            };
            (*s).last_bytes_ = *storage.offset((storage_ix >> 3i32) as isize) as u16;
            (*s).last_bytes_bits_ = (storage_ix & 7u64) as u8;
            UpdateLastProcessedPos(s);
            *output = &mut *storage.offset(0 as isize) as *mut u8;
            *out_size = storage_ix >> 3;
            return 1;
        }
        let mut newsize = ((*s).num_commands_)
            .wrapping_add(bytes.wrapping_div(2) as u64)
            .wrapping_add(1);
        if newsize > (*s).cmd_alloc_size_ {
            let mut new_commands = 0 as *mut Command;
            newsize =
                (newsize as u64).wrapping_add(bytes.wrapping_div(4).wrapping_add(16) as u64) as u64;
            (*s).cmd_alloc_size_ = newsize;
            new_commands = if newsize > 0 {
                BrotliAllocate(
                    m,
                    newsize.wrapping_mul(::std::mem::size_of::<Command>() as u64),
                ) as *mut Command
            } else {
                0 as *mut Command
            };
            if 0 != 0 || 0 != 0 {
                return 0;
            }
            if !((*s).commands_).is_null() {
                memcpy(
                    new_commands as *mut libc::c_void,
                    (*s).commands_ as *const libc::c_void,
                    (::std::mem::size_of::<Command>() as u64).wrapping_mul((*s).num_commands_),
                );
                BrotliFree(m, (*s).commands_ as *mut libc::c_void);
                let ref mut fresh89 = (*s).commands_;
                *fresh89 = 0 as *mut Command;
            }
            let ref mut fresh90 = (*s).commands_;
            *fresh90 = new_commands;
        }
        InitOrStitchToPreviousBlock(
            m,
            &mut (*s).hasher_,
            data,
            mask as u64,
            &mut (*s).params,
            wrapped_last_processed_pos as u64,
            bytes as u64,
            is_last,
        );
        literal_context_mode = ChooseContextMode(
            &mut (*s).params,
            data,
            WrapPosition((*s).last_flush_pos_) as u64,
            mask as u64,
            ((*s).input_pos_).wrapping_sub((*s).last_flush_pos_),
        );
        literal_context_lut = &*_kBrotliContextLookupTable
            .as_ptr()
            .offset(((literal_context_mode as u32) << 9i32) as isize)
            as *const u8;
        if 0 != 0 {
            return 0;
        }
        if (*s).num_commands_ != 0 && (*s).last_insert_len_ == 0 {
            ExtendLastCommand(s, &mut bytes, &mut wrapped_last_processed_pos);
        }
        if (*s).params.quality == 10 {
            BrotliCreateZopfliBackwardReferences(
                m,
                bytes as u64,
                wrapped_last_processed_pos as u64,
                data,
                mask as u64,
                literal_context_lut,
                &mut (*s).params,
                &mut (*s).hasher_,
                ((*s).dist_cache_).as_mut_ptr(),
                &mut (*s).last_insert_len_,
                &mut *((*s).commands_).offset((*s).num_commands_ as isize),
                &mut (*s).num_commands_,
                &mut (*s).num_literals_,
            );
            if 0 != 0 {
                return 0;
            }
        } else if (*s).params.quality == 11 {
            BrotliCreateHqZopfliBackwardReferences(
                m,
                bytes as u64,
                wrapped_last_processed_pos as u64,
                data,
                mask as u64,
                literal_context_lut,
                &mut (*s).params,
                &mut (*s).hasher_,
                ((*s).dist_cache_).as_mut_ptr(),
                &mut (*s).last_insert_len_,
                &mut *((*s).commands_).offset((*s).num_commands_ as isize),
                &mut (*s).num_commands_,
                &mut (*s).num_literals_,
            );
            if 0 != 0 {
                return 0;
            }
        } else {
            BrotliCreateBackwardReferences(
                bytes as u64,
                wrapped_last_processed_pos as u64,
                data,
                mask as u64,
                literal_context_lut,
                &mut (*s).params,
                &mut (*s).hasher_,
                ((*s).dist_cache_).as_mut_ptr(),
                &mut (*s).last_insert_len_,
                &mut *((*s).commands_).offset((*s).num_commands_ as isize),
                &mut (*s).num_commands_,
                &mut (*s).num_literals_,
            );
        }
        let max_length = MaxMetablockSize(&mut (*s).params);
        let max_literals = max_length.wrapping_div(8);
        let max_commands = max_length.wrapping_div(8);
        let processed_bytes = ((*s).input_pos_).wrapping_sub((*s).last_flush_pos_);
        let next_input_fits_metablock =
            if processed_bytes.wrapping_add(InputBlockSize(s)) <= max_length {
                1
            } else {
                0
            };
        let should_flush = if (*s).params.quality < 4
            && ((*s).num_literals_).wrapping_add((*s).num_commands_) >= 0x2fff
        {
            1
        } else {
            0
        };
        if is_last == 0
            && force_flush == 0
            && should_flush == 0
            && next_input_fits_metablock != 0
            && (*s).num_literals_ < max_literals
            && (*s).num_commands_ < max_commands
        {
            if UpdateLastProcessedPos(s) != 0 {
                HasherReset(&mut (*s).hasher_);
            }
            *out_size = 0;
            return 1;
        }
        if (*s).last_insert_len_ > 0 {
            let ref mut fresh91 = (*s).num_commands_;
            let fresh92 = *fresh91;
            *fresh91 = (*fresh91).wrapping_add(1);
            InitInsertCommand(
                &mut *((*s).commands_).offset(fresh92 as isize),
                (*s).last_insert_len_,
            );
            let ref mut fresh93 = (*s).num_literals_;
            *fresh93 = (*fresh93 as u64).wrapping_add((*s).last_insert_len_) as u64;
            (*s).last_insert_len_ = 0;
        }
        if is_last == 0 && (*s).input_pos_ == (*s).last_flush_pos_ {
            *out_size = 0;
            return 1;
        }
        let metablock_size = ((*s).input_pos_).wrapping_sub((*s).last_flush_pos_) as u32;
        let mut storage_0 = GetBrotliStorage(
            s,
            2u32.wrapping_mul(metablock_size).wrapping_add(503) as u64,
        );
        let mut storage_ix_0 = (*s).last_bytes_bits_ as u64;
        if 0 != 0 {
            return 0;
        };
        *storage_0.offset(0 as isize) = (*s).last_bytes_ as u8;
        *storage_0.offset(1 as isize) = ((*s).last_bytes_ as i32 >> 8i32) as u8;
        WriteMetaBlockInternal(
            m,
            data,
            mask as u64,
            (*s).last_flush_pos_,
            metablock_size as u64,
            is_last,
            literal_context_mode,
            &mut (*s).params,
            (*s).prev_byte_,
            (*s).prev_byte2_,
            (*s).num_literals_,
            (*s).num_commands_,
            (*s).commands_,
            ((*s).saved_dist_cache_).as_mut_ptr(),
            ((*s).dist_cache_).as_mut_ptr(),
            &mut storage_ix_0,
            storage_0,
        );
        if 0 != 0 {
            return 0;
        };
        (*s).last_bytes_ = *storage_0.offset((storage_ix_0 >> 3i32) as isize) as u16;
        (*s).last_bytes_bits_ = (storage_ix_0 & 7u64) as u8;
        (*s).last_flush_pos_ = (*s).input_pos_;
        if UpdateLastProcessedPos(s) != 0 {
            HasherReset(&mut (*s).hasher_);
        }
        if (*s).last_flush_pos_ > 0 {
            (*s).prev_byte_ =
                *data.offset((((*s).last_flush_pos_ as u32).wrapping_sub(1u32) & mask) as isize);
        }
        if (*s).last_flush_pos_ > 1 {
            (*s).prev_byte2_ =
                *data.offset((((*s).last_flush_pos_).wrapping_sub(2u64) as u32 & mask) as isize);
        };
        (*s).num_commands_ = 0;
        (*s).num_literals_ = 0;
        memcpy(
            ((*s).saved_dist_cache_).as_mut_ptr() as *mut libc::c_void,
            ((*s).dist_cache_).as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[i32; 4]>() as u64,
        );
        *output = &mut *storage_0.offset(0 as isize) as *mut u8;
        *out_size = storage_ix_0 >> 3;
        return 1;
    }
}

extern "C" fn WriteMetadataHeader(
    mut s: *mut BrotliEncoderState,
    block_size: u64,
    mut header: *mut u8,
) -> u64 {
    unsafe {
        let mut storage_ix: u64 = 0;
        storage_ix = (*s).last_bytes_bits_ as u64;
        *header.offset(0 as isize) = (*s).last_bytes_ as u8;
        *header.offset(1 as isize) = ((*s).last_bytes_ as i32 >> 8i32) as u8;
        (*s).last_bytes_ = 0;
        (*s).last_bytes_bits_ = 0;
        BrotliWriteBits(1, 0, &mut storage_ix, header);
        BrotliWriteBits(2, 3, &mut storage_ix, header);
        BrotliWriteBits(1, 0, &mut storage_ix, header);
        if block_size == 0 {
            BrotliWriteBits(2, 0, &mut storage_ix, header);
        } else {
            let mut nbits = if block_size == 1 {
                0
            } else {
                (Log2FloorNonZero((block_size as u32).wrapping_sub(1u32) as u64)).wrapping_add(1)
            };
            let mut nbytes = nbits.wrapping_add(7).wrapping_div(8);
            BrotliWriteBits(2, nbytes as u64, &mut storage_ix, header);
            BrotliWriteBits(
                8u32.wrapping_mul(nbytes) as u64,
                block_size.wrapping_sub(1),
                &mut storage_ix,
                header,
            );
        }
        return storage_ix.wrapping_add(7) >> 3;
    }
}

extern "C" fn BrotliCompressBufferQuality10(
    mut lgwin: i32,
    mut input_size: u64,
    mut input_buffer: *const u8,
    mut encoded_size: *mut u64,
    mut encoded_buffer: *mut u8,
) -> i32 {
    unsafe {
        let mut current_block: u64;
        let mut memory_manager = MemoryManager {
            alloc_func: None,
            free_func: None,
            opaque: 0 as *mut libc::c_void,
        };
        let mut m: *mut MemoryManager = &mut memory_manager;
        let mask = !0 >> 1;
        let mut dist_cache: [i32; 4] = [4, 11, 15, 16];
        let mut saved_dist_cache: [i32; 4] = [4, 11, 15, 16];
        let mut ok = 1;
        let max_out_size = *encoded_size;
        let mut total_out_size = 0;
        let mut last_bytes: u16 = 0;
        let mut last_bytes_bits: u8 = 0;
        let hasher_eff_size = brotli_min_size_t(
            input_size,
            (1u64 << lgwin).wrapping_sub(16).wrapping_add(16),
        );
        let mut params = BrotliEncoderParams {
            mode: BROTLI_MODE_GENERIC,
            quality: 0,
            lgwin: 0,
            lgblock: 0,
            stream_offset: 0,
            size_hint: 0,
            disable_literal_context_modeling: 0,
            large_window: 0,
            hasher: BrotliHasherParams {
                type_0: 0,
                bucket_bits: 0,
                block_bits: 0,
                hash_len: 0,
                num_last_distances_to_check: 0,
            },
            dist: BrotliDistanceParams {
                distance_postfix_bits: 0,
                num_direct_distance_codes: 0,
                alphabet_size_max: 0,
                alphabet_size_limit: 0,
                max_distance: 0,
            },
            dictionary: BrotliEncoderDictionary {
                words: 0 as *const BrotliDictionary,
                num_transforms: 0,
                cutoffTransformsCount: 0,
                cutoffTransforms: 0,
                hash_table_words: 0 as *const u16,
                hash_table_lengths: 0 as *const u8,
                buckets: 0 as *const u16,
                dict_words: 0 as *const DictWord,
            },
        };
        let lgmetablock = brotli_min_int(24, lgwin + 1);
        let mut max_block_size: u64 = 0;
        let max_metablock_size = 1 << lgmetablock;
        let max_literals_per_metablock = max_metablock_size.wrapping_div(8);
        let max_commands_per_metablock = max_metablock_size.wrapping_div(8);
        let mut metablock_start = 0;
        let mut prev_byte = 0;
        let mut prev_byte2 = 0;
        let mut hasher = Hasher {
            common: HasherCommon {
                extra: 0 as *mut libc::c_void,
                dict_num_lookups: 0,
                dict_num_matches: 0,
                params: BrotliHasherParams {
                    type_0: 0,
                    bucket_bits: 0,
                    block_bits: 0,
                    hash_len: 0,
                    num_last_distances_to_check: 0,
                },
                is_prepared_: 0,
            },
            privat: C2RustUnnamed_0 {
                _H2: H2 {
                    common: 0 as *mut HasherCommon,
                    buckets_: 0 as *mut u32,
                },
            },
        };
        HasherInit(&mut hasher);
        BrotliEncoderInitParams(&mut params);
        params.quality = 10;
        params.lgwin = lgwin;
        if lgwin > 24 {
            params.large_window = 1;
        }
        SanitizeParams(&mut params);
        params.lgblock = ComputeLgBlock(&mut params);
        ChooseDistanceParams(&mut params);
        max_block_size = 1 << params.lgblock;
        BrotliInitMemoryManager(m, None, None, 0 as *mut libc::c_void);
        EncodeWindowBits(
            lgwin,
            params.large_window,
            &mut last_bytes,
            &mut last_bytes_bits,
        );
        InitOrStitchToPreviousBlock(
            m,
            &mut hasher,
            input_buffer,
            mask,
            &mut params,
            0,
            hasher_eff_size,
            1,
        );
        if 0 != 0 {
            current_block = 16891123271837848903;
        } else {
            current_block = 4068382217303356765;
        }
        'c_20643: loop {
            match current_block {
                16891123271837848903 => {
                    BrotliWipeOutMemoryManager(m);
                    return 0;
                }
                _ => {
                    if ok != 0 && metablock_start < input_size {
                        let metablock_end = brotli_min_size_t(
                            input_size,
                            metablock_start.wrapping_add(max_metablock_size),
                        );
                        let expected_num_commands = metablock_end
                            .wrapping_sub(metablock_start)
                            .wrapping_div(12)
                            .wrapping_add(16);
                        let mut commands = 0 as *mut Command;
                        let mut num_commands = 0;
                        let mut last_insert_len = 0;
                        let mut num_literals = 0;
                        let mut metablock_size = 0;
                        let mut cmd_alloc_size = 0;
                        let mut is_last: i32 = 0;
                        let mut storage = 0 as *mut u8;
                        let mut storage_ix: u64 = 0;
                        let mut literal_context_mode = ChooseContextMode(
                            &mut params,
                            input_buffer,
                            metablock_start,
                            mask,
                            metablock_end.wrapping_sub(metablock_start),
                        );
                        let mut literal_context_lut: ContextLut = &*_kBrotliContextLookupTable
                            .as_ptr()
                            .offset(((literal_context_mode as u32) << 9i32) as isize)
                            as *const u8;
                        let mut block_start: u64 = 0;
                        block_start = metablock_start;
                        while block_start < metablock_end {
                            let mut block_size = brotli_min_size_t(
                                metablock_end.wrapping_sub(block_start),
                                max_block_size,
                            );
                            let mut nodes = if block_size.wrapping_add(1) > 0 {
                                BrotliAllocate(
                                    m,
                                    block_size
                                        .wrapping_add(1)
                                        .wrapping_mul(::std::mem::size_of::<ZopfliNode>() as u64),
                                ) as *mut ZopfliNode
                            } else {
                                0 as *mut ZopfliNode
                            };
                            let mut path_size: u64 = 0;
                            let mut new_cmd_alloc_size: u64 = 0;
                            if 0 != 0 || 0 != 0 {
                                current_block = 16891123271837848903;
                                continue 'c_20643;
                            }
                            BrotliInitZopfliNodes(nodes, block_size.wrapping_add(1));
                            StitchToPreviousBlockH10(
                                &mut hasher.privat._H10,
                                block_size,
                                block_start,
                                input_buffer,
                                mask,
                            );
                            path_size = BrotliZopfliComputeShortestPath(
                                m,
                                block_size,
                                block_start,
                                input_buffer,
                                mask,
                                literal_context_lut,
                                &mut params,
                                dist_cache.as_mut_ptr(),
                                &mut hasher,
                                nodes,
                            );
                            if 0 != 0 {
                                current_block = 16891123271837848903;
                                continue 'c_20643;
                            }
                            new_cmd_alloc_size = brotli_max_size_t(
                                expected_num_commands,
                                num_commands.wrapping_add(path_size).wrapping_add(1),
                            );
                            if cmd_alloc_size != new_cmd_alloc_size {
                                let mut new_commands = if new_cmd_alloc_size > 0 {
                                    BrotliAllocate(
                                        m,
                                        new_cmd_alloc_size
                                            .wrapping_mul(::std::mem::size_of::<Command>() as u64),
                                    ) as *mut Command
                                } else {
                                    0 as *mut Command
                                };
                                if 0 != 0 || 0 != 0 {
                                    current_block = 16891123271837848903;
                                    continue 'c_20643;
                                }
                                cmd_alloc_size = new_cmd_alloc_size;
                                if !commands.is_null() {
                                    memcpy(
                                        new_commands as *mut libc::c_void,
                                        commands as *const libc::c_void,
                                        (::std::mem::size_of::<Command>() as u64)
                                            .wrapping_mul(num_commands),
                                    );
                                    BrotliFree(m, commands as *mut libc::c_void);
                                    commands = 0 as *mut Command;
                                }
                                commands = new_commands;
                            }
                            BrotliZopfliCreateCommands(
                                block_size,
                                block_start,
                                &mut *nodes.offset(0 as isize),
                                dist_cache.as_mut_ptr(),
                                &mut last_insert_len,
                                &mut params,
                                &mut *commands.offset(num_commands as isize),
                                &mut num_literals,
                            );
                            num_commands = (num_commands as u64).wrapping_add(path_size) as u64;
                            block_start = (block_start).wrapping_add(block_size) as u64;
                            metablock_size =
                                (metablock_size as u64).wrapping_add(block_size) as u64;
                            BrotliFree(m, nodes as *mut libc::c_void);
                            nodes = 0 as *mut ZopfliNode;
                            if num_literals > max_literals_per_metablock
                                || num_commands > max_commands_per_metablock
                            {
                                break;
                            }
                        }
                        if last_insert_len > 0 {
                            let fresh94 = num_commands;
                            num_commands = num_commands.wrapping_add(1);
                            InitInsertCommand(
                                &mut *commands.offset(fresh94 as isize),
                                last_insert_len,
                            );
                            num_literals =
                                (num_literals as u64).wrapping_add(last_insert_len) as u64;
                        }
                        is_last = if metablock_start.wrapping_add(metablock_size) == input_size {
                            1
                        } else {
                            0
                        };
                        storage = 0 as *mut u8;
                        storage_ix = last_bytes_bits as u64;
                        if metablock_size == 0 {
                            storage = if 16 > 0 {
                                BrotliAllocate(
                                    m,
                                    16u64.wrapping_mul(::std::mem::size_of::<u8>() as u64),
                                ) as *mut u8
                            } else {
                                0 as *mut u8
                            };
                            if 0 != 0 || 0 != 0 {
                                current_block = 16891123271837848903;
                                continue;
                            };
                            *storage.offset(0 as isize) = last_bytes as u8;
                            *storage.offset(1 as isize) = (last_bytes as i32 >> 8i32) as u8;
                            BrotliWriteBits(2, 3, &mut storage_ix, storage);
                            storage_ix = storage_ix.wrapping_add(7) & !7 as u64;
                        } else if ShouldCompress(
                            input_buffer,
                            mask,
                            metablock_start,
                            metablock_size,
                            num_literals,
                            num_commands,
                        ) == 0
                        {
                            memcpy(
                                dist_cache.as_mut_ptr() as *mut libc::c_void,
                                saved_dist_cache.as_mut_ptr() as *const libc::c_void,
                                4u64.wrapping_mul(::std::mem::size_of::<i32>() as u64),
                            );
                            storage = if metablock_size.wrapping_add(16) > 0 {
                                BrotliAllocate(
                                    m,
                                    metablock_size
                                        .wrapping_add(16)
                                        .wrapping_mul(::std::mem::size_of::<u8>() as u64),
                                ) as *mut u8
                            } else {
                                0 as *mut u8
                            };
                            if 0 != 0 || 0 != 0 {
                                current_block = 16891123271837848903;
                                continue;
                            };
                            *storage.offset(0 as isize) = last_bytes as u8;
                            *storage.offset(1 as isize) = (last_bytes as i32 >> 8i32) as u8;
                            BrotliStoreUncompressedMetaBlock(
                                is_last,
                                input_buffer,
                                metablock_start,
                                mask,
                                metablock_size,
                                &mut storage_ix,
                                storage,
                            );
                        } else {
                            let mut mb = MetaBlockSplit {
                                literal_split: BlockSplit {
                                    num_types: 0,
                                    num_blocks: 0,
                                    types: 0 as *mut u8,
                                    lengths: 0 as *mut u32,
                                    types_alloc_size: 0,
                                    lengths_alloc_size: 0,
                                },
                                command_split: BlockSplit {
                                    num_types: 0,
                                    num_blocks: 0,
                                    types: 0 as *mut u8,
                                    lengths: 0 as *mut u32,
                                    types_alloc_size: 0,
                                    lengths_alloc_size: 0,
                                },
                                distance_split: BlockSplit {
                                    num_types: 0,
                                    num_blocks: 0,
                                    types: 0 as *mut u8,
                                    lengths: 0 as *mut u32,
                                    types_alloc_size: 0,
                                    lengths_alloc_size: 0,
                                },
                                literal_context_map: 0 as *mut u32,
                                literal_context_map_size: 0,
                                distance_context_map: 0 as *mut u32,
                                distance_context_map_size: 0,
                                literal_histograms: 0 as *mut HistogramLiteral,
                                literal_histograms_size: 0,
                                command_histograms: 0 as *mut HistogramCommand,
                                command_histograms_size: 0,
                                distance_histograms: 0 as *mut HistogramDistance,
                                distance_histograms_size: 0,
                            };
                            let mut block_params = params;
                            InitMetaBlockSplit(&mut mb);
                            BrotliBuildMetaBlock(
                                m,
                                input_buffer,
                                metablock_start,
                                mask,
                                &mut block_params,
                                prev_byte,
                                prev_byte2,
                                commands,
                                num_commands,
                                literal_context_mode,
                                &mut mb,
                            );
                            if 0 != 0 {
                                current_block = 16891123271837848903;
                                continue;
                            }
                            BrotliOptimizeHistograms(
                                block_params.dist.alphabet_size_limit,
                                &mut mb,
                            );
                            storage = if 2u64.wrapping_mul(metablock_size).wrapping_add(503) > 0 {
                                BrotliAllocate(
                                    m,
                                    2u64.wrapping_mul(metablock_size)
                                        .wrapping_add(503)
                                        .wrapping_mul(::std::mem::size_of::<u8>() as u64),
                                ) as *mut u8
                            } else {
                                0 as *mut u8
                            };
                            if 0 != 0 || 0 != 0 {
                                current_block = 16891123271837848903;
                                continue;
                            };
                            *storage.offset(0 as isize) = last_bytes as u8;
                            *storage.offset(1 as isize) = (last_bytes as i32 >> 8i32) as u8;
                            BrotliStoreMetaBlock(
                                m,
                                input_buffer,
                                metablock_start,
                                metablock_size,
                                mask,
                                prev_byte,
                                prev_byte2,
                                is_last,
                                &mut block_params,
                                literal_context_mode,
                                commands,
                                num_commands,
                                &mut mb,
                                &mut storage_ix,
                                storage,
                            );
                            if 0 != 0 {
                                current_block = 16891123271837848903;
                                continue;
                            }
                            if metablock_size.wrapping_add(4) < storage_ix >> 3 {
                                memcpy(
                                    dist_cache.as_mut_ptr() as *mut libc::c_void,
                                    saved_dist_cache.as_mut_ptr() as *const libc::c_void,
                                    4u64.wrapping_mul(::std::mem::size_of::<i32>() as u64),
                                );
                                *storage.offset(0 as isize) = last_bytes as u8;
                                *storage.offset(1 as isize) = (last_bytes as i32 >> 8i32) as u8;
                                storage_ix = last_bytes_bits as u64;
                                BrotliStoreUncompressedMetaBlock(
                                    is_last,
                                    input_buffer,
                                    metablock_start,
                                    mask,
                                    metablock_size,
                                    &mut storage_ix,
                                    storage,
                                );
                            }
                            DestroyMetaBlockSplit(m, &mut mb);
                        }
                        last_bytes = *storage.offset((storage_ix >> 3i32) as isize) as u16;
                        last_bytes_bits = (storage_ix & 7u64) as u8;
                        metablock_start =
                            (metablock_start as u64).wrapping_add(metablock_size) as u64;
                        if metablock_start < input_size {
                            prev_byte =
                                *input_buffer.offset(metablock_start.wrapping_sub(1) as isize);
                            prev_byte2 =
                                *input_buffer.offset(metablock_start.wrapping_sub(2) as isize);
                        }
                        memcpy(
                            saved_dist_cache.as_mut_ptr() as *mut libc::c_void,
                            dist_cache.as_mut_ptr() as *const libc::c_void,
                            4u64.wrapping_mul(::std::mem::size_of::<i32>() as u64),
                        );
                        let out_size = storage_ix >> 3;
                        total_out_size = (total_out_size as u64).wrapping_add(out_size) as u64;
                        if total_out_size <= max_out_size {
                            memcpy(
                                encoded_buffer as *mut libc::c_void,
                                storage as *const libc::c_void,
                                out_size,
                            );
                            encoded_buffer = encoded_buffer.offset(out_size as isize);
                        } else {
                            ok = 0;
                        }
                        BrotliFree(m, storage as *mut libc::c_void);
                        storage = 0 as *mut u8;
                        BrotliFree(m, commands as *mut libc::c_void);
                        commands = 0 as *mut Command;
                        current_block = 4068382217303356765;
                    } else {
                        *encoded_size = total_out_size;
                        DestroyHasher(m, &mut hasher);
                        return ok;
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliEncoderMaxCompressedSize(mut input_size: u64) -> u64 {
    let mut num_large_blocks = input_size >> 14;
    let mut overhead = 2u64
        .wrapping_add(4u64.wrapping_mul(num_large_blocks))
        .wrapping_add(3)
        .wrapping_add(1);
    let mut result = input_size.wrapping_add(overhead);
    if input_size == 0 {
        return 2;
    }
    return if result < input_size { 0 } else { result };
}

extern "C" fn MakeUncompressedStream(
    mut input: *const u8,
    mut input_size: u64,
    mut output: *mut u8,
) -> u64 {
    unsafe {
        let mut size = input_size;
        let mut result = 0;
        let mut offset = 0;
        if input_size == 0 {
            *output.offset(0 as isize) = 6;
            return 1;
        }
        let fresh95 = result;
        result = result.wrapping_add(1);
        *output.offset(fresh95 as isize) = 0x21;
        let fresh96 = result;
        result = result.wrapping_add(1);
        *output.offset(fresh96 as isize) = 0x3;
        while size > 0 {
            let mut nibbles = 0;
            let mut chunk_size: u32 = 0;
            let mut bits: u32 = 0;
            chunk_size = if size > (1u32 << 24) as u64 {
                1 << 24
            } else {
                size as u32
            };
            if chunk_size > 1 << 16 {
                nibbles = (if chunk_size > 1u32 << 20 { 2 } else { 1 }) as u32;
            }
            bits = nibbles << 1
                | chunk_size.wrapping_sub(1) << 3
                | 1u32 << 19u32.wrapping_add(4u32.wrapping_mul(nibbles));
            let fresh97 = result;
            result = result.wrapping_add(1);
            *output.offset(fresh97 as isize) = bits as u8;
            let fresh98 = result;
            result = result.wrapping_add(1);
            *output.offset(fresh98 as isize) = (bits >> 8i32) as u8;
            let fresh99 = result;
            result = result.wrapping_add(1);
            *output.offset(fresh99 as isize) = (bits >> 16i32) as u8;
            if nibbles == 2 {
                let fresh100 = result;
                result = result.wrapping_add(1);
                *output.offset(fresh100 as isize) = (bits >> 24i32) as u8;
            }
            memcpy(
                &mut *output.offset(result as isize) as *mut u8 as *mut libc::c_void,
                &*input.offset(offset as isize) as *const u8 as *const libc::c_void,
                chunk_size as u64,
            );
            result = (result as u64).wrapping_add(chunk_size as u64) as u64;
            offset = (offset as u64).wrapping_add(chunk_size as u64) as u64;
            size = (size as u64).wrapping_sub(chunk_size as u64) as u64;
        }
        let fresh101 = result;
        result = result.wrapping_add(1);
        *output.offset(fresh101 as isize) = 3;
        return result;
    }
}

#[no_mangle]
pub extern "C" fn BrotliEncoderCompress(
    mut quality: i32,
    mut lgwin: i32,
    mut mode: u32,
    mut input_size: u64,
    mut input_buffer: *const u8,
    mut encoded_size: *mut u64,
    mut encoded_buffer: *mut u8,
) -> i32 {
    unsafe {
        let mut s = 0 as *mut BrotliEncoderState;
        let mut out_size = *encoded_size;
        let mut input_start = input_buffer;
        let mut output_start = encoded_buffer;
        let mut max_out_size = BrotliEncoderMaxCompressedSize(input_size);
        if out_size == 0 {
            return 0;
        }
        if input_size == 0 {
            *encoded_size = 1;
            *encoded_buffer = 6;
            return 1;
        }
        if quality == 10 {
            let lg_win = brotli_min_int(30, brotli_max_int(16, lgwin));
            let mut ok = BrotliCompressBufferQuality10(
                lg_win,
                input_size,
                input_buffer,
                encoded_size,
                encoded_buffer,
            );
            if !(ok == 0 || max_out_size != 0 && *encoded_size > max_out_size) {
                return 1;
            }
        } else {
            s = BrotliEncoderCreateInstance(None, None, 0 as *mut libc::c_void);
            if s.is_null() {
                return 0;
            } else {
                let mut available_in = input_size;
                let mut next_in = input_buffer;
                let mut available_out = *encoded_size;
                let mut next_out = encoded_buffer;
                let mut total_out = 0;
                let mut result = 0;
                BrotliEncoderSetParameter(s, BROTLI_PARAM_QUALITY, quality as u32);
                BrotliEncoderSetParameter(s, BROTLI_PARAM_LGWIN, lgwin as u32);
                BrotliEncoderSetParameter(s, BROTLI_PARAM_MODE, mode as u32);
                BrotliEncoderSetParameter(s, BROTLI_PARAM_SIZE_HINT, input_size as u32);
                if lgwin > 24 {
                    BrotliEncoderSetParameter(s, BROTLI_PARAM_LARGE_WINDOW, 1);
                }
                result = BrotliEncoderCompressStream(
                    s,
                    BROTLI_OPERATION_FINISH,
                    &mut available_in,
                    &mut next_in,
                    &mut available_out,
                    &mut next_out,
                    &mut total_out,
                );
                if BrotliEncoderIsFinished(s) == 0 {
                    result = 0;
                }
                *encoded_size = total_out;
                BrotliEncoderDestroyInstance(s);
                if !(result == 0 || max_out_size != 0 && *encoded_size > max_out_size) {
                    return 1;
                }
            }
        }
        *encoded_size = 0;
        if max_out_size == 0 {
            return 0;
        }
        if out_size >= max_out_size {
            *encoded_size = MakeUncompressedStream(input_start, input_size, output_start);
            return 1;
        }
        return 0;
    }
}

extern "C" fn InjectBytePaddingBlock(mut s: *mut BrotliEncoderState) {
    unsafe {
        let mut seal = (*s).last_bytes_ as u32;
        let mut seal_bits = (*s).last_bytes_bits_ as u64;
        let mut destination = 0 as *mut u8;
        (*s).last_bytes_ = 0;
        (*s).last_bytes_bits_ = 0;
        seal |= 0x6 << seal_bits;
        seal_bits = (seal_bits as u64).wrapping_add(6) as u64;
        if !((*s).next_out_).is_null() {
            destination = ((*s).next_out_).offset((*s).available_out_ as isize);
        } else {
            destination = ((*s).tiny_buf_.u8_0).as_mut_ptr();
            let ref mut fresh102 = (*s).next_out_;
            *fresh102 = destination;
        };
        *destination.offset(0 as isize) = seal as u8;
        if seal_bits > 8 {
            *destination.offset(1 as isize) = (seal >> 8i32) as u8;
        }
        if seal_bits > 16 {
            *destination.offset(2 as isize) = (seal >> 16i32) as u8;
        }
        let ref mut fresh103 = (*s).available_out_;
        *fresh103 = (*fresh103 as u64).wrapping_add(seal_bits.wrapping_add(7) >> 3) as u64;
    }
}

extern "C" fn InjectFlushOrPushOutput(
    mut s: *mut BrotliEncoderState,
    mut available_out: *mut u64,
    mut next_out: *mut *mut u8,
    mut total_out: *mut u64,
) -> i32 {
    unsafe {
        if (*s).stream_state_ as u32 == BROTLI_STREAM_FLUSH_REQUESTED as u32
            && (*s).last_bytes_bits_ as i32 != 0
        {
            InjectBytePaddingBlock(s);
            return 1;
        }
        if (*s).available_out_ != 0 && *available_out != 0 {
            let mut copy_output_size = brotli_min_size_t((*s).available_out_, *available_out);
            memcpy(
                *next_out as *mut libc::c_void,
                (*s).next_out_ as *const libc::c_void,
                copy_output_size,
            );
            *next_out = (*next_out).offset(copy_output_size as isize);
            *available_out = (*available_out as u64).wrapping_sub(copy_output_size) as u64;
            let ref mut fresh104 = (*s).next_out_;
            *fresh104 = (*fresh104).offset(copy_output_size as isize);
            let ref mut fresh105 = (*s).available_out_;
            *fresh105 = (*fresh105 as u64).wrapping_sub(copy_output_size) as u64;
            let ref mut fresh106 = (*s).total_out_;
            *fresh106 = (*fresh106 as u64).wrapping_add(copy_output_size) as u64;
            if !total_out.is_null() {
                *total_out = (*s).total_out_;
            }
            return 1;
        }
        return 0;
    }
}

extern "C" fn CheckFlushComplete(mut s: *mut BrotliEncoderState) {
    unsafe {
        if (*s).stream_state_ as u32 == BROTLI_STREAM_FLUSH_REQUESTED as u32
            && (*s).available_out_ == 0
        {
            (*s).stream_state_ = BROTLI_STREAM_PROCESSING;
            let ref mut fresh107 = (*s).next_out_;
            *fresh107 = 0 as *mut u8;
        }
    }
}

extern "C" fn BrotliEncoderCompressStreamFast(
    mut s: *mut BrotliEncoderState,
    mut op: u32,
    mut available_in: *mut u64,
    mut next_in: *mut *const u8,
    mut available_out: *mut u64,
    mut next_out: *mut *mut u8,
    mut total_out: *mut u64,
) -> i32 {
    unsafe {
        let block_size_limit = 1 << (*s).params.lgwin;
        let buf_size = brotli_min_size_t(
            kCompressFragmentTwoPassBlockSize,
            brotli_min_size_t(*available_in, block_size_limit),
        );
        let mut tmp_command_buf = 0 as *mut u32;
        let mut command_buf = 0 as *mut u32;
        let mut tmp_literal_buf = 0 as *mut u8;
        let mut literal_buf = 0 as *mut u8;
        let mut m: *mut MemoryManager = &mut (*s).memory_manager_;
        if (*s).params.quality != 0 && (*s).params.quality != 1 {
            return 0;
        }
        if (*s).params.quality == 1 {
            if ((*s).command_buf_).is_null() && buf_size == kCompressFragmentTwoPassBlockSize {
                let ref mut fresh108 = (*s).command_buf_;
                *fresh108 = if kCompressFragmentTwoPassBlockSize > 0 {
                    BrotliAllocate(
                        m,
                        kCompressFragmentTwoPassBlockSize
                            .wrapping_mul(::std::mem::size_of::<u32>() as u64),
                    ) as *mut u32
                } else {
                    0 as *mut u32
                };
                let ref mut fresh109 = (*s).literal_buf_;
                *fresh109 = if kCompressFragmentTwoPassBlockSize > 0 {
                    BrotliAllocate(
                        m,
                        kCompressFragmentTwoPassBlockSize
                            .wrapping_mul(::std::mem::size_of::<u8>() as u64),
                    ) as *mut u8
                } else {
                    0 as *mut u8
                };
                if 0 != 0 || 0 != 0 || 0 != 0 {
                    return 0;
                }
            }
            if !((*s).command_buf_).is_null() {
                command_buf = (*s).command_buf_;
                literal_buf = (*s).literal_buf_;
            } else {
                tmp_command_buf = if buf_size > 0 {
                    BrotliAllocate(
                        m,
                        buf_size.wrapping_mul(::std::mem::size_of::<u32>() as u64),
                    ) as *mut u32
                } else {
                    0 as *mut u32
                };
                tmp_literal_buf = if buf_size > 0 {
                    BrotliAllocate(m, buf_size.wrapping_mul(::std::mem::size_of::<u8>() as u64))
                        as *mut u8
                } else {
                    0 as *mut u8
                };
                if 0 != 0 || 0 != 0 || 0 != 0 {
                    return 0;
                }
                command_buf = tmp_command_buf;
                literal_buf = tmp_literal_buf;
            }
        }
        loop {
            if InjectFlushOrPushOutput(s, available_out, next_out, total_out) != 0 {
                continue;
            }
            if !((*s).available_out_ == 0
                && (*s).stream_state_ as u32 == BROTLI_STREAM_PROCESSING as u32
                && (*available_in != 0 || op as u32 != BROTLI_OPERATION_PROCESS as u32))
            {
                break;
            }
            let mut block_size = brotli_min_size_t(block_size_limit, *available_in);
            let mut is_last =
                (*available_in == block_size && op as u32 == BROTLI_OPERATION_FINISH as u32) as i32;
            let mut force_flush =
                (*available_in == block_size && op as u32 == BROTLI_OPERATION_FLUSH as u32) as i32;
            let mut max_out_size = 2u64.wrapping_mul(block_size).wrapping_add(503);
            let mut inplace = 1;
            let mut storage = 0 as *mut u8;
            let mut storage_ix = (*s).last_bytes_bits_ as u64;
            let mut table_size: u64 = 0;
            let mut table = 0 as *mut i32;
            if force_flush != 0 && block_size == 0 {
                (*s).stream_state_ = BROTLI_STREAM_FLUSH_REQUESTED;
            } else {
                if max_out_size <= *available_out {
                    storage = *next_out;
                } else {
                    inplace = 0;
                    storage = GetBrotliStorage(s, max_out_size);
                    if 0 != 0 {
                        return 0;
                    }
                };
                *storage.offset(0 as isize) = (*s).last_bytes_ as u8;
                *storage.offset(1 as isize) = ((*s).last_bytes_ as i32 >> 8i32) as u8;
                table = GetHashTable(s, (*s).params.quality, block_size, &mut table_size);
                if 0 != 0 {
                    return 0;
                }
                if (*s).params.quality == 0 {
                    BrotliCompressFragmentFast(
                        m,
                        *next_in,
                        block_size,
                        is_last,
                        table,
                        table_size,
                        ((*s).cmd_depths_).as_mut_ptr(),
                        ((*s).cmd_bits_).as_mut_ptr(),
                        &mut (*s).cmd_code_numbits_,
                        ((*s).cmd_code_).as_mut_ptr(),
                        &mut storage_ix,
                        storage,
                    );
                    if 0 != 0 {
                        return 0;
                    }
                } else {
                    BrotliCompressFragmentTwoPass(
                        m,
                        *next_in,
                        block_size,
                        is_last,
                        command_buf,
                        literal_buf,
                        table,
                        table_size,
                        &mut storage_ix,
                        storage,
                    );
                    if 0 != 0 {
                        return 0;
                    }
                }
                if block_size != 0 {
                    *next_in = (*next_in).offset(block_size as isize);
                    *available_in = (*available_in as u64).wrapping_sub(block_size) as u64;
                }
                if inplace != 0 {
                    let mut out_bytes = storage_ix >> 3;
                    *next_out = (*next_out).offset(out_bytes as isize);
                    *available_out = (*available_out as u64).wrapping_sub(out_bytes) as u64;
                    let ref mut fresh110 = (*s).total_out_;
                    *fresh110 = (*fresh110 as u64).wrapping_add(out_bytes) as u64;
                    if !total_out.is_null() {
                        *total_out = (*s).total_out_;
                    }
                } else {
                    let mut out_bytes_0 = storage_ix >> 3;
                    let ref mut fresh111 = (*s).next_out_;
                    *fresh111 = storage;
                    (*s).available_out_ = out_bytes_0;
                };
                (*s).last_bytes_ = *storage.offset((storage_ix >> 3i32) as isize) as u16;
                (*s).last_bytes_bits_ = (storage_ix & 7u64) as u8;
                if force_flush != 0 {
                    (*s).stream_state_ = BROTLI_STREAM_FLUSH_REQUESTED;
                }
                if is_last != 0 {
                    (*s).stream_state_ = BROTLI_STREAM_FINISHED;
                }
            }
        }
        BrotliFree(m, tmp_command_buf as *mut libc::c_void);
        tmp_command_buf = 0 as *mut u32;
        BrotliFree(m, tmp_literal_buf as *mut libc::c_void);
        tmp_literal_buf = 0 as *mut u8;
        CheckFlushComplete(s);
        return 1;
    }
}

extern "C" fn ProcessMetadata(
    mut s: *mut BrotliEncoderState,
    mut available_in: *mut u64,
    mut next_in: *mut *const u8,
    mut available_out: *mut u64,
    mut next_out: *mut *mut u8,
    mut total_out: *mut u64,
) -> i32 {
    unsafe {
        if *available_in > (1u32 << 24) as u64 {
            return 0;
        }
        if (*s).stream_state_ as u32 == BROTLI_STREAM_PROCESSING as u32 {
            (*s).remaining_metadata_bytes_ = *available_in as u32;
            (*s).stream_state_ = BROTLI_STREAM_METADATA_HEAD;
        }
        if (*s).stream_state_ as u32 != BROTLI_STREAM_METADATA_HEAD as u32
            && (*s).stream_state_ as u32 != BROTLI_STREAM_METADATA_BODY as u32
        {
            return 0;
        }
        loop {
            if InjectFlushOrPushOutput(s, available_out, next_out, total_out) != 0 {
                continue;
            }
            if (*s).available_out_ != 0 {
                break;
            }
            if (*s).input_pos_ != (*s).last_flush_pos_ {
                let mut result = EncodeData(s, 0, 1, &mut (*s).available_out_, &mut (*s).next_out_);
                if result == 0 {
                    return 0;
                }
            } else if (*s).stream_state_ as u32 == BROTLI_STREAM_METADATA_HEAD as u32 {
                let ref mut fresh112 = (*s).next_out_;
                *fresh112 = ((*s).tiny_buf_.u8_0).as_mut_ptr();
                (*s).available_out_ =
                    WriteMetadataHeader(s, (*s).remaining_metadata_bytes_ as u64, (*s).next_out_);
                (*s).stream_state_ = BROTLI_STREAM_METADATA_BODY;
            } else if (*s).remaining_metadata_bytes_ == 0 {
                (*s).remaining_metadata_bytes_ = !0;
                (*s).stream_state_ = BROTLI_STREAM_PROCESSING;
                break;
            } else if *available_out != 0 {
                let mut copy =
                    brotli_min_size_t((*s).remaining_metadata_bytes_ as u64, *available_out) as u32;
                memcpy(
                    *next_out as *mut libc::c_void,
                    *next_in as *const libc::c_void,
                    copy as u64,
                );
                *next_in = (*next_in).offset(copy as isize);
                *available_in = (*available_in as u64).wrapping_sub(copy as u64) as u64;
                let ref mut fresh113 = (*s).remaining_metadata_bytes_;
                *fresh113 = (*fresh113 as u32).wrapping_sub(copy) as u32;
                *next_out = (*next_out).offset(copy as isize);
                *available_out = (*available_out as u64).wrapping_sub(copy as u64) as u64;
            } else {
                let mut copy_0 = brotli_min_uint32_t((*s).remaining_metadata_bytes_, 16);
                let ref mut fresh114 = (*s).next_out_;
                *fresh114 = ((*s).tiny_buf_.u8_0).as_mut_ptr();
                memcpy(
                    (*s).next_out_ as *mut libc::c_void,
                    *next_in as *const libc::c_void,
                    copy_0 as u64,
                );
                *next_in = (*next_in).offset(copy_0 as isize);
                *available_in = (*available_in as u64).wrapping_sub(copy_0 as u64) as u64;
                let ref mut fresh115 = (*s).remaining_metadata_bytes_;
                *fresh115 = (*fresh115 as u32).wrapping_sub(copy_0) as u32;
                (*s).available_out_ = copy_0 as u64;
            }
        }
        return 1;
    }
}

extern "C" fn UpdateSizeHint(mut s: *mut BrotliEncoderState, mut available_in: u64) {
    unsafe {
        if (*s).params.size_hint == 0 {
            let mut delta = UnprocessedInputSize(s);
            let mut tail = available_in;
            let mut limit = 1 << 30;
            let mut total: u32 = 0;
            if delta >= limit as u64
                || tail >= limit as u64
                || delta.wrapping_add(tail) >= limit as u64
            {
                total = limit;
            } else {
                total = delta.wrapping_add(tail) as u32;
            };
            (*s).params.size_hint = total as u64;
        }
    }
}

#[no_mangle]
pub extern "C" fn BrotliEncoderCompressStream(
    mut s: *mut BrotliEncoderState,
    mut op: u32,
    mut available_in: *mut u64,
    mut next_in: *mut *const u8,
    mut available_out: *mut u64,
    mut next_out: *mut *mut u8,
    mut total_out: *mut u64,
) -> i32 {
    unsafe {
        if EnsureInitialized(s) == 0 {
            return 0;
        }
        if (*s).remaining_metadata_bytes_ != !0 {
            if *available_in != (*s).remaining_metadata_bytes_ as u64 {
                return 0;
            }
            if op as u32 != BROTLI_OPERATION_EMIT_METADATA as u32 {
                return 0;
            }
        }
        if op as u32 == BROTLI_OPERATION_EMIT_METADATA as u32 {
            UpdateSizeHint(s, 0);
            return ProcessMetadata(s, available_in, next_in, available_out, next_out, total_out);
        }
        if (*s).stream_state_ as u32 == BROTLI_STREAM_METADATA_HEAD as u32
            || (*s).stream_state_ as u32 == BROTLI_STREAM_METADATA_BODY as u32
        {
            return 0;
        }
        if (*s).stream_state_ as u32 != BROTLI_STREAM_PROCESSING as u32 && *available_in != 0 {
            return 0;
        }
        if (*s).params.quality == 0 || (*s).params.quality == 1 {
            return BrotliEncoderCompressStreamFast(
                s,
                op,
                available_in,
                next_in,
                available_out,
                next_out,
                total_out,
            );
        }
        loop {
            let mut remaining_block_size = RemainingInputBlockSize(s);
            if (*s).flint_ as i32 >= 0 && remaining_block_size > (*s).flint_ as u64 {
                remaining_block_size = (*s).flint_ as u64;
            }
            if remaining_block_size != 0 && *available_in != 0 {
                let mut copy_input_size = brotli_min_size_t(remaining_block_size, *available_in);
                CopyInputToRingBuffer(s, copy_input_size, *next_in);
                *next_in = (*next_in).offset(copy_input_size as isize);
                *available_in = (*available_in as u64).wrapping_sub(copy_input_size) as u64;
                if (*s).flint_ as i32 > 0 {
                    (*s).flint_ = ((*s).flint_ as i32 - copy_input_size as i32) as i8;
                }
            } else if InjectFlushOrPushOutput(s, available_out, next_out, total_out) != 0 {
                if (*s).flint_ as i32 == BROTLI_FLINT_WAITING_FOR_FLUSHING as i32 {
                    CheckFlushComplete(s);
                    if (*s).stream_state_ as u32 == BROTLI_STREAM_PROCESSING as u32 {
                        (*s).flint_ = BROTLI_FLINT_DONE as i8;
                    }
                }
            } else {
                if !((*s).available_out_ == 0
                    && (*s).stream_state_ as u32 == BROTLI_STREAM_PROCESSING as u32)
                {
                    break;
                }
                if !(remaining_block_size == 0 || op as u32 != BROTLI_OPERATION_PROCESS as u32) {
                    break;
                }
                let mut is_last =
                    if *available_in == 0 && op as u32 == BROTLI_OPERATION_FINISH as u32 {
                        1
                    } else {
                        0
                    };
                let mut force_flush =
                    if *available_in == 0 && op as u32 == BROTLI_OPERATION_FLUSH as u32 {
                        1
                    } else {
                        0
                    };
                let mut result: i32 = 0;
                if is_last == 0 && (*s).flint_ as i32 == 0 {
                    (*s).flint_ = BROTLI_FLINT_WAITING_FOR_FLUSHING as i8;
                    force_flush = 1;
                }
                UpdateSizeHint(s, *available_in);
                result = EncodeData(
                    s,
                    is_last,
                    force_flush,
                    &mut (*s).available_out_,
                    &mut (*s).next_out_,
                );
                if result == 0 {
                    return 0;
                }
                if force_flush != 0 {
                    (*s).stream_state_ = BROTLI_STREAM_FLUSH_REQUESTED;
                }
                if is_last != 0 {
                    (*s).stream_state_ = BROTLI_STREAM_FINISHED;
                }
            }
        }
        CheckFlushComplete(s);
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn BrotliEncoderIsFinished(mut s: *mut BrotliEncoderState) -> i32 {
    unsafe {
        return if (*s).stream_state_ as u32 == BROTLI_STREAM_FINISHED as u32
            && BrotliEncoderHasMoreOutput(s) == 0
        {
            1
        } else {
            0
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliEncoderHasMoreOutput(mut s: *mut BrotliEncoderState) -> i32 {
    unsafe {
        return if (*s).available_out_ != 0 { 1 } else { 0 };
    }
}

#[no_mangle]
pub extern "C" fn BrotliEncoderTakeOutput(
    mut s: *mut BrotliEncoderState,
    mut size: *mut u64,
) -> *const u8 {
    unsafe {
        let mut consumed_size = (*s).available_out_;
        let mut result = (*s).next_out_;
        if *size != 0 {
            consumed_size = brotli_min_size_t(*size, (*s).available_out_);
        }
        if consumed_size != 0 {
            let ref mut fresh116 = (*s).next_out_;
            *fresh116 = (*fresh116).offset(consumed_size as isize);
            let ref mut fresh117 = (*s).available_out_;
            *fresh117 = (*fresh117 as u64).wrapping_sub(consumed_size) as u64;
            let ref mut fresh118 = (*s).total_out_;
            *fresh118 = (*fresh118 as u64).wrapping_add(consumed_size) as u64;
            CheckFlushComplete(s);
            *size = consumed_size;
        } else {
            *size = 0;
            result = 0 as *mut u8;
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn BrotliEncoderVersion() -> u32 {
    return 0x1000009;
}
