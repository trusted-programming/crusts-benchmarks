use libc;
pub const BROTLI_NUM_TRANSFORM_TYPES: u32 = 23;
pub const BROTLI_TRANSFORM_SHIFT_ALL: u32 = 22;
pub const BROTLI_TRANSFORM_SHIFT_FIRST: u32 = 21;
pub const BROTLI_TRANSFORM_OMIT_FIRST_9: u32 = 20;
pub const BROTLI_TRANSFORM_OMIT_FIRST_8: u32 = 19;
pub const BROTLI_TRANSFORM_OMIT_FIRST_7: u32 = 18;
pub const BROTLI_TRANSFORM_OMIT_FIRST_6: u32 = 17;
pub const BROTLI_TRANSFORM_OMIT_FIRST_5: u32 = 16;
pub const BROTLI_TRANSFORM_OMIT_FIRST_4: u32 = 15;
pub const BROTLI_TRANSFORM_OMIT_FIRST_3: u32 = 14;
pub const BROTLI_TRANSFORM_OMIT_FIRST_2: u32 = 13;
pub const BROTLI_TRANSFORM_OMIT_FIRST_1: u32 = 12;
pub const BROTLI_TRANSFORM_UPPERCASE_ALL: u32 = 11;
pub const BROTLI_TRANSFORM_UPPERCASE_FIRST: u32 = 10;
pub const BROTLI_TRANSFORM_OMIT_LAST_9: u32 = 9;
pub const BROTLI_TRANSFORM_OMIT_LAST_8: u32 = 8;
pub const BROTLI_TRANSFORM_OMIT_LAST_7: u32 = 7;
pub const BROTLI_TRANSFORM_OMIT_LAST_6: u32 = 6;
pub const BROTLI_TRANSFORM_OMIT_LAST_5: u32 = 5;
pub const BROTLI_TRANSFORM_OMIT_LAST_4: u32 = 4;
pub const BROTLI_TRANSFORM_OMIT_LAST_3: u32 = 3;
pub const BROTLI_TRANSFORM_OMIT_LAST_2: u32 = 2;
pub const BROTLI_TRANSFORM_OMIT_LAST_1: u32 = 1;
pub const BROTLI_TRANSFORM_IDENTITY: u32 = 0;
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
static mut kPrefixSuffix: [i8; 217] = unsafe {
    * :: std :: mem :: transmute :: < & [u8; 217], & [i8; 217], > (
b"\x01 \x02, \x08 of the \x04 of \x02s \x01.\x05 and \x04 in \x01\"\x04 to \x02\">\x01\n\x02. \x01]\x05 for \x03 a \x06 that \x01'\x06 with \x06 from \x04 by \x01(\x06. The \x04 on \x04 as \x04 is \x04ing \x02\n\t\x01:\x03ed \x02=\"\x04 at \x03ly \x01,\x02='\x05.com/\x07. This \x05 not \x03er \x03al \x04ful \x04ive \x05less \x04est \x04ize \x02\xC2\xA0\x04ous \x05 the \x02e \0"
    ,)
};
static mut kPrefixSuffixMap: [u16; 50] = [
    0, 0x2, 0x5, 0xe, 0x13, 0x16, 0x18, 0x1e, 0x23, 0x25, 0x2a, 0x2d, 0x2f, 0x32, 0x34, 0x3a, 0x3e,
    0x45, 0x47, 0x4e, 0x55, 0x5a, 0x5c, 0x63, 0x68, 0x6d, 0x72, 0x77, 0x7a, 0x7c, 0x80, 0x83, 0x88,
    0x8c, 0x8e, 0x91, 0x97, 0x9f, 0xa5, 0xa9, 0xad, 0xb2, 0xb7, 0xbd, 0xc2, 0xc7, 0xca, 0xcf, 0xd5,
    0xd8,
];
static mut kTransformsData: [u8; 363] = [
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    49,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    0,
    0,
    BROTLI_TRANSFORM_IDENTITY as u8,
    0,
    49,
    BROTLI_TRANSFORM_OMIT_FIRST_1 as u8,
    49,
    49,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    0,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    47,
    0,
    BROTLI_TRANSFORM_IDENTITY as u8,
    49,
    4,
    BROTLI_TRANSFORM_IDENTITY as u8,
    0,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    3,
    49,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    49,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    6,
    49,
    BROTLI_TRANSFORM_OMIT_FIRST_2 as u8,
    49,
    49,
    BROTLI_TRANSFORM_OMIT_LAST_1 as u8,
    49,
    1,
    BROTLI_TRANSFORM_IDENTITY as u8,
    0,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    1,
    0,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    0,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    7,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    9,
    48,
    BROTLI_TRANSFORM_IDENTITY as u8,
    0,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    8,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    5,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    10,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    11,
    49,
    BROTLI_TRANSFORM_OMIT_LAST_3 as u8,
    49,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    13,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    14,
    49,
    BROTLI_TRANSFORM_OMIT_FIRST_3 as u8,
    49,
    49,
    BROTLI_TRANSFORM_OMIT_LAST_2 as u8,
    49,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    15,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    16,
    0,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    49,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    12,
    5,
    BROTLI_TRANSFORM_IDENTITY as u8,
    49,
    0,
    BROTLI_TRANSFORM_IDENTITY as u8,
    1,
    49,
    BROTLI_TRANSFORM_OMIT_FIRST_4 as u8,
    49,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    18,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    17,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    19,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    20,
    49,
    BROTLI_TRANSFORM_OMIT_FIRST_5 as u8,
    49,
    49,
    BROTLI_TRANSFORM_OMIT_FIRST_6 as u8,
    49,
    47,
    BROTLI_TRANSFORM_IDENTITY as u8,
    49,
    49,
    BROTLI_TRANSFORM_OMIT_LAST_4 as u8,
    49,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    22,
    49,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    49,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    23,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    24,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    25,
    49,
    BROTLI_TRANSFORM_OMIT_LAST_7 as u8,
    49,
    49,
    BROTLI_TRANSFORM_OMIT_LAST_1 as u8,
    26,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    27,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    28,
    0,
    BROTLI_TRANSFORM_IDENTITY as u8,
    12,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    29,
    49,
    BROTLI_TRANSFORM_OMIT_FIRST_9 as u8,
    49,
    49,
    BROTLI_TRANSFORM_OMIT_FIRST_7 as u8,
    49,
    49,
    BROTLI_TRANSFORM_OMIT_LAST_6 as u8,
    49,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    21,
    49,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    1,
    49,
    BROTLI_TRANSFORM_OMIT_LAST_8 as u8,
    49,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    31,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    32,
    47,
    BROTLI_TRANSFORM_IDENTITY as u8,
    3,
    49,
    BROTLI_TRANSFORM_OMIT_LAST_5 as u8,
    49,
    49,
    BROTLI_TRANSFORM_OMIT_LAST_9 as u8,
    49,
    0,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    1,
    49,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    8,
    5,
    BROTLI_TRANSFORM_IDENTITY as u8,
    21,
    49,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    0,
    49,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    10,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    30,
    0,
    BROTLI_TRANSFORM_IDENTITY as u8,
    5,
    35,
    BROTLI_TRANSFORM_IDENTITY as u8,
    49,
    47,
    BROTLI_TRANSFORM_IDENTITY as u8,
    2,
    49,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    17,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    36,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    33,
    5,
    BROTLI_TRANSFORM_IDENTITY as u8,
    0,
    49,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    21,
    49,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    5,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    37,
    0,
    BROTLI_TRANSFORM_IDENTITY as u8,
    30,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    38,
    0,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    0,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    39,
    0,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    49,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    34,
    49,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    8,
    49,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    12,
    0,
    BROTLI_TRANSFORM_IDENTITY as u8,
    21,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    40,
    0,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    12,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    41,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    42,
    49,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    17,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    43,
    0,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    5,
    49,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    10,
    0,
    BROTLI_TRANSFORM_IDENTITY as u8,
    34,
    49,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    33,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    44,
    49,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    5,
    45,
    BROTLI_TRANSFORM_IDENTITY as u8,
    49,
    0,
    BROTLI_TRANSFORM_IDENTITY as u8,
    33,
    49,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    30,
    49,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    30,
    49,
    BROTLI_TRANSFORM_IDENTITY as u8,
    46,
    49,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    1,
    49,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    34,
    0,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    33,
    0,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    30,
    0,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    1,
    49,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    33,
    49,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    21,
    49,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    12,
    0,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    5,
    49,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    34,
    0,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    12,
    0,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    30,
    0,
    BROTLI_TRANSFORM_UPPERCASE_ALL as u8,
    34,
    0,
    BROTLI_TRANSFORM_UPPERCASE_FIRST as u8,
    34,
];
static mut kBrotliTransforms: BrotliTransforms = BrotliTransforms {
    prefix_suffix_size: 0,
    prefix_suffix: 0 as *const u8,
    prefix_suffix_map: 0 as *const u16,
    num_transforms: 0,
    transforms: 0 as *const u8,
    params: 0 as *const u8,
    cutOffTransforms: [0; 10],
};
#[no_mangle]
pub extern "C" fn BrotliGetTransforms() -> *const BrotliTransforms {
    unsafe {
        return &kBrotliTransforms;
    }
}

