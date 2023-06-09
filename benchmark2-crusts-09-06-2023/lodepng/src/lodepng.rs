use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn ftell(__stream: *mut FILE) -> i64;
    fn fseek(__stream: *mut FILE, __off: i64, __whence: i32) -> i32;
    fn fwrite(_: *const libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn fread(_: *mut libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub const LCT_MAX_OCTET_VALUE: u32 = 255;
pub const LCT_RGBA: u32 = 6;
pub const LCT_GREY_ALPHA: u32 = 4;
pub const LCT_PALETTE: u32 = 3;
pub const LCT_RGB: u32 = 2;
pub const LCT_GREY: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGState {
    pub decoder: LodePNGDecoderSettings,
    pub encoder: LodePNGEncoderSettings,
    pub info_raw: LodePNGColorMode,
    pub info_png: LodePNGInfo,
    pub error: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGInfo {
    pub compression_method: u32,
    pub filter_method: u32,
    pub interlace_method: u32,
    pub color: LodePNGColorMode,
    pub background_defined: u32,
    pub background_r: u32,
    pub background_g: u32,
    pub background_b: u32,
    pub text_num: u64,
    pub text_keys: *mut *mut i8,
    pub text_strings: *mut *mut i8,
    pub itext_num: u64,
    pub itext_keys: *mut *mut i8,
    pub itext_langtags: *mut *mut i8,
    pub itext_transkeys: *mut *mut i8,
    pub itext_strings: *mut *mut i8,
    pub time_defined: u32,
    pub time: LodePNGTime,
    pub phys_defined: u32,
    pub phys_x: u32,
    pub phys_y: u32,
    pub phys_unit: u32,
    pub gama_defined: u32,
    pub gama_gamma: u32,
    pub chrm_defined: u32,
    pub chrm_white_x: u32,
    pub chrm_white_y: u32,
    pub chrm_red_x: u32,
    pub chrm_red_y: u32,
    pub chrm_green_x: u32,
    pub chrm_green_y: u32,
    pub chrm_blue_x: u32,
    pub chrm_blue_y: u32,
    pub srgb_defined: u32,
    pub srgb_intent: u32,
    pub iccp_defined: u32,
    pub iccp_name: *mut i8,
    pub iccp_profile: *mut u8,
    pub iccp_profile_size: u32,
    pub sbit_defined: u32,
    pub sbit_r: u32,
    pub sbit_g: u32,
    pub sbit_b: u32,
    pub sbit_a: u32,
    pub unknown_chunks_data: [*mut u8; 3],
    pub unknown_chunks_size: [u64; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGTime {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGColorMode {
    pub colortype: u32,
    pub bitdepth: u32,
    pub palette: *mut u8,
    pub palettesize: u64,
    pub key_defined: u32,
    pub key_r: u32,
    pub key_g: u32,
    pub key_b: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGEncoderSettings {
    pub zlibsettings: LodePNGCompressSettings,
    pub auto_convert: u32,
    pub filter_palette_zero: u32,
    pub filter_strategy: u32,
    pub predefined_filters: *const u8,
    pub force_palette: u32,
    pub add_id: u32,
    pub text_compression: u32,
}
pub const LFS_PREDEFINED: u32 = 8;
pub const LFS_BRUTE_FORCE: u32 = 7;
pub const LFS_ENTROPY: u32 = 6;
pub const LFS_MINSUM: u32 = 5;
pub const LFS_FOUR: u32 = 4;
pub const LFS_THREE: u32 = 3;
pub const LFS_TWO: u32 = 2;
pub const LFS_ONE: u32 = 1;
pub const LFS_ZERO: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGCompressSettings {
    pub btype: u32,
    pub use_lz77: u32,
    pub windowsize: u32,
    pub minmatch: u32,
    pub nicematch: u32,
    pub lazymatching: u32,
    pub custom_zlib: Option<
        unsafe extern "C" fn(
            *mut *mut u8,
            *mut u64,
            *const u8,
            u64,
            *const LodePNGCompressSettings,
        ) -> u32,
    >,
    pub custom_deflate: Option<
        unsafe extern "C" fn(
            *mut *mut u8,
            *mut u64,
            *const u8,
            u64,
            *const LodePNGCompressSettings,
        ) -> u32,
    >,
    pub custom_context: *const libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGDecoderSettings {
    pub zlibsettings: LodePNGDecompressSettings,
    pub ignore_crc: u32,
    pub ignore_critical: u32,
    pub ignore_end: u32,
    pub color_convert: u32,
    pub read_text_chunks: u32,
    pub remember_unknown_chunks: u32,
    pub max_text_size: u64,
    pub max_icc_size: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGDecompressSettings {
    pub ignore_adler32: u32,
    pub ignore_nlen: u32,
    pub max_output_size: u64,
    pub custom_zlib: Option<
        unsafe extern "C" fn(
            *mut *mut u8,
            *mut u64,
            *const u8,
            u64,
            *const LodePNGDecompressSettings,
        ) -> u32,
    >,
    pub custom_inflate: Option<
        unsafe extern "C" fn(
            *mut *mut u8,
            *mut u64,
            *const u8,
            u64,
            *const LodePNGDecompressSettings,
        ) -> u32,
    >,
    pub custom_context: *const libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ColorTree {
    pub children: [*mut ColorTree; 16],
    pub index: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucvector {
    pub data: *mut u8,
    pub size: u64,
    pub allocsize: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGBitReader {
    pub data: *const u8,
    pub size: u64,
    pub bitsize: u64,
    pub bp: u64,
    pub buffer: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HuffmanTree {
    pub codes: *mut u32,
    pub lengths: *mut u32,
    pub maxbitlen: u32,
    pub numcodes: u32,
    pub table_len: *mut u8,
    pub table_value: *mut u16,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Hash {
    pub head: *mut i32,
    pub chain: *mut u16,
    pub val: *mut i32,
    pub headz: *mut i32,
    pub chainz: *mut u16,
    pub zeros: *mut u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGBitWriter {
    pub data: *mut ucvector,
    pub bp: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uivector {
    pub data: *mut u32,
    pub size: u64,
    pub allocsize: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BPMNode {
    pub weight: i32,
    pub index: u32,
    pub tail: *mut BPMNode,
    pub in_use: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BPMLists {
    pub memsize: u32,
    pub memory: *mut BPMNode,
    pub numfree: u32,
    pub nextfree: u32,
    pub freelist: *mut *mut BPMNode,
    pub listsize: u32,
    pub chains0: *mut *mut BPMNode,
    pub chains1: *mut *mut BPMNode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LodePNGColorStats {
    pub colored: u32,
    pub key: u32,
    pub key_r: u16,
    pub key_g: u16,
    pub key_b: u16,
    pub alpha: u32,
    pub numcolors: u32,
    pub palette: [u8; 1024],
    pub bits: u32,
    pub numpixels: u64,
    pub allow_palette: u32,
    pub allow_greyscale: u32,
}
#[no_mangle]
pub static mut LODEPNG_VERSION_STRING: *const i8 = b"20221108\0" as *const u8 as *const i8;
extern "C" fn lodepng_malloc(mut size: u64) -> *mut libc::c_void {
    unsafe {
        return malloc(size);
    }
}

extern "C" fn lodepng_realloc(mut ptr: *mut libc::c_void, mut new_size: u64) -> *mut libc::c_void {
    unsafe {
        return realloc(ptr, new_size);
    }
}

extern "C" fn lodepng_free(mut ptr: *mut libc::c_void) {
    unsafe {
        free(ptr);
    }
}

extern "C" fn lodepng_memcpy(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut size: u64,
) {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i < size {
            *(dst as *mut i8).offset(i as isize) = *(src as *const i8).offset(i as isize);
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn lodepng_memset(mut dst: *mut libc::c_void, mut value: i32, mut num: u64) {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i < num {
            *(dst as *mut i8).offset(i as isize) = value as i8;
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn lodepng_strlen(mut a: *const i8) -> u64 {
    unsafe {
        let mut orig = a;
        while *a != 0 {
            a = a.offset(1);
        }
        return a.offset_from(orig) as u64;
    }
}

extern "C" fn lodepng_addofl(mut a: u64, mut b: u64, mut result: *mut u64) -> i32 {
    unsafe {
        *result = a.wrapping_add(b);
        return (*result < a) as i32;
    }
}

extern "C" fn lodepng_mulofl(mut a: u64, mut b: u64, mut result: *mut u64) -> i32 {
    unsafe {
        *result = a.wrapping_mul(b);
        return (a != 0 && (*result).wrapping_div(a) != b) as i32;
    }
}

extern "C" fn lodepng_gtofl(mut a: u64, mut b: u64, mut c: u64) -> i32 {
    let mut d: u64 = 0;
    if lodepng_addofl(a, b, &mut d) != 0 {
        return 1;
    }
    return (d > c) as i32;
}

extern "C" fn uivector_cleanup(mut p: *mut libc::c_void) {
    unsafe {
        let ref mut fresh0 = (*(p as *mut uivector)).allocsize;
        *fresh0 = 0;
        (*(p as *mut uivector)).size = *fresh0;
        lodepng_free((*(p as *mut uivector)).data as *mut libc::c_void);
        let ref mut fresh1 = (*(p as *mut uivector)).data;
        *fresh1 = 0 as *mut u32;
    }
}

extern "C" fn uivector_resize(mut p: *mut uivector, mut size: u64) -> u32 {
    unsafe {
        let mut allocsize = size.wrapping_mul(::std::mem::size_of::<u32>() as u64);
        if allocsize > (*p).allocsize {
            let mut newsize = allocsize.wrapping_add((*p).allocsize >> 1);
            let mut data = lodepng_realloc((*p).data as *mut libc::c_void, newsize);
            if !data.is_null() {
                (*p).allocsize = newsize;
                let ref mut fresh2 = (*p).data;
                *fresh2 = data as *mut u32;
            } else {
                return 0;
            }
        };
        (*p).size = size;
        return 1;
    }
}

extern "C" fn uivector_init(mut p: *mut uivector) {
    unsafe {
        let ref mut fresh3 = (*p).data;
        *fresh3 = 0 as *mut u32;
        let ref mut fresh4 = (*p).allocsize;
        *fresh4 = 0;
        (*p).size = *fresh4;
    }
}

extern "C" fn uivector_push_back(mut p: *mut uivector, mut c: u32) -> u32 {
    unsafe {
        if uivector_resize(p, ((*p).size).wrapping_add(1)) == 0 {
            return 0;
        };
        *((*p).data).offset(((*p).size).wrapping_sub(1) as isize) = c;
        return 1;
    }
}

extern "C" fn ucvector_reserve(mut p: *mut ucvector, mut size: u64) -> u32 {
    unsafe {
        if size > (*p).allocsize {
            let mut newsize = size.wrapping_add((*p).allocsize >> 1);
            let mut data = lodepng_realloc((*p).data as *mut libc::c_void, newsize);
            if !data.is_null() {
                (*p).allocsize = newsize;
                let ref mut fresh5 = (*p).data;
                *fresh5 = data as *mut u8;
            } else {
                return 0;
            }
        }
        return 1;
    }
}

extern "C" fn ucvector_resize(mut p: *mut ucvector, mut size: u64) -> u32 {
    unsafe {
        (*p).size = size;
        return ucvector_reserve(p, size);
    }
}

extern "C" fn ucvector_init(mut buffer: *mut u8, mut size: u64) -> ucvector {
    unsafe {
        let mut v = ucvector {
            data: 0 as *mut u8,
            size: 0,
            allocsize: 0,
        };
        v.data = buffer;
        v.size = size;
        v.allocsize = v.size;
        return v;
    }
}

extern "C" fn string_cleanup(mut out: *mut *mut i8) {
    unsafe {
        lodepng_free(*out as *mut libc::c_void);
        *out = 0 as *mut i8;
    }
}

extern "C" fn alloc_string_sized(mut in_0: *const i8, mut insize: u64) -> *mut i8 {
    unsafe {
        let mut out = lodepng_malloc(insize.wrapping_add(1)) as *mut i8;
        if !out.is_null() {
            lodepng_memcpy(
                out as *mut libc::c_void,
                in_0 as *const libc::c_void,
                insize,
            );
            *out.offset(insize as isize) = 0;
        }
        return out;
    }
}

extern "C" fn alloc_string(mut in_0: *const i8) -> *mut i8 {
    unsafe {
        return alloc_string_sized(in_0, lodepng_strlen(in_0));
    }
}

extern "C" fn lodepng_read32bitInt(mut buffer: *const u8) -> u32 {
    unsafe {
        return (*buffer.offset(0 as isize) as u32) << 24
            | (*buffer.offset(1 as isize) as u32) << 16
            | (*buffer.offset(2 as isize) as u32) << 8
            | *buffer.offset(3 as isize) as u32;
    }
}

extern "C" fn lodepng_set32bitInt(mut buffer: *mut u8, mut value: u32) {
    unsafe {
        *buffer.offset(0 as isize) = (value >> 24 & 0xffu32) as u8;
        *buffer.offset(1 as isize) = (value >> 16 & 0xffu32) as u8;
        *buffer.offset(2 as isize) = (value >> 8 & 0xffu32) as u8;
        *buffer.offset(3 as isize) = (value & 0xffu32) as u8;
    }
}

extern "C" fn lodepng_filesize(mut filename: *const i8) -> i64 {
    unsafe {
        let mut file = 0 as *mut FILE;
        let mut size: i64 = 0;
        file = fopen(filename, b"rb\0" as *const u8 as *const i8);
        if file.is_null() {
            return -1 as i64;
        }
        if fseek(file, 0, 2) != 0 {
            fclose(file);
            return -1 as i64;
        }
        size = ftell(file);
        if size == 9223372036854775807 {
            size = -1 as i64;
        }
        fclose(file);
        return size;
    }
}

extern "C" fn lodepng_buffer_file(mut out: *mut u8, mut size: u64, mut filename: *const i8) -> u32 {
    unsafe {
        let mut file = 0 as *mut FILE;
        let mut readsize: u64 = 0;
        file = fopen(filename, b"rb\0" as *const u8 as *const i8);
        if file.is_null() {
            return 78;
        }
        readsize = fread(out as *mut libc::c_void, 1, size, file);
        fclose(file);
        if readsize != size {
            return 78;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_load_file(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut filename: *const i8,
) -> u32 {
    unsafe {
        let mut size = lodepng_filesize(filename);
        if size < 0 {
            return 78;
        }
        *outsize = size as u64;
        *out = lodepng_malloc(size as u64) as *mut u8;
        if (*out).is_null() && size > 0 {
            return 83;
        }
        return lodepng_buffer_file(*out, size as u64, filename);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_save_file(
    mut buffer: *const u8,
    mut buffersize: u64,
    mut filename: *const i8,
) -> u32 {
    unsafe {
        let mut file = 0 as *mut FILE;
        file = fopen(filename, b"wb\0" as *const u8 as *const i8);
        if file.is_null() {
            return 79;
        }
        fwrite(buffer as *const libc::c_void, 1, buffersize, file);
        fclose(file);
        return 0;
    }
}

extern "C" fn LodePNGBitWriter_init(mut writer: *mut LodePNGBitWriter, mut data: *mut ucvector) {
    unsafe {
        let ref mut fresh6 = (*writer).data;
        *fresh6 = data;
        (*writer).bp = 0;
    }
}

extern "C" fn writeBits(mut writer: *mut LodePNGBitWriter, mut value: u32, mut nbits: u64) {
    unsafe {
        if nbits == 1 {
            if (*writer).bp as u32 & 7 == 0 {
                if ucvector_resize((*writer).data, ((*(*writer).data).size).wrapping_add(1)) == 0 {
                    return;
                };
                *((*(*writer).data).data)
                    .offset(((*(*writer).data).size).wrapping_sub(1) as isize) = 0;
            }
            let ref mut fresh7 =
                *((*(*writer).data).data).offset(((*(*writer).data).size).wrapping_sub(1) as isize);
            *fresh7 = (*fresh7 as u32 | value << ((*writer).bp as u32 & 7u32)) as u8;
            let ref mut fresh8 = (*writer).bp;
            *fresh8 = (*fresh8).wrapping_add(1);
        } else {
            let mut i: u64 = 0;
            i = 0;
            while i != nbits {
                if (*writer).bp as u32 & 7 == 0 {
                    if ucvector_resize((*writer).data, ((*(*writer).data).size).wrapping_add(1))
                        == 0
                    {
                        return;
                    };
                    *((*(*writer).data).data)
                        .offset(((*(*writer).data).size).wrapping_sub(1) as isize) = 0;
                }
                let ref mut fresh9 = *((*(*writer).data).data)
                    .offset(((*(*writer).data).size).wrapping_sub(1) as isize);
                *fresh9 = (*fresh9 as i32
                    | ((value >> i & 1u32) as i32) << ((*writer).bp as u32 & 7))
                    as u8;
                let ref mut fresh10 = (*writer).bp;
                *fresh10 = (*fresh10).wrapping_add(1);
                i = i.wrapping_add(1);
            }
        };
    }
}

extern "C" fn writeBitsReversed(mut writer: *mut LodePNGBitWriter, mut value: u32, mut nbits: u64) {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i != nbits {
            if (*writer).bp as u32 & 7 == 0 {
                if ucvector_resize((*writer).data, ((*(*writer).data).size).wrapping_add(1)) == 0 {
                    return;
                };
                *((*(*writer).data).data)
                    .offset(((*(*writer).data).size).wrapping_sub(1) as isize) = 0;
            }
            let ref mut fresh11 =
                *((*(*writer).data).data).offset(((*(*writer).data).size).wrapping_sub(1) as isize);
            *fresh11 = (*fresh11 as i32
                | ((value >> nbits.wrapping_sub(1u64).wrapping_sub(i) & 1u32) as i32)
                    << ((*writer).bp as u32 & 7)) as u8;
            let ref mut fresh12 = (*writer).bp;
            *fresh12 = (*fresh12).wrapping_add(1);
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn LodePNGBitReader_init(
    mut reader: *mut LodePNGBitReader,
    mut data: *const u8,
    mut size: u64,
) -> u32 {
    unsafe {
        let mut temp: u64 = 0;
        let ref mut fresh13 = (*reader).data;
        *fresh13 = data;
        (*reader).size = size;
        if lodepng_mulofl(size, 8, &mut (*reader).bitsize) != 0 {
            return 105;
        }
        if lodepng_addofl((*reader).bitsize, 64, &mut temp) != 0 {
            return 105;
        };
        (*reader).bp = 0;
        (*reader).buffer = 0;
        return 0;
    }
}

extern "C" fn ensureBits9(mut reader: *mut LodePNGBitReader, mut nbits: u64) {
    unsafe {
        let mut start = (*reader).bp >> 3;
        let mut size = (*reader).size;
        if start.wrapping_add(1) < size {
            (*reader).buffer = *((*reader).data).offset(start.wrapping_add(0) as isize) as u32
                | (*((*reader).data).offset(start.wrapping_add(1) as isize) as u32) << 8;
            (*reader).buffer >>= (*reader).bp & 7;
        } else {
            (*reader).buffer = 0;
            if start.wrapping_add(0) < size {
                (*reader).buffer = *((*reader).data).offset(start.wrapping_add(0) as isize) as u32;
            };
            (*reader).buffer >>= (*reader).bp & 7;
        };
    }
}

extern "C" fn ensureBits17(mut reader: *mut LodePNGBitReader, mut nbits: u64) {
    unsafe {
        let mut start = (*reader).bp >> 3;
        let mut size = (*reader).size;
        if start.wrapping_add(2) < size {
            (*reader).buffer = *((*reader).data).offset(start.wrapping_add(0) as isize) as u32
                | (*((*reader).data).offset(start.wrapping_add(1u64) as isize) as u32) << 8
                | (*((*reader).data).offset(start.wrapping_add(2) as isize) as u32) << 16;
            (*reader).buffer >>= (*reader).bp & 7;
        } else {
            (*reader).buffer = 0;
            if start.wrapping_add(0) < size {
                (*reader).buffer |= *((*reader).data).offset(start.wrapping_add(0) as isize) as u32;
            }
            if start.wrapping_add(1) < size {
                (*reader).buffer |=
                    (*((*reader).data).offset(start.wrapping_add(1) as isize) as u32) << 8;
            };
            (*reader).buffer >>= (*reader).bp & 7;
        };
    }
}

extern "C" fn ensureBits25(mut reader: *mut LodePNGBitReader, mut nbits: u64) {
    unsafe {
        let mut start = (*reader).bp >> 3;
        let mut size = (*reader).size;
        if start.wrapping_add(3) < size {
            (*reader).buffer = *((*reader).data).offset(start.wrapping_add(0) as isize) as u32
                | (*((*reader).data).offset(start.wrapping_add(1u64) as isize) as u32) << 8
                | (*((*reader).data).offset(start.wrapping_add(2u64) as isize) as u32) << 16
                | (*((*reader).data).offset(start.wrapping_add(3) as isize) as u32) << 24;
            (*reader).buffer >>= (*reader).bp & 7;
        } else {
            (*reader).buffer = 0;
            if start.wrapping_add(0) < size {
                (*reader).buffer |= *((*reader).data).offset(start.wrapping_add(0) as isize) as u32;
            }
            if start.wrapping_add(1) < size {
                (*reader).buffer |=
                    (*((*reader).data).offset(start.wrapping_add(1) as isize) as u32) << 8;
            }
            if start.wrapping_add(2) < size {
                (*reader).buffer |=
                    (*((*reader).data).offset(start.wrapping_add(2) as isize) as u32) << 16;
            };
            (*reader).buffer >>= (*reader).bp & 7;
        };
    }
}

extern "C" fn ensureBits32(mut reader: *mut LodePNGBitReader, mut nbits: u64) {
    unsafe {
        let mut start = (*reader).bp >> 3;
        let mut size = (*reader).size;
        if start.wrapping_add(4) < size {
            (*reader).buffer = *((*reader).data).offset(start.wrapping_add(0) as isize) as u32
                | (*((*reader).data).offset(start.wrapping_add(1u64) as isize) as u32) << 8
                | (*((*reader).data).offset(start.wrapping_add(2u64) as isize) as u32) << 16
                | (*((*reader).data).offset(start.wrapping_add(3) as isize) as u32) << 24;
            (*reader).buffer >>= (*reader).bp & 7;
            (*reader).buffer |=
                ((*((*reader).data).offset(start.wrapping_add(4u64) as isize) as u32) << 24)
                    << 8u64.wrapping_sub((*reader).bp & 7);
        } else {
            (*reader).buffer = 0;
            if start.wrapping_add(0) < size {
                (*reader).buffer |= *((*reader).data).offset(start.wrapping_add(0) as isize) as u32;
            }
            if start.wrapping_add(1) < size {
                (*reader).buffer |=
                    (*((*reader).data).offset(start.wrapping_add(1) as isize) as u32) << 8;
            }
            if start.wrapping_add(2) < size {
                (*reader).buffer |=
                    (*((*reader).data).offset(start.wrapping_add(2) as isize) as u32) << 16;
            }
            if start.wrapping_add(3) < size {
                (*reader).buffer |=
                    (*((*reader).data).offset(start.wrapping_add(3) as isize) as u32) << 24;
            };
            (*reader).buffer >>= (*reader).bp & 7;
        };
    }
}

extern "C" fn peekBits(mut reader: *mut LodePNGBitReader, mut nbits: u64) -> u32 {
    unsafe {
        return (*reader).buffer & (1u32 << nbits).wrapping_sub(1);
    }
}

extern "C" fn advanceBits(mut reader: *mut LodePNGBitReader, mut nbits: u64) {
    unsafe {
        (*reader).buffer >>= nbits;
        let ref mut fresh14 = (*reader).bp;
        *fresh14 = (*fresh14 as u64).wrapping_add(nbits) as u64;
    }
}

extern "C" fn readBits(mut reader: *mut LodePNGBitReader, mut nbits: u64) -> u32 {
    unsafe {
        let mut result = peekBits(reader, nbits);
        advanceBits(reader, nbits);
        return result;
    }
}

extern "C" fn reverseBits(mut bits: u32, mut num: u32) -> u32 {
    let mut i: u32 = 0;
    let mut result = 0;
    i = 0;
    while i < num {
        result |= (bits >> num.wrapping_sub(i).wrapping_sub(1) & 1) << i;
        i = i.wrapping_add(1);
    }
    return result;
}

static mut LENGTHBASE: [u32; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
static mut LENGTHEXTRA: [u32; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
static mut DISTANCEBASE: [u32; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
static mut DISTANCEEXTRA: [u32; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];
static mut CLCL_ORDER: [u32; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];
extern "C" fn HuffmanTree_init(mut tree: *mut HuffmanTree) {
    unsafe {
        let ref mut fresh15 = (*tree).codes;
        *fresh15 = 0 as *mut u32;
        let ref mut fresh16 = (*tree).lengths;
        *fresh16 = 0 as *mut u32;
        let ref mut fresh17 = (*tree).table_len;
        *fresh17 = 0 as *mut u8;
        let ref mut fresh18 = (*tree).table_value;
        *fresh18 = 0 as *mut u16;
    }
}

extern "C" fn HuffmanTree_cleanup(mut tree: *mut HuffmanTree) {
    unsafe {
        lodepng_free((*tree).codes as *mut libc::c_void);
        lodepng_free((*tree).lengths as *mut libc::c_void);
        lodepng_free((*tree).table_len as *mut libc::c_void);
        lodepng_free((*tree).table_value as *mut libc::c_void);
    }
}

static mut mask: u32 = 0;
extern "C" fn HuffmanTree_makeTable(mut tree: *mut HuffmanTree) -> u32 {
    unsafe {
        static mut headsize: u32 = 1 << 9;
        let mut i: u64 = 0;
        let mut numpresent: u64 = 0;
        let mut pointer: u64 = 0;
        let mut size: u64 = 0;
        let mut maxlens =
            lodepng_malloc((headsize as u64).wrapping_mul(::std::mem::size_of::<u32>() as u64))
                as *mut u32;
        if maxlens.is_null() {
            return 83;
        }
        lodepng_memset(
            maxlens as *mut libc::c_void,
            0,
            (headsize as u64).wrapping_mul(::std::mem::size_of::<u32>() as u64),
        );
        i = 0;
        while i < (*tree).numcodes as u64 {
            let mut symbol = *((*tree).codes).offset(i as isize);
            let mut l = *((*tree).lengths).offset(i as isize);
            let mut index: u32 = 0;
            if !(l <= 9) {
                index = reverseBits(symbol >> l.wrapping_sub(9), 9);
                *maxlens.offset(index as isize) = if *maxlens.offset(index as isize) > l {
                    *maxlens.offset(index as isize)
                } else {
                    l
                };
            }
            i = i.wrapping_add(1);
        }
        size = headsize as u64;
        i = 0;
        while i < headsize as u64 {
            let mut l_0 = *maxlens.offset(i as isize);
            if l_0 > 9 {
                size = (size).wrapping_add((1u32 << l_0.wrapping_sub(9u32)) as u64) as u64;
            }
            i = i.wrapping_add(1);
        }
        let ref mut fresh19 = (*tree).table_len;
        *fresh19 = lodepng_malloc(size.wrapping_mul(::std::mem::size_of::<u8>() as u64)) as *mut u8;
        let ref mut fresh20 = (*tree).table_value;
        *fresh20 =
            lodepng_malloc(size.wrapping_mul(::std::mem::size_of::<u16>() as u64)) as *mut u16;
        if ((*tree).table_len).is_null() || ((*tree).table_value).is_null() {
            lodepng_free(maxlens as *mut libc::c_void);
            return 83;
        }
        i = 0;
        while i < size {
            *((*tree).table_len).offset(i as isize) = 16;
            i = i.wrapping_add(1);
        }
        pointer = headsize as u64;
        i = 0;
        while i < headsize as u64 {
            let mut l_1 = *maxlens.offset(i as isize);
            if !(l_1 <= 9) {
                *((*tree).table_len).offset(i as isize) = l_1 as u8;
                *((*tree).table_value).offset(i as isize) = pointer as u16;
                pointer = (pointer).wrapping_add((1u32 << l_1.wrapping_sub(9u32)) as u64) as u64;
            }
            i = i.wrapping_add(1);
        }
        lodepng_free(maxlens as *mut libc::c_void);
        numpresent = 0;
        i = 0;
        while i < (*tree).numcodes as u64 {
            let mut l_2 = *((*tree).lengths).offset(i as isize);
            let mut symbol_0: u32 = 0;
            let mut reverse: u32 = 0;
            if !(l_2 == 0) {
                symbol_0 = *((*tree).codes).offset(i as isize);
                reverse = reverseBits(symbol_0, l_2);
                numpresent = numpresent.wrapping_add(1);
                if l_2 <= 9 {
                    let mut num = 1u32 << 9u32.wrapping_sub(l_2);
                    let mut j: u32 = 0;
                    j = 0;
                    while j < num {
                        let mut index_0 = reverse | j << l_2;
                        if *((*tree).table_len).offset(index_0 as isize) as i32 != 16 {
                            return 55;
                        };
                        *((*tree).table_len).offset(index_0 as isize) = l_2 as u8;
                        *((*tree).table_value).offset(index_0 as isize) = i as u16;
                        j = j.wrapping_add(1);
                    }
                } else {
                    let mut index_1 = reverse & mask;
                    let mut maxlen = *((*tree).table_len).offset(index_1 as isize) as u32;
                    let mut tablelen = maxlen.wrapping_sub(9);
                    let mut start = *((*tree).table_value).offset(index_1 as isize) as u32;
                    let mut num_0 = 1u32 << tablelen.wrapping_sub(l_2.wrapping_sub(9));
                    let mut j_0: u32 = 0;
                    if maxlen < l_2 {
                        return 55;
                    }
                    j_0 = 0;
                    while j_0 < num_0 {
                        let mut reverse2 = reverse >> 9;
                        let mut index2 = start.wrapping_add(reverse2 | j_0 << l_2.wrapping_sub(9));
                        *((*tree).table_len).offset(index2 as isize) = l_2 as u8;
                        *((*tree).table_value).offset(index2 as isize) = i as u16;
                        j_0 = j_0.wrapping_add(1);
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        if numpresent < 2 {
            i = 0;
            while i < size {
                if *((*tree).table_len).offset(i as isize) as i32 == 16 {
                    *((*tree).table_len).offset(i as isize) = (if i < headsize as u64 {
                        1
                    } else {
                        9u32.wrapping_add(1)
                    }) as u8;
                    *((*tree).table_value).offset(i as isize) = 65535;
                }
                i = i.wrapping_add(1);
            }
        } else {
            i = 0;
            while i < size {
                if *((*tree).table_len).offset(i as isize) as i32 == 16 {
                    return 55;
                }
                i = i.wrapping_add(1);
            }
        }
        return 0;
    }
}

extern "C" fn HuffmanTree_makeFromLengths2(mut tree: *mut HuffmanTree) -> u32 {
    unsafe {
        let mut blcount = 0 as *mut u32;
        let mut nextcode = 0 as *mut u32;
        let mut error = 0;
        let mut bits: u32 = 0;
        let mut n: u32 = 0;
        let ref mut fresh21 = (*tree).codes;
        *fresh21 = lodepng_malloc(
            ((*tree).numcodes as u64).wrapping_mul(::std::mem::size_of::<u32>() as u64),
        ) as *mut u32;
        blcount = lodepng_malloc(
            (((*tree).maxbitlen).wrapping_add(1u32) as u64)
                .wrapping_mul(::std::mem::size_of::<u32>() as u64),
        ) as *mut u32;
        nextcode = lodepng_malloc(
            (((*tree).maxbitlen).wrapping_add(1u32) as u64)
                .wrapping_mul(::std::mem::size_of::<u32>() as u64),
        ) as *mut u32;
        if ((*tree).codes).is_null() || blcount.is_null() || nextcode.is_null() {
            error = 83;
        }
        if error == 0 {
            n = 0;
            while n != ((*tree).maxbitlen).wrapping_add(1) {
                let ref mut fresh22 = *nextcode.offset(n as isize);
                *fresh22 = 0;
                *blcount.offset(n as isize) = *fresh22;
                n = n.wrapping_add(1);
            }
            bits = 0;
            while bits != (*tree).numcodes {
                let ref mut fresh23 =
                    *blcount.offset(*((*tree).lengths).offset(bits as isize) as isize);
                *fresh23 = (*fresh23).wrapping_add(1);
                bits = bits.wrapping_add(1);
            }
            bits = 1;
            while bits <= (*tree).maxbitlen {
                *nextcode.offset(bits as isize) = (*nextcode
                    .offset(bits.wrapping_sub(1u32) as isize))
                .wrapping_add(*blcount.offset(bits.wrapping_sub(1) as isize))
                    << 1;
                bits = bits.wrapping_add(1);
            }
            n = 0;
            while n != (*tree).numcodes {
                if *((*tree).lengths).offset(n as isize) != 0 {
                    let ref mut fresh24 =
                        *nextcode.offset(*((*tree).lengths).offset(n as isize) as isize);
                    let fresh25 = *fresh24;
                    *fresh24 = (*fresh24).wrapping_add(1);
                    *((*tree).codes).offset(n as isize) = fresh25;
                    *((*tree).codes).offset(n as isize) &=
                        (1u32 << *((*tree).lengths).offset(n as isize)).wrapping_sub(1);
                }
                n = n.wrapping_add(1);
            }
        }
        lodepng_free(blcount as *mut libc::c_void);
        lodepng_free(nextcode as *mut libc::c_void);
        if error == 0 {
            error = HuffmanTree_makeTable(tree);
        }
        return error;
    }
}

extern "C" fn HuffmanTree_makeFromLengths(
    mut tree: *mut HuffmanTree,
    mut bitlen: *const u32,
    mut numcodes: u64,
    mut maxbitlen: u32,
) -> u32 {
    unsafe {
        let mut i: u32 = 0;
        let ref mut fresh26 = (*tree).lengths;
        *fresh26 =
            lodepng_malloc(numcodes.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32;
        if ((*tree).lengths).is_null() {
            return 83;
        }
        i = 0;
        while i as u64 != numcodes {
            *((*tree).lengths).offset(i as isize) = *bitlen.offset(i as isize);
            i = i.wrapping_add(1);
        }
        (*tree).numcodes = numcodes as u32;
        (*tree).maxbitlen = maxbitlen;
        return HuffmanTree_makeFromLengths2(tree);
    }
}

extern "C" fn bpmnode_create(
    mut lists: *mut BPMLists,
    mut weight: i32,
    mut index: u32,
    mut tail: *mut BPMNode,
) -> *mut BPMNode {
    unsafe {
        let mut i: u32 = 0;
        let mut result = 0 as *mut BPMNode;
        if (*lists).nextfree >= (*lists).numfree {
            i = 0;
            while i != (*lists).memsize {
                (*((*lists).memory).offset(i as isize)).in_use = 0;
                i = i.wrapping_add(1);
            }
            i = 0;
            while i != (*lists).listsize {
                let mut node = 0 as *mut BPMNode;
                node = *((*lists).chains0).offset(i as isize);
                while !node.is_null() {
                    (*node).in_use = 1;
                    node = (*node).tail;
                }
                node = *((*lists).chains1).offset(i as isize);
                while !node.is_null() {
                    (*node).in_use = 1;
                    node = (*node).tail;
                }
                i = i.wrapping_add(1);
            }
            (*lists).numfree = 0;
            i = 0;
            while i != (*lists).memsize {
                if (*((*lists).memory).offset(i as isize)).in_use == 0 {
                    let ref mut fresh27 = (*lists).numfree;
                    let fresh28 = *fresh27;
                    *fresh27 = (*fresh27).wrapping_add(1);
                    let ref mut fresh29 = *((*lists).freelist).offset(fresh28 as isize);
                    *fresh29 = &mut *((*lists).memory).offset(i as isize) as *mut BPMNode;
                }
                i = i.wrapping_add(1);
            }
            (*lists).nextfree = 0;
        }
        let ref mut fresh30 = (*lists).nextfree;
        let fresh31 = *fresh30;
        *fresh30 = (*fresh30).wrapping_add(1);
        result = *((*lists).freelist).offset(fresh31 as isize);
        (*result).weight = weight;
        (*result).index = index;
        let ref mut fresh32 = (*result).tail;
        *fresh32 = tail;
        return result;
    }
}

extern "C" fn bpmnode_sort(mut leaves: *mut BPMNode, mut num: u64) {
    unsafe {
        let mut mem = lodepng_malloc((::std::mem::size_of::<BPMNode>() as u64).wrapping_mul(num))
            as *mut BPMNode;
        let mut width: u64 = 0;
        let mut counter = 0;
        width = 1;
        while width < num {
            let mut a = if counter & 1 != 0 { mem } else { leaves };
            let mut b = if counter & 1 != 0 { leaves } else { mem };
            let mut p: u64 = 0;
            p = 0;
            while p < num {
                let mut q = if p.wrapping_add(width) > num {
                    num
                } else {
                    p.wrapping_add(width)
                };
                let mut r = if p.wrapping_add(2u64.wrapping_mul(width)) > num {
                    num
                } else {
                    p.wrapping_add(2u64.wrapping_mul(width))
                };
                let mut i = p;
                let mut j = q;
                let mut k: u64 = 0;
                k = p;
                while k < r {
                    if i < q
                        && (j >= r
                            || (*a.offset(i as isize)).weight <= (*a.offset(j as isize)).weight)
                    {
                        let fresh33 = i;
                        i = i.wrapping_add(1);
                        *b.offset(k as isize) = *a.offset(fresh33 as isize);
                    } else {
                        let fresh34 = j;
                        j = j.wrapping_add(1);
                        *b.offset(k as isize) = *a.offset(fresh34 as isize);
                    }
                    k = k.wrapping_add(1);
                }
                p = (p).wrapping_add(2u64.wrapping_mul(width)) as u64;
            }
            counter = counter.wrapping_add(1);
            width = (width).wrapping_mul(2) as u64;
        }
        if counter & 1 != 0 {
            lodepng_memcpy(
                leaves as *mut libc::c_void,
                mem as *const libc::c_void,
                (::std::mem::size_of::<BPMNode>() as u64).wrapping_mul(num),
            );
        }
        lodepng_free(mem as *mut libc::c_void);
    }
}

extern "C" fn boundaryPM(
    mut lists: *mut BPMLists,
    mut leaves: *mut BPMNode,
    mut numpresent: u64,
    mut c: i32,
    mut num: i32,
) {
    unsafe {
        let mut lastindex = (**((*lists).chains1).offset(c as isize)).index;
        if c == 0 {
            if lastindex as u64 >= numpresent {
                return;
            }
            let ref mut fresh35 = *((*lists).chains0).offset(c as isize);
            *fresh35 = *((*lists).chains1).offset(c as isize);
            let ref mut fresh36 = *((*lists).chains1).offset(c as isize);
            *fresh36 = bpmnode_create(
                lists,
                (*leaves.offset(lastindex as isize)).weight,
                lastindex.wrapping_add(1),
                0 as *mut BPMNode,
            );
        } else {
            let mut sum = (**((*lists).chains0).offset((c - 1i32) as isize)).weight
                + (**((*lists).chains1).offset((c - 1i32) as isize)).weight;
            let ref mut fresh37 = *((*lists).chains0).offset(c as isize);
            *fresh37 = *((*lists).chains1).offset(c as isize);
            if (lastindex as u64) < numpresent && sum > (*leaves.offset(lastindex as isize)).weight
            {
                let ref mut fresh38 = *((*lists).chains1).offset(c as isize);
                *fresh38 = bpmnode_create(
                    lists,
                    (*leaves.offset(lastindex as isize)).weight,
                    lastindex.wrapping_add(1),
                    (**((*lists).chains1).offset(c as isize)).tail,
                );
                return;
            }
            let ref mut fresh39 = *((*lists).chains1).offset(c as isize);
            *fresh39 = bpmnode_create(
                lists,
                sum,
                lastindex,
                *((*lists).chains1).offset((c - 1i32) as isize),
            );
            if (num + 1i32) < 2u64.wrapping_mul(numpresent).wrapping_sub(2) as i32 {
                boundaryPM(lists, leaves, numpresent, c - 1, num);
                boundaryPM(lists, leaves, numpresent, c - 1, num);
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn lodepng_huffman_code_lengths(
    mut lengths: *mut u32,
    mut frequencies: *const u32,
    mut numcodes: u64,
    mut maxbitlen: u32,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut i: u32 = 0;
        let mut numpresent = 0;
        let mut leaves = 0 as *mut BPMNode;
        if numcodes == 0 {
            return 80;
        }
        if 1 << maxbitlen < numcodes as u32 {
            return 80;
        }
        leaves = lodepng_malloc(numcodes.wrapping_mul(::std::mem::size_of::<BPMNode>() as u64))
            as *mut BPMNode;
        if leaves.is_null() {
            return 83;
        }
        i = 0;
        while i as u64 != numcodes {
            if *frequencies.offset(i as isize) > 0 {
                (*leaves.offset(numpresent as isize)).weight =
                    *frequencies.offset(i as isize) as i32;
                (*leaves.offset(numpresent as isize)).index = i;
                numpresent = numpresent.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        lodepng_memset(
            lengths as *mut libc::c_void,
            0,
            numcodes.wrapping_mul(::std::mem::size_of::<u32>() as u64),
        );
        if numpresent == 0 {
            let ref mut fresh40 = *lengths.offset(1 as isize);
            *fresh40 = 1;
            *lengths.offset(0 as isize) = *fresh40;
        } else if numpresent == 1 {
            *lengths.offset((*leaves.offset(0 as isize)).index as isize) = 1;
            *lengths.offset(
                (if (*leaves.offset(0 as isize)).index == 0u32 {
                    1
                } else {
                    0
                }) as isize,
            ) = 1;
        } else {
            let mut lists = BPMLists {
                memsize: 0,
                memory: 0 as *mut BPMNode,
                numfree: 0,
                nextfree: 0,
                freelist: 0 as *mut *mut BPMNode,
                listsize: 0,
                chains0: 0 as *mut *mut BPMNode,
                chains1: 0 as *mut *mut BPMNode,
            };
            let mut node = 0 as *mut BPMNode;
            bpmnode_sort(leaves, numpresent);
            lists.listsize = maxbitlen;
            lists.memsize = 2u32
                .wrapping_mul(maxbitlen)
                .wrapping_mul(maxbitlen.wrapping_add(1));
            lists.nextfree = 0;
            lists.numfree = lists.memsize;
            lists.memory = lodepng_malloc(
                (lists.memsize as u64).wrapping_mul(::std::mem::size_of::<BPMNode>() as u64),
            ) as *mut BPMNode;
            lists.freelist = lodepng_malloc(
                (lists.memsize as u64).wrapping_mul(::std::mem::size_of::<*mut BPMNode>() as u64),
            ) as *mut *mut BPMNode;
            lists.chains0 = lodepng_malloc(
                (lists.listsize as u64).wrapping_mul(::std::mem::size_of::<*mut BPMNode>() as u64),
            ) as *mut *mut BPMNode;
            lists.chains1 = lodepng_malloc(
                (lists.listsize as u64).wrapping_mul(::std::mem::size_of::<*mut BPMNode>() as u64),
            ) as *mut *mut BPMNode;
            if (lists.memory).is_null()
                || (lists.freelist).is_null()
                || (lists.chains0).is_null()
                || (lists.chains1).is_null()
            {
                error = 83;
            }
            if error == 0 {
                i = 0;
                while i != lists.memsize {
                    let ref mut fresh41 = *(lists.freelist).offset(i as isize);
                    *fresh41 = &mut *(lists.memory).offset(i as isize) as *mut BPMNode;
                    i = i.wrapping_add(1);
                }
                bpmnode_create(
                    &mut lists,
                    (*leaves.offset(0 as isize)).weight,
                    1,
                    0 as *mut BPMNode,
                );
                bpmnode_create(
                    &mut lists,
                    (*leaves.offset(1 as isize)).weight,
                    2,
                    0 as *mut BPMNode,
                );
                i = 0;
                while i != lists.listsize {
                    let ref mut fresh42 = *(lists.chains0).offset(i as isize);
                    *fresh42 = &mut *(lists.memory).offset(0 as isize) as *mut BPMNode;
                    let ref mut fresh43 = *(lists.chains1).offset(i as isize);
                    *fresh43 = &mut *(lists.memory).offset(1 as isize) as *mut BPMNode;
                    i = i.wrapping_add(1);
                }
                i = 2;
                while i as u64 != 2u64.wrapping_mul(numpresent).wrapping_sub(2) {
                    boundaryPM(
                        &mut lists,
                        leaves,
                        numpresent,
                        maxbitlen as i32 - 1,
                        i as i32,
                    );
                    i = i.wrapping_add(1);
                }
                node = *(lists.chains1).offset(maxbitlen.wrapping_sub(1) as isize);
                while !node.is_null() {
                    i = 0;
                    while i != (*node).index {
                        let ref mut fresh44 =
                            *lengths.offset((*leaves.offset(i as isize)).index as isize);
                        *fresh44 = (*fresh44).wrapping_add(1);
                        i = i.wrapping_add(1);
                    }
                    node = (*node).tail;
                }
            }
            lodepng_free(lists.memory as *mut libc::c_void);
            lodepng_free(lists.freelist as *mut libc::c_void);
            lodepng_free(lists.chains0 as *mut libc::c_void);
            lodepng_free(lists.chains1 as *mut libc::c_void);
        }
        lodepng_free(leaves as *mut libc::c_void);
        return error;
    }
}

extern "C" fn HuffmanTree_makeFromFrequencies(
    mut tree: *mut HuffmanTree,
    mut frequencies: *const u32,
    mut mincodes: u64,
    mut numcodes: u64,
    mut maxbitlen: u32,
) -> u32 {
    unsafe {
        let mut error = 0;
        while *frequencies.offset(numcodes.wrapping_sub(1) as isize) == 0 && numcodes > mincodes {
            numcodes = numcodes.wrapping_sub(1);
        }
        let ref mut fresh45 = (*tree).lengths;
        *fresh45 =
            lodepng_malloc(numcodes.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32;
        if ((*tree).lengths).is_null() {
            return 83;
        };
        (*tree).maxbitlen = maxbitlen;
        (*tree).numcodes = numcodes as u32;
        error = lodepng_huffman_code_lengths((*tree).lengths, frequencies, numcodes, maxbitlen);
        if error == 0 {
            error = HuffmanTree_makeFromLengths2(tree);
        }
        return error;
    }
}

extern "C" fn generateFixedLitLenTree(mut tree: *mut HuffmanTree) -> u32 {
    unsafe {
        let mut i: u32 = 0;
        let mut error = 0;
        let mut bitlen =
            lodepng_malloc(288u64.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32;
        if bitlen.is_null() {
            return 83;
        }
        i = 0;
        while i <= 143 {
            *bitlen.offset(i as isize) = 8;
            i = i.wrapping_add(1);
        }
        i = 144;
        while i <= 255 {
            *bitlen.offset(i as isize) = 9;
            i = i.wrapping_add(1);
        }
        i = 256;
        while i <= 279 {
            *bitlen.offset(i as isize) = 7;
            i = i.wrapping_add(1);
        }
        i = 280;
        while i <= 287 {
            *bitlen.offset(i as isize) = 8;
            i = i.wrapping_add(1);
        }
        error = HuffmanTree_makeFromLengths(tree, bitlen, 288, 15);
        lodepng_free(bitlen as *mut libc::c_void);
        return error;
    }
}

extern "C" fn generateFixedDistanceTree(mut tree: *mut HuffmanTree) -> u32 {
    unsafe {
        let mut i: u32 = 0;
        let mut error = 0;
        let mut bitlen =
            lodepng_malloc(32u64.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32;
        if bitlen.is_null() {
            return 83;
        }
        i = 0;
        while i != 32 {
            *bitlen.offset(i as isize) = 5;
            i = i.wrapping_add(1);
        }
        error = HuffmanTree_makeFromLengths(tree, bitlen, 32, 15);
        lodepng_free(bitlen as *mut libc::c_void);
        return error;
    }
}

extern "C" fn huffmanDecodeSymbol(
    mut reader: *mut LodePNGBitReader,
    mut codetree: *const HuffmanTree,
) -> u32 {
    unsafe {
        let mut code = peekBits(reader, 9) as u16;
        let mut l = *((*codetree).table_len).offset(code as isize) as u16;
        let mut value = *((*codetree).table_value).offset(code as isize);
        if l as u32 <= 9 {
            advanceBits(reader, l as u64);
            return value as u32;
        } else {
            advanceBits(reader, 9);
            value = (value as u32).wrapping_add(peekBits(reader, (l as u32).wrapping_sub(9) as u64))
                as u16;
            advanceBits(
                reader,
                (*((*codetree).table_len).offset(value as isize) as u32).wrapping_sub(9) as u64,
            );
            return *((*codetree).table_value).offset(value as isize) as u32;
        };
    }
}

extern "C" fn getTreeInflateFixed(
    mut tree_ll: *mut HuffmanTree,
    mut tree_d: *mut HuffmanTree,
) -> u32 {
    unsafe {
        let mut error = generateFixedLitLenTree(tree_ll);
        if error != 0 {
            return error;
        }
        return generateFixedDistanceTree(tree_d);
    }
}

extern "C" fn getTreeInflateDynamic(
    mut tree_ll: *mut HuffmanTree,
    mut tree_d: *mut HuffmanTree,
    mut reader: *mut LodePNGBitReader,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut n: u32 = 0;
        let mut HLIT: u32 = 0;
        let mut HDIST: u32 = 0;
        let mut HCLEN: u32 = 0;
        let mut i: u32 = 0;
        let mut bitlen_ll = 0 as *mut u32;
        let mut bitlen_d = 0 as *mut u32;
        let mut bitlen_cl = 0 as *mut u32;
        let mut tree_cl = HuffmanTree {
            codes: 0 as *mut u32,
            lengths: 0 as *mut u32,
            maxbitlen: 0,
            numcodes: 0,
            table_len: 0 as *mut u8,
            table_value: 0 as *mut u16,
        };
        if ((*reader).bitsize).wrapping_sub((*reader).bp) < 14 {
            return 49;
        }
        ensureBits17(reader, 14);
        HLIT = (readBits(reader, 5u64)).wrapping_add(257);
        HDIST = (readBits(reader, 5u64)).wrapping_add(1);
        HCLEN = (readBits(reader, 4u64)).wrapping_add(4);
        bitlen_cl =
            lodepng_malloc(19u64.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32;
        if bitlen_cl.is_null() {
            return 83;
        }
        HuffmanTree_init(&mut tree_cl);
        if error == 0 {
            if lodepng_gtofl(
                (*reader).bp,
                HCLEN.wrapping_mul(3) as u64,
                (*reader).bitsize,
            ) != 0
            {
                error = 50;
            } else {
                i = 0;
                while i != HCLEN {
                    ensureBits9(reader, 3);
                    *bitlen_cl.offset(CLCL_ORDER[i as usize] as isize) = readBits(reader, 3);
                    i = i.wrapping_add(1);
                }
                i = HCLEN;
                while i != 19 {
                    *bitlen_cl.offset(CLCL_ORDER[i as usize] as isize) = 0;
                    i = i.wrapping_add(1);
                }
                error = HuffmanTree_makeFromLengths(&mut tree_cl, bitlen_cl, 19, 7);
                if !(error != 0) {
                    bitlen_ll =
                        lodepng_malloc(288u64.wrapping_mul(::std::mem::size_of::<u32>() as u64))
                            as *mut u32;
                    bitlen_d =
                        lodepng_malloc(32u64.wrapping_mul(::std::mem::size_of::<u32>() as u64))
                            as *mut u32;
                    if bitlen_ll.is_null() || bitlen_d.is_null() {
                        error = 83;
                    } else {
                        lodepng_memset(
                            bitlen_ll as *mut libc::c_void,
                            0,
                            288u64.wrapping_mul(::std::mem::size_of::<u32>() as u64),
                        );
                        lodepng_memset(
                            bitlen_d as *mut libc::c_void,
                            0,
                            32u64.wrapping_mul(::std::mem::size_of::<u32>() as u64),
                        );
                        i = 0;
                        while i < HLIT.wrapping_add(HDIST) {
                            let mut code: u32 = 0;
                            ensureBits25(reader, 22);
                            code = huffmanDecodeSymbol(reader, &mut tree_cl);
                            if code <= 15 {
                                if i < HLIT {
                                    *bitlen_ll.offset(i as isize) = code;
                                } else {
                                    *bitlen_d.offset(i.wrapping_sub(HLIT) as isize) = code;
                                }
                                i = i.wrapping_add(1);
                            } else if code == 16 {
                                let mut replength = 3;
                                let mut value: u32 = 0;
                                if i == 0 {
                                    error = 54;
                                    break;
                                } else {
                                    replength = replength.wrapping_add(readBits(reader, 2));
                                    if i < HLIT.wrapping_add(1) {
                                        value = *bitlen_ll.offset(i.wrapping_sub(1) as isize);
                                    } else {
                                        value = *bitlen_d
                                            .offset(i.wrapping_sub(HLIT).wrapping_sub(1) as isize);
                                    }
                                    n = 0;
                                    while n < replength {
                                        if i >= HLIT.wrapping_add(HDIST) {
                                            error = 13;
                                            break;
                                        } else {
                                            if i < HLIT {
                                                *bitlen_ll.offset(i as isize) = value;
                                            } else {
                                                *bitlen_d.offset(i.wrapping_sub(HLIT) as isize) =
                                                    value;
                                            }
                                            i = i.wrapping_add(1);
                                            n = n.wrapping_add(1);
                                        }
                                    }
                                }
                            } else if code == 17 {
                                let mut replength_0 = 3;
                                replength_0 = replength_0.wrapping_add(readBits(reader, 3));
                                n = 0;
                                while n < replength_0 {
                                    if i >= HLIT.wrapping_add(HDIST) {
                                        error = 14;
                                        break;
                                    } else {
                                        if i < HLIT {
                                            *bitlen_ll.offset(i as isize) = 0;
                                        } else {
                                            *bitlen_d.offset(i.wrapping_sub(HLIT) as isize) = 0;
                                        }
                                        i = i.wrapping_add(1);
                                        n = n.wrapping_add(1);
                                    }
                                }
                            } else if code == 18 {
                                let mut replength_1 = 11;
                                replength_1 = replength_1.wrapping_add(readBits(reader, 7));
                                n = 0;
                                while n < replength_1 {
                                    if i >= HLIT.wrapping_add(HDIST) {
                                        error = 15;
                                        break;
                                    } else {
                                        if i < HLIT {
                                            *bitlen_ll.offset(i as isize) = 0;
                                        } else {
                                            *bitlen_d.offset(i.wrapping_sub(HLIT) as isize) = 0;
                                        }
                                        i = i.wrapping_add(1);
                                        n = n.wrapping_add(1);
                                    }
                                }
                            } else {
                                error = 16;
                                break;
                            }
                            if !((*reader).bp > (*reader).bitsize) {
                                continue;
                            }
                            error = 50;
                            break;
                        }
                        if !(error != 0) {
                            if *bitlen_ll.offset(256 as isize) == 0 {
                                error = 64;
                            } else {
                                error = HuffmanTree_makeFromLengths(tree_ll, bitlen_ll, 288, 15);
                                if !(error != 0) {
                                    error = HuffmanTree_makeFromLengths(tree_d, bitlen_d, 32, 15);
                                }
                            }
                        }
                    }
                }
            }
        }
        lodepng_free(bitlen_cl as *mut libc::c_void);
        lodepng_free(bitlen_ll as *mut libc::c_void);
        lodepng_free(bitlen_d as *mut libc::c_void);
        HuffmanTree_cleanup(&mut tree_cl);
        return error;
    }
}

extern "C" fn inflateHuffmanBlock(
    mut out: *mut ucvector,
    mut reader: *mut LodePNGBitReader,
    mut btype: u32,
    mut max_output_size: u64,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut tree_ll = HuffmanTree {
            codes: 0 as *mut u32,
            lengths: 0 as *mut u32,
            maxbitlen: 0,
            numcodes: 0,
            table_len: 0 as *mut u8,
            table_value: 0 as *mut u16,
        };
        let mut tree_d = HuffmanTree {
            codes: 0 as *mut u32,
            lengths: 0 as *mut u32,
            maxbitlen: 0,
            numcodes: 0,
            table_len: 0 as *mut u8,
            table_value: 0 as *mut u16,
        };
        let reserved_size = 260;
        let mut done = 0;
        if ucvector_reserve(out, ((*out).size).wrapping_add(reserved_size)) == 0 {
            return 83;
        }
        HuffmanTree_init(&mut tree_ll);
        HuffmanTree_init(&mut tree_d);
        if btype == 1 {
            error = getTreeInflateFixed(&mut tree_ll, &mut tree_d);
        } else {
            error = getTreeInflateDynamic(&mut tree_ll, &mut tree_d, reader);
        }
        while error == 0 && done == 0 {
            let mut code_ll: u32 = 0;
            ensureBits32(reader, 30);
            code_ll = huffmanDecodeSymbol(reader, &mut tree_ll);
            if code_ll <= 255 {
                let ref mut fresh46 = (*out).size;
                let fresh47 = *fresh46;
                *fresh46 = (*fresh46).wrapping_add(1);
                *((*out).data).offset(fresh47 as isize) = code_ll as u8;
                code_ll = huffmanDecodeSymbol(reader, &mut tree_ll);
            }
            if code_ll <= 255 {
                let ref mut fresh48 = (*out).size;
                let fresh49 = *fresh48;
                *fresh48 = (*fresh48).wrapping_add(1);
                *((*out).data).offset(fresh49 as isize) = code_ll as u8;
            } else if code_ll >= 257 && code_ll <= 285 {
                let mut code_d: u32 = 0;
                let mut distance: u32 = 0;
                let mut numextrabits_l: u32 = 0;
                let mut numextrabits_d: u32 = 0;
                let mut start: u64 = 0;
                let mut backward: u64 = 0;
                let mut length: u64 = 0;
                length = LENGTHBASE[code_ll.wrapping_sub(257) as usize] as u64;
                numextrabits_l = LENGTHEXTRA[code_ll.wrapping_sub(257) as usize];
                if numextrabits_l != 0 {
                    ensureBits25(reader, 5);
                    length = (length).wrapping_add(readBits(reader, numextrabits_l as u64) as u64)
                        as u64;
                }
                ensureBits32(reader, 28);
                code_d = huffmanDecodeSymbol(reader, &mut tree_d);
                if code_d > 29 {
                    if code_d <= 31 {
                        error = 18;
                        break;
                    } else {
                        error = 16;
                        break;
                    }
                } else {
                    distance = DISTANCEBASE[code_d as usize];
                    numextrabits_d = DISTANCEEXTRA[code_d as usize];
                    if numextrabits_d != 0 {
                        distance = distance.wrapping_add(readBits(reader, numextrabits_d as u64));
                    }
                    start = (*out).size;
                    if distance as u64 > start {
                        error = 52;
                        break;
                    } else {
                        backward = start.wrapping_sub(distance as u64);
                        let ref mut fresh50 = (*out).size;
                        *fresh50 = (*fresh50 as u64).wrapping_add(length) as u64;
                        if (distance as u64) < length {
                            let mut forward: u64 = 0;
                            lodepng_memcpy(
                                ((*out).data).offset(start as isize) as *mut libc::c_void,
                                ((*out).data).offset(backward as isize) as *const libc::c_void,
                                distance as u64,
                            );
                            start = (start).wrapping_add(distance as u64) as u64;
                            forward = distance as u64;
                            while forward < length {
                                let fresh51 = backward;
                                backward = backward.wrapping_add(1);
                                let fresh52 = start;
                                start = start.wrapping_add(1);
                                *((*out).data).offset(fresh52 as isize) =
                                    *((*out).data).offset(fresh51 as isize);
                                forward = forward.wrapping_add(1);
                            }
                        } else {
                            lodepng_memcpy(
                                ((*out).data).offset(start as isize) as *mut libc::c_void,
                                ((*out).data).offset(backward as isize) as *const libc::c_void,
                                length,
                            );
                        }
                    }
                }
            } else if code_ll == 256 {
                done = 1;
            } else {
                error = 16;
                break;
            }
            if ((*out).allocsize).wrapping_sub((*out).size) < reserved_size {
                if ucvector_reserve(out, ((*out).size).wrapping_add(reserved_size)) == 0 {
                    error = 83;
                    break;
                }
            }
            if (*reader).bp > (*reader).bitsize {
                error = 51;
                break;
            } else {
                if !(max_output_size != 0 && (*out).size > max_output_size) {
                    continue;
                }
                error = 109;
                break;
            }
        }
        HuffmanTree_cleanup(&mut tree_ll);
        HuffmanTree_cleanup(&mut tree_d);
        return error;
    }
}

extern "C" fn inflateNoCompression(
    mut out: *mut ucvector,
    mut reader: *mut LodePNGBitReader,
    mut settings: *const LodePNGDecompressSettings,
) -> u32 {
    unsafe {
        let mut bytepos: u64 = 0;
        let mut size = (*reader).size;
        let mut LEN: u32 = 0;
        let mut NLEN: u32 = 0;
        let mut error = 0;
        bytepos = ((*reader).bp).wrapping_add(7) >> 3;
        if bytepos.wrapping_add(4) >= size {
            return 52;
        }
        LEN = (*((*reader).data).offset(bytepos as isize) as u32)
            .wrapping_add((*((*reader).data).offset(bytepos.wrapping_add(1) as isize) as u32) << 8);
        bytepos = (bytepos).wrapping_add(2) as u64;
        NLEN = (*((*reader).data).offset(bytepos as isize) as u32)
            .wrapping_add((*((*reader).data).offset(bytepos.wrapping_add(1) as isize) as u32) << 8);
        bytepos = (bytepos).wrapping_add(2) as u64;
        if (*settings).ignore_nlen == 0 && LEN.wrapping_add(NLEN) != 65535 {
            return 21;
        }
        if ucvector_resize(out, ((*out).size).wrapping_add(LEN as u64)) == 0 {
            return 83;
        }
        if bytepos.wrapping_add(LEN as u64) > size {
            return 23;
        }
        if LEN != 0 {
            lodepng_memcpy(
                ((*out).data)
                    .offset((*out).size as isize)
                    .offset(-(LEN as isize)) as *mut libc::c_void,
                ((*reader).data).offset(bytepos as isize) as *const libc::c_void,
                LEN as u64,
            );
            bytepos = (bytepos).wrapping_add(LEN as u64) as u64;
        };
        (*reader).bp = bytepos << 3;
        return error;
    }
}

extern "C" fn lodepng_inflatev(
    mut out: *mut ucvector,
    mut in_0: *const u8,
    mut insize: u64,
    mut settings: *const LodePNGDecompressSettings,
) -> u32 {
    unsafe {
        let mut BFINAL = 0;
        let mut reader = LodePNGBitReader {
            data: 0 as *const u8,
            size: 0,
            bitsize: 0,
            bp: 0,
            buffer: 0,
        };
        let mut error = LodePNGBitReader_init(&mut reader, in_0, insize);
        if error != 0 {
            return error;
        }
        while BFINAL == 0 {
            let mut BTYPE: u32 = 0;
            if (reader.bitsize).wrapping_sub(reader.bp) < 3 {
                return 52;
            }
            ensureBits9(&mut reader, 3);
            BFINAL = readBits(&mut reader, 1);
            BTYPE = readBits(&mut reader, 2);
            if BTYPE == 3 {
                return 20;
            } else {
                if BTYPE == 0 {
                    error = inflateNoCompression(out, &mut reader, settings);
                } else {
                    error =
                        inflateHuffmanBlock(out, &mut reader, BTYPE, (*settings).max_output_size);
                }
            }
            if error == 0
                && (*settings).max_output_size != 0
                && (*out).size > (*settings).max_output_size
            {
                error = 109;
            }
            if error != 0 {
                break;
            }
        }
        return error;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_inflate(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut in_0: *const u8,
    mut insize: u64,
    mut settings: *const LodePNGDecompressSettings,
) -> u32 {
    unsafe {
        let mut v = ucvector_init(*out, *outsize);
        let mut error = lodepng_inflatev(&mut v, in_0, insize, settings);
        *out = v.data;
        *outsize = v.size;
        return error;
    }
}

extern "C" fn inflatev(
    mut out: *mut ucvector,
    mut in_0: *const u8,
    mut insize: u64,
    mut settings: *const LodePNGDecompressSettings,
) -> u32 {
    unsafe {
        if ((*settings).custom_inflate).is_some() {
            let mut error = ((*settings).custom_inflate).expect("non-null function pointer")(
                &mut (*out).data,
                &mut (*out).size,
                in_0,
                insize,
                settings,
            );
            (*out).allocsize = (*out).size;
            if error != 0 {
                error = 110;
                if (*settings).max_output_size != 0 && (*out).size > (*settings).max_output_size {
                    error = 109;
                }
            }
            return error;
        } else {
            return lodepng_inflatev(out, in_0, insize, settings);
        };
    }
}

static mut MAX_SUPPORTED_DEFLATE_LENGTH: u64 = 258;
extern "C" fn searchCodeIndex(mut array: *const u32, mut array_size: u64, mut value: u64) -> u64 {
    unsafe {
        let mut left = 1;
        let mut right = array_size.wrapping_sub(1);
        while left <= right {
            let mut mid = left.wrapping_add(right) >> 1;
            if *array.offset(mid as isize) as u64 >= value {
                right = mid.wrapping_sub(1);
            } else {
                left = mid.wrapping_add(1);
            }
        }
        if left >= array_size || *array.offset(left as isize) as u64 > value {
            left = left.wrapping_sub(1);
        }
        return left;
    }
}

extern "C" fn addLengthDistance(mut values: *mut uivector, mut length: u64, mut distance: u64) {
    unsafe {
        let mut length_code = searchCodeIndex(LENGTHBASE.as_ptr(), 29, length) as u32;
        let mut extra_length = length.wrapping_sub(LENGTHBASE[length_code as usize] as u64) as u32;
        let mut dist_code = searchCodeIndex(DISTANCEBASE.as_ptr(), 30, distance) as u32;
        let mut extra_distance =
            distance.wrapping_sub(DISTANCEBASE[dist_code as usize] as u64) as u32;
        let mut pos = (*values).size;
        let mut ok = uivector_resize(values, ((*values).size).wrapping_add(4));
        if ok != 0 {
            *((*values).data).offset(pos.wrapping_add(0) as isize) = length_code.wrapping_add(257);
            *((*values).data).offset(pos.wrapping_add(1) as isize) = extra_length;
            *((*values).data).offset(pos.wrapping_add(2) as isize) = dist_code;
            *((*values).data).offset(pos.wrapping_add(3) as isize) = extra_distance;
        }
    }
}

static mut HASH_NUM_VALUES: u32 = 65536;
static mut HASH_BIT_MASK: u32 = 65535;
extern "C" fn hash_init(mut hash: *mut Hash, mut windowsize: u32) -> u32 {
    unsafe {
        let mut i: u32 = 0;
        let ref mut fresh53 = (*hash).head;
        *fresh53 = lodepng_malloc(
            (::std::mem::size_of::<i32>() as u64).wrapping_mul(HASH_NUM_VALUES as u64),
        ) as *mut i32;
        let ref mut fresh54 = (*hash).val;
        *fresh54 =
            lodepng_malloc((::std::mem::size_of::<i32>() as u64).wrapping_mul(windowsize as u64))
                as *mut i32;
        let ref mut fresh55 = (*hash).chain;
        *fresh55 =
            lodepng_malloc((::std::mem::size_of::<u16>() as u64).wrapping_mul(windowsize as u64))
                as *mut u16;
        let ref mut fresh56 = (*hash).zeros;
        *fresh56 =
            lodepng_malloc((::std::mem::size_of::<u16>() as u64).wrapping_mul(windowsize as u64))
                as *mut u16;
        let ref mut fresh57 = (*hash).headz;
        *fresh57 = lodepng_malloc(
            (::std::mem::size_of::<i32>() as u64)
                .wrapping_mul(MAX_SUPPORTED_DEFLATE_LENGTH.wrapping_add(1)),
        ) as *mut i32;
        let ref mut fresh58 = (*hash).chainz;
        *fresh58 =
            lodepng_malloc((::std::mem::size_of::<u16>() as u64).wrapping_mul(windowsize as u64))
                as *mut u16;
        if ((*hash).head).is_null()
            || ((*hash).chain).is_null()
            || ((*hash).val).is_null()
            || ((*hash).headz).is_null()
            || ((*hash).chainz).is_null()
            || ((*hash).zeros).is_null()
        {
            return 83;
        }
        i = 0;
        while i != HASH_NUM_VALUES {
            *((*hash).head).offset(i as isize) = -1;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i != windowsize {
            *((*hash).val).offset(i as isize) = -1;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i != windowsize {
            *((*hash).chain).offset(i as isize) = i as u16;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i as u64 <= MAX_SUPPORTED_DEFLATE_LENGTH {
            *((*hash).headz).offset(i as isize) = -1;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i != windowsize {
            *((*hash).chainz).offset(i as isize) = i as u16;
            i = i.wrapping_add(1);
        }
        return 0;
    }
}

extern "C" fn hash_cleanup(mut hash: *mut Hash) {
    unsafe {
        lodepng_free((*hash).head as *mut libc::c_void);
        lodepng_free((*hash).val as *mut libc::c_void);
        lodepng_free((*hash).chain as *mut libc::c_void);
        lodepng_free((*hash).zeros as *mut libc::c_void);
        lodepng_free((*hash).headz as *mut libc::c_void);
        lodepng_free((*hash).chainz as *mut libc::c_void);
    }
}

extern "C" fn getHash(mut data: *const u8, mut size: u64, mut pos: u64) -> u32 {
    unsafe {
        let mut result = 0;
        if pos.wrapping_add(2) < size {
            result ^= (*data.offset(pos.wrapping_add(0) as isize) as u32) << 0;
            result ^= (*data.offset(pos.wrapping_add(1) as isize) as u32) << 4;
            result ^= (*data.offset(pos.wrapping_add(2) as isize) as u32) << 8;
        } else {
            let mut amount: u64 = 0;
            let mut i: u64 = 0;
            if pos >= size {
                return 0;
            }
            amount = size.wrapping_sub(pos);
            i = 0;
            while i != amount {
                result ^= (*data.offset(pos.wrapping_add(i) as isize) as u32) << i.wrapping_mul(8);
                i = i.wrapping_add(1);
            }
        }
        return result & HASH_BIT_MASK;
    }
}

extern "C" fn countZeros(mut data: *const u8, mut size: u64, mut pos: u64) -> u32 {
    unsafe {
        let mut start = data.offset(pos as isize);
        let mut end = start.offset(MAX_SUPPORTED_DEFLATE_LENGTH as isize);
        if end > data.offset(size as isize) {
            end = data.offset(size as isize);
        }
        data = start;
        while data != end && *data as i32 == 0 {
            data = data.offset(1);
        }
        return data.offset_from(start) as u32;
    }
}

extern "C" fn updateHashChain(
    mut hash: *mut Hash,
    mut wpos: u64,
    mut hashval: u32,
    mut numzeros: u16,
) {
    unsafe {
        *((*hash).val).offset(wpos as isize) = hashval as i32;
        if *((*hash).head).offset(hashval as isize) != -1 {
            *((*hash).chain).offset(wpos as isize) =
                *((*hash).head).offset(hashval as isize) as u16;
        };
        *((*hash).head).offset(hashval as isize) = wpos as i32;
        *((*hash).zeros).offset(wpos as isize) = numzeros;
        if *((*hash).headz).offset(numzeros as isize) != -1 {
            *((*hash).chainz).offset(wpos as isize) =
                *((*hash).headz).offset(numzeros as isize) as u16;
        };
        *((*hash).headz).offset(numzeros as isize) = wpos as i32;
    }
}

extern "C" fn encodeLZ77(
    mut out: *mut uivector,
    mut hash: *mut Hash,
    mut in_0: *const u8,
    mut inpos: u64,
    mut insize: u64,
    mut windowsize: u32,
    mut minmatch: u32,
    mut nicematch: u32,
    mut lazymatching: u32,
) -> u32 {
    unsafe {
        let mut pos: u64 = 0;
        let mut i: u32 = 0;
        let mut error = 0;
        let mut maxchainlength = if windowsize >= 8192 {
            windowsize
        } else {
            windowsize.wrapping_div(8)
        };
        let mut maxlazymatch = (if windowsize >= 8192 {
            MAX_SUPPORTED_DEFLATE_LENGTH
        } else {
            64
        }) as u32;
        let mut usezeros = 1;
        let mut numzeros = 0;
        let mut offset: u32 = 0;
        let mut length: u32 = 0;
        let mut lazy = 0;
        let mut lazylength = 0;
        let mut lazyoffset = 0;
        let mut hashval: u32 = 0;
        let mut current_offset: u32 = 0;
        let mut current_length: u32 = 0;
        let mut prev_offset: u32 = 0;
        let mut lastptr = 0 as *const u8;
        let mut foreptr = 0 as *const u8;
        let mut backptr = 0 as *const u8;
        let mut hashpos: u32 = 0;
        if windowsize == 0 || windowsize > 32768 {
            return 60;
        }
        if windowsize & windowsize.wrapping_sub(1) != 0 {
            return 90;
        }
        if nicematch as u64 > MAX_SUPPORTED_DEFLATE_LENGTH {
            nicematch = MAX_SUPPORTED_DEFLATE_LENGTH as u32;
        }
        let mut current_block_78: u64;
        pos = inpos;
        while pos < insize {
            let mut wpos = pos & windowsize.wrapping_sub(1) as u64;
            let mut chainlength = 0;
            hashval = getHash(in_0, insize, pos);
            if usezeros != 0 && hashval == 0 {
                if numzeros == 0 {
                    numzeros = countZeros(in_0, insize, pos);
                } else if pos.wrapping_add(numzeros as u64) > insize
                    || *in_0.offset(pos.wrapping_add(numzeros as u64).wrapping_sub(1) as isize)
                        as i32
                        != 0
                {
                    numzeros = numzeros.wrapping_sub(1);
                }
            } else {
                numzeros = 0;
            }
            updateHashChain(hash, wpos, hashval, numzeros as u16);
            length = 0;
            offset = 0;
            hashpos = *((*hash).chain).offset(wpos as isize) as u32;
            lastptr = &*in_0.offset(
                (if insize < pos.wrapping_add(MAX_SUPPORTED_DEFLATE_LENGTH) {
                    insize
                } else {
                    pos.wrapping_add(MAX_SUPPORTED_DEFLATE_LENGTH)
                }) as isize,
            ) as *const u8;
            prev_offset = 0;
            loop {
                let fresh59 = chainlength;
                chainlength = chainlength.wrapping_add(1);
                if fresh59 >= maxchainlength {
                    break;
                }
                current_offset = (if hashpos as u64 <= wpos {
                    wpos.wrapping_sub(hashpos as u64)
                } else {
                    wpos.wrapping_sub(hashpos as u64)
                        .wrapping_add(windowsize as u64)
                }) as u32;
                if current_offset < prev_offset {
                    break;
                }
                prev_offset = current_offset;
                if current_offset > 0 {
                    foreptr = &*in_0.offset(pos as isize) as *const u8;
                    backptr = &*in_0.offset(pos.wrapping_sub(current_offset as u64) as isize)
                        as *const u8;
                    if numzeros >= 3 {
                        let mut skip = *((*hash).zeros).offset(hashpos as isize) as u32;
                        if skip > numzeros {
                            skip = numzeros;
                        }
                        backptr = backptr.offset(skip as isize);
                        foreptr = foreptr.offset(skip as isize);
                    }
                    while foreptr != lastptr && *backptr as i32 == *foreptr as i32 {
                        backptr = backptr.offset(1);
                        foreptr = foreptr.offset(1);
                    }
                    current_length =
                        foreptr.offset_from(&*in_0.offset(pos as isize) as *const u8) as u32;
                    if current_length > length {
                        length = current_length;
                        offset = current_offset;
                        if current_length >= nicematch {
                            break;
                        }
                    }
                }
                if hashpos == *((*hash).chain).offset(hashpos as isize) as u32 {
                    break;
                }
                if numzeros >= 3 && length > numzeros {
                    hashpos = *((*hash).chainz).offset(hashpos as isize) as u32;
                    if *((*hash).zeros).offset(hashpos as isize) as u32 != numzeros {
                        break;
                    }
                } else {
                    hashpos = *((*hash).chain).offset(hashpos as isize) as u32;
                    if *((*hash).val).offset(hashpos as isize) != hashval as i32 {
                        break;
                    }
                }
            }
            if lazymatching != 0 {
                if lazy == 0
                    && length >= 3
                    && length <= maxlazymatch
                    && (length as u64) < MAX_SUPPORTED_DEFLATE_LENGTH
                {
                    lazy = 1;
                    lazylength = length;
                    lazyoffset = offset;
                    current_block_78 = 8236137900636309791;
                } else if lazy != 0 {
                    lazy = 0;
                    if pos == 0 {
                        error = 81;
                        break;
                    } else if length > lazylength.wrapping_add(1) {
                        if uivector_push_back(
                            out,
                            *in_0.offset(pos.wrapping_sub(1) as isize) as u32,
                        ) == 0
                        {
                            error = 83;
                            break;
                        }
                    } else {
                        length = lazylength;
                        offset = lazyoffset;
                        *((*hash).head).offset(hashval as isize) = -1;
                        *((*hash).headz).offset(numzeros as isize) = -1;
                        pos = pos.wrapping_sub(1);
                    }
                    current_block_78 = 8716029205547827362;
                } else {
                    current_block_78 = 8716029205547827362;
                }
            } else {
                current_block_78 = 8716029205547827362;
            }
            match current_block_78 {
                8716029205547827362 => {
                    if length >= 3 && offset > windowsize {
                        error = 86;
                        break;
                    } else if length < 3 {
                        if uivector_push_back(out, *in_0.offset(pos as isize) as u32) == 0 {
                            error = 83;
                            break;
                        }
                    } else if length < minmatch || length == 3 && offset > 4096 {
                        if uivector_push_back(out, *in_0.offset(pos as isize) as u32) == 0 {
                            error = 83;
                            break;
                        }
                    } else {
                        addLengthDistance(out, length as u64, offset as u64);
                        i = 1;
                        while i < length {
                            pos = pos.wrapping_add(1);
                            wpos = pos & windowsize.wrapping_sub(1) as u64;
                            hashval = getHash(in_0, insize, pos);
                            if usezeros != 0 && hashval == 0 {
                                if numzeros == 0 {
                                    numzeros = countZeros(in_0, insize, pos);
                                } else if pos.wrapping_add(numzeros as u64) > insize
                                    || *in_0
                                        .offset(pos.wrapping_add(numzeros as u64).wrapping_sub(1)
                                            as isize) as i32
                                        != 0
                                {
                                    numzeros = numzeros.wrapping_sub(1);
                                }
                            } else {
                                numzeros = 0;
                            }
                            updateHashChain(hash, wpos, hashval, numzeros as u16);
                            i = i.wrapping_add(1);
                        }
                    }
                }
                _ => {}
            }
            pos = pos.wrapping_add(1);
        }
        return error;
    }
}

extern "C" fn deflateNoCompression(
    mut out: *mut ucvector,
    mut data: *const u8,
    mut datasize: u64,
) -> u32 {
    unsafe {
        let mut i: u64 = 0;
        let mut numdeflateblocks = datasize.wrapping_add(65534).wrapping_div(65535);
        let mut datapos = 0;
        i = 0;
        while i != numdeflateblocks {
            let mut BFINAL: u32 = 0;
            let mut BTYPE: u32 = 0;
            let mut LEN: u32 = 0;
            let mut NLEN: u32 = 0;
            let mut firstbyte: u8 = 0;
            let mut pos = (*out).size;
            BFINAL = (i == numdeflateblocks.wrapping_sub(1)) as u32;
            BTYPE = 0;
            LEN = 65535;
            if datasize.wrapping_sub(datapos as u64) < 65535 {
                LEN = (datasize as u32).wrapping_sub(datapos);
            }
            NLEN = 65535u32.wrapping_sub(LEN);
            if ucvector_resize(out, ((*out).size).wrapping_add(LEN as u64).wrapping_add(5)) == 0 {
                return 83;
            }
            firstbyte = BFINAL
                .wrapping_add((BTYPE & 1) << 1)
                .wrapping_add((BTYPE & 2) << 1) as u8;
            *((*out).data).offset(pos.wrapping_add(0) as isize) = firstbyte;
            *((*out).data).offset(pos.wrapping_add(1) as isize) = (LEN & 255u32) as u8;
            *((*out).data).offset(pos.wrapping_add(2) as isize) = (LEN >> 8u32) as u8;
            *((*out).data).offset(pos.wrapping_add(3) as isize) = (NLEN & 255u32) as u8;
            *((*out).data).offset(pos.wrapping_add(4) as isize) = (NLEN >> 8u32) as u8;
            lodepng_memcpy(
                ((*out).data).offset(pos as isize).offset(5 as isize) as *mut libc::c_void,
                data.offset(datapos as isize) as *const libc::c_void,
                LEN as u64,
            );
            datapos = datapos.wrapping_add(LEN);
            i = i.wrapping_add(1);
        }
        return 0;
    }
}

extern "C" fn writeLZ77data(
    mut writer: *mut LodePNGBitWriter,
    mut lz77_encoded: *const uivector,
    mut tree_ll: *const HuffmanTree,
    mut tree_d: *const HuffmanTree,
) {
    unsafe {
        let mut i = 0;
        i = 0;
        while i != (*lz77_encoded).size {
            let mut val = *((*lz77_encoded).data).offset(i as isize);
            writeBitsReversed(
                writer,
                *((*tree_ll).codes).offset(val as isize),
                *((*tree_ll).lengths).offset(val as isize) as u64,
            );
            if val > 256 {
                let mut length_index = val.wrapping_sub(257);
                let mut n_length_extra_bits = LENGTHEXTRA[length_index as usize];
                i = i.wrapping_add(1);
                let mut length_extra_bits = *((*lz77_encoded).data).offset(i as isize);
                i = i.wrapping_add(1);
                let mut distance_code = *((*lz77_encoded).data).offset(i as isize);
                let mut distance_index = distance_code;
                let mut n_distance_extra_bits = DISTANCEEXTRA[distance_index as usize];
                i = i.wrapping_add(1);
                let mut distance_extra_bits = *((*lz77_encoded).data).offset(i as isize);
                writeBits(writer, length_extra_bits, n_length_extra_bits as u64);
                writeBitsReversed(
                    writer,
                    *((*tree_d).codes).offset(distance_code as isize),
                    *((*tree_d).lengths).offset(distance_code as isize) as u64,
                );
                writeBits(writer, distance_extra_bits, n_distance_extra_bits as u64);
            }
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn deflateDynamic(
    mut writer: *mut LodePNGBitWriter,
    mut hash: *mut Hash,
    mut data: *const u8,
    mut datapos: u64,
    mut dataend: u64,
    mut settings: *const LodePNGCompressSettings,
    mut final_0: u32,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut lz77_encoded = uivector {
            data: 0 as *mut u32,
            size: 0,
            allocsize: 0,
        };
        let mut tree_ll = HuffmanTree {
            codes: 0 as *mut u32,
            lengths: 0 as *mut u32,
            maxbitlen: 0,
            numcodes: 0,
            table_len: 0 as *mut u8,
            table_value: 0 as *mut u16,
        };
        let mut tree_d = HuffmanTree {
            codes: 0 as *mut u32,
            lengths: 0 as *mut u32,
            maxbitlen: 0,
            numcodes: 0,
            table_len: 0 as *mut u8,
            table_value: 0 as *mut u16,
        };
        let mut tree_cl = HuffmanTree {
            codes: 0 as *mut u32,
            lengths: 0 as *mut u32,
            maxbitlen: 0,
            numcodes: 0,
            table_len: 0 as *mut u8,
            table_value: 0 as *mut u16,
        };
        let mut frequencies_ll = 0 as *mut u32;
        let mut frequencies_d = 0 as *mut u32;
        let mut frequencies_cl = 0 as *mut u32;
        let mut bitlen_lld = 0 as *mut u32;
        let mut bitlen_lld_e = 0 as *mut u32;
        let mut datasize = dataend.wrapping_sub(datapos);
        let mut BFINAL = final_0;
        let mut i: u64 = 0;
        let mut numcodes_ll: u64 = 0;
        let mut numcodes_d: u64 = 0;
        let mut numcodes_lld: u64 = 0;
        let mut numcodes_lld_e: u64 = 0;
        let mut numcodes_cl: u64 = 0;
        let mut HLIT: u32 = 0;
        let mut HDIST: u32 = 0;
        let mut HCLEN: u32 = 0;
        uivector_init(&mut lz77_encoded);
        HuffmanTree_init(&mut tree_ll);
        HuffmanTree_init(&mut tree_d);
        HuffmanTree_init(&mut tree_cl);
        frequencies_ll =
            lodepng_malloc(286u64.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32;
        frequencies_d =
            lodepng_malloc(30u64.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32;
        frequencies_cl =
            lodepng_malloc(19u64.wrapping_mul(::std::mem::size_of::<u32>() as u64)) as *mut u32;
        if frequencies_ll.is_null() || frequencies_d.is_null() || frequencies_cl.is_null() {
            error = 83;
        }
        let mut current_block_113: u64;
        if error == 0 {
            lodepng_memset(
                frequencies_ll as *mut libc::c_void,
                0,
                286u64.wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
            lodepng_memset(
                frequencies_d as *mut libc::c_void,
                0,
                30u64.wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
            lodepng_memset(
                frequencies_cl as *mut libc::c_void,
                0,
                19u64.wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
            if (*settings).use_lz77 != 0 {
                error = encodeLZ77(
                    &mut lz77_encoded,
                    hash,
                    data,
                    datapos,
                    dataend,
                    (*settings).windowsize,
                    (*settings).minmatch,
                    (*settings).nicematch,
                    (*settings).lazymatching,
                );
                if error != 0 {
                    current_block_113 = 6988365858197790817;
                } else {
                    current_block_113 = 17788412896529399552;
                }
            } else if uivector_resize(&mut lz77_encoded, datasize) == 0 {
                error = 83;
                current_block_113 = 6988365858197790817;
            } else {
                i = datapos;
                while i < dataend {
                    *(lz77_encoded.data).offset(i.wrapping_sub(datapos) as isize) =
                        *data.offset(i as isize) as u32;
                    i = i.wrapping_add(1);
                }
                current_block_113 = 17788412896529399552;
            }
            match current_block_113 {
                6988365858197790817 => {}
                _ => {
                    i = 0;
                    while i != lz77_encoded.size {
                        let mut symbol = *(lz77_encoded.data).offset(i as isize);
                        let ref mut fresh60 = *frequencies_ll.offset(symbol as isize);
                        *fresh60 = (*fresh60).wrapping_add(1);
                        if symbol > 256 {
                            let mut dist = *(lz77_encoded.data).offset(i.wrapping_add(2) as isize);
                            let ref mut fresh61 = *frequencies_d.offset(dist as isize);
                            *fresh61 = (*fresh61).wrapping_add(1);
                            i = (i).wrapping_add(3) as u64;
                        }
                        i = i.wrapping_add(1);
                    }
                    *frequencies_ll.offset(256 as isize) = 1;
                    error =
                        HuffmanTree_makeFromFrequencies(&mut tree_ll, frequencies_ll, 257, 286, 15);
                    if !(error != 0) {
                        error =
                            HuffmanTree_makeFromFrequencies(&mut tree_d, frequencies_d, 2, 30, 15);
                        if !(error != 0) {
                            numcodes_ll = (if tree_ll.numcodes < 286 {
                                tree_ll.numcodes
                            } else {
                                286
                            }) as u64;
                            numcodes_d = (if tree_d.numcodes < 30 {
                                tree_d.numcodes
                            } else {
                                30
                            }) as u64;
                            numcodes_lld = numcodes_ll.wrapping_add(numcodes_d);
                            bitlen_lld = lodepng_malloc(
                                numcodes_lld.wrapping_mul(::std::mem::size_of::<u32>() as u64),
                            ) as *mut u32;
                            bitlen_lld_e = lodepng_malloc(
                                numcodes_lld.wrapping_mul(::std::mem::size_of::<u32>() as u64),
                            ) as *mut u32;
                            if bitlen_lld.is_null() || bitlen_lld_e.is_null() {
                                error = 83;
                            } else {
                                numcodes_lld_e = 0;
                                i = 0;
                                while i != numcodes_ll {
                                    *bitlen_lld.offset(i as isize) =
                                        *(tree_ll.lengths).offset(i as isize);
                                    i = i.wrapping_add(1);
                                }
                                i = 0;
                                while i != numcodes_d {
                                    *bitlen_lld.offset(numcodes_ll.wrapping_add(i) as isize) =
                                        *(tree_d.lengths).offset(i as isize);
                                    i = i.wrapping_add(1);
                                }
                                i = 0;
                                while i != numcodes_lld {
                                    let mut j = 0;
                                    while i.wrapping_add(j as u64).wrapping_add(1) < numcodes_lld
                                        && *bitlen_lld.offset(
                                            i.wrapping_add(j as u64).wrapping_add(1) as isize,
                                        ) == *bitlen_lld.offset(i as isize)
                                    {
                                        j = j.wrapping_add(1);
                                    }
                                    if *bitlen_lld.offset(i as isize) == 0 && j >= 2 {
                                        j = j.wrapping_add(1);
                                        if j <= 10 {
                                            let fresh62 = numcodes_lld_e;
                                            numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                            *bitlen_lld_e.offset(fresh62 as isize) = 17;
                                            let fresh63 = numcodes_lld_e;
                                            numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                            *bitlen_lld_e.offset(fresh63 as isize) =
                                                j.wrapping_sub(3);
                                        } else {
                                            if j > 138 {
                                                j = 138;
                                            }
                                            let fresh64 = numcodes_lld_e;
                                            numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                            *bitlen_lld_e.offset(fresh64 as isize) = 18;
                                            let fresh65 = numcodes_lld_e;
                                            numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                            *bitlen_lld_e.offset(fresh65 as isize) =
                                                j.wrapping_sub(11);
                                        }
                                        i = (i).wrapping_add(j.wrapping_sub(1) as u64) as u64;
                                    } else if j >= 3 {
                                        let mut k: u64 = 0;
                                        let mut num = j.wrapping_div(6);
                                        let mut rest = j.wrapping_rem(6);
                                        let fresh66 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e.offset(fresh66 as isize) =
                                            *bitlen_lld.offset(i as isize);
                                        k = 0;
                                        while k < num as u64 {
                                            let fresh67 = numcodes_lld_e;
                                            numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                            *bitlen_lld_e.offset(fresh67 as isize) = 16;
                                            let fresh68 = numcodes_lld_e;
                                            numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                            *bitlen_lld_e.offset(fresh68 as isize) =
                                                (6 - 3i32) as u32;
                                            k = k.wrapping_add(1);
                                        }
                                        if rest >= 3 {
                                            let fresh69 = numcodes_lld_e;
                                            numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                            *bitlen_lld_e.offset(fresh69 as isize) = 16;
                                            let fresh70 = numcodes_lld_e;
                                            numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                            *bitlen_lld_e.offset(fresh70 as isize) =
                                                rest.wrapping_sub(3);
                                        } else {
                                            j = j.wrapping_sub(rest);
                                        }
                                        i = (i).wrapping_add(j as u64) as u64;
                                    } else {
                                        let fresh71 = numcodes_lld_e;
                                        numcodes_lld_e = numcodes_lld_e.wrapping_add(1);
                                        *bitlen_lld_e.offset(fresh71 as isize) =
                                            *bitlen_lld.offset(i as isize);
                                    }
                                    i = i.wrapping_add(1);
                                }
                                i = 0;
                                while i != numcodes_lld_e {
                                    let ref mut fresh72 = *frequencies_cl
                                        .offset(*bitlen_lld_e.offset(i as isize) as isize);
                                    *fresh72 = (*fresh72).wrapping_add(1);
                                    if *bitlen_lld_e.offset(i as isize) >= 16 {
                                        i = i.wrapping_add(1);
                                    }
                                    i = i.wrapping_add(1);
                                }
                                error = HuffmanTree_makeFromFrequencies(
                                    &mut tree_cl,
                                    frequencies_cl,
                                    19,
                                    19,
                                    7,
                                );
                                if !(error != 0) {
                                    numcodes_cl = 19;
                                    while numcodes_cl > 4
                                        && *(tree_cl.lengths).offset(
                                            CLCL_ORDER[numcodes_cl.wrapping_sub(1) as usize]
                                                as isize,
                                        ) == 0
                                    {
                                        numcodes_cl = numcodes_cl.wrapping_sub(1);
                                    }
                                    writeBits(writer, BFINAL, 1);
                                    writeBits(writer, 0, 1);
                                    writeBits(writer, 1, 1);
                                    HLIT = numcodes_ll.wrapping_sub(257) as u32;
                                    HDIST = numcodes_d.wrapping_sub(1) as u32;
                                    HCLEN = numcodes_cl.wrapping_sub(4) as u32;
                                    writeBits(writer, HLIT, 5);
                                    writeBits(writer, HDIST, 5);
                                    writeBits(writer, HCLEN, 4);
                                    i = 0;
                                    while i != numcodes_cl {
                                        writeBits(
                                            writer,
                                            *(tree_cl.lengths)
                                                .offset(CLCL_ORDER[i as usize] as isize),
                                            3,
                                        );
                                        i = i.wrapping_add(1);
                                    }
                                    i = 0;
                                    while i != numcodes_lld_e {
                                        writeBitsReversed(
                                            writer,
                                            *(tree_cl.codes)
                                                .offset(*bitlen_lld_e.offset(i as isize) as isize),
                                            *(tree_cl.lengths)
                                                .offset(*bitlen_lld_e.offset(i as isize) as isize)
                                                as u64,
                                        );
                                        if *bitlen_lld_e.offset(i as isize) == 16 {
                                            i = i.wrapping_add(1);
                                            writeBits(writer, *bitlen_lld_e.offset(i as isize), 2);
                                        } else if *bitlen_lld_e.offset(i as isize) == 17 {
                                            i = i.wrapping_add(1);
                                            writeBits(writer, *bitlen_lld_e.offset(i as isize), 3);
                                        } else if *bitlen_lld_e.offset(i as isize) == 18 {
                                            i = i.wrapping_add(1);
                                            writeBits(writer, *bitlen_lld_e.offset(i as isize), 7);
                                        }
                                        i = i.wrapping_add(1);
                                    }
                                    writeLZ77data(
                                        writer,
                                        &mut lz77_encoded,
                                        &mut tree_ll,
                                        &mut tree_d,
                                    );
                                    if *(tree_ll.lengths).offset(256 as isize) == 0 {
                                        error = 64;
                                    } else {
                                        writeBitsReversed(
                                            writer,
                                            *(tree_ll.codes).offset(256 as isize),
                                            *(tree_ll.lengths).offset(256 as isize) as u64,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        uivector_cleanup(&mut lz77_encoded as *mut uivector as *mut libc::c_void);
        HuffmanTree_cleanup(&mut tree_ll);
        HuffmanTree_cleanup(&mut tree_d);
        HuffmanTree_cleanup(&mut tree_cl);
        lodepng_free(frequencies_ll as *mut libc::c_void);
        lodepng_free(frequencies_d as *mut libc::c_void);
        lodepng_free(frequencies_cl as *mut libc::c_void);
        lodepng_free(bitlen_lld as *mut libc::c_void);
        lodepng_free(bitlen_lld_e as *mut libc::c_void);
        return error;
    }
}

extern "C" fn deflateFixed(
    mut writer: *mut LodePNGBitWriter,
    mut hash: *mut Hash,
    mut data: *const u8,
    mut datapos: u64,
    mut dataend: u64,
    mut settings: *const LodePNGCompressSettings,
    mut final_0: u32,
) -> u32 {
    unsafe {
        let mut tree_ll = HuffmanTree {
            codes: 0 as *mut u32,
            lengths: 0 as *mut u32,
            maxbitlen: 0,
            numcodes: 0,
            table_len: 0 as *mut u8,
            table_value: 0 as *mut u16,
        };
        let mut tree_d = HuffmanTree {
            codes: 0 as *mut u32,
            lengths: 0 as *mut u32,
            maxbitlen: 0,
            numcodes: 0,
            table_len: 0 as *mut u8,
            table_value: 0 as *mut u16,
        };
        let mut BFINAL = final_0;
        let mut error = 0;
        let mut i: u64 = 0;
        HuffmanTree_init(&mut tree_ll);
        HuffmanTree_init(&mut tree_d);
        error = generateFixedLitLenTree(&mut tree_ll);
        if error == 0 {
            error = generateFixedDistanceTree(&mut tree_d);
        }
        if error == 0 {
            writeBits(writer, BFINAL, 1);
            writeBits(writer, 1, 1);
            writeBits(writer, 0, 1);
            if (*settings).use_lz77 != 0 {
                let mut lz77_encoded = uivector {
                    data: 0 as *mut u32,
                    size: 0,
                    allocsize: 0,
                };
                uivector_init(&mut lz77_encoded);
                error = encodeLZ77(
                    &mut lz77_encoded,
                    hash,
                    data,
                    datapos,
                    dataend,
                    (*settings).windowsize,
                    (*settings).minmatch,
                    (*settings).nicematch,
                    (*settings).lazymatching,
                );
                if error == 0 {
                    writeLZ77data(writer, &mut lz77_encoded, &mut tree_ll, &mut tree_d);
                }
                uivector_cleanup(&mut lz77_encoded as *mut uivector as *mut libc::c_void);
            } else {
                i = datapos;
                while i < dataend {
                    writeBitsReversed(
                        writer,
                        *(tree_ll.codes).offset(*data.offset(i as isize) as isize),
                        *(tree_ll.lengths).offset(*data.offset(i as isize) as isize) as u64,
                    );
                    i = i.wrapping_add(1);
                }
            }
            if error == 0 {
                writeBitsReversed(
                    writer,
                    *(tree_ll.codes).offset(256 as isize),
                    *(tree_ll.lengths).offset(256 as isize) as u64,
                );
            }
        }
        HuffmanTree_cleanup(&mut tree_ll);
        HuffmanTree_cleanup(&mut tree_d);
        return error;
    }
}

extern "C" fn lodepng_deflatev(
    mut out: *mut ucvector,
    mut in_0: *const u8,
    mut insize: u64,
    mut settings: *const LodePNGCompressSettings,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut i: u64 = 0;
        let mut blocksize: u64 = 0;
        let mut numdeflateblocks: u64 = 0;
        let mut hash = Hash {
            head: 0 as *mut i32,
            chain: 0 as *mut u16,
            val: 0 as *mut i32,
            headz: 0 as *mut i32,
            chainz: 0 as *mut u16,
            zeros: 0 as *mut u16,
        };
        let mut writer = LodePNGBitWriter {
            data: 0 as *mut ucvector,
            bp: 0,
        };
        LodePNGBitWriter_init(&mut writer, out);
        if (*settings).btype > 2 {
            return 61;
        } else {
            if (*settings).btype == 0 {
                return deflateNoCompression(out, in_0, insize);
            } else {
                if (*settings).btype == 1 {
                    blocksize = insize;
                } else {
                    blocksize = insize.wrapping_div(8).wrapping_add(8);
                    if blocksize < 65536 {
                        blocksize = 65536;
                    }
                    if blocksize > 262144 {
                        blocksize = 262144;
                    }
                }
            }
        }
        numdeflateblocks = insize
            .wrapping_add(blocksize)
            .wrapping_sub(1)
            .wrapping_div(blocksize);
        if numdeflateblocks == 0 {
            numdeflateblocks = 1;
        }
        error = hash_init(&mut hash, (*settings).windowsize);
        if error == 0 {
            i = 0;
            while i != numdeflateblocks && error == 0 {
                let mut final_0 = (i == numdeflateblocks.wrapping_sub(1)) as u32;
                let mut start = i.wrapping_mul(blocksize);
                let mut end = start.wrapping_add(blocksize);
                if end > insize {
                    end = insize;
                }
                if (*settings).btype == 1 {
                    error =
                        deflateFixed(&mut writer, &mut hash, in_0, start, end, settings, final_0);
                } else if (*settings).btype == 2 {
                    error =
                        deflateDynamic(&mut writer, &mut hash, in_0, start, end, settings, final_0);
                }
                i = i.wrapping_add(1);
            }
        }
        hash_cleanup(&mut hash);
        return error;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_deflate(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut in_0: *const u8,
    mut insize: u64,
    mut settings: *const LodePNGCompressSettings,
) -> u32 {
    unsafe {
        let mut v = ucvector_init(*out, *outsize);
        let mut error = lodepng_deflatev(&mut v, in_0, insize, settings);
        *out = v.data;
        *outsize = v.size;
        return error;
    }
}

extern "C" fn deflate(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut in_0: *const u8,
    mut insize: u64,
    mut settings: *const LodePNGCompressSettings,
) -> u32 {
    unsafe {
        if ((*settings).custom_deflate).is_some() {
            let mut error = ((*settings).custom_deflate).expect("non-null function pointer")(
                out, outsize, in_0, insize, settings,
            );
            return (if error != 0 { 111 } else { 0 }) as u32;
        } else {
            return lodepng_deflate(out, outsize, in_0, insize, settings);
        };
    }
}

extern "C" fn update_adler32(mut adler: u32, mut data: *const u8, mut len: u32) -> u32 {
    unsafe {
        let mut s1 = adler & 0xffff;
        let mut s2 = adler >> 16 & 0xffff;
        while len != 0 {
            let mut i: u32 = 0;
            let mut amount = if len > 5552 { 5552 } else { len };
            len = len.wrapping_sub(amount);
            i = 0;
            while i != amount {
                let fresh73 = data;
                data = data.offset(1);
                s1 = s1.wrapping_add(*fresh73 as u32);
                s2 = s2.wrapping_add(s1);
                i = i.wrapping_add(1);
            }
            s1 = s1.wrapping_rem(65521);
            s2 = s2.wrapping_rem(65521);
        }
        return s2 << 16 | s1;
    }
}

extern "C" fn adler32(mut data: *const u8, mut len: u32) -> u32 {
    unsafe {
        return update_adler32(1, data, len);
    }
}

extern "C" fn lodepng_zlib_decompressv(
    mut out: *mut ucvector,
    mut in_0: *const u8,
    mut insize: u64,
    mut settings: *const LodePNGDecompressSettings,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut CM: u32 = 0;
        let mut CINFO: u32 = 0;
        let mut FDICT: u32 = 0;
        if insize < 2 {
            return 53;
        }
        if (*in_0.offset(0 as isize) as i32 * 256 + *in_0.offset(1 as isize) as i32) % 31 != 0 {
            return 24;
        }
        CM = (*in_0.offset(0 as isize) as i32 & 15i32) as u32;
        CINFO = (*in_0.offset(0 as isize) as i32 >> 4 & 15i32) as u32;
        FDICT = (*in_0.offset(1 as isize) as i32 >> 5 & 1i32) as u32;
        if CM != 8 || CINFO > 7 {
            return 25;
        }
        if FDICT != 0 {
            return 26;
        }
        error = inflatev(
            out,
            in_0.offset(2 as isize),
            insize.wrapping_sub(2),
            settings,
        );
        if error != 0 {
            return error;
        }
        if (*settings).ignore_adler32 == 0 {
            let mut ADLER32 = lodepng_read32bitInt(&*in_0.offset(insize.wrapping_sub(4) as isize));
            let mut checksum = adler32((*out).data, (*out).size as u32);
            if checksum != ADLER32 {
                return 58;
            }
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_zlib_decompress(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut in_0: *const u8,
    mut insize: u64,
    mut settings: *const LodePNGDecompressSettings,
) -> u32 {
    unsafe {
        let mut v = ucvector_init(*out, *outsize);
        let mut error = lodepng_zlib_decompressv(&mut v, in_0, insize, settings);
        *out = v.data;
        *outsize = v.size;
        return error;
    }
}

extern "C" fn zlib_decompress(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut expected_size: u64,
    mut in_0: *const u8,
    mut insize: u64,
    mut settings: *const LodePNGDecompressSettings,
) -> u32 {
    unsafe {
        let mut error: u32 = 0;
        if ((*settings).custom_zlib).is_some() {
            error = ((*settings).custom_zlib).expect("non-null function pointer")(
                out, outsize, in_0, insize, settings,
            );
            if error != 0 {
                error = 110;
                if (*settings).max_output_size != 0 && *outsize > (*settings).max_output_size {
                    error = 109;
                }
            }
        } else {
            let mut v = ucvector_init(*out, *outsize);
            if expected_size != 0 {
                ucvector_resize(&mut v, (*outsize).wrapping_add(expected_size));
                v.size = *outsize;
            }
            error = lodepng_zlib_decompressv(&mut v, in_0, insize, settings);
            *out = v.data;
            *outsize = v.size;
        }
        return error;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_zlib_compress(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut in_0: *const u8,
    mut insize: u64,
    mut settings: *const LodePNGCompressSettings,
) -> u32 {
    unsafe {
        let mut i: u64 = 0;
        let mut error: u32 = 0;
        let mut deflatedata = 0 as *mut u8;
        let mut deflatesize = 0;
        error = deflate(&mut deflatedata, &mut deflatesize, in_0, insize, settings);
        *out = 0 as *mut u8;
        *outsize = 0;
        if error == 0 {
            *outsize = deflatesize.wrapping_add(6);
            *out = lodepng_malloc(*outsize) as *mut u8;
            if (*out).is_null() {
                error = 83;
            }
        }
        if error == 0 {
            let mut ADLER32 = adler32(in_0, insize as u32);
            let mut CMF = 120;
            let mut FLEVEL = 0;
            let mut FDICT = 0;
            let mut CMFFLG = 256u32
                .wrapping_mul(CMF)
                .wrapping_add(FDICT.wrapping_mul(32))
                .wrapping_add(FLEVEL.wrapping_mul(64));
            let mut FCHECK = 31u32.wrapping_sub(CMFFLG.wrapping_rem(31));
            CMFFLG = CMFFLG.wrapping_add(FCHECK);
            *(*out).offset(0 as isize) = (CMFFLG >> 8i32) as u8;
            *(*out).offset(1 as isize) = (CMFFLG & 255u32) as u8;
            i = 0;
            while i != deflatesize {
                *(*out).offset(i.wrapping_add(2) as isize) = *deflatedata.offset(i as isize);
                i = i.wrapping_add(1);
            }
            lodepng_set32bitInt(
                &mut *(*out).offset((*outsize).wrapping_sub(4) as isize),
                ADLER32,
            );
        }
        lodepng_free(deflatedata as *mut libc::c_void);
        return error;
    }
}

extern "C" fn zlib_compress(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut in_0: *const u8,
    mut insize: u64,
    mut settings: *const LodePNGCompressSettings,
) -> u32 {
    unsafe {
        if ((*settings).custom_zlib).is_some() {
            let mut error = ((*settings).custom_zlib).expect("non-null function pointer")(
                out, outsize, in_0, insize, settings,
            );
            return (if error != 0 { 111 } else { 0 }) as u32;
        } else {
            return lodepng_zlib_compress(out, outsize, in_0, insize, settings);
        };
    }
}

#[no_mangle]
pub extern "C" fn lodepng_compress_settings_init(mut settings: *mut LodePNGCompressSettings) {
    unsafe {
        (*settings).btype = 2;
        (*settings).use_lz77 = 1;
        (*settings).windowsize = 2048;
        (*settings).minmatch = 3;
        (*settings).nicematch = 128;
        (*settings).lazymatching = 1;
        let ref mut fresh74 = (*settings).custom_zlib;
        *fresh74 = None;
        let ref mut fresh75 = (*settings).custom_deflate;
        *fresh75 = None;
        let ref mut fresh76 = (*settings).custom_context;
        *fresh76 = 0 as *const libc::c_void;
    }
}

#[no_mangle]
pub static mut lodepng_default_compress_settings: LodePNGCompressSettings = {
    let mut init = LodePNGCompressSettings {
        btype: 2,
        use_lz77: 1,
        windowsize: 2048,
        minmatch: 3,
        nicematch: 128,
        lazymatching: 1,
        custom_zlib: None,
        custom_deflate: None,
        custom_context: 0 as *const libc::c_void,
    };
    init
};
#[no_mangle]
pub extern "C" fn lodepng_decompress_settings_init(mut settings: *mut LodePNGDecompressSettings) {
    unsafe {
        (*settings).ignore_adler32 = 0;
        (*settings).ignore_nlen = 0;
        (*settings).max_output_size = 0;
        let ref mut fresh77 = (*settings).custom_zlib;
        *fresh77 = None;
        let ref mut fresh78 = (*settings).custom_inflate;
        *fresh78 = None;
        let ref mut fresh79 = (*settings).custom_context;
        *fresh79 = 0 as *const libc::c_void;
    }
}

#[no_mangle]
pub static mut lodepng_default_decompress_settings: LodePNGDecompressSettings = {
    let mut init = LodePNGDecompressSettings {
        ignore_adler32: 0,
        ignore_nlen: 0,
        max_output_size: 0,
        custom_zlib: None,
        custom_inflate: None,
        custom_context: 0 as *const libc::c_void,
    };
    init
};
static mut lodepng_crc32_table: [u32; 256] = [
    0, 1996959894, 3993919788, 2567524794, 124634137, 1886057615, 3915621685, 2657392035,
    249268274, 2044508324, 3772115230, 2547177864, 162941995, 2125561021, 3887607047, 2428444049,
    498536548, 1789927666, 4089016648, 2227061214, 450548861, 1843258603, 4107580753, 2211677639,
    325883990, 1684777152, 4251122042, 2321926636, 335633487, 1661365465, 4195302755, 2366115317,
    997073096, 1281953886, 3579855332, 2724688242, 1006888145, 1258607687, 3524101629, 2768942443,
    901097722, 1119000684, 3686517206, 2898065728, 853044451, 1172266101, 3705015759, 2882616665,
    651767980, 1373503546, 3369554304, 3218104598, 565507253, 1454621731, 3485111705, 3099436303,
    671266974, 1594198024, 3322730930, 2970347812, 795835527, 1483230225, 3244367275, 3060149565,
    1994146192, 31158534, 2563907772, 4023717930, 1907459465, 112637215, 2680153253, 3904427059,
    2013776290, 251722036, 2517215374, 3775830040, 2137656763, 141376813, 2439277719, 3865271297,
    1802195444, 476864866, 2238001368, 4066508878, 1812370925, 453092731, 2181625025, 4111451223,
    1706088902, 314042704, 2344532202, 4240017532, 1658658271, 366619977, 2362670323, 4224994405,
    1303535960, 984961486, 2747007092, 3569037538, 1256170817, 1037604311, 2765210733, 3554079995,
    1131014506, 879679996, 2909243462, 3663771856, 1141124467, 855842277, 2852801631, 3708648649,
    1342533948, 654459306, 3188396048, 3373015174, 1466479909, 544179635, 3110523913, 3462522015,
    1591671054, 702138776, 2966460450, 3352799412, 1504918807, 783551873, 3082640443, 3233442989,
    3988292384, 2596254646, 62317068, 1957810842, 3939845945, 2647816111, 81470997, 1943803523,
    3814918930, 2489596804, 225274430, 2053790376, 3826175755, 2466906013, 167816743, 2097651377,
    4027552580, 2265490386, 503444072, 1762050814, 4150417245, 2154129355, 426522225, 1852507879,
    4275313526, 2312317920, 282753626, 1742555852, 4189708143, 2394877945, 397917763, 1622183637,
    3604390888, 2714866558, 953729732, 1340076626, 3518719985, 2797360999, 1068828381, 1219638859,
    3624741850, 2936675148, 906185462, 1090812512, 3747672003, 2825379669, 829329135, 1181335161,
    3412177804, 3160834842, 628085408, 1382605366, 3423369109, 3138078467, 570562233, 1426400815,
    3317316542, 2998733608, 733239954, 1555261956, 3268935591, 3050360625, 752459403, 1541320221,
    2607071920, 3965973030, 1969922972, 40735498, 2617837225, 3943577151, 1913087877, 83908371,
    2512341634, 3803740692, 2075208622, 213261112, 2463272603, 3855990285, 2094854071, 198958881,
    2262029012, 4057260610, 1759359992, 534414190, 2176718541, 4139329115, 1873836001, 414664567,
    2282248934, 4279200368, 1711684554, 285281116, 2405801727, 4167216745, 1634467795, 376229701,
    2685067896, 3608007406, 1308918612, 956543938, 2808555105, 3495958263, 1231636301, 1047427035,
    2932959818, 3654703836, 1088359270, 936918000, 2847714899, 3736837829, 1202900863, 817233897,
    3183342108, 3401237130, 1404277552, 615818150, 3134207493, 3453421203, 1423857449, 601450431,
    3009837614, 3294710456, 1567103746, 711928724, 3020668471, 3272380065, 1510334235, 755167117,
];
#[no_mangle]
pub extern "C" fn lodepng_crc32(mut data: *const u8, mut length: u64) -> u32 {
    unsafe {
        let mut r = 0xffffffff;
        let mut i: u64 = 0;
        i = 0;
        while i < length {
            r = lodepng_crc32_table[((r ^ *data.offset(i as isize) as u32) & 0xffu32) as usize]
                ^ r >> 8;
            i = i.wrapping_add(1);
        }
        return r ^ 0xffffffff;
    }
}

extern "C" fn readBitFromReversedStream(mut bitpointer: *mut u64, mut bitstream: *const u8) -> u8 {
    unsafe {
        let mut result = (*bitstream.offset((*bitpointer >> 3i32) as isize) as i32
            >> 7u64.wrapping_sub(*bitpointer & 0x7u64)
            & 1) as u8;
        *bitpointer = (*bitpointer).wrapping_add(1);
        return result;
    }
}

extern "C" fn readBitsFromReversedStream(
    mut bitpointer: *mut u64,
    mut bitstream: *const u8,
    mut nbits: u64,
) -> u32 {
    unsafe {
        let mut result = 0;
        let mut i: u64 = 0;
        i = 0;
        while i < nbits {
            result <<= 1;
            result |= readBitFromReversedStream(bitpointer, bitstream) as u32;
            i = i.wrapping_add(1);
        }
        return result;
    }
}

extern "C" fn setBitOfReversedStream(
    mut bitpointer: *mut u64,
    mut bitstream: *mut u8,
    mut bit: u8,
) {
    unsafe {
        if bit as i32 == 0 {
            let ref mut fresh80 = *bitstream.offset((*bitpointer >> 3u32) as isize);
            *fresh80 =
                (*fresh80 as i32 & !(1u32 << 7u64.wrapping_sub(*bitpointer & 7u64)) as i32) as u8;
        } else {
            let ref mut fresh81 = *bitstream.offset((*bitpointer >> 3u32) as isize);
            *fresh81 = (*fresh81 as u32 | 1u32 << 7u64.wrapping_sub(*bitpointer & 7u64)) as u8;
        }
        *bitpointer = (*bitpointer).wrapping_add(1);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_length(mut chunk: *const u8) -> u32 {
    unsafe {
        return lodepng_read32bitInt(chunk);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_type(mut type_0: *mut i8, mut chunk: *const u8) {
    unsafe {
        let mut i: u32 = 0;
        i = 0;
        while i != 4 {
            *type_0.offset(i as isize) = *chunk.offset(4u32.wrapping_add(i) as isize) as i8;
            i = i.wrapping_add(1);
        }
        *type_0.offset(4 as isize) = 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_type_equals(mut chunk: *const u8, mut type_0: *const i8) -> u8 {
    unsafe {
        if lodepng_strlen(type_0) != 4 {
            return 0;
        }
        return (*chunk.offset(4 as isize) as i32 == *type_0.offset(0 as isize) as i32
            && *chunk.offset(5 as isize) as i32 == *type_0.offset(1 as isize) as i32
            && *chunk.offset(6 as isize) as i32 == *type_0.offset(2 as isize) as i32
            && *chunk.offset(7 as isize) as i32 == *type_0.offset(3 as isize) as i32)
            as u8;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_ancillary(mut chunk: *const u8) -> u8 {
    unsafe {
        return (*chunk.offset(4 as isize) as i32 & 32 != 0i32) as u8;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_private(mut chunk: *const u8) -> u8 {
    unsafe {
        return (*chunk.offset(6 as isize) as i32 & 32 != 0i32) as u8;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_safetocopy(mut chunk: *const u8) -> u8 {
    unsafe {
        return (*chunk.offset(7 as isize) as i32 & 32 != 0i32) as u8;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_data(mut chunk: *mut u8) -> *mut u8 {
    unsafe {
        return &mut *chunk.offset(8 as isize) as *mut u8;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_data_const(mut chunk: *const u8) -> *const u8 {
    unsafe {
        return &*chunk.offset(8 as isize) as *const u8;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_check_crc(mut chunk: *const u8) -> u32 {
    unsafe {
        let mut length = lodepng_chunk_length(chunk);
        let mut CRC = lodepng_read32bitInt(&*chunk.offset(length.wrapping_add(8) as isize));
        let mut checksum = lodepng_crc32(&*chunk.offset(4 as isize), length.wrapping_add(4) as u64);
        if CRC != checksum {
            return 1;
        } else {
            return 0;
        };
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_generate_crc(mut chunk: *mut u8) {
    unsafe {
        let mut length = lodepng_chunk_length(chunk);
        let mut CRC = lodepng_crc32(
            &mut *chunk.offset(4 as isize),
            length.wrapping_add(4) as u64,
        );
        lodepng_set32bitInt(chunk.offset(8 as isize).offset(length as isize), CRC);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_next(mut chunk: *mut u8, mut end: *mut u8) -> *mut u8 {
    unsafe {
        let mut available_size = end.offset_from(chunk) as u64;
        if chunk >= end || available_size < 12 {
            return end;
        }
        if *chunk.offset(0 as isize) as i32 == 0x89
            && *chunk.offset(1 as isize) as i32 == 0x50
            && *chunk.offset(2 as isize) as i32 == 0x4e
            && *chunk.offset(3 as isize) as i32 == 0x47
            && *chunk.offset(4 as isize) as i32 == 0xd
            && *chunk.offset(5 as isize) as i32 == 0xa
            && *chunk.offset(6 as isize) as i32 == 0x1a
            && *chunk.offset(7 as isize) as i32 == 0xa
        {
            return chunk.offset(8 as isize);
        } else {
            let mut total_chunk_length: u64 = 0;
            if lodepng_addofl(
                lodepng_chunk_length(chunk) as u64,
                12,
                &mut total_chunk_length,
            ) != 0
            {
                return end;
            }
            if total_chunk_length > available_size {
                return end;
            }
            return chunk.offset(total_chunk_length as isize);
        };
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_next_const(mut chunk: *const u8, mut end: *const u8) -> *const u8 {
    unsafe {
        let mut available_size = end.offset_from(chunk) as u64;
        if chunk >= end || available_size < 12 {
            return end;
        }
        if *chunk.offset(0 as isize) as i32 == 0x89
            && *chunk.offset(1 as isize) as i32 == 0x50
            && *chunk.offset(2 as isize) as i32 == 0x4e
            && *chunk.offset(3 as isize) as i32 == 0x47
            && *chunk.offset(4 as isize) as i32 == 0xd
            && *chunk.offset(5 as isize) as i32 == 0xa
            && *chunk.offset(6 as isize) as i32 == 0x1a
            && *chunk.offset(7 as isize) as i32 == 0xa
        {
            return chunk.offset(8 as isize);
        } else {
            let mut total_chunk_length: u64 = 0;
            if lodepng_addofl(
                lodepng_chunk_length(chunk) as u64,
                12,
                &mut total_chunk_length,
            ) != 0
            {
                return end;
            }
            if total_chunk_length > available_size {
                return end;
            }
            return chunk.offset(total_chunk_length as isize);
        };
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_find(
    mut chunk: *mut u8,
    mut end: *mut u8,
    mut type_0: *const i8,
) -> *mut u8 {
    unsafe {
        loop {
            if chunk >= end || (end.offset_from(chunk) as i64) < 12 {
                return 0 as *mut u8;
            }
            if lodepng_chunk_type_equals(chunk, type_0) != 0 {
                return chunk;
            }
            chunk = lodepng_chunk_next(chunk, end);
        }
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_find_const(
    mut chunk: *const u8,
    mut end: *const u8,
    mut type_0: *const i8,
) -> *const u8 {
    unsafe {
        loop {
            if chunk >= end || (end.offset_from(chunk) as i64) < 12 {
                return 0 as *const u8;
            }
            if lodepng_chunk_type_equals(chunk, type_0) != 0 {
                return chunk;
            }
            chunk = lodepng_chunk_next_const(chunk, end);
        }
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_append(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut chunk: *const u8,
) -> u32 {
    unsafe {
        let mut i: u32 = 0;
        let mut total_chunk_length: u64 = 0;
        let mut new_length: u64 = 0;
        let mut chunk_start = 0 as *mut u8;
        let mut new_buffer = 0 as *mut u8;
        if lodepng_addofl(
            lodepng_chunk_length(chunk) as u64,
            12,
            &mut total_chunk_length,
        ) != 0
        {
            return 77;
        }
        if lodepng_addofl(*outsize, total_chunk_length, &mut new_length) != 0 {
            return 77;
        }
        new_buffer = lodepng_realloc(*out as *mut libc::c_void, new_length) as *mut u8;
        if new_buffer.is_null() {
            return 83;
        }
        *out = new_buffer;
        *outsize = new_length;
        chunk_start =
            &mut *(*out).offset(new_length.wrapping_sub(total_chunk_length) as isize) as *mut u8;
        i = 0;
        while i as u64 != total_chunk_length {
            *chunk_start.offset(i as isize) = *chunk.offset(i as isize);
            i = i.wrapping_add(1);
        }
        return 0;
    }
}

extern "C" fn lodepng_chunk_init(
    mut chunk: *mut *mut u8,
    mut out: *mut ucvector,
    mut length: u32,
    mut type_0: *const i8,
) -> u32 {
    unsafe {
        let mut new_length = (*out).size;
        if lodepng_addofl(new_length, length as u64, &mut new_length) != 0 {
            return 77;
        }
        if lodepng_addofl(new_length, 12, &mut new_length) != 0 {
            return 77;
        }
        if ucvector_resize(out, new_length) == 0 {
            return 83;
        }
        *chunk = ((*out).data)
            .offset(new_length as isize)
            .offset(-(length as isize))
            .offset(-(12 as isize));
        lodepng_set32bitInt(*chunk, length);
        lodepng_memcpy(
            (*chunk).offset(4 as isize) as *mut libc::c_void,
            type_0 as *const libc::c_void,
            4,
        );
        return 0;
    }
}

extern "C" fn lodepng_chunk_createv(
    mut out: *mut ucvector,
    mut length: u32,
    mut type_0: *const i8,
    mut data: *const u8,
) -> u32 {
    unsafe {
        let mut chunk = 0 as *mut u8;
        let mut error = lodepng_chunk_init(&mut chunk, out, length, type_0);
        if error != 0 {
            return error;
        }
        lodepng_memcpy(
            chunk.offset(8 as isize) as *mut libc::c_void,
            data as *const libc::c_void,
            length as u64,
        );
        lodepng_chunk_generate_crc(chunk);
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_chunk_create(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut length: u32,
    mut type_0: *const i8,
    mut data: *const u8,
) -> u32 {
    unsafe {
        let mut v = ucvector_init(*out, *outsize);
        let mut error = lodepng_chunk_createv(&mut v, length, type_0, data);
        *out = v.data;
        *outsize = v.size;
        return error;
    }
}

extern "C" fn checkColorValidity(mut colortype: u32, mut bd: u32) -> u32 {
    match colortype as u32 {
        0 => {
            if !(bd == 1 || bd == 2 || bd == 4 || bd == 8 || bd == 16) {
                return 37;
            }
        }
        2 => {
            if !(bd == 8 || bd == 16) {
                return 37;
            }
        }
        3 => {
            if !(bd == 1 || bd == 2 || bd == 4 || bd == 8) {
                return 37;
            }
        }
        4 => {
            if !(bd == 8 || bd == 16) {
                return 37;
            }
        }
        6 => {
            if !(bd == 8 || bd == 16) {
                return 37;
            }
        }
        255 => return 31,
        _ => return 31,
    }
    return 0;
}

extern "C" fn getNumColorChannels(mut colortype: u32) -> u32 {
    match colortype as u32 {
        0 => return 1,
        2 => return 3,
        3 => return 1,
        4 => return 2,
        6 => return 4,
        255 => return 0,
        _ => return 0,
    };
}

extern "C" fn lodepng_get_bpp_lct(mut colortype: u32, mut bitdepth: u32) -> u32 {
    return (getNumColorChannels(colortype)).wrapping_mul(bitdepth);
}

#[no_mangle]
pub extern "C" fn lodepng_color_mode_init(mut info: *mut LodePNGColorMode) {
    unsafe {
        (*info).key_defined = 0;
        let ref mut fresh82 = (*info).key_b;
        *fresh82 = 0;
        let ref mut fresh83 = (*info).key_g;
        *fresh83 = *fresh82;
        (*info).key_r = *fresh83;
        (*info).colortype = LCT_RGBA;
        (*info).bitdepth = 8;
        let ref mut fresh84 = (*info).palette;
        *fresh84 = 0 as *mut u8;
        (*info).palettesize = 0;
    }
}

extern "C" fn lodepng_color_mode_alloc_palette(mut info: *mut LodePNGColorMode) {
    unsafe {
        let mut i: u64 = 0;
        if ((*info).palette).is_null() {
            let ref mut fresh85 = (*info).palette;
            *fresh85 = lodepng_malloc(1024) as *mut u8;
        }
        if ((*info).palette).is_null() {
            return;
        }
        i = 0;
        while i != 256 {
            *((*info).palette).offset(i.wrapping_mul(4).wrapping_add(0) as isize) = 0;
            *((*info).palette).offset(i.wrapping_mul(4).wrapping_add(1) as isize) = 0;
            *((*info).palette).offset(i.wrapping_mul(4).wrapping_add(2) as isize) = 0;
            *((*info).palette).offset(i.wrapping_mul(4).wrapping_add(3) as isize) = 255;
            i = i.wrapping_add(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn lodepng_color_mode_cleanup(mut info: *mut LodePNGColorMode) {
    unsafe {
        lodepng_palette_clear(info);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_color_mode_copy(
    mut dest: *mut LodePNGColorMode,
    mut source: *const LodePNGColorMode,
) -> u32 {
    unsafe {
        lodepng_color_mode_cleanup(dest);
        lodepng_memcpy(
            dest as *mut libc::c_void,
            source as *const libc::c_void,
            ::std::mem::size_of::<LodePNGColorMode>() as u64,
        );
        if !((*source).palette).is_null() {
            let ref mut fresh86 = (*dest).palette;
            *fresh86 = lodepng_malloc(1024) as *mut u8;
            if ((*dest).palette).is_null() && (*source).palettesize != 0 {
                return 83;
            }
            lodepng_memcpy(
                (*dest).palette as *mut libc::c_void,
                (*source).palette as *const libc::c_void,
                ((*source).palettesize).wrapping_mul(4),
            );
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_color_mode_make(
    mut colortype: u32,
    mut bitdepth: u32,
) -> LodePNGColorMode {
    let mut result = LodePNGColorMode {
        colortype: LCT_GREY,
        bitdepth: 0,
        palette: 0 as *mut u8,
        palettesize: 0,
        key_defined: 0,
        key_r: 0,
        key_g: 0,
        key_b: 0,
    };
    lodepng_color_mode_init(&mut result);
    result.colortype = colortype;
    result.bitdepth = bitdepth;
    return result;
}

extern "C" fn lodepng_color_mode_equal(
    mut a: *const LodePNGColorMode,
    mut b: *const LodePNGColorMode,
) -> i32 {
    unsafe {
        let mut i: u64 = 0;
        if (*a).colortype as u32 != (*b).colortype as u32 {
            return 0;
        }
        if (*a).bitdepth != (*b).bitdepth {
            return 0;
        }
        if (*a).key_defined != (*b).key_defined {
            return 0;
        }
        if (*a).key_defined != 0 {
            if (*a).key_r != (*b).key_r {
                return 0;
            }
            if (*a).key_g != (*b).key_g {
                return 0;
            }
            if (*a).key_b != (*b).key_b {
                return 0;
            }
        }
        if (*a).palettesize != (*b).palettesize {
            return 0;
        }
        i = 0;
        while i != ((*a).palettesize).wrapping_mul(4) {
            if *((*a).palette).offset(i as isize) as i32
                != *((*b).palette).offset(i as isize) as i32
            {
                return 0;
            }
            i = i.wrapping_add(1);
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_palette_clear(mut info: *mut LodePNGColorMode) {
    unsafe {
        if !((*info).palette).is_null() {
            lodepng_free((*info).palette as *mut libc::c_void);
        }
        let ref mut fresh87 = (*info).palette;
        *fresh87 = 0 as *mut u8;
        (*info).palettesize = 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_palette_add(
    mut info: *mut LodePNGColorMode,
    mut r: u8,
    mut g: u8,
    mut b: u8,
    mut a: u8,
) -> u32 {
    unsafe {
        if ((*info).palette).is_null() {
            lodepng_color_mode_alloc_palette(info);
            if ((*info).palette).is_null() {
                return 83;
            }
        }
        if (*info).palettesize >= 256 {
            return 108;
        };
        *((*info).palette)
            .offset(4u64.wrapping_mul((*info).palettesize).wrapping_add(0) as isize) = r;
        *((*info).palette)
            .offset(4u64.wrapping_mul((*info).palettesize).wrapping_add(1) as isize) = g;
        *((*info).palette)
            .offset(4u64.wrapping_mul((*info).palettesize).wrapping_add(2) as isize) = b;
        *((*info).palette)
            .offset(4u64.wrapping_mul((*info).palettesize).wrapping_add(3) as isize) = a;
        let ref mut fresh88 = (*info).palettesize;
        *fresh88 = (*fresh88).wrapping_add(1);
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_get_bpp(mut info: *const LodePNGColorMode) -> u32 {
    unsafe {
        return lodepng_get_bpp_lct((*info).colortype, (*info).bitdepth);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_get_channels(mut info: *const LodePNGColorMode) -> u32 {
    unsafe {
        return getNumColorChannels((*info).colortype);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_is_greyscale_type(mut info: *const LodePNGColorMode) -> u32 {
    unsafe {
        return ((*info).colortype as u32 == LCT_GREY as u32
            || (*info).colortype as u32 == LCT_GREY_ALPHA as u32) as u32;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_is_alpha_type(mut info: *const LodePNGColorMode) -> u32 {
    unsafe {
        return ((*info).colortype as u32 & 4 != 0u32) as u32;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_is_palette_type(mut info: *const LodePNGColorMode) -> u32 {
    unsafe {
        return ((*info).colortype as u32 == LCT_PALETTE as u32) as u32;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_has_palette_alpha(mut info: *const LodePNGColorMode) -> u32 {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i != (*info).palettesize {
            if (*((*info).palette).offset(i.wrapping_mul(4).wrapping_add(3) as isize) as i32) < 255
            {
                return 1;
            }
            i = i.wrapping_add(1);
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_can_have_alpha(mut info: *const LodePNGColorMode) -> u32 {
    unsafe {
        return ((*info).key_defined != 0
            || lodepng_is_alpha_type(info) != 0
            || lodepng_has_palette_alpha(info) != 0) as u32;
    }
}

extern "C" fn lodepng_get_raw_size_lct(
    mut w: u32,
    mut h: u32,
    mut colortype: u32,
    mut bitdepth: u32,
) -> u64 {
    let mut bpp = lodepng_get_bpp_lct(colortype, bitdepth) as u64;
    let mut n = (w as u64).wrapping_mul(h as u64);
    return n
        .wrapping_div(8)
        .wrapping_mul(bpp)
        .wrapping_add((n & 7u64).wrapping_mul(bpp).wrapping_add(7).wrapping_div(8));
}

#[no_mangle]
pub extern "C" fn lodepng_get_raw_size(
    mut w: u32,
    mut h: u32,
    mut color: *const LodePNGColorMode,
) -> u64 {
    unsafe {
        return lodepng_get_raw_size_lct(w, h, (*color).colortype, (*color).bitdepth);
    }
}

extern "C" fn lodepng_get_raw_size_idat(mut w: u32, mut h: u32, mut bpp: u32) -> u64 {
    let mut line = (w.wrapping_div(8u32) as u64)
        .wrapping_mul(bpp as u64)
        .wrapping_add(1)
        .wrapping_add((w & 7u32).wrapping_mul(bpp).wrapping_add(7).wrapping_div(8) as u64);
    return (h as u64).wrapping_mul(line);
}

extern "C" fn lodepng_pixel_overflow(
    mut w: u32,
    mut h: u32,
    mut pngcolor: *const LodePNGColorMode,
    mut rawcolor: *const LodePNGColorMode,
) -> i32 {
    unsafe {
        let mut bpp = (if lodepng_get_bpp(pngcolor) > lodepng_get_bpp(rawcolor) {
            lodepng_get_bpp(pngcolor)
        } else {
            lodepng_get_bpp(rawcolor)
        }) as u64;
        let mut numpixels: u64 = 0;
        let mut total: u64 = 0;
        let mut line: u64 = 0;
        if lodepng_mulofl(w as u64, h as u64, &mut numpixels) != 0 {
            return 1;
        }
        if lodepng_mulofl(numpixels, 8, &mut total) != 0 {
            return 1;
        }
        if lodepng_mulofl(w.wrapping_div(8) as u64, bpp, &mut line) != 0 {
            return 1;
        }
        if lodepng_addofl(
            line,
            ((w & 7u32) as u64)
                .wrapping_mul(bpp)
                .wrapping_add(7)
                .wrapping_div(8),
            &mut line,
        ) != 0
        {
            return 1;
        }
        if lodepng_addofl(line, 5, &mut line) != 0 {
            return 1;
        }
        if lodepng_mulofl(line, h as u64, &mut total) != 0 {
            return 1;
        }
        return 0;
    }
}

extern "C" fn LodePNGUnknownChunks_init(mut info: *mut LodePNGInfo) {
    unsafe {
        let mut i: u32 = 0;
        i = 0;
        while i != 3 {
            let ref mut fresh89 = (*info).unknown_chunks_data[i as usize];
            *fresh89 = 0 as *mut u8;
            i = i.wrapping_add(1);
        }
        i = 0;
        while i != 3 {
            (*info).unknown_chunks_size[i as usize] = 0;
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn LodePNGUnknownChunks_cleanup(mut info: *mut LodePNGInfo) {
    unsafe {
        let mut i: u32 = 0;
        i = 0;
        while i != 3 {
            lodepng_free((*info).unknown_chunks_data[i as usize] as *mut libc::c_void);
            i = i.wrapping_add(1);
        }
    }
}

extern "C" fn LodePNGUnknownChunks_copy(
    mut dest: *mut LodePNGInfo,
    mut src: *const LodePNGInfo,
) -> u32 {
    unsafe {
        let mut i: u32 = 0;
        LodePNGUnknownChunks_cleanup(dest);
        i = 0;
        while i != 3 {
            let mut j: u64 = 0;
            (*dest).unknown_chunks_size[i as usize] = (*src).unknown_chunks_size[i as usize];
            let ref mut fresh90 = (*dest).unknown_chunks_data[i as usize];
            *fresh90 = lodepng_malloc((*src).unknown_chunks_size[i as usize]) as *mut u8;
            if ((*dest).unknown_chunks_data[i as usize]).is_null()
                && (*dest).unknown_chunks_size[i as usize] != 0
            {
                return 83;
            }
            j = 0;
            while j < (*src).unknown_chunks_size[i as usize] {
                *((*dest).unknown_chunks_data[i as usize]).offset(j as isize) =
                    *((*src).unknown_chunks_data[i as usize]).offset(j as isize);
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        return 0;
    }
}

extern "C" fn LodePNGText_init(mut info: *mut LodePNGInfo) {
    unsafe {
        (*info).text_num = 0;
        let ref mut fresh91 = (*info).text_keys;
        *fresh91 = 0 as *mut *mut i8;
        let ref mut fresh92 = (*info).text_strings;
        *fresh92 = 0 as *mut *mut i8;
    }
}

extern "C" fn LodePNGText_cleanup(mut info: *mut LodePNGInfo) {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i != (*info).text_num {
            string_cleanup(&mut *((*info).text_keys).offset(i as isize));
            string_cleanup(&mut *((*info).text_strings).offset(i as isize));
            i = i.wrapping_add(1);
        }
        lodepng_free((*info).text_keys as *mut libc::c_void);
        lodepng_free((*info).text_strings as *mut libc::c_void);
    }
}

extern "C" fn LodePNGText_copy(mut dest: *mut LodePNGInfo, mut source: *const LodePNGInfo) -> u32 {
    unsafe {
        let mut i = 0;
        let ref mut fresh93 = (*dest).text_keys;
        *fresh93 = 0 as *mut *mut i8;
        let ref mut fresh94 = (*dest).text_strings;
        *fresh94 = 0 as *mut *mut i8;
        (*dest).text_num = 0;
        i = 0;
        while i != (*source).text_num {
            let mut error = lodepng_add_text(
                dest,
                *((*source).text_keys).offset(i as isize),
                *((*source).text_strings).offset(i as isize),
            );
            if error != 0 {
                return error;
            }
            i = i.wrapping_add(1);
        }
        return 0;
    }
}

extern "C" fn lodepng_add_text_sized(
    mut info: *mut LodePNGInfo,
    mut key: *const i8,
    mut str: *const i8,
    mut size: u64,
) -> u32 {
    unsafe {
        let mut new_keys = lodepng_realloc(
            (*info).text_keys as *mut libc::c_void,
            (::std::mem::size_of::<*mut i8>() as u64)
                .wrapping_mul(((*info).text_num).wrapping_add(1)),
        ) as *mut *mut i8;
        let mut new_strings = lodepng_realloc(
            (*info).text_strings as *mut libc::c_void,
            (::std::mem::size_of::<*mut i8>() as u64)
                .wrapping_mul(((*info).text_num).wrapping_add(1)),
        ) as *mut *mut i8;
        if !new_keys.is_null() {
            let ref mut fresh95 = (*info).text_keys;
            *fresh95 = new_keys;
        }
        if !new_strings.is_null() {
            let ref mut fresh96 = (*info).text_strings;
            *fresh96 = new_strings;
        }
        if new_keys.is_null() || new_strings.is_null() {
            return 83;
        }
        let ref mut fresh97 = (*info).text_num;
        *fresh97 = (*fresh97).wrapping_add(1);
        let ref mut fresh98 =
            *((*info).text_keys).offset(((*info).text_num).wrapping_sub(1) as isize);
        *fresh98 = alloc_string(key);
        let ref mut fresh99 =
            *((*info).text_strings).offset(((*info).text_num).wrapping_sub(1) as isize);
        *fresh99 = alloc_string_sized(str, size);
        if (*((*info).text_keys).offset(((*info).text_num).wrapping_sub(1u64) as isize)).is_null()
            || (*((*info).text_strings).offset(((*info).text_num).wrapping_sub(1) as isize))
                .is_null()
        {
            return 83;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_add_text(
    mut info: *mut LodePNGInfo,
    mut key: *const i8,
    mut str: *const i8,
) -> u32 {
    unsafe {
        return lodepng_add_text_sized(info, key, str, lodepng_strlen(str));
    }
}

#[no_mangle]
pub extern "C" fn lodepng_clear_text(mut info: *mut LodePNGInfo) {
    unsafe {
        LodePNGText_cleanup(info);
    }
}

extern "C" fn LodePNGIText_init(mut info: *mut LodePNGInfo) {
    unsafe {
        (*info).itext_num = 0;
        let ref mut fresh100 = (*info).itext_keys;
        *fresh100 = 0 as *mut *mut i8;
        let ref mut fresh101 = (*info).itext_langtags;
        *fresh101 = 0 as *mut *mut i8;
        let ref mut fresh102 = (*info).itext_transkeys;
        *fresh102 = 0 as *mut *mut i8;
        let ref mut fresh103 = (*info).itext_strings;
        *fresh103 = 0 as *mut *mut i8;
    }
}

extern "C" fn LodePNGIText_cleanup(mut info: *mut LodePNGInfo) {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i != (*info).itext_num {
            string_cleanup(&mut *((*info).itext_keys).offset(i as isize));
            string_cleanup(&mut *((*info).itext_langtags).offset(i as isize));
            string_cleanup(&mut *((*info).itext_transkeys).offset(i as isize));
            string_cleanup(&mut *((*info).itext_strings).offset(i as isize));
            i = i.wrapping_add(1);
        }
        lodepng_free((*info).itext_keys as *mut libc::c_void);
        lodepng_free((*info).itext_langtags as *mut libc::c_void);
        lodepng_free((*info).itext_transkeys as *mut libc::c_void);
        lodepng_free((*info).itext_strings as *mut libc::c_void);
    }
}

extern "C" fn LodePNGIText_copy(mut dest: *mut LodePNGInfo, mut source: *const LodePNGInfo) -> u32 {
    unsafe {
        let mut i = 0;
        let ref mut fresh104 = (*dest).itext_keys;
        *fresh104 = 0 as *mut *mut i8;
        let ref mut fresh105 = (*dest).itext_langtags;
        *fresh105 = 0 as *mut *mut i8;
        let ref mut fresh106 = (*dest).itext_transkeys;
        *fresh106 = 0 as *mut *mut i8;
        let ref mut fresh107 = (*dest).itext_strings;
        *fresh107 = 0 as *mut *mut i8;
        (*dest).itext_num = 0;
        i = 0;
        while i != (*source).itext_num {
            let mut error = lodepng_add_itext(
                dest,
                *((*source).itext_keys).offset(i as isize),
                *((*source).itext_langtags).offset(i as isize),
                *((*source).itext_transkeys).offset(i as isize),
                *((*source).itext_strings).offset(i as isize),
            );
            if error != 0 {
                return error;
            }
            i = i.wrapping_add(1);
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_clear_itext(mut info: *mut LodePNGInfo) {
    unsafe {
        LodePNGIText_cleanup(info);
    }
}

extern "C" fn lodepng_add_itext_sized(
    mut info: *mut LodePNGInfo,
    mut key: *const i8,
    mut langtag: *const i8,
    mut transkey: *const i8,
    mut str: *const i8,
    mut size: u64,
) -> u32 {
    unsafe {
        let mut new_keys = lodepng_realloc(
            (*info).itext_keys as *mut libc::c_void,
            (::std::mem::size_of::<*mut i8>() as u64)
                .wrapping_mul(((*info).itext_num).wrapping_add(1)),
        ) as *mut *mut i8;
        let mut new_langtags = lodepng_realloc(
            (*info).itext_langtags as *mut libc::c_void,
            (::std::mem::size_of::<*mut i8>() as u64)
                .wrapping_mul(((*info).itext_num).wrapping_add(1)),
        ) as *mut *mut i8;
        let mut new_transkeys = lodepng_realloc(
            (*info).itext_transkeys as *mut libc::c_void,
            (::std::mem::size_of::<*mut i8>() as u64)
                .wrapping_mul(((*info).itext_num).wrapping_add(1)),
        ) as *mut *mut i8;
        let mut new_strings = lodepng_realloc(
            (*info).itext_strings as *mut libc::c_void,
            (::std::mem::size_of::<*mut i8>() as u64)
                .wrapping_mul(((*info).itext_num).wrapping_add(1)),
        ) as *mut *mut i8;
        if !new_keys.is_null() {
            let ref mut fresh108 = (*info).itext_keys;
            *fresh108 = new_keys;
        }
        if !new_langtags.is_null() {
            let ref mut fresh109 = (*info).itext_langtags;
            *fresh109 = new_langtags;
        }
        if !new_transkeys.is_null() {
            let ref mut fresh110 = (*info).itext_transkeys;
            *fresh110 = new_transkeys;
        }
        if !new_strings.is_null() {
            let ref mut fresh111 = (*info).itext_strings;
            *fresh111 = new_strings;
        }
        if new_keys.is_null()
            || new_langtags.is_null()
            || new_transkeys.is_null()
            || new_strings.is_null()
        {
            return 83;
        }
        let ref mut fresh112 = (*info).itext_num;
        *fresh112 = (*fresh112).wrapping_add(1);
        let ref mut fresh113 =
            *((*info).itext_keys).offset(((*info).itext_num).wrapping_sub(1) as isize);
        *fresh113 = alloc_string(key);
        let ref mut fresh114 =
            *((*info).itext_langtags).offset(((*info).itext_num).wrapping_sub(1) as isize);
        *fresh114 = alloc_string(langtag);
        let ref mut fresh115 =
            *((*info).itext_transkeys).offset(((*info).itext_num).wrapping_sub(1) as isize);
        *fresh115 = alloc_string(transkey);
        let ref mut fresh116 =
            *((*info).itext_strings).offset(((*info).itext_num).wrapping_sub(1) as isize);
        *fresh116 = alloc_string_sized(str, size);
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_add_itext(
    mut info: *mut LodePNGInfo,
    mut key: *const i8,
    mut langtag: *const i8,
    mut transkey: *const i8,
    mut str: *const i8,
) -> u32 {
    unsafe {
        return lodepng_add_itext_sized(info, key, langtag, transkey, str, lodepng_strlen(str));
    }
}

extern "C" fn lodepng_assign_icc(
    mut info: *mut LodePNGInfo,
    mut name: *const i8,
    mut profile: *const u8,
    mut profile_size: u32,
) -> u32 {
    unsafe {
        if profile_size == 0 {
            return 100;
        }
        let ref mut fresh117 = (*info).iccp_name;
        *fresh117 = alloc_string(name);
        let ref mut fresh118 = (*info).iccp_profile;
        *fresh118 = lodepng_malloc(profile_size as u64) as *mut u8;
        if ((*info).iccp_name).is_null() || ((*info).iccp_profile).is_null() {
            return 83;
        }
        lodepng_memcpy(
            (*info).iccp_profile as *mut libc::c_void,
            profile as *const libc::c_void,
            profile_size as u64,
        );
        (*info).iccp_profile_size = profile_size;
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_set_icc(
    mut info: *mut LodePNGInfo,
    mut name: *const i8,
    mut profile: *const u8,
    mut profile_size: u32,
) -> u32 {
    unsafe {
        if !((*info).iccp_name).is_null() {
            lodepng_clear_icc(info);
        };
        (*info).iccp_defined = 1;
        return lodepng_assign_icc(info, name, profile, profile_size);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_clear_icc(mut info: *mut LodePNGInfo) {
    unsafe {
        string_cleanup(&mut (*info).iccp_name);
        lodepng_free((*info).iccp_profile as *mut libc::c_void);
        let ref mut fresh119 = (*info).iccp_profile;
        *fresh119 = 0 as *mut u8;
        (*info).iccp_profile_size = 0;
        (*info).iccp_defined = 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_info_init(mut info: *mut LodePNGInfo) {
    unsafe {
        lodepng_color_mode_init(&mut (*info).color);
        (*info).interlace_method = 0;
        (*info).compression_method = 0;
        (*info).filter_method = 0;
        (*info).background_defined = 0;
        let ref mut fresh120 = (*info).background_b;
        *fresh120 = 0;
        let ref mut fresh121 = (*info).background_g;
        *fresh121 = *fresh120;
        (*info).background_r = *fresh121;
        LodePNGText_init(info);
        LodePNGIText_init(info);
        (*info).time_defined = 0;
        (*info).phys_defined = 0;
        (*info).gama_defined = 0;
        (*info).chrm_defined = 0;
        (*info).srgb_defined = 0;
        (*info).iccp_defined = 0;
        let ref mut fresh122 = (*info).iccp_name;
        *fresh122 = 0 as *mut i8;
        let ref mut fresh123 = (*info).iccp_profile;
        *fresh123 = 0 as *mut u8;
        (*info).sbit_defined = 0;
        let ref mut fresh124 = (*info).sbit_a;
        *fresh124 = 0;
        let ref mut fresh125 = (*info).sbit_b;
        *fresh125 = *fresh124;
        let ref mut fresh126 = (*info).sbit_g;
        *fresh126 = *fresh125;
        (*info).sbit_r = *fresh126;
        LodePNGUnknownChunks_init(info);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_info_cleanup(mut info: *mut LodePNGInfo) {
    unsafe {
        lodepng_color_mode_cleanup(&mut (*info).color);
        LodePNGText_cleanup(info);
        LodePNGIText_cleanup(info);
        lodepng_clear_icc(info);
        LodePNGUnknownChunks_cleanup(info);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_info_copy(
    mut dest: *mut LodePNGInfo,
    mut source: *const LodePNGInfo,
) -> u32 {
    unsafe {
        lodepng_info_cleanup(dest);
        lodepng_memcpy(
            dest as *mut libc::c_void,
            source as *const libc::c_void,
            ::std::mem::size_of::<LodePNGInfo>() as u64,
        );
        lodepng_color_mode_init(&mut (*dest).color);
        let mut error = lodepng_color_mode_copy(&mut (*dest).color, &(*source).color);
        if error != 0 {
            return error;
        }
        let mut error_0 = LodePNGText_copy(dest, source);
        if error_0 != 0 {
            return error_0;
        }
        let mut error_1 = LodePNGIText_copy(dest, source);
        if error_1 != 0 {
            return error_1;
        }
        if (*source).iccp_defined != 0 {
            let mut error_2 = lodepng_assign_icc(
                dest,
                (*source).iccp_name,
                (*source).iccp_profile,
                (*source).iccp_profile_size,
            );
            if error_2 != 0 {
                return error_2;
            }
        }
        LodePNGUnknownChunks_init(dest);
        let mut error_3 = LodePNGUnknownChunks_copy(dest, source);
        if error_3 != 0 {
            return error_3;
        }
        return 0;
    }
}

extern "C" fn addColorBits(mut out: *mut u8, mut index: u64, mut bits: u32, mut in_0: u32) {
    unsafe {
        let mut m = (if bits == 1 {
            7
        } else if bits == 2 {
            3
        } else {
            1
        }) as u32;
        let mut p = (index & m as u64) as u32;
        in_0 &= (1u32 << bits).wrapping_sub(1);
        in_0 = in_0 << bits.wrapping_mul(m.wrapping_sub(p));
        if p == 0 {
            *out.offset(index.wrapping_mul(bits as u64).wrapping_div(8) as isize) = in_0 as u8;
        } else {
            let ref mut fresh127 =
                *out.offset(index.wrapping_mul(bits as u64).wrapping_div(8) as isize);
            *fresh127 = (*fresh127 as u32 | in_0) as u8;
        };
    }
}

extern "C" fn color_tree_init(mut tree: *mut ColorTree) {
    unsafe {
        lodepng_memset(
            ((*tree).children).as_mut_ptr() as *mut libc::c_void,
            0,
            16u64.wrapping_mul(::std::mem::size_of::<*mut ColorTree>() as u64),
        );
        (*tree).index = -1;
    }
}

extern "C" fn color_tree_cleanup(mut tree: *mut ColorTree) {
    unsafe {
        let mut i: i32 = 0;
        i = 0;
        while i != 16 {
            if !((*tree).children[i as usize]).is_null() {
                color_tree_cleanup((*tree).children[i as usize]);
                lodepng_free((*tree).children[i as usize] as *mut libc::c_void);
            }
            i += 1;
        }
    }
}

extern "C" fn color_tree_get(
    mut tree: *mut ColorTree,
    mut r: u8,
    mut g: u8,
    mut b: u8,
    mut a: u8,
) -> i32 {
    unsafe {
        let mut bit = 0;
        bit = 0;
        while bit < 8 {
            let mut i = 8 * (r as i32 >> bit & 1)
                + 4 * (g as i32 >> bit & 1)
                + 2 * (b as i32 >> bit & 1)
                + 1 * (a as i32 >> bit & 1);
            if ((*tree).children[i as usize]).is_null() {
                return -1;
            } else {
                tree = (*tree).children[i as usize];
            }
            bit += 1;
        }
        return if !tree.is_null() { (*tree).index } else { -1 };
    }
}

extern "C" fn color_tree_has(
    mut tree: *mut ColorTree,
    mut r: u8,
    mut g: u8,
    mut b: u8,
    mut a: u8,
) -> i32 {
    unsafe {
        return (color_tree_get(tree, r, g, b, a) >= 0) as i32;
    }
}

extern "C" fn color_tree_add(
    mut tree: *mut ColorTree,
    mut r: u8,
    mut g: u8,
    mut b: u8,
    mut a: u8,
    mut index: u32,
) -> u32 {
    unsafe {
        let mut bit: i32 = 0;
        bit = 0;
        while bit < 8 {
            let mut i = 8 * (r as i32 >> bit & 1)
                + 4 * (g as i32 >> bit & 1)
                + 2 * (b as i32 >> bit & 1)
                + 1 * (a as i32 >> bit & 1);
            if ((*tree).children[i as usize]).is_null() {
                let ref mut fresh128 = (*tree).children[i as usize];
                *fresh128 =
                    lodepng_malloc(::std::mem::size_of::<ColorTree>() as u64) as *mut ColorTree;
                if ((*tree).children[i as usize]).is_null() {
                    return 83;
                }
                color_tree_init((*tree).children[i as usize]);
            }
            tree = (*tree).children[i as usize];
            bit += 1;
        }
        (*tree).index = index as i32;
        return 0;
    }
}

extern "C" fn rgba8ToPixel(
    mut out: *mut u8,
    mut i: u64,
    mut mode: *const LodePNGColorMode,
    mut tree: *mut ColorTree,
    mut r: u8,
    mut g: u8,
    mut b: u8,
    mut a: u8,
) -> u32 {
    unsafe {
        if (*mode).colortype as u32 == LCT_GREY as u32 {
            let mut gray = r;
            if (*mode).bitdepth == 8 {
                *out.offset(i as isize) = gray;
            } else if (*mode).bitdepth == 16 {
                let ref mut fresh129 = *out.offset(i.wrapping_mul(2).wrapping_add(1) as isize);
                *fresh129 = gray;
                *out.offset(i.wrapping_mul(2).wrapping_add(0) as isize) = *fresh129;
            } else {
                gray = (gray as u32 >> 8u32.wrapping_sub((*mode).bitdepth)
                    & (1u32 << (*mode).bitdepth).wrapping_sub(1u32)) as u8;
                addColorBits(out, i, (*mode).bitdepth, gray as u32);
            }
        } else if (*mode).colortype as u32 == LCT_RGB as u32 {
            if (*mode).bitdepth == 8 {
                *out.offset(i.wrapping_mul(3).wrapping_add(0) as isize) = r;
                *out.offset(i.wrapping_mul(3).wrapping_add(1) as isize) = g;
                *out.offset(i.wrapping_mul(3).wrapping_add(2) as isize) = b;
            } else {
                let ref mut fresh130 = *out.offset(i.wrapping_mul(6).wrapping_add(1) as isize);
                *fresh130 = r;
                *out.offset(i.wrapping_mul(6).wrapping_add(0) as isize) = *fresh130;
                let ref mut fresh131 = *out.offset(i.wrapping_mul(6).wrapping_add(3) as isize);
                *fresh131 = g;
                *out.offset(i.wrapping_mul(6).wrapping_add(2) as isize) = *fresh131;
                let ref mut fresh132 = *out.offset(i.wrapping_mul(6).wrapping_add(5) as isize);
                *fresh132 = b;
                *out.offset(i.wrapping_mul(6).wrapping_add(4) as isize) = *fresh132;
            }
        } else if (*mode).colortype as u32 == LCT_PALETTE as u32 {
            let mut index = color_tree_get(tree, r, g, b, a);
            if index < 0 {
                return 82;
            }
            if (*mode).bitdepth == 8 {
                *out.offset(i as isize) = index as u8;
            } else {
                addColorBits(out, i, (*mode).bitdepth, index as u32);
            }
        } else if (*mode).colortype as u32 == LCT_GREY_ALPHA as u32 {
            let mut gray_0 = r;
            if (*mode).bitdepth == 8 {
                *out.offset(i.wrapping_mul(2).wrapping_add(0) as isize) = gray_0;
                *out.offset(i.wrapping_mul(2).wrapping_add(1) as isize) = a;
            } else if (*mode).bitdepth == 16 {
                let ref mut fresh133 = *out.offset(i.wrapping_mul(4).wrapping_add(1) as isize);
                *fresh133 = gray_0;
                *out.offset(i.wrapping_mul(4).wrapping_add(0) as isize) = *fresh133;
                let ref mut fresh134 = *out.offset(i.wrapping_mul(4).wrapping_add(3) as isize);
                *fresh134 = a;
                *out.offset(i.wrapping_mul(4).wrapping_add(2) as isize) = *fresh134;
            }
        } else if (*mode).colortype as u32 == LCT_RGBA as u32 {
            if (*mode).bitdepth == 8 {
                *out.offset(i.wrapping_mul(4).wrapping_add(0) as isize) = r;
                *out.offset(i.wrapping_mul(4).wrapping_add(1) as isize) = g;
                *out.offset(i.wrapping_mul(4).wrapping_add(2) as isize) = b;
                *out.offset(i.wrapping_mul(4).wrapping_add(3) as isize) = a;
            } else {
                let ref mut fresh135 = *out.offset(i.wrapping_mul(8).wrapping_add(1) as isize);
                *fresh135 = r;
                *out.offset(i.wrapping_mul(8).wrapping_add(0) as isize) = *fresh135;
                let ref mut fresh136 = *out.offset(i.wrapping_mul(8).wrapping_add(3) as isize);
                *fresh136 = g;
                *out.offset(i.wrapping_mul(8).wrapping_add(2) as isize) = *fresh136;
                let ref mut fresh137 = *out.offset(i.wrapping_mul(8).wrapping_add(5) as isize);
                *fresh137 = b;
                *out.offset(i.wrapping_mul(8).wrapping_add(4) as isize) = *fresh137;
                let ref mut fresh138 = *out.offset(i.wrapping_mul(8).wrapping_add(7) as isize);
                *fresh138 = a;
                *out.offset(i.wrapping_mul(8).wrapping_add(6) as isize) = *fresh138;
            }
        }
        return 0;
    }
}

extern "C" fn rgba16ToPixel(
    mut out: *mut u8,
    mut i: u64,
    mut mode: *const LodePNGColorMode,
    mut r: u16,
    mut g: u16,
    mut b: u16,
    mut a: u16,
) {
    unsafe {
        if (*mode).colortype as u32 == LCT_GREY as u32 {
            let mut gray = r;
            *out.offset(i.wrapping_mul(2).wrapping_add(0) as isize) =
                (gray as i32 >> 8 & 255i32) as u8;
            *out.offset(i.wrapping_mul(2).wrapping_add(1) as isize) = (gray as i32 & 255i32) as u8;
        } else if (*mode).colortype as u32 == LCT_RGB as u32 {
            *out.offset(i.wrapping_mul(6).wrapping_add(0) as isize) =
                (r as i32 >> 8 & 255i32) as u8;
            *out.offset(i.wrapping_mul(6).wrapping_add(1) as isize) = (r as i32 & 255i32) as u8;
            *out.offset(i.wrapping_mul(6).wrapping_add(2) as isize) =
                (g as i32 >> 8 & 255i32) as u8;
            *out.offset(i.wrapping_mul(6).wrapping_add(3) as isize) = (g as i32 & 255i32) as u8;
            *out.offset(i.wrapping_mul(6).wrapping_add(4) as isize) =
                (b as i32 >> 8 & 255i32) as u8;
            *out.offset(i.wrapping_mul(6).wrapping_add(5) as isize) = (b as i32 & 255i32) as u8;
        } else if (*mode).colortype as u32 == LCT_GREY_ALPHA as u32 {
            let mut gray_0 = r;
            *out.offset(i.wrapping_mul(4).wrapping_add(0) as isize) =
                (gray_0 as i32 >> 8 & 255i32) as u8;
            *out.offset(i.wrapping_mul(4).wrapping_add(1) as isize) =
                (gray_0 as i32 & 255i32) as u8;
            *out.offset(i.wrapping_mul(4).wrapping_add(2) as isize) =
                (a as i32 >> 8 & 255i32) as u8;
            *out.offset(i.wrapping_mul(4).wrapping_add(3) as isize) = (a as i32 & 255i32) as u8;
        } else if (*mode).colortype as u32 == LCT_RGBA as u32 {
            *out.offset(i.wrapping_mul(8).wrapping_add(0) as isize) =
                (r as i32 >> 8 & 255i32) as u8;
            *out.offset(i.wrapping_mul(8).wrapping_add(1) as isize) = (r as i32 & 255i32) as u8;
            *out.offset(i.wrapping_mul(8).wrapping_add(2) as isize) =
                (g as i32 >> 8 & 255i32) as u8;
            *out.offset(i.wrapping_mul(8).wrapping_add(3) as isize) = (g as i32 & 255i32) as u8;
            *out.offset(i.wrapping_mul(8).wrapping_add(4) as isize) =
                (b as i32 >> 8 & 255i32) as u8;
            *out.offset(i.wrapping_mul(8).wrapping_add(5) as isize) = (b as i32 & 255i32) as u8;
            *out.offset(i.wrapping_mul(8).wrapping_add(6) as isize) =
                (a as i32 >> 8 & 255i32) as u8;
            *out.offset(i.wrapping_mul(8).wrapping_add(7) as isize) = (a as i32 & 255i32) as u8;
        }
    }
}

extern "C" fn getPixelColorRGBA8(
    mut r: *mut u8,
    mut g: *mut u8,
    mut b: *mut u8,
    mut a: *mut u8,
    mut in_0: *const u8,
    mut i: u64,
    mut mode: *const LodePNGColorMode,
) {
    unsafe {
        if (*mode).colortype as u32 == LCT_GREY as u32 {
            if (*mode).bitdepth == 8 {
                *b = *in_0.offset(i as isize);
                *g = *b;
                *r = *g;
                if (*mode).key_defined != 0 && *r as u32 == (*mode).key_r {
                    *a = 0;
                } else {
                    *a = 255;
                }
            } else if (*mode).bitdepth == 16 {
                *b = *in_0.offset(i.wrapping_mul(2).wrapping_add(0) as isize);
                *g = *b;
                *r = *g;
                if (*mode).key_defined != 0
                    && 256u32
                        .wrapping_mul(
                            *in_0.offset(i.wrapping_mul(2).wrapping_add(0) as isize) as u32
                        )
                        .wrapping_add(
                            *in_0.offset(i.wrapping_mul(2).wrapping_add(1) as isize) as u32
                        )
                        == (*mode).key_r
                {
                    *a = 0;
                } else {
                    *a = 255;
                }
            } else {
                let mut highest = (1u32 << (*mode).bitdepth).wrapping_sub(1);
                let mut j = i.wrapping_mul((*mode).bitdepth as u64);
                let mut value = readBitsFromReversedStream(&mut j, in_0, (*mode).bitdepth as u64);
                *b = value.wrapping_mul(255).wrapping_div(highest) as u8;
                *g = *b;
                *r = *g;
                if (*mode).key_defined != 0 && value == (*mode).key_r {
                    *a = 0;
                } else {
                    *a = 255;
                }
            }
        } else if (*mode).colortype as u32 == LCT_RGB as u32 {
            if (*mode).bitdepth == 8 {
                *r = *in_0.offset(i.wrapping_mul(3).wrapping_add(0) as isize);
                *g = *in_0.offset(i.wrapping_mul(3).wrapping_add(1) as isize);
                *b = *in_0.offset(i.wrapping_mul(3).wrapping_add(2) as isize);
                if (*mode).key_defined != 0
                    && *r as u32 == (*mode).key_r
                    && *g as u32 == (*mode).key_g
                    && *b as u32 == (*mode).key_b
                {
                    *a = 0;
                } else {
                    *a = 255;
                }
            } else {
                *r = *in_0.offset(i.wrapping_mul(6).wrapping_add(0) as isize);
                *g = *in_0.offset(i.wrapping_mul(6).wrapping_add(2) as isize);
                *b = *in_0.offset(i.wrapping_mul(6).wrapping_add(4) as isize);
                if (*mode).key_defined != 0
                    && 256u32
                        .wrapping_mul(
                            *in_0.offset(i.wrapping_mul(6).wrapping_add(0) as isize) as u32
                        )
                        .wrapping_add(
                            *in_0.offset(i.wrapping_mul(6).wrapping_add(1) as isize) as u32
                        )
                        == (*mode).key_r
                    && 256u32
                        .wrapping_mul(
                            *in_0.offset(i.wrapping_mul(6).wrapping_add(2) as isize) as u32
                        )
                        .wrapping_add(
                            *in_0.offset(i.wrapping_mul(6).wrapping_add(3) as isize) as u32
                        )
                        == (*mode).key_g
                    && 256u32
                        .wrapping_mul(
                            *in_0.offset(i.wrapping_mul(6).wrapping_add(4) as isize) as u32
                        )
                        .wrapping_add(
                            *in_0.offset(i.wrapping_mul(6).wrapping_add(5) as isize) as u32
                        )
                        == (*mode).key_b
                {
                    *a = 0;
                } else {
                    *a = 255;
                }
            }
        } else if (*mode).colortype as u32 == LCT_PALETTE as u32 {
            let mut index: u32 = 0;
            if (*mode).bitdepth == 8 {
                index = *in_0.offset(i as isize) as u32;
            } else {
                let mut j_0 = i.wrapping_mul((*mode).bitdepth as u64);
                index = readBitsFromReversedStream(&mut j_0, in_0, (*mode).bitdepth as u64);
            }
            *r = *((*mode).palette).offset(index.wrapping_mul(4).wrapping_add(0) as isize);
            *g = *((*mode).palette).offset(index.wrapping_mul(4).wrapping_add(1) as isize);
            *b = *((*mode).palette).offset(index.wrapping_mul(4).wrapping_add(2) as isize);
            *a = *((*mode).palette).offset(index.wrapping_mul(4).wrapping_add(3) as isize);
        } else if (*mode).colortype as u32 == LCT_GREY_ALPHA as u32 {
            if (*mode).bitdepth == 8 {
                *b = *in_0.offset(i.wrapping_mul(2).wrapping_add(0) as isize);
                *g = *b;
                *r = *g;
                *a = *in_0.offset(i.wrapping_mul(2).wrapping_add(1) as isize);
            } else {
                *b = *in_0.offset(i.wrapping_mul(4).wrapping_add(0) as isize);
                *g = *b;
                *r = *g;
                *a = *in_0.offset(i.wrapping_mul(4).wrapping_add(2) as isize);
            }
        } else if (*mode).colortype as u32 == LCT_RGBA as u32 {
            if (*mode).bitdepth == 8 {
                *r = *in_0.offset(i.wrapping_mul(4).wrapping_add(0) as isize);
                *g = *in_0.offset(i.wrapping_mul(4).wrapping_add(1) as isize);
                *b = *in_0.offset(i.wrapping_mul(4).wrapping_add(2) as isize);
                *a = *in_0.offset(i.wrapping_mul(4).wrapping_add(3) as isize);
            } else {
                *r = *in_0.offset(i.wrapping_mul(8).wrapping_add(0) as isize);
                *g = *in_0.offset(i.wrapping_mul(8).wrapping_add(2) as isize);
                *b = *in_0.offset(i.wrapping_mul(8).wrapping_add(4) as isize);
                *a = *in_0.offset(i.wrapping_mul(8).wrapping_add(6) as isize);
            }
        }
    }
}

extern "C" fn getPixelColorsRGBA8(
    mut buffer: *mut u8,
    mut numpixels: u64,
    mut in_0: *const u8,
    mut mode: *const LodePNGColorMode,
) {
    unsafe {
        let mut num_channels = 4;
        let mut i: u64 = 0;
        if (*mode).colortype as u32 == LCT_GREY as u32 {
            if (*mode).bitdepth == 8 {
                i = 0;
                while i != numpixels {
                    let ref mut fresh139 = *buffer.offset(2 as isize);
                    *fresh139 = *in_0.offset(i as isize);
                    let ref mut fresh140 = *buffer.offset(1 as isize);
                    *fresh140 = *fresh139;
                    *buffer.offset(0 as isize) = *fresh140;
                    *buffer.offset(3 as isize) = 255;
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
                if (*mode).key_defined != 0 {
                    buffer = buffer.offset(-(numpixels.wrapping_mul(num_channels as u64) as isize));
                    i = 0;
                    while i != numpixels {
                        if *buffer.offset(0 as isize) as u32 == (*mode).key_r {
                            *buffer.offset(3 as isize) = 0;
                        }
                        i = i.wrapping_add(1);
                        buffer = buffer.offset(num_channels as isize);
                    }
                }
            } else if (*mode).bitdepth == 16 {
                i = 0;
                while i != numpixels {
                    let ref mut fresh141 = *buffer.offset(2 as isize);
                    *fresh141 = *in_0.offset(i.wrapping_mul(2) as isize);
                    let ref mut fresh142 = *buffer.offset(1 as isize);
                    *fresh142 = *fresh141;
                    *buffer.offset(0 as isize) = *fresh142;
                    *buffer.offset(3 as isize) = (if (*mode).key_defined != 0
                        && 256u32
                            .wrapping_mul(
                                *in_0.offset(i.wrapping_mul(2).wrapping_add(0) as isize) as u32
                            )
                            .wrapping_add(
                                *in_0.offset(i.wrapping_mul(2).wrapping_add(1) as isize) as u32
                            )
                            == (*mode).key_r
                    {
                        0
                    } else {
                        255
                    }) as u8;
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            } else {
                let mut highest = (1u32 << (*mode).bitdepth).wrapping_sub(1);
                let mut j = 0;
                i = 0;
                while i != numpixels {
                    let mut value =
                        readBitsFromReversedStream(&mut j, in_0, (*mode).bitdepth as u64);
                    let ref mut fresh143 = *buffer.offset(2 as isize);
                    *fresh143 = value.wrapping_mul(255).wrapping_div(highest) as u8;
                    let ref mut fresh144 = *buffer.offset(1 as isize);
                    *fresh144 = *fresh143;
                    *buffer.offset(0 as isize) = *fresh144;
                    *buffer.offset(3 as isize) =
                        (if (*mode).key_defined != 0 && value == (*mode).key_r {
                            0
                        } else {
                            255
                        }) as u8;
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        } else if (*mode).colortype as u32 == LCT_RGB as u32 {
            if (*mode).bitdepth == 8 {
                i = 0;
                while i != numpixels {
                    lodepng_memcpy(
                        buffer as *mut libc::c_void,
                        &*in_0.offset(i.wrapping_mul(3) as isize) as *const u8
                            as *const libc::c_void,
                        3,
                    );
                    *buffer.offset(3 as isize) = 255;
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
                if (*mode).key_defined != 0 {
                    buffer = buffer.offset(-(numpixels.wrapping_mul(num_channels as u64) as isize));
                    i = 0;
                    while i != numpixels {
                        if *buffer.offset(0 as isize) as u32 == (*mode).key_r
                            && *buffer.offset(1 as isize) as u32 == (*mode).key_g
                            && *buffer.offset(2 as isize) as u32 == (*mode).key_b
                        {
                            *buffer.offset(3 as isize) = 0;
                        }
                        i = i.wrapping_add(1);
                        buffer = buffer.offset(num_channels as isize);
                    }
                }
            } else {
                i = 0;
                while i != numpixels {
                    *buffer.offset(0 as isize) =
                        *in_0.offset(i.wrapping_mul(6).wrapping_add(0) as isize);
                    *buffer.offset(1 as isize) =
                        *in_0.offset(i.wrapping_mul(6).wrapping_add(2) as isize);
                    *buffer.offset(2 as isize) =
                        *in_0.offset(i.wrapping_mul(6).wrapping_add(4) as isize);
                    *buffer.offset(3 as isize) = (if (*mode).key_defined != 0
                        && 256u32
                            .wrapping_mul(
                                *in_0.offset(i.wrapping_mul(6).wrapping_add(0) as isize) as u32
                            )
                            .wrapping_add(
                                *in_0.offset(i.wrapping_mul(6).wrapping_add(1) as isize) as u32
                            )
                            == (*mode).key_r
                        && 256u32
                            .wrapping_mul(
                                *in_0.offset(i.wrapping_mul(6).wrapping_add(2) as isize) as u32
                            )
                            .wrapping_add(
                                *in_0.offset(i.wrapping_mul(6).wrapping_add(3) as isize) as u32
                            )
                            == (*mode).key_g
                        && 256u32
                            .wrapping_mul(
                                *in_0.offset(i.wrapping_mul(6).wrapping_add(4) as isize) as u32
                            )
                            .wrapping_add(
                                *in_0.offset(i.wrapping_mul(6).wrapping_add(5) as isize) as u32
                            )
                            == (*mode).key_b
                    {
                        0
                    } else {
                        255
                    }) as u8;
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        } else if (*mode).colortype as u32 == LCT_PALETTE as u32 {
            if (*mode).bitdepth == 8 {
                i = 0;
                while i != numpixels {
                    let mut index = *in_0.offset(i as isize) as u32;
                    lodepng_memcpy(
                        buffer as *mut libc::c_void,
                        &mut *((*mode).palette).offset(index.wrapping_mul(4) as isize) as *mut u8
                            as *const libc::c_void,
                        4,
                    );
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            } else {
                let mut j_0 = 0;
                i = 0;
                while i != numpixels {
                    let mut index_0 =
                        readBitsFromReversedStream(&mut j_0, in_0, (*mode).bitdepth as u64);
                    lodepng_memcpy(
                        buffer as *mut libc::c_void,
                        &mut *((*mode).palette).offset(index_0.wrapping_mul(4) as isize) as *mut u8
                            as *const libc::c_void,
                        4,
                    );
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        } else if (*mode).colortype as u32 == LCT_GREY_ALPHA as u32 {
            if (*mode).bitdepth == 8 {
                i = 0;
                while i != numpixels {
                    let ref mut fresh145 = *buffer.offset(2 as isize);
                    *fresh145 = *in_0.offset(i.wrapping_mul(2).wrapping_add(0) as isize);
                    let ref mut fresh146 = *buffer.offset(1 as isize);
                    *fresh146 = *fresh145;
                    *buffer.offset(0 as isize) = *fresh146;
                    *buffer.offset(3 as isize) =
                        *in_0.offset(i.wrapping_mul(2).wrapping_add(1) as isize);
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            } else {
                i = 0;
                while i != numpixels {
                    let ref mut fresh147 = *buffer.offset(2 as isize);
                    *fresh147 = *in_0.offset(i.wrapping_mul(4).wrapping_add(0) as isize);
                    let ref mut fresh148 = *buffer.offset(1 as isize);
                    *fresh148 = *fresh147;
                    *buffer.offset(0 as isize) = *fresh148;
                    *buffer.offset(3 as isize) =
                        *in_0.offset(i.wrapping_mul(4).wrapping_add(2) as isize);
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        } else if (*mode).colortype as u32 == LCT_RGBA as u32 {
            if (*mode).bitdepth == 8 {
                lodepng_memcpy(
                    buffer as *mut libc::c_void,
                    in_0 as *const libc::c_void,
                    numpixels.wrapping_mul(4),
                );
            } else {
                i = 0;
                while i != numpixels {
                    *buffer.offset(0 as isize) =
                        *in_0.offset(i.wrapping_mul(8).wrapping_add(0) as isize);
                    *buffer.offset(1 as isize) =
                        *in_0.offset(i.wrapping_mul(8).wrapping_add(2) as isize);
                    *buffer.offset(2 as isize) =
                        *in_0.offset(i.wrapping_mul(8).wrapping_add(4) as isize);
                    *buffer.offset(3 as isize) =
                        *in_0.offset(i.wrapping_mul(8).wrapping_add(6) as isize);
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        }
    }
}

extern "C" fn getPixelColorsRGB8(
    mut buffer: *mut u8,
    mut numpixels: u64,
    mut in_0: *const u8,
    mut mode: *const LodePNGColorMode,
) {
    unsafe {
        let num_channels = 3;
        let mut i: u64 = 0;
        if (*mode).colortype as u32 == LCT_GREY as u32 {
            if (*mode).bitdepth == 8 {
                i = 0;
                while i != numpixels {
                    let ref mut fresh149 = *buffer.offset(2 as isize);
                    *fresh149 = *in_0.offset(i as isize);
                    let ref mut fresh150 = *buffer.offset(1 as isize);
                    *fresh150 = *fresh149;
                    *buffer.offset(0 as isize) = *fresh150;
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            } else if (*mode).bitdepth == 16 {
                i = 0;
                while i != numpixels {
                    let ref mut fresh151 = *buffer.offset(2 as isize);
                    *fresh151 = *in_0.offset(i.wrapping_mul(2) as isize);
                    let ref mut fresh152 = *buffer.offset(1 as isize);
                    *fresh152 = *fresh151;
                    *buffer.offset(0 as isize) = *fresh152;
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            } else {
                let mut highest = (1u32 << (*mode).bitdepth).wrapping_sub(1);
                let mut j = 0;
                i = 0;
                while i != numpixels {
                    let mut value =
                        readBitsFromReversedStream(&mut j, in_0, (*mode).bitdepth as u64);
                    let ref mut fresh153 = *buffer.offset(2 as isize);
                    *fresh153 = value.wrapping_mul(255).wrapping_div(highest) as u8;
                    let ref mut fresh154 = *buffer.offset(1 as isize);
                    *fresh154 = *fresh153;
                    *buffer.offset(0 as isize) = *fresh154;
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        } else if (*mode).colortype as u32 == LCT_RGB as u32 {
            if (*mode).bitdepth == 8 {
                lodepng_memcpy(
                    buffer as *mut libc::c_void,
                    in_0 as *const libc::c_void,
                    numpixels.wrapping_mul(3),
                );
            } else {
                i = 0;
                while i != numpixels {
                    *buffer.offset(0 as isize) =
                        *in_0.offset(i.wrapping_mul(6).wrapping_add(0) as isize);
                    *buffer.offset(1 as isize) =
                        *in_0.offset(i.wrapping_mul(6).wrapping_add(2) as isize);
                    *buffer.offset(2 as isize) =
                        *in_0.offset(i.wrapping_mul(6).wrapping_add(4) as isize);
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        } else if (*mode).colortype as u32 == LCT_PALETTE as u32 {
            if (*mode).bitdepth == 8 {
                i = 0;
                while i != numpixels {
                    let mut index = *in_0.offset(i as isize) as u32;
                    lodepng_memcpy(
                        buffer as *mut libc::c_void,
                        &mut *((*mode).palette).offset(index.wrapping_mul(4) as isize) as *mut u8
                            as *const libc::c_void,
                        3,
                    );
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            } else {
                let mut j_0 = 0;
                i = 0;
                while i != numpixels {
                    let mut index_0 =
                        readBitsFromReversedStream(&mut j_0, in_0, (*mode).bitdepth as u64);
                    lodepng_memcpy(
                        buffer as *mut libc::c_void,
                        &mut *((*mode).palette).offset(index_0.wrapping_mul(4) as isize) as *mut u8
                            as *const libc::c_void,
                        3,
                    );
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        } else if (*mode).colortype as u32 == LCT_GREY_ALPHA as u32 {
            if (*mode).bitdepth == 8 {
                i = 0;
                while i != numpixels {
                    let ref mut fresh155 = *buffer.offset(2 as isize);
                    *fresh155 = *in_0.offset(i.wrapping_mul(2).wrapping_add(0) as isize);
                    let ref mut fresh156 = *buffer.offset(1 as isize);
                    *fresh156 = *fresh155;
                    *buffer.offset(0 as isize) = *fresh156;
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            } else {
                i = 0;
                while i != numpixels {
                    let ref mut fresh157 = *buffer.offset(2 as isize);
                    *fresh157 = *in_0.offset(i.wrapping_mul(4).wrapping_add(0) as isize);
                    let ref mut fresh158 = *buffer.offset(1 as isize);
                    *fresh158 = *fresh157;
                    *buffer.offset(0 as isize) = *fresh158;
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        } else if (*mode).colortype as u32 == LCT_RGBA as u32 {
            if (*mode).bitdepth == 8 {
                i = 0;
                while i != numpixels {
                    lodepng_memcpy(
                        buffer as *mut libc::c_void,
                        &*in_0.offset(i.wrapping_mul(4) as isize) as *const u8
                            as *const libc::c_void,
                        3,
                    );
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            } else {
                i = 0;
                while i != numpixels {
                    *buffer.offset(0 as isize) =
                        *in_0.offset(i.wrapping_mul(8).wrapping_add(0) as isize);
                    *buffer.offset(1 as isize) =
                        *in_0.offset(i.wrapping_mul(8).wrapping_add(2) as isize);
                    *buffer.offset(2 as isize) =
                        *in_0.offset(i.wrapping_mul(8).wrapping_add(4) as isize);
                    i = i.wrapping_add(1);
                    buffer = buffer.offset(num_channels as isize);
                }
            }
        }
    }
}

extern "C" fn getPixelColorRGBA16(
    mut r: *mut u16,
    mut g: *mut u16,
    mut b: *mut u16,
    mut a: *mut u16,
    mut in_0: *const u8,
    mut i: u64,
    mut mode: *const LodePNGColorMode,
) {
    unsafe {
        if (*mode).colortype as u32 == LCT_GREY as u32 {
            *b = (256 * *in_0.offset(i.wrapping_mul(2u64).wrapping_add(0) as isize) as i32
                + *in_0.offset(i.wrapping_mul(2).wrapping_add(1) as isize) as i32)
                as u16;
            *g = *b;
            *r = *g;
            if (*mode).key_defined != 0
                && 256u32
                    .wrapping_mul(*in_0.offset(i.wrapping_mul(2).wrapping_add(0) as isize) as u32)
                    .wrapping_add(*in_0.offset(i.wrapping_mul(2).wrapping_add(1) as isize) as u32)
                    == (*mode).key_r
            {
                *a = 0;
            } else {
                *a = 65535;
            }
        } else if (*mode).colortype as u32 == LCT_RGB as u32 {
            *r = 256u32
                .wrapping_mul(*in_0.offset(i.wrapping_mul(6).wrapping_add(0) as isize) as u32)
                .wrapping_add(*in_0.offset(i.wrapping_mul(6).wrapping_add(1) as isize) as u32)
                as u16;
            *g = 256u32
                .wrapping_mul(*in_0.offset(i.wrapping_mul(6).wrapping_add(2) as isize) as u32)
                .wrapping_add(*in_0.offset(i.wrapping_mul(6).wrapping_add(3) as isize) as u32)
                as u16;
            *b = 256u32
                .wrapping_mul(*in_0.offset(i.wrapping_mul(6).wrapping_add(4) as isize) as u32)
                .wrapping_add(*in_0.offset(i.wrapping_mul(6).wrapping_add(5) as isize) as u32)
                as u16;
            if (*mode).key_defined != 0
                && 256u32
                    .wrapping_mul(*in_0.offset(i.wrapping_mul(6).wrapping_add(0) as isize) as u32)
                    .wrapping_add(*in_0.offset(i.wrapping_mul(6).wrapping_add(1) as isize) as u32)
                    == (*mode).key_r
                && 256u32
                    .wrapping_mul(*in_0.offset(i.wrapping_mul(6).wrapping_add(2) as isize) as u32)
                    .wrapping_add(*in_0.offset(i.wrapping_mul(6).wrapping_add(3) as isize) as u32)
                    == (*mode).key_g
                && 256u32
                    .wrapping_mul(*in_0.offset(i.wrapping_mul(6).wrapping_add(4) as isize) as u32)
                    .wrapping_add(*in_0.offset(i.wrapping_mul(6).wrapping_add(5) as isize) as u32)
                    == (*mode).key_b
            {
                *a = 0;
            } else {
                *a = 65535;
            }
        } else if (*mode).colortype as u32 == LCT_GREY_ALPHA as u32 {
            *b = 256u32
                .wrapping_mul(*in_0.offset(i.wrapping_mul(4).wrapping_add(0) as isize) as u32)
                .wrapping_add(*in_0.offset(i.wrapping_mul(4).wrapping_add(1) as isize) as u32)
                as u16;
            *g = *b;
            *r = *g;
            *a = 256u32
                .wrapping_mul(*in_0.offset(i.wrapping_mul(4).wrapping_add(2) as isize) as u32)
                .wrapping_add(*in_0.offset(i.wrapping_mul(4).wrapping_add(3) as isize) as u32)
                as u16;
        } else if (*mode).colortype as u32 == LCT_RGBA as u32 {
            *r = 256u32
                .wrapping_mul(*in_0.offset(i.wrapping_mul(8).wrapping_add(0) as isize) as u32)
                .wrapping_add(*in_0.offset(i.wrapping_mul(8).wrapping_add(1) as isize) as u32)
                as u16;
            *g = 256u32
                .wrapping_mul(*in_0.offset(i.wrapping_mul(8).wrapping_add(2) as isize) as u32)
                .wrapping_add(*in_0.offset(i.wrapping_mul(8).wrapping_add(3) as isize) as u32)
                as u16;
            *b = 256u32
                .wrapping_mul(*in_0.offset(i.wrapping_mul(8).wrapping_add(4) as isize) as u32)
                .wrapping_add(*in_0.offset(i.wrapping_mul(8).wrapping_add(5) as isize) as u32)
                as u16;
            *a = 256u32
                .wrapping_mul(*in_0.offset(i.wrapping_mul(8).wrapping_add(6) as isize) as u32)
                .wrapping_add(*in_0.offset(i.wrapping_mul(8).wrapping_add(7) as isize) as u32)
                as u16;
        }
    }
}

#[no_mangle]
pub extern "C" fn lodepng_convert(
    mut out: *mut u8,
    mut in_0: *const u8,
    mut mode_out: *const LodePNGColorMode,
    mut mode_in: *const LodePNGColorMode,
    mut w: u32,
    mut h: u32,
) -> u32 {
    unsafe {
        let mut i: u64 = 0;
        let mut tree = ColorTree {
            children: [0 as *mut ColorTree; 16],
            index: 0,
        };
        let mut numpixels = (w as u64).wrapping_mul(h as u64);
        let mut error = 0;
        if (*mode_in).colortype as u32 == LCT_PALETTE as u32 && ((*mode_in).palette).is_null() {
            return 107;
        }
        if lodepng_color_mode_equal(mode_out, mode_in) != 0 {
            let mut numbytes = lodepng_get_raw_size(w, h, mode_in);
            lodepng_memcpy(
                out as *mut libc::c_void,
                in_0 as *const libc::c_void,
                numbytes,
            );
            return 0;
        }
        if (*mode_out).colortype as u32 == LCT_PALETTE as u32 {
            let mut palettesize = (*mode_out).palettesize;
            let mut palette: *const u8 = (*mode_out).palette;
            let mut palsize = 1 << (*mode_out).bitdepth;
            if palettesize == 0 {
                palettesize = (*mode_in).palettesize;
                palette = (*mode_in).palette;
                if (*mode_in).colortype as u32 == LCT_PALETTE as u32
                    && (*mode_in).bitdepth == (*mode_out).bitdepth
                {
                    let mut numbytes_0 = lodepng_get_raw_size(w, h, mode_in);
                    lodepng_memcpy(
                        out as *mut libc::c_void,
                        in_0 as *const libc::c_void,
                        numbytes_0,
                    );
                    return 0;
                }
            }
            if palettesize < palsize {
                palsize = palettesize;
            }
            color_tree_init(&mut tree);
            i = 0;
            while i != palsize {
                let mut p: *const u8 = &*palette.offset(i.wrapping_mul(4) as isize) as *const u8;
                error = color_tree_add(
                    &mut tree,
                    *p.offset(0 as isize),
                    *p.offset(1 as isize),
                    *p.offset(2 as isize),
                    *p.offset(3 as isize),
                    i as u32,
                );
                if error != 0 {
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
        if error == 0 {
            if (*mode_in).bitdepth == 16 && (*mode_out).bitdepth == 16 {
                i = 0;
                while i != numpixels {
                    let mut r = 0;
                    let mut g = 0;
                    let mut b = 0;
                    let mut a = 0;
                    getPixelColorRGBA16(&mut r, &mut g, &mut b, &mut a, in_0, i, mode_in);
                    rgba16ToPixel(out, i, mode_out, r, g, b, a);
                    i = i.wrapping_add(1);
                }
            } else if (*mode_out).bitdepth == 8 && (*mode_out).colortype as u32 == LCT_RGBA as u32 {
                getPixelColorsRGBA8(out, numpixels, in_0, mode_in);
            } else if (*mode_out).bitdepth == 8 && (*mode_out).colortype as u32 == LCT_RGB as u32 {
                getPixelColorsRGB8(out, numpixels, in_0, mode_in);
            } else {
                let mut r_0 = 0;
                let mut g_0 = 0;
                let mut b_0 = 0;
                let mut a_0 = 0;
                i = 0;
                while i != numpixels {
                    getPixelColorRGBA8(&mut r_0, &mut g_0, &mut b_0, &mut a_0, in_0, i, mode_in);
                    error = rgba8ToPixel(out, i, mode_out, &mut tree, r_0, g_0, b_0, a_0);
                    if error != 0 {
                        break;
                    }
                    i = i.wrapping_add(1);
                }
            }
        }
        if (*mode_out).colortype as u32 == LCT_PALETTE as u32 {
            color_tree_cleanup(&mut tree);
        }
        return error;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_convert_rgb(
    mut r_out: *mut u32,
    mut g_out: *mut u32,
    mut b_out: *mut u32,
    mut r_in: u32,
    mut g_in: u32,
    mut b_in: u32,
    mut mode_out: *const LodePNGColorMode,
    mut mode_in: *const LodePNGColorMode,
) -> u32 {
    unsafe {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut mul = 65535u32.wrapping_div((1u32 << (*mode_in).bitdepth).wrapping_sub(1));
        let mut shift = 16u32.wrapping_sub((*mode_out).bitdepth);
        if (*mode_in).colortype as u32 == LCT_GREY as u32
            || (*mode_in).colortype as u32 == LCT_GREY_ALPHA as u32
        {
            b = r_in.wrapping_mul(mul);
            g = b;
            r = g;
        } else if (*mode_in).colortype as u32 == LCT_RGB as u32
            || (*mode_in).colortype as u32 == LCT_RGBA as u32
        {
            r = r_in.wrapping_mul(mul);
            g = g_in.wrapping_mul(mul);
            b = b_in.wrapping_mul(mul);
        } else if (*mode_in).colortype as u32 == LCT_PALETTE as u32 {
            if r_in as u64 >= (*mode_in).palettesize {
                return 82;
            }
            r = (*((*mode_in).palette).offset(r_in.wrapping_mul(4u32).wrapping_add(0) as isize)
                as u32)
                .wrapping_mul(257);
            g = (*((*mode_in).palette).offset(r_in.wrapping_mul(4u32).wrapping_add(1) as isize)
                as u32)
                .wrapping_mul(257);
            b = (*((*mode_in).palette).offset(r_in.wrapping_mul(4u32).wrapping_add(2) as isize)
                as u32)
                .wrapping_mul(257);
        } else {
            return 31;
        }
        if (*mode_out).colortype as u32 == LCT_GREY as u32
            || (*mode_out).colortype as u32 == LCT_GREY_ALPHA as u32
        {
            *r_out = r >> shift;
        } else if (*mode_out).colortype as u32 == LCT_RGB as u32
            || (*mode_out).colortype as u32 == LCT_RGBA as u32
        {
            *r_out = r >> shift;
            *g_out = g >> shift;
            *b_out = b >> shift;
        } else if (*mode_out).colortype as u32 == LCT_PALETTE as u32 {
            let mut i: u32 = 0;
            if r >> 8 != r & 255 || g >> 8 != g & 255 || b >> 8 != b & 255 {
                return 82;
            }
            i = 0;
            while (i as u64) < (*mode_out).palettesize {
                let mut j = i.wrapping_mul(4);
                if r >> 8 == *((*mode_out).palette).offset(j.wrapping_add(0) as isize) as u32
                    && g >> 8 == *((*mode_out).palette).offset(j.wrapping_add(1) as isize) as u32
                    && b >> 8 == *((*mode_out).palette).offset(j.wrapping_add(2) as isize) as u32
                {
                    *r_out = i;
                    return 0;
                }
                i = i.wrapping_add(1);
            }
            return 82;
        } else {
            return 31;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_color_stats_init(mut stats: *mut LodePNGColorStats) {
    unsafe {
        (*stats).colored = 0;
        (*stats).key = 0;
        let ref mut fresh159 = (*stats).key_b;
        *fresh159 = 0;
        let ref mut fresh160 = (*stats).key_g;
        *fresh160 = *fresh159;
        (*stats).key_r = *fresh160;
        (*stats).alpha = 0;
        (*stats).numcolors = 0;
        (*stats).bits = 1;
        (*stats).numpixels = 0;
        (*stats).allow_palette = 1;
        (*stats).allow_greyscale = 1;
    }
}

extern "C" fn getValueRequiredBits(mut value: u8) -> u32 {
    if value as i32 == 0 || value as i32 == 255 {
        return 1;
    }
    if value as i32 % 17 == 0 {
        return (if value as i32 % 85 == 0i32 { 2 } else { 4 }) as u32;
    }
    return 8;
}

#[no_mangle]
pub extern "C" fn lodepng_compute_color_stats(
    mut stats: *mut LodePNGColorStats,
    mut in_0: *const u8,
    mut w: u32,
    mut h: u32,
    mut mode_in: *const LodePNGColorMode,
) -> u32 {
    unsafe {
        let mut current_block: u64;
        let mut i: u64 = 0;
        let mut tree = ColorTree {
            children: [0 as *mut ColorTree; 16],
            index: 0,
        };
        let mut numpixels = (w as u64).wrapping_mul(h as u64);
        let mut error = 0;
        let mut colored_done = (if lodepng_is_greyscale_type(mode_in) != 0 {
            1
        } else {
            0
        }) as u32;
        let mut alpha_done = (if lodepng_can_have_alpha(mode_in) != 0 {
            0
        } else {
            1
        }) as u32;
        let mut numcolors_done = 0;
        let mut bpp = lodepng_get_bpp(mode_in);
        let mut bits_done = (if (*stats).bits == 1 && bpp == 1 { 1 } else { 0 }) as u32;
        let mut sixteen = 0;
        let mut maxnumcolors = 257;
        if bpp <= 8 {
            maxnumcolors = if 257u32 < ((*stats).numcolors).wrapping_add(1 << bpp) {
                257
            } else {
                ((*stats).numcolors).wrapping_add(1 << bpp)
            };
        }
        let ref mut fresh161 = (*stats).numpixels;
        *fresh161 = (*fresh161 as u64).wrapping_add(numpixels) as u64;
        if (*stats).allow_palette == 0 {
            numcolors_done = 1;
        }
        color_tree_init(&mut tree);
        if (*stats).alpha != 0 {
            alpha_done = 1;
        }
        if (*stats).colored != 0 {
            colored_done = 1;
        }
        if (*stats).bits == 16 {
            numcolors_done = 1;
        }
        if (*stats).bits >= bpp {
            bits_done = 1;
        }
        if (*stats).numcolors >= maxnumcolors {
            numcolors_done = 1;
        }
        if numcolors_done == 0 {
            i = 0;
            loop {
                if !(i < (*stats).numcolors as u64) {
                    current_block = 15925075030174552612;
                    break;
                }
                let mut color: *const u8 = &mut *((*stats).palette)
                    .as_mut_ptr()
                    .offset(i.wrapping_mul(4) as isize)
                    as *mut u8;
                error = color_tree_add(
                    &mut tree,
                    *color.offset(0 as isize),
                    *color.offset(1 as isize),
                    *color.offset(2 as isize),
                    *color.offset(3 as isize),
                    i as u32,
                );
                if error != 0 {
                    current_block = 2485031591849961683;
                    break;
                }
                i = i.wrapping_add(1);
            }
        } else {
            current_block = 15925075030174552612;
        }
        match current_block {
            15925075030174552612 => {
                if (*mode_in).bitdepth == 16 && sixteen == 0 {
                    let mut r = 0;
                    let mut g = 0;
                    let mut b = 0;
                    let mut a = 0;
                    i = 0;
                    while i != numpixels {
                        getPixelColorRGBA16(&mut r, &mut g, &mut b, &mut a, in_0, i, mode_in);
                        if r as i32 & 255 != r as i32 >> 8 & 255
                            || g as i32 & 255 != g as i32 >> 8 & 255
                            || b as i32 & 255 != b as i32 >> 8 & 255
                            || a as i32 & 255 != a as i32 >> 8 & 255
                        {
                            (*stats).bits = 16;
                            sixteen = 1;
                            bits_done = 1;
                            numcolors_done = 1;
                            break;
                        } else {
                            i = i.wrapping_add(1);
                        }
                    }
                }
                if sixteen != 0 {
                    let mut r_0 = 0;
                    let mut g_0 = 0;
                    let mut b_0 = 0;
                    let mut a_0 = 0;
                    i = 0;
                    while i != numpixels {
                        getPixelColorRGBA16(
                            &mut r_0, &mut g_0, &mut b_0, &mut a_0, in_0, i, mode_in,
                        );
                        if colored_done == 0
                            && (r_0 as i32 != g_0 as i32 || r_0 as i32 != b_0 as i32)
                        {
                            (*stats).colored = 1;
                            colored_done = 1;
                        }
                        if alpha_done == 0 {
                            let mut matchkey = (r_0 as i32 == (*stats).key_r as i32
                                && g_0 as i32 == (*stats).key_g as i32
                                && b_0 as i32 == (*stats).key_b as i32)
                                as u32;
                            if a_0 as i32 != 65535
                                && (a_0 as i32 != 0 || (*stats).key != 0 && matchkey == 0)
                            {
                                (*stats).alpha = 1;
                                (*stats).key = 0;
                                alpha_done = 1;
                            } else if a_0 as i32 == 0 && (*stats).alpha == 0 && (*stats).key == 0 {
                                (*stats).key = 1;
                                (*stats).key_r = r_0;
                                (*stats).key_g = g_0;
                                (*stats).key_b = b_0;
                            } else if a_0 as i32 == 65535 && (*stats).key != 0 && matchkey != 0 {
                                (*stats).alpha = 1;
                                (*stats).key = 0;
                                alpha_done = 1;
                            }
                        }
                        if alpha_done != 0
                            && numcolors_done != 0
                            && colored_done != 0
                            && bits_done != 0
                        {
                            break;
                        }
                        i = i.wrapping_add(1);
                    }
                    if (*stats).key != 0 && (*stats).alpha == 0 {
                        i = 0;
                        while i != numpixels {
                            getPixelColorRGBA16(
                                &mut r_0, &mut g_0, &mut b_0, &mut a_0, in_0, i, mode_in,
                            );
                            if a_0 as i32 != 0
                                && r_0 as i32 == (*stats).key_r as i32
                                && g_0 as i32 == (*stats).key_g as i32
                                && b_0 as i32 == (*stats).key_b as i32
                            {
                                (*stats).alpha = 1;
                                (*stats).key = 0;
                                alpha_done = 1;
                            }
                            i = i.wrapping_add(1);
                        }
                    }
                } else {
                    let mut r_1 = 0;
                    let mut g_1 = 0;
                    let mut b_1 = 0;
                    let mut a_1 = 0;
                    i = 0;
                    loop {
                        if !(i != numpixels) {
                            current_block = 17736998403848444560;
                            break;
                        }
                        getPixelColorRGBA8(
                            &mut r_1, &mut g_1, &mut b_1, &mut a_1, in_0, i, mode_in,
                        );
                        if bits_done == 0 && (*stats).bits < 8 {
                            let mut bits = getValueRequiredBits(r_1);
                            if bits > (*stats).bits {
                                (*stats).bits = bits;
                            }
                        }
                        bits_done = ((*stats).bits >= bpp) as u32;
                        if colored_done == 0
                            && (r_1 as i32 != g_1 as i32 || r_1 as i32 != b_1 as i32)
                        {
                            (*stats).colored = 1;
                            colored_done = 1;
                            if (*stats).bits < 8 {
                                (*stats).bits = 8;
                            }
                        }
                        if alpha_done == 0 {
                            let mut matchkey_0 = (r_1 as i32 == (*stats).key_r as i32
                                && g_1 as i32 == (*stats).key_g as i32
                                && b_1 as i32 == (*stats).key_b as i32)
                                as u32;
                            if a_1 as i32 != 255
                                && (a_1 as i32 != 0 || (*stats).key != 0 && matchkey_0 == 0)
                            {
                                (*stats).alpha = 1;
                                (*stats).key = 0;
                                alpha_done = 1;
                                if (*stats).bits < 8 {
                                    (*stats).bits = 8;
                                }
                            } else if a_1 as i32 == 0 && (*stats).alpha == 0 && (*stats).key == 0 {
                                (*stats).key = 1;
                                (*stats).key_r = r_1 as u16;
                                (*stats).key_g = g_1 as u16;
                                (*stats).key_b = b_1 as u16;
                            } else if a_1 as i32 == 255 && (*stats).key != 0 && matchkey_0 != 0 {
                                (*stats).alpha = 1;
                                (*stats).key = 0;
                                alpha_done = 1;
                                if (*stats).bits < 8 {
                                    (*stats).bits = 8;
                                }
                            }
                        }
                        if numcolors_done == 0 {
                            if color_tree_has(&mut tree, r_1, g_1, b_1, a_1) == 0 {
                                error = color_tree_add(
                                    &mut tree,
                                    r_1,
                                    g_1,
                                    b_1,
                                    a_1,
                                    (*stats).numcolors,
                                );
                                if error != 0 {
                                    current_block = 2485031591849961683;
                                    break;
                                }
                                if (*stats).numcolors < 256 {
                                    let mut p = ((*stats).palette).as_mut_ptr();
                                    let mut n = (*stats).numcolors;
                                    *p.offset(n.wrapping_mul(4).wrapping_add(0) as isize) = r_1;
                                    *p.offset(n.wrapping_mul(4).wrapping_add(1) as isize) = g_1;
                                    *p.offset(n.wrapping_mul(4).wrapping_add(2) as isize) = b_1;
                                    *p.offset(n.wrapping_mul(4).wrapping_add(3) as isize) = a_1;
                                }
                                let ref mut fresh162 = (*stats).numcolors;
                                *fresh162 = (*fresh162).wrapping_add(1);
                                numcolors_done = ((*stats).numcolors >= maxnumcolors) as u32;
                            }
                        }
                        if alpha_done != 0
                            && numcolors_done != 0
                            && colored_done != 0
                            && bits_done != 0
                        {
                            current_block = 17736998403848444560;
                            break;
                        }
                        i = i.wrapping_add(1);
                    }
                    match current_block {
                        2485031591849961683 => {}
                        _ => {
                            if (*stats).key != 0 && (*stats).alpha == 0 {
                                i = 0;
                                while i != numpixels {
                                    getPixelColorRGBA8(
                                        &mut r_1, &mut g_1, &mut b_1, &mut a_1, in_0, i, mode_in,
                                    );
                                    if a_1 as i32 != 0
                                        && r_1 as i32 == (*stats).key_r as i32
                                        && g_1 as i32 == (*stats).key_g as i32
                                        && b_1 as i32 == (*stats).key_b as i32
                                    {
                                        (*stats).alpha = 1;
                                        (*stats).key = 0;
                                        alpha_done = 1;
                                        if (*stats).bits < 8 {
                                            (*stats).bits = 8;
                                        }
                                    }
                                    i = i.wrapping_add(1);
                                }
                            }
                            let ref mut fresh163 = (*stats).key_r;
                            *fresh163 =
                                (*fresh163 as i32 + (((*stats).key_r as i32) << 8i32)) as u16;
                            let ref mut fresh164 = (*stats).key_g;
                            *fresh164 =
                                (*fresh164 as i32 + (((*stats).key_g as i32) << 8i32)) as u16;
                            let ref mut fresh165 = (*stats).key_b;
                            *fresh165 =
                                (*fresh165 as i32 + (((*stats).key_b as i32) << 8i32)) as u16;
                        }
                    }
                }
            }
            _ => {}
        }
        color_tree_cleanup(&mut tree);
        return error;
    }
}

extern "C" fn lodepng_color_stats_add(
    mut stats: *mut LodePNGColorStats,
    mut r: u32,
    mut g: u32,
    mut b: u32,
    mut a: u32,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut image: [u8; 8] = [0; 8];
        let mut mode = LodePNGColorMode {
            colortype: LCT_GREY,
            bitdepth: 0,
            palette: 0 as *mut u8,
            palettesize: 0,
            key_defined: 0,
            key_r: 0,
            key_g: 0,
            key_b: 0,
        };
        lodepng_color_mode_init(&mut mode);
        image[0 as usize] = (r >> 8i32) as u8;
        image[1 as usize] = r as u8;
        image[2 as usize] = (g >> 8i32) as u8;
        image[3 as usize] = g as u8;
        image[4 as usize] = (b >> 8i32) as u8;
        image[5 as usize] = b as u8;
        image[6 as usize] = (a >> 8i32) as u8;
        image[7 as usize] = a as u8;
        mode.bitdepth = 16;
        mode.colortype = LCT_RGBA;
        error = lodepng_compute_color_stats(stats, image.as_mut_ptr(), 1, 1, &mut mode);
        lodepng_color_mode_cleanup(&mut mode);
        return error;
    }
}

extern "C" fn auto_choose_color(
    mut mode_out: *mut LodePNGColorMode,
    mut mode_in: *const LodePNGColorMode,
    mut stats: *const LodePNGColorStats,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut palettebits: u32 = 0;
        let mut i: u64 = 0;
        let mut n: u64 = 0;
        let mut numpixels = (*stats).numpixels;
        let mut palette_ok: u32 = 0;
        let mut gray_ok: u32 = 0;
        let mut alpha = (*stats).alpha;
        let mut key = (*stats).key;
        let mut bits = (*stats).bits;
        (*mode_out).key_defined = 0;
        if key != 0 && numpixels <= 16 {
            alpha = 1;
            key = 0;
            if bits < 8 {
                bits = 8;
            }
        }
        gray_ok = ((*stats).colored == 0) as u32;
        if (*stats).allow_greyscale == 0 {
            gray_ok = 0;
        }
        if gray_ok == 0 && bits < 8 {
            bits = 8;
        }
        n = (*stats).numcolors as u64;
        palettebits = (if n <= 2 {
            1
        } else if n <= 4 {
            2
        } else if n <= 16 {
            4
        } else {
            8
        }) as u32;
        palette_ok = (n <= 256 && bits <= 8 && n != 0) as u32;
        if numpixels < n.wrapping_mul(2) {
            palette_ok = 0;
        }
        if gray_ok != 0 && alpha == 0 && bits <= palettebits {
            palette_ok = 0;
        }
        if (*stats).allow_palette == 0 {
            palette_ok = 0;
        }
        if palette_ok != 0 {
            let mut p = ((*stats).palette).as_ptr();
            lodepng_palette_clear(mode_out);
            i = 0;
            while i != (*stats).numcolors as u64 {
                error = lodepng_palette_add(
                    mode_out,
                    *p.offset(i.wrapping_mul(4).wrapping_add(0) as isize),
                    *p.offset(i.wrapping_mul(4).wrapping_add(1) as isize),
                    *p.offset(i.wrapping_mul(4).wrapping_add(2) as isize),
                    *p.offset(i.wrapping_mul(4).wrapping_add(3) as isize),
                );
                if error != 0 {
                    break;
                }
                i = i.wrapping_add(1);
            }
            (*mode_out).colortype = LCT_PALETTE;
            (*mode_out).bitdepth = palettebits;
            if (*mode_in).colortype as u32 == LCT_PALETTE as u32
                && (*mode_in).palettesize >= (*mode_out).palettesize
                && (*mode_in).bitdepth == (*mode_out).bitdepth
            {
                lodepng_color_mode_cleanup(mode_out);
                lodepng_color_mode_copy(mode_out, mode_in);
            }
        } else {
            (*mode_out).bitdepth = bits;
            (*mode_out).colortype = (if alpha != 0 {
                if gray_ok != 0 {
                    LCT_GREY_ALPHA as i32
                } else {
                    LCT_RGBA as i32
                }
            } else if gray_ok != 0 {
                LCT_GREY as i32
            } else {
                LCT_RGB as i32
            }) as u32;
            if key != 0 {
                let mut mask_0 = (1u32 << (*mode_out).bitdepth).wrapping_sub(1);
                (*mode_out).key_r = (*stats).key_r as u32 & mask_0;
                (*mode_out).key_g = (*stats).key_g as u32 & mask_0;
                (*mode_out).key_b = (*stats).key_b as u32 & mask_0;
                (*mode_out).key_defined = 1;
            }
        }
        return error;
    }
}

extern "C" fn paethPredictor(mut a: i16, mut b: i16, mut c: i16) -> u8 {
    let mut pa = (if (b as i32 - c as i32) < 0i32 {
        -(b as i32 - c as i32)
    } else {
        b as i32 - c as i32
    }) as i16;
    let mut pb = (if (a as i32 - c as i32) < 0i32 {
        -(a as i32 - c as i32)
    } else {
        a as i32 - c as i32
    }) as i16;
    let mut pc = (if (a as i32 + b as i32 - c as i32 - c as i32) < 0i32 {
        -(a as i32 + b as i32 - c as i32 - c as i32)
    } else {
        a as i32 + b as i32 - c as i32 - c as i32
    }) as i16;
    if (pb as i32) < pa as i32 {
        a = b;
        pa = pb;
    }
    return (if (pc as i32) < pa as i32 {
        c as i32
    } else {
        a as i32
    }) as u8;
}

static mut ADAM7_IX: [u32; 7] = [0, 4, 0, 2, 0, 1, 0];
static mut ADAM7_IY: [u32; 7] = [0, 0, 4, 0, 2, 0, 1];
static mut ADAM7_DX: [u32; 7] = [8, 8, 4, 4, 2, 2, 1];
static mut ADAM7_DY: [u32; 7] = [8, 8, 8, 4, 4, 2, 2];
extern "C" fn Adam7_getpassvalues(
    mut passw: *mut u32,
    mut passh: *mut u32,
    mut filter_passstart: *mut u64,
    mut padded_passstart: *mut u64,
    mut passstart: *mut u64,
    mut w: u32,
    mut h: u32,
    mut bpp: u32,
) {
    unsafe {
        let mut i: u32 = 0;
        i = 0;
        while i != 7 {
            *passw.offset(i as isize) = w
                .wrapping_add(ADAM7_DX[i as usize])
                .wrapping_sub(ADAM7_IX[i as usize])
                .wrapping_sub(1)
                .wrapping_div(ADAM7_DX[i as usize]);
            *passh.offset(i as isize) = h
                .wrapping_add(ADAM7_DY[i as usize])
                .wrapping_sub(ADAM7_IY[i as usize])
                .wrapping_sub(1)
                .wrapping_div(ADAM7_DY[i as usize]);
            if *passw.offset(i as isize) == 0 {
                *passh.offset(i as isize) = 0;
            }
            if *passh.offset(i as isize) == 0 {
                *passw.offset(i as isize) = 0;
            }
            i = i.wrapping_add(1);
        }
        let ref mut fresh166 = *passstart.offset(0 as isize);
        *fresh166 = 0;
        let ref mut fresh167 = *padded_passstart.offset(0 as isize);
        *fresh167 = *fresh166;
        *filter_passstart.offset(0 as isize) = *fresh167;
        i = 0;
        while i != 7 {
            *filter_passstart.offset(i.wrapping_add(1) as isize) =
                (*filter_passstart.offset(i as isize)).wrapping_add(
                    (if *passw.offset(i as isize) != 0 && *passh.offset(i as isize) != 0 {
                        (*passh.offset(i as isize)).wrapping_mul(
                            1u32.wrapping_add(
                                (*passw.offset(i as isize))
                                    .wrapping_mul(bpp)
                                    .wrapping_add(7)
                                    .wrapping_div(8),
                            ),
                        )
                    } else {
                        0
                    }) as u64,
                );
            *padded_passstart.offset(i.wrapping_add(1) as isize) =
                (*padded_passstart.offset(i as isize)).wrapping_add(
                    (*passh.offset(i as isize)).wrapping_mul(
                        (*passw.offset(i as isize))
                            .wrapping_mul(bpp)
                            .wrapping_add(7)
                            .wrapping_div(8),
                    ) as u64,
                );
            *passstart.offset(i.wrapping_add(1) as isize) = (*passstart.offset(i as isize))
                .wrapping_add(
                    (*passh.offset(i as isize))
                        .wrapping_mul(*passw.offset(i as isize))
                        .wrapping_mul(bpp)
                        .wrapping_add(7)
                        .wrapping_div(8) as u64,
                );
            i = i.wrapping_add(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn lodepng_inspect(
    mut w: *mut u32,
    mut h: *mut u32,
    mut state: *mut LodePNGState,
    mut in_0: *const u8,
    mut insize: u64,
) -> u32 {
    unsafe {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut info: *mut LodePNGInfo = &mut (*state).info_png;
        if insize == 0 || in_0.is_null() {
            (*state).error = 48;
            return 48;
        }
        if insize < 33 {
            (*state).error = 27;
            return 27;
        }
        lodepng_info_cleanup(info);
        lodepng_info_init(info);
        if *in_0.offset(0 as isize) as i32 != 137
            || *in_0.offset(1 as isize) as i32 != 80
            || *in_0.offset(2 as isize) as i32 != 78
            || *in_0.offset(3 as isize) as i32 != 71
            || *in_0.offset(4 as isize) as i32 != 13
            || *in_0.offset(5 as isize) as i32 != 10
            || *in_0.offset(6 as isize) as i32 != 26
            || *in_0.offset(7 as isize) as i32 != 10
        {
            (*state).error = 28;
            return 28;
        }
        if lodepng_chunk_length(in_0.offset(8 as isize)) != 13 {
            (*state).error = 94;
            return 94;
        }
        if lodepng_chunk_type_equals(in_0.offset(8 as isize), b"IHDR\0" as *const u8 as *const i8)
            == 0
        {
            (*state).error = 29;
            return 29;
        }
        width = lodepng_read32bitInt(&*in_0.offset(16 as isize));
        height = lodepng_read32bitInt(&*in_0.offset(20 as isize));
        if !w.is_null() {
            *w = width;
        }
        if !h.is_null() {
            *h = height;
        };
        (*info).color.bitdepth = *in_0.offset(24 as isize) as u32;
        (*info).color.colortype = *in_0.offset(25 as isize) as u32;
        (*info).compression_method = *in_0.offset(26 as isize) as u32;
        (*info).filter_method = *in_0.offset(27 as isize) as u32;
        (*info).interlace_method = *in_0.offset(28 as isize) as u32;
        if width == 0 || height == 0 {
            (*state).error = 93;
            return 93;
        };
        (*state).error = checkColorValidity((*info).color.colortype, (*info).color.bitdepth);
        if (*state).error != 0 {
            return (*state).error;
        }
        if (*info).compression_method != 0 {
            (*state).error = 32;
            return 32;
        }
        if (*info).filter_method != 0 {
            (*state).error = 33;
            return 33;
        }
        if (*info).interlace_method > 1 {
            (*state).error = 34;
            return 34;
        }
        if (*state).decoder.ignore_crc == 0 {
            let mut CRC = lodepng_read32bitInt(&*in_0.offset(29 as isize));
            let mut checksum = lodepng_crc32(&*in_0.offset(12 as isize), 17);
            if CRC != checksum {
                (*state).error = 57;
                return 57;
            }
        }
        return (*state).error;
    }
}

extern "C" fn unfilterScanline(
    mut recon: *mut u8,
    mut scanline: *const u8,
    mut precon: *const u8,
    mut bytewidth: u64,
    mut filterType: u8,
    mut length: u64,
) -> u32 {
    unsafe {
        let mut i: u64 = 0;
        match filterType as i32 {
            0 => {
                i = 0;
                while i != length {
                    *recon.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
            }
            1 => {
                let mut j = 0;
                i = 0;
                while i != bytewidth {
                    *recon.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i != length {
                    *recon.offset(i as isize) = (*scanline.offset(i as isize) as i32
                        + *recon.offset(j as isize) as i32)
                        as u8;
                    i = i.wrapping_add(1);
                    j = j.wrapping_add(1);
                }
            }
            2 => {
                if !precon.is_null() {
                    i = 0;
                    while i != length {
                        *recon.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            + *precon.offset(i as isize) as i32)
                            as u8;
                        i = i.wrapping_add(1);
                    }
                } else {
                    i = 0;
                    while i != length {
                        *recon.offset(i as isize) = *scanline.offset(i as isize);
                        i = i.wrapping_add(1);
                    }
                }
            }
            3 => {
                if !precon.is_null() {
                    let mut j_0 = 0;
                    i = 0;
                    while i != bytewidth {
                        *recon.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            + (*precon.offset(i as isize) as i32 >> 1u32))
                            as u8;
                        i = i.wrapping_add(1);
                    }
                    if bytewidth >= 4 {
                        while i.wrapping_add(3) < length {
                            let mut s0 = *scanline.offset(i.wrapping_add(0) as isize);
                            let mut s1 = *scanline.offset(i.wrapping_add(1) as isize);
                            let mut s2 = *scanline.offset(i.wrapping_add(2) as isize);
                            let mut s3 = *scanline.offset(i.wrapping_add(3) as isize);
                            let mut r0 = *recon.offset(j_0.wrapping_add(0) as isize);
                            let mut r1 = *recon.offset(j_0.wrapping_add(1) as isize);
                            let mut r2 = *recon.offset(j_0.wrapping_add(2) as isize);
                            let mut r3 = *recon.offset(j_0.wrapping_add(3) as isize);
                            let mut p0 = *precon.offset(i.wrapping_add(0) as isize);
                            let mut p1 = *precon.offset(i.wrapping_add(1) as isize);
                            let mut p2 = *precon.offset(i.wrapping_add(2) as isize);
                            let mut p3 = *precon.offset(i.wrapping_add(3) as isize);
                            *recon.offset(i.wrapping_add(0) as isize) =
                                (s0 as i32 + (r0 as i32 + p0 as i32 >> 1u32)) as u8;
                            *recon.offset(i.wrapping_add(1) as isize) =
                                (s1 as i32 + (r1 as i32 + p1 as i32 >> 1u32)) as u8;
                            *recon.offset(i.wrapping_add(2) as isize) =
                                (s2 as i32 + (r2 as i32 + p2 as i32 >> 1u32)) as u8;
                            *recon.offset(i.wrapping_add(3) as isize) =
                                (s3 as i32 + (r3 as i32 + p3 as i32 >> 1u32)) as u8;
                            i = (i).wrapping_add(4) as u64;
                            j_0 = (j_0 as u64).wrapping_add(4) as u64;
                        }
                    } else if bytewidth >= 3 {
                        while i.wrapping_add(2) < length {
                            let mut s0_0 = *scanline.offset(i.wrapping_add(0) as isize);
                            let mut s1_0 = *scanline.offset(i.wrapping_add(1) as isize);
                            let mut s2_0 = *scanline.offset(i.wrapping_add(2) as isize);
                            let mut r0_0 = *recon.offset(j_0.wrapping_add(0) as isize);
                            let mut r1_0 = *recon.offset(j_0.wrapping_add(1) as isize);
                            let mut r2_0 = *recon.offset(j_0.wrapping_add(2) as isize);
                            let mut p0_0 = *precon.offset(i.wrapping_add(0) as isize);
                            let mut p1_0 = *precon.offset(i.wrapping_add(1) as isize);
                            let mut p2_0 = *precon.offset(i.wrapping_add(2) as isize);
                            *recon.offset(i.wrapping_add(0) as isize) =
                                (s0_0 as i32 + (r0_0 as i32 + p0_0 as i32 >> 1u32)) as u8;
                            *recon.offset(i.wrapping_add(1) as isize) =
                                (s1_0 as i32 + (r1_0 as i32 + p1_0 as i32 >> 1u32)) as u8;
                            *recon.offset(i.wrapping_add(2) as isize) =
                                (s2_0 as i32 + (r2_0 as i32 + p2_0 as i32 >> 1u32)) as u8;
                            i = (i).wrapping_add(3) as u64;
                            j_0 = (j_0 as u64).wrapping_add(3) as u64;
                        }
                    } else if bytewidth >= 2 {
                        while i.wrapping_add(1) < length {
                            let mut s0_1 = *scanline.offset(i.wrapping_add(0) as isize);
                            let mut s1_1 = *scanline.offset(i.wrapping_add(1) as isize);
                            let mut r0_1 = *recon.offset(j_0.wrapping_add(0) as isize);
                            let mut r1_1 = *recon.offset(j_0.wrapping_add(1) as isize);
                            let mut p0_1 = *precon.offset(i.wrapping_add(0) as isize);
                            let mut p1_1 = *precon.offset(i.wrapping_add(1) as isize);
                            *recon.offset(i.wrapping_add(0) as isize) =
                                (s0_1 as i32 + (r0_1 as i32 + p0_1 as i32 >> 1u32)) as u8;
                            *recon.offset(i.wrapping_add(1) as isize) =
                                (s1_1 as i32 + (r1_1 as i32 + p1_1 as i32 >> 1u32)) as u8;
                            i = (i).wrapping_add(2) as u64;
                            j_0 = (j_0 as u64).wrapping_add(2) as u64;
                        }
                    }
                    while i != length {
                        *recon.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            + (*recon.offset(j_0 as isize) as i32
                                + *precon.offset(i as isize) as i32
                                >> 1u32)) as u8;
                        i = i.wrapping_add(1);
                        j_0 = j_0.wrapping_add(1);
                    }
                } else {
                    let mut j_1 = 0;
                    i = 0;
                    while i != bytewidth {
                        *recon.offset(i as isize) = *scanline.offset(i as isize);
                        i = i.wrapping_add(1);
                    }
                    i = bytewidth;
                    while i != length {
                        *recon.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            + (*recon.offset(j_1 as isize) as i32 >> 1u32))
                            as u8;
                        i = i.wrapping_add(1);
                        j_1 = j_1.wrapping_add(1);
                    }
                }
            }
            4 => {
                if !precon.is_null() {
                    let mut j_2 = 0;
                    i = 0;
                    while i != bytewidth {
                        *recon.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            + *precon.offset(i as isize) as i32)
                            as u8;
                        i = i.wrapping_add(1);
                    }
                    if bytewidth >= 4 {
                        while i.wrapping_add(3) < length {
                            let mut s0_2 = *scanline.offset(i.wrapping_add(0) as isize);
                            let mut s1_2 = *scanline.offset(i.wrapping_add(1) as isize);
                            let mut s2_1 = *scanline.offset(i.wrapping_add(2) as isize);
                            let mut s3_0 = *scanline.offset(i.wrapping_add(3) as isize);
                            let mut r0_2 = *recon.offset(j_2.wrapping_add(0) as isize);
                            let mut r1_2 = *recon.offset(j_2.wrapping_add(1) as isize);
                            let mut r2_1 = *recon.offset(j_2.wrapping_add(2) as isize);
                            let mut r3_0 = *recon.offset(j_2.wrapping_add(3) as isize);
                            let mut p0_2 = *precon.offset(i.wrapping_add(0) as isize);
                            let mut p1_2 = *precon.offset(i.wrapping_add(1) as isize);
                            let mut p2_1 = *precon.offset(i.wrapping_add(2) as isize);
                            let mut p3_0 = *precon.offset(i.wrapping_add(3) as isize);
                            let mut q0 = *precon.offset(j_2.wrapping_add(0) as isize);
                            let mut q1 = *precon.offset(j_2.wrapping_add(1) as isize);
                            let mut q2 = *precon.offset(j_2.wrapping_add(2) as isize);
                            let mut q3 = *precon.offset(j_2.wrapping_add(3) as isize);
                            *recon.offset(i.wrapping_add(0) as isize) = (s0_2 as i32
                                + paethPredictor(r0_2 as i16, p0_2 as i16, q0 as i16) as i32)
                                as u8;
                            *recon.offset(i.wrapping_add(1) as isize) = (s1_2 as i32
                                + paethPredictor(r1_2 as i16, p1_2 as i16, q1 as i16) as i32)
                                as u8;
                            *recon.offset(i.wrapping_add(2) as isize) = (s2_1 as i32
                                + paethPredictor(r2_1 as i16, p2_1 as i16, q2 as i16) as i32)
                                as u8;
                            *recon.offset(i.wrapping_add(3) as isize) = (s3_0 as i32
                                + paethPredictor(r3_0 as i16, p3_0 as i16, q3 as i16) as i32)
                                as u8;
                            i = (i).wrapping_add(4) as u64;
                            j_2 = (j_2 as u64).wrapping_add(4) as u64;
                        }
                    } else if bytewidth >= 3 {
                        while i.wrapping_add(2) < length {
                            let mut s0_3 = *scanline.offset(i.wrapping_add(0) as isize);
                            let mut s1_3 = *scanline.offset(i.wrapping_add(1) as isize);
                            let mut s2_2 = *scanline.offset(i.wrapping_add(2) as isize);
                            let mut r0_3 = *recon.offset(j_2.wrapping_add(0) as isize);
                            let mut r1_3 = *recon.offset(j_2.wrapping_add(1) as isize);
                            let mut r2_2 = *recon.offset(j_2.wrapping_add(2) as isize);
                            let mut p0_3 = *precon.offset(i.wrapping_add(0) as isize);
                            let mut p1_3 = *precon.offset(i.wrapping_add(1) as isize);
                            let mut p2_2 = *precon.offset(i.wrapping_add(2) as isize);
                            let mut q0_0 = *precon.offset(j_2.wrapping_add(0) as isize);
                            let mut q1_0 = *precon.offset(j_2.wrapping_add(1) as isize);
                            let mut q2_0 = *precon.offset(j_2.wrapping_add(2) as isize);
                            *recon.offset(i.wrapping_add(0) as isize) = (s0_3 as i32
                                + paethPredictor(r0_3 as i16, p0_3 as i16, q0_0 as i16) as i32)
                                as u8;
                            *recon.offset(i.wrapping_add(1) as isize) = (s1_3 as i32
                                + paethPredictor(r1_3 as i16, p1_3 as i16, q1_0 as i16) as i32)
                                as u8;
                            *recon.offset(i.wrapping_add(2) as isize) = (s2_2 as i32
                                + paethPredictor(r2_2 as i16, p2_2 as i16, q2_0 as i16) as i32)
                                as u8;
                            i = (i).wrapping_add(3) as u64;
                            j_2 = (j_2 as u64).wrapping_add(3) as u64;
                        }
                    } else if bytewidth >= 2 {
                        while i.wrapping_add(1) < length {
                            let mut s0_4 = *scanline.offset(i.wrapping_add(0) as isize);
                            let mut s1_4 = *scanline.offset(i.wrapping_add(1) as isize);
                            let mut r0_4 = *recon.offset(j_2.wrapping_add(0) as isize);
                            let mut r1_4 = *recon.offset(j_2.wrapping_add(1) as isize);
                            let mut p0_4 = *precon.offset(i.wrapping_add(0) as isize);
                            let mut p1_4 = *precon.offset(i.wrapping_add(1) as isize);
                            let mut q0_1 = *precon.offset(j_2.wrapping_add(0) as isize);
                            let mut q1_1 = *precon.offset(j_2.wrapping_add(1) as isize);
                            *recon.offset(i.wrapping_add(0) as isize) = (s0_4 as i32
                                + paethPredictor(r0_4 as i16, p0_4 as i16, q0_1 as i16) as i32)
                                as u8;
                            *recon.offset(i.wrapping_add(1) as isize) = (s1_4 as i32
                                + paethPredictor(r1_4 as i16, p1_4 as i16, q1_1 as i16) as i32)
                                as u8;
                            i = (i).wrapping_add(2) as u64;
                            j_2 = (j_2 as u64).wrapping_add(2) as u64;
                        }
                    }
                    while i != length {
                        *recon.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            + paethPredictor(
                                *recon.offset(i.wrapping_sub(bytewidth) as isize) as i16,
                                *precon.offset(i as isize) as i16,
                                *precon.offset(j_2 as isize) as i16,
                            ) as i32) as u8;
                        i = i.wrapping_add(1);
                        j_2 = j_2.wrapping_add(1);
                    }
                } else {
                    let mut j_3 = 0;
                    i = 0;
                    while i != bytewidth {
                        *recon.offset(i as isize) = *scanline.offset(i as isize);
                        i = i.wrapping_add(1);
                    }
                    i = bytewidth;
                    while i != length {
                        *recon.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            + *recon.offset(j_3 as isize) as i32)
                            as u8;
                        i = i.wrapping_add(1);
                        j_3 = j_3.wrapping_add(1);
                    }
                }
            }
            _ => return 36,
        }
        return 0;
    }
}

extern "C" fn unfilter(
    mut out: *mut u8,
    mut in_0: *const u8,
    mut w: u32,
    mut h: u32,
    mut bpp: u32,
) -> u32 {
    unsafe {
        let mut y: u32 = 0;
        let mut prevline = 0 as *mut u8;
        let mut bytewidth = bpp.wrapping_add(7).wrapping_div(8) as u64;
        let mut linebytes = (lodepng_get_raw_size_idat(w, 1u32, bpp)).wrapping_sub(1);
        y = 0;
        while y < h {
            let mut outindex = linebytes.wrapping_mul(y as u64);
            let mut inindex = 1u64.wrapping_add(linebytes).wrapping_mul(y as u64);
            let mut filterType = *in_0.offset(inindex as isize);
            let mut error = unfilterScanline(
                &mut *out.offset(outindex as isize),
                &*in_0.offset(inindex.wrapping_add(1) as isize),
                prevline,
                bytewidth,
                filterType,
                linebytes,
            );
            if error != 0 {
                return error;
            }
            prevline = &mut *out.offset(outindex as isize) as *mut u8;
            y = y.wrapping_add(1);
        }
        return 0;
    }
}

extern "C" fn Adam7_deinterlace(
    mut out: *mut u8,
    mut in_0: *const u8,
    mut w: u32,
    mut h: u32,
    mut bpp: u32,
) {
    unsafe {
        let mut passw: [u32; 7] = [0; 7];
        let mut passh: [u32; 7] = [0; 7];
        let mut filter_passstart: [u64; 8] = [0; 8];
        let mut padded_passstart: [u64; 8] = [0; 8];
        let mut passstart: [u64; 8] = [0; 8];
        let mut i: u32 = 0;
        Adam7_getpassvalues(
            passw.as_mut_ptr(),
            passh.as_mut_ptr(),
            filter_passstart.as_mut_ptr(),
            padded_passstart.as_mut_ptr(),
            passstart.as_mut_ptr(),
            w,
            h,
            bpp,
        );
        if bpp >= 8 {
            i = 0;
            while i != 7 {
                let mut x: u32 = 0;
                let mut y: u32 = 0;
                let mut b: u32 = 0;
                let mut bytewidth = bpp.wrapping_div(8) as u64;
                y = 0;
                while y < passh[i as usize] {
                    x = 0;
                    while x < passw[i as usize] {
                        let mut pixelinstart = (passstart[i as usize]).wrapping_add(
                            (y.wrapping_mul(passw[i as usize]).wrapping_add(x) as u64)
                                .wrapping_mul(bytewidth),
                        );
                        let mut pixeloutstart = (ADAM7_IY[i as usize] as u64)
                            .wrapping_add((y as u64).wrapping_mul(ADAM7_DY[i as usize] as u64))
                            .wrapping_mul(w as u64)
                            .wrapping_add(ADAM7_IX[i as usize] as u64)
                            .wrapping_add((x as u64).wrapping_mul(ADAM7_DX[i as usize] as u64))
                            .wrapping_mul(bytewidth);
                        b = 0;
                        while (b as u64) < bytewidth {
                            *out.offset(pixeloutstart.wrapping_add(b as u64) as isize) =
                                *in_0.offset(pixelinstart.wrapping_add(b as u64) as isize);
                            b = b.wrapping_add(1);
                        }
                        x = x.wrapping_add(1);
                    }
                    y = y.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
        } else {
            i = 0;
            while i != 7 {
                let mut x_0: u32 = 0;
                let mut y_0: u32 = 0;
                let mut b_0: u32 = 0;
                let mut ilinebits = bpp.wrapping_mul(passw[i as usize]);
                let mut olinebits = bpp.wrapping_mul(w);
                let mut obp: u64 = 0;
                let mut ibp: u64 = 0;
                y_0 = 0;
                while y_0 < passh[i as usize] {
                    x_0 = 0;
                    while x_0 < passw[i as usize] {
                        ibp = 8u64.wrapping_mul(passstart[i as usize]).wrapping_add(
                            y_0.wrapping_mul(ilinebits)
                                .wrapping_add(x_0.wrapping_mul(bpp))
                                as u64,
                        );
                        obp = (ADAM7_IY[i as usize] as u64)
                            .wrapping_add((y_0 as u64).wrapping_mul(ADAM7_DY[i as usize] as u64))
                            .wrapping_mul(olinebits as u64)
                            .wrapping_add(
                                (ADAM7_IX[i as usize] as u64)
                                    .wrapping_add(
                                        (x_0 as u64).wrapping_mul(ADAM7_DX[i as usize] as u64),
                                    )
                                    .wrapping_mul(bpp as u64),
                            );
                        b_0 = 0;
                        while b_0 < bpp {
                            let mut bit = readBitFromReversedStream(&mut ibp, in_0);
                            setBitOfReversedStream(&mut obp, out, bit);
                            b_0 = b_0.wrapping_add(1);
                        }
                        x_0 = x_0.wrapping_add(1);
                    }
                    y_0 = y_0.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
        };
    }
}

extern "C" fn removePaddingBits(
    mut out: *mut u8,
    mut in_0: *const u8,
    mut olinebits: u64,
    mut ilinebits: u64,
    mut h: u32,
) {
    unsafe {
        let mut y: u32 = 0;
        let mut diff = ilinebits.wrapping_sub(olinebits);
        let mut ibp = 0;
        let mut obp = 0;
        y = 0;
        while y < h {
            let mut x: u64 = 0;
            x = 0;
            while x < olinebits {
                let mut bit = readBitFromReversedStream(&mut ibp, in_0);
                setBitOfReversedStream(&mut obp, out, bit);
                x = x.wrapping_add(1);
            }
            ibp = (ibp as u64).wrapping_add(diff) as u64;
            y = y.wrapping_add(1);
        }
    }
}

extern "C" fn postProcessScanlines(
    mut out: *mut u8,
    mut in_0: *mut u8,
    mut w: u32,
    mut h: u32,
    mut info_png: *const LodePNGInfo,
) -> u32 {
    unsafe {
        let mut bpp = lodepng_get_bpp(&(*info_png).color);
        if bpp == 0 {
            return 31;
        }
        if (*info_png).interlace_method == 0 {
            if bpp < 8
                && w.wrapping_mul(bpp)
                    != w.wrapping_mul(bpp)
                        .wrapping_add(7)
                        .wrapping_div(8)
                        .wrapping_mul(8)
            {
                let mut error = unfilter(in_0, in_0, w, h, bpp);
                if error != 0 {
                    return error;
                }
                removePaddingBits(
                    out,
                    in_0,
                    w.wrapping_mul(bpp) as u64,
                    w.wrapping_mul(bpp)
                        .wrapping_add(7)
                        .wrapping_div(8)
                        .wrapping_mul(8) as u64,
                    h,
                );
            } else {
                let mut error_0 = unfilter(out, in_0, w, h, bpp);
                if error_0 != 0 {
                    return error_0;
                }
            }
        } else {
            let mut passw: [u32; 7] = [0; 7];
            let mut passh: [u32; 7] = [0; 7];
            let mut filter_passstart: [u64; 8] = [0; 8];
            let mut padded_passstart: [u64; 8] = [0; 8];
            let mut passstart: [u64; 8] = [0; 8];
            let mut i: u32 = 0;
            Adam7_getpassvalues(
                passw.as_mut_ptr(),
                passh.as_mut_ptr(),
                filter_passstart.as_mut_ptr(),
                padded_passstart.as_mut_ptr(),
                passstart.as_mut_ptr(),
                w,
                h,
                bpp,
            );
            i = 0;
            while i != 7 {
                let mut error_1 = unfilter(
                    &mut *in_0.offset(*padded_passstart.as_mut_ptr().offset(i as isize) as isize),
                    &mut *in_0.offset(*filter_passstart.as_mut_ptr().offset(i as isize) as isize),
                    passw[i as usize],
                    passh[i as usize],
                    bpp,
                );
                if error_1 != 0 {
                    return error_1;
                }
                if bpp < 8 {
                    removePaddingBits(
                        &mut *in_0.offset(*passstart.as_mut_ptr().offset(i as isize) as isize),
                        &mut *in_0
                            .offset(*padded_passstart.as_mut_ptr().offset(i as isize) as isize),
                        (passw[i as usize]).wrapping_mul(bpp) as u64,
                        (passw[i as usize])
                            .wrapping_mul(bpp)
                            .wrapping_add(7)
                            .wrapping_div(8)
                            .wrapping_mul(8) as u64,
                        passh[i as usize],
                    );
                }
                i = i.wrapping_add(1);
            }
            Adam7_deinterlace(out, in_0, w, h, bpp);
        }
        return 0;
    }
}

extern "C" fn readChunk_PLTE(
    mut color: *mut LodePNGColorMode,
    mut data: *const u8,
    mut chunkLength: u64,
) -> u32 {
    unsafe {
        let mut pos = 0;
        let mut i: u32 = 0;
        (*color).palettesize = chunkLength.wrapping_div(3);
        if (*color).palettesize == 0 || (*color).palettesize > 256 {
            return 38;
        }
        lodepng_color_mode_alloc_palette(color);
        if ((*color).palette).is_null() && (*color).palettesize != 0 {
            (*color).palettesize = 0;
            return 83;
        }
        i = 0;
        while i as u64 != (*color).palettesize {
            let fresh168 = pos;
            pos = pos.wrapping_add(1);
            *((*color).palette).offset(4u32.wrapping_mul(i).wrapping_add(0) as isize) =
                *data.offset(fresh168 as isize);
            let fresh169 = pos;
            pos = pos.wrapping_add(1);
            *((*color).palette).offset(4u32.wrapping_mul(i).wrapping_add(1) as isize) =
                *data.offset(fresh169 as isize);
            let fresh170 = pos;
            pos = pos.wrapping_add(1);
            *((*color).palette).offset(4u32.wrapping_mul(i).wrapping_add(2) as isize) =
                *data.offset(fresh170 as isize);
            *((*color).palette).offset(4u32.wrapping_mul(i).wrapping_add(3) as isize) = 255;
            i = i.wrapping_add(1);
        }
        return 0;
    }
}

extern "C" fn readChunk_tRNS(
    mut color: *mut LodePNGColorMode,
    mut data: *const u8,
    mut chunkLength: u64,
) -> u32 {
    unsafe {
        let mut i: u32 = 0;
        if (*color).colortype as u32 == LCT_PALETTE as u32 {
            if chunkLength > (*color).palettesize {
                return 39;
            }
            i = 0;
            while i as u64 != chunkLength {
                *((*color).palette).offset(4u32.wrapping_mul(i).wrapping_add(3) as isize) =
                    *data.offset(i as isize);
                i = i.wrapping_add(1);
            }
        } else if (*color).colortype as u32 == LCT_GREY as u32 {
            if chunkLength != 2 {
                return 30;
            };
            (*color).key_defined = 1;
            let ref mut fresh171 = (*color).key_b;
            *fresh171 = 256u32
                .wrapping_mul(*data.offset(0 as isize) as u32)
                .wrapping_add(*data.offset(1 as isize) as u32);
            let ref mut fresh172 = (*color).key_g;
            *fresh172 = *fresh171;
            (*color).key_r = *fresh172;
        } else if (*color).colortype as u32 == LCT_RGB as u32 {
            if chunkLength != 6 {
                return 41;
            };
            (*color).key_defined = 1;
            (*color).key_r = 256u32
                .wrapping_mul(*data.offset(0 as isize) as u32)
                .wrapping_add(*data.offset(1 as isize) as u32);
            (*color).key_g = 256u32
                .wrapping_mul(*data.offset(2 as isize) as u32)
                .wrapping_add(*data.offset(3 as isize) as u32);
            (*color).key_b = 256u32
                .wrapping_mul(*data.offset(4 as isize) as u32)
                .wrapping_add(*data.offset(5 as isize) as u32);
        } else {
            return 42;
        }
        return 0;
    }
}

extern "C" fn readChunk_bKGD(
    mut info: *mut LodePNGInfo,
    mut data: *const u8,
    mut chunkLength: u64,
) -> u32 {
    unsafe {
        if (*info).color.colortype as u32 == LCT_PALETTE as u32 {
            if chunkLength != 1 {
                return 43;
            }
            if *data.offset(0 as isize) as u64 >= (*info).color.palettesize {
                return 103;
            };
            (*info).background_defined = 1;
            let ref mut fresh173 = (*info).background_b;
            *fresh173 = *data.offset(0 as isize) as u32;
            let ref mut fresh174 = (*info).background_g;
            *fresh174 = *fresh173;
            (*info).background_r = *fresh174;
        } else if (*info).color.colortype as u32 == LCT_GREY as u32
            || (*info).color.colortype as u32 == LCT_GREY_ALPHA as u32
        {
            if chunkLength != 2 {
                return 44;
            };
            (*info).background_defined = 1;
            let ref mut fresh175 = (*info).background_b;
            *fresh175 = 256u32
                .wrapping_mul(*data.offset(0 as isize) as u32)
                .wrapping_add(*data.offset(1 as isize) as u32);
            let ref mut fresh176 = (*info).background_g;
            *fresh176 = *fresh175;
            (*info).background_r = *fresh176;
        } else if (*info).color.colortype as u32 == LCT_RGB as u32
            || (*info).color.colortype as u32 == LCT_RGBA as u32
        {
            if chunkLength != 6 {
                return 45;
            };
            (*info).background_defined = 1;
            (*info).background_r = 256u32
                .wrapping_mul(*data.offset(0 as isize) as u32)
                .wrapping_add(*data.offset(1 as isize) as u32);
            (*info).background_g = 256u32
                .wrapping_mul(*data.offset(2 as isize) as u32)
                .wrapping_add(*data.offset(3 as isize) as u32);
            (*info).background_b = 256u32
                .wrapping_mul(*data.offset(4 as isize) as u32)
                .wrapping_add(*data.offset(5 as isize) as u32);
        }
        return 0;
    }
}

extern "C" fn readChunk_tEXt(
    mut info: *mut LodePNGInfo,
    mut data: *const u8,
    mut chunkLength: u64,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut key = 0 as *mut i8;
        let mut str = 0 as *mut i8;
        if error == 0 {
            let mut length: u32 = 0;
            let mut string2_begin: u32 = 0;
            length = 0;
            while (length as u64) < chunkLength && *data.offset(length as isize) as i32 != 0 {
                length = length.wrapping_add(1);
            }
            if length < 1 || length > 79 {
                error = 89;
            } else {
                key = lodepng_malloc(length.wrapping_add(1) as u64) as *mut i8;
                if key.is_null() {
                    error = 83;
                } else {
                    lodepng_memcpy(
                        key as *mut libc::c_void,
                        data as *const libc::c_void,
                        length as u64,
                    );
                    *key.offset(length as isize) = 0;
                    string2_begin = length.wrapping_add(1);
                    length = (if chunkLength < string2_begin as u64 {
                        0
                    } else {
                        chunkLength.wrapping_sub(string2_begin as u64)
                    }) as u32;
                    str = lodepng_malloc(length.wrapping_add(1) as u64) as *mut i8;
                    if str.is_null() {
                        error = 83;
                    } else {
                        lodepng_memcpy(
                            str as *mut libc::c_void,
                            data.offset(string2_begin as isize) as *const libc::c_void,
                            length as u64,
                        );
                        *str.offset(length as isize) = 0;
                        error = lodepng_add_text(info, key, str);
                    }
                }
            }
        }
        lodepng_free(key as *mut libc::c_void);
        lodepng_free(str as *mut libc::c_void);
        return error;
    }
}

extern "C" fn readChunk_zTXt(
    mut info: *mut LodePNGInfo,
    mut decoder: *const LodePNGDecoderSettings,
    mut data: *const u8,
    mut chunkLength: u64,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut zlibsettings = (*decoder).zlibsettings;
        let mut length: u32 = 0;
        let mut string2_begin: u32 = 0;
        let mut key = 0 as *mut i8;
        let mut str = 0 as *mut u8;
        let mut size = 0;
        if error == 0 {
            length = 0;
            while (length as u64) < chunkLength && *data.offset(length as isize) as i32 != 0 {
                length = length.wrapping_add(1);
            }
            if length.wrapping_add(2) as u64 >= chunkLength {
                error = 75;
            } else if length < 1 || length > 79 {
                error = 89;
            } else {
                key = lodepng_malloc(length.wrapping_add(1) as u64) as *mut i8;
                if key.is_null() {
                    error = 83;
                } else {
                    lodepng_memcpy(
                        key as *mut libc::c_void,
                        data as *const libc::c_void,
                        length as u64,
                    );
                    *key.offset(length as isize) = 0;
                    if *data.offset(length.wrapping_add(1) as isize) as i32 != 0 {
                        error = 72;
                    } else {
                        string2_begin = length.wrapping_add(2);
                        if string2_begin as u64 > chunkLength {
                            error = 75;
                        } else {
                            length = (chunkLength as u32).wrapping_sub(string2_begin);
                            zlibsettings.max_output_size = (*decoder).max_text_size;
                            error = zlib_decompress(
                                &mut str,
                                &mut size,
                                0,
                                &*data.offset(string2_begin as isize),
                                length as u64,
                                &mut zlibsettings,
                            );
                            if error != 0 && size > zlibsettings.max_output_size {
                                error = 112;
                            }
                            if !(error != 0) {
                                error = lodepng_add_text_sized(info, key, str as *mut i8, size);
                            }
                        }
                    }
                }
            }
        }
        lodepng_free(key as *mut libc::c_void);
        lodepng_free(str as *mut libc::c_void);
        return error;
    }
}

extern "C" fn readChunk_iTXt(
    mut info: *mut LodePNGInfo,
    mut decoder: *const LodePNGDecoderSettings,
    mut data: *const u8,
    mut chunkLength: u64,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut i: u32 = 0;
        let mut zlibsettings = (*decoder).zlibsettings;
        let mut length: u32 = 0;
        let mut begin: u32 = 0;
        let mut compressed: u32 = 0;
        let mut key = 0 as *mut i8;
        let mut langtag = 0 as *mut i8;
        let mut transkey = 0 as *mut i8;
        if error == 0 {
            if chunkLength < 5 {
                error = 30;
            } else {
                length = 0;
                while (length as u64) < chunkLength && *data.offset(length as isize) as i32 != 0 {
                    length = length.wrapping_add(1);
                }
                if length.wrapping_add(3) as u64 >= chunkLength {
                    error = 75;
                } else if length < 1 || length > 79 {
                    error = 89;
                } else {
                    key = lodepng_malloc(length.wrapping_add(1) as u64) as *mut i8;
                    if key.is_null() {
                        error = 83;
                    } else {
                        lodepng_memcpy(
                            key as *mut libc::c_void,
                            data as *const libc::c_void,
                            length as u64,
                        );
                        *key.offset(length as isize) = 0;
                        compressed = *data.offset(length.wrapping_add(1) as isize) as u32;
                        if *data.offset(length.wrapping_add(2) as isize) as i32 != 0 {
                            error = 72;
                        } else {
                            begin = length.wrapping_add(3);
                            length = 0;
                            i = begin;
                            while (i as u64) < chunkLength && *data.offset(i as isize) as i32 != 0 {
                                length = length.wrapping_add(1);
                                i = i.wrapping_add(1);
                            }
                            langtag = lodepng_malloc(length.wrapping_add(1) as u64) as *mut i8;
                            if langtag.is_null() {
                                error = 83;
                            } else {
                                lodepng_memcpy(
                                    langtag as *mut libc::c_void,
                                    data.offset(begin as isize) as *const libc::c_void,
                                    length as u64,
                                );
                                *langtag.offset(length as isize) = 0;
                                begin = begin.wrapping_add(length.wrapping_add(1));
                                length = 0;
                                i = begin;
                                while (i as u64) < chunkLength
                                    && *data.offset(i as isize) as i32 != 0
                                {
                                    length = length.wrapping_add(1);
                                    i = i.wrapping_add(1);
                                }
                                transkey = lodepng_malloc(length.wrapping_add(1) as u64) as *mut i8;
                                if transkey.is_null() {
                                    error = 83;
                                } else {
                                    lodepng_memcpy(
                                        transkey as *mut libc::c_void,
                                        data.offset(begin as isize) as *const libc::c_void,
                                        length as u64,
                                    );
                                    *transkey.offset(length as isize) = 0;
                                    begin = begin.wrapping_add(length.wrapping_add(1));
                                    length = if (chunkLength as u32) < begin {
                                        0
                                    } else {
                                        (chunkLength as u32).wrapping_sub(begin)
                                    };
                                    if compressed != 0 {
                                        let mut str = 0 as *mut u8;
                                        let mut size = 0;
                                        zlibsettings.max_output_size = (*decoder).max_text_size;
                                        error = zlib_decompress(
                                            &mut str,
                                            &mut size,
                                            0,
                                            &*data.offset(begin as isize),
                                            length as u64,
                                            &mut zlibsettings,
                                        );
                                        if error != 0 && size > zlibsettings.max_output_size {
                                            error = 112;
                                        }
                                        if error == 0 {
                                            error = lodepng_add_itext_sized(
                                                info,
                                                key,
                                                langtag,
                                                transkey,
                                                str as *mut i8,
                                                size,
                                            );
                                        }
                                        lodepng_free(str as *mut libc::c_void);
                                    } else {
                                        error = lodepng_add_itext_sized(
                                            info,
                                            key,
                                            langtag,
                                            transkey,
                                            data.offset(begin as isize) as *mut i8,
                                            length as u64,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        lodepng_free(key as *mut libc::c_void);
        lodepng_free(langtag as *mut libc::c_void);
        lodepng_free(transkey as *mut libc::c_void);
        return error;
    }
}

extern "C" fn readChunk_tIME(
    mut info: *mut LodePNGInfo,
    mut data: *const u8,
    mut chunkLength: u64,
) -> u32 {
    unsafe {
        if chunkLength != 7 {
            return 73;
        };
        (*info).time_defined = 1;
        (*info).time.year = 256u32
            .wrapping_mul(*data.offset(0 as isize) as u32)
            .wrapping_add(*data.offset(1 as isize) as u32);
        (*info).time.month = *data.offset(2 as isize) as u32;
        (*info).time.day = *data.offset(3 as isize) as u32;
        (*info).time.hour = *data.offset(4 as isize) as u32;
        (*info).time.minute = *data.offset(5 as isize) as u32;
        (*info).time.second = *data.offset(6 as isize) as u32;
        return 0;
    }
}

extern "C" fn readChunk_pHYs(
    mut info: *mut LodePNGInfo,
    mut data: *const u8,
    mut chunkLength: u64,
) -> u32 {
    unsafe {
        if chunkLength != 9 {
            return 74;
        };
        (*info).phys_defined = 1;
        (*info).phys_x = 16777216u32
            .wrapping_mul(*data.offset(0 as isize) as u32)
            .wrapping_add(65536u32.wrapping_mul(*data.offset(1 as isize) as u32))
            .wrapping_add(256u32.wrapping_mul(*data.offset(2 as isize) as u32))
            .wrapping_add(*data.offset(3 as isize) as u32);
        (*info).phys_y = 16777216u32
            .wrapping_mul(*data.offset(4 as isize) as u32)
            .wrapping_add(65536u32.wrapping_mul(*data.offset(5 as isize) as u32))
            .wrapping_add(256u32.wrapping_mul(*data.offset(6 as isize) as u32))
            .wrapping_add(*data.offset(7 as isize) as u32);
        (*info).phys_unit = *data.offset(8 as isize) as u32;
        return 0;
    }
}

extern "C" fn readChunk_gAMA(
    mut info: *mut LodePNGInfo,
    mut data: *const u8,
    mut chunkLength: u64,
) -> u32 {
    unsafe {
        if chunkLength != 4 {
            return 96;
        };
        (*info).gama_defined = 1;
        (*info).gama_gamma = 16777216u32
            .wrapping_mul(*data.offset(0 as isize) as u32)
            .wrapping_add(65536u32.wrapping_mul(*data.offset(1 as isize) as u32))
            .wrapping_add(256u32.wrapping_mul(*data.offset(2 as isize) as u32))
            .wrapping_add(*data.offset(3 as isize) as u32);
        return 0;
    }
}

extern "C" fn readChunk_cHRM(
    mut info: *mut LodePNGInfo,
    mut data: *const u8,
    mut chunkLength: u64,
) -> u32 {
    unsafe {
        if chunkLength != 32 {
            return 97;
        };
        (*info).chrm_defined = 1;
        (*info).chrm_white_x = 16777216u32
            .wrapping_mul(*data.offset(0 as isize) as u32)
            .wrapping_add(65536u32.wrapping_mul(*data.offset(1 as isize) as u32))
            .wrapping_add(256u32.wrapping_mul(*data.offset(2 as isize) as u32))
            .wrapping_add(*data.offset(3 as isize) as u32);
        (*info).chrm_white_y = 16777216u32
            .wrapping_mul(*data.offset(4 as isize) as u32)
            .wrapping_add(65536u32.wrapping_mul(*data.offset(5 as isize) as u32))
            .wrapping_add(256u32.wrapping_mul(*data.offset(6 as isize) as u32))
            .wrapping_add(*data.offset(7 as isize) as u32);
        (*info).chrm_red_x = 16777216u32
            .wrapping_mul(*data.offset(8 as isize) as u32)
            .wrapping_add(65536u32.wrapping_mul(*data.offset(9 as isize) as u32))
            .wrapping_add(256u32.wrapping_mul(*data.offset(10 as isize) as u32))
            .wrapping_add(*data.offset(11 as isize) as u32);
        (*info).chrm_red_y = 16777216u32
            .wrapping_mul(*data.offset(12 as isize) as u32)
            .wrapping_add(65536u32.wrapping_mul(*data.offset(13 as isize) as u32))
            .wrapping_add(256u32.wrapping_mul(*data.offset(14 as isize) as u32))
            .wrapping_add(*data.offset(15 as isize) as u32);
        (*info).chrm_green_x = 16777216u32
            .wrapping_mul(*data.offset(16 as isize) as u32)
            .wrapping_add(65536u32.wrapping_mul(*data.offset(17 as isize) as u32))
            .wrapping_add(256u32.wrapping_mul(*data.offset(18 as isize) as u32))
            .wrapping_add(*data.offset(19 as isize) as u32);
        (*info).chrm_green_y = 16777216u32
            .wrapping_mul(*data.offset(20 as isize) as u32)
            .wrapping_add(65536u32.wrapping_mul(*data.offset(21 as isize) as u32))
            .wrapping_add(256u32.wrapping_mul(*data.offset(22 as isize) as u32))
            .wrapping_add(*data.offset(23 as isize) as u32);
        (*info).chrm_blue_x = 16777216u32
            .wrapping_mul(*data.offset(24 as isize) as u32)
            .wrapping_add(65536u32.wrapping_mul(*data.offset(25 as isize) as u32))
            .wrapping_add(256u32.wrapping_mul(*data.offset(26 as isize) as u32))
            .wrapping_add(*data.offset(27 as isize) as u32);
        (*info).chrm_blue_y = 16777216u32
            .wrapping_mul(*data.offset(28 as isize) as u32)
            .wrapping_add(65536u32.wrapping_mul(*data.offset(29 as isize) as u32))
            .wrapping_add(256u32.wrapping_mul(*data.offset(30 as isize) as u32))
            .wrapping_add(*data.offset(31 as isize) as u32);
        return 0;
    }
}

extern "C" fn readChunk_sRGB(
    mut info: *mut LodePNGInfo,
    mut data: *const u8,
    mut chunkLength: u64,
) -> u32 {
    unsafe {
        if chunkLength != 1 {
            return 98;
        };
        (*info).srgb_defined = 1;
        (*info).srgb_intent = *data.offset(0 as isize) as u32;
        return 0;
    }
}

extern "C" fn readChunk_iCCP(
    mut info: *mut LodePNGInfo,
    mut decoder: *const LodePNGDecoderSettings,
    mut data: *const u8,
    mut chunkLength: u64,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut i: u32 = 0;
        let mut size = 0;
        let mut zlibsettings = (*decoder).zlibsettings;
        let mut length: u32 = 0;
        let mut string2_begin: u32 = 0;
        (*info).iccp_defined = 1;
        if !((*info).iccp_name).is_null() {
            lodepng_clear_icc(info);
        }
        length = 0;
        while (length as u64) < chunkLength && *data.offset(length as isize) as i32 != 0 {
            length = length.wrapping_add(1);
        }
        if length.wrapping_add(2) as u64 >= chunkLength {
            return 75;
        }
        if length < 1 || length > 79 {
            return 89;
        }
        let ref mut fresh177 = (*info).iccp_name;
        *fresh177 = lodepng_malloc(length.wrapping_add(1) as u64) as *mut i8;
        if ((*info).iccp_name).is_null() {
            return 83;
        };
        *((*info).iccp_name).offset(length as isize) = 0;
        i = 0;
        while i != length {
            *((*info).iccp_name).offset(i as isize) = *data.offset(i as isize) as i8;
            i = i.wrapping_add(1);
        }
        if *data.offset(length.wrapping_add(1) as isize) as i32 != 0 {
            return 72;
        }
        string2_begin = length.wrapping_add(2);
        if string2_begin as u64 > chunkLength {
            return 75;
        }
        length = (chunkLength as u32).wrapping_sub(string2_begin);
        zlibsettings.max_output_size = (*decoder).max_icc_size;
        error = zlib_decompress(
            &mut (*info).iccp_profile,
            &mut size,
            0,
            &*data.offset(string2_begin as isize),
            length as u64,
            &mut zlibsettings,
        );
        if error != 0 && size > zlibsettings.max_output_size {
            error = 113;
        };
        (*info).iccp_profile_size = size as u32;
        if error == 0 && (*info).iccp_profile_size == 0 {
            error = 100;
        }
        return error;
    }
}

extern "C" fn readChunk_sBIT(
    mut info: *mut LodePNGInfo,
    mut data: *const u8,
    mut chunkLength: u64,
) -> u32 {
    unsafe {
        let mut bitdepth = if (*info).color.colortype as u32 == LCT_PALETTE as u32 {
            8
        } else {
            (*info).color.bitdepth
        };
        if (*info).color.colortype as u32 == LCT_GREY as u32 {
            if chunkLength != 1 {
                return 114;
            }
            if *data.offset(0 as isize) as i32 == 0 || *data.offset(0 as isize) as u32 > bitdepth {
                return 115;
            };
            (*info).sbit_defined = 1;
            let ref mut fresh178 = (*info).sbit_b;
            *fresh178 = *data.offset(0 as isize) as u32;
            let ref mut fresh179 = (*info).sbit_g;
            *fresh179 = *fresh178;
            (*info).sbit_r = *fresh179;
        } else if (*info).color.colortype as u32 == LCT_RGB as u32
            || (*info).color.colortype as u32 == LCT_PALETTE as u32
        {
            if chunkLength != 3 {
                return 114;
            }
            if *data.offset(0 as isize) as i32 == 0
                || *data.offset(1 as isize) as i32 == 0
                || *data.offset(2 as isize) as i32 == 0
            {
                return 115;
            }
            if *data.offset(0 as isize) as u32 > bitdepth
                || *data.offset(1 as isize) as u32 > bitdepth
                || *data.offset(2 as isize) as u32 > bitdepth
            {
                return 115;
            };
            (*info).sbit_defined = 1;
            (*info).sbit_r = *data.offset(0 as isize) as u32;
            (*info).sbit_g = *data.offset(1 as isize) as u32;
            (*info).sbit_b = *data.offset(2 as isize) as u32;
        } else if (*info).color.colortype as u32 == LCT_GREY_ALPHA as u32 {
            if chunkLength != 2 {
                return 114;
            }
            if *data.offset(0 as isize) as i32 == 0 || *data.offset(1 as isize) as i32 == 0 {
                return 115;
            }
            if *data.offset(0 as isize) as u32 > bitdepth
                || *data.offset(1 as isize) as u32 > bitdepth
            {
                return 115;
            };
            (*info).sbit_defined = 1;
            let ref mut fresh180 = (*info).sbit_b;
            *fresh180 = *data.offset(0 as isize) as u32;
            let ref mut fresh181 = (*info).sbit_g;
            *fresh181 = *fresh180;
            (*info).sbit_r = *fresh181;
            (*info).sbit_a = *data.offset(1 as isize) as u32;
        } else if (*info).color.colortype as u32 == LCT_RGBA as u32 {
            if chunkLength != 4 {
                return 114;
            }
            if *data.offset(0 as isize) as i32 == 0
                || *data.offset(1 as isize) as i32 == 0
                || *data.offset(2 as isize) as i32 == 0
                || *data.offset(3 as isize) as i32 == 0
            {
                return 115;
            }
            if *data.offset(0 as isize) as u32 > bitdepth
                || *data.offset(1 as isize) as u32 > bitdepth
                || *data.offset(2 as isize) as u32 > bitdepth
                || *data.offset(3 as isize) as u32 > bitdepth
            {
                return 115;
            };
            (*info).sbit_defined = 1;
            (*info).sbit_r = *data.offset(0 as isize) as u32;
            (*info).sbit_g = *data.offset(1 as isize) as u32;
            (*info).sbit_b = *data.offset(2 as isize) as u32;
            (*info).sbit_a = *data.offset(3 as isize) as u32;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_inspect_chunk(
    mut state: *mut LodePNGState,
    mut pos: u64,
    mut in_0: *const u8,
    mut insize: u64,
) -> u32 {
    unsafe {
        let mut chunk = in_0.offset(pos as isize);
        let mut chunkLength: u32 = 0;
        let mut data = 0 as *const u8;
        let mut unhandled = 0;
        let mut error = 0;
        if pos.wrapping_add(4) > insize {
            return 30;
        }
        chunkLength = lodepng_chunk_length(chunk);
        if chunkLength > 2147483647 {
            return 63;
        }
        data = lodepng_chunk_data_const(chunk);
        if chunkLength.wrapping_add(12) as u64 > insize.wrapping_sub(pos) {
            return 30;
        }
        if lodepng_chunk_type_equals(chunk, b"PLTE\0" as *const u8 as *const i8) != 0 {
            error = readChunk_PLTE(&mut (*state).info_png.color, data, chunkLength as u64);
        } else if lodepng_chunk_type_equals(chunk, b"tRNS\0" as *const u8 as *const i8) != 0 {
            error = readChunk_tRNS(&mut (*state).info_png.color, data, chunkLength as u64);
        } else if lodepng_chunk_type_equals(chunk, b"bKGD\0" as *const u8 as *const i8) != 0 {
            error = readChunk_bKGD(&mut (*state).info_png, data, chunkLength as u64);
        } else if lodepng_chunk_type_equals(chunk, b"tEXt\0" as *const u8 as *const i8) != 0 {
            error = readChunk_tEXt(&mut (*state).info_png, data, chunkLength as u64);
        } else if lodepng_chunk_type_equals(chunk, b"zTXt\0" as *const u8 as *const i8) != 0 {
            error = readChunk_zTXt(
                &mut (*state).info_png,
                &mut (*state).decoder,
                data,
                chunkLength as u64,
            );
        } else if lodepng_chunk_type_equals(chunk, b"iTXt\0" as *const u8 as *const i8) != 0 {
            error = readChunk_iTXt(
                &mut (*state).info_png,
                &mut (*state).decoder,
                data,
                chunkLength as u64,
            );
        } else if lodepng_chunk_type_equals(chunk, b"tIME\0" as *const u8 as *const i8) != 0 {
            error = readChunk_tIME(&mut (*state).info_png, data, chunkLength as u64);
        } else if lodepng_chunk_type_equals(chunk, b"pHYs\0" as *const u8 as *const i8) != 0 {
            error = readChunk_pHYs(&mut (*state).info_png, data, chunkLength as u64);
        } else if lodepng_chunk_type_equals(chunk, b"gAMA\0" as *const u8 as *const i8) != 0 {
            error = readChunk_gAMA(&mut (*state).info_png, data, chunkLength as u64);
        } else if lodepng_chunk_type_equals(chunk, b"cHRM\0" as *const u8 as *const i8) != 0 {
            error = readChunk_cHRM(&mut (*state).info_png, data, chunkLength as u64);
        } else if lodepng_chunk_type_equals(chunk, b"sRGB\0" as *const u8 as *const i8) != 0 {
            error = readChunk_sRGB(&mut (*state).info_png, data, chunkLength as u64);
        } else if lodepng_chunk_type_equals(chunk, b"iCCP\0" as *const u8 as *const i8) != 0 {
            error = readChunk_iCCP(
                &mut (*state).info_png,
                &mut (*state).decoder,
                data,
                chunkLength as u64,
            );
        } else if lodepng_chunk_type_equals(chunk, b"sBIT\0" as *const u8 as *const i8) != 0 {
            error = readChunk_sBIT(&mut (*state).info_png, data, chunkLength as u64);
        } else {
            unhandled = 1;
        }
        if error == 0 && unhandled == 0 && (*state).decoder.ignore_crc == 0 {
            if lodepng_chunk_check_crc(chunk) != 0 {
                return 57;
            }
        }
        return error;
    }
}

extern "C" fn decodeGeneric(
    mut out: *mut *mut u8,
    mut w: *mut u32,
    mut h: *mut u32,
    mut state: *mut LodePNGState,
    mut in_0: *const u8,
    mut insize: u64,
) {
    unsafe {
        let mut IEND = 0;
        let mut chunk = 0 as *const u8;
        let mut idat = 0 as *mut u8;
        let mut idatsize = 0;
        let mut scanlines = 0 as *mut u8;
        let mut scanlines_size = 0;
        let mut expected_size = 0;
        let mut outsize = 0;
        let mut unknown = 0;
        let mut critical_pos = 1;
        *out = 0 as *mut u8;
        *h = 0;
        *w = *h;
        (*state).error = lodepng_inspect(w, h, state, in_0, insize);
        if (*state).error != 0 {
            return;
        }
        if lodepng_pixel_overflow(*w, *h, &mut (*state).info_png.color, &mut (*state).info_raw) != 0
        {
            (*state).error = 92;
            return;
        }
        idat = lodepng_malloc(insize) as *mut u8;
        if idat.is_null() {
            (*state).error = 83;
            return;
        }
        chunk = &*in_0.offset(33 as isize) as *const u8;
        while IEND == 0 && (*state).error == 0 {
            let mut chunkLength: u32 = 0;
            let mut data = 0 as *const u8;
            let mut pos = chunk.offset_from(in_0) as u64;
            if chunk < in_0 || pos.wrapping_add(12) > insize {
                if (*state).decoder.ignore_end != 0 {
                    break;
                };
                (*state).error = 30;
                break;
            } else {
                chunkLength = lodepng_chunk_length(chunk);
                if chunkLength > 2147483647 {
                    if (*state).decoder.ignore_end != 0 {
                        break;
                    };
                    (*state).error = 63;
                    break;
                } else if pos.wrapping_add(chunkLength as u64).wrapping_add(12) > insize
                    || pos.wrapping_add(chunkLength as u64).wrapping_add(12) < pos
                {
                    (*state).error = 64;
                    break;
                } else {
                    data = lodepng_chunk_data_const(chunk);
                    unknown = 0;
                    if lodepng_chunk_type_equals(chunk, b"IDAT\0" as *const u8 as *const i8) != 0 {
                        let mut newsize: u64 = 0;
                        if lodepng_addofl(idatsize, chunkLength as u64, &mut newsize) != 0 {
                            (*state).error = 95;
                            break;
                        } else if newsize > insize {
                            (*state).error = 95;
                            break;
                        } else {
                            lodepng_memcpy(
                                idat.offset(idatsize as isize) as *mut libc::c_void,
                                data as *const libc::c_void,
                                chunkLength as u64,
                            );
                            idatsize = (idatsize as u64).wrapping_add(chunkLength as u64) as u64;
                            critical_pos = 3;
                        }
                    } else if lodepng_chunk_type_equals(chunk, b"IEND\0" as *const u8 as *const i8)
                        != 0
                    {
                        IEND = 1;
                    } else if lodepng_chunk_type_equals(chunk, b"PLTE\0" as *const u8 as *const i8)
                        != 0
                    {
                        (*state).error =
                            readChunk_PLTE(&mut (*state).info_png.color, data, chunkLength as u64);
                        if (*state).error != 0 {
                            break;
                        }
                        critical_pos = 2;
                    } else if lodepng_chunk_type_equals(chunk, b"tRNS\0" as *const u8 as *const i8)
                        != 0
                    {
                        (*state).error =
                            readChunk_tRNS(&mut (*state).info_png.color, data, chunkLength as u64);
                        if (*state).error != 0 {
                            break;
                        }
                    } else if lodepng_chunk_type_equals(chunk, b"bKGD\0" as *const u8 as *const i8)
                        != 0
                    {
                        (*state).error =
                            readChunk_bKGD(&mut (*state).info_png, data, chunkLength as u64);
                        if (*state).error != 0 {
                            break;
                        }
                    } else if lodepng_chunk_type_equals(chunk, b"tEXt\0" as *const u8 as *const i8)
                        != 0
                    {
                        if (*state).decoder.read_text_chunks != 0 {
                            (*state).error =
                                readChunk_tEXt(&mut (*state).info_png, data, chunkLength as u64);
                            if (*state).error != 0 {
                                break;
                            }
                        }
                    } else if lodepng_chunk_type_equals(chunk, b"zTXt\0" as *const u8 as *const i8)
                        != 0
                    {
                        if (*state).decoder.read_text_chunks != 0 {
                            (*state).error = readChunk_zTXt(
                                &mut (*state).info_png,
                                &mut (*state).decoder,
                                data,
                                chunkLength as u64,
                            );
                            if (*state).error != 0 {
                                break;
                            }
                        }
                    } else if lodepng_chunk_type_equals(chunk, b"iTXt\0" as *const u8 as *const i8)
                        != 0
                    {
                        if (*state).decoder.read_text_chunks != 0 {
                            (*state).error = readChunk_iTXt(
                                &mut (*state).info_png,
                                &mut (*state).decoder,
                                data,
                                chunkLength as u64,
                            );
                            if (*state).error != 0 {
                                break;
                            }
                        }
                    } else if lodepng_chunk_type_equals(chunk, b"tIME\0" as *const u8 as *const i8)
                        != 0
                    {
                        (*state).error =
                            readChunk_tIME(&mut (*state).info_png, data, chunkLength as u64);
                        if (*state).error != 0 {
                            break;
                        }
                    } else if lodepng_chunk_type_equals(chunk, b"pHYs\0" as *const u8 as *const i8)
                        != 0
                    {
                        (*state).error =
                            readChunk_pHYs(&mut (*state).info_png, data, chunkLength as u64);
                        if (*state).error != 0 {
                            break;
                        }
                    } else if lodepng_chunk_type_equals(chunk, b"gAMA\0" as *const u8 as *const i8)
                        != 0
                    {
                        (*state).error =
                            readChunk_gAMA(&mut (*state).info_png, data, chunkLength as u64);
                        if (*state).error != 0 {
                            break;
                        }
                    } else if lodepng_chunk_type_equals(chunk, b"cHRM\0" as *const u8 as *const i8)
                        != 0
                    {
                        (*state).error =
                            readChunk_cHRM(&mut (*state).info_png, data, chunkLength as u64);
                        if (*state).error != 0 {
                            break;
                        }
                    } else if lodepng_chunk_type_equals(chunk, b"sRGB\0" as *const u8 as *const i8)
                        != 0
                    {
                        (*state).error =
                            readChunk_sRGB(&mut (*state).info_png, data, chunkLength as u64);
                        if (*state).error != 0 {
                            break;
                        }
                    } else if lodepng_chunk_type_equals(chunk, b"iCCP\0" as *const u8 as *const i8)
                        != 0
                    {
                        (*state).error = readChunk_iCCP(
                            &mut (*state).info_png,
                            &mut (*state).decoder,
                            data,
                            chunkLength as u64,
                        );
                        if (*state).error != 0 {
                            break;
                        }
                    } else if lodepng_chunk_type_equals(chunk, b"sBIT\0" as *const u8 as *const i8)
                        != 0
                    {
                        (*state).error =
                            readChunk_sBIT(&mut (*state).info_png, data, chunkLength as u64);
                        if (*state).error != 0 {
                            break;
                        }
                    } else if (*state).decoder.ignore_critical == 0
                        && lodepng_chunk_ancillary(chunk) == 0
                    {
                        (*state).error = 69;
                        break;
                    } else {
                        unknown = 1;
                        if (*state).decoder.remember_unknown_chunks != 0 {
                            (*state).error = lodepng_chunk_append(
                                &mut *((*state).info_png.unknown_chunks_data)
                                    .as_mut_ptr()
                                    .offset(critical_pos.wrapping_sub(1) as isize),
                                &mut *((*state).info_png.unknown_chunks_size)
                                    .as_mut_ptr()
                                    .offset(critical_pos.wrapping_sub(1) as isize),
                                chunk,
                            );
                            if (*state).error != 0 {
                                break;
                            }
                        }
                    }
                    if (*state).decoder.ignore_crc == 0 && unknown == 0 {
                        if lodepng_chunk_check_crc(chunk) != 0 {
                            (*state).error = 57;
                            break;
                        }
                    }
                    if IEND == 0 {
                        chunk = lodepng_chunk_next_const(chunk, in_0.offset(insize as isize));
                    }
                }
            }
        }
        if (*state).error == 0
            && (*state).info_png.color.colortype as u32 == LCT_PALETTE as u32
            && ((*state).info_png.color.palette).is_null()
        {
            (*state).error = 106;
        }
        if (*state).error == 0 {
            if (*state).info_png.interlace_method == 0 {
                let mut bpp = lodepng_get_bpp(&mut (*state).info_png.color) as u64;
                expected_size = lodepng_get_raw_size_idat(*w, *h, bpp as u32);
            } else {
                let mut bpp_0 = lodepng_get_bpp(&mut (*state).info_png.color) as u64;
                expected_size = 0;
                expected_size = (expected_size as u64).wrapping_add(lodepng_get_raw_size_idat(
                    (*w).wrapping_add(7) >> 3,
                    (*h).wrapping_add(7) >> 3,
                    bpp_0 as u32,
                )) as u64;
                if *w > 4 {
                    expected_size = (expected_size as u64).wrapping_add(lodepng_get_raw_size_idat(
                        (*w).wrapping_add(3) >> 3,
                        (*h).wrapping_add(7) >> 3,
                        bpp_0 as u32,
                    )) as u64;
                }
                expected_size = (expected_size as u64).wrapping_add(lodepng_get_raw_size_idat(
                    (*w).wrapping_add(3) >> 2,
                    (*h).wrapping_add(3) >> 3,
                    bpp_0 as u32,
                )) as u64;
                if *w > 2 {
                    expected_size = (expected_size as u64).wrapping_add(lodepng_get_raw_size_idat(
                        (*w).wrapping_add(1) >> 2,
                        (*h).wrapping_add(3) >> 2,
                        bpp_0 as u32,
                    )) as u64;
                }
                expected_size = (expected_size as u64).wrapping_add(lodepng_get_raw_size_idat(
                    (*w).wrapping_add(1) >> 1,
                    (*h).wrapping_add(1) >> 2,
                    bpp_0 as u32,
                )) as u64;
                if *w > 1 {
                    expected_size = (expected_size as u64).wrapping_add(lodepng_get_raw_size_idat(
                        (*w).wrapping_add(0) >> 1,
                        (*h).wrapping_add(1) >> 1,
                        bpp_0 as u32,
                    )) as u64;
                }
                expected_size = (expected_size as u64).wrapping_add(lodepng_get_raw_size_idat(
                    (*w).wrapping_add(0),
                    (*h).wrapping_add(0) >> 1,
                    bpp_0 as u32,
                )) as u64;
            };
            (*state).error = zlib_decompress(
                &mut scanlines,
                &mut scanlines_size,
                expected_size,
                idat,
                idatsize,
                &mut (*state).decoder.zlibsettings,
            );
        }
        if (*state).error == 0 && scanlines_size != expected_size {
            (*state).error = 91;
        }
        lodepng_free(idat as *mut libc::c_void);
        if (*state).error == 0 {
            outsize = lodepng_get_raw_size(*w, *h, &mut (*state).info_png.color);
            *out = lodepng_malloc(outsize) as *mut u8;
            if (*out).is_null() {
                (*state).error = 83;
            }
        }
        if (*state).error == 0 {
            lodepng_memset(*out as *mut libc::c_void, 0, outsize);
            (*state).error = postProcessScanlines(*out, scanlines, *w, *h, &mut (*state).info_png);
        }
        lodepng_free(scanlines as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_decode(
    mut out: *mut *mut u8,
    mut w: *mut u32,
    mut h: *mut u32,
    mut state: *mut LodePNGState,
    mut in_0: *const u8,
    mut insize: u64,
) -> u32 {
    unsafe {
        *out = 0 as *mut u8;
        decodeGeneric(out, w, h, state, in_0, insize);
        if (*state).error != 0 {
            return (*state).error;
        }
        if (*state).decoder.color_convert == 0
            || lodepng_color_mode_equal(&mut (*state).info_raw, &mut (*state).info_png.color) != 0
        {
            if (*state).decoder.color_convert == 0 {
                (*state).error =
                    lodepng_color_mode_copy(&mut (*state).info_raw, &mut (*state).info_png.color);
                if (*state).error != 0 {
                    return (*state).error;
                }
            }
        } else {
            let mut data = *out;
            let mut outsize: u64 = 0;
            if !((*state).info_raw.colortype as u32 == LCT_RGB as u32
                || (*state).info_raw.colortype as u32 == LCT_RGBA as u32)
                && !((*state).info_raw.bitdepth == 8)
            {
                return 56;
            }
            outsize = lodepng_get_raw_size(*w, *h, &mut (*state).info_raw);
            *out = lodepng_malloc(outsize) as *mut u8;
            if (*out).is_null() {
                (*state).error = 83;
            } else {
                (*state).error = lodepng_convert(
                    *out,
                    data,
                    &mut (*state).info_raw,
                    &mut (*state).info_png.color,
                    *w,
                    *h,
                );
            }
            lodepng_free(data as *mut libc::c_void);
        }
        return (*state).error;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_decode_memory(
    mut out: *mut *mut u8,
    mut w: *mut u32,
    mut h: *mut u32,
    mut in_0: *const u8,
    mut insize: u64,
    mut colortype: u32,
    mut bitdepth: u32,
) -> u32 {
    unsafe {
        let mut error: u32 = 0;
        let mut state = LodePNGState {
            decoder: LodePNGDecoderSettings {
                zlibsettings: LodePNGDecompressSettings {
                    ignore_adler32: 0,
                    ignore_nlen: 0,
                    max_output_size: 0,
                    custom_zlib: None,
                    custom_inflate: None,
                    custom_context: 0 as *const libc::c_void,
                },
                ignore_crc: 0,
                ignore_critical: 0,
                ignore_end: 0,
                color_convert: 0,
                read_text_chunks: 0,
                remember_unknown_chunks: 0,
                max_text_size: 0,
                max_icc_size: 0,
            },
            encoder: LodePNGEncoderSettings {
                zlibsettings: LodePNGCompressSettings {
                    btype: 0,
                    use_lz77: 0,
                    windowsize: 0,
                    minmatch: 0,
                    nicematch: 0,
                    lazymatching: 0,
                    custom_zlib: None,
                    custom_deflate: None,
                    custom_context: 0 as *const libc::c_void,
                },
                auto_convert: 0,
                filter_palette_zero: 0,
                filter_strategy: LFS_ZERO,
                predefined_filters: 0 as *const u8,
                force_palette: 0,
                add_id: 0,
                text_compression: 0,
            },
            info_raw: LodePNGColorMode {
                colortype: LCT_GREY,
                bitdepth: 0,
                palette: 0 as *mut u8,
                palettesize: 0,
                key_defined: 0,
                key_r: 0,
                key_g: 0,
                key_b: 0,
            },
            info_png: LodePNGInfo {
                compression_method: 0,
                filter_method: 0,
                interlace_method: 0,
                color: LodePNGColorMode {
                    colortype: LCT_GREY,
                    bitdepth: 0,
                    palette: 0 as *mut u8,
                    palettesize: 0,
                    key_defined: 0,
                    key_r: 0,
                    key_g: 0,
                    key_b: 0,
                },
                background_defined: 0,
                background_r: 0,
                background_g: 0,
                background_b: 0,
                text_num: 0,
                text_keys: 0 as *mut *mut i8,
                text_strings: 0 as *mut *mut i8,
                itext_num: 0,
                itext_keys: 0 as *mut *mut i8,
                itext_langtags: 0 as *mut *mut i8,
                itext_transkeys: 0 as *mut *mut i8,
                itext_strings: 0 as *mut *mut i8,
                time_defined: 0,
                time: LodePNGTime {
                    year: 0,
                    month: 0,
                    day: 0,
                    hour: 0,
                    minute: 0,
                    second: 0,
                },
                phys_defined: 0,
                phys_x: 0,
                phys_y: 0,
                phys_unit: 0,
                gama_defined: 0,
                gama_gamma: 0,
                chrm_defined: 0,
                chrm_white_x: 0,
                chrm_white_y: 0,
                chrm_red_x: 0,
                chrm_red_y: 0,
                chrm_green_x: 0,
                chrm_green_y: 0,
                chrm_blue_x: 0,
                chrm_blue_y: 0,
                srgb_defined: 0,
                srgb_intent: 0,
                iccp_defined: 0,
                iccp_name: 0 as *mut i8,
                iccp_profile: 0 as *mut u8,
                iccp_profile_size: 0,
                sbit_defined: 0,
                sbit_r: 0,
                sbit_g: 0,
                sbit_b: 0,
                sbit_a: 0,
                unknown_chunks_data: [0 as *mut u8; 3],
                unknown_chunks_size: [0; 3],
            },
            error: 0,
        };
        lodepng_state_init(&mut state);
        state.info_raw.colortype = colortype;
        state.info_raw.bitdepth = bitdepth;
        state.decoder.read_text_chunks = 0;
        state.decoder.remember_unknown_chunks = 0;
        error = lodepng_decode(out, w, h, &mut state, in_0, insize);
        lodepng_state_cleanup(&mut state);
        return error;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_decode32(
    mut out: *mut *mut u8,
    mut w: *mut u32,
    mut h: *mut u32,
    mut in_0: *const u8,
    mut insize: u64,
) -> u32 {
    unsafe {
        return lodepng_decode_memory(out, w, h, in_0, insize, LCT_RGBA, 8);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_decode24(
    mut out: *mut *mut u8,
    mut w: *mut u32,
    mut h: *mut u32,
    mut in_0: *const u8,
    mut insize: u64,
) -> u32 {
    unsafe {
        return lodepng_decode_memory(out, w, h, in_0, insize, LCT_RGB, 8);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_decode_file(
    mut out: *mut *mut u8,
    mut w: *mut u32,
    mut h: *mut u32,
    mut filename: *const i8,
    mut colortype: u32,
    mut bitdepth: u32,
) -> u32 {
    unsafe {
        let mut buffer = 0 as *mut u8;
        let mut buffersize: u64 = 0;
        let mut error: u32 = 0;
        *out = 0 as *mut u8;
        *h = 0;
        *w = *h;
        error = lodepng_load_file(&mut buffer, &mut buffersize, filename);
        if error == 0 {
            error = lodepng_decode_memory(out, w, h, buffer, buffersize, colortype, bitdepth);
        }
        lodepng_free(buffer as *mut libc::c_void);
        return error;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_decode32_file(
    mut out: *mut *mut u8,
    mut w: *mut u32,
    mut h: *mut u32,
    mut filename: *const i8,
) -> u32 {
    unsafe {
        return lodepng_decode_file(out, w, h, filename, LCT_RGBA, 8);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_decode24_file(
    mut out: *mut *mut u8,
    mut w: *mut u32,
    mut h: *mut u32,
    mut filename: *const i8,
) -> u32 {
    unsafe {
        return lodepng_decode_file(out, w, h, filename, LCT_RGB, 8);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_decoder_settings_init(mut settings: *mut LodePNGDecoderSettings) {
    unsafe {
        (*settings).color_convert = 1;
        (*settings).read_text_chunks = 1;
        (*settings).remember_unknown_chunks = 0;
        (*settings).max_text_size = 16777216;
        (*settings).max_icc_size = 16777216;
        (*settings).ignore_crc = 0;
        (*settings).ignore_critical = 0;
        (*settings).ignore_end = 0;
        lodepng_decompress_settings_init(&mut (*settings).zlibsettings);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_state_init(mut state: *mut LodePNGState) {
    unsafe {
        lodepng_decoder_settings_init(&mut (*state).decoder);
        lodepng_encoder_settings_init(&mut (*state).encoder);
        lodepng_color_mode_init(&mut (*state).info_raw);
        lodepng_info_init(&mut (*state).info_png);
        (*state).error = 1;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_state_cleanup(mut state: *mut LodePNGState) {
    unsafe {
        lodepng_color_mode_cleanup(&mut (*state).info_raw);
        lodepng_info_cleanup(&mut (*state).info_png);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_state_copy(mut dest: *mut LodePNGState, mut source: *const LodePNGState) {
    unsafe {
        lodepng_state_cleanup(dest);
        *dest = *source;
        lodepng_color_mode_init(&mut (*dest).info_raw);
        lodepng_info_init(&mut (*dest).info_png);
        (*dest).error = lodepng_color_mode_copy(&mut (*dest).info_raw, &(*source).info_raw);
        if (*dest).error != 0 {
            return;
        };
        (*dest).error = lodepng_info_copy(&mut (*dest).info_png, &(*source).info_png);
        if (*dest).error != 0 {
            return;
        }
    }
}

extern "C" fn writeSignature(mut out: *mut ucvector) -> u32 {
    unsafe {
        let mut pos = (*out).size;
        let signature: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
        if ucvector_resize(out, ((*out).size).wrapping_add(8)) == 0 {
            return 83;
        }
        lodepng_memcpy(
            ((*out).data).offset(pos as isize) as *mut libc::c_void,
            signature.as_ptr() as *const libc::c_void,
            8,
        );
        return 0;
    }
}

extern "C" fn addChunk_IHDR(
    mut out: *mut ucvector,
    mut w: u32,
    mut h: u32,
    mut colortype: u32,
    mut bitdepth: u32,
    mut interlace_method: u32,
) -> u32 {
    unsafe {
        let mut chunk = 0 as *mut u8;
        let mut data = 0 as *mut u8;
        let mut error =
            lodepng_chunk_init(&mut chunk, out, 13, b"IHDR\0" as *const u8 as *const i8);
        if error != 0 {
            return error;
        }
        data = chunk.offset(8 as isize);
        lodepng_set32bitInt(data.offset(0 as isize), w);
        lodepng_set32bitInt(data.offset(4 as isize), h);
        *data.offset(8 as isize) = bitdepth as u8;
        *data.offset(9 as isize) = colortype as u8;
        *data.offset(10 as isize) = 0;
        *data.offset(11 as isize) = 0;
        *data.offset(12 as isize) = interlace_method as u8;
        lodepng_chunk_generate_crc(chunk);
        return 0;
    }
}

extern "C" fn addChunk_PLTE(mut out: *mut ucvector, mut info: *const LodePNGColorMode) -> u32 {
    unsafe {
        let mut chunk = 0 as *mut u8;
        let mut i: u64 = 0;
        let mut j = 8;
        if (*info).palettesize == 0 || (*info).palettesize > 256 {
            return 68;
        }
        let mut error = lodepng_chunk_init(
            &mut chunk,
            out,
            ((*info).palettesize).wrapping_mul(3) as u32,
            b"PLTE\0" as *const u8 as *const i8,
        );
        if error != 0 {
            return error;
        }
        i = 0;
        while i != (*info).palettesize {
            let fresh182 = j;
            j = j.wrapping_add(1);
            *chunk.offset(fresh182 as isize) =
                *((*info).palette).offset(i.wrapping_mul(4).wrapping_add(0) as isize);
            let fresh183 = j;
            j = j.wrapping_add(1);
            *chunk.offset(fresh183 as isize) =
                *((*info).palette).offset(i.wrapping_mul(4).wrapping_add(1) as isize);
            let fresh184 = j;
            j = j.wrapping_add(1);
            *chunk.offset(fresh184 as isize) =
                *((*info).palette).offset(i.wrapping_mul(4).wrapping_add(2) as isize);
            i = i.wrapping_add(1);
        }
        lodepng_chunk_generate_crc(chunk);
        return 0;
    }
}

extern "C" fn addChunk_tRNS(mut out: *mut ucvector, mut info: *const LodePNGColorMode) -> u32 {
    unsafe {
        let mut chunk = 0 as *mut u8;
        if (*info).colortype as u32 == LCT_PALETTE as u32 {
            let mut i: u64 = 0;
            let mut amount = (*info).palettesize;
            i = (*info).palettesize;
            while i != 0 {
                if *((*info).palette)
                    .offset(4u64.wrapping_mul(i.wrapping_sub(1)).wrapping_add(3) as isize)
                    as i32
                    != 255
                {
                    break;
                }
                amount = amount.wrapping_sub(1);
                i = i.wrapping_sub(1);
            }
            if amount != 0 {
                let mut error = lodepng_chunk_init(
                    &mut chunk,
                    out,
                    amount as u32,
                    b"tRNS\0" as *const u8 as *const i8,
                );
                if error != 0 {
                    return error;
                }
                i = 0;
                while i != amount {
                    *chunk.offset(8u64.wrapping_add(i) as isize) =
                        *((*info).palette).offset(4u64.wrapping_mul(i).wrapping_add(3) as isize);
                    i = i.wrapping_add(1);
                }
            }
        } else if (*info).colortype as u32 == LCT_GREY as u32 {
            if (*info).key_defined != 0 {
                let mut error_0 =
                    lodepng_chunk_init(&mut chunk, out, 2, b"tRNS\0" as *const u8 as *const i8);
                if error_0 != 0 {
                    return error_0;
                };
                *chunk.offset(8 as isize) = ((*info).key_r >> 8i32) as u8;
                *chunk.offset(9 as isize) = ((*info).key_r & 255u32) as u8;
            }
        } else if (*info).colortype as u32 == LCT_RGB as u32 {
            if (*info).key_defined != 0 {
                let mut error_1 =
                    lodepng_chunk_init(&mut chunk, out, 6, b"tRNS\0" as *const u8 as *const i8);
                if error_1 != 0 {
                    return error_1;
                };
                *chunk.offset(8 as isize) = ((*info).key_r >> 8i32) as u8;
                *chunk.offset(9 as isize) = ((*info).key_r & 255u32) as u8;
                *chunk.offset(10 as isize) = ((*info).key_g >> 8i32) as u8;
                *chunk.offset(11 as isize) = ((*info).key_g & 255u32) as u8;
                *chunk.offset(12 as isize) = ((*info).key_b >> 8i32) as u8;
                *chunk.offset(13 as isize) = ((*info).key_b & 255u32) as u8;
            }
        }
        if !chunk.is_null() {
            lodepng_chunk_generate_crc(chunk);
        }
        return 0;
    }
}

extern "C" fn addChunk_IDAT(
    mut out: *mut ucvector,
    mut data: *const u8,
    mut datasize: u64,
    mut zlibsettings: *mut LodePNGCompressSettings,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut zlib = 0 as *mut u8;
        let mut zlibsize = 0;
        error = zlib_compress(&mut zlib, &mut zlibsize, data, datasize, zlibsettings);
        if error == 0 {
            error = lodepng_chunk_createv(
                out,
                zlibsize as u32,
                b"IDAT\0" as *const u8 as *const i8,
                zlib,
            );
        }
        lodepng_free(zlib as *mut libc::c_void);
        return error;
    }
}

extern "C" fn addChunk_IEND(mut out: *mut ucvector) -> u32 {
    unsafe {
        return lodepng_chunk_createv(out, 0, b"IEND\0" as *const u8 as *const i8, 0 as *const u8);
    }
}

extern "C" fn addChunk_tEXt(
    mut out: *mut ucvector,
    mut keyword: *const i8,
    mut textstring: *const i8,
) -> u32 {
    unsafe {
        let mut chunk = 0 as *mut u8;
        let mut keysize = lodepng_strlen(keyword);
        let mut textsize = lodepng_strlen(textstring);
        let mut size = keysize.wrapping_add(1).wrapping_add(textsize);
        if keysize < 1 || keysize > 79 {
            return 89;
        }
        let mut error = lodepng_chunk_init(
            &mut chunk,
            out,
            size as u32,
            b"tEXt\0" as *const u8 as *const i8,
        );
        if error != 0 {
            return error;
        }
        lodepng_memcpy(
            chunk.offset(8 as isize) as *mut libc::c_void,
            keyword as *const libc::c_void,
            keysize,
        );
        *chunk.offset(8u64.wrapping_add(keysize) as isize) = 0;
        lodepng_memcpy(
            chunk.offset(9 as isize).offset(keysize as isize) as *mut libc::c_void,
            textstring as *const libc::c_void,
            textsize,
        );
        lodepng_chunk_generate_crc(chunk);
        return 0;
    }
}

extern "C" fn addChunk_zTXt(
    mut out: *mut ucvector,
    mut keyword: *const i8,
    mut textstring: *const i8,
    mut zlibsettings: *mut LodePNGCompressSettings,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut chunk = 0 as *mut u8;
        let mut compressed = 0 as *mut u8;
        let mut compressedsize = 0;
        let mut textsize = lodepng_strlen(textstring);
        let mut keysize = lodepng_strlen(keyword);
        if keysize < 1 || keysize > 79 {
            return 89;
        }
        error = zlib_compress(
            &mut compressed,
            &mut compressedsize,
            textstring as *const u8,
            textsize,
            zlibsettings,
        );
        if error == 0 {
            let mut size = keysize.wrapping_add(2).wrapping_add(compressedsize);
            error = lodepng_chunk_init(
                &mut chunk,
                out,
                size as u32,
                b"zTXt\0" as *const u8 as *const i8,
            );
        }
        if error == 0 {
            lodepng_memcpy(
                chunk.offset(8 as isize) as *mut libc::c_void,
                keyword as *const libc::c_void,
                keysize,
            );
            *chunk.offset(8u64.wrapping_add(keysize) as isize) = 0;
            *chunk.offset(9u64.wrapping_add(keysize) as isize) = 0;
            lodepng_memcpy(
                chunk.offset(10 as isize).offset(keysize as isize) as *mut libc::c_void,
                compressed as *const libc::c_void,
                compressedsize,
            );
            lodepng_chunk_generate_crc(chunk);
        }
        lodepng_free(compressed as *mut libc::c_void);
        return error;
    }
}

extern "C" fn addChunk_iTXt(
    mut out: *mut ucvector,
    mut compress: u32,
    mut keyword: *const i8,
    mut langtag: *const i8,
    mut transkey: *const i8,
    mut textstring: *const i8,
    mut zlibsettings: *mut LodePNGCompressSettings,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut chunk = 0 as *mut u8;
        let mut compressed = 0 as *mut u8;
        let mut compressedsize = 0;
        let mut textsize = lodepng_strlen(textstring);
        let mut keysize = lodepng_strlen(keyword);
        let mut langsize = lodepng_strlen(langtag);
        let mut transsize = lodepng_strlen(transkey);
        if keysize < 1 || keysize > 79 {
            return 89;
        }
        if compress != 0 {
            error = zlib_compress(
                &mut compressed,
                &mut compressedsize,
                textstring as *const u8,
                textsize,
                zlibsettings,
            );
        }
        if error == 0 {
            let mut size = keysize
                .wrapping_add(3)
                .wrapping_add(langsize)
                .wrapping_add(1)
                .wrapping_add(transsize)
                .wrapping_add(1)
                .wrapping_add(
                    (if compress != 0 {
                        compressedsize
                    } else {
                        textsize
                    }),
                );
            error = lodepng_chunk_init(
                &mut chunk,
                out,
                size as u32,
                b"iTXt\0" as *const u8 as *const i8,
            );
        }
        if error == 0 {
            let mut pos = 8;
            lodepng_memcpy(
                chunk.offset(pos as isize) as *mut libc::c_void,
                keyword as *const libc::c_void,
                keysize,
            );
            pos = (pos as u64).wrapping_add(keysize) as u64;
            let fresh185 = pos;
            pos = pos.wrapping_add(1);
            *chunk.offset(fresh185 as isize) = 0;
            let fresh186 = pos;
            pos = pos.wrapping_add(1);
            *chunk.offset(fresh186 as isize) = (if compress != 0 { 1 } else { 0 }) as u8;
            let fresh187 = pos;
            pos = pos.wrapping_add(1);
            *chunk.offset(fresh187 as isize) = 0;
            lodepng_memcpy(
                chunk.offset(pos as isize) as *mut libc::c_void,
                langtag as *const libc::c_void,
                langsize,
            );
            pos = (pos as u64).wrapping_add(langsize) as u64;
            let fresh188 = pos;
            pos = pos.wrapping_add(1);
            *chunk.offset(fresh188 as isize) = 0;
            lodepng_memcpy(
                chunk.offset(pos as isize) as *mut libc::c_void,
                transkey as *const libc::c_void,
                transsize,
            );
            pos = (pos as u64).wrapping_add(transsize) as u64;
            let fresh189 = pos;
            pos = pos.wrapping_add(1);
            *chunk.offset(fresh189 as isize) = 0;
            if compress != 0 {
                lodepng_memcpy(
                    chunk.offset(pos as isize) as *mut libc::c_void,
                    compressed as *const libc::c_void,
                    compressedsize,
                );
            } else {
                lodepng_memcpy(
                    chunk.offset(pos as isize) as *mut libc::c_void,
                    textstring as *const libc::c_void,
                    textsize,
                );
            }
            lodepng_chunk_generate_crc(chunk);
        }
        lodepng_free(compressed as *mut libc::c_void);
        return error;
    }
}

extern "C" fn addChunk_bKGD(mut out: *mut ucvector, mut info: *const LodePNGInfo) -> u32 {
    unsafe {
        let mut chunk = 0 as *mut u8;
        if (*info).color.colortype as u32 == LCT_GREY as u32
            || (*info).color.colortype as u32 == LCT_GREY_ALPHA as u32
        {
            let mut error =
                lodepng_chunk_init(&mut chunk, out, 2, b"bKGD\0" as *const u8 as *const i8);
            if error != 0 {
                return error;
            };
            *chunk.offset(8 as isize) = ((*info).background_r >> 8i32) as u8;
            *chunk.offset(9 as isize) = ((*info).background_r & 255u32) as u8;
        } else if (*info).color.colortype as u32 == LCT_RGB as u32
            || (*info).color.colortype as u32 == LCT_RGBA as u32
        {
            let mut error_0 =
                lodepng_chunk_init(&mut chunk, out, 6, b"bKGD\0" as *const u8 as *const i8);
            if error_0 != 0 {
                return error_0;
            };
            *chunk.offset(8 as isize) = ((*info).background_r >> 8i32) as u8;
            *chunk.offset(9 as isize) = ((*info).background_r & 255u32) as u8;
            *chunk.offset(10 as isize) = ((*info).background_g >> 8i32) as u8;
            *chunk.offset(11 as isize) = ((*info).background_g & 255u32) as u8;
            *chunk.offset(12 as isize) = ((*info).background_b >> 8i32) as u8;
            *chunk.offset(13 as isize) = ((*info).background_b & 255u32) as u8;
        } else if (*info).color.colortype as u32 == LCT_PALETTE as u32 {
            let mut error_1 =
                lodepng_chunk_init(&mut chunk, out, 1, b"bKGD\0" as *const u8 as *const i8);
            if error_1 != 0 {
                return error_1;
            };
            *chunk.offset(8 as isize) = ((*info).background_r & 255u32) as u8;
        }
        if !chunk.is_null() {
            lodepng_chunk_generate_crc(chunk);
        }
        return 0;
    }
}

extern "C" fn addChunk_tIME(mut out: *mut ucvector, mut time: *const LodePNGTime) -> u32 {
    unsafe {
        let mut chunk = 0 as *mut u8;
        let mut error = lodepng_chunk_init(&mut chunk, out, 7, b"tIME\0" as *const u8 as *const i8);
        if error != 0 {
            return error;
        };
        *chunk.offset(8 as isize) = ((*time).year >> 8i32) as u8;
        *chunk.offset(9 as isize) = ((*time).year & 255u32) as u8;
        *chunk.offset(10 as isize) = (*time).month as u8;
        *chunk.offset(11 as isize) = (*time).day as u8;
        *chunk.offset(12 as isize) = (*time).hour as u8;
        *chunk.offset(13 as isize) = (*time).minute as u8;
        *chunk.offset(14 as isize) = (*time).second as u8;
        lodepng_chunk_generate_crc(chunk);
        return 0;
    }
}

extern "C" fn addChunk_pHYs(mut out: *mut ucvector, mut info: *const LodePNGInfo) -> u32 {
    unsafe {
        let mut chunk = 0 as *mut u8;
        let mut error = lodepng_chunk_init(&mut chunk, out, 9, b"pHYs\0" as *const u8 as *const i8);
        if error != 0 {
            return error;
        }
        lodepng_set32bitInt(chunk.offset(8 as isize), (*info).phys_x);
        lodepng_set32bitInt(chunk.offset(12 as isize), (*info).phys_y);
        *chunk.offset(16 as isize) = (*info).phys_unit as u8;
        lodepng_chunk_generate_crc(chunk);
        return 0;
    }
}

extern "C" fn addChunk_gAMA(mut out: *mut ucvector, mut info: *const LodePNGInfo) -> u32 {
    unsafe {
        let mut chunk = 0 as *mut u8;
        let mut error = lodepng_chunk_init(&mut chunk, out, 4, b"gAMA\0" as *const u8 as *const i8);
        if error != 0 {
            return error;
        }
        lodepng_set32bitInt(chunk.offset(8 as isize), (*info).gama_gamma);
        lodepng_chunk_generate_crc(chunk);
        return 0;
    }
}

extern "C" fn addChunk_cHRM(mut out: *mut ucvector, mut info: *const LodePNGInfo) -> u32 {
    unsafe {
        let mut chunk = 0 as *mut u8;
        let mut error =
            lodepng_chunk_init(&mut chunk, out, 32, b"cHRM\0" as *const u8 as *const i8);
        if error != 0 {
            return error;
        }
        lodepng_set32bitInt(chunk.offset(8 as isize), (*info).chrm_white_x);
        lodepng_set32bitInt(chunk.offset(12 as isize), (*info).chrm_white_y);
        lodepng_set32bitInt(chunk.offset(16 as isize), (*info).chrm_red_x);
        lodepng_set32bitInt(chunk.offset(20 as isize), (*info).chrm_red_y);
        lodepng_set32bitInt(chunk.offset(24 as isize), (*info).chrm_green_x);
        lodepng_set32bitInt(chunk.offset(28 as isize), (*info).chrm_green_y);
        lodepng_set32bitInt(chunk.offset(32 as isize), (*info).chrm_blue_x);
        lodepng_set32bitInt(chunk.offset(36 as isize), (*info).chrm_blue_y);
        lodepng_chunk_generate_crc(chunk);
        return 0;
    }
}

extern "C" fn addChunk_sRGB(mut out: *mut ucvector, mut info: *const LodePNGInfo) -> u32 {
    unsafe {
        let mut data = (*info).srgb_intent as u8;
        return lodepng_chunk_createv(out, 1, b"sRGB\0" as *const u8 as *const i8, &mut data);
    }
}

extern "C" fn addChunk_iCCP(
    mut out: *mut ucvector,
    mut info: *const LodePNGInfo,
    mut zlibsettings: *mut LodePNGCompressSettings,
) -> u32 {
    unsafe {
        let mut error = 0;
        let mut chunk = 0 as *mut u8;
        let mut compressed = 0 as *mut u8;
        let mut compressedsize = 0;
        let mut keysize = lodepng_strlen((*info).iccp_name);
        if keysize < 1 || keysize > 79 {
            return 89;
        }
        error = zlib_compress(
            &mut compressed,
            &mut compressedsize,
            (*info).iccp_profile,
            (*info).iccp_profile_size as u64,
            zlibsettings,
        );
        if error == 0 {
            let mut size = keysize.wrapping_add(2).wrapping_add(compressedsize);
            error = lodepng_chunk_init(
                &mut chunk,
                out,
                size as u32,
                b"iCCP\0" as *const u8 as *const i8,
            );
        }
        if error == 0 {
            lodepng_memcpy(
                chunk.offset(8 as isize) as *mut libc::c_void,
                (*info).iccp_name as *const libc::c_void,
                keysize,
            );
            *chunk.offset(8u64.wrapping_add(keysize) as isize) = 0;
            *chunk.offset(9u64.wrapping_add(keysize) as isize) = 0;
            lodepng_memcpy(
                chunk.offset(10 as isize).offset(keysize as isize) as *mut libc::c_void,
                compressed as *const libc::c_void,
                compressedsize,
            );
            lodepng_chunk_generate_crc(chunk);
        }
        lodepng_free(compressed as *mut libc::c_void);
        return error;
    }
}

extern "C" fn addChunk_sBIT(mut out: *mut ucvector, mut info: *const LodePNGInfo) -> u32 {
    unsafe {
        let mut bitdepth = if (*info).color.colortype as u32 == LCT_PALETTE as u32 {
            8
        } else {
            (*info).color.bitdepth
        };
        let mut chunk = 0 as *mut u8;
        if (*info).color.colortype as u32 == LCT_GREY as u32 {
            if (*info).sbit_r == 0 || (*info).sbit_r > bitdepth {
                return 115;
            }
            let mut error =
                lodepng_chunk_init(&mut chunk, out, 1, b"sBIT\0" as *const u8 as *const i8);
            if error != 0 {
                return error;
            };
            *chunk.offset(8 as isize) = (*info).sbit_r as u8;
        } else if (*info).color.colortype as u32 == LCT_RGB as u32
            || (*info).color.colortype as u32 == LCT_PALETTE as u32
        {
            if (*info).sbit_r == 0 || (*info).sbit_g == 0 || (*info).sbit_b == 0 {
                return 115;
            }
            if (*info).sbit_r > bitdepth || (*info).sbit_g > bitdepth || (*info).sbit_b > bitdepth {
                return 115;
            }
            let mut error_0 =
                lodepng_chunk_init(&mut chunk, out, 3, b"sBIT\0" as *const u8 as *const i8);
            if error_0 != 0 {
                return error_0;
            };
            *chunk.offset(8 as isize) = (*info).sbit_r as u8;
            *chunk.offset(9 as isize) = (*info).sbit_g as u8;
            *chunk.offset(10 as isize) = (*info).sbit_b as u8;
        } else if (*info).color.colortype as u32 == LCT_GREY_ALPHA as u32 {
            if (*info).sbit_r == 0 || (*info).sbit_a == 0 {
                return 115;
            }
            if (*info).sbit_r > bitdepth || (*info).sbit_a > bitdepth {
                return 115;
            }
            let mut error_1 =
                lodepng_chunk_init(&mut chunk, out, 2, b"sBIT\0" as *const u8 as *const i8);
            if error_1 != 0 {
                return error_1;
            };
            *chunk.offset(8 as isize) = (*info).sbit_r as u8;
            *chunk.offset(9 as isize) = (*info).sbit_a as u8;
        } else if (*info).color.colortype as u32 == LCT_RGBA as u32 {
            if (*info).sbit_r == 0
                || (*info).sbit_g == 0
                || (*info).sbit_b == 0
                || (*info).sbit_a == 0
                || (*info).sbit_r > bitdepth
                || (*info).sbit_g > bitdepth
                || (*info).sbit_b > bitdepth
                || (*info).sbit_a > bitdepth
            {
                return 115;
            }
            let mut error_2 =
                lodepng_chunk_init(&mut chunk, out, 4, b"sBIT\0" as *const u8 as *const i8);
            if error_2 != 0 {
                return error_2;
            };
            *chunk.offset(8 as isize) = (*info).sbit_r as u8;
            *chunk.offset(9 as isize) = (*info).sbit_g as u8;
            *chunk.offset(10 as isize) = (*info).sbit_b as u8;
            *chunk.offset(11 as isize) = (*info).sbit_a as u8;
        }
        if !chunk.is_null() {
            lodepng_chunk_generate_crc(chunk);
        }
        return 0;
    }
}

extern "C" fn filterScanline(
    mut out: *mut u8,
    mut scanline: *const u8,
    mut prevline: *const u8,
    mut length: u64,
    mut bytewidth: u64,
    mut filterType: u8,
) {
    unsafe {
        let mut i: u64 = 0;
        match filterType as i32 {
            0 => {
                i = 0;
                while i != length {
                    *out.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
            }
            1 => {
                i = 0;
                while i != bytewidth {
                    *out.offset(i as isize) = *scanline.offset(i as isize);
                    i = i.wrapping_add(1);
                }
                i = bytewidth;
                while i < length {
                    *out.offset(i as isize) = (*scanline.offset(i as isize) as i32
                        - *scanline.offset(i.wrapping_sub(bytewidth) as isize) as i32)
                        as u8;
                    i = i.wrapping_add(1);
                }
            }
            2 => {
                if !prevline.is_null() {
                    i = 0;
                    while i != length {
                        *out.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            - *prevline.offset(i as isize) as i32)
                            as u8;
                        i = i.wrapping_add(1);
                    }
                } else {
                    i = 0;
                    while i != length {
                        *out.offset(i as isize) = *scanline.offset(i as isize);
                        i = i.wrapping_add(1);
                    }
                }
            }
            3 => {
                if !prevline.is_null() {
                    i = 0;
                    while i != bytewidth {
                        *out.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            - (*prevline.offset(i as isize) as i32 >> 1i32))
                            as u8;
                        i = i.wrapping_add(1);
                    }
                    i = bytewidth;
                    while i < length {
                        *out.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            - (*scanline.offset(i.wrapping_sub(bytewidth) as isize) as i32
                                + *prevline.offset(i as isize) as i32
                                >> 1i32)) as u8;
                        i = i.wrapping_add(1);
                    }
                } else {
                    i = 0;
                    while i != bytewidth {
                        *out.offset(i as isize) = *scanline.offset(i as isize);
                        i = i.wrapping_add(1);
                    }
                    i = bytewidth;
                    while i < length {
                        *out.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            - (*scanline.offset(i.wrapping_sub(bytewidth) as isize) as i32 >> 1i32))
                            as u8;
                        i = i.wrapping_add(1);
                    }
                }
            }
            4 => {
                if !prevline.is_null() {
                    i = 0;
                    while i != bytewidth {
                        *out.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            - *prevline.offset(i as isize) as i32)
                            as u8;
                        i = i.wrapping_add(1);
                    }
                    i = bytewidth;
                    while i < length {
                        *out.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            - paethPredictor(
                                *scanline.offset(i.wrapping_sub(bytewidth) as isize) as i16,
                                *prevline.offset(i as isize) as i16,
                                *prevline.offset(i.wrapping_sub(bytewidth) as isize) as i16,
                            ) as i32) as u8;
                        i = i.wrapping_add(1);
                    }
                } else {
                    i = 0;
                    while i != bytewidth {
                        *out.offset(i as isize) = *scanline.offset(i as isize);
                        i = i.wrapping_add(1);
                    }
                    i = bytewidth;
                    while i < length {
                        *out.offset(i as isize) = (*scanline.offset(i as isize) as i32
                            - *scanline.offset(i.wrapping_sub(bytewidth) as isize) as i32)
                            as u8;
                        i = i.wrapping_add(1);
                    }
                }
            }
            _ => return,
        };
    }
}

extern "C" fn ilog2(mut i: u64) -> u64 {
    let mut result = 0;
    if i >= 65536 {
        result = (result as u64).wrapping_add(16) as u64;
        i >>= 16;
    }
    if i >= 256 {
        result = (result as u64).wrapping_add(8) as u64;
        i >>= 8;
    }
    if i >= 16 {
        result = (result as u64).wrapping_add(4) as u64;
        i >>= 4;
    }
    if i >= 4 {
        result = (result as u64).wrapping_add(2) as u64;
        i >>= 2;
    }
    if i >= 2 {
        result = (result as u64).wrapping_add(1) as u64;
    }
    return result;
}

extern "C" fn ilog2i(mut i: u64) -> u64 {
    let mut l: u64 = 0;
    if i == 0 {
        return 0;
    }
    l = ilog2(i);
    return i
        .wrapping_mul(l)
        .wrapping_add(i.wrapping_sub((1u32 << l) as u64) << 1);
}

extern "C" fn filter(
    mut out: *mut u8,
    mut in_0: *const u8,
    mut w: u32,
    mut h: u32,
    mut color: *const LodePNGColorMode,
    mut settings: *const LodePNGEncoderSettings,
) -> u32 {
    unsafe {
        let mut bpp = lodepng_get_bpp(color);
        let mut linebytes = (lodepng_get_raw_size_idat(w, 1u32, bpp)).wrapping_sub(1);
        let mut bytewidth = bpp.wrapping_add(7).wrapping_div(8) as u64;
        let mut prevline = 0 as *const u8;
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut error = 0;
        let mut strategy = (*settings).filter_strategy;
        if (*settings).filter_palette_zero != 0
            && ((*color).colortype as u32 == LCT_PALETTE as u32 || (*color).bitdepth < 8)
        {
            strategy = LFS_ZERO;
        }
        if bpp == 0 {
            return 31;
        }
        if strategy as u32 >= LFS_ZERO as u32 && strategy as u32 <= LFS_FOUR as u32 {
            let mut type_0 = strategy as u8;
            y = 0;
            while y != h {
                let mut outindex = 1u64.wrapping_add(linebytes).wrapping_mul(y as u64);
                let mut inindex = linebytes.wrapping_mul(y as u64);
                *out.offset(outindex as isize) = type_0;
                filterScanline(
                    &mut *out.offset(outindex.wrapping_add(1) as isize),
                    &*in_0.offset(inindex as isize),
                    prevline,
                    linebytes,
                    bytewidth,
                    type_0,
                );
                prevline = &*in_0.offset(inindex as isize) as *const u8;
                y = y.wrapping_add(1);
            }
        } else if strategy as u32 == LFS_MINSUM as u32 {
            let mut attempt: [*mut u8; 5] = [0 as *mut u8; 5];
            let mut smallest = 0;
            let mut type_1: u8 = 0;
            let mut bestType = 0;
            type_1 = 0;
            while type_1 as i32 != 5 {
                attempt[type_1 as usize] = lodepng_malloc(linebytes) as *mut u8;
                if (attempt[type_1 as usize]).is_null() {
                    error = 83;
                }
                type_1 = type_1.wrapping_add(1);
            }
            if error == 0 {
                y = 0;
                while y != h {
                    type_1 = 0;
                    while type_1 as i32 != 5 {
                        let mut sum = 0;
                        filterScanline(
                            attempt[type_1 as usize],
                            &*in_0.offset((y as u64).wrapping_mul(linebytes) as isize),
                            prevline,
                            linebytes,
                            bytewidth,
                            type_1,
                        );
                        if type_1 as i32 == 0 {
                            x = 0;
                            while x as u64 != linebytes {
                                sum = (sum as u64).wrapping_add(
                                    *(attempt[type_1 as usize]).offset(x as isize) as u64,
                                ) as u64;
                                x = x.wrapping_add(1);
                            }
                        } else {
                            x = 0;
                            while x as u64 != linebytes {
                                let mut s = *(attempt[type_1 as usize]).offset(x as isize);
                                sum = (sum as u64).wrapping_add(
                                    (if (s as i32) < 128 {
                                        s as u32
                                    } else {
                                        255u32.wrapping_sub(s as u32)
                                    }) as u64,
                                ) as u64;
                                x = x.wrapping_add(1);
                            }
                        }
                        if type_1 as i32 == 0 || sum < smallest {
                            bestType = type_1;
                            smallest = sum;
                        }
                        type_1 = type_1.wrapping_add(1);
                    }
                    prevline =
                        &*in_0.offset((y as u64).wrapping_mul(linebytes) as isize) as *const u8;
                    *out.offset((y as u64).wrapping_mul(linebytes.wrapping_add(1)) as isize) =
                        bestType;
                    x = 0;
                    while x as u64 != linebytes {
                        *out.offset(
                            (y as u64)
                                .wrapping_mul(linebytes.wrapping_add(1))
                                .wrapping_add(1)
                                .wrapping_add(x as u64) as isize,
                        ) = *(attempt[bestType as usize]).offset(x as isize);
                        x = x.wrapping_add(1);
                    }
                    y = y.wrapping_add(1);
                }
            }
            type_1 = 0;
            while type_1 as i32 != 5 {
                lodepng_free(attempt[type_1 as usize] as *mut libc::c_void);
                type_1 = type_1.wrapping_add(1);
            }
        } else if strategy as u32 == LFS_ENTROPY as u32 {
            let mut attempt_0: [*mut u8; 5] = [0 as *mut u8; 5];
            let mut bestSum = 0;
            let mut type_2: u32 = 0;
            let mut bestType_0 = 0;
            let mut count: [u32; 256] = [0; 256];
            type_2 = 0;
            while type_2 != 5 {
                attempt_0[type_2 as usize] = lodepng_malloc(linebytes) as *mut u8;
                if (attempt_0[type_2 as usize]).is_null() {
                    error = 83;
                }
                type_2 = type_2.wrapping_add(1);
            }
            if error == 0 {
                y = 0;
                while y != h {
                    type_2 = 0;
                    while type_2 != 5 {
                        let mut sum_0 = 0;
                        filterScanline(
                            attempt_0[type_2 as usize],
                            &*in_0.offset((y as u64).wrapping_mul(linebytes) as isize),
                            prevline,
                            linebytes,
                            bytewidth,
                            type_2 as u8,
                        );
                        lodepng_memset(
                            count.as_mut_ptr() as *mut libc::c_void,
                            0,
                            256u64.wrapping_mul(::std::mem::size_of::<u32>() as u64),
                        );
                        x = 0;
                        while x as u64 != linebytes {
                            count[*(attempt_0[type_2 as usize]).offset(x as isize) as usize] =
                                (count[*(attempt_0[type_2 as usize]).offset(x as isize) as usize])
                                    .wrapping_add(1);
                            x = x.wrapping_add(1);
                        }
                        count[type_2 as usize] = (count[type_2 as usize]).wrapping_add(1);
                        x = 0;
                        while x != 256 {
                            sum_0 = (sum_0 as u64).wrapping_add(ilog2i(count[x as usize] as u64))
                                as u64;
                            x = x.wrapping_add(1);
                        }
                        if type_2 == 0 || sum_0 > bestSum {
                            bestType_0 = type_2;
                            bestSum = sum_0;
                        }
                        type_2 = type_2.wrapping_add(1);
                    }
                    prevline =
                        &*in_0.offset((y as u64).wrapping_mul(linebytes) as isize) as *const u8;
                    *out.offset((y as u64).wrapping_mul(linebytes.wrapping_add(1)) as isize) =
                        bestType_0 as u8;
                    x = 0;
                    while x as u64 != linebytes {
                        *out.offset(
                            (y as u64)
                                .wrapping_mul(linebytes.wrapping_add(1))
                                .wrapping_add(1)
                                .wrapping_add(x as u64) as isize,
                        ) = *(attempt_0[bestType_0 as usize]).offset(x as isize);
                        x = x.wrapping_add(1);
                    }
                    y = y.wrapping_add(1);
                }
            }
            type_2 = 0;
            while type_2 != 5 {
                lodepng_free(attempt_0[type_2 as usize] as *mut libc::c_void);
                type_2 = type_2.wrapping_add(1);
            }
        } else if strategy as u32 == LFS_PREDEFINED as u32 {
            y = 0;
            while y != h {
                let mut outindex_0 = 1u64.wrapping_add(linebytes).wrapping_mul(y as u64);
                let mut inindex_0 = linebytes.wrapping_mul(y as u64);
                let mut type_3 = *((*settings).predefined_filters).offset(y as isize);
                *out.offset(outindex_0 as isize) = type_3;
                filterScanline(
                    &mut *out.offset(outindex_0.wrapping_add(1) as isize),
                    &*in_0.offset(inindex_0 as isize),
                    prevline,
                    linebytes,
                    bytewidth,
                    type_3,
                );
                prevline = &*in_0.offset(inindex_0 as isize) as *const u8;
                y = y.wrapping_add(1);
            }
        } else if strategy as u32 == LFS_BRUTE_FORCE as u32 {
            let mut size: [u64; 5] = [0; 5];
            let mut attempt_1: [*mut u8; 5] = [0 as *mut u8; 5];
            let mut smallest_0 = 0;
            let mut type_4 = 0;
            let mut bestType_1 = 0;
            let mut dummy = 0 as *mut u8;
            let mut zlibsettings = LodePNGCompressSettings {
                btype: 0,
                use_lz77: 0,
                windowsize: 0,
                minmatch: 0,
                nicematch: 0,
                lazymatching: 0,
                custom_zlib: None,
                custom_deflate: None,
                custom_context: 0 as *const libc::c_void,
            };
            lodepng_memcpy(
                &mut zlibsettings as *mut LodePNGCompressSettings as *mut libc::c_void,
                &(*settings).zlibsettings as *const LodePNGCompressSettings as *const libc::c_void,
                ::std::mem::size_of::<LodePNGCompressSettings>() as u64,
            );
            zlibsettings.btype = 1;
            zlibsettings.custom_zlib = None;
            zlibsettings.custom_deflate = None;
            type_4 = 0;
            while type_4 != 5 {
                attempt_1[type_4 as usize] = lodepng_malloc(linebytes) as *mut u8;
                if (attempt_1[type_4 as usize]).is_null() {
                    error = 83;
                }
                type_4 = type_4.wrapping_add(1);
            }
            if error == 0 {
                y = 0;
                while y != h {
                    type_4 = 0;
                    while type_4 != 5 {
                        let mut testsize = linebytes as u32;
                        filterScanline(
                            attempt_1[type_4 as usize],
                            &*in_0.offset((y as u64).wrapping_mul(linebytes) as isize),
                            prevline,
                            linebytes,
                            bytewidth,
                            type_4 as u8,
                        );
                        size[type_4 as usize] = 0;
                        dummy = 0 as *mut u8;
                        zlib_compress(
                            &mut dummy,
                            &mut *size.as_mut_ptr().offset(type_4 as isize),
                            attempt_1[type_4 as usize],
                            testsize as u64,
                            &mut zlibsettings,
                        );
                        lodepng_free(dummy as *mut libc::c_void);
                        if type_4 == 0 || size[type_4 as usize] < smallest_0 {
                            bestType_1 = type_4;
                            smallest_0 = size[type_4 as usize];
                        }
                        type_4 = type_4.wrapping_add(1);
                    }
                    prevline =
                        &*in_0.offset((y as u64).wrapping_mul(linebytes) as isize) as *const u8;
                    *out.offset((y as u64).wrapping_mul(linebytes.wrapping_add(1)) as isize) =
                        bestType_1 as u8;
                    x = 0;
                    while x as u64 != linebytes {
                        *out.offset(
                            (y as u64)
                                .wrapping_mul(linebytes.wrapping_add(1))
                                .wrapping_add(1)
                                .wrapping_add(x as u64) as isize,
                        ) = *(attempt_1[bestType_1 as usize]).offset(x as isize);
                        x = x.wrapping_add(1);
                    }
                    y = y.wrapping_add(1);
                }
            }
            type_4 = 0;
            while type_4 != 5 {
                lodepng_free(attempt_1[type_4 as usize] as *mut libc::c_void);
                type_4 = type_4.wrapping_add(1);
            }
        } else {
            return 88;
        }
        return error;
    }
}

extern "C" fn addPaddingBits(
    mut out: *mut u8,
    mut in_0: *const u8,
    mut olinebits: u64,
    mut ilinebits: u64,
    mut h: u32,
) {
    unsafe {
        let mut y: u32 = 0;
        let mut diff = olinebits.wrapping_sub(ilinebits);
        let mut obp = 0;
        let mut ibp = 0;
        y = 0;
        while y != h {
            let mut x: u64 = 0;
            x = 0;
            while x < ilinebits {
                let mut bit = readBitFromReversedStream(&mut ibp, in_0);
                setBitOfReversedStream(&mut obp, out, bit);
                x = x.wrapping_add(1);
            }
            x = 0;
            while x != diff {
                setBitOfReversedStream(&mut obp, out, 0);
                x = x.wrapping_add(1);
            }
            y = y.wrapping_add(1);
        }
    }
}

extern "C" fn Adam7_interlace(
    mut out: *mut u8,
    mut in_0: *const u8,
    mut w: u32,
    mut h: u32,
    mut bpp: u32,
) {
    unsafe {
        let mut passw: [u32; 7] = [0; 7];
        let mut passh: [u32; 7] = [0; 7];
        let mut filter_passstart: [u64; 8] = [0; 8];
        let mut padded_passstart: [u64; 8] = [0; 8];
        let mut passstart: [u64; 8] = [0; 8];
        let mut i: u32 = 0;
        Adam7_getpassvalues(
            passw.as_mut_ptr(),
            passh.as_mut_ptr(),
            filter_passstart.as_mut_ptr(),
            padded_passstart.as_mut_ptr(),
            passstart.as_mut_ptr(),
            w,
            h,
            bpp,
        );
        if bpp >= 8 {
            i = 0;
            while i != 7 {
                let mut x: u32 = 0;
                let mut y: u32 = 0;
                let mut b: u32 = 0;
                let mut bytewidth = bpp.wrapping_div(8) as u64;
                y = 0;
                while y < passh[i as usize] {
                    x = 0;
                    while x < passw[i as usize] {
                        let mut pixelinstart = ((ADAM7_IY[i as usize])
                            .wrapping_add(y.wrapping_mul(ADAM7_DY[i as usize]))
                            .wrapping_mul(w)
                            .wrapping_add(ADAM7_IX[i as usize])
                            .wrapping_add(x.wrapping_mul(ADAM7_DX[i as usize]))
                            as u64)
                            .wrapping_mul(bytewidth);
                        let mut pixeloutstart = (passstart[i as usize]).wrapping_add(
                            (y.wrapping_mul(passw[i as usize]).wrapping_add(x) as u64)
                                .wrapping_mul(bytewidth),
                        );
                        b = 0;
                        while (b as u64) < bytewidth {
                            *out.offset(pixeloutstart.wrapping_add(b as u64) as isize) =
                                *in_0.offset(pixelinstart.wrapping_add(b as u64) as isize);
                            b = b.wrapping_add(1);
                        }
                        x = x.wrapping_add(1);
                    }
                    y = y.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
        } else {
            i = 0;
            while i != 7 {
                let mut x_0: u32 = 0;
                let mut y_0: u32 = 0;
                let mut b_0: u32 = 0;
                let mut ilinebits = bpp.wrapping_mul(passw[i as usize]);
                let mut olinebits = bpp.wrapping_mul(w);
                let mut obp: u64 = 0;
                let mut ibp: u64 = 0;
                y_0 = 0;
                while y_0 < passh[i as usize] {
                    x_0 = 0;
                    while x_0 < passw[i as usize] {
                        ibp = (ADAM7_IY[i as usize])
                            .wrapping_add(y_0.wrapping_mul(ADAM7_DY[i as usize]))
                            .wrapping_mul(olinebits)
                            .wrapping_add(
                                (ADAM7_IX[i as usize])
                                    .wrapping_add(x_0.wrapping_mul(ADAM7_DX[i as usize]))
                                    .wrapping_mul(bpp),
                            ) as u64;
                        obp = 8u64.wrapping_mul(passstart[i as usize]).wrapping_add(
                            y_0.wrapping_mul(ilinebits)
                                .wrapping_add(x_0.wrapping_mul(bpp))
                                as u64,
                        );
                        b_0 = 0;
                        while b_0 < bpp {
                            let mut bit = readBitFromReversedStream(&mut ibp, in_0);
                            setBitOfReversedStream(&mut obp, out, bit);
                            b_0 = b_0.wrapping_add(1);
                        }
                        x_0 = x_0.wrapping_add(1);
                    }
                    y_0 = y_0.wrapping_add(1);
                }
                i = i.wrapping_add(1);
            }
        };
    }
}

extern "C" fn preProcessScanlines(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut in_0: *const u8,
    mut w: u32,
    mut h: u32,
    mut info_png: *const LodePNGInfo,
    mut settings: *const LodePNGEncoderSettings,
) -> u32 {
    unsafe {
        let mut bpp = lodepng_get_bpp(&(*info_png).color);
        let mut error = 0;
        if (*info_png).interlace_method == 0 {
            *outsize = h
                .wrapping_add(h.wrapping_mul(w.wrapping_mul(bpp).wrapping_add(7).wrapping_div(8)))
                as u64;
            *out = lodepng_malloc(*outsize) as *mut u8;
            if (*out).is_null() && *outsize != 0 {
                error = 83;
            }
            if error == 0 {
                if bpp < 8
                    && w.wrapping_mul(bpp)
                        != w.wrapping_mul(bpp)
                            .wrapping_add(7)
                            .wrapping_div(8)
                            .wrapping_mul(8)
                {
                    let mut padded = lodepng_malloc(
                        h.wrapping_mul(w.wrapping_mul(bpp).wrapping_add(7).wrapping_div(8)) as u64,
                    ) as *mut u8;
                    if padded.is_null() {
                        error = 83;
                    }
                    if error == 0 {
                        addPaddingBits(
                            padded,
                            in_0,
                            w.wrapping_mul(bpp)
                                .wrapping_add(7)
                                .wrapping_div(8)
                                .wrapping_mul(8) as u64,
                            w.wrapping_mul(bpp) as u64,
                            h,
                        );
                        error = filter(*out, padded, w, h, &(*info_png).color, settings);
                    }
                    lodepng_free(padded as *mut libc::c_void);
                } else {
                    error = filter(*out, in_0, w, h, &(*info_png).color, settings);
                }
            }
        } else {
            let mut passw: [u32; 7] = [0; 7];
            let mut passh: [u32; 7] = [0; 7];
            let mut filter_passstart: [u64; 8] = [0; 8];
            let mut padded_passstart: [u64; 8] = [0; 8];
            let mut passstart: [u64; 8] = [0; 8];
            let mut adam7 = 0 as *mut u8;
            Adam7_getpassvalues(
                passw.as_mut_ptr(),
                passh.as_mut_ptr(),
                filter_passstart.as_mut_ptr(),
                padded_passstart.as_mut_ptr(),
                passstart.as_mut_ptr(),
                w,
                h,
                bpp,
            );
            *outsize = filter_passstart[7 as usize];
            *out = lodepng_malloc(*outsize) as *mut u8;
            if (*out).is_null() {
                error = 83;
            }
            adam7 = lodepng_malloc(passstart[7 as usize]) as *mut u8;
            if adam7.is_null() && passstart[7 as usize] != 0 {
                error = 83;
            }
            if error == 0 {
                let mut i: u32 = 0;
                Adam7_interlace(adam7, in_0, w, h, bpp);
                i = 0;
                while i != 7 {
                    if bpp < 8 {
                        let mut padded_0 = lodepng_malloc(
                            (padded_passstart[i.wrapping_add(1u32) as usize])
                                .wrapping_sub(padded_passstart[i as usize]),
                        ) as *mut u8;
                        if padded_0.is_null() {
                            error = 83;
                            break;
                        } else {
                            addPaddingBits(
                                padded_0,
                                &mut *adam7
                                    .offset(*passstart.as_mut_ptr().offset(i as isize) as isize),
                                (passw[i as usize])
                                    .wrapping_mul(bpp)
                                    .wrapping_add(7)
                                    .wrapping_div(8)
                                    .wrapping_mul(8) as u64,
                                (passw[i as usize]).wrapping_mul(bpp) as u64,
                                passh[i as usize],
                            );
                            error = filter(
                                &mut *(*out).offset(
                                    *filter_passstart.as_mut_ptr().offset(i as isize) as isize,
                                ),
                                padded_0,
                                passw[i as usize],
                                passh[i as usize],
                                &(*info_png).color,
                                settings,
                            );
                            lodepng_free(padded_0 as *mut libc::c_void);
                        }
                    } else {
                        error = filter(
                            &mut *(*out)
                                .offset(*filter_passstart.as_mut_ptr().offset(i as isize) as isize),
                            &mut *adam7
                                .offset(*padded_passstart.as_mut_ptr().offset(i as isize) as isize),
                            passw[i as usize],
                            passh[i as usize],
                            &(*info_png).color,
                            settings,
                        );
                    }
                    if error != 0 {
                        break;
                    }
                    i = i.wrapping_add(1);
                }
            }
            lodepng_free(adam7 as *mut libc::c_void);
        }
        return error;
    }
}

extern "C" fn addUnknownChunks(
    mut out: *mut ucvector,
    mut data: *mut u8,
    mut datasize: u64,
) -> u32 {
    unsafe {
        let mut inchunk = data;
        while (inchunk.offset_from(data) as u64) < datasize {
            let mut error = lodepng_chunk_append(&mut (*out).data, &mut (*out).size, inchunk);
            if error != 0 {
                return error;
            };
            (*out).allocsize = (*out).size;
            inchunk = lodepng_chunk_next(inchunk, data.offset(datasize as isize));
        }
        return 0;
    }
}

extern "C" fn isGrayICCProfile(mut profile: *const u8, mut size: u32) -> u32 {
    unsafe {
        if size < 20 {
            return 0;
        }
        return (*profile.offset(16 as isize) as i32 == 'G' as i32
            && *profile.offset(17 as isize) as i32 == 'R' as i32
            && *profile.offset(18 as isize) as i32 == 'A' as i32
            && *profile.offset(19 as isize) as i32 == 'Y' as i32) as u32;
    }
}

extern "C" fn isRGBICCProfile(mut profile: *const u8, mut size: u32) -> u32 {
    unsafe {
        if size < 20 {
            return 0;
        }
        return (*profile.offset(16 as isize) as i32 == 'R' as i32
            && *profile.offset(17 as isize) as i32 == 'G' as i32
            && *profile.offset(18 as isize) as i32 == 'B' as i32
            && *profile.offset(19 as isize) as i32 == ' ' as i32) as u32;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_encode(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut image: *const u8,
    mut w: u32,
    mut h: u32,
    mut state: *mut LodePNGState,
) -> u32 {
    unsafe {
        let mut current_block: u64;
        let mut data = 0 as *mut u8;
        let mut datasize = 0;
        let mut outv = ucvector_init(0 as *mut u8, 0);
        let mut info = LodePNGInfo {
            compression_method: 0,
            filter_method: 0,
            interlace_method: 0,
            color: LodePNGColorMode {
                colortype: LCT_GREY,
                bitdepth: 0,
                palette: 0 as *mut u8,
                palettesize: 0,
                key_defined: 0,
                key_r: 0,
                key_g: 0,
                key_b: 0,
            },
            background_defined: 0,
            background_r: 0,
            background_g: 0,
            background_b: 0,
            text_num: 0,
            text_keys: 0 as *mut *mut i8,
            text_strings: 0 as *mut *mut i8,
            itext_num: 0,
            itext_keys: 0 as *mut *mut i8,
            itext_langtags: 0 as *mut *mut i8,
            itext_transkeys: 0 as *mut *mut i8,
            itext_strings: 0 as *mut *mut i8,
            time_defined: 0,
            time: LodePNGTime {
                year: 0,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
                second: 0,
            },
            phys_defined: 0,
            phys_x: 0,
            phys_y: 0,
            phys_unit: 0,
            gama_defined: 0,
            gama_gamma: 0,
            chrm_defined: 0,
            chrm_white_x: 0,
            chrm_white_y: 0,
            chrm_red_x: 0,
            chrm_red_y: 0,
            chrm_green_x: 0,
            chrm_green_y: 0,
            chrm_blue_x: 0,
            chrm_blue_y: 0,
            srgb_defined: 0,
            srgb_intent: 0,
            iccp_defined: 0,
            iccp_name: 0 as *mut i8,
            iccp_profile: 0 as *mut u8,
            iccp_profile_size: 0,
            sbit_defined: 0,
            sbit_r: 0,
            sbit_g: 0,
            sbit_b: 0,
            sbit_a: 0,
            unknown_chunks_data: [0 as *mut u8; 3],
            unknown_chunks_size: [0; 3],
        };
        let mut info_png: *const LodePNGInfo = &mut (*state).info_png;
        let mut auto_color = LodePNGColorMode {
            colortype: LCT_GREY,
            bitdepth: 0,
            palette: 0 as *mut u8,
            palettesize: 0,
            key_defined: 0,
            key_r: 0,
            key_g: 0,
            key_b: 0,
        };
        lodepng_info_init(&mut info);
        lodepng_color_mode_init(&mut auto_color);
        *out = 0 as *mut u8;
        *outsize = 0;
        (*state).error = 0;
        if ((*info_png).color.colortype as u32 == LCT_PALETTE as u32
            || (*state).encoder.force_palette != 0)
            && ((*info_png).color.palettesize == 0 || (*info_png).color.palettesize > 256)
        {
            (*state).error = 68;
        } else if (*state).encoder.zlibsettings.btype > 2 {
            (*state).error = 61;
        } else if (*info_png).interlace_method > 1 {
            (*state).error = 71;
        } else {
            (*state).error =
                checkColorValidity((*info_png).color.colortype, (*info_png).color.bitdepth);
            if !((*state).error != 0) {
                (*state).error =
                    checkColorValidity((*state).info_raw.colortype, (*state).info_raw.bitdepth);
                if !((*state).error != 0) {
                    lodepng_info_copy(&mut info, &mut (*state).info_png);
                    if (*state).encoder.auto_convert != 0 {
                        let mut stats = LodePNGColorStats {
                            colored: 0,
                            key: 0,
                            key_r: 0,
                            key_g: 0,
                            key_b: 0,
                            alpha: 0,
                            numcolors: 0,
                            palette: [0; 1024],
                            bits: 0,
                            numpixels: 0,
                            allow_palette: 0,
                            allow_greyscale: 0,
                        };
                        let mut allow_convert = 1;
                        lodepng_color_stats_init(&mut stats);
                        if (*info_png).iccp_defined != 0
                            && isGrayICCProfile(
                                (*info_png).iccp_profile,
                                (*info_png).iccp_profile_size,
                            ) != 0
                        {
                            stats.allow_palette = 0;
                        }
                        if (*info_png).iccp_defined != 0
                            && isRGBICCProfile(
                                (*info_png).iccp_profile,
                                (*info_png).iccp_profile_size,
                            ) != 0
                        {
                            stats.allow_greyscale = 0;
                        };
                        (*state).error = lodepng_compute_color_stats(
                            &mut stats,
                            image,
                            w,
                            h,
                            &mut (*state).info_raw,
                        );
                        if (*state).error != 0 {
                            current_block = 11418055246242690407;
                        } else {
                            if (*info_png).background_defined != 0 {
                                let mut r = 0;
                                let mut g = 0;
                                let mut b = 0;
                                let mut mode16 = lodepng_color_mode_make(LCT_RGB, 16);
                                lodepng_convert_rgb(
                                    &mut r,
                                    &mut g,
                                    &mut b,
                                    (*info_png).background_r,
                                    (*info_png).background_g,
                                    (*info_png).background_b,
                                    &mut mode16,
                                    &(*info_png).color,
                                );
                                (*state).error =
                                    lodepng_color_stats_add(&mut stats, r, g, b, 65535);
                                if (*state).error != 0 {
                                    current_block = 11418055246242690407;
                                } else {
                                    current_block = 2604890879466389055;
                                }
                            } else {
                                current_block = 2604890879466389055;
                            }
                            match current_block {
                                11418055246242690407 => {}
                                _ => {
                                    (*state).error = auto_choose_color(
                                        &mut auto_color,
                                        &mut (*state).info_raw,
                                        &mut stats,
                                    );
                                    if (*state).error != 0 {
                                        current_block = 11418055246242690407;
                                    } else {
                                        if (*info_png).sbit_defined != 0 {
                                            let mut sbit_max = if (if (if (*info_png).sbit_r
                                                > (*info_png).sbit_g
                                            {
                                                (*info_png).sbit_r
                                            } else {
                                                (*info_png).sbit_g
                                            }) > (*info_png).sbit_b
                                            {
                                                (if (*info_png).sbit_r > (*info_png).sbit_g {
                                                    (*info_png).sbit_r
                                                } else {
                                                    (*info_png).sbit_g
                                                })
                                            } else {
                                                (*info_png).sbit_b
                                            }) > (*info_png).sbit_a
                                            {
                                                if (if (*info_png).sbit_r > (*info_png).sbit_g {
                                                    (*info_png).sbit_r
                                                } else {
                                                    (*info_png).sbit_g
                                                }) > (*info_png).sbit_b
                                                {
                                                    if (*info_png).sbit_r > (*info_png).sbit_g {
                                                        (*info_png).sbit_r
                                                    } else {
                                                        (*info_png).sbit_g
                                                    }
                                                } else {
                                                    (*info_png).sbit_b
                                                }
                                            } else {
                                                (*info_png).sbit_a
                                            };
                                            let mut equal = (((*info_png).sbit_g == 0
                                                || (*info_png).sbit_g == (*info_png).sbit_r)
                                                && ((*info_png).sbit_b == 0
                                                    || (*info_png).sbit_b == (*info_png).sbit_r)
                                                && ((*info_png).sbit_a == 0
                                                    || (*info_png).sbit_a == (*info_png).sbit_r))
                                                as u32;
                                            allow_convert = 0;
                                            if info.color.colortype as u32 == LCT_PALETTE as u32
                                                && auto_color.colortype as u32 == LCT_PALETTE as u32
                                            {
                                                allow_convert = 1;
                                            }
                                            if info.color.colortype as u32 == LCT_RGB as u32
                                                && auto_color.colortype as u32 == LCT_PALETTE as u32
                                                && sbit_max <= 8
                                            {
                                                allow_convert = 1;
                                            }
                                            if info.color.colortype as u32 == LCT_RGBA as u32
                                                && auto_color.colortype as u32 == LCT_PALETTE as u32
                                                && (*info_png).sbit_a == 8
                                                && sbit_max <= 8
                                            {
                                                allow_convert = 1;
                                            }
                                            if (info.color.colortype as u32 == LCT_RGB as u32
                                                || info.color.colortype as u32 == LCT_RGBA as u32)
                                                && info.color.bitdepth == 16
                                                && auto_color.colortype as u32
                                                    == info.color.colortype as u32
                                                && auto_color.bitdepth == 8
                                                && sbit_max <= 8
                                            {
                                                allow_convert = 1;
                                            }
                                            if info.color.colortype as u32 != LCT_PALETTE as u32
                                                && auto_color.colortype as u32 != LCT_PALETTE as u32
                                                && equal != 0
                                                && (*info_png).sbit_r == auto_color.bitdepth
                                            {
                                                allow_convert = 1;
                                            }
                                        }
                                        if (*state).encoder.force_palette != 0 {
                                            if info.color.colortype as u32 != LCT_GREY as u32
                                                && info.color.colortype as u32
                                                    != LCT_GREY_ALPHA as u32
                                                && (auto_color.colortype as u32 == LCT_GREY as u32
                                                    || auto_color.colortype as u32
                                                        == LCT_GREY_ALPHA as u32)
                                            {
                                                allow_convert = 0;
                                            }
                                        }
                                        if allow_convert != 0 {
                                            lodepng_color_mode_copy(
                                                &mut info.color,
                                                &mut auto_color,
                                            );
                                            if (*info_png).background_defined != 0 {
                                                if lodepng_convert_rgb(
                                                    &mut info.background_r,
                                                    &mut info.background_g,
                                                    &mut info.background_b,
                                                    (*info_png).background_r,
                                                    (*info_png).background_g,
                                                    (*info_png).background_b,
                                                    &mut info.color,
                                                    &(*info_png).color,
                                                ) != 0
                                                {
                                                    (*state).error = 104;
                                                    current_block = 11418055246242690407;
                                                } else {
                                                    current_block = 10067844863897285902;
                                                }
                                            } else {
                                                current_block = 10067844863897285902;
                                            }
                                        } else {
                                            current_block = 10067844863897285902;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        current_block = 10067844863897285902;
                    }
                    match current_block {
                        11418055246242690407 => {}
                        _ => {
                            if (*info_png).iccp_defined != 0 {
                                let mut gray_icc = isGrayICCProfile(
                                    (*info_png).iccp_profile,
                                    (*info_png).iccp_profile_size,
                                );
                                let mut rgb_icc = isRGBICCProfile(
                                    (*info_png).iccp_profile,
                                    (*info_png).iccp_profile_size,
                                );
                                let mut gray_png = (info.color.colortype as u32 == LCT_GREY as u32
                                    || info.color.colortype as u32 == LCT_GREY_ALPHA as u32)
                                    as u32;
                                if gray_icc == 0 && rgb_icc == 0 {
                                    (*state).error = 100;
                                    current_block = 11418055246242690407;
                                } else if gray_icc != gray_png {
                                    (*state).error = (if (*state).encoder.auto_convert != 0 {
                                        102
                                    } else {
                                        101
                                    }) as u32;
                                    current_block = 11418055246242690407;
                                } else {
                                    current_block = 10809827304263610514;
                                }
                            } else {
                                current_block = 10809827304263610514;
                            }
                            match current_block {
                                11418055246242690407 => {}
                                _ => {
                                    if lodepng_color_mode_equal(
                                        &mut (*state).info_raw,
                                        &mut info.color,
                                    ) == 0
                                    {
                                        let mut converted = 0 as *mut u8;
                                        let mut size = (w as u64)
                                            .wrapping_mul(h as u64)
                                            .wrapping_mul(lodepng_get_bpp(&mut info.color) as u64)
                                            .wrapping_add(7)
                                            .wrapping_div(8);
                                        converted = lodepng_malloc(size) as *mut u8;
                                        if converted.is_null() && size != 0 {
                                            (*state).error = 83;
                                        }
                                        if (*state).error == 0 {
                                            (*state).error = lodepng_convert(
                                                converted,
                                                image,
                                                &mut info.color,
                                                &mut (*state).info_raw,
                                                w,
                                                h,
                                            );
                                        }
                                        if (*state).error == 0 {
                                            (*state).error = preProcessScanlines(
                                                &mut data,
                                                &mut datasize,
                                                converted,
                                                w,
                                                h,
                                                &mut info,
                                                &mut (*state).encoder,
                                            );
                                        }
                                        lodepng_free(converted as *mut libc::c_void);
                                        if (*state).error != 0 {
                                            current_block = 11418055246242690407;
                                        } else {
                                            current_block = 11071260907632769126;
                                        }
                                    } else {
                                        (*state).error = preProcessScanlines(
                                            &mut data,
                                            &mut datasize,
                                            image,
                                            w,
                                            h,
                                            &mut info,
                                            &mut (*state).encoder,
                                        );
                                        if (*state).error != 0 {
                                            current_block = 11418055246242690407;
                                        } else {
                                            current_block = 11071260907632769126;
                                        }
                                    }
                                    match current_block {
                                        11418055246242690407 => {}
                                        _ => {
                                            let mut i: u64 = 0;
                                            (*state).error = writeSignature(&mut outv);
                                            if !((*state).error != 0) {
                                                (*state).error = addChunk_IHDR(
                                                    &mut outv,
                                                    w,
                                                    h,
                                                    info.color.colortype,
                                                    info.color.bitdepth,
                                                    info.interlace_method,
                                                );
                                                if !((*state).error != 0) {
                                                    if !(info.unknown_chunks_data[0 as usize])
                                                        .is_null()
                                                    {
                                                        (*state).error = addUnknownChunks(
                                                            &mut outv,
                                                            info.unknown_chunks_data[0 as usize],
                                                            info.unknown_chunks_size[0 as usize],
                                                        );
                                                        if (*state).error != 0 {
                                                            current_block = 11418055246242690407;
                                                        } else {
                                                            current_block = 4899250571165509867;
                                                        }
                                                    } else {
                                                        current_block = 4899250571165509867;
                                                    }
                                                    match current_block {
                                                        11418055246242690407 => {}
                                                        _ => {
                                                            if info.iccp_defined != 0 {
                                                                (*state).error = addChunk_iCCP(
                                                                    &mut outv,
                                                                    &mut info,
                                                                    &mut (*state)
                                                                        .encoder
                                                                        .zlibsettings,
                                                                );
                                                                if (*state).error != 0 {
                                                                    current_block =
                                                                        11418055246242690407;
                                                                } else {
                                                                    current_block =
                                                                        5265702136860997526;
                                                                }
                                                            } else {
                                                                current_block = 5265702136860997526;
                                                            }
                                                            match current_block {
                                                                11418055246242690407 => {}
                                                                _ => {
                                                                    if info.srgb_defined != 0 {
                                                                        (*state).error =
                                                                            addChunk_sRGB(
                                                                                &mut outv,
                                                                                &mut info,
                                                                            );
                                                                        if (*state).error != 0 {
                                                                            current_block = 11418055246242690407;
                                                                        } else {
                                                                            current_block =
                                                                                5409161009579131794;
                                                                        }
                                                                    } else {
                                                                        current_block =
                                                                            5409161009579131794;
                                                                    }
                                                                    match current_block {
                                                                        11418055246242690407 => {}
                                                                        _ => {
                                                                            if info.gama_defined
                                                                                != 0
                                                                            {
                                                                                (*state).error =
                                                                                    addChunk_gAMA(
                                                                                        &mut outv,
                                                                                        &mut info,
                                                                                    );
                                                                                if (*state).error
                                                                                    != 0
                                                                                {
                                                                                    current_block = 11418055246242690407;
                                                                                } else {
                                                                                    current_block = 10109057886293123569;
                                                                                }
                                                                            } else {
                                                                                current_block = 10109057886293123569;
                                                                            }
                                                                            match current_block {
                                                                                11418055246242690407 => {
                                                                                }
                                                                                _ => {
                                                                                    if info.chrm_defined != 0 {
                                                                                        (* state).error = addChunk_cHRM (& mut outv, & mut info);
                                                                                        if (* state).error != 0 {
                                                                                            current_block = 11418055246242690407;
                                                                                        } else {
                                                                                            current_block = 14612007084265645573;
                                                                                        }
                                                                                    } else {
                                                                                        current_block = 14612007084265645573;
                                                                                    }
                                                                                    match current_block {
                                                                                        11418055246242690407 => {
                                                                                        }
                                                                                        _ => {
                                                                                            if (* info_png).sbit_defined != 0 {
                                                                                                (* state).error = addChunk_sBIT (& mut outv, & mut info);
                                                                                                if (* state).error != 0 {
                                                                                                    current_block = 11418055246242690407;
                                                                                                } else {
                                                                                                    current_block = 12963528325254160332;
                                                                                                }
                                                                                            } else {
                                                                                                current_block = 12963528325254160332;
                                                                                            }
                                                                                            match current_block {
                                                                                                11418055246242690407 => {
                                                                                                }
                                                                                                _ => {
                                                                                                    if info.color.colortype as u32 == LCT_PALETTE as u32 {
                                                                                                        (* state).error = addChunk_PLTE (& mut outv, & mut info.color);
                                                                                                        if (* state).error != 0 {
                                                                                                            current_block = 11418055246242690407;
                                                                                                        } else {
                                                                                                            current_block = 15417752026496523887;
                                                                                                        }
                                                                                                    } else {
                                                                                                        current_block = 15417752026496523887;
                                                                                                    }
                                                                                                    match current_block {
                                                                                                        11418055246242690407 => {
                                                                                                        }
                                                                                                        _ => {
                                                                                                            if (* state).encoder.force_palette != 0 && (info.color.colortype as u32 == LCT_RGB as u32 || info.color.colortype as u32 == LCT_RGBA as u32) {
                                                                                                                (* state).error = addChunk_PLTE (& mut outv, & mut info.color);
                                                                                                                if (* state).error != 0 {
                                                                                                                    current_block = 11418055246242690407;
                                                                                                                } else {
                                                                                                                    current_block = 17736998403848444560;
                                                                                                                }
                                                                                                            } else {
                                                                                                                current_block = 17736998403848444560;
                                                                                                            }
                                                                                                            match current_block {
                                                                                                                11418055246242690407 => {
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    (* state).error = addChunk_tRNS (& mut outv, & mut info.color);
                                                                                                                    if ! ((* state).error != 0) {
                                                                                                                        if info.background_defined != 0 {
                                                                                                                            (* state).error = addChunk_bKGD (& mut outv, & mut info);
                                                                                                                            if (* state).error != 0 {
                                                                                                                                current_block = 11418055246242690407;
                                                                                                                            } else {
                                                                                                                                current_block = 18201902862271706575;
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            current_block = 18201902862271706575;
                                                                                                                        }
                                                                                                                        match current_block {
                                                                                                                            11418055246242690407 => {
                                                                                                                            }
                                                                                                                            _ => {
                                                                                                                                if info.phys_defined != 0 {
                                                                                                                                (* state).error = addChunk_pHYs (& mut outv, & mut info);
                                                                                                                                if (* state).error != 0 {
                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                } else {
                                                                                                                                current_block = 12608488225262500095;
                                                                                                                                }
                                                                                                                                } else {
                                                                                                                                current_block = 12608488225262500095;
                                                                                                                                }
                                                                                                                                match current_block {
                                                                                                                                11418055246242690407 => {
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                if ! (info.unknown_chunks_data [1 as usize]).is_null () {
                                                                                                                                (* state).error = addUnknownChunks (& mut outv, info.unknown_chunks_data [1 as usize], info.unknown_chunks_size [1 as usize]);
                                                                                                                                if (* state).error != 0 {
                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                } else {
                                                                                                                                current_block = 9343041660989783267;
                                                                                                                                }
                                                                                                                                } else {
                                                                                                                                current_block = 9343041660989783267;
                                                                                                                                }
                                                                                                                                match current_block {
                                                                                                                                11418055246242690407 => {
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                (* state).error = addChunk_IDAT (& mut outv, data, datasize, & mut (* state).encoder.zlibsettings);
                                                                                                                                if ! ((* state).error != 0) {
                                                                                                                                if info.time_defined != 0 {
                                                                                                                                (* state).error = addChunk_tIME (& mut outv, & mut info.time);
                                                                                                                                if (* state).error != 0 {
                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                } else {
                                                                                                                                current_block = 5710330377809666066;
                                                                                                                                }
                                                                                                                                } else {
                                                                                                                                current_block = 5710330377809666066;
                                                                                                                                }
                                                                                                                                match current_block {
                                                                                                                                11418055246242690407 => {
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                i = 0;
                                                                                                                                loop {
                                                                                                                                if ! (i != info.text_num) {
                                                                                                                                current_block = 5511877782510663281;
                                                                                                                                break;
                                                                                                                                }
                                                                                                                                if lodepng_strlen (* (info.text_keys).offset (i as isize)) > 79 {
                                                                                                                                (* state).error = 66;
                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                break;
                                                                                                                                } else if lodepng_strlen (* (info.text_keys).offset (i as isize),) < 1 {
                                                                                                                                (* state).error = 67;
                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                break;
                                                                                                                                } else {
                                                                                                                                if (* state).encoder.text_compression != 0 {
                                                                                                                                (* state).error = addChunk_zTXt (& mut outv, * (info.text_keys).offset (i as isize), * (info.text_strings).offset (i as isize),
                                                                                                                                & mut (* state).encoder.zlibsettings);
                                                                                                                                if (* state).error != 0 {
                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                break;
                                                                                                                                }
                                                                                                                                } else {
                                                                                                                                (* state).error = addChunk_tEXt (& mut outv, * (info.text_keys).offset (i as isize), * (info.text_strings).offset (i as isize))
                                                                                                                                ;
                                                                                                                                if (* state).error != 0 {
                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                break;
                                                                                                                                }
                                                                                                                                }
                                                                                                                                i = i.wrapping_add (1);
                                                                                                                                }
                                                                                                                                }
                                                                                                                                match current_block {
                                                                                                                                11418055246242690407 => {
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                if (* state).encoder.add_id != 0 {
                                                                                                                                let mut already_added_id_text = 0;
                                                                                                                                i = 0;
                                                                                                                                while i != info.text_num {
                                                                                                                                let mut k : * const i8 = * (info.text_keys).offset (i as isize);
                                                                                                                                if * k.offset (0 as isize) as i32 == 'L' as i32 && * k.offset (1 as isize) as i32 == 'o' as i32 && * k.offset (2 as isize) as
                                                                                                                                i32 == 'd' as i32 && * k.offset (3 as isize) as i32 == 'e' as i32 && * k.offset (4 as isize) as i32 == 'P' as i32 && * k.offset
                                                                                                                                (5 as isize) as i32 == 'N' as i32 && * k.offset (6 as isize) as i32 == 'G' as i32 && * k.offset (7 as isize) as i32 == '\0' as
                                                                                                                                i32 {
                                                                                                                                already_added_id_text = 1;
                                                                                                                                break;
                                                                                                                                } else {
                                                                                                                                i = i.wrapping_add (1);
                                                                                                                                }
                                                                                                                                }
                                                                                                                                if already_added_id_text == 0 {
                                                                                                                                (* state).error = addChunk_tEXt (& mut outv, b"LodePNG\0" as * const u8 as * const i8, LODEPNG_VERSION_STRING);
                                                                                                                                if (* state).error != 0 {
                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                } else {
                                                                                                                                current_block = 6880299496751257707;
                                                                                                                                }
                                                                                                                                } else {
                                                                                                                                current_block = 6880299496751257707;
                                                                                                                                }
                                                                                                                                } else {
                                                                                                                                current_block = 6880299496751257707;
                                                                                                                                }
                                                                                                                                match current_block {
                                                                                                                                11418055246242690407 => {
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                i = 0;
                                                                                                                                loop {
                                                                                                                                if ! (i != info.itext_num) {
                                                                                                                                current_block = 13796196565926322821;
                                                                                                                                break;
                                                                                                                                }
                                                                                                                                if lodepng_strlen (* (info.itext_keys).offset (i as isize)) > 79 {
                                                                                                                                (* state).error = 66;
                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                break;
                                                                                                                                } else if lodepng_strlen (* (info.itext_keys).offset (i as isize),) < 1 {
                                                                                                                                (* state).error = 67;
                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                break;
                                                                                                                                } else {
                                                                                                                                (* state).error = addChunk_iTXt (& mut outv, (* state).encoder.text_compression, * (info.itext_keys).offset (i as isize), * (
                                                                                                                                info.itext_langtags).offset (i as isize), * (info.itext_transkeys).offset (i as isize), * (info.itext_strings).offset (i as
                                                                                                                                isize), & mut (* state).encoder.zlibsettings);
                                                                                                                                if (* state).error != 0 {
                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                break;
                                                                                                                                }
                                                                                                                                i = i.wrapping_add (1);
                                                                                                                                }
                                                                                                                                }
                                                                                                                                match current_block {
                                                                                                                                11418055246242690407 => {
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                if ! (info.unknown_chunks_data [2 as usize]).is_null () {
                                                                                                                                (* state).error = addUnknownChunks (& mut outv, info.unknown_chunks_data [2 as usize], info.unknown_chunks_size [2 as usize]);
                                                                                                                                if (* state).error != 0 {
                                                                                                                                current_block = 11418055246242690407;
                                                                                                                                } else {
                                                                                                                                current_block = 11322929247169729670;
                                                                                                                                }
                                                                                                                                } else {
                                                                                                                                current_block = 11322929247169729670;
                                                                                                                                }
                                                                                                                                match current_block {
                                                                                                                                11418055246242690407 => {
                                                                                                                                }
                                                                                                                                _ => {
                                                                                                                                (* state).error = addChunk_IEND (& mut outv);
                                                                                                                                (* state).error != 0;
                                                                                                                                }
                                                                                                                                }
                                                                                                                                }
                                                                                                                                }
                                                                                                                                }
                                                                                                                                }
                                                                                                                                }
                                                                                                                                }
                                                                                                                                }
                                                                                                                                }
                                                                                                                                }
                                                                                                                                }
                                                                                                                                }
                                                                                                                                }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        lodepng_info_cleanup(&mut info);
        lodepng_free(data as *mut libc::c_void);
        lodepng_color_mode_cleanup(&mut auto_color);
        *out = outv.data;
        *outsize = outv.size;
        return (*state).error;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_encode_memory(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut image: *const u8,
    mut w: u32,
    mut h: u32,
    mut colortype: u32,
    mut bitdepth: u32,
) -> u32 {
    unsafe {
        let mut error: u32 = 0;
        let mut state = LodePNGState {
            decoder: LodePNGDecoderSettings {
                zlibsettings: LodePNGDecompressSettings {
                    ignore_adler32: 0,
                    ignore_nlen: 0,
                    max_output_size: 0,
                    custom_zlib: None,
                    custom_inflate: None,
                    custom_context: 0 as *const libc::c_void,
                },
                ignore_crc: 0,
                ignore_critical: 0,
                ignore_end: 0,
                color_convert: 0,
                read_text_chunks: 0,
                remember_unknown_chunks: 0,
                max_text_size: 0,
                max_icc_size: 0,
            },
            encoder: LodePNGEncoderSettings {
                zlibsettings: LodePNGCompressSettings {
                    btype: 0,
                    use_lz77: 0,
                    windowsize: 0,
                    minmatch: 0,
                    nicematch: 0,
                    lazymatching: 0,
                    custom_zlib: None,
                    custom_deflate: None,
                    custom_context: 0 as *const libc::c_void,
                },
                auto_convert: 0,
                filter_palette_zero: 0,
                filter_strategy: LFS_ZERO,
                predefined_filters: 0 as *const u8,
                force_palette: 0,
                add_id: 0,
                text_compression: 0,
            },
            info_raw: LodePNGColorMode {
                colortype: LCT_GREY,
                bitdepth: 0,
                palette: 0 as *mut u8,
                palettesize: 0,
                key_defined: 0,
                key_r: 0,
                key_g: 0,
                key_b: 0,
            },
            info_png: LodePNGInfo {
                compression_method: 0,
                filter_method: 0,
                interlace_method: 0,
                color: LodePNGColorMode {
                    colortype: LCT_GREY,
                    bitdepth: 0,
                    palette: 0 as *mut u8,
                    palettesize: 0,
                    key_defined: 0,
                    key_r: 0,
                    key_g: 0,
                    key_b: 0,
                },
                background_defined: 0,
                background_r: 0,
                background_g: 0,
                background_b: 0,
                text_num: 0,
                text_keys: 0 as *mut *mut i8,
                text_strings: 0 as *mut *mut i8,
                itext_num: 0,
                itext_keys: 0 as *mut *mut i8,
                itext_langtags: 0 as *mut *mut i8,
                itext_transkeys: 0 as *mut *mut i8,
                itext_strings: 0 as *mut *mut i8,
                time_defined: 0,
                time: LodePNGTime {
                    year: 0,
                    month: 0,
                    day: 0,
                    hour: 0,
                    minute: 0,
                    second: 0,
                },
                phys_defined: 0,
                phys_x: 0,
                phys_y: 0,
                phys_unit: 0,
                gama_defined: 0,
                gama_gamma: 0,
                chrm_defined: 0,
                chrm_white_x: 0,
                chrm_white_y: 0,
                chrm_red_x: 0,
                chrm_red_y: 0,
                chrm_green_x: 0,
                chrm_green_y: 0,
                chrm_blue_x: 0,
                chrm_blue_y: 0,
                srgb_defined: 0,
                srgb_intent: 0,
                iccp_defined: 0,
                iccp_name: 0 as *mut i8,
                iccp_profile: 0 as *mut u8,
                iccp_profile_size: 0,
                sbit_defined: 0,
                sbit_r: 0,
                sbit_g: 0,
                sbit_b: 0,
                sbit_a: 0,
                unknown_chunks_data: [0 as *mut u8; 3],
                unknown_chunks_size: [0; 3],
            },
            error: 0,
        };
        lodepng_state_init(&mut state);
        state.info_raw.colortype = colortype;
        state.info_raw.bitdepth = bitdepth;
        state.info_png.color.colortype = colortype;
        state.info_png.color.bitdepth = bitdepth;
        lodepng_encode(out, outsize, image, w, h, &mut state);
        error = state.error;
        lodepng_state_cleanup(&mut state);
        return error;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_encode32(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut image: *const u8,
    mut w: u32,
    mut h: u32,
) -> u32 {
    unsafe {
        return lodepng_encode_memory(out, outsize, image, w, h, LCT_RGBA, 8);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_encode24(
    mut out: *mut *mut u8,
    mut outsize: *mut u64,
    mut image: *const u8,
    mut w: u32,
    mut h: u32,
) -> u32 {
    unsafe {
        return lodepng_encode_memory(out, outsize, image, w, h, LCT_RGB, 8);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_encode_file(
    mut filename: *const i8,
    mut image: *const u8,
    mut w: u32,
    mut h: u32,
    mut colortype: u32,
    mut bitdepth: u32,
) -> u32 {
    unsafe {
        let mut buffer = 0 as *mut u8;
        let mut buffersize: u64 = 0;
        let mut error = lodepng_encode_memory(
            &mut buffer,
            &mut buffersize,
            image,
            w,
            h,
            colortype,
            bitdepth,
        );
        if error == 0 {
            error = lodepng_save_file(buffer, buffersize, filename);
        }
        lodepng_free(buffer as *mut libc::c_void);
        return error;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_encode32_file(
    mut filename: *const i8,
    mut image: *const u8,
    mut w: u32,
    mut h: u32,
) -> u32 {
    unsafe {
        return lodepng_encode_file(filename, image, w, h, LCT_RGBA, 8);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_encode24_file(
    mut filename: *const i8,
    mut image: *const u8,
    mut w: u32,
    mut h: u32,
) -> u32 {
    unsafe {
        return lodepng_encode_file(filename, image, w, h, LCT_RGB, 8);
    }
}

#[no_mangle]
pub extern "C" fn lodepng_encoder_settings_init(mut settings: *mut LodePNGEncoderSettings) {
    unsafe {
        lodepng_compress_settings_init(&mut (*settings).zlibsettings);
        (*settings).filter_palette_zero = 1;
        (*settings).filter_strategy = LFS_MINSUM;
        (*settings).auto_convert = 1;
        (*settings).force_palette = 0;
        let ref mut fresh190 = (*settings).predefined_filters;
        *fresh190 = 0 as *const u8;
        (*settings).add_id = 0;
        (*settings).text_compression = 1;
    }
}

#[no_mangle]
pub extern "C" fn lodepng_error_text(mut code: u32) -> *const i8 {
    match code {
        0 => return b"no error, everything went ok\0" as *const u8 as *const i8,
        1 => return b"nothing done yet\0" as *const u8 as *const i8,
        10 => {
            return b"end of input memory reached without huffman end code\0" as *const u8
                as *const i8;
        }
        11 => {
            return b"error in code tree made it jump outside of huffman tree\0" as *const u8
                as *const i8;
        }
        13 => {
            return b"problem while processing dynamic deflate block\0" as *const u8 as *const i8;
        }
        14 => {
            return b"problem while processing dynamic deflate block\0" as *const u8 as *const i8;
        }
        15 => {
            return b"problem while processing dynamic deflate block\0" as *const u8 as *const i8;
        }
        16 => {
            return b"invalid code while processing dynamic deflate block\0" as *const u8
                as *const i8;
        }
        17 => {
            return b"end of out buffer memory reached while inflating\0" as *const u8 as *const i8;
        }
        18 => {
            return b"invalid distance code while inflating\0" as *const u8 as *const i8;
        }
        19 => {
            return b"end of out buffer memory reached while inflating\0" as *const u8 as *const i8;
        }
        20 => {
            return b"invalid deflate block BTYPE encountered while decoding\0" as *const u8
                as *const i8;
        }
        21 => {
            return b"NLEN is not ones complement of LEN in a deflate block\0" as *const u8
                as *const i8;
        }
        22 => {
            return b"end of out buffer memory reached while inflating\0" as *const u8 as *const i8;
        }
        23 => {
            return b"end of in buffer memory reached while inflating\0" as *const u8 as *const i8;
        }
        24 => {
            return b"invalid FCHECK in zlib header\0" as *const u8 as *const i8;
        }
        25 => {
            return b"invalid compression method in zlib header\0" as *const u8 as *const i8;
        }
        26 => {
            return b"FDICT encountered in zlib header while it's not used for PNG\0" as *const u8
                as *const i8;
        }
        27 => {
            return b"PNG file is smaller than a PNG header\0" as *const u8 as *const i8;
        }
        28 => {
            return b"incorrect PNG signature, it's no PNG or corrupted\0" as *const u8
                as *const i8;
        }
        29 => {
            return b"first chunk is not the header chunk\0" as *const u8 as *const i8;
        }
        30 => {
            return b"chunk length too large, chunk broken off at end of file\0" as *const u8
                as *const i8;
        }
        31 => {
            return b"illegal PNG color type or bpp\0" as *const u8 as *const i8;
        }
        32 => {
            return b"illegal PNG compression method\0" as *const u8 as *const i8;
        }
        33 => return b"illegal PNG filter method\0" as *const u8 as *const i8,
        34 => {
            return b"illegal PNG interlace method\0" as *const u8 as *const i8;
        }
        35 => {
            return b"chunk length of a chunk is too large or the chunk too small\0" as *const u8
                as *const i8;
        }
        36 => {
            return b"illegal PNG filter type encountered\0" as *const u8 as *const i8;
        }
        37 => {
            return b"illegal bit depth for this color type given\0" as *const u8 as *const i8;
        }
        38 => {
            return b"the palette is too small or too big\0" as *const u8 as *const i8;
        }
        39 => {
            return b"tRNS chunk before PLTE or has more entries than palette size\0" as *const u8
                as *const i8;
        }
        40 => {
            return b"tRNS chunk has wrong size for grayscale image\0" as *const u8 as *const i8;
        }
        41 => {
            return b"tRNS chunk has wrong size for RGB image\0" as *const u8 as *const i8;
        }
        42 => {
            return b"tRNS chunk appeared while it was not allowed for this color type\0"
                as *const u8 as *const i8;
        }
        43 => {
            return b"bKGD chunk has wrong size for palette image\0" as *const u8 as *const i8;
        }
        44 => {
            return b"bKGD chunk has wrong size for grayscale image\0" as *const u8 as *const i8;
        }
        45 => {
            return b"bKGD chunk has wrong size for RGB image\0" as *const u8 as *const i8;
        }
        48 => {
            return b"empty input buffer given to decoder. Maybe caused by non-existing file?\0"
                as *const u8 as *const i8;
        }
        49 => {
            return b"jumped past memory while generating dynamic huffman tree\0" as *const u8
                as *const i8;
        }
        50 => {
            return b"jumped past memory while generating dynamic huffman tree\0" as *const u8
                as *const i8;
        }
        51 => {
            return b"jumped past memory while inflating huffman block\0" as *const u8 as *const i8;
        }
        52 => {
            return b"jumped past memory while inflating\0" as *const u8 as *const i8;
        }
        53 => return b"size of zlib data too small\0" as *const u8 as *const i8,
        54 => {
            return b"repeat symbol in tree while there was no value symbol yet\0" as *const u8
                as *const i8;
        }
        55 => {
            return b"jumped past tree while generating huffman tree\0" as *const u8 as *const i8;
        }
        56 => {
            return b"given output image colortype or bitdepth not supported for color conversion\0"
                as *const u8 as *const i8;
        }
        57 => {
            return b"invalid CRC encountered (checking CRC can be disabled)\0" as *const u8
                as *const i8;
        }
        58 => {
            return b"invalid ADLER32 encountered (checking ADLER32 can be disabled)\0" as *const u8
                as *const i8;
        }
        59 => {
            return b"requested color conversion not supported\0" as *const u8 as *const i8;
        }
        60 => {
            return b"invalid window size given in the settings of the encoder (must be 0-32768)\0"
                as *const u8 as *const i8;
        }
        61 => {
            return b"invalid BTYPE given in the settings of the encoder (only 0, 1 and 2 are allowed)\0" as * const u8 as * const i8;
        }
        62 => {
            return b"conversion from color to grayscale not supported\0" as *const u8 as *const i8;
        }
        63 => {
            return b"length of a chunk too long, max allowed for PNG is 2147483647 bytes per chunk\0" as * const u8 as * const i8;
        }
        64 => {
            return b"the length of the END symbol 256 in the Huffman tree is 0\0" as *const u8
                as *const i8;
        }
        66 => {
            return b"the length of a text chunk keyword given to the encoder is longer than the maximum of 79 bytes\0" as * const u8 as * const i8;
        }
        67 => {
            return b"the length of a text chunk keyword given to the encoder is smaller than the minimum of 1 byte\0" as * const u8 as * const i8;
        }
        68 => {
            return b"tried to encode a PLTE chunk with a palette that has less than 1 or more than 256 colors\0" as * const u8 as * const i8;
        }
        69 => {
            return b"unknown chunk type with 'critical' flag encountered by the decoder\0"
                as *const u8 as *const i8;
        }
        71 => {
            return b"invalid interlace mode given to encoder (must be 0 or 1)\0" as *const u8
                as *const i8;
        }
        72 => {
            return b"while decoding, invalid compression method encountering in zTXt or iTXt chunk (it must be 0)\0" as * const u8 as * const i8;
        }
        73 => return b"invalid tIME chunk size\0" as *const u8 as *const i8,
        74 => return b"invalid pHYs chunk size\0" as *const u8 as *const i8,
        75 => {
            return b"no null termination char found while decoding text chunk\0" as *const u8
                as *const i8;
        }
        76 => {
            return b"iTXt chunk too short to contain required bytes\0" as *const u8 as *const i8;
        }
        77 => {
            return b"integer overflow in buffer size\0" as *const u8 as *const i8;
        }
        78 => {
            return b"failed to open file for reading\0" as *const u8 as *const i8;
        }
        79 => {
            return b"failed to open file for writing\0" as *const u8 as *const i8;
        }
        80 => {
            return b"tried creating a tree of 0 symbols\0" as *const u8 as *const i8;
        }
        81 => {
            return b"lazy matching at pos 0 is impossible\0" as *const u8 as *const i8;
        }
        82 => {
            return b"color conversion to palette requested while a color isn't in palette, or index out of bounds\0" as * const u8 as * const i8;
        }
        83 => return b"memory allocation failed\0" as *const u8 as *const i8,
        84 => {
            return b"given image too small to contain all pixels to be encoded\0" as *const u8
                as *const i8;
        }
        86 => {
            return b"impossible offset in lz77 encoding (internal bug)\0" as *const u8
                as *const i8;
        }
        87 => {
            return b"must provide custom zlib function pointer if LODEPNG_COMPILE_ZLIB is not defined\0" as * const u8 as * const i8;
        }
        88 => {
            return b"invalid filter strategy given for LodePNGEncoderSettings.filter_strategy\0"
                as *const u8 as *const i8;
        }
        89 => {
            return b"text chunk keyword too short or long: must have size 1-79\0" as *const u8
                as *const i8;
        }
        90 => {
            return b"windowsize must be a power of two\0" as *const u8 as *const i8;
        }
        91 => {
            return b"invalid decompressed idat size\0" as *const u8 as *const i8;
        }
        92 => {
            return b"integer overflow due to too many pixels\0" as *const u8 as *const i8;
        }
        93 => {
            return b"zero width or height is invalid\0" as *const u8 as *const i8;
        }
        94 => {
            return b"header chunk must have a size of 13 bytes\0" as *const u8 as *const i8;
        }
        95 => {
            return b"integer overflow with combined idat chunk size\0" as *const u8 as *const i8;
        }
        96 => return b"invalid gAMA chunk size\0" as *const u8 as *const i8,
        97 => return b"invalid cHRM chunk size\0" as *const u8 as *const i8,
        98 => return b"invalid sRGB chunk size\0" as *const u8 as *const i8,
        99 => {
            return b"invalid sRGB rendering intent\0" as *const u8 as *const i8;
        }
        100 => {
            return b"invalid ICC profile color type, the PNG specification only allows RGB or GRAY\0" as * const u8 as * const i8;
        }
        101 => {
            return b"PNG specification does not allow RGB ICC profile on gray color types and vice versa\0" as * const u8 as * const i8;
        }
        102 => {
            return b"not allowed to set grayscale ICC profile with colored pixels by PNG specification\0" as * const u8 as * const i8;
        }
        103 => {
            return b"invalid palette index in bKGD chunk. Maybe it came before PLTE chunk?\0"
                as *const u8 as *const i8;
        }
        104 => {
            return b"invalid bKGD color while encoding (e.g. palette index out of range)\0"
                as *const u8 as *const i8;
        }
        105 => {
            return b"integer overflow of bitsize\0" as *const u8 as *const i8;
        }
        106 => {
            return b"PNG file must have PLTE chunk if color type is palette\0" as *const u8
                as *const i8;
        }
        107 => {
            return b"color convert from palette mode requested without setting the palette data in it\0" as * const u8 as * const i8;
        }
        108 => {
            return b"tried to add more than 256 values to a palette\0" as *const u8 as *const i8;
        }
        109 => {
            return b"tried to decompress zlib or deflate data larger than desired max_output_size\0"
                as *const u8 as *const i8;
        }
        110 => {
            return b"custom zlib or inflate decompression failed\0" as *const u8 as *const i8;
        }
        111 => {
            return b"custom zlib or deflate compression failed\0" as *const u8 as *const i8;
        }
        112 => {
            return b"compressed text unreasonably large\0" as *const u8 as *const i8;
        }
        113 => {
            return b"ICC profile unreasonably large\0" as *const u8 as *const i8;
        }
        114 => {
            return b"sBIT chunk has wrong size for the color type of the image\0" as *const u8
                as *const i8;
        }
        115 => return b"sBIT value out of range\0" as *const u8 as *const i8,
        _ => {}
    }
    return b"unknown error code\0" as *const u8 as *const i8;
}

extern "C" fn run_static_initializers() {
    unsafe {
        mask = (1u32 << 9).wrapping_sub(1);
    }
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
