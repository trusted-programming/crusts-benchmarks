use libc;
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
pub const BROTLI_TRANSFORM_UPPERCASE_FIRST: u32 = 10;
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
static mut kDictHashMul32: u32 = 0x1e35a7bd;
static mut kDictNumBits: i32 = 15;
#[inline(always)]
extern "C" fn brotli_min_uint32_t(mut a: u32, mut b: u32) -> u32 {
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
extern "C" fn Hash(mut data: *const u8) -> u32 {
    unsafe {
        let mut h =
            (BrotliUnalignedRead32(data as *const libc::c_void)).wrapping_mul(kDictHashMul32);
        return h >> 32 - kDictNumBits;
    }
}

#[inline(always)]
extern "C" fn AddMatch(mut distance: u64, mut len: u64, mut len_code: u64, mut matches: *mut u32) {
    unsafe {
        let mut match_0 = (distance << 5i32).wrapping_add(len_code) as u32;
        *matches.offset(len as isize) = brotli_min_uint32_t(*matches.offset(len as isize), match_0);
    }
}

#[inline(always)]
extern "C" fn DictMatchLength(
    mut dictionary: *const BrotliDictionary,
    mut data: *const u8,
    mut id: u64,
    mut len: u64,
    mut maxlen: u64,
) -> u64 {
    unsafe {
        let offset = ((*dictionary).offsets_by_length[len as usize] as u64)
            .wrapping_add(len.wrapping_mul(id));
        return FindMatchLengthWithLimit(
            &*((*dictionary).data).offset(offset as isize),
            data,
            brotli_min_size_t(len, maxlen),
        );
    }
}

#[inline(always)]
extern "C" fn IsMatch(
    mut dictionary: *const BrotliDictionary,
    mut w: DictWord,
    mut data: *const u8,
    mut max_length: u64,
) -> i32 {
    unsafe {
        if w.len as u64 > max_length {
            return 0;
        } else {
            let offset = ((*dictionary).offsets_by_length[w.len as usize] as u64)
                .wrapping_add((w.len as u64).wrapping_mul(w.idx as u64));
            let mut dict: *const u8 = &*((*dictionary).data).offset(offset as isize) as *const u8;
            if w.transform as i32 == 0 {
                return if FindMatchLengthWithLimit(dict, data, w.len as u64) == w.len as u64 {
                    1
                } else {
                    0
                };
            } else if w.transform as i32 == 10 {
                return if *dict.offset(0 as isize) as i32 >= 'a' as i32
                    && *dict.offset(0 as isize) as i32 <= 'z' as i32
                    && *dict.offset(0 as isize) as i32 ^ 32 == *data.offset(0 as isize) as i32
                    && FindMatchLengthWithLimit(
                        &*dict.offset(1 as isize),
                        &*data.offset(1 as isize),
                        (w.len as u32).wrapping_sub(1) as u64,
                    ) == (w.len as u32).wrapping_sub(1) as u64
                {
                    1
                } else {
                    0
                };
            } else {
                let mut i: u64 = 0;
                i = 0;
                while i < w.len as u64 {
                    if *dict.offset(i as isize) as i32 >= 'a' as i32
                        && *dict.offset(i as isize) as i32 <= 'z' as i32
                    {
                        if *dict.offset(i as isize) as i32 ^ 32 != *data.offset(i as isize) as i32 {
                            return 0;
                        }
                    } else if *dict.offset(i as isize) as i32 != *data.offset(i as isize) as i32 {
                        return 0;
                    }
                    i = i.wrapping_add(1);
                }
                return 1;
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliFindAllStaticDictionaryMatches(
    mut dictionary: *const BrotliEncoderDictionary,
    mut data: *const u8,
    mut min_length: u64,
    mut max_length: u64,
    mut matches: *mut u32,
) -> i32 {
    unsafe {
        let mut has_found_match = 0;
        let mut offset = *((*dictionary).buckets).offset(Hash(data) as isize) as u64;
        let mut end = (offset == 0) as i32;
        while end == 0 {
            let fresh0 = offset;
            offset = offset.wrapping_add(1);
            let mut w = *((*dictionary).dict_words).offset(fresh0 as isize);
            let l = (w.len as i32 & 0x1fi32) as u64;
            let n = 1 << (*(*dictionary).words).size_bits_by_length[l as usize] as i32;
            let id = w.idx as u64;
            end = (w.len as i32 & 0x80 != 0) as i32;
            w.len = l as u8;
            if w.transform as i32 == 0 {
                let matchlen = DictMatchLength((*dictionary).words, data, id, l, max_length);
                let mut s = 0 as *const u8;
                let mut minlen: u64 = 0;
                let mut maxlen: u64 = 0;
                let mut len: u64 = 0;
                if matchlen == l {
                    AddMatch(id, l, l, matches);
                    has_found_match = 1;
                }
                if matchlen >= l.wrapping_sub(1) {
                    AddMatch(
                        id.wrapping_add(12u64.wrapping_mul(n)),
                        l.wrapping_sub(1),
                        l,
                        matches,
                    );
                    if l.wrapping_add(2) < max_length
                        && *data.offset(l.wrapping_sub(1) as isize) as i32 == 'i' as i32
                        && *data.offset(l as isize) as i32 == 'n' as i32
                        && *data.offset(l.wrapping_add(1) as isize) as i32 == 'g' as i32
                        && *data.offset(l.wrapping_add(2) as isize) as i32 == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add(49u64.wrapping_mul(n)),
                            l.wrapping_add(3),
                            l,
                            matches,
                        );
                    }
                    has_found_match = 1;
                }
                minlen = min_length;
                if l > 9 {
                    minlen = brotli_max_size_t(minlen, l.wrapping_sub(9));
                }
                maxlen = brotli_min_size_t(matchlen, l.wrapping_sub(2));
                len = minlen;
                while len <= maxlen {
                    let mut cut = l.wrapping_sub(len);
                    let mut transform_id = (cut << 2i32)
                        .wrapping_add((*dictionary).cutoffTransforms >> cut.wrapping_mul(6) & 0x3f);
                    AddMatch(
                        id.wrapping_add(transform_id.wrapping_mul(n)),
                        len,
                        l,
                        matches,
                    );
                    has_found_match = 1;
                    len = len.wrapping_add(1);
                }
                if matchlen < l || l.wrapping_add(6) >= max_length {
                    continue;
                }
                s = &*data.offset(l as isize) as *const u8;
                if *s.offset(0 as isize) as i32 == ' ' as i32 {
                    AddMatch(id.wrapping_add(n), l.wrapping_add(1), l, matches);
                    if *s.offset(1 as isize) as i32 == 'a' as i32 {
                        if *s.offset(2 as isize) as i32 == ' ' as i32 {
                            AddMatch(
                                id.wrapping_add(28u64.wrapping_mul(n)),
                                l.wrapping_add(3),
                                l,
                                matches,
                            );
                        } else if *s.offset(2 as isize) as i32 == 's' as i32 {
                            if *s.offset(3 as isize) as i32 == ' ' as i32 {
                                AddMatch(
                                    id.wrapping_add(46u64.wrapping_mul(n)),
                                    l.wrapping_add(4),
                                    l,
                                    matches,
                                );
                            }
                        } else if *s.offset(2 as isize) as i32 == 't' as i32 {
                            if *s.offset(3 as isize) as i32 == ' ' as i32 {
                                AddMatch(
                                    id.wrapping_add(60u64.wrapping_mul(n)),
                                    l.wrapping_add(4),
                                    l,
                                    matches,
                                );
                            }
                        } else if *s.offset(2 as isize) as i32 == 'n' as i32 {
                            if *s.offset(3 as isize) as i32 == 'd' as i32
                                && *s.offset(4 as isize) as i32 == ' ' as i32
                            {
                                AddMatch(
                                    id.wrapping_add(10u64.wrapping_mul(n)),
                                    l.wrapping_add(5),
                                    l,
                                    matches,
                                );
                            }
                        }
                    } else if *s.offset(1 as isize) as i32 == 'b' as i32 {
                        if *s.offset(2 as isize) as i32 == 'y' as i32
                            && *s.offset(3 as isize) as i32 == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add(38u64.wrapping_mul(n)),
                                l.wrapping_add(4),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(1 as isize) as i32 == 'i' as i32 {
                        if *s.offset(2 as isize) as i32 == 'n' as i32 {
                            if *s.offset(3 as isize) as i32 == ' ' as i32 {
                                AddMatch(
                                    id.wrapping_add(16u64.wrapping_mul(n)),
                                    l.wrapping_add(4),
                                    l,
                                    matches,
                                );
                            }
                        } else if *s.offset(2 as isize) as i32 == 's' as i32 {
                            if *s.offset(3 as isize) as i32 == ' ' as i32 {
                                AddMatch(
                                    id.wrapping_add(47u64.wrapping_mul(n)),
                                    l.wrapping_add(4),
                                    l,
                                    matches,
                                );
                            }
                        }
                    } else if *s.offset(1 as isize) as i32 == 'f' as i32 {
                        if *s.offset(2 as isize) as i32 == 'o' as i32 {
                            if *s.offset(3 as isize) as i32 == 'r' as i32
                                && *s.offset(4 as isize) as i32 == ' ' as i32
                            {
                                AddMatch(
                                    id.wrapping_add(25u64.wrapping_mul(n)),
                                    l.wrapping_add(5),
                                    l,
                                    matches,
                                );
                            }
                        } else if *s.offset(2 as isize) as i32 == 'r' as i32 {
                            if *s.offset(3 as isize) as i32 == 'o' as i32
                                && *s.offset(4 as isize) as i32 == 'm' as i32
                                && *s.offset(5 as isize) as i32 == ' ' as i32
                            {
                                AddMatch(
                                    id.wrapping_add(37u64.wrapping_mul(n)),
                                    l.wrapping_add(6),
                                    l,
                                    matches,
                                );
                            }
                        }
                    } else if *s.offset(1 as isize) as i32 == 'o' as i32 {
                        if *s.offset(2 as isize) as i32 == 'f' as i32 {
                            if *s.offset(3 as isize) as i32 == ' ' as i32 {
                                AddMatch(
                                    id.wrapping_add(8u64.wrapping_mul(n)),
                                    l.wrapping_add(4),
                                    l,
                                    matches,
                                );
                            }
                        } else if *s.offset(2 as isize) as i32 == 'n' as i32 {
                            if *s.offset(3 as isize) as i32 == ' ' as i32 {
                                AddMatch(
                                    id.wrapping_add(45u64.wrapping_mul(n)),
                                    l.wrapping_add(4),
                                    l,
                                    matches,
                                );
                            }
                        }
                    } else if *s.offset(1 as isize) as i32 == 'n' as i32 {
                        if *s.offset(2 as isize) as i32 == 'o' as i32
                            && *s.offset(3 as isize) as i32 == 't' as i32
                            && *s.offset(4 as isize) as i32 == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add(80u64.wrapping_mul(n)),
                                l.wrapping_add(5),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(1 as isize) as i32 == 't' as i32 {
                        if *s.offset(2 as isize) as i32 == 'h' as i32 {
                            if *s.offset(3 as isize) as i32 == 'e' as i32 {
                                if *s.offset(4 as isize) as i32 == ' ' as i32 {
                                    AddMatch(
                                        id.wrapping_add(5u64.wrapping_mul(n)),
                                        l.wrapping_add(5),
                                        l,
                                        matches,
                                    );
                                }
                            } else if *s.offset(3 as isize) as i32 == 'a' as i32 {
                                if *s.offset(4 as isize) as i32 == 't' as i32
                                    && *s.offset(5 as isize) as i32 == ' ' as i32
                                {
                                    AddMatch(
                                        id.wrapping_add(29u64.wrapping_mul(n)),
                                        l.wrapping_add(6),
                                        l,
                                        matches,
                                    );
                                }
                            }
                        } else if *s.offset(2 as isize) as i32 == 'o' as i32 {
                            if *s.offset(3 as isize) as i32 == ' ' as i32 {
                                AddMatch(
                                    id.wrapping_add(17u64.wrapping_mul(n)),
                                    l.wrapping_add(4),
                                    l,
                                    matches,
                                );
                            }
                        }
                    } else if *s.offset(1 as isize) as i32 == 'w' as i32 {
                        if *s.offset(2 as isize) as i32 == 'i' as i32
                            && *s.offset(3 as isize) as i32 == 't' as i32
                            && *s.offset(4 as isize) as i32 == 'h' as i32
                            && *s.offset(5 as isize) as i32 == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add(35u64.wrapping_mul(n)),
                                l.wrapping_add(6),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(0 as isize) as i32 == '"' as i32 {
                    AddMatch(
                        id.wrapping_add(19u64.wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if *s.offset(1 as isize) as i32 == '>' as i32 {
                        AddMatch(
                            id.wrapping_add(21u64.wrapping_mul(n)),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(0 as isize) as i32 == '.' as i32 {
                    AddMatch(
                        id.wrapping_add(20u64.wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if *s.offset(1 as isize) as i32 == ' ' as i32 {
                        AddMatch(
                            id.wrapping_add(31u64.wrapping_mul(n)),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                        if *s.offset(2 as isize) as i32 == 'T' as i32
                            && *s.offset(3 as isize) as i32 == 'h' as i32
                        {
                            if *s.offset(4 as isize) as i32 == 'e' as i32 {
                                if *s.offset(5 as isize) as i32 == ' ' as i32 {
                                    AddMatch(
                                        id.wrapping_add(43u64.wrapping_mul(n)),
                                        l.wrapping_add(6),
                                        l,
                                        matches,
                                    );
                                }
                            } else if *s.offset(4 as isize) as i32 == 'i' as i32 {
                                if *s.offset(5 as isize) as i32 == 's' as i32
                                    && *s.offset(6 as isize) as i32 == ' ' as i32
                                {
                                    AddMatch(
                                        id.wrapping_add(75u64.wrapping_mul(n)),
                                        l.wrapping_add(7),
                                        l,
                                        matches,
                                    );
                                }
                            }
                        }
                    }
                } else if *s.offset(0 as isize) as i32 == ',' as i32 {
                    AddMatch(
                        id.wrapping_add(76u64.wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if *s.offset(1 as isize) as i32 == ' ' as i32 {
                        AddMatch(
                            id.wrapping_add(14u64.wrapping_mul(n)),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(0 as isize) as i32 == '\n' as i32 {
                    AddMatch(
                        id.wrapping_add(22u64.wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if *s.offset(1 as isize) as i32 == '\t' as i32 {
                        AddMatch(
                            id.wrapping_add(50u64.wrapping_mul(n)),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(0 as isize) as i32 == ']' as i32 {
                    AddMatch(
                        id.wrapping_add(24u64.wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if *s.offset(0 as isize) as i32 == '\'' as i32 {
                    AddMatch(
                        id.wrapping_add(36u64.wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if *s.offset(0 as isize) as i32 == ':' as i32 {
                    AddMatch(
                        id.wrapping_add(51u64.wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if *s.offset(0 as isize) as i32 == '(' as i32 {
                    AddMatch(
                        id.wrapping_add(57u64.wrapping_mul(n)),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if *s.offset(0 as isize) as i32 == '=' as i32 {
                    if *s.offset(1 as isize) as i32 == '"' as i32 {
                        AddMatch(
                            id.wrapping_add(70u64.wrapping_mul(n)),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    } else if *s.offset(1 as isize) as i32 == '\'' as i32 {
                        AddMatch(
                            id.wrapping_add(86u64.wrapping_mul(n)),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(0 as isize) as i32 == 'a' as i32 {
                    if *s.offset(1 as isize) as i32 == 'l' as i32
                        && *s.offset(2 as isize) as i32 == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add(84u64.wrapping_mul(n)),
                            l.wrapping_add(3),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(0 as isize) as i32 == 'e' as i32 {
                    if *s.offset(1 as isize) as i32 == 'd' as i32 {
                        if *s.offset(2 as isize) as i32 == ' ' as i32 {
                            AddMatch(
                                id.wrapping_add(53u64.wrapping_mul(n)),
                                l.wrapping_add(3),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(1 as isize) as i32 == 'r' as i32 {
                        if *s.offset(2 as isize) as i32 == ' ' as i32 {
                            AddMatch(
                                id.wrapping_add(82u64.wrapping_mul(n)),
                                l.wrapping_add(3),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(1 as isize) as i32 == 's' as i32 {
                        if *s.offset(2 as isize) as i32 == 't' as i32
                            && *s.offset(3 as isize) as i32 == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add(95u64.wrapping_mul(n)),
                                l.wrapping_add(4),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(0 as isize) as i32 == 'f' as i32 {
                    if *s.offset(1 as isize) as i32 == 'u' as i32
                        && *s.offset(2 as isize) as i32 == 'l' as i32
                        && *s.offset(3 as isize) as i32 == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add(90u64.wrapping_mul(n)),
                            l.wrapping_add(4),
                            l,
                            matches,
                        );
                    }
                } else if *s.offset(0 as isize) as i32 == 'i' as i32 {
                    if *s.offset(1 as isize) as i32 == 'v' as i32 {
                        if *s.offset(2 as isize) as i32 == 'e' as i32
                            && *s.offset(3 as isize) as i32 == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add(92u64.wrapping_mul(n)),
                                l.wrapping_add(4),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(1 as isize) as i32 == 'z' as i32 {
                        if *s.offset(2 as isize) as i32 == 'e' as i32
                            && *s.offset(3 as isize) as i32 == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add(100u64.wrapping_mul(n)),
                                l.wrapping_add(4),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(0 as isize) as i32 == 'l' as i32 {
                    if *s.offset(1 as isize) as i32 == 'e' as i32 {
                        if *s.offset(2 as isize) as i32 == 's' as i32
                            && *s.offset(3 as isize) as i32 == 's' as i32
                            && *s.offset(4 as isize) as i32 == ' ' as i32
                        {
                            AddMatch(
                                id.wrapping_add(93u64.wrapping_mul(n)),
                                l.wrapping_add(5),
                                l,
                                matches,
                            );
                        }
                    } else if *s.offset(1 as isize) as i32 == 'y' as i32 {
                        if *s.offset(2 as isize) as i32 == ' ' as i32 {
                            AddMatch(
                                id.wrapping_add(61u64.wrapping_mul(n)),
                                l.wrapping_add(3),
                                l,
                                matches,
                            );
                        }
                    }
                } else if *s.offset(0 as isize) as i32 == 'o' as i32 {
                    if *s.offset(1 as isize) as i32 == 'u' as i32
                        && *s.offset(2 as isize) as i32 == 's' as i32
                        && *s.offset(3 as isize) as i32 == ' ' as i32
                    {
                        AddMatch(
                            id.wrapping_add(106u64.wrapping_mul(n)),
                            l.wrapping_add(4),
                            l,
                            matches,
                        );
                    }
                }
            } else {
                let is_all_caps = if w.transform as i32 != BROTLI_TRANSFORM_UPPERCASE_FIRST as i32 {
                    1
                } else {
                    0
                };
                let mut s_0 = 0 as *const u8;
                if IsMatch((*dictionary).words, w, data, max_length) == 0 {
                    continue;
                }
                AddMatch(
                    id.wrapping_add(
                        ((if is_all_caps != 0 { 44i32 } else { 9 }) as u64).wrapping_mul(n),
                    ),
                    l,
                    l,
                    matches,
                );
                has_found_match = 1;
                if l.wrapping_add(1) >= max_length {
                    continue;
                }
                s_0 = &*data.offset(l as isize) as *const u8;
                if *s_0.offset(0 as isize) as i32 == ' ' as i32 {
                    AddMatch(
                        id.wrapping_add(
                            ((if is_all_caps != 0 { 68i32 } else { 4 }) as u64).wrapping_mul(n),
                        ),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if *s_0.offset(0 as isize) as i32 == '"' as i32 {
                    AddMatch(
                        id.wrapping_add(
                            ((if is_all_caps != 0 { 87i32 } else { 66 }) as u64).wrapping_mul(n),
                        ),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if *s_0.offset(1 as isize) as i32 == '>' as i32 {
                        AddMatch(
                            id.wrapping_add(
                                ((if is_all_caps != 0 { 97i32 } else { 69 }) as u64)
                                    .wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if *s_0.offset(0 as isize) as i32 == '.' as i32 {
                    AddMatch(
                        id.wrapping_add(
                            ((if is_all_caps != 0 { 101i32 } else { 79 }) as u64).wrapping_mul(n),
                        ),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if *s_0.offset(1 as isize) as i32 == ' ' as i32 {
                        AddMatch(
                            id.wrapping_add(
                                ((if is_all_caps != 0 { 114i32 } else { 88 }) as u64)
                                    .wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if *s_0.offset(0 as isize) as i32 == ',' as i32 {
                    AddMatch(
                        id.wrapping_add(
                            ((if is_all_caps != 0 { 112i32 } else { 99 }) as u64).wrapping_mul(n),
                        ),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                    if *s_0.offset(1 as isize) as i32 == ' ' as i32 {
                        AddMatch(
                            id.wrapping_add(
                                ((if is_all_caps != 0 { 107i32 } else { 58 }) as u64)
                                    .wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                } else if *s_0.offset(0 as isize) as i32 == '\'' as i32 {
                    AddMatch(
                        id.wrapping_add(
                            ((if is_all_caps != 0 { 94i32 } else { 74 }) as u64).wrapping_mul(n),
                        ),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if *s_0.offset(0 as isize) as i32 == '(' as i32 {
                    AddMatch(
                        id.wrapping_add(
                            ((if is_all_caps != 0 { 113i32 } else { 78 }) as u64).wrapping_mul(n),
                        ),
                        l.wrapping_add(1),
                        l,
                        matches,
                    );
                } else if *s_0.offset(0 as isize) as i32 == '=' as i32 {
                    if *s_0.offset(1 as isize) as i32 == '"' as i32 {
                        AddMatch(
                            id.wrapping_add(
                                ((if is_all_caps != 0 { 105i32 } else { 104 }) as u64)
                                    .wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    } else if *s_0.offset(1 as isize) as i32 == '\'' as i32 {
                        AddMatch(
                            id.wrapping_add(
                                ((if is_all_caps != 0 { 116i32 } else { 108 }) as u64)
                                    .wrapping_mul(n),
                            ),
                            l.wrapping_add(2),
                            l,
                            matches,
                        );
                    }
                }
            }
        }
        if max_length >= 5
            && (*data.offset(0 as isize) as i32 == ' ' as i32
                || *data.offset(0 as isize) as i32 == '.' as i32)
        {
            let mut is_space = if *data.offset(0 as isize) as i32 == ' ' as i32 {
                1
            } else {
                0
            };
            let mut offset_0 =
                *((*dictionary).buckets).offset(Hash(&*data.offset(1 as isize)) as isize) as u64;
            let mut end_0 = (offset_0 == 0) as i32;
            while end_0 == 0 {
                let fresh1 = offset_0;
                offset_0 = offset_0.wrapping_add(1);
                let mut w_0 = *((*dictionary).dict_words).offset(fresh1 as isize);
                let l_0 = (w_0.len as i32 & 0x1fi32) as u64;
                let n_0 = 1 << (*(*dictionary).words).size_bits_by_length[l_0 as usize] as i32;
                let id_0 = w_0.idx as u64;
                end_0 = (w_0.len as i32 & 0x80 != 0) as i32;
                w_0.len = l_0 as u8;
                if w_0.transform as i32 == 0 {
                    let mut s_1 = 0 as *const u8;
                    if IsMatch(
                        (*dictionary).words,
                        w_0,
                        &*data.offset(1 as isize),
                        max_length.wrapping_sub(1),
                    ) == 0
                    {
                        continue;
                    }
                    AddMatch(
                        id_0.wrapping_add(
                            ((if is_space != 0 { 6i32 } else { 32 }) as u64).wrapping_mul(n_0),
                        ),
                        l_0.wrapping_add(1),
                        l_0,
                        matches,
                    );
                    has_found_match = 1;
                    if l_0.wrapping_add(2) >= max_length {
                        continue;
                    }
                    s_1 = &*data.offset(l_0.wrapping_add(1) as isize) as *const u8;
                    if *s_1.offset(0 as isize) as i32 == ' ' as i32 {
                        AddMatch(
                            id_0.wrapping_add(
                                ((if is_space != 0 { 2i32 } else { 77 }) as u64).wrapping_mul(n_0),
                            ),
                            l_0.wrapping_add(2),
                            l_0,
                            matches,
                        );
                    } else if *s_1.offset(0 as isize) as i32 == '(' as i32 {
                        AddMatch(
                            id_0.wrapping_add(
                                ((if is_space != 0 { 89i32 } else { 67 }) as u64).wrapping_mul(n_0),
                            ),
                            l_0.wrapping_add(2),
                            l_0,
                            matches,
                        );
                    } else if is_space != 0 {
                        if *s_1.offset(0 as isize) as i32 == ',' as i32 {
                            AddMatch(
                                id_0.wrapping_add(103u64.wrapping_mul(n_0)),
                                l_0.wrapping_add(2),
                                l_0,
                                matches,
                            );
                            if *s_1.offset(1 as isize) as i32 == ' ' as i32 {
                                AddMatch(
                                    id_0.wrapping_add(33u64.wrapping_mul(n_0)),
                                    l_0.wrapping_add(3),
                                    l_0,
                                    matches,
                                );
                            }
                        } else if *s_1.offset(0 as isize) as i32 == '.' as i32 {
                            AddMatch(
                                id_0.wrapping_add(71u64.wrapping_mul(n_0)),
                                l_0.wrapping_add(2),
                                l_0,
                                matches,
                            );
                            if *s_1.offset(1 as isize) as i32 == ' ' as i32 {
                                AddMatch(
                                    id_0.wrapping_add(52u64.wrapping_mul(n_0)),
                                    l_0.wrapping_add(3),
                                    l_0,
                                    matches,
                                );
                            }
                        } else if *s_1.offset(0 as isize) as i32 == '=' as i32 {
                            if *s_1.offset(1 as isize) as i32 == '"' as i32 {
                                AddMatch(
                                    id_0.wrapping_add(81u64.wrapping_mul(n_0)),
                                    l_0.wrapping_add(3),
                                    l_0,
                                    matches,
                                );
                            } else if *s_1.offset(1 as isize) as i32 == '\'' as i32 {
                                AddMatch(
                                    id_0.wrapping_add(98u64.wrapping_mul(n_0)),
                                    l_0.wrapping_add(3),
                                    l_0,
                                    matches,
                                );
                            }
                        }
                    }
                } else {
                    if !(is_space != 0) {
                        continue;
                    }
                    let is_all_caps_0 =
                        if w_0.transform as i32 != BROTLI_TRANSFORM_UPPERCASE_FIRST as i32 {
                            1
                        } else {
                            0
                        };
                    let mut s_2 = 0 as *const u8;
                    if IsMatch(
                        (*dictionary).words,
                        w_0,
                        &*data.offset(1 as isize),
                        max_length.wrapping_sub(1),
                    ) == 0
                    {
                        continue;
                    }
                    AddMatch(
                        id_0.wrapping_add(
                            ((if is_all_caps_0 != 0 { 85i32 } else { 30 }) as u64)
                                .wrapping_mul(n_0),
                        ),
                        l_0.wrapping_add(1),
                        l_0,
                        matches,
                    );
                    has_found_match = 1;
                    if l_0.wrapping_add(2) >= max_length {
                        continue;
                    }
                    s_2 = &*data.offset(l_0.wrapping_add(1) as isize) as *const u8;
                    if *s_2.offset(0 as isize) as i32 == ' ' as i32 {
                        AddMatch(
                            id_0.wrapping_add(
                                ((if is_all_caps_0 != 0 { 83i32 } else { 15 }) as u64)
                                    .wrapping_mul(n_0),
                            ),
                            l_0.wrapping_add(2),
                            l_0,
                            matches,
                        );
                    } else if *s_2.offset(0 as isize) as i32 == ',' as i32 {
                        if is_all_caps_0 == 0 {
                            AddMatch(
                                id_0.wrapping_add(109u64.wrapping_mul(n_0)),
                                l_0.wrapping_add(2),
                                l_0,
                                matches,
                            );
                        }
                        if *s_2.offset(1 as isize) as i32 == ' ' as i32 {
                            AddMatch(
                                id_0.wrapping_add(
                                    ((if is_all_caps_0 != 0 { 111i32 } else { 65 }) as u64)
                                        .wrapping_mul(n_0),
                                ),
                                l_0.wrapping_add(3),
                                l_0,
                                matches,
                            );
                        }
                    } else if *s_2.offset(0 as isize) as i32 == '.' as i32 {
                        AddMatch(
                            id_0.wrapping_add(
                                ((if is_all_caps_0 != 0 { 115i32 } else { 96 }) as u64)
                                    .wrapping_mul(n_0),
                            ),
                            l_0.wrapping_add(2),
                            l_0,
                            matches,
                        );
                        if *s_2.offset(1 as isize) as i32 == ' ' as i32 {
                            AddMatch(
                                id_0.wrapping_add(
                                    ((if is_all_caps_0 != 0 { 117i32 } else { 91 }) as u64)
                                        .wrapping_mul(n_0),
                                ),
                                l_0.wrapping_add(3),
                                l_0,
                                matches,
                            );
                        }
                    } else if *s_2.offset(0 as isize) as i32 == '=' as i32 {
                        if *s_2.offset(1 as isize) as i32 == '"' as i32 {
                            AddMatch(
                                id_0.wrapping_add(
                                    ((if is_all_caps_0 != 0 { 110i32 } else { 118 }) as u64)
                                        .wrapping_mul(n_0),
                                ),
                                l_0.wrapping_add(3),
                                l_0,
                                matches,
                            );
                        } else if *s_2.offset(1 as isize) as i32 == '\'' as i32 {
                            AddMatch(
                                id_0.wrapping_add(
                                    ((if is_all_caps_0 != 0 { 119i32 } else { 120 }) as u64)
                                        .wrapping_mul(n_0),
                                ),
                                l_0.wrapping_add(3),
                                l_0,
                                matches,
                            );
                        }
                    }
                }
            }
        }
        if max_length >= 6 {
            if *data.offset(1 as isize) as i32 == ' ' as i32
                && (*data.offset(0 as isize) as i32 == 'e' as i32
                    || *data.offset(0 as isize) as i32 == 's' as i32
                    || *data.offset(0 as isize) as i32 == ',' as i32)
                || *data.offset(0 as isize) as i32 == 0xc2
                    && *data.offset(1 as isize) as i32 == 0xa0
            {
                let mut offset_1 = *((*dictionary).buckets)
                    .offset(Hash(&*data.offset(2 as isize)) as isize)
                    as u64;
                let mut end_1 = (offset_1 == 0) as i32;
                while end_1 == 0 {
                    let fresh2 = offset_1;
                    offset_1 = offset_1.wrapping_add(1);
                    let mut w_1 = *((*dictionary).dict_words).offset(fresh2 as isize);
                    let l_1 = (w_1.len as i32 & 0x1fi32) as u64;
                    let n_1 = 1 << (*(*dictionary).words).size_bits_by_length[l_1 as usize] as i32;
                    let id_1 = w_1.idx as u64;
                    end_1 = (w_1.len as i32 & 0x80 != 0) as i32;
                    w_1.len = l_1 as u8;
                    if w_1.transform as i32 == 0
                        && IsMatch(
                            (*dictionary).words,
                            w_1,
                            &*data.offset(2 as isize),
                            max_length.wrapping_sub(2),
                        ) != 0
                    {
                        if *data.offset(0 as isize) as i32 == 0xc2 {
                            AddMatch(
                                id_1.wrapping_add(102u64.wrapping_mul(n_1)),
                                l_1.wrapping_add(2),
                                l_1,
                                matches,
                            );
                            has_found_match = 1;
                        } else if l_1.wrapping_add(2) < max_length
                            && *data.offset(l_1.wrapping_add(2) as isize) as i32 == ' ' as i32
                        {
                            let mut t = (if *data.offset(0 as isize) as i32 == 'e' as i32 {
                                18
                            } else if *data.offset(0 as isize) as i32 == 's' as i32 {
                                7
                            } else {
                                13
                            }) as u64;
                            AddMatch(
                                id_1.wrapping_add(t.wrapping_mul(n_1)),
                                l_1.wrapping_add(3),
                                l_1,
                                matches,
                            );
                            has_found_match = 1;
                        }
                    }
                }
            }
        }
        if max_length >= 9 {
            if *data.offset(0 as isize) as i32 == ' ' as i32
                && *data.offset(1 as isize) as i32 == 't' as i32
                && *data.offset(2 as isize) as i32 == 'h' as i32
                && *data.offset(3 as isize) as i32 == 'e' as i32
                && *data.offset(4 as isize) as i32 == ' ' as i32
                || *data.offset(0 as isize) as i32 == '.' as i32
                    && *data.offset(1 as isize) as i32 == 'c' as i32
                    && *data.offset(2 as isize) as i32 == 'o' as i32
                    && *data.offset(3 as isize) as i32 == 'm' as i32
                    && *data.offset(4 as isize) as i32 == '/' as i32
            {
                let mut offset_2 = *((*dictionary).buckets)
                    .offset(Hash(&*data.offset(5 as isize)) as isize)
                    as u64;
                let mut end_2 = (offset_2 == 0) as i32;
                while end_2 == 0 {
                    let fresh3 = offset_2;
                    offset_2 = offset_2.wrapping_add(1);
                    let mut w_2 = *((*dictionary).dict_words).offset(fresh3 as isize);
                    let l_2 = (w_2.len as i32 & 0x1fi32) as u64;
                    let n_2 = 1 << (*(*dictionary).words).size_bits_by_length[l_2 as usize] as i32;
                    let id_2 = w_2.idx as u64;
                    end_2 = (w_2.len as i32 & 0x80 != 0) as i32;
                    w_2.len = l_2 as u8;
                    if w_2.transform as i32 == 0
                        && IsMatch(
                            (*dictionary).words,
                            w_2,
                            &*data.offset(5 as isize),
                            max_length.wrapping_sub(5),
                        ) != 0
                    {
                        AddMatch(
                            id_2.wrapping_add(
                                ((if *data.offset(0 as isize) as i32 == ' ' as i32 {
                                    41i32
                                } else {
                                    72
                                }) as u64)
                                    .wrapping_mul(n_2),
                            ),
                            l_2.wrapping_add(5),
                            l_2,
                            matches,
                        );
                        has_found_match = 1;
                        if l_2.wrapping_add(5) < max_length {
                            let mut s_3: *const u8 =
                                &*data.offset(l_2.wrapping_add(5) as isize) as *const u8;
                            if *data.offset(0 as isize) as i32 == ' ' as i32 {
                                if l_2.wrapping_add(8) < max_length
                                    && *s_3.offset(0 as isize) as i32 == ' ' as i32
                                    && *s_3.offset(1 as isize) as i32 == 'o' as i32
                                    && *s_3.offset(2 as isize) as i32 == 'f' as i32
                                    && *s_3.offset(3 as isize) as i32 == ' ' as i32
                                {
                                    AddMatch(
                                        id_2.wrapping_add(62u64.wrapping_mul(n_2)),
                                        l_2.wrapping_add(9),
                                        l_2,
                                        matches,
                                    );
                                    if l_2.wrapping_add(12) < max_length
                                        && *s_3.offset(4 as isize) as i32 == 't' as i32
                                        && *s_3.offset(5 as isize) as i32 == 'h' as i32
                                        && *s_3.offset(6 as isize) as i32 == 'e' as i32
                                        && *s_3.offset(7 as isize) as i32 == ' ' as i32
                                    {
                                        AddMatch(
                                            id_2.wrapping_add(73u64.wrapping_mul(n_2)),
                                            l_2.wrapping_add(13),
                                            l_2,
                                            matches,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return has_found_match;
    }
}
