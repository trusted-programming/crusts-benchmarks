use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
}
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
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csv_parser {
    pub pstate: i32,
    pub quoted: i32,
    pub spaces: u64,
    pub entry_buf: *mut u8,
    pub entry_pos: u64,
    pub entry_size: u64,
    pub status: i32,
    pub options: u8,
    pub quote_char: u8,
    pub delim_char: u8,
    pub is_space: Option<unsafe extern "C" fn(u8) -> i32>,
    pub is_term: Option<unsafe extern "C" fn(u8) -> i32>,
    pub blk_size: u64,
    pub malloc_func: Option<unsafe extern "C" fn(u64) -> *mut libc::c_void>,
    pub realloc_func: Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>,
    pub free_func: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
static mut csv_errors: [*const i8; 5] = [
    b"success\0" as *const u8 as *const i8,
    b"error parsing data while strict checking enabled\0" as *const u8 as *const i8,
    b"memory exhausted while increasing buffer size\0" as *const u8 as *const i8,
    b"data size too large\0" as *const u8 as *const i8,
    b"invalid status code\0" as *const u8 as *const i8,
];
#[no_mangle]
pub extern "C" fn csv_error(mut p: *const csv_parser) -> i32 {
    unsafe {
        if !p.is_null() && !(b"received null csv_parser\0" as *const u8 as *const i8).is_null() {
        } else {
            __assert_fail(
                b"p && \"received null csv_parser\"\0" as *const u8 as *const i8,
                b"libcsv.c\0" as *const u8 as *const i8,
                82,
                (*::std::mem::transmute::<&[u8; 41], &[i8; 41]>(
                    b"int csv_error(const struct csv_parser *)\0",
                ))
                .as_ptr(),
            );
        }
        return (*p).status;
    }
}

#[no_mangle]
pub extern "C" fn csv_strerror(mut status: i32) -> *const i8 {
    unsafe {
        if status >= 4 || status < 0 {
            return csv_errors[4 as usize];
        } else {
            return csv_errors[status as usize];
        };
    }
}

#[no_mangle]
pub extern "C" fn csv_get_opts(mut p: *const csv_parser) -> i32 {
    unsafe {
        if p.is_null() {
            return -1;
        }
        return (*p).options as i32;
    }
}

#[no_mangle]
pub extern "C" fn csv_set_opts(mut p: *mut csv_parser, mut options: u8) -> i32 {
    unsafe {
        if p.is_null() {
            return -1;
        };
        (*p).options = options;
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn csv_init(mut p: *mut csv_parser, mut options: u8) -> i32 {
    unsafe {
        if p.is_null() {
            return -1;
        }
        let ref mut fresh0 = (*p).entry_buf;
        *fresh0 = 0 as *mut u8;
        (*p).pstate = 0;
        (*p).quoted = 0;
        (*p).spaces = 0;
        (*p).entry_pos = 0;
        (*p).entry_size = 0;
        (*p).status = 0;
        (*p).options = options;
        (*p).quote_char = 0x22;
        (*p).delim_char = 0x2c;
        let ref mut fresh1 = (*p).is_space;
        *fresh1 = None;
        let ref mut fresh2 = (*p).is_term;
        *fresh2 = None;
        (*p).blk_size = 128;
        let ref mut fresh3 = (*p).malloc_func;
        *fresh3 = None;
        let ref mut fresh4 = (*p).realloc_func;
        *fresh4 =
            Some(realloc as unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void);
        let ref mut fresh5 = (*p).free_func;
        *fresh5 = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn csv_free(mut p: *mut csv_parser) {
    unsafe {
        if p.is_null() {
            return;
        }
        if !((*p).entry_buf).is_null() && ((*p).free_func).is_some() {
            ((*p).free_func).expect("non-null function pointer")(
                (*p).entry_buf as *mut libc::c_void,
            );
        }
        let ref mut fresh6 = (*p).entry_buf;
        *fresh6 = 0 as *mut u8;
        (*p).entry_size = 0;
    }
}

#[no_mangle]
pub extern "C" fn csv_fini(
    mut p: *mut csv_parser,
    mut cb1: Option<unsafe extern "C" fn(*mut libc::c_void, u64, *mut libc::c_void) -> ()>,
    mut cb2: Option<unsafe extern "C" fn(i32, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) -> i32 {
    unsafe {
        if p.is_null() {
            return -1;
        }
        let mut quoted = (*p).quoted;
        let mut pstate = (*p).pstate;
        let mut spaces = (*p).spaces;
        let mut entry_pos = (*p).entry_pos;
        if pstate == 2
            && (*p).quoted != 0
            && (*p).options as i32 & 1 != 0
            && (*p).options as i32 & 4 != 0
        {
            (*p).status = 1;
            return -1;
        }
        let mut current_block_26: u64;
        match pstate {
            3 => {
                let ref mut fresh7 = (*p).entry_pos;
                *fresh7 = (*fresh7 as u64).wrapping_sub(((*p).spaces).wrapping_add(1)) as u64;
                entry_pos = (*p).entry_pos;
                current_block_26 = 5901124769980541461;
            }
            1 | 2 => {
                current_block_26 = 5901124769980541461;
            }
            0 | _ => {
                current_block_26 = 4761528863920922185;
            }
        }
        match current_block_26 {
            5901124769980541461 => {
                if quoted == 0 {
                    entry_pos = (entry_pos as u64).wrapping_sub(spaces) as u64;
                }
                if (*p).options as i32 & 8 != 0 {
                    *((*p).entry_buf).offset(entry_pos as isize) = '\0' as u8;
                }
                if cb1.is_some() && (*p).options as i32 & 16 != 0 && quoted == 0 && entry_pos == 0 {
                    cb1.expect("non-null function pointer")(
                        0 as *mut libc::c_void,
                        entry_pos,
                        data,
                    );
                } else if cb1.is_some() {
                    cb1.expect("non-null function pointer")(
                        (*p).entry_buf as *mut libc::c_void,
                        entry_pos,
                        data,
                    );
                }
                pstate = 1;
                spaces = 0;
                quoted = spaces as i32;
                entry_pos = quoted as u64;
                if cb2.is_some() {
                    cb2.expect("non-null function pointer")(-1, data);
                }
                pstate = 0;
                spaces = 0;
                quoted = spaces as i32;
                entry_pos = quoted as u64;
            }
            _ => {}
        }
        let ref mut fresh8 = (*p).status;
        *fresh8 = 0;
        let ref mut fresh9 = (*p).entry_pos;
        *fresh9 = *fresh8 as u64;
        let ref mut fresh10 = (*p).quoted;
        *fresh10 = *fresh9 as i32;
        (*p).spaces = *fresh10 as u64;
        (*p).pstate = 0;
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn csv_set_delim(mut p: *mut csv_parser, mut c: u8) {
    unsafe {
        if !p.is_null() {
            (*p).delim_char = c;
        }
    }
}

#[no_mangle]
pub extern "C" fn csv_set_quote(mut p: *mut csv_parser, mut c: u8) {
    unsafe {
        if !p.is_null() {
            (*p).quote_char = c;
        }
    }
}

#[no_mangle]
pub extern "C" fn csv_get_delim(mut p: *const csv_parser) -> u8 {
    unsafe {
        if !p.is_null() && !(b"received null csv_parser\0" as *const u8 as *const i8).is_null() {
        } else {
            __assert_fail(
                b"p && \"received null csv_parser\"\0" as *const u8 as *const i8,
                b"libcsv.c\0" as *const u8 as *const i8,
                222,
                (*::std::mem::transmute::<&[u8; 55], &[i8; 55]>(
                    b"unsigned char csv_get_delim(const struct csv_parser *)\0",
                ))
                .as_ptr(),
            );
        }
        return (*p).delim_char;
    }
}

#[no_mangle]
pub extern "C" fn csv_get_quote(mut p: *const csv_parser) -> u8 {
    unsafe {
        if !p.is_null() && !(b"received null csv_parser\0" as *const u8 as *const i8).is_null() {
        } else {
            __assert_fail(
                b"p && \"received null csv_parser\"\0" as *const u8 as *const i8,
                b"libcsv.c\0" as *const u8 as *const i8,
                231,
                (*::std::mem::transmute::<&[u8; 55], &[i8; 55]>(
                    b"unsigned char csv_get_quote(const struct csv_parser *)\0",
                ))
                .as_ptr(),
            );
        }
        return (*p).quote_char;
    }
}

#[no_mangle]
pub extern "C" fn csv_set_space_func(
    mut p: *mut csv_parser,
    mut f: Option<unsafe extern "C" fn(u8) -> i32>,
) {
    unsafe {
        if !p.is_null() {
            let ref mut fresh11 = (*p).is_space;
            *fresh11 = f;
        }
    }
}

#[no_mangle]
pub extern "C" fn csv_set_term_func(
    mut p: *mut csv_parser,
    mut f: Option<unsafe extern "C" fn(u8) -> i32>,
) {
    unsafe {
        if !p.is_null() {
            let ref mut fresh12 = (*p).is_term;
            *fresh12 = f;
        }
    }
}

#[no_mangle]
pub extern "C" fn csv_set_realloc_func(
    mut p: *mut csv_parser,
    mut f: Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>,
) {
    unsafe {
        if !p.is_null() && f.is_some() {
            let ref mut fresh13 = (*p).realloc_func;
            *fresh13 = f;
        }
    }
}

#[no_mangle]
pub extern "C" fn csv_set_free_func(
    mut p: *mut csv_parser,
    mut f: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    unsafe {
        if !p.is_null() && f.is_some() {
            let ref mut fresh14 = (*p).free_func;
            *fresh14 = f;
        }
    }
}

#[no_mangle]
pub extern "C" fn csv_set_blk_size(mut p: *mut csv_parser, mut size: u64) {
    unsafe {
        if !p.is_null() {
            (*p).blk_size = size;
        }
    }
}

#[no_mangle]
pub extern "C" fn csv_get_buffer_size(mut p: *const csv_parser) -> u64 {
    unsafe {
        if !p.is_null() {
            return (*p).entry_size;
        }
        return 0;
    }
}

extern "C" fn csv_increase_buffer(mut p: *mut csv_parser) -> i32 {
    unsafe {
        if p.is_null() {
            return 0;
        }
        if ((*p).realloc_func).is_none() {
            return 0;
        }
        let mut to_add = (*p).blk_size;
        let mut vp = 0 as *mut libc::c_void;
        if (*p).entry_size >= 18446744073709551615u64.wrapping_sub(to_add) {
            to_add = 18446744073709551615u64.wrapping_sub((*p).entry_size);
        }
        if to_add == 0 {
            (*p).status = 3;
            return -1;
        }
        loop {
            vp = ((*p).realloc_func).expect("non-null function pointer")(
                (*p).entry_buf as *mut libc::c_void,
                ((*p).entry_size).wrapping_add(to_add),
            );
            if !vp.is_null() {
                break;
            }
            to_add = (to_add as u64).wrapping_div(2) as u64;
            if to_add == 0 {
                (*p).status = 2;
                return -1;
            }
        }
        let ref mut fresh15 = (*p).entry_buf;
        *fresh15 = vp as *mut u8;
        let ref mut fresh16 = (*p).entry_size;
        *fresh16 = (*fresh16 as u64).wrapping_add(to_add) as u64;
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn csv_parse(
    mut p: *mut csv_parser,
    mut s: *const libc::c_void,
    mut len: u64,
    mut cb1: Option<unsafe extern "C" fn(*mut libc::c_void, u64, *mut libc::c_void) -> ()>,
    mut cb2: Option<unsafe extern "C" fn(i32, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) -> u64 {
    unsafe {
        if !p.is_null() && !(b"received null csv_parser\0" as *const u8 as *const i8).is_null() {
        } else {
            __assert_fail (b"p && \"received null csv_parser\"\0" as * const u8 as * const i8, b"libcsv.c\0" as * const u8 as * const i8, 321, (* :: std :: mem :: transmute :: < & [u8; 125], & [i8; 125], > (
              b"size_t csv_parse(struct csv_parser *, const void *, size_t, void (*)(void *, size_t, void *), void (*)(int, void *), void *)\0",)).as_ptr (),);
        }
        if s.is_null() {
            return 0;
        }
        let mut us = s as *const u8;
        let mut c: u8 = 0;
        let mut pos = 0;
        let mut delim = (*p).delim_char;
        let mut quote = (*p).quote_char;
        let mut is_space: Option<unsafe extern "C" fn(u8) -> i32> = (*p).is_space;
        let mut is_term: Option<unsafe extern "C" fn(u8) -> i32> = (*p).is_term;
        let mut quoted = (*p).quoted;
        let mut pstate = (*p).pstate;
        let mut spaces = (*p).spaces;
        let mut entry_pos = (*p).entry_pos;
        if ((*p).entry_buf).is_null() && pos < len {
            if csv_increase_buffer(p) != 0 {
                (*p).quoted = quoted;
                (*p).pstate = pstate;
                (*p).spaces = spaces;
                (*p).entry_pos = entry_pos;
                return pos;
            }
        }
        while pos < len {
            if entry_pos
                == (if (*p).options as i32 & 8 != 0 {
                    ((*p).entry_size).wrapping_sub(1)
                } else {
                    (*p).entry_size
                })
            {
                if csv_increase_buffer(p) != 0 {
                    (*p).quoted = quoted;
                    (*p).pstate = pstate;
                    (*p).spaces = spaces;
                    (*p).entry_pos = entry_pos;
                    return pos;
                }
            }
            let fresh17 = pos;
            pos = pos.wrapping_add(1);
            c = *us.offset(fresh17 as isize);
            match pstate {
                0 | 1 => {
                    if (if is_space.is_some() {
                        is_space.expect("non-null function pointer")(c)
                    } else {
                        (c as i32 == 0x20 || c as i32 == 0x9) as i32
                    }) != 0
                        && c as i32 != delim as i32
                    {
                        continue;
                    }
                    if if is_term.is_some() {
                        is_term.expect("non-null function pointer")(c)
                    } else {
                        (c as i32 == 0xd || c as i32 == 0xa) as i32
                    } != 0
                    {
                        if pstate == 1 {
                            if quoted == 0 {
                                entry_pos = (entry_pos as u64).wrapping_sub(spaces) as u64;
                            }
                            if (*p).options as i32 & 8 != 0 {
                                *((*p).entry_buf).offset(entry_pos as isize) = '\0' as u8;
                            }
                            if cb1.is_some()
                                && (*p).options as i32 & 16 != 0
                                && quoted == 0
                                && entry_pos == 0
                            {
                                cb1.expect("non-null function pointer")(
                                    0 as *mut libc::c_void,
                                    entry_pos,
                                    data,
                                );
                            } else if cb1.is_some() {
                                cb1.expect("non-null function pointer")(
                                    (*p).entry_buf as *mut libc::c_void,
                                    entry_pos,
                                    data,
                                );
                            }
                            pstate = 1;
                            spaces = 0;
                            quoted = spaces as i32;
                            entry_pos = quoted as u64;
                            if cb2.is_some() {
                                cb2.expect("non-null function pointer")(c as i32, data);
                            }
                            pstate = 0;
                            spaces = 0;
                            quoted = spaces as i32;
                            entry_pos = quoted as u64;
                        } else if (*p).options as i32 & 2 != 0 {
                            if cb2.is_some() {
                                cb2.expect("non-null function pointer")(c as i32, data);
                            }
                            pstate = 0;
                            spaces = 0;
                            quoted = spaces as i32;
                            entry_pos = quoted as u64;
                        }
                    } else if c as i32 == delim as i32 {
                        if quoted == 0 {
                            entry_pos = (entry_pos as u64).wrapping_sub(spaces) as u64;
                        }
                        if (*p).options as i32 & 8 != 0 {
                            *((*p).entry_buf).offset(entry_pos as isize) = '\0' as u8;
                        }
                        if cb1.is_some()
                            && (*p).options as i32 & 16 != 0
                            && quoted == 0
                            && entry_pos == 0
                        {
                            cb1.expect("non-null function pointer")(
                                0 as *mut libc::c_void,
                                entry_pos,
                                data,
                            );
                        } else if cb1.is_some() {
                            cb1.expect("non-null function pointer")(
                                (*p).entry_buf as *mut libc::c_void,
                                entry_pos,
                                data,
                            );
                        }
                        pstate = 1;
                        spaces = 0;
                        quoted = spaces as i32;
                        entry_pos = quoted as u64;
                    } else if c as i32 == quote as i32 {
                        pstate = 2;
                        quoted = 1;
                    } else {
                        pstate = 2;
                        quoted = 0;
                        let fresh18 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh18 as isize) = c;
                    }
                }
                2 => {
                    if c as i32 == quote as i32 {
                        if quoted != 0 {
                            let fresh19 = entry_pos;
                            entry_pos = entry_pos.wrapping_add(1);
                            *((*p).entry_buf).offset(fresh19 as isize) = c;
                            pstate = 3;
                        } else {
                            if (*p).options as i32 & 1 != 0 {
                                (*p).status = 1;
                                (*p).quoted = quoted;
                                (*p).pstate = pstate;
                                (*p).spaces = spaces;
                                (*p).entry_pos = entry_pos;
                                return pos.wrapping_sub(1);
                            }
                            let fresh20 = entry_pos;
                            entry_pos = entry_pos.wrapping_add(1);
                            *((*p).entry_buf).offset(fresh20 as isize) = c;
                            spaces = 0;
                        }
                    } else if c as i32 == delim as i32 {
                        if quoted != 0 {
                            let fresh21 = entry_pos;
                            entry_pos = entry_pos.wrapping_add(1);
                            *((*p).entry_buf).offset(fresh21 as isize) = c;
                        } else {
                            if quoted == 0 {
                                entry_pos = (entry_pos as u64).wrapping_sub(spaces) as u64;
                            }
                            if (*p).options as i32 & 8 != 0 {
                                *((*p).entry_buf).offset(entry_pos as isize) = '\0' as u8;
                            }
                            if cb1.is_some()
                                && (*p).options as i32 & 16 != 0
                                && quoted == 0
                                && entry_pos == 0
                            {
                                cb1.expect("non-null function pointer")(
                                    0 as *mut libc::c_void,
                                    entry_pos,
                                    data,
                                );
                            } else if cb1.is_some() {
                                cb1.expect("non-null function pointer")(
                                    (*p).entry_buf as *mut libc::c_void,
                                    entry_pos,
                                    data,
                                );
                            }
                            pstate = 1;
                            spaces = 0;
                            quoted = spaces as i32;
                            entry_pos = quoted as u64;
                        }
                    } else if if is_term.is_some() {
                        is_term.expect("non-null function pointer")(c)
                    } else {
                        (c as i32 == 0xd || c as i32 == 0xa) as i32
                    } != 0
                    {
                        if quoted == 0 {
                            if quoted == 0 {
                                entry_pos = (entry_pos as u64).wrapping_sub(spaces) as u64;
                            }
                            if (*p).options as i32 & 8 != 0 {
                                *((*p).entry_buf).offset(entry_pos as isize) = '\0' as u8;
                            }
                            if cb1.is_some()
                                && (*p).options as i32 & 16 != 0
                                && quoted == 0
                                && entry_pos == 0
                            {
                                cb1.expect("non-null function pointer")(
                                    0 as *mut libc::c_void,
                                    entry_pos,
                                    data,
                                );
                            } else if cb1.is_some() {
                                cb1.expect("non-null function pointer")(
                                    (*p).entry_buf as *mut libc::c_void,
                                    entry_pos,
                                    data,
                                );
                            }
                            pstate = 1;
                            spaces = 0;
                            quoted = spaces as i32;
                            entry_pos = quoted as u64;
                            if cb2.is_some() {
                                cb2.expect("non-null function pointer")(c as i32, data);
                            }
                            pstate = 0;
                            spaces = 0;
                            quoted = spaces as i32;
                            entry_pos = quoted as u64;
                        } else {
                            let fresh22 = entry_pos;
                            entry_pos = entry_pos.wrapping_add(1);
                            *((*p).entry_buf).offset(fresh22 as isize) = c;
                        }
                    } else if quoted == 0
                        && (if is_space.is_some() {
                            is_space.expect("non-null function pointer")(c)
                        } else {
                            (c as i32 == 0x20 || c as i32 == 0x9) as i32
                        }) != 0
                    {
                        let fresh23 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh23 as isize) = c;
                        spaces = spaces.wrapping_add(1);
                    } else {
                        let fresh24 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh24 as isize) = c;
                        spaces = 0;
                    }
                }
                3 => {
                    if c as i32 == delim as i32 {
                        entry_pos = (entry_pos as u64).wrapping_sub(spaces.wrapping_add(1)) as u64;
                        if quoted == 0 {
                            entry_pos = (entry_pos as u64).wrapping_sub(spaces) as u64;
                        }
                        if (*p).options as i32 & 8 != 0 {
                            *((*p).entry_buf).offset(entry_pos as isize) = '\0' as u8;
                        }
                        if cb1.is_some()
                            && (*p).options as i32 & 16 != 0
                            && quoted == 0
                            && entry_pos == 0
                        {
                            cb1.expect("non-null function pointer")(
                                0 as *mut libc::c_void,
                                entry_pos,
                                data,
                            );
                        } else if cb1.is_some() {
                            cb1.expect("non-null function pointer")(
                                (*p).entry_buf as *mut libc::c_void,
                                entry_pos,
                                data,
                            );
                        }
                        pstate = 1;
                        spaces = 0;
                        quoted = spaces as i32;
                        entry_pos = quoted as u64;
                    } else if if is_term.is_some() {
                        is_term.expect("non-null function pointer")(c)
                    } else {
                        (c as i32 == 0xd || c as i32 == 0xa) as i32
                    } != 0
                    {
                        entry_pos = (entry_pos as u64).wrapping_sub(spaces.wrapping_add(1)) as u64;
                        if quoted == 0 {
                            entry_pos = (entry_pos as u64).wrapping_sub(spaces) as u64;
                        }
                        if (*p).options as i32 & 8 != 0 {
                            *((*p).entry_buf).offset(entry_pos as isize) = '\0' as u8;
                        }
                        if cb1.is_some()
                            && (*p).options as i32 & 16 != 0
                            && quoted == 0
                            && entry_pos == 0
                        {
                            cb1.expect("non-null function pointer")(
                                0 as *mut libc::c_void,
                                entry_pos,
                                data,
                            );
                        } else if cb1.is_some() {
                            cb1.expect("non-null function pointer")(
                                (*p).entry_buf as *mut libc::c_void,
                                entry_pos,
                                data,
                            );
                        }
                        pstate = 1;
                        spaces = 0;
                        quoted = spaces as i32;
                        entry_pos = quoted as u64;
                        if cb2.is_some() {
                            cb2.expect("non-null function pointer")(c as i32, data);
                        }
                        pstate = 0;
                        spaces = 0;
                        quoted = spaces as i32;
                        entry_pos = quoted as u64;
                    } else if if is_space.is_some() {
                        is_space.expect("non-null function pointer")(c)
                    } else {
                        (c as i32 == 0x20 || c as i32 == 0x9) as i32
                    } != 0
                    {
                        let fresh25 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh25 as isize) = c;
                        spaces = spaces.wrapping_add(1);
                    } else if c as i32 == quote as i32 {
                        if spaces != 0 {
                            if (*p).options as i32 & 1 != 0 {
                                (*p).status = 1;
                                (*p).quoted = quoted;
                                (*p).pstate = pstate;
                                (*p).spaces = spaces;
                                (*p).entry_pos = entry_pos;
                                return pos.wrapping_sub(1);
                            }
                            spaces = 0;
                            let fresh26 = entry_pos;
                            entry_pos = entry_pos.wrapping_add(1);
                            *((*p).entry_buf).offset(fresh26 as isize) = c;
                        } else {
                            pstate = 2;
                        }
                    } else {
                        if (*p).options as i32 & 1 != 0 {
                            (*p).status = 1;
                            (*p).quoted = quoted;
                            (*p).pstate = pstate;
                            (*p).spaces = spaces;
                            (*p).entry_pos = entry_pos;
                            return pos.wrapping_sub(1);
                        }
                        pstate = 2;
                        spaces = 0;
                        let fresh27 = entry_pos;
                        entry_pos = entry_pos.wrapping_add(1);
                        *((*p).entry_buf).offset(fresh27 as isize) = c;
                    }
                }
                _ => {}
            }
        }
        (*p).quoted = quoted;
        (*p).pstate = pstate;
        (*p).spaces = spaces;
        (*p).entry_pos = entry_pos;
        return pos;
    }
}

#[no_mangle]
pub extern "C" fn csv_write(
    mut dest: *mut libc::c_void,
    mut dest_size: u64,
    mut src: *const libc::c_void,
    mut src_size: u64,
) -> u64 {
    unsafe {
        return csv_write2(dest, dest_size, src, src_size, 0x22);
    }
}

#[no_mangle]
pub extern "C" fn csv_fwrite(
    mut fp: *mut FILE,
    mut src: *const libc::c_void,
    mut src_size: u64,
) -> i32 {
    unsafe {
        return csv_fwrite2(fp, src, src_size, 0x22);
    }
}

#[no_mangle]
pub extern "C" fn csv_write2(
    mut dest: *mut libc::c_void,
    mut dest_size: u64,
    mut src: *const libc::c_void,
    mut src_size: u64,
    mut quote: u8,
) -> u64 {
    unsafe {
        let mut cdest = dest as *mut u8;
        let mut csrc = src as *const u8;
        let mut chars = 0;
        if src.is_null() {
            return 0;
        }
        if dest.is_null() {
            dest_size = 0;
        }
        if dest_size > 0 {
            let fresh28 = cdest;
            cdest = cdest.offset(1);
            *fresh28 = quote;
        }
        chars = chars.wrapping_add(1);
        while src_size != 0 {
            if *csrc as i32 == quote as i32 {
                if dest_size > chars {
                    let fresh29 = cdest;
                    cdest = cdest.offset(1);
                    *fresh29 = quote;
                }
                if chars < 18446744073709551615 {
                    chars = chars.wrapping_add(1);
                }
            }
            if dest_size > chars {
                let fresh30 = cdest;
                cdest = cdest.offset(1);
                *fresh30 = *csrc;
            }
            if chars < 18446744073709551615 {
                chars = chars.wrapping_add(1);
            }
            src_size = src_size.wrapping_sub(1);
            csrc = csrc.offset(1);
        }
        if dest_size > chars {
            *cdest = quote;
        }
        if chars < 18446744073709551615 {
            chars = chars.wrapping_add(1);
        }
        return chars;
    }
}

#[no_mangle]
pub extern "C" fn csv_fwrite2(
    mut fp: *mut FILE,
    mut src: *const libc::c_void,
    mut src_size: u64,
    mut quote: u8,
) -> i32 {
    unsafe {
        let mut csrc = src as *const u8;
        if fp.is_null() || src.is_null() {
            return 0;
        }
        if fputc(quote as i32, fp) == -1 {
            return -1;
        }
        while src_size != 0 {
            if *csrc as i32 == quote as i32 {
                if fputc(quote as i32, fp) == -1 {
                    return -1;
                }
            }
            if fputc(*csrc as i32, fp) == -1 {
                return -1;
            }
            src_size = src_size.wrapping_sub(1);
            csrc = csrc.offset(1);
        }
        if fputc(quote as i32, fp) == -1 {
            return -1;
        }
        return 0;
    }
}