extern "C" fn ToUpperCase(mut p: *mut u8) -> i32 {
    unsafe {
        if (*p.offset(0 as isize) as i32) < 0xc0 {
            if *p.offset(0 as isize) as i32 >= 'a' as i32
                && *p.offset(0 as isize) as i32 <= 'z' as i32
            {
                let ref mut fresh0 = *p.offset(0 as isize);
                *fresh0 = (*fresh0 as i32 ^ 32i32) as u8;
            }
            return 1;
        }
        if (*p.offset(0 as isize) as i32) < 0xe0 {
            let ref mut fresh1 = *p.offset(1 as isize);
            *fresh1 = (*fresh1 as i32 ^ 32i32) as u8;
            return 2;
        }
        let ref mut fresh2 = *p.offset(2 as isize);
        *fresh2 = (*fresh2 as i32 ^ 5i32) as u8;
        return 3;
    }
}

extern "C" fn Shift(mut word: *mut u8, mut word_len: i32, mut parameter: u16) -> i32 {
    unsafe {
        let mut scalar = (parameter as u32 & 0x7fffu32)
            .wrapping_add(0x1000000u32.wrapping_sub(parameter as u32 & 0x8000));
        if (*word.offset(0 as isize) as i32) < 0x80 {
            scalar = (scalar as u32).wrapping_add(*word.offset(0 as isize) as u32) as u32;
            *word.offset(0 as isize) = (scalar & 0x7fu32) as u8;
            return 1;
        } else {
            if (*word.offset(0 as isize) as i32) < 0xc0 {
                return 1;
            } else {
                if (*word.offset(0 as isize) as i32) < 0xe0 {
                    if word_len < 2 {
                        return 1;
                    }
                    scalar = (scalar as u32).wrapping_add(
                        *word.offset(1 as isize) as u32 & 0x3f
                            | (*word.offset(0 as isize) as u32 & 0x1f) << 6,
                    ) as u32;
                    *word.offset(0 as isize) = (0xc0 | scalar >> 6 & 0x1fu32) as u8;
                    *word.offset(1 as isize) =
                        ((*word.offset(1 as isize) as i32 & 0xc0i32) as u32 | scalar & 0x3f) as u8;
                    return 2;
                } else {
                    if (*word.offset(0 as isize) as i32) < 0xf0 {
                        if word_len < 3 {
                            return word_len;
                        }
                        scalar = (scalar as u32).wrapping_add(
                            *word.offset(2 as isize) as u32 & 0x3f
                                | (*word.offset(1 as isize) as u32 & 0x3f) << 6
                                | (*word.offset(0 as isize) as u32 & 0xf) << 12,
                        ) as u32;
                        *word.offset(0 as isize) = (0xe0 | scalar >> 12 & 0xfu32) as u8;
                        *word.offset(1 as isize) =
                            ((*word.offset(1 as isize) as i32 & 0xc0i32) as u32
                                | scalar >> 6 & 0x3f) as u8;
                        *word.offset(2 as isize) =
                            ((*word.offset(2 as isize) as i32 & 0xc0i32) as u32 | scalar & 0x3f)
                                as u8;
                        return 3;
                    } else {
                        if (*word.offset(0 as isize) as i32) < 0xf8 {
                            if word_len < 4 {
                                return word_len;
                            }
                            scalar = (scalar as u32).wrapping_add(
                                *word.offset(3 as isize) as u32 & 0x3f
                                    | (*word.offset(2 as isize) as u32 & 0x3f) << 6
                                    | (*word.offset(1 as isize) as u32 & 0x3f) << 12
                                    | (*word.offset(0 as isize) as u32 & 0x7) << 18,
                            ) as u32;
                            *word.offset(0 as isize) = (0xf0 | scalar >> 18 & 0x7u32) as u8;
                            *word.offset(1 as isize) =
                                ((*word.offset(1 as isize) as i32 & 0xc0i32) as u32
                                    | scalar >> 12 & 0x3f) as u8;
                            *word.offset(2 as isize) =
                                ((*word.offset(2 as isize) as i32 & 0xc0i32) as u32
                                    | scalar >> 6 & 0x3f) as u8;
                            *word.offset(3 as isize) = ((*word.offset(3 as isize) as i32 & 0xc0i32)
                                as u32
                                | scalar & 0x3f)
                                as u8;
                            return 4;
                        }
                    }
                }
            }
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn BrotliTransformDictionaryWord(
    mut dst: *mut u8,
    mut word: *const u8,
    mut len: i32,
    mut transforms: *const BrotliTransforms,
    mut transform_idx: i32,
) -> i32 {
    unsafe {
        let mut idx = 0;
        let mut prefix: *const u8 =
            &*((*transforms).prefix_suffix).offset(*((*transforms).prefix_suffix_map).offset(
                *((*transforms).transforms).offset((transform_idx * 3 + 0i32) as isize) as isize,
            ) as isize) as *const u8;
        let mut type_0 = *((*transforms).transforms).offset((transform_idx * 3 + 1i32) as isize);
        let mut suffix: *const u8 =
            &*((*transforms).prefix_suffix).offset(*((*transforms).prefix_suffix_map).offset(
                *((*transforms).transforms).offset((transform_idx * 3 + 2i32) as isize) as isize,
            ) as isize) as *const u8;
        let fresh3 = prefix;
        prefix = prefix.offset(1);
        let mut prefix_len = *fresh3 as i32;
        loop {
            let fresh4 = prefix_len;
            prefix_len = prefix_len - 1;
            if !(fresh4 != 0) {
                break;
            }
            let fresh5 = prefix;
            prefix = prefix.offset(1);
            let fresh6 = idx;
            idx = idx + 1;
            *dst.offset(fresh6 as isize) = *fresh5;
        }
        let t = type_0 as i32;
        let mut i = 0;
        if t <= BROTLI_TRANSFORM_OMIT_LAST_9 as i32 {
            len -= t;
        } else if t >= BROTLI_TRANSFORM_OMIT_FIRST_1 as i32
            && t <= BROTLI_TRANSFORM_OMIT_FIRST_9 as i32
        {
            let mut skip = t - (BROTLI_TRANSFORM_OMIT_FIRST_1 as i32 - 1);
            word = word.offset(skip as isize);
            len -= skip;
        }
        while i < len {
            let fresh7 = i;
            i = i + 1;
            let fresh8 = idx;
            idx = idx + 1;
            *dst.offset(fresh8 as isize) = *word.offset(fresh7 as isize);
        }
        if t == BROTLI_TRANSFORM_UPPERCASE_FIRST as i32 {
            ToUpperCase(&mut *dst.offset((idx - len) as isize));
        } else if t == BROTLI_TRANSFORM_UPPERCASE_ALL as i32 {
            let mut uppercase: *mut u8 = &mut *dst.offset((idx - len) as isize) as *mut u8;
            while len > 0 {
                let mut step = ToUpperCase(uppercase);
                uppercase = uppercase.offset(step as isize);
                len -= step;
            }
        } else if t == BROTLI_TRANSFORM_SHIFT_FIRST as i32 {
            let mut param = (*((*transforms).params).offset((transform_idx * 2i32) as isize) as i32
                + ((*((*transforms).params).offset((transform_idx * 2 + 1i32) as isize) as i32)
                    << 8u32)) as u16;
            Shift(&mut *dst.offset((idx - len) as isize), len, param);
        } else if t == BROTLI_TRANSFORM_SHIFT_ALL as i32 {
            let mut param_0 = (*((*transforms).params).offset((transform_idx * 2i32) as isize)
                as i32
                + ((*((*transforms).params).offset((transform_idx * 2 + 1i32) as isize) as i32)
                    << 8u32)) as u16;
            let mut shift: *mut u8 = &mut *dst.offset((idx - len) as isize) as *mut u8;
            while len > 0 {
                let mut step_0 = Shift(shift, len, param_0);
                shift = shift.offset(step_0 as isize);
                len -= step_0;
            }
        }
        let fresh9 = suffix;
        suffix = suffix.offset(1);
        let mut suffix_len = *fresh9 as i32;
        loop {
            let fresh10 = suffix_len;
            suffix_len = suffix_len - 1;
            if !(fresh10 != 0) {
                break;
            }
            let fresh11 = suffix;
            suffix = suffix.offset(1);
            let fresh12 = idx;
            idx = idx + 1;
            *dst.offset(fresh12 as isize) = *fresh11;
        }
        return idx;
    }
}

extern "C" fn run_static_initializers() {
    unsafe {
        kBrotliTransforms = {
            let mut init = BrotliTransforms {
                prefix_suffix_size: ::std::mem::size_of::<[i8; 217]>() as u16,
                prefix_suffix: kPrefixSuffix.as_ptr() as *const u8,
                prefix_suffix_map: kPrefixSuffixMap.as_ptr(),
                num_transforms: (::std::mem::size_of::<[u8; 363]>() as u64)
                    .wrapping_div(3u64.wrapping_mul(::std::mem::size_of::<u8>() as u64))
                    as u32,
                transforms: kTransformsData.as_ptr(),
                params: 0 as *const u8,
                cutOffTransforms: [0, 12, 27, 23, 42, 63, 56, 48, 59, 64],
            };
            init
        };
    }
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
