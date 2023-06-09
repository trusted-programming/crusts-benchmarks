use libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strtoumax(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_value_s {
    pub payload: *mut libc::c_void,
    pub type_0: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_parse_result_s {
    pub error: u64,
    pub error_offset: u64,
    pub error_line_no: u64,
    pub error_row_no: u64,
}
pub const json_parse_flags_allow_json5: u32 = 16163;
pub const json_parse_flags_allow_simplified_json: u32 = 31;
pub const json_parse_flags_allow_multi_line_strings: u32 = 8192;
pub const json_parse_flags_allow_inf_and_nan: u32 = 4096;
pub const json_parse_flags_allow_leading_or_trailing_decimal_point: u32 = 2048;
pub const json_parse_flags_allow_leading_plus_sign: u32 = 1024;
pub const json_parse_flags_allow_hexadecimal_numbers: u32 = 512;
pub const json_parse_flags_allow_single_quoted_strings: u32 = 256;
pub const json_parse_flags_allow_location_information: u32 = 128;
pub const json_parse_flags_deprecated: u32 = 64;
pub const json_parse_flags_allow_c_style_comments: u32 = 32;
pub const json_parse_flags_allow_no_commas: u32 = 16;
pub const json_parse_flags_allow_equals_in_object: u32 = 8;
pub const json_parse_flags_allow_global_object: u32 = 4;
pub const json_parse_flags_allow_unquoted_keys: u32 = 2;
pub const json_parse_flags_allow_trailing_comma: u32 = 1;
pub const json_parse_flags_default: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_parse_state_s {
    pub src: *const i8,
    pub size: u64,
    pub offset: u64,
    pub flags_bitset: u64,
    pub data: *mut i8,
    pub dom: *mut i8,
    pub dom_size: u64,
    pub data_size: u64,
    pub line_no: u64,
    pub line_offset: u64,
    pub error: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_number_s {
    pub number: *const i8,
    pub number_size: u64,
}
pub const json_type_number: u32 = 1;
pub const json_type_null: u32 = 6;
pub const json_type_false: u32 = 5;
pub const json_type_true: u32 = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_array_s {
    pub start: *mut json_array_element_s,
    pub length: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_array_element_s {
    pub value: *mut json_value_s,
    pub next: *mut json_array_element_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_value_ex_s {
    pub value: json_value_s,
    pub offset: u64,
    pub line_no: u64,
    pub row_no: u64,
}
pub const json_parse_error_premature_end_of_buffer: u32 = 7;
pub const json_type_array: u32 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_s {
    pub start: *mut json_object_element_s,
    pub length: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_element_s {
    pub name: *mut json_string_s,
    pub value: *mut json_value_s,
    pub next: *mut json_object_element_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_string_s {
    pub string: *const i8,
    pub string_size: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_string_ex_s {
    pub string: json_string_s,
    pub offset: u64,
    pub line_no: u64,
    pub row_no: u64,
}
pub const json_type_object: u32 = 2;
pub const json_type_string: u32 = 0;
pub const json_parse_error_allocator_failed: u32 = 9;
pub const json_parse_error_unexpected_trailing_characters: u32 = 10;
pub const json_parse_error_invalid_value: u32 = 6;
pub const json_parse_error_invalid_number_format: u32 = 5;
pub const json_parse_error_expected_comma_or_closing_bracket: u32 = 1;
pub const json_parse_error_unknown: u32 = 11;
pub const json_parse_error_expected_colon: u32 = 2;
pub const json_parse_error_invalid_string: u32 = 8;
pub const json_parse_error_invalid_string_escape_sequence: u32 = 4;
pub const json_parse_error_expected_opening_quote: u32 = 3;
pub const json_parse_error_none: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_extract_state_s {
    pub dom: *mut i8,
    pub data: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_extract_result_s {
    pub dom_size: u64,
    pub data_size: u64,
}
#[no_mangle]
pub extern "C" fn json_hexadecimal_digit(c: i8) -> i32 {
    if '0' as i32 <= c as i32 && c as i32 <= '9' as i32 {
        return c as i32 - '0' as i32;
    }
    if 'a' as i32 <= c as i32 && c as i32 <= 'f' as i32 {
        return c as i32 - 'a' as i32 + 10;
    }
    if 'A' as i32 <= c as i32 && c as i32 <= 'F' as i32 {
        return c as i32 - 'A' as i32 + 10;
    }
    return -1;
}

#[no_mangle]
pub extern "C" fn json_hexadecimal_value(mut c: *const i8, size: u64, mut result: *mut u64) -> i32 {
    unsafe {
        let mut p = 0 as *const i8;
        let mut digit: i32 = 0;
        if size > (::std::mem::size_of::<u64>() as u64).wrapping_mul(2) {
            return 0;
        }
        *result = 0;
        p = c;
        while (p.offset_from(c) as u64) < size {
            *result <<= 4;
            digit = json_hexadecimal_digit(*p);
            if digit < 0 || digit > 15 {
                return 0;
            };
            *result |= digit as u64;
            p = p.offset(1);
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn json_skip_whitespace(mut state: *mut json_parse_state_s) -> i32 {
    unsafe {
        let mut offset = (*state).offset;
        let size = (*state).size;
        let src = (*state).src;
        if offset >= (*state).size {
            return 0;
        }
        match *src.offset(offset as isize) as i32 {
            32 | 13 | 9 | 10 => {}
            _ => return 0,
        }
        loop {
            match *src.offset(offset as isize) as i32 {
                32 | 13 | 9 => {}
                10 => {
                    let ref mut fresh0 = (*state).line_no;
                    *fresh0 = (*fresh0).wrapping_add(1);
                    (*state).line_offset = offset;
                }
                _ => {
                    (*state).offset = offset;
                    return 1;
                }
            }
            offset = offset.wrapping_add(1);
            if !(offset < size) {
                break;
            }
        }
        (*state).offset = offset;
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn json_skip_c_style_comments(mut state: *mut json_parse_state_s) -> i32 {
    unsafe {
        if ((*state).offset).wrapping_add(2) > (*state).size {
            return 0;
        }
        if '/' as i32 == *((*state).src).offset((*state).offset as isize) as i32 {
            if '/' as i32
                == *((*state).src).offset(((*state).offset).wrapping_add(1) as isize) as i32
            {
                let ref mut fresh1 = (*state).offset;
                *fresh1 = (*fresh1).wrapping_add(1);
                let ref mut fresh2 = (*state).offset;
                *fresh2 = (*fresh2).wrapping_add(1);
                while (*state).offset < (*state).size {
                    match *((*state).src).offset((*state).offset as isize) as i32 {
                        10 => {
                            let ref mut fresh4 = (*state).offset;
                            *fresh4 = (*fresh4).wrapping_add(1);
                            let ref mut fresh5 = (*state).line_no;
                            *fresh5 = (*fresh5).wrapping_add(1);
                            (*state).line_offset = (*state).offset;
                            return 1;
                        }
                        _ => {
                            let ref mut fresh3 = (*state).offset;
                            *fresh3 = (*fresh3).wrapping_add(1);
                        }
                    }
                }
                return 1;
            } else {
                if '*' as i32
                    == *((*state).src).offset(((*state).offset).wrapping_add(1) as isize) as i32
                {
                    let ref mut fresh6 = (*state).offset;
                    *fresh6 = (*fresh6).wrapping_add(1);
                    let ref mut fresh7 = (*state).offset;
                    *fresh7 = (*fresh7).wrapping_add(1);
                    while ((*state).offset).wrapping_add(1) < (*state).size {
                        if '*' as i32 == *((*state).src).offset((*state).offset as isize) as i32
                            && '/' as i32
                                == *((*state).src)
                                    .offset(((*state).offset).wrapping_add(1) as isize)
                                    as i32
                        {
                            let ref mut fresh8 = (*state).offset;
                            *fresh8 = (*fresh8 as u64).wrapping_add(2) as u64;
                            return 1;
                        } else {
                            if '\n' as i32
                                == *((*state).src).offset((*state).offset as isize) as i32
                            {
                                let ref mut fresh9 = (*state).line_no;
                                *fresh9 = (*fresh9).wrapping_add(1);
                                (*state).line_offset = (*state).offset;
                            }
                        }
                        let ref mut fresh10 = (*state).offset;
                        *fresh10 = (*fresh10).wrapping_add(1);
                    }
                    return 1;
                }
            }
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn json_skip_all_skippables(mut state: *mut json_parse_state_s) -> i32 {
    unsafe {
        let mut did_consume = 0;
        let size = (*state).size;
        if json_parse_flags_allow_c_style_comments as u64 & (*state).flags_bitset != 0 {
            loop {
                if (*state).offset == size {
                    (*state).error = json_parse_error_premature_end_of_buffer as u64;
                    return 1;
                }
                did_consume = json_skip_whitespace(state);
                if (*state).offset >= size {
                    (*state).error = json_parse_error_premature_end_of_buffer as u64;
                    return 1;
                }
                did_consume |= json_skip_c_style_comments(state);
                if !(0 != did_consume) {
                    break;
                }
            }
        } else {
            loop {
                if (*state).offset == size {
                    (*state).error = json_parse_error_premature_end_of_buffer as u64;
                    return 1;
                }
                did_consume = json_skip_whitespace(state);
                if !(0 != did_consume) {
                    break;
                }
            }
        }
        if (*state).offset == size {
            (*state).error = json_parse_error_premature_end_of_buffer as u64;
            return 1;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn json_get_string_size(mut state: *mut json_parse_state_s, mut is_key: u64) -> i32 {
    unsafe {
        let mut offset = (*state).offset;
        let size = (*state).size;
        let mut data_size = 0;
        let src = (*state).src;
        let is_single_quote = ('\'' as i32 == *src.offset(offset as isize) as i32) as i32;
        let quote_to_use = (if is_single_quote != 0 {
            '\'' as i32
        } else {
            '"' as i32
        }) as i8;
        let flags_bitset = (*state).flags_bitset;
        let mut codepoint: u64 = 0;
        let mut high_surrogate = 0;
        if json_parse_flags_allow_location_information as u64 & flags_bitset != 0 && is_key != 0 {
            let ref mut fresh11 = (*state).dom_size;
            *fresh11 = (*fresh11 as u64)
                .wrapping_add(::std::mem::size_of::<json_string_ex_s>() as u64)
                as u64;
        } else {
            let ref mut fresh12 = (*state).dom_size;
            *fresh12 = (*fresh12 as u64).wrapping_add(::std::mem::size_of::<json_string_s>() as u64)
                as u64;
        }
        if '"' as i32 != *src.offset(offset as isize) as i32 {
            if !(json_parse_flags_allow_single_quoted_strings as u64 & flags_bitset != 0
                && is_single_quote != 0)
            {
                (*state).error = json_parse_error_expected_opening_quote as u64;
                (*state).offset = offset;
                return 1;
            }
        }
        offset = offset.wrapping_add(1);
        while offset < size && quote_to_use as i32 != *src.offset(offset as isize) as i32 {
            data_size = data_size.wrapping_add(1);
            match *src.offset(offset as isize) as i32 {
                0 | 9 => {
                    (*state).error = json_parse_error_invalid_string as u64;
                    (*state).offset = offset;
                    return 1;
                }
                _ => {}
            }
            if '\\' as i32 == *src.offset(offset as isize) as i32 {
                offset = offset.wrapping_add(1);
                if offset == size {
                    (*state).error = json_parse_error_premature_end_of_buffer as u64;
                    (*state).offset = offset;
                    return 1;
                }
                match *src.offset(offset as isize) as i32 {
                    34 | 92 | 47 | 98 | 102 | 110 | 114 | 116 => {
                        offset = offset.wrapping_add(1);
                    }
                    117 => {
                        if !(offset.wrapping_add(5) < size) {
                            (*state).error = json_parse_error_invalid_string_escape_sequence as u64;
                            (*state).offset = offset;
                            return 1;
                        }
                        codepoint = 0;
                        if json_hexadecimal_value(
                            &*src.offset(offset.wrapping_add(1) as isize),
                            4,
                            &mut codepoint,
                        ) == 0
                        {
                            (*state).error = json_parse_error_invalid_string_escape_sequence as u64;
                            (*state).offset = offset;
                            return 1;
                        }
                        if high_surrogate != 0 {
                            if codepoint >= 0xdc00 && codepoint <= 0xdfff {
                                data_size = (data_size as u64).wrapping_add(3) as u64;
                                high_surrogate = 0;
                            } else {
                                (*state).error =
                                    json_parse_error_invalid_string_escape_sequence as u64;
                                (*state).offset = offset;
                                return 1;
                            }
                        } else if codepoint <= 0x7f {
                            data_size = (data_size as u64).wrapping_add(0) as u64;
                        } else if codepoint <= 0x7ff {
                            data_size = (data_size as u64).wrapping_add(1) as u64;
                        } else if codepoint >= 0xd800 && codepoint <= 0xdbff {
                            if offset.wrapping_add(11) > size
                                || '\\' as i32
                                    != *src.offset(offset.wrapping_add(5) as isize) as i32
                                || 'u' as i32 != *src.offset(offset.wrapping_add(6) as isize) as i32
                            {
                                (*state).error =
                                    json_parse_error_invalid_string_escape_sequence as u64;
                                (*state).offset = offset;
                                return 1;
                            }
                            high_surrogate = codepoint;
                        } else if codepoint >= 0xd800 && codepoint <= 0xdfff {
                            (*state).error = json_parse_error_invalid_string_escape_sequence as u64;
                            (*state).offset = offset;
                            return 1;
                        } else {
                            data_size = (data_size as u64).wrapping_add(2) as u64;
                        }
                        offset = (offset as u64).wrapping_add(5) as u64;
                    }
                    _ => {
                        (*state).error = json_parse_error_invalid_string_escape_sequence as u64;
                        (*state).offset = offset;
                        return 1;
                    }
                }
            } else if '\r' as i32 == *src.offset(offset as isize) as i32
                || '\n' as i32 == *src.offset(offset as isize) as i32
            {
                if json_parse_flags_allow_multi_line_strings as u64 & flags_bitset == 0 {
                    (*state).error = json_parse_error_invalid_string_escape_sequence as u64;
                    (*state).offset = offset;
                    return 1;
                }
                offset = offset.wrapping_add(1);
            } else {
                offset = offset.wrapping_add(1);
            }
        }
        if offset == size {
            (*state).error = json_parse_error_premature_end_of_buffer as u64;
            (*state).offset = offset.wrapping_sub(1);
            return 1;
        }
        offset = offset.wrapping_add(1);
        let ref mut fresh13 = (*state).data_size;
        *fresh13 = (*fresh13 as u64).wrapping_add(data_size) as u64;
        let ref mut fresh14 = (*state).data_size;
        *fresh14 = (*fresh14).wrapping_add(1);
        (*state).offset = offset;
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn is_valid_unquoted_key_char(c: i8) -> i32 {
    return ('0' as i32 <= c as i32 && c as i32 <= '9' as i32
        || 'a' as i32 <= c as i32 && c as i32 <= 'z' as i32
        || 'A' as i32 <= c as i32 && c as i32 <= 'Z' as i32
        || '_' as i32 == c as i32) as i32;
}

#[no_mangle]
pub extern "C" fn json_get_key_size(mut state: *mut json_parse_state_s) -> i32 {
    unsafe {
        let flags_bitset = (*state).flags_bitset;
        if json_parse_flags_allow_unquoted_keys as u64 & flags_bitset != 0 {
            let mut offset = (*state).offset;
            let size = (*state).size;
            let src = (*state).src;
            let mut data_size = (*state).data_size;
            if '"' as i32 == *src.offset(offset as isize) as i32 {
                return json_get_string_size(state, 1);
            } else if json_parse_flags_allow_single_quoted_strings as u64 & flags_bitset != 0
                && '\'' as i32 == *src.offset(offset as isize) as i32
            {
                return json_get_string_size(state, 1);
            } else {
                while offset < size && is_valid_unquoted_key_char(*src.offset(offset as isize)) != 0
                {
                    offset = offset.wrapping_add(1);
                    data_size = data_size.wrapping_add(1);
                }
                data_size = data_size.wrapping_add(1);
                if json_parse_flags_allow_location_information as u64 & flags_bitset != 0 {
                    let ref mut fresh15 = (*state).dom_size;
                    *fresh15 = (*fresh15 as u64)
                        .wrapping_add(::std::mem::size_of::<json_string_ex_s>() as u64)
                        as u64;
                } else {
                    let ref mut fresh16 = (*state).dom_size;
                    *fresh16 = (*fresh16 as u64)
                        .wrapping_add(::std::mem::size_of::<json_string_s>() as u64)
                        as u64;
                };
                (*state).offset = offset;
                (*state).data_size = data_size;
                return 0;
            }
        } else {
            return json_get_string_size(state, 1);
        };
    }
}

#[no_mangle]
pub extern "C" fn json_get_object_size(
    mut state: *mut json_parse_state_s,
    mut is_global_object: i32,
) -> i32 {
    unsafe {
        let flags_bitset = (*state).flags_bitset;
        let src = (*state).src;
        let size = (*state).size;
        let mut elements = 0;
        let mut allow_comma = 0;
        let mut found_closing_brace = 0;
        if is_global_object != 0 {
            if json_skip_all_skippables(state) == 0
                && '{' as i32 == *((*state).src).offset((*state).offset as isize) as i32
            {
                is_global_object = 0;
            }
        }
        if is_global_object == 0 {
            if '{' as i32 != *src.offset((*state).offset as isize) as i32 {
                (*state).error = json_parse_error_unknown as u64;
                return 1;
            }
            let ref mut fresh17 = (*state).offset;
            *fresh17 = (*fresh17).wrapping_add(1);
        }
        let ref mut fresh18 = (*state).dom_size;
        *fresh18 =
            (*fresh18 as u64).wrapping_add(::std::mem::size_of::<json_object_s>() as u64) as u64;
        if (*state).offset == size && is_global_object == 0 {
            (*state).error = json_parse_error_premature_end_of_buffer as u64;
            return 1;
        }
        let mut current_block_66: u64;
        loop {
            if is_global_object == 0 {
                if json_skip_all_skippables(state) != 0 {
                    (*state).error = json_parse_error_premature_end_of_buffer as u64;
                    return 1;
                }
                if '}' as i32 == *src.offset((*state).offset as isize) as i32 {
                    let ref mut fresh19 = (*state).offset;
                    *fresh19 = (*fresh19).wrapping_add(1);
                    found_closing_brace = 1;
                    break;
                }
            } else if json_skip_all_skippables(state) != 0 {
                break;
            }
            if allow_comma != 0 {
                if ',' as i32 == *src.offset((*state).offset as isize) as i32 {
                    let ref mut fresh20 = (*state).offset;
                    *fresh20 = (*fresh20).wrapping_add(1);
                    allow_comma = 0;
                } else if json_parse_flags_allow_no_commas as u64 & flags_bitset != 0 {
                    allow_comma = 0;
                } else {
                    (*state).error = json_parse_error_expected_comma_or_closing_bracket as u64;
                    return 1;
                }
                if json_parse_flags_allow_trailing_comma as u64 & flags_bitset != 0 {
                    current_block_66 = 6057473163062296781;
                } else {
                    if json_skip_all_skippables(state) != 0 {
                        (*state).error = json_parse_error_premature_end_of_buffer as u64;
                        return 1;
                    }
                    current_block_66 = 2122094917359643297;
                }
            } else {
                current_block_66 = 2122094917359643297;
            }
            match current_block_66 {
                2122094917359643297 => {
                    if json_get_key_size(state) != 0 {
                        (*state).error = json_parse_error_invalid_string as u64;
                        return 1;
                    }
                    if json_skip_all_skippables(state) != 0 {
                        (*state).error = json_parse_error_premature_end_of_buffer as u64;
                        return 1;
                    }
                    if json_parse_flags_allow_equals_in_object as u64 & flags_bitset != 0 {
                        let current = *src.offset((*state).offset as isize);
                        if ':' as i32 != current as i32 && '=' as i32 != current as i32 {
                            (*state).error = json_parse_error_expected_colon as u64;
                            return 1;
                        }
                    } else if ':' as i32 != *src.offset((*state).offset as isize) as i32 {
                        (*state).error = json_parse_error_expected_colon as u64;
                        return 1;
                    }
                    let ref mut fresh21 = (*state).offset;
                    *fresh21 = (*fresh21).wrapping_add(1);
                    if json_skip_all_skippables(state) != 0 {
                        (*state).error = json_parse_error_premature_end_of_buffer as u64;
                        return 1;
                    }
                    if json_get_value_size(state, 0) != 0 {
                        return 1;
                    }
                    elements = elements.wrapping_add(1);
                    allow_comma = 1;
                }
                _ => {}
            }
            if !((*state).offset < size) {
                break;
            }
        }
        if (*state).offset == size && is_global_object == 0 && found_closing_brace == 0 {
            (*state).error = json_parse_error_premature_end_of_buffer as u64;
            return 1;
        }
        let ref mut fresh22 = (*state).dom_size;
        *fresh22 = (*fresh22 as u64).wrapping_add(
            (::std::mem::size_of::<json_object_element_s>() as u64).wrapping_mul(elements),
        ) as u64;
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn json_get_array_size(mut state: *mut json_parse_state_s) -> i32 {
    unsafe {
        let flags_bitset = (*state).flags_bitset;
        let mut elements = 0;
        let mut allow_comma = 0;
        let src = (*state).src;
        let size = (*state).size;
        if '[' as i32 != *src.offset((*state).offset as isize) as i32 {
            (*state).error = json_parse_error_unknown as u64;
            return 1;
        }
        let ref mut fresh23 = (*state).offset;
        *fresh23 = (*fresh23).wrapping_add(1);
        let ref mut fresh24 = (*state).dom_size;
        *fresh24 =
            (*fresh24 as u64).wrapping_add(::std::mem::size_of::<json_array_s>() as u64) as u64;
        while (*state).offset < size {
            if json_skip_all_skippables(state) != 0 {
                (*state).error = json_parse_error_premature_end_of_buffer as u64;
                return 1;
            }
            if ']' as i32 == *src.offset((*state).offset as isize) as i32 {
                let ref mut fresh25 = (*state).offset;
                *fresh25 = (*fresh25).wrapping_add(1);
                let ref mut fresh26 = (*state).dom_size;
                *fresh26 = (*fresh26 as u64).wrapping_add(
                    (::std::mem::size_of::<json_array_element_s>() as u64).wrapping_mul(elements),
                ) as u64;
                return 0;
            }
            if allow_comma != 0 {
                if ',' as i32 == *src.offset((*state).offset as isize) as i32 {
                    let ref mut fresh27 = (*state).offset;
                    *fresh27 = (*fresh27).wrapping_add(1);
                    allow_comma = 0;
                } else if json_parse_flags_allow_no_commas as u64 & flags_bitset == 0 {
                    (*state).error = json_parse_error_expected_comma_or_closing_bracket as u64;
                    return 1;
                }
                if json_parse_flags_allow_trailing_comma as u64 & flags_bitset != 0 {
                    allow_comma = 0;
                    continue;
                } else if json_skip_all_skippables(state) != 0 {
                    (*state).error = json_parse_error_premature_end_of_buffer as u64;
                    return 1;
                }
            }
            if json_get_value_size(state, 0) != 0 {
                return 1;
            }
            elements = elements.wrapping_add(1);
            allow_comma = 1;
        }
        (*state).error = json_parse_error_premature_end_of_buffer as u64;
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn json_get_number_size(mut state: *mut json_parse_state_s) -> i32 {
    unsafe {
        let flags_bitset = (*state).flags_bitset;
        let mut offset = (*state).offset;
        let size = (*state).size;
        let mut had_leading_digits = 0;
        let src = (*state).src;
        let ref mut fresh28 = (*state).dom_size;
        *fresh28 =
            (*fresh28 as u64).wrapping_add(::std::mem::size_of::<json_number_s>() as u64) as u64;
        if json_parse_flags_allow_hexadecimal_numbers as u64 & flags_bitset != 0
            && offset.wrapping_add(1) < size
            && '0' as i32 == *src.offset(offset as isize) as i32
            && ('x' as i32 == *src.offset(offset.wrapping_add(1) as isize) as i32
                || 'X' as i32 == *src.offset(offset.wrapping_add(1) as isize) as i32)
        {
            offset = (offset as u64).wrapping_add(2) as u64;
            while offset < size
                && ('0' as i32 <= *src.offset(offset as isize) as i32
                    && *src.offset(offset as isize) as i32 <= '9' as i32
                    || 'a' as i32 <= *src.offset(offset as isize) as i32
                        && *src.offset(offset as isize) as i32 <= 'f' as i32
                    || 'A' as i32 <= *src.offset(offset as isize) as i32
                        && *src.offset(offset as isize) as i32 <= 'F' as i32)
            {
                offset = offset.wrapping_add(1);
            }
        } else {
            let mut found_sign = 0;
            let mut inf_or_nan = 0;
            if offset < size
                && ('-' as i32 == *src.offset(offset as isize) as i32
                    || json_parse_flags_allow_leading_plus_sign as u64 & flags_bitset != 0
                        && '+' as i32 == *src.offset(offset as isize) as i32)
            {
                offset = offset.wrapping_add(1);
                found_sign = 1;
            }
            if json_parse_flags_allow_inf_and_nan as u64 & flags_bitset != 0 {
                let inf: [i8; 9] = *::std::mem::transmute::<&[u8; 9], &[i8; 9]>(b"Infinity\0");
                let inf_strlen = (::std::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1);
                let nan: [i8; 4] = *::std::mem::transmute::<&[u8; 4], &[i8; 4]>(b"NaN\0");
                let nan_strlen = (::std::mem::size_of::<[i8; 4]>() as u64).wrapping_sub(1);
                if offset.wrapping_add(inf_strlen) < size {
                    let mut found = 1;
                    let mut i: u64 = 0;
                    i = 0;
                    while i < inf_strlen {
                        if inf[i as usize] as i32
                            != *src.offset(offset.wrapping_add(i) as isize) as i32
                        {
                            found = 0;
                            break;
                        } else {
                            i = i.wrapping_add(1);
                        }
                    }
                    if found != 0 {
                        offset = (offset as u64).wrapping_add(inf_strlen) as u64;
                        inf_or_nan = 1;
                    }
                }
                if offset.wrapping_add(nan_strlen) < size {
                    let mut found_0 = 1;
                    let mut i_0: u64 = 0;
                    i_0 = 0;
                    while i_0 < nan_strlen {
                        if nan[i_0 as usize] as i32
                            != *src.offset(offset.wrapping_add(i_0) as isize) as i32
                        {
                            found_0 = 0;
                            break;
                        } else {
                            i_0 = i_0.wrapping_add(1);
                        }
                    }
                    if found_0 != 0 {
                        offset = (offset as u64).wrapping_add(nan_strlen) as u64;
                        inf_or_nan = 1;
                    }
                }
                if inf_or_nan != 0 {
                    if offset < size {
                        match *src.offset(offset as isize) as i32 {
                            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 101 | 69 => {
                                (*state).error = json_parse_error_invalid_number_format as u64;
                                (*state).offset = offset;
                                return 1;
                            }
                            _ => {}
                        }
                    }
                }
            }
            if found_sign != 0
                && inf_or_nan == 0
                && offset < size
                && !('0' as i32 <= *src.offset(offset as isize) as i32
                    && *src.offset(offset as isize) as i32 <= '9' as i32)
            {
                if json_parse_flags_allow_leading_or_trailing_decimal_point as u64 & flags_bitset
                    == 0
                    || '.' as i32 != *src.offset(offset as isize) as i32
                {
                    (*state).error = json_parse_error_invalid_number_format as u64;
                    (*state).offset = offset;
                    return 1;
                }
            }
            if offset < size && '0' as i32 == *src.offset(offset as isize) as i32 {
                offset = offset.wrapping_add(1);
                had_leading_digits = 1;
                if offset < size
                    && ('0' as i32 <= *src.offset(offset as isize) as i32
                        && *src.offset(offset as isize) as i32 <= '9' as i32)
                {
                    (*state).error = json_parse_error_invalid_number_format as u64;
                    (*state).offset = offset;
                    return 1;
                }
            }
            while offset < size
                && ('0' as i32 <= *src.offset(offset as isize) as i32
                    && *src.offset(offset as isize) as i32 <= '9' as i32)
            {
                offset = offset.wrapping_add(1);
                had_leading_digits = 1;
            }
            if offset < size && '.' as i32 == *src.offset(offset as isize) as i32 {
                offset = offset.wrapping_add(1);
                if offset >= size
                    || !('0' as i32 <= *src.offset(offset as isize) as i32
                        && *src.offset(offset as isize) as i32 <= '9' as i32)
                {
                    if json_parse_flags_allow_leading_or_trailing_decimal_point as u64
                        & flags_bitset
                        == 0
                        || had_leading_digits == 0
                    {
                        (*state).error = json_parse_error_invalid_number_format as u64;
                        (*state).offset = offset;
                        return 1;
                    }
                }
                while offset < size
                    && ('0' as i32 <= *src.offset(offset as isize) as i32
                        && *src.offset(offset as isize) as i32 <= '9' as i32)
                {
                    offset = offset.wrapping_add(1);
                }
            }
            if offset < size
                && ('e' as i32 == *src.offset(offset as isize) as i32
                    || 'E' as i32 == *src.offset(offset as isize) as i32)
            {
                offset = offset.wrapping_add(1);
                if offset < size
                    && ('-' as i32 == *src.offset(offset as isize) as i32
                        || '+' as i32 == *src.offset(offset as isize) as i32)
                {
                    offset = offset.wrapping_add(1);
                }
                if offset < size
                    && !('0' as i32 <= *src.offset(offset as isize) as i32
                        && *src.offset(offset as isize) as i32 <= '9' as i32)
                {
                    (*state).error = json_parse_error_invalid_number_format as u64;
                    (*state).offset = offset;
                    return 1;
                }
                loop {
                    offset = offset.wrapping_add(1);
                    if !(offset < size
                        && ('0' as i32 <= *src.offset(offset as isize) as i32
                            && *src.offset(offset as isize) as i32 <= '9' as i32))
                    {
                        break;
                    }
                }
            }
        }
        if offset < size {
            match *src.offset(offset as isize) as i32 {
                32 | 9 | 13 | 10 | 125 | 44 | 93 => {}
                61 => {
                    if !(json_parse_flags_allow_equals_in_object as u64 & flags_bitset != 0) {
                        (*state).error = json_parse_error_invalid_number_format as u64;
                        (*state).offset = offset;
                        return 1;
                    }
                }
                _ => {
                    (*state).error = json_parse_error_invalid_number_format as u64;
                    (*state).offset = offset;
                    return 1;
                }
            }
        }
        let ref mut fresh29 = (*state).data_size;
        *fresh29 = (*fresh29 as u64).wrapping_add(offset.wrapping_sub((*state).offset)) as u64;
        let ref mut fresh30 = (*state).data_size;
        *fresh30 = (*fresh30).wrapping_add(1);
        (*state).offset = offset;
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn json_get_value_size(
    mut state: *mut json_parse_state_s,
    mut is_global_object: i32,
) -> i32 {
    unsafe {
        let flags_bitset = (*state).flags_bitset;
        let src = (*state).src;
        let mut offset: u64 = 0;
        let size = (*state).size;
        if json_parse_flags_allow_location_information as u64 & flags_bitset != 0 {
            let ref mut fresh31 = (*state).dom_size;
            *fresh31 = (*fresh31 as u64)
                .wrapping_add(::std::mem::size_of::<json_value_ex_s>() as u64)
                as u64;
        } else {
            let ref mut fresh32 = (*state).dom_size;
            *fresh32 =
                (*fresh32 as u64).wrapping_add(::std::mem::size_of::<json_value_s>() as u64) as u64;
        }
        if is_global_object != 0 {
            return json_get_object_size(state, 1);
        } else {
            if json_skip_all_skippables(state) != 0 {
                (*state).error = json_parse_error_premature_end_of_buffer as u64;
                return 1;
            }
            offset = (*state).offset;
            match *src.offset(offset as isize) as i32 {
                34 => return json_get_string_size(state, 0),
                39 => {
                    if json_parse_flags_allow_single_quoted_strings as u64 & flags_bitset != 0 {
                        return json_get_string_size(state, 0);
                    } else {
                        (*state).error = json_parse_error_invalid_value as u64;
                        return 1;
                    }
                }
                123 => return json_get_object_size(state, 0),
                91 => return json_get_array_size(state),
                45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                    return json_get_number_size(state);
                }
                43 => {
                    if json_parse_flags_allow_leading_plus_sign as u64 & flags_bitset != 0 {
                        return json_get_number_size(state);
                    } else {
                        (*state).error = json_parse_error_invalid_number_format as u64;
                        return 1;
                    }
                }
                46 => {
                    if json_parse_flags_allow_leading_or_trailing_decimal_point as u64
                        & flags_bitset
                        != 0
                    {
                        return json_get_number_size(state);
                    } else {
                        (*state).error = json_parse_error_invalid_number_format as u64;
                        return 1;
                    }
                }
                _ => {
                    if offset.wrapping_add(4) <= size
                        && 't' as i32 == *src.offset(offset.wrapping_add(0) as isize) as i32
                        && 'r' as i32 == *src.offset(offset.wrapping_add(1) as isize) as i32
                        && 'u' as i32 == *src.offset(offset.wrapping_add(2) as isize) as i32
                        && 'e' as i32 == *src.offset(offset.wrapping_add(3) as isize) as i32
                    {
                        let ref mut fresh33 = (*state).offset;
                        *fresh33 = (*fresh33 as u64).wrapping_add(4) as u64;
                        return 0;
                    } else {
                        if offset.wrapping_add(5) <= size
                            && 'f' as i32 == *src.offset(offset.wrapping_add(0) as isize) as i32
                            && 'a' as i32 == *src.offset(offset.wrapping_add(1) as isize) as i32
                            && 'l' as i32 == *src.offset(offset.wrapping_add(2) as isize) as i32
                            && 's' as i32 == *src.offset(offset.wrapping_add(3) as isize) as i32
                            && 'e' as i32 == *src.offset(offset.wrapping_add(4) as isize) as i32
                        {
                            let ref mut fresh34 = (*state).offset;
                            *fresh34 = (*fresh34 as u64).wrapping_add(5) as u64;
                            return 0;
                        } else {
                            if offset.wrapping_add(4) <= size
                                && 'n' as i32
                                    == *((*state).src).offset(offset.wrapping_add(0) as isize)
                                        as i32
                                && 'u' as i32
                                    == *((*state).src).offset(offset.wrapping_add(1) as isize)
                                        as i32
                                && 'l' as i32
                                    == *((*state).src).offset(offset.wrapping_add(2) as isize)
                                        as i32
                                && 'l' as i32
                                    == *((*state).src).offset(offset.wrapping_add(3) as isize)
                                        as i32
                            {
                                let ref mut fresh35 = (*state).offset;
                                *fresh35 = (*fresh35 as u64).wrapping_add(4) as u64;
                                return 0;
                            } else {
                                if json_parse_flags_allow_inf_and_nan as u64 & flags_bitset != 0
                                    && offset.wrapping_add(3) <= size
                                    && 'N' as i32
                                        == *src.offset(offset.wrapping_add(0) as isize) as i32
                                    && 'a' as i32
                                        == *src.offset(offset.wrapping_add(1) as isize) as i32
                                    && 'N' as i32
                                        == *src.offset(offset.wrapping_add(2) as isize) as i32
                                {
                                    return json_get_number_size(state);
                                } else {
                                    if json_parse_flags_allow_inf_and_nan as u64 & flags_bitset != 0
                                        && offset.wrapping_add(8) <= size
                                        && 'I' as i32
                                            == *src.offset(offset.wrapping_add(0) as isize) as i32
                                        && 'n' as i32
                                            == *src.offset(offset.wrapping_add(1) as isize) as i32
                                        && 'f' as i32
                                            == *src.offset(offset.wrapping_add(2) as isize) as i32
                                        && 'i' as i32
                                            == *src.offset(offset.wrapping_add(3) as isize) as i32
                                        && 'n' as i32
                                            == *src.offset(offset.wrapping_add(4) as isize) as i32
                                        && 'i' as i32
                                            == *src.offset(offset.wrapping_add(5) as isize) as i32
                                        && 't' as i32
                                            == *src.offset(offset.wrapping_add(6) as isize) as i32
                                        && 'y' as i32
                                            == *src.offset(offset.wrapping_add(7) as isize) as i32
                                    {
                                        return json_get_number_size(state);
                                    }
                                }
                            }
                        }
                    };
                    (*state).error = json_parse_error_invalid_value as u64;
                    return 1;
                }
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn json_parse_string(
    mut state: *mut json_parse_state_s,
    mut string: *mut json_string_s,
) {
    unsafe {
        let mut offset = (*state).offset;
        let mut bytes_written = 0;
        let src = (*state).src;
        let quote_to_use = (if '\'' as i32 == *src.offset(offset as isize) as i32 {
            '\'' as i32
        } else {
            '"' as i32
        }) as i8;
        let mut data = (*state).data;
        let mut high_surrogate = 0;
        let mut codepoint: u64 = 0;
        let ref mut fresh36 = (*string).string;
        *fresh36 = data;
        offset = offset.wrapping_add(1);
        while quote_to_use as i32 != *src.offset(offset as isize) as i32 {
            if '\\' as i32 == *src.offset(offset as isize) as i32 {
                offset = offset.wrapping_add(1);
                let fresh37 = offset;
                offset = offset.wrapping_add(1);
                match *src.offset(fresh37 as isize) as i32 {
                    117 => {
                        codepoint = 0;
                        if json_hexadecimal_value(&*src.offset(offset as isize), 4, &mut codepoint)
                            == 0
                        {
                            return;
                        }
                        offset = (offset as u64).wrapping_add(4) as u64;
                        if codepoint <= 0x7f {
                            let fresh38 = bytes_written;
                            bytes_written = bytes_written.wrapping_add(1);
                            *data.offset(fresh38 as isize) = codepoint as i8;
                        } else if codepoint <= 0x7ff {
                            let fresh39 = bytes_written;
                            bytes_written = bytes_written.wrapping_add(1);
                            *data.offset(fresh39 as isize) = (0xc0 | codepoint >> 6i32) as i8;
                            let fresh40 = bytes_written;
                            bytes_written = bytes_written.wrapping_add(1);
                            *data.offset(fresh40 as isize) = (0x80 | codepoint & 0x3fu64) as i8;
                        } else if codepoint >= 0xd800 && codepoint <= 0xdbff {
                            high_surrogate = codepoint;
                        } else if codepoint >= 0xdc00 && codepoint <= 0xdfff {
                            let surrogate_offset =
                                0x10000u32.wrapping_sub(0xd800 << 10).wrapping_sub(0xdc00) as u64;
                            codepoint = (high_surrogate << 10i32)
                                .wrapping_add(codepoint)
                                .wrapping_add(surrogate_offset);
                            high_surrogate = 0;
                            let fresh41 = bytes_written;
                            bytes_written = bytes_written.wrapping_add(1);
                            *data.offset(fresh41 as isize) = (0xf0 | codepoint >> 18i32) as i8;
                            let fresh42 = bytes_written;
                            bytes_written = bytes_written.wrapping_add(1);
                            *data.offset(fresh42 as isize) =
                                (0x80 | codepoint >> 12 & 0x3fu64) as i8;
                            let fresh43 = bytes_written;
                            bytes_written = bytes_written.wrapping_add(1);
                            *data.offset(fresh43 as isize) =
                                (0x80 | codepoint >> 6 & 0x3fu64) as i8;
                            let fresh44 = bytes_written;
                            bytes_written = bytes_written.wrapping_add(1);
                            *data.offset(fresh44 as isize) = (0x80 | codepoint & 0x3fu64) as i8;
                        } else {
                            let fresh45 = bytes_written;
                            bytes_written = bytes_written.wrapping_add(1);
                            *data.offset(fresh45 as isize) = (0xe0 | codepoint >> 12i32) as i8;
                            let fresh46 = bytes_written;
                            bytes_written = bytes_written.wrapping_add(1);
                            *data.offset(fresh46 as isize) =
                                (0x80 | codepoint >> 6 & 0x3fu64) as i8;
                            let fresh47 = bytes_written;
                            bytes_written = bytes_written.wrapping_add(1);
                            *data.offset(fresh47 as isize) = (0x80 | codepoint & 0x3fu64) as i8;
                        }
                    }
                    34 => {
                        let fresh48 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh48 as isize) = '"' as i8;
                    }
                    92 => {
                        let fresh49 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh49 as isize) = '\\' as i8;
                    }
                    47 => {
                        let fresh50 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh50 as isize) = '/' as i8;
                    }
                    98 => {
                        let fresh51 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh51 as isize) = '\u{8}' as i8;
                    }
                    102 => {
                        let fresh52 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh52 as isize) = '\u{c}' as i8;
                    }
                    110 => {
                        let fresh53 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh53 as isize) = '\n' as i8;
                    }
                    114 => {
                        let fresh54 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh54 as isize) = '\r' as i8;
                    }
                    116 => {
                        let fresh55 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh55 as isize) = '\t' as i8;
                    }
                    13 => {
                        let fresh56 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh56 as isize) = '\r' as i8;
                        if '\n' as i32 == *src.offset(offset as isize) as i32 {
                            let fresh57 = bytes_written;
                            bytes_written = bytes_written.wrapping_add(1);
                            *data.offset(fresh57 as isize) = '\n' as i8;
                            offset = offset.wrapping_add(1);
                        }
                    }
                    10 => {
                        let fresh58 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh58 as isize) = '\n' as i8;
                    }
                    _ => return,
                }
            } else {
                let fresh59 = offset;
                offset = offset.wrapping_add(1);
                let fresh60 = bytes_written;
                bytes_written = bytes_written.wrapping_add(1);
                *data.offset(fresh60 as isize) = *src.offset(fresh59 as isize);
            }
        }
        offset = offset.wrapping_add(1);
        (*string).string_size = bytes_written;
        let fresh61 = bytes_written;
        bytes_written = bytes_written.wrapping_add(1);
        *data.offset(fresh61 as isize) = '\0' as i8;
        let ref mut fresh62 = (*state).data;
        *fresh62 = (*fresh62).offset(bytes_written as isize);
        (*state).offset = offset;
    }
}

#[no_mangle]
pub extern "C" fn json_parse_key(
    mut state: *mut json_parse_state_s,
    mut string: *mut json_string_s,
) {
    unsafe {
        if json_parse_flags_allow_unquoted_keys as u64 & (*state).flags_bitset != 0 {
            let src = (*state).src;
            let data = (*state).data;
            let mut offset = (*state).offset;
            if '"' as i32 == *src.offset(offset as isize) as i32
                || '\'' as i32 == *src.offset(offset as isize) as i32
            {
                json_parse_string(state, string);
            } else {
                let mut size = 0;
                let ref mut fresh63 = (*string).string;
                *fresh63 = (*state).data;
                while is_valid_unquoted_key_char(*src.offset(offset as isize)) != 0 {
                    let fresh64 = offset;
                    offset = offset.wrapping_add(1);
                    let fresh65 = size;
                    size = size.wrapping_add(1);
                    *data.offset(fresh65 as isize) = *src.offset(fresh64 as isize);
                }
                *data.offset(size as isize) = '\0' as i8;
                let fresh66 = size;
                size = size.wrapping_add(1);
                (*string).string_size = fresh66;
                let ref mut fresh67 = (*state).data;
                *fresh67 = (*fresh67).offset(size as isize);
                (*state).offset = offset;
            }
        } else {
            json_parse_string(state, string);
        };
    }
}

#[no_mangle]
pub extern "C" fn json_parse_object(
    mut state: *mut json_parse_state_s,
    mut is_global_object: i32,
    mut object: *mut json_object_s,
) {
    unsafe {
        let flags_bitset = (*state).flags_bitset;
        let size = (*state).size;
        let src = (*state).src;
        let mut elements = 0;
        let mut allow_comma = 0;
        let mut previous = 0 as *mut json_object_element_s;
        if is_global_object != 0 {
            if '{' as i32 == *src.offset((*state).offset as isize) as i32 {
                is_global_object = 0;
            }
        }
        if is_global_object == 0 {
            let ref mut fresh68 = (*state).offset;
            *fresh68 = (*fresh68).wrapping_add(1);
        }
        json_skip_all_skippables(state);
        elements = 0;
        while (*state).offset < size {
            let mut element = 0 as *mut json_object_element_s;
            let mut string = 0 as *mut json_string_s;
            let mut value = 0 as *mut json_value_s;
            if is_global_object == 0 {
                json_skip_all_skippables(state);
                if '}' as i32 == *src.offset((*state).offset as isize) as i32 {
                    let ref mut fresh69 = (*state).offset;
                    *fresh69 = (*fresh69).wrapping_add(1);
                    break;
                }
            } else if json_skip_all_skippables(state) != 0 {
                break;
            }
            if allow_comma != 0 {
                if ',' as i32 == *src.offset((*state).offset as isize) as i32 {
                    let ref mut fresh70 = (*state).offset;
                    *fresh70 = (*fresh70).wrapping_add(1);
                    allow_comma = 0;
                    continue;
                }
            }
            element = (*state).dom as *mut json_object_element_s;
            let ref mut fresh71 = (*state).dom;
            *fresh71 =
                (*fresh71).offset(::std::mem::size_of::<json_object_element_s>() as u64 as isize);
            if previous.is_null() {
                let ref mut fresh72 = (*object).start;
                *fresh72 = element;
            } else {
                let ref mut fresh73 = (*previous).next;
                *fresh73 = element;
            }
            previous = element;
            if json_parse_flags_allow_location_information as u64 & flags_bitset != 0 {
                let mut string_ex = (*state).dom as *mut json_string_ex_s;
                let ref mut fresh74 = (*state).dom;
                *fresh74 =
                    (*fresh74).offset(::std::mem::size_of::<json_string_ex_s>() as u64 as isize);
                (*string_ex).offset = (*state).offset;
                (*string_ex).line_no = (*state).line_no;
                (*string_ex).row_no = ((*state).offset).wrapping_sub((*state).line_offset);
                string = &mut (*string_ex).string;
            } else {
                string = (*state).dom as *mut json_string_s;
                let ref mut fresh75 = (*state).dom;
                *fresh75 =
                    (*fresh75).offset(::std::mem::size_of::<json_string_s>() as u64 as isize);
            }
            let ref mut fresh76 = (*element).name;
            *fresh76 = string;
            json_parse_key(state, string);
            json_skip_all_skippables(state);
            let ref mut fresh77 = (*state).offset;
            *fresh77 = (*fresh77).wrapping_add(1);
            json_skip_all_skippables(state);
            if json_parse_flags_allow_location_information as u64 & flags_bitset != 0 {
                let mut value_ex = (*state).dom as *mut json_value_ex_s;
                let ref mut fresh78 = (*state).dom;
                *fresh78 =
                    (*fresh78).offset(::std::mem::size_of::<json_value_ex_s>() as u64 as isize);
                (*value_ex).offset = (*state).offset;
                (*value_ex).line_no = (*state).line_no;
                (*value_ex).row_no = ((*state).offset).wrapping_sub((*state).line_offset);
                value = &mut (*value_ex).value;
            } else {
                value = (*state).dom as *mut json_value_s;
                let ref mut fresh79 = (*state).dom;
                *fresh79 = (*fresh79).offset(::std::mem::size_of::<json_value_s>() as u64 as isize);
            }
            let ref mut fresh80 = (*element).value;
            *fresh80 = value;
            json_parse_value(state, 0, value);
            elements = elements.wrapping_add(1);
            allow_comma = 1;
        }
        if !previous.is_null() {
            let ref mut fresh81 = (*previous).next;
            *fresh81 = 0 as *mut json_object_element_s;
        }
        if 0 == elements {
            let ref mut fresh82 = (*object).start;
            *fresh82 = 0 as *mut json_object_element_s;
        };
        (*object).length = elements;
    }
}

#[no_mangle]
pub extern "C" fn json_parse_array(
    mut state: *mut json_parse_state_s,
    mut array: *mut json_array_s,
) {
    unsafe {
        let src = (*state).src;
        let size = (*state).size;
        let mut elements = 0;
        let mut allow_comma = 0;
        let mut previous = 0 as *mut json_array_element_s;
        let ref mut fresh83 = (*state).offset;
        *fresh83 = (*fresh83).wrapping_add(1);
        json_skip_all_skippables(state);
        elements = 0;
        let mut current_block_28: u64;
        loop {
            let mut element = 0 as *mut json_array_element_s;
            let mut value = 0 as *mut json_value_s;
            json_skip_all_skippables(state);
            if ']' as i32 == *src.offset((*state).offset as isize) as i32 {
                let ref mut fresh84 = (*state).offset;
                *fresh84 = (*fresh84).wrapping_add(1);
                break;
            } else {
                if allow_comma != 0 {
                    if ',' as i32 == *src.offset((*state).offset as isize) as i32 {
                        let ref mut fresh85 = (*state).offset;
                        *fresh85 = (*fresh85).wrapping_add(1);
                        allow_comma = 0;
                        current_block_28 = 6873731126896040597;
                    } else {
                        current_block_28 = 13056961889198038528;
                    }
                } else {
                    current_block_28 = 13056961889198038528;
                }
                match current_block_28 {
                    13056961889198038528 => {
                        element = (*state).dom as *mut json_array_element_s;
                        let ref mut fresh86 = (*state).dom;
                        *fresh86 = (*fresh86)
                            .offset(::std::mem::size_of::<json_array_element_s>() as u64 as isize);
                        if previous.is_null() {
                            let ref mut fresh87 = (*array).start;
                            *fresh87 = element;
                        } else {
                            let ref mut fresh88 = (*previous).next;
                            *fresh88 = element;
                        }
                        previous = element;
                        if json_parse_flags_allow_location_information as u64
                            & (*state).flags_bitset
                            != 0
                        {
                            let mut value_ex = (*state).dom as *mut json_value_ex_s;
                            let ref mut fresh89 = (*state).dom;
                            *fresh89 = (*fresh89)
                                .offset(::std::mem::size_of::<json_value_ex_s>() as u64 as isize);
                            (*value_ex).offset = (*state).offset;
                            (*value_ex).line_no = (*state).line_no;
                            (*value_ex).row_no =
                                ((*state).offset).wrapping_sub((*state).line_offset);
                            value = &mut (*value_ex).value;
                        } else {
                            value = (*state).dom as *mut json_value_s;
                            let ref mut fresh90 = (*state).dom;
                            *fresh90 = (*fresh90)
                                .offset(::std::mem::size_of::<json_value_s>() as u64 as isize);
                        }
                        let ref mut fresh91 = (*element).value;
                        *fresh91 = value;
                        json_parse_value(state, 0, value);
                        elements = elements.wrapping_add(1);
                        allow_comma = 1;
                    }
                    _ => {}
                }
                if !((*state).offset < size) {
                    break;
                }
            }
        }
        if !previous.is_null() {
            let ref mut fresh92 = (*previous).next;
            *fresh92 = 0 as *mut json_array_element_s;
        }
        if 0 == elements {
            let ref mut fresh93 = (*array).start;
            *fresh93 = 0 as *mut json_array_element_s;
        };
        (*array).length = elements;
    }
}

#[no_mangle]
pub extern "C" fn json_parse_number(
    mut state: *mut json_parse_state_s,
    mut number: *mut json_number_s,
) {
    unsafe {
        let flags_bitset = (*state).flags_bitset;
        let mut offset = (*state).offset;
        let size = (*state).size;
        let mut bytes_written = 0;
        let src = (*state).src;
        let mut data = (*state).data;
        let ref mut fresh94 = (*number).number;
        *fresh94 = data;
        if json_parse_flags_allow_hexadecimal_numbers as u64 & flags_bitset != 0 {
            if '0' as i32 == *src.offset(offset as isize) as i32
                && ('x' as i32 == *src.offset(offset.wrapping_add(1) as isize) as i32
                    || 'X' as i32 == *src.offset(offset.wrapping_add(1) as isize) as i32)
            {
                while offset < size
                    && ('0' as i32 <= *src.offset(offset as isize) as i32
                        && *src.offset(offset as isize) as i32 <= '9' as i32
                        || 'a' as i32 <= *src.offset(offset as isize) as i32
                            && *src.offset(offset as isize) as i32 <= 'f' as i32
                        || 'A' as i32 <= *src.offset(offset as isize) as i32
                            && *src.offset(offset as isize) as i32 <= 'F' as i32
                        || 'x' as i32 == *src.offset(offset as isize) as i32
                        || 'X' as i32 == *src.offset(offset as isize) as i32)
                {
                    let fresh95 = offset;
                    offset = offset.wrapping_add(1);
                    let fresh96 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh96 as isize) = *src.offset(fresh95 as isize);
                }
            }
        }
        while offset < size {
            let mut end = 0;
            match *src.offset(offset as isize) as i32 {
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 46 | 101 | 69 | 43 | 45 => {
                    let fresh97 = offset;
                    offset = offset.wrapping_add(1);
                    let fresh98 = bytes_written;
                    bytes_written = bytes_written.wrapping_add(1);
                    *data.offset(fresh98 as isize) = *src.offset(fresh97 as isize);
                }
                _ => {
                    end = 1;
                }
            }
            if 0 != end {
                break;
            }
        }
        if json_parse_flags_allow_inf_and_nan as u64 & flags_bitset != 0 {
            let inf_strlen = 8;
            let nan_strlen = 3;
            if offset.wrapping_add(inf_strlen) < size {
                if 'I' as i32 == *src.offset(offset as isize) as i32 {
                    let mut i: u64 = 0;
                    i = 0;
                    while i < inf_strlen {
                        let fresh99 = offset;
                        offset = offset.wrapping_add(1);
                        let fresh100 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh100 as isize) = *src.offset(fresh99 as isize);
                        i = i.wrapping_add(1);
                    }
                }
            }
            if offset.wrapping_add(nan_strlen) < size {
                if 'N' as i32 == *src.offset(offset as isize) as i32 {
                    let mut i_0: u64 = 0;
                    i_0 = 0;
                    while i_0 < nan_strlen {
                        let fresh101 = offset;
                        offset = offset.wrapping_add(1);
                        let fresh102 = bytes_written;
                        bytes_written = bytes_written.wrapping_add(1);
                        *data.offset(fresh102 as isize) = *src.offset(fresh101 as isize);
                        i_0 = i_0.wrapping_add(1);
                    }
                }
            }
        };
        (*number).number_size = bytes_written;
        let fresh103 = bytes_written;
        bytes_written = bytes_written.wrapping_add(1);
        *data.offset(fresh103 as isize) = '\0' as i8;
        let ref mut fresh104 = (*state).data;
        *fresh104 = (*fresh104).offset(bytes_written as isize);
        (*state).offset = offset;
    }
}

#[no_mangle]
pub extern "C" fn json_parse_value(
    mut state: *mut json_parse_state_s,
    mut is_global_object: i32,
    mut value: *mut json_value_s,
) {
    unsafe {
        let flags_bitset = (*state).flags_bitset;
        let src = (*state).src;
        let size = (*state).size;
        let mut offset: u64 = 0;
        json_skip_all_skippables(state);
        offset = (*state).offset;
        if is_global_object != 0 {
            (*value).type_0 = json_type_object as u64;
            let ref mut fresh105 = (*value).payload;
            *fresh105 = (*state).dom as *mut libc::c_void;
            let ref mut fresh106 = (*state).dom;
            *fresh106 = (*fresh106).offset(::std::mem::size_of::<json_object_s>() as u64 as isize);
            json_parse_object(state, 1, (*value).payload as *mut json_object_s);
        } else {
            match *src.offset(offset as isize) as i32 {
                34 | 39 => {
                    (*value).type_0 = json_type_string as u64;
                    let ref mut fresh107 = (*value).payload;
                    *fresh107 = (*state).dom as *mut libc::c_void;
                    let ref mut fresh108 = (*state).dom;
                    *fresh108 =
                        (*fresh108).offset(::std::mem::size_of::<json_string_s>() as u64 as isize);
                    json_parse_string(state, (*value).payload as *mut json_string_s);
                }
                123 => {
                    (*value).type_0 = json_type_object as u64;
                    let ref mut fresh109 = (*value).payload;
                    *fresh109 = (*state).dom as *mut libc::c_void;
                    let ref mut fresh110 = (*state).dom;
                    *fresh110 =
                        (*fresh110).offset(::std::mem::size_of::<json_object_s>() as u64 as isize);
                    json_parse_object(state, 0, (*value).payload as *mut json_object_s);
                }
                91 => {
                    (*value).type_0 = json_type_array as u64;
                    let ref mut fresh111 = (*value).payload;
                    *fresh111 = (*state).dom as *mut libc::c_void;
                    let ref mut fresh112 = (*state).dom;
                    *fresh112 =
                        (*fresh112).offset(::std::mem::size_of::<json_array_s>() as u64 as isize);
                    json_parse_array(state, (*value).payload as *mut json_array_s);
                }
                45 | 43 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 46 => {
                    (*value).type_0 = json_type_number as u64;
                    let ref mut fresh113 = (*value).payload;
                    *fresh113 = (*state).dom as *mut libc::c_void;
                    let ref mut fresh114 = (*state).dom;
                    *fresh114 =
                        (*fresh114).offset(::std::mem::size_of::<json_number_s>() as u64 as isize);
                    json_parse_number(state, (*value).payload as *mut json_number_s);
                }
                _ => {
                    if offset.wrapping_add(4) <= size
                        && 't' as i32 == *src.offset(offset.wrapping_add(0) as isize) as i32
                        && 'r' as i32 == *src.offset(offset.wrapping_add(1) as isize) as i32
                        && 'u' as i32 == *src.offset(offset.wrapping_add(2) as isize) as i32
                        && 'e' as i32 == *src.offset(offset.wrapping_add(3) as isize) as i32
                    {
                        (*value).type_0 = json_type_true as u64;
                        let ref mut fresh115 = (*value).payload;
                        *fresh115 = 0 as *mut libc::c_void;
                        let ref mut fresh116 = (*state).offset;
                        *fresh116 = (*fresh116 as u64).wrapping_add(4) as u64;
                    } else if offset.wrapping_add(5) <= size
                        && 'f' as i32 == *src.offset(offset.wrapping_add(0) as isize) as i32
                        && 'a' as i32 == *src.offset(offset.wrapping_add(1) as isize) as i32
                        && 'l' as i32 == *src.offset(offset.wrapping_add(2) as isize) as i32
                        && 's' as i32 == *src.offset(offset.wrapping_add(3) as isize) as i32
                        && 'e' as i32 == *src.offset(offset.wrapping_add(4) as isize) as i32
                    {
                        (*value).type_0 = json_type_false as u64;
                        let ref mut fresh117 = (*value).payload;
                        *fresh117 = 0 as *mut libc::c_void;
                        let ref mut fresh118 = (*state).offset;
                        *fresh118 = (*fresh118 as u64).wrapping_add(5) as u64;
                    } else if offset.wrapping_add(4) <= size
                        && 'n' as i32 == *src.offset(offset.wrapping_add(0) as isize) as i32
                        && 'u' as i32 == *src.offset(offset.wrapping_add(1) as isize) as i32
                        && 'l' as i32 == *src.offset(offset.wrapping_add(2) as isize) as i32
                        && 'l' as i32 == *src.offset(offset.wrapping_add(3) as isize) as i32
                    {
                        (*value).type_0 = json_type_null as u64;
                        let ref mut fresh119 = (*value).payload;
                        *fresh119 = 0 as *mut libc::c_void;
                        let ref mut fresh120 = (*state).offset;
                        *fresh120 = (*fresh120 as u64).wrapping_add(4) as u64;
                    } else if json_parse_flags_allow_inf_and_nan as u64 & flags_bitset != 0
                        && offset.wrapping_add(3) <= size
                        && 'N' as i32 == *src.offset(offset.wrapping_add(0) as isize) as i32
                        && 'a' as i32 == *src.offset(offset.wrapping_add(1) as isize) as i32
                        && 'N' as i32 == *src.offset(offset.wrapping_add(2) as isize) as i32
                    {
                        (*value).type_0 = json_type_number as u64;
                        let ref mut fresh121 = (*value).payload;
                        *fresh121 = (*state).dom as *mut libc::c_void;
                        let ref mut fresh122 = (*state).dom;
                        *fresh122 = (*fresh122)
                            .offset(::std::mem::size_of::<json_number_s>() as u64 as isize);
                        json_parse_number(state, (*value).payload as *mut json_number_s);
                    } else if json_parse_flags_allow_inf_and_nan as u64 & flags_bitset != 0
                        && offset.wrapping_add(8) <= size
                        && 'I' as i32 == *src.offset(offset.wrapping_add(0) as isize) as i32
                        && 'n' as i32 == *src.offset(offset.wrapping_add(1) as isize) as i32
                        && 'f' as i32 == *src.offset(offset.wrapping_add(2) as isize) as i32
                        && 'i' as i32 == *src.offset(offset.wrapping_add(3) as isize) as i32
                        && 'n' as i32 == *src.offset(offset.wrapping_add(4) as isize) as i32
                        && 'i' as i32 == *src.offset(offset.wrapping_add(5) as isize) as i32
                        && 't' as i32 == *src.offset(offset.wrapping_add(6) as isize) as i32
                        && 'y' as i32 == *src.offset(offset.wrapping_add(7) as isize) as i32
                    {
                        (*value).type_0 = json_type_number as u64;
                        let ref mut fresh123 = (*value).payload;
                        *fresh123 = (*state).dom as *mut libc::c_void;
                        let ref mut fresh124 = (*state).dom;
                        *fresh124 = (*fresh124)
                            .offset(::std::mem::size_of::<json_number_s>() as u64 as isize);
                        json_parse_number(state, (*value).payload as *mut json_number_s);
                    }
                }
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn json_parse_ex(
    mut src: *const libc::c_void,
    mut src_size: u64,
    mut flags_bitset: u64,
    mut alloc_func_ptr: Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>,
    mut user_data: *mut libc::c_void,
    mut result: *mut json_parse_result_s,
) -> *mut json_value_s {
    unsafe {
        let mut state = json_parse_state_s {
            src: 0 as *const i8,
            size: 0,
            offset: 0,
            flags_bitset: 0,
            data: 0 as *mut i8,
            dom: 0 as *mut i8,
            dom_size: 0,
            data_size: 0,
            line_no: 0,
            line_offset: 0,
            error: 0,
        };
        let mut allocation = 0 as *mut libc::c_void;
        let mut value = 0 as *mut json_value_s;
        let mut total_size: u64 = 0;
        let mut input_error: i32 = 0;
        if !result.is_null() {
            (*result).error = json_parse_error_none as u64;
            (*result).error_offset = 0;
            (*result).error_line_no = 0;
            (*result).error_row_no = 0;
        }
        if src.is_null() {
            return 0 as *mut json_value_s;
        }
        state.src = src as *const i8;
        state.size = src_size;
        state.offset = 0;
        state.line_no = 1;
        state.line_offset = 0;
        state.error = json_parse_error_none as u64;
        state.dom_size = 0;
        state.data_size = 0;
        state.flags_bitset = flags_bitset;
        input_error = json_get_value_size(
            &mut state,
            (json_parse_flags_allow_global_object as u64 & state.flags_bitset) as i32,
        );
        if 0 == input_error {
            json_skip_all_skippables(&mut state);
            if state.offset != state.size {
                state.error = json_parse_error_unexpected_trailing_characters as u64;
                input_error = 1;
            }
        }
        if input_error != 0 {
            if !result.is_null() {
                (*result).error = state.error;
                (*result).error_offset = state.offset;
                (*result).error_line_no = state.line_no;
                (*result).error_row_no = (state.offset).wrapping_sub(state.line_offset);
            }
            return 0 as *mut json_value_s;
        }
        total_size = (state.dom_size).wrapping_add(state.data_size);
        if alloc_func_ptr.is_none() {
            allocation = malloc(total_size);
        } else {
            allocation = alloc_func_ptr.expect("non-null function pointer")(user_data, total_size);
        }
        if allocation.is_null() {
            if !result.is_null() {
                (*result).error = json_parse_error_allocator_failed as u64;
                (*result).error_offset = 0;
                (*result).error_line_no = 0;
                (*result).error_row_no = 0;
            }
            return 0 as *mut json_value_s;
        }
        state.offset = 0;
        state.line_no = 1;
        state.line_offset = 0;
        state.dom = allocation as *mut i8;
        state.data = (state.dom).offset(state.dom_size as isize);
        if json_parse_flags_allow_location_information as u64 & state.flags_bitset != 0 {
            let mut value_ex = state.dom as *mut json_value_ex_s;
            state.dom =
                (state.dom).offset(::std::mem::size_of::<json_value_ex_s>() as u64 as isize);
            (*value_ex).offset = state.offset;
            (*value_ex).line_no = state.line_no;
            (*value_ex).row_no = (state.offset).wrapping_sub(state.line_offset);
            value = &mut (*value_ex).value;
        } else {
            value = state.dom as *mut json_value_s;
            state.dom = (state.dom).offset(::std::mem::size_of::<json_value_s>() as u64 as isize);
        }
        json_parse_value(
            &mut state,
            (json_parse_flags_allow_global_object as u64 & state.flags_bitset) as i32,
            value,
        );
        return allocation as *mut json_value_s;
    }
}

#[no_mangle]
pub extern "C" fn json_parse(mut src: *const libc::c_void, mut src_size: u64) -> *mut json_value_s {
    unsafe {
        return json_parse_ex(
            src,
            src_size,
            json_parse_flags_default as u64,
            None,
            0 as *mut libc::c_void,
            0 as *mut json_parse_result_s,
        );
    }
}

#[no_mangle]
pub extern "C" fn json_extract_value(mut value: *const json_value_s) -> *mut json_value_s {
    unsafe {
        return json_extract_value_ex(value, None, 0 as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn json_extract_get_number_size(
    number: *const json_number_s,
) -> json_extract_result_s {
    unsafe {
        let mut result = json_extract_result_s {
            dom_size: 0,
            data_size: 0,
        };
        result.dom_size = ::std::mem::size_of::<json_number_s>() as u64;
        result.data_size = (*number).number_size;
        return result;
    }
}

#[no_mangle]
pub extern "C" fn json_extract_get_string_size(
    string: *const json_string_s,
) -> json_extract_result_s {
    unsafe {
        let mut result = json_extract_result_s {
            dom_size: 0,
            data_size: 0,
        };
        result.dom_size = ::std::mem::size_of::<json_string_s>() as u64;
        result.data_size = ((*string).string_size).wrapping_add(1);
        return result;
    }
}

#[no_mangle]
pub extern "C" fn json_extract_get_object_size(
    object: *const json_object_s,
) -> json_extract_result_s {
    unsafe {
        let mut result = json_extract_result_s {
            dom_size: 0,
            data_size: 0,
        };
        let mut i: u64 = 0;
        let mut element: *const json_object_element_s = (*object).start;
        result.dom_size = (::std::mem::size_of::<json_object_s>() as u64).wrapping_add(
            (::std::mem::size_of::<json_object_element_s>() as u64).wrapping_mul((*object).length),
        );
        result.data_size = 0;
        i = 0;
        while i < (*object).length {
            let string_result = json_extract_get_string_size((*element).name);
            let value_result = json_extract_get_value_size((*element).value);
            result.dom_size = (result.dom_size as u64).wrapping_add(string_result.dom_size) as u64;
            result.data_size =
                (result.data_size as u64).wrapping_add(string_result.data_size) as u64;
            result.dom_size = (result.dom_size as u64).wrapping_add(value_result.dom_size) as u64;
            result.data_size =
                (result.data_size as u64).wrapping_add(value_result.data_size) as u64;
            element = (*element).next;
            i = i.wrapping_add(1);
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn json_extract_get_array_size(array: *const json_array_s) -> json_extract_result_s {
    unsafe {
        let mut result = json_extract_result_s {
            dom_size: 0,
            data_size: 0,
        };
        let mut i: u64 = 0;
        let mut element: *const json_array_element_s = (*array).start;
        result.dom_size = (::std::mem::size_of::<json_array_s>() as u64).wrapping_add(
            (::std::mem::size_of::<json_array_element_s>() as u64).wrapping_mul((*array).length),
        );
        result.data_size = 0;
        i = 0;
        while i < (*array).length {
            let value_result = json_extract_get_value_size((*element).value);
            result.dom_size = (result.dom_size as u64).wrapping_add(value_result.dom_size) as u64;
            result.data_size =
                (result.data_size as u64).wrapping_add(value_result.data_size) as u64;
            element = (*element).next;
            i = i.wrapping_add(1);
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn json_extract_get_value_size(value: *const json_value_s) -> json_extract_result_s {
    unsafe {
        let mut result = {
            let mut init = json_extract_result_s {
                dom_size: 0,
                data_size: 0,
            };
            init
        };
        match (*value).type_0 {
            2 => {
                result = json_extract_get_object_size((*value).payload as *const json_object_s);
            }
            3 => {
                result = json_extract_get_array_size((*value).payload as *const json_array_s);
            }
            1 => {
                result = json_extract_get_number_size((*value).payload as *const json_number_s);
            }
            0 => {
                result = json_extract_get_string_size((*value).payload as *const json_string_s);
            }
            _ => {}
        }
        result.dom_size = (result.dom_size as u64)
            .wrapping_add(::std::mem::size_of::<json_value_s>() as u64)
            as u64;
        return result;
    }
}

#[no_mangle]
pub extern "C" fn json_extract_copy_value(
    mut state: *mut json_extract_state_s,
    value: *const json_value_s,
) {
    unsafe {
        let mut string = 0 as *mut json_string_s;
        let mut number = 0 as *mut json_number_s;
        let mut object = 0 as *mut json_object_s;
        let mut array = 0 as *mut json_array_s;
        let mut new_value = 0 as *mut json_value_s;
        memcpy(
            (*state).dom as *mut libc::c_void,
            value as *const libc::c_void,
            ::std::mem::size_of::<json_value_s>() as u64,
        );
        new_value = (*state).dom as *mut json_value_s;
        let ref mut fresh125 = (*state).dom;
        *fresh125 = (*fresh125).offset(::std::mem::size_of::<json_value_s>() as u64 as isize);
        let ref mut fresh126 = (*new_value).payload;
        *fresh126 = (*state).dom as *mut libc::c_void;
        if json_type_string as u64 == (*value).type_0 {
            memcpy(
                (*state).dom as *mut libc::c_void,
                (*value).payload,
                ::std::mem::size_of::<json_string_s>() as u64,
            );
            string = (*state).dom as *mut json_string_s;
            let ref mut fresh127 = (*state).dom;
            *fresh127 = (*fresh127).offset(::std::mem::size_of::<json_string_s>() as u64 as isize);
            memcpy(
                (*state).data as *mut libc::c_void,
                (*string).string as *const libc::c_void,
                ((*string).string_size).wrapping_add(1),
            );
            let ref mut fresh128 = (*string).string;
            *fresh128 = (*state).data;
            let ref mut fresh129 = (*state).data;
            *fresh129 = (*fresh129).offset(((*string).string_size).wrapping_add(1) as isize);
        } else if json_type_number as u64 == (*value).type_0 {
            memcpy(
                (*state).dom as *mut libc::c_void,
                (*value).payload,
                ::std::mem::size_of::<json_number_s>() as u64,
            );
            number = (*state).dom as *mut json_number_s;
            let ref mut fresh130 = (*state).dom;
            *fresh130 = (*fresh130).offset(::std::mem::size_of::<json_number_s>() as u64 as isize);
            memcpy(
                (*state).data as *mut libc::c_void,
                (*number).number as *const libc::c_void,
                (*number).number_size,
            );
            let ref mut fresh131 = (*number).number;
            *fresh131 = (*state).data;
            let ref mut fresh132 = (*state).data;
            *fresh132 = (*fresh132).offset((*number).number_size as isize);
        } else if json_type_object as u64 == (*value).type_0 {
            let mut element = 0 as *mut json_object_element_s;
            let mut i: u64 = 0;
            memcpy(
                (*state).dom as *mut libc::c_void,
                (*value).payload,
                ::std::mem::size_of::<json_object_s>() as u64,
            );
            object = (*state).dom as *mut json_object_s;
            let ref mut fresh133 = (*state).dom;
            *fresh133 = (*fresh133).offset(::std::mem::size_of::<json_object_s>() as u64 as isize);
            element = (*object).start;
            let ref mut fresh134 = (*object).start;
            *fresh134 = (*state).dom as *mut json_object_element_s;
            i = 0;
            while i < (*object).length {
                let mut previous_value = 0 as *mut json_value_s;
                let mut previous_element = 0 as *mut json_object_element_s;
                memcpy(
                    (*state).dom as *mut libc::c_void,
                    element as *const libc::c_void,
                    ::std::mem::size_of::<json_object_element_s>() as u64,
                );
                element = (*state).dom as *mut json_object_element_s;
                let ref mut fresh135 = (*state).dom;
                *fresh135 = (*fresh135)
                    .offset(::std::mem::size_of::<json_object_element_s>() as u64 as isize);
                string = (*element).name;
                memcpy(
                    (*state).dom as *mut libc::c_void,
                    string as *const libc::c_void,
                    ::std::mem::size_of::<json_string_s>() as u64,
                );
                string = (*state).dom as *mut json_string_s;
                let ref mut fresh136 = (*state).dom;
                *fresh136 =
                    (*fresh136).offset(::std::mem::size_of::<json_string_s>() as u64 as isize);
                let ref mut fresh137 = (*element).name;
                *fresh137 = string;
                memcpy(
                    (*state).data as *mut libc::c_void,
                    (*string).string as *const libc::c_void,
                    ((*string).string_size).wrapping_add(1),
                );
                let ref mut fresh138 = (*string).string;
                *fresh138 = (*state).data;
                let ref mut fresh139 = (*state).data;
                *fresh139 = (*fresh139).offset(((*string).string_size).wrapping_add(1) as isize);
                previous_value = (*element).value;
                let ref mut fresh140 = (*element).value;
                *fresh140 = (*state).dom as *mut json_value_s;
                json_extract_copy_value(state, previous_value);
                previous_element = element;
                element = (*element).next;
                if !element.is_null() {
                    let ref mut fresh141 = (*previous_element).next;
                    *fresh141 = (*state).dom as *mut json_object_element_s;
                }
                i = i.wrapping_add(1);
            }
        } else if json_type_array as u64 == (*value).type_0 {
            let mut element_0 = 0 as *mut json_array_element_s;
            let mut i_0: u64 = 0;
            memcpy(
                (*state).dom as *mut libc::c_void,
                (*value).payload,
                ::std::mem::size_of::<json_array_s>() as u64,
            );
            array = (*state).dom as *mut json_array_s;
            let ref mut fresh142 = (*state).dom;
            *fresh142 = (*fresh142).offset(::std::mem::size_of::<json_array_s>() as u64 as isize);
            element_0 = (*array).start;
            let ref mut fresh143 = (*array).start;
            *fresh143 = (*state).dom as *mut json_array_element_s;
            i_0 = 0;
            while i_0 < (*array).length {
                let mut previous_value_0 = 0 as *mut json_value_s;
                let mut previous_element_0 = 0 as *mut json_array_element_s;
                memcpy(
                    (*state).dom as *mut libc::c_void,
                    element_0 as *const libc::c_void,
                    ::std::mem::size_of::<json_array_element_s>() as u64,
                );
                element_0 = (*state).dom as *mut json_array_element_s;
                let ref mut fresh144 = (*state).dom;
                *fresh144 = (*fresh144)
                    .offset(::std::mem::size_of::<json_array_element_s>() as u64 as isize);
                previous_value_0 = (*element_0).value;
                let ref mut fresh145 = (*element_0).value;
                *fresh145 = (*state).dom as *mut json_value_s;
                json_extract_copy_value(state, previous_value_0);
                previous_element_0 = element_0;
                element_0 = (*element_0).next;
                if !element_0.is_null() {
                    let ref mut fresh146 = (*previous_element_0).next;
                    *fresh146 = (*state).dom as *mut json_array_element_s;
                }
                i_0 = i_0.wrapping_add(1);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn json_extract_value_ex(
    mut value: *const json_value_s,
    mut alloc_func_ptr: Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>,
    mut user_data: *mut libc::c_void,
) -> *mut json_value_s {
    unsafe {
        let mut allocation = 0 as *mut libc::c_void;
        let mut result = json_extract_result_s {
            dom_size: 0,
            data_size: 0,
        };
        let mut state = json_extract_state_s {
            dom: 0 as *mut i8,
            data: 0 as *mut i8,
        };
        let mut total_size: u64 = 0;
        if value.is_null() {
            return 0 as *mut json_value_s;
        }
        result = json_extract_get_value_size(value);
        total_size = (result.dom_size).wrapping_add(result.data_size);
        if alloc_func_ptr.is_none() {
            allocation = malloc(total_size);
        } else {
            allocation = alloc_func_ptr.expect("non-null function pointer")(user_data, total_size);
        }
        state.dom = allocation as *mut i8;
        state.data = (state.dom).offset(result.dom_size as isize);
        json_extract_copy_value(&mut state, value);
        return allocation as *mut json_value_s;
    }
}

#[no_mangle]
pub extern "C" fn json_value_as_string(value: *mut json_value_s) -> *mut json_string_s {
    unsafe {
        if (*value).type_0 != json_type_string as u64 {
            return 0 as *mut json_string_s;
        }
        return (*value).payload as *mut json_string_s;
    }
}

#[no_mangle]
pub extern "C" fn json_value_as_number(value: *mut json_value_s) -> *mut json_number_s {
    unsafe {
        if (*value).type_0 != json_type_number as u64 {
            return 0 as *mut json_number_s;
        }
        return (*value).payload as *mut json_number_s;
    }
}

#[no_mangle]
pub extern "C" fn json_value_as_object(value: *mut json_value_s) -> *mut json_object_s {
    unsafe {
        if (*value).type_0 != json_type_object as u64 {
            return 0 as *mut json_object_s;
        }
        return (*value).payload as *mut json_object_s;
    }
}

#[no_mangle]
pub extern "C" fn json_value_as_array(value: *mut json_value_s) -> *mut json_array_s {
    unsafe {
        if (*value).type_0 != json_type_array as u64 {
            return 0 as *mut json_array_s;
        }
        return (*value).payload as *mut json_array_s;
    }
}

#[no_mangle]
pub extern "C" fn json_value_is_true(value: *const json_value_s) -> i32 {
    unsafe {
        return ((*value).type_0 == json_type_true as u64) as i32;
    }
}

#[no_mangle]
pub extern "C" fn json_value_is_false(value: *const json_value_s) -> i32 {
    unsafe {
        return ((*value).type_0 == json_type_false as u64) as i32;
    }
}

#[no_mangle]
pub extern "C" fn json_value_is_null(value: *const json_value_s) -> i32 {
    unsafe {
        return ((*value).type_0 == json_type_null as u64) as i32;
    }
}

#[no_mangle]
pub extern "C" fn json_write_get_number_size(
    mut number: *const json_number_s,
    mut size: *mut u64,
) -> i32 {
    unsafe {
        let mut parsed_number: u64 = 0;
        let mut i: u64 = 0;
        if (*number).number_size >= 2 {
            match *((*number).number).offset(1 as isize) as i32 {
                120 | 88 => {
                    parsed_number = strtoumax((*number).number, 0 as *mut *mut i8, 0);
                    i = 0;
                    while 0 != parsed_number {
                        parsed_number = (parsed_number).wrapping_div(10) as u64;
                        i = i.wrapping_add(1);
                    }
                    *size = (*size as u64).wrapping_add(i) as u64;
                    return 0;
                }
                _ => {}
            }
        }
        i = 0;
        if i < (*number).number_size
            && ('+' as i32 == *((*number).number).offset(i as isize) as i32
                || '-' as i32 == *((*number).number).offset(i as isize) as i32)
        {
            i = i.wrapping_add(1);
        }
        if i < (*number).number_size && 'I' as i32 == *((*number).number).offset(i as isize) as i32
        {
            let mut inf = b"Infinity\0" as *const u8 as *const i8;
            let mut k: u64 = 0;
            k = i;
            while k < (*number).number_size {
                let fresh147 = inf;
                inf = inf.offset(1);
                let c = *fresh147;
                if '\0' as i32 == c as i32 {
                    break;
                }
                if c as i32 != *((*number).number).offset(k as isize) as i32 {
                    break;
                }
                k = k.wrapping_add(1);
            }
            if '\0' as i32 == *inf as i32 {
                *size = (*size as u64).wrapping_add(22) as u64;
                if '-' as i32 == *((*number).number).offset(0 as isize) as i32 {
                    *size = (*size as u64).wrapping_add(1) as u64;
                }
            }
            return 0;
        }
        if i < (*number).number_size && 'N' as i32 == *((*number).number).offset(i as isize) as i32
        {
            let mut nan = b"NaN\0" as *const u8 as *const i8;
            let mut k_0: u64 = 0;
            k_0 = i;
            while k_0 < (*number).number_size {
                let fresh148 = nan;
                nan = nan.offset(1);
                let c_0 = *fresh148;
                if '\0' as i32 == c_0 as i32 {
                    break;
                }
                if c_0 as i32 != *((*number).number).offset(k_0 as isize) as i32 {
                    break;
                }
                k_0 = k_0.wrapping_add(1);
            }
            if '\0' as i32 == *nan as i32 {
                *size = (*size as u64).wrapping_add(1) as u64;
                return 0;
            }
        }
        if i < (*number).number_size && '.' as i32 == *((*number).number).offset(i as isize) as i32
        {
            *size = (*size as u64).wrapping_add(1) as u64;
        } else {
            while i < (*number).number_size {
                let c_1 = *((*number).number).offset(i as isize);
                if !('0' as i32 <= c_1 as i32 && c_1 as i32 <= '9' as i32) {
                    break;
                }
                i = i.wrapping_add(1);
            }
            if i.wrapping_add(1) == (*number).number_size
                && '.' as i32 == *((*number).number).offset(i as isize) as i32
            {
                *size = (*size as u64).wrapping_add(1) as u64;
            }
        }
        *size = (*size as u64).wrapping_add((*number).number_size) as u64;
        if '+' as i32 == *((*number).number).offset(0 as isize) as i32 {
            *size = (*size as u64).wrapping_sub(1) as u64;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn json_write_get_string_size(
    mut string: *const json_string_s,
    mut size: *mut u64,
) -> i32 {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i < (*string).string_size {
            match *((*string).string).offset(i as isize) as i32 {
                34 | 92 | 8 | 12 | 10 | 13 | 9 => {
                    *size = (*size as u64).wrapping_add(2) as u64;
                }
                _ => {
                    *size = (*size as u64).wrapping_add(1) as u64;
                }
            }
            i = i.wrapping_add(1);
        }
        *size = (*size as u64).wrapping_add(2) as u64;
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn json_write_minified_get_array_size(
    mut array: *const json_array_s,
    mut size: *mut u64,
) -> i32 {
    unsafe {
        let mut element = 0 as *mut json_array_element_s;
        *size = (*size as u64).wrapping_add(2) as u64;
        if 1 < (*array).length {
            *size = (*size as u64).wrapping_add(((*array).length).wrapping_sub(1)) as u64;
        }
        element = (*array).start;
        while !element.is_null() {
            if json_write_minified_get_value_size((*element).value, size) != 0 {
                return 1;
            }
            element = (*element).next;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn json_write_minified_get_object_size(
    mut object: *const json_object_s,
    mut size: *mut u64,
) -> i32 {
    unsafe {
        let mut element = 0 as *mut json_object_element_s;
        *size = (*size as u64).wrapping_add(2) as u64;
        *size = (*size as u64).wrapping_add((*object).length) as u64;
        if 1 < (*object).length {
            *size = (*size as u64).wrapping_add(((*object).length).wrapping_sub(1)) as u64;
        }
        element = (*object).start;
        while !element.is_null() {
            if json_write_get_string_size((*element).name, size) != 0 {
                return 1;
            }
            if json_write_minified_get_value_size((*element).value, size) != 0 {
                return 1;
            }
            element = (*element).next;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn json_write_minified_get_value_size(
    mut value: *const json_value_s,
    mut size: *mut u64,
) -> i32 {
    unsafe {
        match (*value).type_0 {
            1 => {
                return json_write_get_number_size((*value).payload as *mut json_number_s, size);
            }
            0 => {
                return json_write_get_string_size((*value).payload as *mut json_string_s, size);
            }
            3 => {
                return json_write_minified_get_array_size(
                    (*value).payload as *mut json_array_s,
                    size,
                );
            }
            2 => {
                return json_write_minified_get_object_size(
                    (*value).payload as *mut json_object_s,
                    size,
                );
            }
            4 => {
                *size = (*size as u64).wrapping_add(4) as u64;
                return 0;
            }
            5 => {
                *size = (*size as u64).wrapping_add(5) as u64;
                return 0;
            }
            6 => {
                *size = (*size as u64).wrapping_add(4) as u64;
                return 0;
            }
            _ => return 1,
        };
    }
}

#[no_mangle]
pub extern "C" fn json_write_number(
    mut number: *const json_number_s,
    mut data: *mut i8,
) -> *mut i8 {
    unsafe {
        let mut parsed_number: u64 = 0;
        let mut backup: u64 = 0;
        let mut i: u64 = 0;
        if (*number).number_size >= 2 {
            match *((*number).number).offset(1 as isize) as i32 {
                120 | 88 => {
                    parsed_number = strtoumax((*number).number, 0 as *mut *mut i8, 0);
                    backup = parsed_number;
                    i = 0;
                    while 0 != parsed_number {
                        parsed_number = (parsed_number).wrapping_div(10) as u64;
                        i = i.wrapping_add(1);
                    }
                    parsed_number = backup;
                    backup = i;
                    loop {
                        *data.offset(i as isize).offset(-(1 as isize)) =
                            ('0' as i32 + parsed_number.wrapping_rem(10u64) as i32) as i8;
                        parsed_number = (parsed_number).wrapping_div(10) as u64;
                        i = i.wrapping_sub(1);
                        if !(0 != parsed_number) {
                            break;
                        }
                    }
                    data = data.offset(backup as isize);
                    return data;
                }
                _ => {}
            }
        }
        i = 0;
        if i < (*number).number_size
            && ('+' as i32 == *((*number).number).offset(i as isize) as i32
                || '-' as i32 == *((*number).number).offset(i as isize) as i32)
        {
            i = i.wrapping_add(1);
        }
        if i < (*number).number_size && 'I' as i32 == *((*number).number).offset(i as isize) as i32
        {
            let mut inf = b"Infinity\0" as *const u8 as *const i8;
            let mut k: u64 = 0;
            k = i;
            while k < (*number).number_size {
                let fresh149 = inf;
                inf = inf.offset(1);
                let c = *fresh149;
                if '\0' as i32 == c as i32 {
                    break;
                }
                if c as i32 != *((*number).number).offset(k as isize) as i32 {
                    break;
                }
                k = k.wrapping_add(1);
            }
            let fresh150 = inf;
            inf = inf.offset(1);
            if '\0' as i32 == *fresh150 as i32 {
                let mut dbl_max = 0 as *const i8;
                if '-' as i32 == *((*number).number).offset(0 as isize) as i32 {
                    let fresh151 = data;
                    data = data.offset(1);
                    *fresh151 = '-' as i8;
                }
                dbl_max = b"1.7976931348623158e308\0" as *const u8 as *const i8;
                while '\0' as i32 != *dbl_max as i32 {
                    let fresh152 = data;
                    data = data.offset(1);
                    *fresh152 = *dbl_max;
                    dbl_max = dbl_max.offset(1);
                }
                return data;
            }
        }
        if i < (*number).number_size && 'N' as i32 == *((*number).number).offset(i as isize) as i32
        {
            let mut nan = b"NaN\0" as *const u8 as *const i8;
            let mut k_0: u64 = 0;
            k_0 = i;
            while k_0 < (*number).number_size {
                let fresh153 = nan;
                nan = nan.offset(1);
                let c_0 = *fresh153;
                if '\0' as i32 == c_0 as i32 {
                    break;
                }
                if c_0 as i32 != *((*number).number).offset(k_0 as isize) as i32 {
                    break;
                }
                k_0 = k_0.wrapping_add(1);
            }
            let fresh154 = nan;
            nan = nan.offset(1);
            if '\0' as i32 == *fresh154 as i32 {
                let fresh155 = data;
                data = data.offset(1);
                *fresh155 = '0' as i8;
                return data;
            }
        }
        if i < (*number).number_size && '.' as i32 == *((*number).number).offset(i as isize) as i32
        {
            i = 0;
            if '+' as i32 == *((*number).number).offset(i as isize) as i32 {
                i = i.wrapping_add(1);
            }
            if '-' as i32 == *((*number).number).offset(i as isize) as i32 {
                let fresh156 = data;
                data = data.offset(1);
                *fresh156 = '-' as i8;
                i = i.wrapping_add(1);
            }
            let fresh157 = data;
            data = data.offset(1);
            *fresh157 = '0' as i8;
            while i < (*number).number_size {
                let fresh158 = data;
                data = data.offset(1);
                *fresh158 = *((*number).number).offset(i as isize);
                i = i.wrapping_add(1);
            }
            return data;
        }
        while i < (*number).number_size {
            let c_1 = *((*number).number).offset(i as isize);
            if !('0' as i32 <= c_1 as i32 && c_1 as i32 <= '9' as i32) {
                break;
            }
            i = i.wrapping_add(1);
        }
        if i.wrapping_add(1) == (*number).number_size
            && '.' as i32 == *((*number).number).offset(i as isize) as i32
        {
            i = 0;
            if '+' as i32 == *((*number).number).offset(i as isize) as i32 {
                i = i.wrapping_add(1);
            }
            if '-' as i32 == *((*number).number).offset(i as isize) as i32 {
                let fresh159 = data;
                data = data.offset(1);
                *fresh159 = '-' as i8;
                i = i.wrapping_add(1);
            }
            while i < (*number).number_size {
                let fresh160 = data;
                data = data.offset(1);
                *fresh160 = *((*number).number).offset(i as isize);
                i = i.wrapping_add(1);
            }
            let fresh161 = data;
            data = data.offset(1);
            *fresh161 = '0' as i8;
            return data;
        }
        i = 0;
        if '+' as i32 == *((*number).number).offset(i as isize) as i32 {
            i = i.wrapping_add(1);
        }
        while i < (*number).number_size {
            let fresh162 = data;
            data = data.offset(1);
            *fresh162 = *((*number).number).offset(i as isize);
            i = i.wrapping_add(1);
        }
        return data;
    }
}

#[no_mangle]
pub extern "C" fn json_write_string(
    mut string: *const json_string_s,
    mut data: *mut i8,
) -> *mut i8 {
    unsafe {
        let mut i: u64 = 0;
        let fresh163 = data;
        data = data.offset(1);
        *fresh163 = '"' as i8;
        i = 0;
        while i < (*string).string_size {
            match *((*string).string).offset(i as isize) as i32 {
                34 => {
                    let fresh164 = data;
                    data = data.offset(1);
                    *fresh164 = '\\' as i8;
                    let fresh165 = data;
                    data = data.offset(1);
                    *fresh165 = '"' as i8;
                }
                92 => {
                    let fresh166 = data;
                    data = data.offset(1);
                    *fresh166 = '\\' as i8;
                    let fresh167 = data;
                    data = data.offset(1);
                    *fresh167 = '\\' as i8;
                }
                8 => {
                    let fresh168 = data;
                    data = data.offset(1);
                    *fresh168 = '\\' as i8;
                    let fresh169 = data;
                    data = data.offset(1);
                    *fresh169 = 'b' as i8;
                }
                12 => {
                    let fresh170 = data;
                    data = data.offset(1);
                    *fresh170 = '\\' as i8;
                    let fresh171 = data;
                    data = data.offset(1);
                    *fresh171 = 'f' as i8;
                }
                10 => {
                    let fresh172 = data;
                    data = data.offset(1);
                    *fresh172 = '\\' as i8;
                    let fresh173 = data;
                    data = data.offset(1);
                    *fresh173 = 'n' as i8;
                }
                13 => {
                    let fresh174 = data;
                    data = data.offset(1);
                    *fresh174 = '\\' as i8;
                    let fresh175 = data;
                    data = data.offset(1);
                    *fresh175 = 'r' as i8;
                }
                9 => {
                    let fresh176 = data;
                    data = data.offset(1);
                    *fresh176 = '\\' as i8;
                    let fresh177 = data;
                    data = data.offset(1);
                    *fresh177 = 't' as i8;
                }
                _ => {
                    let fresh178 = data;
                    data = data.offset(1);
                    *fresh178 = *((*string).string).offset(i as isize);
                }
            }
            i = i.wrapping_add(1);
        }
        let fresh179 = data;
        data = data.offset(1);
        *fresh179 = '"' as i8;
        return data;
    }
}

#[no_mangle]
pub extern "C" fn json_write_minified_array(
    mut array: *const json_array_s,
    mut data: *mut i8,
) -> *mut i8 {
    unsafe {
        let mut element = 0 as *mut json_array_element_s;
        let fresh180 = data;
        data = data.offset(1);
        *fresh180 = '[' as i8;
        element = (*array).start;
        while !element.is_null() {
            if element != (*array).start {
                let fresh181 = data;
                data = data.offset(1);
                *fresh181 = ',' as i8;
            }
            data = json_write_minified_value((*element).value, data);
            if data.is_null() {
                return 0 as *mut i8;
            }
            element = (*element).next;
        }
        let fresh182 = data;
        data = data.offset(1);
        *fresh182 = ']' as i8;
        return data;
    }
}

#[no_mangle]
pub extern "C" fn json_write_minified_object(
    mut object: *const json_object_s,
    mut data: *mut i8,
) -> *mut i8 {
    unsafe {
        let mut element = 0 as *mut json_object_element_s;
        let fresh183 = data;
        data = data.offset(1);
        *fresh183 = '{' as i8;
        element = (*object).start;
        while !element.is_null() {
            if element != (*object).start {
                let fresh184 = data;
                data = data.offset(1);
                *fresh184 = ',' as i8;
            }
            data = json_write_string((*element).name, data);
            if data.is_null() {
                return 0 as *mut i8;
            }
            let fresh185 = data;
            data = data.offset(1);
            *fresh185 = ':' as i8;
            data = json_write_minified_value((*element).value, data);
            if data.is_null() {
                return 0 as *mut i8;
            }
            element = (*element).next;
        }
        let fresh186 = data;
        data = data.offset(1);
        *fresh186 = '}' as i8;
        return data;
    }
}

#[no_mangle]
pub extern "C" fn json_write_minified_value(
    mut value: *const json_value_s,
    mut data: *mut i8,
) -> *mut i8 {
    unsafe {
        match (*value).type_0 {
            1 => return json_write_number((*value).payload as *mut json_number_s, data),
            0 => return json_write_string((*value).payload as *mut json_string_s, data),
            3 => {
                return json_write_minified_array((*value).payload as *mut json_array_s, data);
            }
            2 => {
                return json_write_minified_object((*value).payload as *mut json_object_s, data);
            }
            4 => {
                *data.offset(0 as isize) = 't' as i8;
                *data.offset(1 as isize) = 'r' as i8;
                *data.offset(2 as isize) = 'u' as i8;
                *data.offset(3 as isize) = 'e' as i8;
                return data.offset(4 as isize);
            }
            5 => {
                *data.offset(0 as isize) = 'f' as i8;
                *data.offset(1 as isize) = 'a' as i8;
                *data.offset(2 as isize) = 'l' as i8;
                *data.offset(3 as isize) = 's' as i8;
                *data.offset(4 as isize) = 'e' as i8;
                return data.offset(5 as isize);
            }
            6 => {
                *data.offset(0 as isize) = 'n' as i8;
                *data.offset(1 as isize) = 'u' as i8;
                *data.offset(2 as isize) = 'l' as i8;
                *data.offset(3 as isize) = 'l' as i8;
                return data.offset(4 as isize);
            }
            _ => return 0 as *mut i8,
        };
    }
}

#[no_mangle]
pub extern "C" fn json_write_minified(
    mut value: *const json_value_s,
    mut out_size: *mut u64,
) -> *mut libc::c_void {
    unsafe {
        let mut size = 0;
        let mut data = 0 as *mut i8;
        let mut data_end = 0 as *mut i8;
        if value.is_null() {
            return 0 as *mut libc::c_void;
        }
        if json_write_minified_get_value_size(value, &mut size) != 0 {
            return 0 as *mut libc::c_void;
        }
        size = (size as u64).wrapping_add(1) as u64;
        data = malloc(size) as *mut i8;
        if data.is_null() {
            return 0 as *mut libc::c_void;
        }
        data_end = json_write_minified_value(value, data);
        if data_end.is_null() {
            free(data as *mut libc::c_void);
            return 0 as *mut libc::c_void;
        }
        *data_end = '\0' as i8;
        if !out_size.is_null() {
            *out_size = size;
        }
        return data as *mut libc::c_void;
    }
}

#[no_mangle]
pub extern "C" fn json_write_pretty_get_array_size(
    mut array: *const json_array_s,
    mut depth: u64,
    mut indent_size: u64,
    mut newline_size: u64,
    mut size: *mut u64,
) -> i32 {
    unsafe {
        let mut element = 0 as *mut json_array_element_s;
        *size = (*size as u64).wrapping_add(1) as u64;
        if 0 < (*array).length {
            *size = (*size as u64).wrapping_add(newline_size) as u64;
            *size = (*size as u64).wrapping_add(((*array).length).wrapping_sub(1)) as u64;
            element = (*array).start;
            while !element.is_null() {
                *size = (*size as u64).wrapping_add(depth.wrapping_add(1).wrapping_mul(indent_size))
                    as u64;
                if json_write_pretty_get_value_size(
                    (*element).value,
                    depth.wrapping_add(1),
                    indent_size,
                    newline_size,
                    size,
                ) != 0
                {
                    return 1;
                }
                *size = (*size as u64).wrapping_add(newline_size) as u64;
                element = (*element).next;
            }
            *size = (*size as u64).wrapping_add(depth.wrapping_mul(indent_size)) as u64;
        }
        *size = (*size as u64).wrapping_add(1) as u64;
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn json_write_pretty_get_object_size(
    mut object: *const json_object_s,
    mut depth: u64,
    mut indent_size: u64,
    mut newline_size: u64,
    mut size: *mut u64,
) -> i32 {
    unsafe {
        let mut element = 0 as *mut json_object_element_s;
        *size = (*size as u64).wrapping_add(1) as u64;
        if 0 < (*object).length {
            *size = (*size as u64).wrapping_add(newline_size) as u64;
            *size = (*size as u64).wrapping_add(((*object).length).wrapping_sub(1)) as u64;
            element = (*object).start;
            while !element.is_null() {
                *size = (*size as u64).wrapping_add(depth.wrapping_add(1).wrapping_mul(indent_size))
                    as u64;
                *size = (*size as u64).wrapping_add(newline_size) as u64;
                if json_write_get_string_size((*element).name, size) != 0 {
                    return 1;
                }
                *size = (*size as u64).wrapping_add(3) as u64;
                if json_write_pretty_get_value_size(
                    (*element).value,
                    depth.wrapping_add(1),
                    indent_size,
                    newline_size,
                    size,
                ) != 0
                {
                    return 1;
                }
                element = (*element).next;
            }
            *size = (*size as u64).wrapping_add(depth.wrapping_mul(indent_size)) as u64;
        }
        *size = (*size as u64).wrapping_add(1) as u64;
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn json_write_pretty_get_value_size(
    mut value: *const json_value_s,
    mut depth: u64,
    mut indent_size: u64,
    mut newline_size: u64,
    mut size: *mut u64,
) -> i32 {
    unsafe {
        match (*value).type_0 {
            1 => {
                return json_write_get_number_size((*value).payload as *mut json_number_s, size);
            }
            0 => {
                return json_write_get_string_size((*value).payload as *mut json_string_s, size);
            }
            3 => {
                return json_write_pretty_get_array_size(
                    (*value).payload as *mut json_array_s,
                    depth,
                    indent_size,
                    newline_size,
                    size,
                );
            }
            2 => {
                return json_write_pretty_get_object_size(
                    (*value).payload as *mut json_object_s,
                    depth,
                    indent_size,
                    newline_size,
                    size,
                );
            }
            4 => {
                *size = (*size as u64).wrapping_add(4) as u64;
                return 0;
            }
            5 => {
                *size = (*size as u64).wrapping_add(5) as u64;
                return 0;
            }
            6 => {
                *size = (*size as u64).wrapping_add(4) as u64;
                return 0;
            }
            _ => return 1,
        };
    }
}

#[no_mangle]
pub extern "C" fn json_write_pretty_array(
    mut array: *const json_array_s,
    mut depth: u64,
    mut indent: *const i8,
    mut newline: *const i8,
    mut data: *mut i8,
) -> *mut i8 {
    unsafe {
        let mut k: u64 = 0;
        let mut m: u64 = 0;
        let mut element = 0 as *mut json_array_element_s;
        let fresh187 = data;
        data = data.offset(1);
        *fresh187 = '[' as i8;
        if 0 < (*array).length {
            k = 0;
            while '\0' as i32 != *newline.offset(k as isize) as i32 {
                let fresh188 = data;
                data = data.offset(1);
                *fresh188 = *newline.offset(k as isize);
                k = k.wrapping_add(1);
            }
            element = (*array).start;
            while !element.is_null() {
                if element != (*array).start {
                    let fresh189 = data;
                    data = data.offset(1);
                    *fresh189 = ',' as i8;
                    k = 0;
                    while '\0' as i32 != *newline.offset(k as isize) as i32 {
                        let fresh190 = data;
                        data = data.offset(1);
                        *fresh190 = *newline.offset(k as isize);
                        k = k.wrapping_add(1);
                    }
                }
                k = 0;
                while k < depth.wrapping_add(1) {
                    m = 0;
                    while '\0' as i32 != *indent.offset(m as isize) as i32 {
                        let fresh191 = data;
                        data = data.offset(1);
                        *fresh191 = *indent.offset(m as isize);
                        m = m.wrapping_add(1);
                    }
                    k = k.wrapping_add(1);
                }
                data = json_write_pretty_value(
                    (*element).value,
                    depth.wrapping_add(1),
                    indent,
                    newline,
                    data,
                );
                if data.is_null() {
                    return 0 as *mut i8;
                }
                element = (*element).next;
            }
            k = 0;
            while '\0' as i32 != *newline.offset(k as isize) as i32 {
                let fresh192 = data;
                data = data.offset(1);
                *fresh192 = *newline.offset(k as isize);
                k = k.wrapping_add(1);
            }
            k = 0;
            while k < depth {
                m = 0;
                while '\0' as i32 != *indent.offset(m as isize) as i32 {
                    let fresh193 = data;
                    data = data.offset(1);
                    *fresh193 = *indent.offset(m as isize);
                    m = m.wrapping_add(1);
                }
                k = k.wrapping_add(1);
            }
        }
        let fresh194 = data;
        data = data.offset(1);
        *fresh194 = ']' as i8;
        return data;
    }
}

#[no_mangle]
pub extern "C" fn json_write_pretty_object(
    mut object: *const json_object_s,
    mut depth: u64,
    mut indent: *const i8,
    mut newline: *const i8,
    mut data: *mut i8,
) -> *mut i8 {
    unsafe {
        let mut k: u64 = 0;
        let mut m: u64 = 0;
        let mut element = 0 as *mut json_object_element_s;
        let fresh195 = data;
        data = data.offset(1);
        *fresh195 = '{' as i8;
        if 0 < (*object).length {
            k = 0;
            while '\0' as i32 != *newline.offset(k as isize) as i32 {
                let fresh196 = data;
                data = data.offset(1);
                *fresh196 = *newline.offset(k as isize);
                k = k.wrapping_add(1);
            }
            element = (*object).start;
            while !element.is_null() {
                if element != (*object).start {
                    let fresh197 = data;
                    data = data.offset(1);
                    *fresh197 = ',' as i8;
                    k = 0;
                    while '\0' as i32 != *newline.offset(k as isize) as i32 {
                        let fresh198 = data;
                        data = data.offset(1);
                        *fresh198 = *newline.offset(k as isize);
                        k = k.wrapping_add(1);
                    }
                }
                k = 0;
                while k < depth.wrapping_add(1) {
                    m = 0;
                    while '\0' as i32 != *indent.offset(m as isize) as i32 {
                        let fresh199 = data;
                        data = data.offset(1);
                        *fresh199 = *indent.offset(m as isize);
                        m = m.wrapping_add(1);
                    }
                    k = k.wrapping_add(1);
                }
                data = json_write_string((*element).name, data);
                if data.is_null() {
                    return 0 as *mut i8;
                }
                let fresh200 = data;
                data = data.offset(1);
                *fresh200 = ' ' as i8;
                let fresh201 = data;
                data = data.offset(1);
                *fresh201 = ':' as i8;
                let fresh202 = data;
                data = data.offset(1);
                *fresh202 = ' ' as i8;
                data = json_write_pretty_value(
                    (*element).value,
                    depth.wrapping_add(1),
                    indent,
                    newline,
                    data,
                );
                if data.is_null() {
                    return 0 as *mut i8;
                }
                element = (*element).next;
            }
            k = 0;
            while '\0' as i32 != *newline.offset(k as isize) as i32 {
                let fresh203 = data;
                data = data.offset(1);
                *fresh203 = *newline.offset(k as isize);
                k = k.wrapping_add(1);
            }
            k = 0;
            while k < depth {
                m = 0;
                while '\0' as i32 != *indent.offset(m as isize) as i32 {
                    let fresh204 = data;
                    data = data.offset(1);
                    *fresh204 = *indent.offset(m as isize);
                    m = m.wrapping_add(1);
                }
                k = k.wrapping_add(1);
            }
        }
        let fresh205 = data;
        data = data.offset(1);
        *fresh205 = '}' as i8;
        return data;
    }
}

#[no_mangle]
pub extern "C" fn json_write_pretty_value(
    mut value: *const json_value_s,
    mut depth: u64,
    mut indent: *const i8,
    mut newline: *const i8,
    mut data: *mut i8,
) -> *mut i8 {
    unsafe {
        match (*value).type_0 {
            1 => return json_write_number((*value).payload as *mut json_number_s, data),
            0 => return json_write_string((*value).payload as *mut json_string_s, data),
            3 => {
                return json_write_pretty_array(
                    (*value).payload as *mut json_array_s,
                    depth,
                    indent,
                    newline,
                    data,
                );
            }
            2 => {
                return json_write_pretty_object(
                    (*value).payload as *mut json_object_s,
                    depth,
                    indent,
                    newline,
                    data,
                );
            }
            4 => {
                *data.offset(0 as isize) = 't' as i8;
                *data.offset(1 as isize) = 'r' as i8;
                *data.offset(2 as isize) = 'u' as i8;
                *data.offset(3 as isize) = 'e' as i8;
                return data.offset(4 as isize);
            }
            5 => {
                *data.offset(0 as isize) = 'f' as i8;
                *data.offset(1 as isize) = 'a' as i8;
                *data.offset(2 as isize) = 'l' as i8;
                *data.offset(3 as isize) = 's' as i8;
                *data.offset(4 as isize) = 'e' as i8;
                return data.offset(5 as isize);
            }
            6 => {
                *data.offset(0 as isize) = 'n' as i8;
                *data.offset(1 as isize) = 'u' as i8;
                *data.offset(2 as isize) = 'l' as i8;
                *data.offset(3 as isize) = 'l' as i8;
                return data.offset(4 as isize);
            }
            _ => return 0 as *mut i8,
        };
    }
}

#[no_mangle]
pub extern "C" fn json_write_pretty(
    mut value: *const json_value_s,
    mut indent: *const i8,
    mut newline: *const i8,
    mut out_size: *mut u64,
) -> *mut libc::c_void {
    unsafe {
        let mut size = 0;
        let mut indent_size = 0;
        let mut newline_size = 0;
        let mut data = 0 as *mut i8;
        let mut data_end = 0 as *mut i8;
        if value.is_null() {
            return 0 as *mut libc::c_void;
        }
        if indent.is_null() {
            indent = b"  \0" as *const u8 as *const i8;
        }
        if newline.is_null() {
            newline = b"\n\0" as *const u8 as *const i8;
        }
        while '\0' as i32 != *indent.offset(indent_size as isize) as i32 {
            indent_size = indent_size.wrapping_add(1);
        }
        while '\0' as i32 != *newline.offset(newline_size as isize) as i32 {
            newline_size = newline_size.wrapping_add(1);
        }
        if json_write_pretty_get_value_size(value, 0, indent_size, newline_size, &mut size) != 0 {
            return 0 as *mut libc::c_void;
        }
        size = (size as u64).wrapping_add(1) as u64;
        data = malloc(size) as *mut i8;
        if data.is_null() {
            return 0 as *mut libc::c_void;
        }
        data_end = json_write_pretty_value(value, 0, indent, newline, data);
        if data_end.is_null() {
            free(data as *mut libc::c_void);
            return 0 as *mut libc::c_void;
        }
        *data_end = '\0' as i8;
        if !out_size.is_null() {
            *out_size = size;
        }
        return data as *mut libc::c_void;
    }
}
