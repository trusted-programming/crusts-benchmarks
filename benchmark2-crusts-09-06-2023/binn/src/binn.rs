use libc;
extern "C" {
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn atof(__nptr: *const i8) -> f64;
    fn atoi(__nptr: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strdup(_: *const i8) -> *mut i8;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn strncasecmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
}
pub type binn_mem_free = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binn_struct {
    pub header: i32,
    pub allocated: i32,
    pub writable: i32,
    pub dirty: i32,
    pub pbuf: *mut libc::c_void,
    pub pre_allocated: i32,
    pub alloc_size: i32,
    pub used_size: i32,
    pub type_0: i32,
    pub ptr: *mut libc::c_void,
    pub size: i32,
    pub count: i32,
    pub freefn: binn_mem_free,
    pub c2rust_unnamed: C2RustUnnamed,
    pub disable_int_compression: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub vint8: i8,
    pub vint16: i16,
    pub vint32: i32,
    pub vint64: i64,
    pub vuint8: u8,
    pub vuint16: u16,
    pub vuint32: u32,
    pub vuint64: u64,
    pub vchar: i8,
    pub vuchar: u8,
    pub vshort: i16,
    pub vushort: u16,
    pub vint: i32,
    pub vuint: u32,
    pub vfloat: libc::c_float,
    pub vdouble: f64,
    pub vbool: i32,
}
pub type binn = binn_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binn_iter_struct {
    pub pnext: *mut u8,
    pub plimit: *mut u8,
    pub type_0: i32,
    pub count: i32,
    pub current: i32,
}
pub type binn_iter = binn_iter_struct;
#[inline(always)]
extern "C" fn binn_list_add_value(mut list: *mut binn, mut value: *mut binn) -> i32 {
    unsafe {
        return binn_list_add(
            list,
            (*value).type_0,
            binn_ptr(value as *mut libc::c_void),
            binn_size(value as *mut libc::c_void),
        );
    }
}

#[inline(always)]
extern "C" fn binn_map_set_value(mut map: *mut binn, mut id: i32, mut value: *mut binn) -> i32 {
    unsafe {
        return binn_map_set(
            map,
            id,
            (*value).type_0,
            binn_ptr(value as *mut libc::c_void),
            binn_size(value as *mut libc::c_void),
        );
    }
}

#[inline(always)]
extern "C" fn binn_object_set_value(
    mut obj: *mut binn,
    mut key: *const i8,
    mut value: *mut binn,
) -> i32 {
    unsafe {
        return binn_object_set(
            obj,
            key,
            (*value).type_0,
            binn_ptr(value as *mut libc::c_void),
            binn_size(value as *mut libc::c_void),
        );
    }
}

#[no_mangle]
pub static mut malloc_fn: Option<unsafe extern "C" fn(u64) -> *mut libc::c_void> = None;
#[no_mangle]
pub static mut realloc_fn: Option<
    unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void,
> = None;
#[no_mangle]
pub static mut free_fn: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()> = None;
extern "C" fn copy_be16(mut pdest: *mut u16, mut psource: *mut u16) {
    unsafe {
        let mut source = psource as *mut u8;
        let mut dest = pdest as *mut u8;
        *dest.offset(0 as isize) = *source.offset(1 as isize);
        *dest.offset(1 as isize) = *source.offset(0 as isize);
    }
}

extern "C" fn copy_be32(mut pdest: *mut u32, mut psource: *mut u32) {
    unsafe {
        let mut source = psource as *mut u8;
        let mut dest = pdest as *mut u8;
        *dest.offset(0 as isize) = *source.offset(3 as isize);
        *dest.offset(1 as isize) = *source.offset(2 as isize);
        *dest.offset(2 as isize) = *source.offset(1 as isize);
        *dest.offset(3 as isize) = *source.offset(0 as isize);
    }
}

extern "C" fn copy_be64(mut pdest: *mut u64, mut psource: *mut u64) {
    unsafe {
        let mut source = psource as *mut u8;
        let mut dest = pdest as *mut u8;
        let mut i: i32 = 0;
        i = 0;
        while i < 8 {
            *dest.offset(i as isize) = *source.offset((7 - i) as isize);
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn binn_version() -> *mut i8 {
    return b"3.0.0\0" as *const u8 as *const i8 as *mut i8;
}

#[no_mangle]
pub extern "C" fn binn_set_alloc_functions(
    mut new_malloc: Option<unsafe extern "C" fn(u64) -> *mut libc::c_void>,
    mut new_realloc: Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>,
    mut new_free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    unsafe {
        malloc_fn = new_malloc;
        realloc_fn = new_realloc;
        free_fn = new_free;
    }
}

extern "C" fn check_alloc_functions() {
    unsafe {
        if malloc_fn.is_none() {
            malloc_fn = Some(malloc as unsafe extern "C" fn(u64) -> *mut libc::c_void);
        }
        if realloc_fn.is_none() {
            realloc_fn =
                Some(realloc as unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void);
        }
        if free_fn.is_none() {
            free_fn = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
        }
    }
}

extern "C" fn binn_malloc(mut size: i32) -> *mut libc::c_void {
    check_alloc_functions();
    unsafe {
        return malloc_fn.expect("non-null function pointer")(size as u64);
    }
}

extern "C" fn binn_memdup(mut src: *mut libc::c_void, mut size: i32) -> *mut libc::c_void {
    unsafe {
        let mut dest = 0 as *mut libc::c_void;
        if src.is_null() || size <= 0 {
            return 0 as *mut libc::c_void;
        }
        dest = binn_malloc(size);
        if dest.is_null() {
            return 0 as *mut libc::c_void;
        }
        memcpy(dest, src, size as u64);
        return dest;
    }
}

extern "C" fn strlen2(mut str: *mut i8) -> u64 {
    unsafe {
        if str.is_null() {
            return 0;
        }
        return strlen(str);
    }
}

#[no_mangle]
pub extern "C" fn binn_create_type(mut storage_type: i32, mut data_type_index: i32) -> i32 {
    if data_type_index < 0 {
        return -1;
    }
    if storage_type < 0 || storage_type > 0xe0 {
        return -1;
    }
    if data_type_index < 16 {
        return storage_type | data_type_index;
    } else if data_type_index < 4096 {
        storage_type |= 0x10;
        storage_type <<= 8;
        data_type_index >>= 4;
        return storage_type | data_type_index;
    } else {
        return -1;
    };
}

#[no_mangle]
pub extern "C" fn binn_get_type_info(
    mut long_type: i32,
    mut pstorage_type: *mut i32,
    mut pextra_type: *mut i32,
) -> i32 {
    unsafe {
        let mut storage_type: i32 = 0;
        let mut extra_type: i32 = 0;
        let mut retval = 1;
        let mut current_block_11: u64;
        loop {
            if long_type < 0 {
                current_block_11 = 9393356739754453642;
                break;
            }
            if long_type <= 0xff {
                storage_type = long_type & 0xe0;
                extra_type = long_type & 0xf;
                current_block_11 = 17833034027772472439;
                break;
            } else if long_type <= 0xffff {
                storage_type = long_type & 0xe000;
                storage_type >>= 8;
                extra_type = long_type & 0xfff;
                extra_type >>= 4;
                current_block_11 = 17833034027772472439;
                break;
            } else {
                if !(long_type & 0x80000 != 0) {
                    current_block_11 = 9393356739754453642;
                    break;
                }
                long_type &= 0xffff;
            }
        }
        match current_block_11 {
            9393356739754453642 => {
                storage_type = -1;
                extra_type = -1;
                retval = 0;
            }
            _ => {}
        }
        if !pstorage_type.is_null() {
            *pstorage_type = storage_type;
        }
        if !pextra_type.is_null() {
            *pextra_type = extra_type;
        }
        return retval;
    }
}

#[no_mangle]
pub extern "C" fn binn_create(
    mut item: *mut binn,
    mut type_0: i32,
    mut size: i32,
    mut pointer: *mut libc::c_void,
) -> i32 {
    unsafe {
        let mut current_block: u64;
        let mut retval = 0;
        match type_0 {
            224 | 225 | 226 => {
                if !(item.is_null() || size < 0) {
                    if size < 3 {
                        if !pointer.is_null() {
                            current_block = 13652966409768237295;
                        } else {
                            size = 0;
                            current_block = 14523784380283086299;
                        }
                    } else {
                        current_block = 14523784380283086299;
                    }
                    match current_block {
                        13652966409768237295 => {}
                        _ => {
                            memset(
                                item as *mut libc::c_void,
                                0,
                                ::std::mem::size_of::<binn>() as u64,
                            );
                            if !pointer.is_null() {
                                (*item).pre_allocated = 1;
                                let ref mut fresh0 = (*item).pbuf;
                                *fresh0 = pointer;
                                (*item).alloc_size = size;
                            } else {
                                (*item).pre_allocated = 0;
                                if size == 0 {
                                    size = 256;
                                }
                                pointer = binn_malloc(size);
                                if pointer.is_null() {
                                    return 0;
                                }
                                let ref mut fresh1 = (*item).pbuf;
                                *fresh1 = pointer;
                                (*item).alloc_size = size;
                            };
                            (*item).header = 0x1f22b11f;
                            (*item).writable = 1;
                            (*item).used_size = 9;
                            (*item).type_0 = type_0;
                            (*item).dirty = 1;
                            retval = 1;
                        }
                    }
                }
            }
            _ => {}
        }
        return retval;
    }
}

#[no_mangle]
pub extern "C" fn binn_new(
    mut type_0: i32,
    mut size: i32,
    mut pointer: *mut libc::c_void,
) -> *mut binn {
    unsafe {
        let mut item = 0 as *mut binn;
        item = binn_malloc(::std::mem::size_of::<binn>() as i32) as *mut binn;
        if binn_create(item, type_0, size, pointer) == 0 {
            free_fn.expect("non-null function pointer")(item as *mut libc::c_void);
            return 0 as *mut binn;
        };
        (*item).allocated = 1;
        return item;
    }
}

#[no_mangle]
pub extern "C" fn binn_create_list(mut list: *mut binn) -> i32 {
    unsafe {
        return binn_create(list, 0xe0, 0, 0 as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn binn_create_map(mut map: *mut binn) -> i32 {
    unsafe {
        return binn_create(map, 0xe1, 0, 0 as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn binn_create_object(mut object: *mut binn) -> i32 {
    unsafe {
        return binn_create(object, 0xe2, 0, 0 as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn binn_list() -> *mut binn {
    return binn_new(0xe0, 0, 0 as *mut libc::c_void);
}

#[no_mangle]
pub extern "C" fn binn_map() -> *mut binn {
    return binn_new(0xe1, 0, 0 as *mut libc::c_void);
}

#[no_mangle]
pub extern "C" fn binn_object() -> *mut binn {
    return binn_new(0xe2, 0, 0 as *mut libc::c_void);
}

#[no_mangle]
pub extern "C" fn binn_copy(mut old: *mut libc::c_void) -> *mut binn {
    unsafe {
        let mut type_0: i32 = 0;
        let mut count: i32 = 0;
        let mut size: i32 = 0;
        let mut header_size: i32 = 0;
        let mut old_ptr = binn_ptr(old) as *mut u8;
        let mut item = 0 as *mut binn;
        size = 0;
        if IsValidBinnHeader(
            old_ptr as *mut libc::c_void,
            &mut type_0,
            &mut count,
            &mut size,
            &mut header_size,
        ) == 0
        {
            return 0 as *mut binn;
        }
        item = binn_new(type_0, size - header_size + 9, 0 as *mut libc::c_void);
        if !item.is_null() {
            let mut dest = 0 as *mut u8;
            dest = ((*item).pbuf as *mut u8).offset(9 as isize);
            memcpy(
                dest as *mut libc::c_void,
                old_ptr.offset(header_size as isize) as *const libc::c_void,
                (size - header_size) as u64,
            );
            (*item).used_size = 9 + size - header_size;
            (*item).count = count;
        }
        return item;
    }
}

#[no_mangle]
pub extern "C" fn binn_load(mut data: *mut libc::c_void, mut value: *mut binn) -> i32 {
    unsafe {
        if data.is_null() || value.is_null() {
            return 0;
        }
        memset(
            value as *mut libc::c_void,
            0,
            ::std::mem::size_of::<binn>() as u64,
        );
        (*value).header = 0x1f22b11f;
        if binn_is_valid(
            data,
            &mut (*value).type_0,
            &mut (*value).count,
            &mut (*value).size,
        ) == 0
        {
            return 0;
        }
        let ref mut fresh2 = (*value).ptr;
        *fresh2 = data;
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn binn_open(mut data: *mut libc::c_void) -> *mut binn {
    unsafe {
        let mut item = 0 as *mut binn;
        item = binn_malloc(::std::mem::size_of::<binn>() as i32) as *mut binn;
        if binn_load(data, item) == 0 {
            free_fn.expect("non-null function pointer")(item as *mut libc::c_void);
            return 0 as *mut binn;
        };
        (*item).allocated = 1;
        return item;
    }
}

extern "C" fn binn_get_ptr_type(mut ptr: *mut libc::c_void) -> i32 {
    unsafe {
        if ptr.is_null() {
            return 0;
        }
        match *(ptr as *mut u32) {
            522367263 => return 1,
            _ => return 2,
        };
    }
}

#[no_mangle]
pub extern "C" fn binn_is_struct(mut ptr: *mut libc::c_void) -> i32 {
    unsafe {
        if ptr.is_null() {
            return 0;
        }
        if *(ptr as *mut u32) == 0x1f22b11f {
            return 1;
        } else {
            return 0;
        };
    }
}

extern "C" fn CalcAllocation(mut needed_size: i32, mut alloc_size: i32) -> i32 {
    let mut calc_size: i32 = 0;
    calc_size = alloc_size;
    while calc_size < needed_size {
        calc_size <<= 1;
    }
    return calc_size;
}

extern "C" fn CheckAllocation(mut item: *mut binn, mut add_size: i32) -> i32 {
    unsafe {
        let mut alloc_size: i32 = 0;
        let mut ptr = 0 as *mut libc::c_void;
        if (*item).used_size + add_size > (*item).alloc_size {
            if (*item).pre_allocated != 0 {
                return 0;
            }
            alloc_size = CalcAllocation((*item).used_size + add_size, (*item).alloc_size);
            ptr = realloc_fn.expect("non-null function pointer")((*item).pbuf, alloc_size as u64);
            if ptr.is_null() {
                return 0;
            }
            let ref mut fresh3 = (*item).pbuf;
            *fresh3 = ptr;
            (*item).alloc_size = alloc_size;
        }
        return 1;
    }
}

extern "C" fn AdvanceDataPos(mut p: *mut u8, mut plimit: *mut u8) -> *mut u8 {
    unsafe {
        let mut byte: u8 = 0;
        let mut storage_type: i32 = 0;
        let mut DataSize: i32 = 0;
        if p > plimit {
            return 0 as *mut u8;
        }
        byte = *p;
        p = p.offset(1);
        storage_type = byte as i32 & 0xe0;
        if byte as i32 & 0x10 != 0 {
            p = p.offset(1);
        }
        match storage_type {
            0 => {}
            32 => {
                p = p.offset(1);
            }
            64 => {
                p = p.offset(2 as isize);
            }
            96 => {
                p = p.offset(4 as isize);
            }
            128 => {
                p = p.offset(8 as isize);
            }
            192 | 160 => {
                if p > plimit {
                    return 0 as *mut u8;
                }
                DataSize = *p as i32;
                if DataSize & 0x80 != 0 {
                    if p.offset(::std::mem::size_of::<i32>() as u64 as isize)
                        .offset(-(1 as isize))
                        > plimit
                    {
                        return 0 as *mut u8;
                    }
                    copy_be32(&mut DataSize as *mut i32 as *mut u32, p as *mut u32);
                    DataSize &= 0x7fffffff;
                    p = p.offset(4 as isize);
                } else {
                    p = p.offset(1);
                }
                p = p.offset(DataSize as isize);
                if storage_type == 0xa0 {
                    p = p.offset(1);
                }
            }
            224 => {
                if p > plimit {
                    return 0 as *mut u8;
                }
                DataSize = *p as i32;
                if DataSize & 0x80 != 0 {
                    if p.offset(::std::mem::size_of::<i32>() as u64 as isize)
                        .offset(-(1 as isize))
                        > plimit
                    {
                        return 0 as *mut u8;
                    }
                    copy_be32(&mut DataSize as *mut i32 as *mut u32, p as *mut u32);
                    DataSize &= 0x7fffffff;
                }
                DataSize -= 1;
                p = p.offset(DataSize as isize);
            }
            _ => return 0 as *mut u8,
        }
        if p > plimit {
            return 0 as *mut u8;
        }
        return p;
    }
}

extern "C" fn read_map_id(mut pp: *mut *mut u8, mut plimit: *mut u8) -> i32 {
    unsafe {
        let mut p = 0 as *mut u8;
        let mut c: u8 = 0;
        let mut sign: u8 = 0;
        let mut type_0: u8 = 0;
        let mut id: i32 = 0;
        let mut extra_bytes: i32 = 0;
        p = *pp;
        let fresh4 = p;
        p = p.offset(1);
        c = *fresh4;
        if c as i32 & 0x80 != 0 {
            extra_bytes = ((c as i32 & 0x60) >> 5) + 1;
            if p.offset(extra_bytes as isize) > plimit {
                *pp = p.offset(extra_bytes as isize);
                return 0;
            }
        }
        type_0 = (c as i32 & 0xe0i32) as u8;
        sign = (c as i32 & 0x10i32) as u8;
        if c as i32 & 0x80 == 0 {
            sign = (c as i32 & 0x40i32) as u8;
            id = c as i32 & 0x3f;
        } else if type_0 as i32 == 0x80 {
            id = c as i32 & 0xf;
            let fresh5 = p;
            p = p.offset(1);
            id = id << 8 | *fresh5 as i32;
        } else if type_0 as i32 == 0xa0 {
            id = c as i32 & 0xf;
            let fresh6 = p;
            p = p.offset(1);
            id = id << 8 | *fresh6 as i32;
            let fresh7 = p;
            p = p.offset(1);
            id = id << 8 | *fresh7 as i32;
        } else if type_0 as i32 == 0xc0 {
            id = c as i32 & 0xf;
            let fresh8 = p;
            p = p.offset(1);
            id = id << 8 | *fresh8 as i32;
            let fresh9 = p;
            p = p.offset(1);
            id = id << 8 | *fresh9 as i32;
            let fresh10 = p;
            p = p.offset(1);
            id = id << 8 | *fresh10 as i32;
        } else if type_0 as i32 == 0xe0 {
            copy_be32(&mut id as *mut i32 as *mut u32, p as *mut u32);
            p = p.offset(4 as isize);
        } else {
            *pp = plimit.offset(2 as isize);
            return 0;
        }
        if sign != 0 {
            id = -id;
        }
        *pp = p;
        return id;
    }
}

extern "C" fn SearchForID(
    mut p: *mut u8,
    mut header_size: i32,
    mut size: i32,
    mut numitems: i32,
    mut id: i32,
) -> *mut u8 {
    unsafe {
        let mut plimit = 0 as *mut u8;
        let mut base = 0 as *mut u8;
        let mut i: i32 = 0;
        let mut int32: i32 = 0;
        base = p;
        plimit = p.offset(size as isize).offset(-(1 as isize));
        p = p.offset(header_size as isize);
        i = 0;
        while i < numitems {
            int32 = read_map_id(&mut p, plimit);
            if p > plimit {
                break;
            }
            if int32 == id {
                return p;
            }
            p = AdvanceDataPos(p, plimit);
            if p.is_null() || p < base {
                break;
            }
            i += 1;
        }
        return 0 as *mut u8;
    }
}

extern "C" fn SearchForKey(
    mut p: *mut u8,
    mut header_size: i32,
    mut size: i32,
    mut numitems: i32,
    mut key: *const i8,
) -> *mut u8 {
    unsafe {
        let mut len: u8 = 0;
        let mut plimit = 0 as *mut u8;
        let mut base = 0 as *mut u8;
        let mut i: i32 = 0;
        let mut keylen: i32 = 0;
        base = p;
        plimit = p.offset(size as isize).offset(-(1 as isize));
        p = p.offset(header_size as isize);
        keylen = strlen(key) as i32;
        i = 0;
        while i < numitems {
            len = *p;
            p = p.offset(1);
            if p > plimit {
                break;
            }
            if len as i32 > 0 {
                if strncasecmp(p as *mut i8, key, len as u64) == 0 {
                    if keylen == len as i32 {
                        p = p.offset(len as i32 as isize);
                        return p;
                    }
                }
                p = p.offset(len as i32 as isize);
                if p > plimit {
                    break;
                }
            } else if len as i32 == keylen {
                return p;
            }
            p = AdvanceDataPos(p, plimit);
            if p.is_null() || p < base {
                break;
            }
            i += 1;
        }
        return 0 as *mut u8;
    }
}

extern "C" fn binn_list_add_raw(
    mut item: *mut binn,
    mut type_0: i32,
    mut pvalue: *mut libc::c_void,
    mut size: i32,
) -> i32 {
    unsafe {
        if item.is_null() || (*item).type_0 != 0xe0 || (*item).writable == 0 {
            return 0;
        }
        if AddValue(item, type_0, pvalue, size) == 0 {
            return 0;
        }
        let ref mut fresh11 = (*item).count;
        *fresh11 += 1;
        return 1;
    }
}

extern "C" fn binn_object_set_raw(
    mut item: *mut binn,
    mut key: *const i8,
    mut type_0: i32,
    mut pvalue: *mut libc::c_void,
    mut size: i32,
) -> i32 {
    unsafe {
        let mut p = 0 as *mut u8;
        let mut len: u8 = 0;
        let mut int32: i32 = 0;
        if item.is_null() || (*item).type_0 != 0xe2 || (*item).writable == 0 {
            return 0;
        }
        if key.is_null() {
            return 0;
        }
        int32 = strlen(key) as i32;
        if int32 > 255 {
            return 0;
        }
        p = SearchForKey(
            (*item).pbuf as *mut u8,
            9,
            (*item).used_size,
            (*item).count,
            key,
        );
        if !p.is_null() {
            return 0;
        }
        if CheckAllocation(item, 1 + int32) == 0 {
            return 0;
        }
        p = ((*item).pbuf as *mut u8).offset((*item).used_size as isize);
        len = int32 as u8;
        *p = len;
        p = p.offset(1);
        memcpy(
            p as *mut libc::c_void,
            key as *const libc::c_void,
            int32 as u64,
        );
        int32 += 1;
        (*item).used_size += int32;
        if AddValue(item, type_0, pvalue, size) == 0 {
            (*item).used_size -= int32;
            return 0;
        }
        let ref mut fresh12 = (*item).count;
        *fresh12 += 1;
        return 1;
    }
}

extern "C" fn binn_map_set_raw(
    mut item: *mut binn,
    mut id: i32,
    mut type_0: i32,
    mut pvalue: *mut libc::c_void,
    mut size: i32,
) -> i32 {
    unsafe {
        let mut base = 0 as *mut u8;
        let mut p = 0 as *mut u8;
        let mut sign: u8 = 0;
        let mut id_size: i32 = 0;
        if item.is_null() || (*item).type_0 != 0xe1 || (*item).writable == 0 {
            return 0;
        }
        p = SearchForID(
            (*item).pbuf as *mut u8,
            9,
            (*item).used_size,
            (*item).count,
            id,
        );
        if !p.is_null() {
            return 0;
        }
        if CheckAllocation(item, 5) == 0 {
            return 0;
        }
        base = ((*item).pbuf as *mut u8).offset((*item).used_size as isize);
        p = base;
        sign = (id < 0) as u8;
        if sign != 0 {
            id = -id;
        }
        if id <= 0x3f {
            let fresh13 = p;
            p = p.offset(1);
            *fresh13 = ((sign as i32) << 6 | id) as u8;
        } else if id <= 0xfff {
            let fresh14 = p;
            p = p.offset(1);
            *fresh14 = (0x80 | (sign as i32) << 4 | (id & 0xf00i32) >> 8) as u8;
            let fresh15 = p;
            p = p.offset(1);
            *fresh15 = (id & 0xffi32) as u8;
        } else if id <= 0xfffff {
            let fresh16 = p;
            p = p.offset(1);
            *fresh16 = (0xa0 | (sign as i32) << 4 | (id & 0xf0000i32) >> 16) as u8;
            let fresh17 = p;
            p = p.offset(1);
            *fresh17 = ((id & 0xff00i32) >> 8) as u8;
            let fresh18 = p;
            p = p.offset(1);
            *fresh18 = (id & 0xffi32) as u8;
        } else if id <= 0xfffffff {
            let fresh19 = p;
            p = p.offset(1);
            *fresh19 = (0xc0 | (sign as i32) << 4 | (id & 0xf000000i32) >> 24) as u8;
            let fresh20 = p;
            p = p.offset(1);
            *fresh20 = ((id & 0xff0000i32) >> 16) as u8;
            let fresh21 = p;
            p = p.offset(1);
            *fresh21 = ((id & 0xff00i32) >> 8) as u8;
            let fresh22 = p;
            p = p.offset(1);
            *fresh22 = (id & 0xffi32) as u8;
        } else {
            let fresh23 = p;
            p = p.offset(1);
            *fresh23 = 0xe0;
            if sign != 0 {
                id = -id;
            }
            copy_be32(p as *mut u32, &mut id as *mut i32 as *mut u32);
            p = p.offset(4 as isize);
        }
        id_size = p.offset_from(base) as i32;
        (*item).used_size += id_size;
        if AddValue(item, type_0, pvalue, size) == 0 {
            (*item).used_size -= id_size;
            return 0;
        }
        let ref mut fresh24 = (*item).count;
        *fresh24 += 1;
        return 1;
    }
}

extern "C" fn compress_int(
    mut pstorage_type: *mut i32,
    mut ptype: *mut i32,
    mut psource: *mut libc::c_void,
) -> *mut libc::c_void {
    unsafe {
        let mut current_block: u64;
        let mut storage_type: i32 = 0;
        let mut storage_type2: i32 = 0;
        let mut type_0: i32 = 0;
        let mut type2 = 0;
        let mut vint = 0;
        let mut vuint: u64 = 0;
        let mut pvalue = 0 as *mut i8;
        storage_type = *pstorage_type;
        if storage_type == 0x20 {
            return psource;
        }
        type_0 = *ptype;
        match type_0 {
            129 => {
                vint = *(psource as *mut i64);
                current_block = 11048997260478501647;
            }
            97 => {
                vint = *(psource as *mut i32) as i64;
                current_block = 11048997260478501647;
            }
            65 => {
                vint = *(psource as *mut i16) as i64;
                current_block = 11048997260478501647;
            }
            128 => {
                vuint = *(psource as *mut u64);
                current_block = 1818554471676300950;
            }
            96 => {
                vuint = *(psource as *mut u32) as u64;
                current_block = 1818554471676300950;
            }
            64 => {
                vuint = *(psource as *mut u16) as u64;
                current_block = 1818554471676300950;
            }
            _ => {
                current_block = 11048997260478501647;
            }
        }
        match current_block {
            11048997260478501647 => {
                if vint >= 0 {
                    vuint = vint as u64;
                    current_block = 1818554471676300950;
                } else {
                    if vint >= -128 as i64 {
                        type2 = 0x21;
                    } else if vint >= (-32767i32 - 1) as i64 {
                        type2 = 0x41;
                    } else if vint >= (-2147483647i32 - 1) as i64 {
                        type2 = 0x61;
                    }
                    current_block = 1757844130948290377;
                }
            }
            _ => {}
        }
        match current_block {
            1818554471676300950 => {
                if vuint <= 255 {
                    type2 = 0x20;
                } else if vuint <= 65535 {
                    type2 = 0x40;
                } else if vuint <= 4294967295 {
                    type2 = 0x60;
                }
            }
            _ => {}
        }
        pvalue = psource as *mut i8;
        if type2 != 0 && type2 != type_0 {
            *ptype = type2;
            storage_type2 = binn_get_write_storage(type2);
            *pstorage_type = storage_type2;
        }
        return pvalue as *mut libc::c_void;
    }
}

extern "C" fn AddValue(
    mut item: *mut binn,
    mut type_0: i32,
    mut pvalue: *mut libc::c_void,
    mut size: i32,
) -> i32 {
    unsafe {
        let mut int32: i32 = 0;
        let mut ArgSize: i32 = 0;
        let mut storage_type: i32 = 0;
        let mut extra_type: i32 = 0;
        let mut p = 0 as *mut u8;
        binn_get_type_info(type_0, &mut storage_type, &mut extra_type);
        if pvalue.is_null() {
            let mut current_block_1: u64;
            match storage_type {
                0 => {
                    current_block_1 = 11006700562992250127;
                }
                192 | 160 => {
                    if size == 0 {
                        current_block_1 = 11006700562992250127;
                    } else {
                        current_block_1 = 14336010400674820937;
                    }
                }
                _ => {
                    current_block_1 = 14336010400674820937;
                }
            }
            match current_block_1 {
                14336010400674820937 => return 0,
                _ => {}
            }
        }
        if type_family(type_0) == 0xf2 && (*item).disable_int_compression == 0 {
            pvalue = compress_int(&mut storage_type, &mut type_0, pvalue);
        }
        match storage_type {
            0 => {
                size = 0;
                ArgSize = size;
            }
            32 => {
                size = 1;
                ArgSize = size;
            }
            64 => {
                size = 2;
                ArgSize = size;
            }
            96 => {
                size = 4;
                ArgSize = size;
            }
            128 => {
                size = 8;
                ArgSize = size;
            }
            192 => {
                if size < 0 {
                    return 0;
                }
                ArgSize = size + 4;
            }
            160 => {
                if size < 0 {
                    return 0;
                }
                if size == 0 {
                    size = strlen2(pvalue as *mut i8) as i32;
                }
                ArgSize = size + 5;
            }
            224 => {
                if size <= 0 {
                    return 0;
                }
                ArgSize = size;
            }
            _ => return 0,
        }
        ArgSize += 2;
        if CheckAllocation(item, ArgSize) == 0 {
            return 0;
        }
        p = ((*item).pbuf as *mut u8).offset((*item).used_size as isize);
        if storage_type != 0xe0 {
            if type_0 > 255 {
                let mut type16 = type_0 as u16;
                copy_be16(p as *mut u16, &mut type16 as *mut u16);
                p = p.offset(2 as isize);
                (*item).used_size += 2;
            } else {
                *p = type_0 as u8;
                p = p.offset(1);
                let ref mut fresh25 = (*item).used_size;
                *fresh25 += 1;
            }
        }
        match storage_type {
            32 => {
                *(p as *mut i8) = *(pvalue as *mut i8);
                (*item).used_size += 1;
            }
            64 => {
                copy_be16(p as *mut u16, pvalue as *mut u16);
                (*item).used_size += 2;
            }
            96 => {
                copy_be32(p as *mut u32, pvalue as *mut u32);
                (*item).used_size += 4;
            }
            128 => {
                copy_be64(p as *mut u64, pvalue as *mut u64);
                (*item).used_size += 8;
            }
            192 | 160 => {
                if size > 127 {
                    int32 = (size as u32 | 0x80000000u32) as i32;
                    copy_be32(p as *mut u32, &mut int32 as *mut i32 as *mut u32);
                    p = p.offset(4 as isize);
                    (*item).used_size += 4;
                } else {
                    *p = size as u8;
                    p = p.offset(1);
                    let ref mut fresh26 = (*item).used_size;
                    *fresh26 += 1;
                }
                memcpy(p as *mut libc::c_void, pvalue, size as u64);
                if storage_type == 0xa0 {
                    p = p.offset(size as isize);
                    *(p as *mut i8) = 0;
                    size += 1;
                };
                (*item).used_size += size;
            }
            224 => {
                memcpy(p as *mut libc::c_void, pvalue, size as u64);
                (*item).used_size += size;
            }
            0 | _ => {}
        };
        (*item).dirty = 1;
        return 1;
    }
}

extern "C" fn binn_save_header(mut item: *mut binn) -> i32 {
    unsafe {
        let mut byte: u8 = 0;
        let mut p = 0 as *mut u8;
        let mut int32: i32 = 0;
        let mut size: i32 = 0;
        if item.is_null() {
            return 0;
        }
        p = ((*item).pbuf as *mut u8).offset(9 as isize);
        size = (*item).used_size - 9 + 3;
        if (*item).count > 127 {
            p = p.offset(-(4 as isize));
            size += 3;
            int32 = ((*item).count as u32 | 0x80000000u32) as i32;
            copy_be32(p as *mut u32, &mut int32 as *mut i32 as *mut u32);
        } else {
            p = p.offset(-1);
            *p = (*item).count as u8;
        }
        if size > 127 {
            p = p.offset(-(4 as isize));
            size += 3;
            int32 = (size as u32 | 0x80000000u32) as i32;
            copy_be32(p as *mut u32, &mut int32 as *mut i32 as *mut u32);
        } else {
            p = p.offset(-1);
            *p = size as u8;
        }
        p = p.offset(-1);
        *p = (*item).type_0 as u8;
        let ref mut fresh27 = (*item).ptr;
        *fresh27 = p as *mut libc::c_void;
        (*item).size = size;
        (*item).dirty = 0;
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn binn_free(mut item: *mut binn) {
    unsafe {
        if item.is_null() {
            return;
        }
        if (*item).writable != 0 && (*item).pre_allocated == 0 {
            free_fn.expect("non-null function pointer")((*item).pbuf);
        }
        if ((*item).freefn).is_some() {
            ((*item).freefn).expect("non-null function pointer")((*item).ptr);
        }
        if (*item).allocated != 0 {
            free_fn.expect("non-null function pointer")(item as *mut libc::c_void);
        } else {
            memset(
                item as *mut libc::c_void,
                0,
                ::std::mem::size_of::<binn>() as u64,
            );
            (*item).header = 0x1f22b11f;
        };
    }
}

#[no_mangle]
pub extern "C" fn binn_release(mut item: *mut binn) -> *mut libc::c_void {
    unsafe {
        let mut data = 0 as *mut libc::c_void;
        if item.is_null() {
            return 0 as *mut libc::c_void;
        }
        data = binn_ptr(item as *mut libc::c_void);
        if data > (*item).pbuf {
            memmove((*item).pbuf, data, (*item).size as u64);
            data = (*item).pbuf;
        }
        if (*item).allocated != 0 {
            free_fn.expect("non-null function pointer")(item as *mut libc::c_void);
        } else {
            memset(
                item as *mut libc::c_void,
                0,
                ::std::mem::size_of::<binn>() as u64,
            );
            (*item).header = 0x1f22b11f;
        }
        return data;
    }
}

extern "C" fn IsValidBinnHeader(
    mut pbuf: *mut libc::c_void,
    mut ptype: *mut i32,
    mut pcount: *mut i32,
    mut psize: *mut i32,
    mut pheadersize: *mut i32,
) -> i32 {
    unsafe {
        let mut byte: u8 = 0;
        let mut p = 0 as *mut u8;
        let mut plimit = 0 as *mut u8;
        let mut int32: i32 = 0;
        let mut type_0: i32 = 0;
        let mut size: i32 = 0;
        let mut count: i32 = 0;
        if pbuf.is_null() {
            return 0;
        }
        p = pbuf as *mut u8;
        if !psize.is_null() && *psize > 0 {
            plimit = p.offset(*psize as isize).offset(-(1 as isize));
        }
        byte = *p;
        p = p.offset(1);
        if byte as i32 & 0xe0 != 0xe0 {
            return 0;
        }
        if byte as i32 & 0x10 != 0 {
            return 0;
        }
        type_0 = byte as i32;
        match type_0 {
            224 | 225 | 226 => {}
            _ => return 0,
        }
        if !plimit.is_null() && p > plimit {
            return 0;
        }
        int32 = *p as i32;
        if int32 & 0x80 != 0 {
            if !plimit.is_null()
                && p.offset(::std::mem::size_of::<i32>() as u64 as isize)
                    .offset(-(1 as isize))
                    > plimit
            {
                return 0;
            }
            copy_be32(&mut int32 as *mut i32 as *mut u32, p as *mut u32);
            int32 &= 0x7fffffff;
            p = p.offset(4 as isize);
        } else {
            p = p.offset(1);
        }
        size = int32;
        if !plimit.is_null() && p > plimit {
            return 0;
        }
        int32 = *p as i32;
        if int32 & 0x80 != 0 {
            if !plimit.is_null()
                && p.offset(::std::mem::size_of::<i32>() as u64 as isize)
                    .offset(-(1 as isize))
                    > plimit
            {
                return 0;
            }
            copy_be32(&mut int32 as *mut i32 as *mut u32, p as *mut u32);
            int32 &= 0x7fffffff;
            p = p.offset(4 as isize);
        } else {
            p = p.offset(1);
        }
        count = int32;
        if size < 3 || count < 0 {
            return 0;
        }
        if !ptype.is_null() {
            *ptype = type_0;
        }
        if !pcount.is_null() {
            *pcount = count;
        }
        if !psize.is_null() && *psize == 0 {
            *psize = size;
        }
        if !pheadersize.is_null() {
            *pheadersize = p.offset_from(pbuf as *mut u8) as i32;
        }
        return 1;
    }
}

extern "C" fn binn_buf_type(mut pbuf: *mut libc::c_void) -> i32 {
    unsafe {
        let mut type_0: i32 = 0;
        if IsValidBinnHeader(
            pbuf,
            &mut type_0,
            0 as *mut i32,
            0 as *mut i32,
            0 as *mut i32,
        ) == 0
        {
            return 0;
        }
        return type_0;
    }
}

extern "C" fn binn_buf_count(mut pbuf: *mut libc::c_void) -> i32 {
    unsafe {
        let mut nitems: i32 = 0;
        if IsValidBinnHeader(
            pbuf,
            0 as *mut i32,
            &mut nitems,
            0 as *mut i32,
            0 as *mut i32,
        ) == 0
        {
            return 0;
        }
        return nitems;
    }
}

extern "C" fn binn_buf_size(mut pbuf: *mut libc::c_void) -> i32 {
    unsafe {
        let mut size = 0;
        if IsValidBinnHeader(pbuf, 0 as *mut i32, 0 as *mut i32, &mut size, 0 as *mut i32) == 0 {
            return 0;
        }
        return size;
    }
}

#[no_mangle]
pub extern "C" fn binn_ptr(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        let mut item = 0 as *mut binn;
        match binn_get_ptr_type(ptr) {
            1 => {
                item = ptr as *mut binn;
                if (*item).writable != 0 && (*item).dirty != 0 {
                    binn_save_header(item);
                }
                return (*item).ptr;
            }
            2 => return ptr,
            _ => return 0 as *mut libc::c_void,
        };
    }
}

#[no_mangle]
pub extern "C" fn binn_size(mut ptr: *mut libc::c_void) -> i32 {
    unsafe {
        let mut item = 0 as *mut binn;
        match binn_get_ptr_type(ptr) {
            1 => {
                item = ptr as *mut binn;
                if (*item).writable != 0 && (*item).dirty != 0 {
                    binn_save_header(item);
                }
                return (*item).size;
            }
            2 => return binn_buf_size(ptr),
            _ => return 0,
        };
    }
}

#[no_mangle]
pub extern "C" fn binn_type(mut ptr: *mut libc::c_void) -> i32 {
    unsafe {
        let mut item = 0 as *mut binn;
        match binn_get_ptr_type(ptr) {
            1 => {
                item = ptr as *mut binn;
                return (*item).type_0;
            }
            2 => return binn_buf_type(ptr),
            _ => return -1,
        };
    }
}

#[no_mangle]
pub extern "C" fn binn_count(mut ptr: *mut libc::c_void) -> i32 {
    unsafe {
        let mut item = 0 as *mut binn;
        match binn_get_ptr_type(ptr) {
            1 => {
                item = ptr as *mut binn;
                return (*item).count;
            }
            2 => return binn_buf_count(ptr),
            _ => return -1,
        };
    }
}

#[no_mangle]
pub extern "C" fn binn_is_valid_ex(
    mut ptr: *mut libc::c_void,
    mut ptype: *mut i32,
    mut pcount: *mut i32,
    mut psize: *mut i32,
) -> i32 {
    unsafe {
        let mut current_block: u64;
        let mut i: i32 = 0;
        let mut type_0: i32 = 0;
        let mut count: i32 = 0;
        let mut size: i32 = 0;
        let mut header_size: i32 = 0;
        let mut p = 0 as *mut u8;
        let mut plimit = 0 as *mut u8;
        let mut base = 0 as *mut u8;
        let mut len: u8 = 0;
        let mut pbuf = 0 as *mut libc::c_void;
        pbuf = binn_ptr(ptr);
        if pbuf.is_null() {
            return 0;
        }
        if !psize.is_null() && *psize > 0 {
            size = *psize;
        } else {
            size = 0;
        }
        if IsValidBinnHeader(pbuf, &mut type_0, &mut count, &mut size, &mut header_size) == 0 {
            return 0;
        }
        if !psize.is_null() && *psize > 0 {
            if size != *psize {
                return 0;
            }
        }
        if !pcount.is_null() && *pcount > 0 {
            if count != *pcount {
                return 0;
            }
        }
        if !ptype.is_null() && *ptype != 0 {
            if type_0 != *ptype {
                return 0;
            }
        }
        p = pbuf as *mut u8;
        base = p;
        plimit = p.offset(size as isize);
        p = p.offset(header_size as isize);
        i = 0;
        loop {
            if !(i < count) {
                current_block = 7245201122033322888;
                break;
            }
            match type_0 {
                226 => {
                    len = *p;
                    p = p.offset(1);
                    p = p.offset(len as i32 as isize);
                }
                225 => {
                    read_map_id(&mut p, plimit);
                }
                _ => {}
            }
            p = AdvanceDataPos(p, plimit);
            if p.is_null() || p < base {
                current_block = 11017899362201962608;
                break;
            }
            i += 1;
        }
        match current_block {
            11017899362201962608 => return 0,
            _ => {
                if !ptype.is_null() && *ptype == 0 {
                    *ptype = type_0;
                }
                if !pcount.is_null() && *pcount == 0 {
                    *pcount = count;
                }
                if !psize.is_null() && *psize == 0 {
                    *psize = size;
                }
                return 1;
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn binn_is_valid(
    mut ptr: *mut libc::c_void,
    mut ptype: *mut i32,
    mut pcount: *mut i32,
    mut psize: *mut i32,
) -> i32 {
    unsafe {
        if !ptype.is_null() {
            *ptype = 0;
        }
        if !pcount.is_null() {
            *pcount = 0;
        }
        if !psize.is_null() {
            *psize = 0;
        }
        return binn_is_valid_ex(ptr, ptype, pcount, psize);
    }
}

extern "C" fn GetValue(mut p: *mut u8, mut value: *mut binn) -> i32 {
    unsafe {
        let mut byte: u8 = 0;
        let mut data_type: i32 = 0;
        let mut storage_type: i32 = 0;
        let mut DataSize: i32 = 0;
        let mut p2 = 0 as *mut libc::c_void;
        if value.is_null() {
            return 0;
        }
        memset(
            value as *mut libc::c_void,
            0,
            ::std::mem::size_of::<binn>() as u64,
        );
        (*value).header = 0x1f22b11f;
        p2 = p as *mut libc::c_void;
        byte = *p;
        p = p.offset(1);
        storage_type = byte as i32 & 0xe0;
        if byte as i32 & 0x10 != 0 {
            data_type = (byte as i32) << 8;
            byte = *p;
            p = p.offset(1);
            data_type |= byte as i32;
        } else {
            data_type = byte as i32;
        };
        (*value).type_0 = data_type;
        match storage_type {
            0 => {}
            32 => {
                (*value).c2rust_unnamed.vuint8 = *p;
                let ref mut fresh28 = (*value).ptr;
                *fresh28 = p as *mut libc::c_void;
            }
            64 => {
                copy_be16(
                    &mut (*value).c2rust_unnamed.vint16 as *mut i16 as *mut u16,
                    p as *mut u16,
                );
                let ref mut fresh29 = (*value).ptr;
                *fresh29 = &mut (*value).c2rust_unnamed.vint16 as *mut i16 as *mut libc::c_void;
            }
            96 => {
                copy_be32(
                    &mut (*value).c2rust_unnamed.vint32 as *mut i32 as *mut u32,
                    p as *mut u32,
                );
                let ref mut fresh30 = (*value).ptr;
                *fresh30 = &mut (*value).c2rust_unnamed.vint32 as *mut i32 as *mut libc::c_void;
            }
            128 => {
                copy_be64(
                    &mut (*value).c2rust_unnamed.vint64 as *mut i64 as *mut u64,
                    p as *mut u64,
                );
                let ref mut fresh31 = (*value).ptr;
                *fresh31 = &mut (*value).c2rust_unnamed.vint64 as *mut i64 as *mut libc::c_void;
            }
            192 | 160 => {
                DataSize = *p as i32;
                if DataSize & 0x80 != 0 {
                    copy_be32(&mut DataSize as *mut i32 as *mut u32, p as *mut u32);
                    DataSize &= 0x7fffffff;
                    p = p.offset(4 as isize);
                } else {
                    p = p.offset(1);
                };
                (*value).size = DataSize;
                let ref mut fresh32 = (*value).ptr;
                *fresh32 = p as *mut libc::c_void;
            }
            224 => {
                let ref mut fresh33 = (*value).ptr;
                *fresh33 = p2;
                if IsValidBinnHeader(
                    p2,
                    0 as *mut i32,
                    &mut (*value).count,
                    &mut (*value).size,
                    0 as *mut i32,
                ) == 0
                {
                    return 0;
                }
            }
            _ => return 0,
        }
        match (*value).type_0 {
            1 => {
                (*value).type_0 = 0x80061;
                (*value).c2rust_unnamed.vbool = 1;
                let ref mut fresh34 = (*value).ptr;
                *fresh34 = &mut (*value).c2rust_unnamed.vbool as *mut i32 as *mut libc::c_void;
            }
            2 => {
                (*value).type_0 = 0x80061;
                (*value).c2rust_unnamed.vbool = 0;
                let ref mut fresh35 = (*value).ptr;
                *fresh35 = &mut (*value).c2rust_unnamed.vbool as *mut i32 as *mut libc::c_void;
            }
            _ => {}
        }
        return 1;
    }
}

#[no_mangle]
pub static mut local_value: binn = binn {
    header: 0,
    allocated: 0,
    writable: 0,
    dirty: 0,
    pbuf: 0 as *const libc::c_void as *mut libc::c_void,
    pre_allocated: 0,
    alloc_size: 0,
    used_size: 0,
    type_0: 0,
    ptr: 0 as *const libc::c_void as *mut libc::c_void,
    size: 0,
    count: 0,
    freefn: None,
    c2rust_unnamed: C2RustUnnamed { vint8: 0 },
    disable_int_compression: 0,
};
extern "C" fn store_value(mut value: *mut binn) -> *mut libc::c_void {
    unsafe {
        memcpy(
            &mut local_value as *mut binn as *mut libc::c_void,
            value as *const libc::c_void,
            ::std::mem::size_of::<binn>() as u64,
        );
        's_18: {
            match binn_get_read_storage((*value).type_0) {
                0 => {}
                64 | 96 | 128 => {}
                _ => {
                    break 's_18;
                }
            }
            return &mut local_value.c2rust_unnamed.vint32 as *mut i32 as *mut libc::c_void;
        }
        return (*value).ptr;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_get_value(
    mut ptr: *mut libc::c_void,
    mut key: *const i8,
    mut value: *mut binn,
) -> i32 {
    unsafe {
        let mut type_0: i32 = 0;
        let mut count: i32 = 0;
        let mut size = 0;
        let mut header_size: i32 = 0;
        let mut p = 0 as *mut u8;
        ptr = binn_ptr(ptr);
        if ptr.is_null() || key.is_null() || value.is_null() {
            return 0;
        }
        if IsValidBinnHeader(ptr, &mut type_0, &mut count, &mut size, &mut header_size) == 0 {
            return 0;
        }
        if type_0 != 0xe2 {
            return 0;
        }
        if count == 0 {
            return 0;
        }
        p = ptr as *mut u8;
        p = SearchForKey(p, header_size, size, count, key);
        if p.is_null() {
            return 0;
        }
        return GetValue(p, value);
    }
}

#[no_mangle]
pub extern "C" fn binn_map_get_value(
    mut ptr: *mut libc::c_void,
    mut id: i32,
    mut value: *mut binn,
) -> i32 {
    unsafe {
        let mut type_0: i32 = 0;
        let mut count: i32 = 0;
        let mut size = 0;
        let mut header_size: i32 = 0;
        let mut p = 0 as *mut u8;
        ptr = binn_ptr(ptr);
        if ptr.is_null() || value.is_null() {
            return 0;
        }
        if IsValidBinnHeader(ptr, &mut type_0, &mut count, &mut size, &mut header_size) == 0 {
            return 0;
        }
        if type_0 != 0xe1 {
            return 0;
        }
        if count == 0 {
            return 0;
        }
        p = ptr as *mut u8;
        p = SearchForID(p, header_size, size, count, id);
        if p.is_null() {
            return 0;
        }
        return GetValue(p, value);
    }
}

#[no_mangle]
pub extern "C" fn binn_list_get_value(
    mut ptr: *mut libc::c_void,
    mut pos: i32,
    mut value: *mut binn,
) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut type_0: i32 = 0;
        let mut count: i32 = 0;
        let mut size = 0;
        let mut header_size: i32 = 0;
        let mut p = 0 as *mut u8;
        let mut plimit = 0 as *mut u8;
        let mut base = 0 as *mut u8;
        ptr = binn_ptr(ptr);
        if ptr.is_null() || value.is_null() {
            return 0;
        }
        if IsValidBinnHeader(ptr, &mut type_0, &mut count, &mut size, &mut header_size) == 0 {
            return 0;
        }
        if type_0 != 0xe0 {
            return 0;
        }
        if count == 0 {
            return 0;
        }
        if (pos <= 0) as i32 | (pos > count) as i32 != 0 {
            return 0;
        }
        pos -= 1;
        p = ptr as *mut u8;
        base = p;
        plimit = p.offset(size as isize);
        p = p.offset(header_size as isize);
        i = 0;
        while i < pos {
            p = AdvanceDataPos(p, plimit);
            if p.is_null() || p < base {
                return 0;
            }
            i += 1;
        }
        return GetValue(p, value);
    }
}

extern "C" fn binn_read_pair(
    mut expected_type: i32,
    mut ptr: *mut libc::c_void,
    mut pos: i32,
    mut pid: *mut i32,
    mut pkey: *mut i8,
    mut value: *mut binn,
) -> i32 {
    unsafe {
        let mut current_block: u64;
        let mut type_0: i32 = 0;
        let mut count: i32 = 0;
        let mut size = 0;
        let mut header_size: i32 = 0;
        let mut i: i32 = 0;
        let mut int32: i32 = 0;
        let mut id = 0;
        let mut counter = 0;
        let mut p = 0 as *mut u8;
        let mut plimit = 0 as *mut u8;
        let mut base = 0 as *mut u8;
        let mut key = 0 as *mut u8;
        let mut len = 0;
        ptr = binn_ptr(ptr);
        if IsValidBinnHeader(ptr, &mut type_0, &mut count, &mut size, &mut header_size) == 0 {
            return 0;
        }
        if type_0 != expected_type || count == 0 || pos < 1 || pos > count {
            return 0;
        }
        p = ptr as *mut u8;
        base = p;
        plimit = p.offset(size as isize).offset(-(1 as isize));
        p = p.offset(header_size as isize);
        i = 0;
        loop {
            if !(i < count) {
                current_block = 14359455889292382949;
                break;
            }
            match type_0 {
                225 => {
                    int32 = read_map_id(&mut p, plimit);
                    if p > plimit {
                        return 0;
                    }
                    id = int32;
                }
                226 => {
                    len = *p;
                    p = p.offset(1);
                    if p > plimit {
                        return 0;
                    }
                    key = p;
                    p = p.offset(len as i32 as isize);
                    if p > plimit {
                        return 0;
                    }
                }
                _ => {}
            }
            counter += 1;
            if counter == pos {
                current_block = 3690914394173635162;
                break;
            }
            p = AdvanceDataPos(p, plimit);
            if p.is_null() || p < base {
                return 0;
            }
            i += 1;
        }
        match current_block {
            14359455889292382949 => return 0,
            _ => {
                match type_0 {
                    225 => {
                        if !pid.is_null() {
                            *pid = id;
                        }
                    }
                    226 => {
                        if !pkey.is_null() {
                            memcpy(
                                pkey as *mut libc::c_void,
                                key as *const libc::c_void,
                                len as u64,
                            );
                            *pkey.offset(len as isize) = 0;
                        }
                    }
                    _ => {}
                }
                return GetValue(p, value);
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn binn_map_get_pair(
    mut ptr: *mut libc::c_void,
    mut pos: i32,
    mut pid: *mut i32,
    mut value: *mut binn,
) -> i32 {
    unsafe {
        return binn_read_pair(0xe1, ptr, pos, pid, 0 as *mut i8, value);
    }
}

#[no_mangle]
pub extern "C" fn binn_object_get_pair(
    mut ptr: *mut libc::c_void,
    mut pos: i32,
    mut pkey: *mut i8,
    mut value: *mut binn,
) -> i32 {
    unsafe {
        return binn_read_pair(0xe2, ptr, pos, 0 as *mut i32, pkey, value);
    }
}

#[no_mangle]
pub extern "C" fn binn_map_pair(
    mut map: *mut libc::c_void,
    mut pos: i32,
    mut pid: *mut i32,
) -> *mut binn {
    unsafe {
        let mut value = 0 as *mut binn;
        value = binn_malloc(::std::mem::size_of::<binn>() as i32) as *mut binn;
        if binn_read_pair(0xe1, map, pos, pid, 0 as *mut i8, value) == 0 {
            free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
            return 0 as *mut binn;
        };
        (*value).allocated = 1;
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_pair(
    mut obj: *mut libc::c_void,
    mut pos: i32,
    mut pkey: *mut i8,
) -> *mut binn {
    unsafe {
        let mut value = 0 as *mut binn;
        value = binn_malloc(::std::mem::size_of::<binn>() as i32) as *mut binn;
        if binn_read_pair(0xe2, obj, pos, 0 as *mut i32, pkey, value) == 0 {
            free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
            return 0 as *mut binn;
        };
        (*value).allocated = 1;
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_read_pair(
    mut ptr: *mut libc::c_void,
    mut pos: i32,
    mut pid: *mut i32,
    mut ptype: *mut i32,
    mut psize: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let mut value = binn {
            header: 0,
            allocated: 0,
            writable: 0,
            dirty: 0,
            pbuf: 0 as *const libc::c_void as *mut libc::c_void,
            pre_allocated: 0,
            alloc_size: 0,
            used_size: 0,
            type_0: 0,
            ptr: 0 as *const libc::c_void as *mut libc::c_void,
            size: 0,
            count: 0,
            freefn: None,
            c2rust_unnamed: C2RustUnnamed { vint8: 0 },
            disable_int_compression: 0,
        };
        if binn_map_get_pair(ptr, pos, pid, &mut value) == 0 {
            return 0 as *mut libc::c_void;
        }
        if !ptype.is_null() {
            *ptype = value.type_0;
        }
        if !psize.is_null() {
            *psize = value.size;
        }
        return store_value(&mut value);
    }
}

#[no_mangle]
pub extern "C" fn binn_object_read_pair(
    mut ptr: *mut libc::c_void,
    mut pos: i32,
    mut pkey: *mut i8,
    mut ptype: *mut i32,
    mut psize: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let mut value = binn {
            header: 0,
            allocated: 0,
            writable: 0,
            dirty: 0,
            pbuf: 0 as *const libc::c_void as *mut libc::c_void,
            pre_allocated: 0,
            alloc_size: 0,
            used_size: 0,
            type_0: 0,
            ptr: 0 as *const libc::c_void as *mut libc::c_void,
            size: 0,
            count: 0,
            freefn: None,
            c2rust_unnamed: C2RustUnnamed { vint8: 0 },
            disable_int_compression: 0,
        };
        if binn_object_get_pair(ptr, pos, pkey, &mut value) == 0 {
            return 0 as *mut libc::c_void;
        }
        if !ptype.is_null() {
            *ptype = value.type_0;
        }
        if !psize.is_null() {
            *psize = value.size;
        }
        return store_value(&mut value);
    }
}

#[no_mangle]
pub extern "C" fn binn_iter_init(
    mut iter: *mut binn_iter,
    mut ptr: *mut libc::c_void,
    mut expected_type: i32,
) -> i32 {
    unsafe {
        let mut type_0: i32 = 0;
        let mut count: i32 = 0;
        let mut size = 0;
        let mut header_size: i32 = 0;
        ptr = binn_ptr(ptr);
        if ptr.is_null() || iter.is_null() {
            return 0;
        }
        memset(
            iter as *mut libc::c_void,
            0,
            ::std::mem::size_of::<binn_iter>() as u64,
        );
        if IsValidBinnHeader(ptr, &mut type_0, &mut count, &mut size, &mut header_size) == 0 {
            return 0;
        }
        if type_0 != expected_type {
            return 0;
        }
        let ref mut fresh36 = (*iter).plimit;
        *fresh36 = (ptr as *mut u8).offset(size as isize).offset(-(1 as isize));
        let ref mut fresh37 = (*iter).pnext;
        *fresh37 = (ptr as *mut u8).offset(header_size as isize);
        (*iter).count = count;
        (*iter).current = 0;
        (*iter).type_0 = type_0;
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_next(mut iter: *mut binn_iter, mut value: *mut binn) -> i32 {
    unsafe {
        let mut pnow = 0 as *mut u8;
        if iter.is_null()
            || ((*iter).pnext).is_null()
            || (*iter).pnext > (*iter).plimit
            || (*iter).current > (*iter).count
            || (*iter).type_0 != 0xe0
        {
            return 0;
        }
        let ref mut fresh38 = (*iter).current;
        *fresh38 += 1;
        if (*iter).current > (*iter).count {
            return 0;
        }
        pnow = (*iter).pnext;
        let ref mut fresh39 = (*iter).pnext;
        *fresh39 = AdvanceDataPos(pnow, (*iter).plimit);
        if !((*iter).pnext).is_null() && (*iter).pnext < pnow {
            return 0;
        }
        return GetValue(pnow, value);
    }
}

extern "C" fn binn_read_next_pair(
    mut expected_type: i32,
    mut iter: *mut binn_iter,
    mut pid: *mut i32,
    mut pkey: *mut i8,
    mut value: *mut binn,
) -> i32 {
    unsafe {
        let mut int32: i32 = 0;
        let mut id: i32 = 0;
        let mut p = 0 as *mut u8;
        let mut key = 0 as *mut u8;
        let mut len: u16 = 0;
        if iter.is_null()
            || ((*iter).pnext).is_null()
            || (*iter).pnext > (*iter).plimit
            || (*iter).current > (*iter).count
            || (*iter).type_0 != expected_type
        {
            return 0;
        }
        let ref mut fresh40 = (*iter).current;
        *fresh40 += 1;
        if (*iter).current > (*iter).count {
            return 0;
        }
        p = (*iter).pnext;
        match expected_type {
            225 => {
                int32 = read_map_id(&mut p, (*iter).plimit);
                if p > (*iter).plimit {
                    return 0;
                }
                id = int32;
                if !pid.is_null() {
                    *pid = id;
                }
            }
            226 => {
                len = *p as u16;
                p = p.offset(1);
                key = p;
                p = p.offset(len as i32 as isize);
                if p > (*iter).plimit {
                    return 0;
                }
                if !pkey.is_null() {
                    memcpy(
                        pkey as *mut libc::c_void,
                        key as *const libc::c_void,
                        len as u64,
                    );
                    *pkey.offset(len as isize) = 0;
                }
            }
            _ => {}
        }
        let ref mut fresh41 = (*iter).pnext;
        *fresh41 = AdvanceDataPos(p, (*iter).plimit);
        if !((*iter).pnext).is_null() && (*iter).pnext < p {
            return 0;
        }
        return GetValue(p, value);
    }
}

#[no_mangle]
pub extern "C" fn binn_map_next(
    mut iter: *mut binn_iter,
    mut pid: *mut i32,
    mut value: *mut binn,
) -> i32 {
    unsafe {
        return binn_read_next_pair(0xe1, iter, pid, 0 as *mut i8, value);
    }
}

#[no_mangle]
pub extern "C" fn binn_object_next(
    mut iter: *mut binn_iter,
    mut pkey: *mut i8,
    mut value: *mut binn,
) -> i32 {
    unsafe {
        return binn_read_next_pair(0xe2, iter, 0 as *mut i32, pkey, value);
    }
}

#[no_mangle]
pub extern "C" fn binn_list_next_value(mut iter: *mut binn_iter) -> *mut binn {
    unsafe {
        let mut value = 0 as *mut binn;
        value = binn_malloc(::std::mem::size_of::<binn>() as i32) as *mut binn;
        if binn_list_next(iter, value) == 0 {
            free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
            return 0 as *mut binn;
        };
        (*value).allocated = 1;
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_next_value(mut iter: *mut binn_iter, mut pid: *mut i32) -> *mut binn {
    unsafe {
        let mut value = 0 as *mut binn;
        value = binn_malloc(::std::mem::size_of::<binn>() as i32) as *mut binn;
        if binn_map_next(iter, pid, value) == 0 {
            free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
            return 0 as *mut binn;
        };
        (*value).allocated = 1;
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_next_value(mut iter: *mut binn_iter, mut pkey: *mut i8) -> *mut binn {
    unsafe {
        let mut value = 0 as *mut binn;
        value = binn_malloc(::std::mem::size_of::<binn>() as i32) as *mut binn;
        if binn_object_next(iter, pkey, value) == 0 {
            free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
            return 0 as *mut binn;
        };
        (*value).allocated = 1;
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_read_next(
    mut iter: *mut binn_iter,
    mut ptype: *mut i32,
    mut psize: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let mut value = binn {
            header: 0,
            allocated: 0,
            writable: 0,
            dirty: 0,
            pbuf: 0 as *const libc::c_void as *mut libc::c_void,
            pre_allocated: 0,
            alloc_size: 0,
            used_size: 0,
            type_0: 0,
            ptr: 0 as *const libc::c_void as *mut libc::c_void,
            size: 0,
            count: 0,
            freefn: None,
            c2rust_unnamed: C2RustUnnamed { vint8: 0 },
            disable_int_compression: 0,
        };
        if binn_list_next(iter, &mut value) == 0 {
            return 0 as *mut libc::c_void;
        }
        if !ptype.is_null() {
            *ptype = value.type_0;
        }
        if !psize.is_null() {
            *psize = value.size;
        }
        return store_value(&mut value);
    }
}

#[no_mangle]
pub extern "C" fn binn_map_read_next(
    mut iter: *mut binn_iter,
    mut pid: *mut i32,
    mut ptype: *mut i32,
    mut psize: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let mut value = binn {
            header: 0,
            allocated: 0,
            writable: 0,
            dirty: 0,
            pbuf: 0 as *const libc::c_void as *mut libc::c_void,
            pre_allocated: 0,
            alloc_size: 0,
            used_size: 0,
            type_0: 0,
            ptr: 0 as *const libc::c_void as *mut libc::c_void,
            size: 0,
            count: 0,
            freefn: None,
            c2rust_unnamed: C2RustUnnamed { vint8: 0 },
            disable_int_compression: 0,
        };
        if binn_map_next(iter, pid, &mut value) == 0 {
            return 0 as *mut libc::c_void;
        }
        if !ptype.is_null() {
            *ptype = value.type_0;
        }
        if !psize.is_null() {
            *psize = value.size;
        }
        return store_value(&mut value);
    }
}

#[no_mangle]
pub extern "C" fn binn_object_read_next(
    mut iter: *mut binn_iter,
    mut pkey: *mut i8,
    mut ptype: *mut i32,
    mut psize: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let mut value = binn {
            header: 0,
            allocated: 0,
            writable: 0,
            dirty: 0,
            pbuf: 0 as *const libc::c_void as *mut libc::c_void,
            pre_allocated: 0,
            alloc_size: 0,
            used_size: 0,
            type_0: 0,
            ptr: 0 as *const libc::c_void as *mut libc::c_void,
            size: 0,
            count: 0,
            freefn: None,
            c2rust_unnamed: C2RustUnnamed { vint8: 0 },
            disable_int_compression: 0,
        };
        if binn_object_next(iter, pkey, &mut value) == 0 {
            return 0 as *mut libc::c_void;
        }
        if !ptype.is_null() {
            *ptype = value.type_0;
        }
        if !psize.is_null() {
            *psize = value.size;
        }
        return store_value(&mut value);
    }
}

#[no_mangle]
pub extern "C" fn binn_get_write_storage(mut type_0: i32) -> i32 {
    let mut storage_type: i32 = 0;
    match type_0 {
        166 | 167 => return 0xa0,
        524385 => return 0,
        _ => {
            binn_get_type_info(type_0, &mut storage_type, 0 as *mut i32);
            return storage_type;
        }
    };
}

#[no_mangle]
pub extern "C" fn binn_get_read_storage(mut type_0: i32) -> i32 {
    let mut storage_type: i32 = 0;
    match type_0 {
        524385 | 1 | 2 => return 0x60,
        _ => {
            binn_get_type_info(type_0, &mut storage_type, 0 as *mut i32);
            return storage_type;
        }
    };
}

extern "C" fn GetWriteConvertedData(
    mut ptype: *mut i32,
    mut ppvalue: *mut *mut libc::c_void,
    mut psize: *mut i32,
) -> i32 {
    unsafe {
        let mut type_0: i32 = 0;
        let mut f1: libc::c_float = 0.;
        let mut d1: f64 = 0.;
        let mut pstr: [i8; 128] = [0; 128];
        type_0 = *ptype;
        if (*ppvalue).is_null() {
            let mut current_block_4: u64;
            match type_0 {
                0 | 1 | 2 => {
                    current_block_4 = 17965632435239708295;
                }
                160 | 192 => {
                    if *psize == 0 {
                        current_block_4 = 17965632435239708295;
                    } else {
                        current_block_4 = 4326909834884863099;
                    }
                }
                _ => {
                    current_block_4 = 4326909834884863099;
                }
            }
            match current_block_4 {
                4326909834884863099 => return 0,
                _ => {}
            }
        }
        match type_0 {
            164 | 165 => return 1,
            162 | 161 | 163 => return 1,
            524385 => {
                if **(ppvalue as *mut *mut i32) == 0 {
                    type_0 = 0x2;
                } else {
                    type_0 = 0x1;
                }
                *ptype = type_0;
            }
            _ => {}
        }
        return 1;
    }
}

extern "C" fn type_family(mut type_0: i32) -> i32 {
    match type_0 {
        224 | 225 | 226 => return 0xf7,
        33 | 65 | 97 | 129 | 32 | 64 | 96 | 128 => return 0xf2,
        98 | 130 => {}
        166 | 167 => {}
        160 | 45057 | 45061 | 45058 | 45059 | 45060 => return 0xf4,
        192 | 53249 | 53250 | 53251 | 53252 => return 0xf5,
        164 | 131 | 162 | 163 | 161 => return 0xf4,
        524385 => return 0xf6,
        0 => return 0xf1,
        _ => return 0,
    }
    return 0xf3;
}

extern "C" fn int_type(mut type_0: i32) -> i32 {
    match type_0 {
        33 | 65 | 97 | 129 => return 11,
        32 | 64 | 96 | 128 => return 22,
        _ => return 0,
    };
}

extern "C" fn copy_raw_value(
    mut psource: *mut libc::c_void,
    mut pdest: *mut libc::c_void,
    mut data_store: i32,
) -> i32 {
    unsafe {
        match data_store {
            0 => {}
            32 => {
                *(pdest as *mut i8) = *(psource as *mut i8);
            }
            64 => {
                *(pdest as *mut i16) = *(psource as *mut i16);
            }
            96 => {
                *(pdest as *mut i32) = *(psource as *mut i32);
            }
            128 => {
                *(pdest as *mut u64) = *(psource as *mut u64);
            }
            192 | 160 | 224 => {
                let ref mut fresh42 = *(pdest as *mut *mut i8);
                *fresh42 = psource as *mut i8;
            }
            _ => return 0,
        }
        return 1;
    }
}

extern "C" fn copy_int_value(
    mut psource: *mut libc::c_void,
    mut pdest: *mut libc::c_void,
    mut source_type: i32,
    mut dest_type: i32,
) -> i32 {
    unsafe {
        let mut vuint64 = 0;
        let mut vint64 = 0;
        match source_type {
            33 => {
                vint64 = *(psource as *mut i8) as i64;
            }
            65 => {
                vint64 = *(psource as *mut i16) as i64;
            }
            97 => {
                vint64 = *(psource as *mut i32) as i64;
            }
            129 => {
                vint64 = *(psource as *mut i64);
            }
            32 => {
                vuint64 = *(psource as *mut u8) as u64;
            }
            64 => {
                vuint64 = *(psource as *mut u16) as u64;
            }
            96 => {
                vuint64 = *(psource as *mut u32) as u64;
            }
            128 => {
                vuint64 = *(psource as *mut u64);
            }
            _ => return 0,
        }
        if int_type(source_type) == 22 && int_type(dest_type) == 11 {
            if vuint64 > 9223372036854775807 {
                return 0;
            }
            vint64 = vuint64 as i64;
        } else if int_type(source_type) == 11 && int_type(dest_type) == 22 {
            if vint64 < 0 {
                return 0;
            }
            vuint64 = vint64 as u64;
        }
        match dest_type {
            33 => {
                if vint64 < -128 as i64 || vint64 > 127 {
                    return 0;
                }
                *(pdest as *mut i8) = vint64 as i8;
            }
            65 => {
                if vint64 < (-32767i32 - 1) as i64 || vint64 > 32767 {
                    return 0;
                }
                *(pdest as *mut i16) = vint64 as i16;
            }
            97 => {
                if vint64 < (-2147483647i32 - 1) as i64 || vint64 > 2147483647 {
                    return 0;
                }
                *(pdest as *mut i32) = vint64 as i32;
            }
            129 => {
                *(pdest as *mut i64) = vint64;
            }
            32 => {
                if vuint64 > 255 {
                    return 0;
                }
                *(pdest as *mut u8) = vuint64 as u8;
            }
            64 => {
                if vuint64 > 65535 {
                    return 0;
                }
                *(pdest as *mut u16) = vuint64 as u16;
            }
            96 => {
                if vuint64 > 4294967295 {
                    return 0;
                }
                *(pdest as *mut u32) = vuint64 as u32;
            }
            128 => {
                *(pdest as *mut u64) = vuint64;
            }
            _ => return 0,
        }
        return 1;
    }
}

extern "C" fn copy_float_value(
    mut psource: *mut libc::c_void,
    mut pdest: *mut libc::c_void,
    mut source_type: i32,
    mut dest_type: i32,
) -> i32 {
    unsafe {
        match source_type {
            98 => {
                *(pdest as *mut f64) = *(psource as *mut libc::c_float) as f64;
            }
            130 => {
                *(pdest as *mut libc::c_float) = *(psource as *mut f64) as libc::c_float;
            }
            _ => return 0,
        }
        return 1;
    }
}

extern "C" fn zero_value(mut pvalue: *mut libc::c_void, mut type_0: i32) {
    unsafe {
        match binn_get_read_storage(type_0) {
            32 => {
                *(pvalue as *mut i8) = 0;
            }
            64 => {
                *(pvalue as *mut i16) = 0;
            }
            96 => {
                *(pvalue as *mut i32) = 0;
            }
            128 => {
                *(pvalue as *mut u64) = 0;
            }
            192 | 160 | 224 => {
                let ref mut fresh43 = *(pvalue as *mut *mut i8);
                *fresh43 = 0 as *mut i8;
            }
            0 | _ => {}
        };
    }
}

extern "C" fn copy_value(
    mut psource: *mut libc::c_void,
    mut pdest: *mut libc::c_void,
    mut source_type: i32,
    mut dest_type: i32,
    mut data_store: i32,
) -> i32 {
    unsafe {
        if type_family(source_type) != type_family(dest_type) {
            return 0;
        }
        if type_family(source_type) == 0xf2 && source_type != dest_type {
            return copy_int_value(psource, pdest, source_type, dest_type);
        } else if type_family(source_type) == 0xf3 && source_type != dest_type {
            return copy_float_value(psource, pdest, source_type, dest_type);
        } else {
            return copy_raw_value(psource, pdest, data_store);
        };
    }
}

#[no_mangle]
pub extern "C" fn binn_list_add(
    mut list: *mut binn,
    mut type_0: i32,
    mut pvalue: *mut libc::c_void,
    mut size: i32,
) -> i32 {
    unsafe {
        if GetWriteConvertedData(&mut type_0, &mut pvalue, &mut size) == 0 {
            return 0;
        }
        return binn_list_add_raw(list, type_0, pvalue, size);
    }
}

#[no_mangle]
pub extern "C" fn binn_map_set(
    mut map: *mut binn,
    mut id: i32,
    mut type_0: i32,
    mut pvalue: *mut libc::c_void,
    mut size: i32,
) -> i32 {
    unsafe {
        if GetWriteConvertedData(&mut type_0, &mut pvalue, &mut size) == 0 {
            return 0;
        }
        return binn_map_set_raw(map, id, type_0, pvalue, size);
    }
}

#[no_mangle]
pub extern "C" fn binn_object_set(
    mut obj: *mut binn,
    mut key: *const i8,
    mut type_0: i32,
    mut pvalue: *mut libc::c_void,
    mut size: i32,
) -> i32 {
    unsafe {
        if GetWriteConvertedData(&mut type_0, &mut pvalue, &mut size) == 0 {
            return 0;
        }
        return binn_object_set_raw(obj, key, type_0, pvalue, size);
    }
}

#[no_mangle]
pub extern "C" fn binn_add_value(
    mut item: *mut binn,
    mut binn_type_0: i32,
    mut id: i32,
    mut name: *mut i8,
    mut type_0: i32,
    mut pvalue: *mut libc::c_void,
    mut size: i32,
) -> i32 {
    unsafe {
        match binn_type_0 {
            224 => return binn_list_add(item, type_0, pvalue, size),
            225 => return binn_map_set(item, id, type_0, pvalue, size),
            226 => return binn_object_set(item, name, type_0, pvalue, size),
            _ => return 0,
        };
    }
}

#[no_mangle]
pub extern "C" fn binn_list_add_new(mut list: *mut binn, mut value: *mut binn) -> i32 {
    unsafe {
        let mut retval: i32 = 0;
        retval = binn_list_add_value(list, value);
        if !value.is_null() {
            free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
        }
        return retval;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_set_new(mut map: *mut binn, mut id: i32, mut value: *mut binn) -> i32 {
    unsafe {
        let mut retval: i32 = 0;
        retval = binn_map_set_value(map, id, value);
        if !value.is_null() {
            free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
        }
        return retval;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_set_new(
    mut obj: *mut binn,
    mut key: *const i8,
    mut value: *mut binn,
) -> i32 {
    unsafe {
        let mut retval: i32 = 0;
        retval = binn_object_set_value(obj, key, value);
        if !value.is_null() {
            free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
        }
        return retval;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_value(mut ptr: *mut libc::c_void, mut pos: i32) -> *mut binn {
    unsafe {
        let mut value = 0 as *mut binn;
        value = binn_malloc(::std::mem::size_of::<binn>() as i32) as *mut binn;
        if binn_list_get_value(ptr, pos, value) == 0 {
            free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
            return 0 as *mut binn;
        };
        (*value).allocated = 1;
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_value(mut ptr: *mut libc::c_void, mut id: i32) -> *mut binn {
    unsafe {
        let mut value = 0 as *mut binn;
        value = binn_malloc(::std::mem::size_of::<binn>() as i32) as *mut binn;
        if binn_map_get_value(ptr, id, value) == 0 {
            free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
            return 0 as *mut binn;
        };
        (*value).allocated = 1;
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_value(mut ptr: *mut libc::c_void, mut key: *const i8) -> *mut binn {
    unsafe {
        let mut value = 0 as *mut binn;
        value = binn_malloc(::std::mem::size_of::<binn>() as i32) as *mut binn;
        if binn_object_get_value(ptr, key, value) == 0 {
            free_fn.expect("non-null function pointer")(value as *mut libc::c_void);
            return 0 as *mut binn;
        };
        (*value).allocated = 1;
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_read(
    mut list: *mut libc::c_void,
    mut pos: i32,
    mut ptype: *mut i32,
    mut psize: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let mut value = binn {
            header: 0,
            allocated: 0,
            writable: 0,
            dirty: 0,
            pbuf: 0 as *const libc::c_void as *mut libc::c_void,
            pre_allocated: 0,
            alloc_size: 0,
            used_size: 0,
            type_0: 0,
            ptr: 0 as *const libc::c_void as *mut libc::c_void,
            size: 0,
            count: 0,
            freefn: None,
            c2rust_unnamed: C2RustUnnamed { vint8: 0 },
            disable_int_compression: 0,
        };
        if binn_list_get_value(list, pos, &mut value) == 0 {
            return 0 as *mut libc::c_void;
        }
        if !ptype.is_null() {
            *ptype = value.type_0;
        }
        if !psize.is_null() {
            *psize = value.size;
        }
        return store_value(&mut value);
    }
}

#[no_mangle]
pub extern "C" fn binn_map_read(
    mut map: *mut libc::c_void,
    mut id: i32,
    mut ptype: *mut i32,
    mut psize: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let mut value = binn {
            header: 0,
            allocated: 0,
            writable: 0,
            dirty: 0,
            pbuf: 0 as *const libc::c_void as *mut libc::c_void,
            pre_allocated: 0,
            alloc_size: 0,
            used_size: 0,
            type_0: 0,
            ptr: 0 as *const libc::c_void as *mut libc::c_void,
            size: 0,
            count: 0,
            freefn: None,
            c2rust_unnamed: C2RustUnnamed { vint8: 0 },
            disable_int_compression: 0,
        };
        if binn_map_get_value(map, id, &mut value) == 0 {
            return 0 as *mut libc::c_void;
        }
        if !ptype.is_null() {
            *ptype = value.type_0;
        }
        if !psize.is_null() {
            *psize = value.size;
        }
        return store_value(&mut value);
    }
}

#[no_mangle]
pub extern "C" fn binn_object_read(
    mut obj: *mut libc::c_void,
    mut key: *const i8,
    mut ptype: *mut i32,
    mut psize: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let mut value = binn {
            header: 0,
            allocated: 0,
            writable: 0,
            dirty: 0,
            pbuf: 0 as *const libc::c_void as *mut libc::c_void,
            pre_allocated: 0,
            alloc_size: 0,
            used_size: 0,
            type_0: 0,
            ptr: 0 as *const libc::c_void as *mut libc::c_void,
            size: 0,
            count: 0,
            freefn: None,
            c2rust_unnamed: C2RustUnnamed { vint8: 0 },
            disable_int_compression: 0,
        };
        if binn_object_get_value(obj, key, &mut value) == 0 {
            return 0 as *mut libc::c_void;
        }
        if !ptype.is_null() {
            *ptype = value.type_0;
        }
        if !psize.is_null() {
            *psize = value.size;
        }
        return store_value(&mut value);
    }
}

#[no_mangle]
pub extern "C" fn binn_list_get(
    mut ptr: *mut libc::c_void,
    mut pos: i32,
    mut type_0: i32,
    mut pvalue: *mut libc::c_void,
    mut psize: *mut i32,
) -> i32 {
    unsafe {
        let mut value = binn {
            header: 0,
            allocated: 0,
            writable: 0,
            dirty: 0,
            pbuf: 0 as *const libc::c_void as *mut libc::c_void,
            pre_allocated: 0,
            alloc_size: 0,
            used_size: 0,
            type_0: 0,
            ptr: 0 as *const libc::c_void as *mut libc::c_void,
            size: 0,
            count: 0,
            freefn: None,
            c2rust_unnamed: C2RustUnnamed { vint8: 0 },
            disable_int_compression: 0,
        };
        let mut storage_type: i32 = 0;
        storage_type = binn_get_read_storage(type_0);
        if storage_type != 0 && pvalue.is_null() {
            return 0;
        }
        zero_value(pvalue, type_0);
        if binn_list_get_value(ptr, pos, &mut value) == 0 {
            return 0;
        }
        if copy_value(value.ptr, pvalue, value.type_0, type_0, storage_type) == 0 {
            return 0;
        }
        if !psize.is_null() {
            *psize = value.size;
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_get(
    mut ptr: *mut libc::c_void,
    mut id: i32,
    mut type_0: i32,
    mut pvalue: *mut libc::c_void,
    mut psize: *mut i32,
) -> i32 {
    unsafe {
        let mut value = binn {
            header: 0,
            allocated: 0,
            writable: 0,
            dirty: 0,
            pbuf: 0 as *const libc::c_void as *mut libc::c_void,
            pre_allocated: 0,
            alloc_size: 0,
            used_size: 0,
            type_0: 0,
            ptr: 0 as *const libc::c_void as *mut libc::c_void,
            size: 0,
            count: 0,
            freefn: None,
            c2rust_unnamed: C2RustUnnamed { vint8: 0 },
            disable_int_compression: 0,
        };
        let mut storage_type: i32 = 0;
        storage_type = binn_get_read_storage(type_0);
        if storage_type != 0 && pvalue.is_null() {
            return 0;
        }
        zero_value(pvalue, type_0);
        if binn_map_get_value(ptr, id, &mut value) == 0 {
            return 0;
        }
        if copy_value(value.ptr, pvalue, value.type_0, type_0, storage_type) == 0 {
            return 0;
        }
        if !psize.is_null() {
            *psize = value.size;
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_get(
    mut ptr: *mut libc::c_void,
    mut key: *const i8,
    mut type_0: i32,
    mut pvalue: *mut libc::c_void,
    mut psize: *mut i32,
) -> i32 {
    unsafe {
        let mut value = binn {
            header: 0,
            allocated: 0,
            writable: 0,
            dirty: 0,
            pbuf: 0 as *const libc::c_void as *mut libc::c_void,
            pre_allocated: 0,
            alloc_size: 0,
            used_size: 0,
            type_0: 0,
            ptr: 0 as *const libc::c_void as *mut libc::c_void,
            size: 0,
            count: 0,
            freefn: None,
            c2rust_unnamed: C2RustUnnamed { vint8: 0 },
            disable_int_compression: 0,
        };
        let mut storage_type: i32 = 0;
        storage_type = binn_get_read_storage(type_0);
        if storage_type != 0 && pvalue.is_null() {
            return 0;
        }
        zero_value(pvalue, type_0);
        if binn_object_get_value(ptr, key, &mut value) == 0 {
            return 0;
        }
        if copy_value(value.ptr, pvalue, value.type_0, type_0, storage_type) == 0 {
            return 0;
        }
        if !psize.is_null() {
            *psize = value.size;
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_int8(mut list: *mut libc::c_void, mut pos: i32) -> i8 {
    unsafe {
        let mut value: i8 = 0;
        binn_list_get(
            list,
            pos,
            0x21,
            &mut value as *mut i8 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_int16(mut list: *mut libc::c_void, mut pos: i32) -> i16 {
    unsafe {
        let mut value: i16 = 0;
        binn_list_get(
            list,
            pos,
            0x41,
            &mut value as *mut i16 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_int32(mut list: *mut libc::c_void, mut pos: i32) -> i32 {
    unsafe {
        let mut value: i32 = 0;
        binn_list_get(
            list,
            pos,
            0x61,
            &mut value as *mut i32 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_int64(mut list: *mut libc::c_void, mut pos: i32) -> i64 {
    unsafe {
        let mut value: i64 = 0;
        binn_list_get(
            list,
            pos,
            0x81,
            &mut value as *mut i64 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_uint8(mut list: *mut libc::c_void, mut pos: i32) -> u8 {
    unsafe {
        let mut value: u8 = 0;
        binn_list_get(
            list,
            pos,
            0x20,
            &mut value as *mut u8 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_uint16(mut list: *mut libc::c_void, mut pos: i32) -> u16 {
    unsafe {
        let mut value: u16 = 0;
        binn_list_get(
            list,
            pos,
            0x40,
            &mut value as *mut u16 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_uint32(mut list: *mut libc::c_void, mut pos: i32) -> u32 {
    unsafe {
        let mut value: u32 = 0;
        binn_list_get(
            list,
            pos,
            0x60,
            &mut value as *mut u32 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_uint64(mut list: *mut libc::c_void, mut pos: i32) -> u64 {
    unsafe {
        let mut value: u64 = 0;
        binn_list_get(
            list,
            pos,
            0x80,
            &mut value as *mut u64 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_float(mut list: *mut libc::c_void, mut pos: i32) -> libc::c_float {
    unsafe {
        let mut value: libc::c_float = 0.;
        binn_list_get(
            list,
            pos,
            0x62,
            &mut value as *mut libc::c_float as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_double(mut list: *mut libc::c_void, mut pos: i32) -> f64 {
    unsafe {
        let mut value: f64 = 0.;
        binn_list_get(
            list,
            pos,
            0x82,
            &mut value as *mut f64 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_bool(mut list: *mut libc::c_void, mut pos: i32) -> i32 {
    unsafe {
        let mut value: i32 = 0;
        binn_list_get(
            list,
            pos,
            0x80061,
            &mut value as *mut i32 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_null(mut list: *mut libc::c_void, mut pos: i32) -> i32 {
    unsafe {
        return binn_list_get(list, pos, 0, 0 as *mut libc::c_void, 0 as *mut i32);
    }
}

#[no_mangle]
pub extern "C" fn binn_list_str(mut list: *mut libc::c_void, mut pos: i32) -> *mut i8 {
    unsafe {
        let mut value = 0 as *mut i8;
        binn_list_get(
            list,
            pos,
            0xa0,
            &mut value as *mut *mut i8 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_blob(
    mut list: *mut libc::c_void,
    mut pos: i32,
    mut psize: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let mut value = 0 as *mut libc::c_void;
        binn_list_get(
            list,
            pos,
            0xc0,
            &mut value as *mut *mut libc::c_void as *mut libc::c_void,
            psize,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_list(mut list: *mut libc::c_void, mut pos: i32) -> *mut libc::c_void {
    unsafe {
        let mut value = 0 as *mut libc::c_void;
        binn_list_get(
            list,
            pos,
            0xe0,
            &mut value as *mut *mut libc::c_void as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_map(mut list: *mut libc::c_void, mut pos: i32) -> *mut libc::c_void {
    unsafe {
        let mut value = 0 as *mut libc::c_void;
        binn_list_get(
            list,
            pos,
            0xe1,
            &mut value as *mut *mut libc::c_void as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_list_object(mut list: *mut libc::c_void, mut pos: i32) -> *mut libc::c_void {
    unsafe {
        let mut value = 0 as *mut libc::c_void;
        binn_list_get(
            list,
            pos,
            0xe2,
            &mut value as *mut *mut libc::c_void as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_int8(mut map: *mut libc::c_void, mut id: i32) -> i8 {
    unsafe {
        let mut value: i8 = 0;
        binn_map_get(
            map,
            id,
            0x21,
            &mut value as *mut i8 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_int16(mut map: *mut libc::c_void, mut id: i32) -> i16 {
    unsafe {
        let mut value: i16 = 0;
        binn_map_get(
            map,
            id,
            0x41,
            &mut value as *mut i16 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_int32(mut map: *mut libc::c_void, mut id: i32) -> i32 {
    unsafe {
        let mut value: i32 = 0;
        binn_map_get(
            map,
            id,
            0x61,
            &mut value as *mut i32 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_int64(mut map: *mut libc::c_void, mut id: i32) -> i64 {
    unsafe {
        let mut value: i64 = 0;
        binn_map_get(
            map,
            id,
            0x81,
            &mut value as *mut i64 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_uint8(mut map: *mut libc::c_void, mut id: i32) -> u8 {
    unsafe {
        let mut value: u8 = 0;
        binn_map_get(
            map,
            id,
            0x20,
            &mut value as *mut u8 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_uint16(mut map: *mut libc::c_void, mut id: i32) -> u16 {
    unsafe {
        let mut value: u16 = 0;
        binn_map_get(
            map,
            id,
            0x40,
            &mut value as *mut u16 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_uint32(mut map: *mut libc::c_void, mut id: i32) -> u32 {
    unsafe {
        let mut value: u32 = 0;
        binn_map_get(
            map,
            id,
            0x60,
            &mut value as *mut u32 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_uint64(mut map: *mut libc::c_void, mut id: i32) -> u64 {
    unsafe {
        let mut value: u64 = 0;
        binn_map_get(
            map,
            id,
            0x80,
            &mut value as *mut u64 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_float(mut map: *mut libc::c_void, mut id: i32) -> libc::c_float {
    unsafe {
        let mut value: libc::c_float = 0.;
        binn_map_get(
            map,
            id,
            0x62,
            &mut value as *mut libc::c_float as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_double(mut map: *mut libc::c_void, mut id: i32) -> f64 {
    unsafe {
        let mut value: f64 = 0.;
        binn_map_get(
            map,
            id,
            0x82,
            &mut value as *mut f64 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_bool(mut map: *mut libc::c_void, mut id: i32) -> i32 {
    unsafe {
        let mut value: i32 = 0;
        binn_map_get(
            map,
            id,
            0x80061,
            &mut value as *mut i32 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_null(mut map: *mut libc::c_void, mut id: i32) -> i32 {
    unsafe {
        return binn_map_get(map, id, 0, 0 as *mut libc::c_void, 0 as *mut i32);
    }
}

#[no_mangle]
pub extern "C" fn binn_map_str(mut map: *mut libc::c_void, mut id: i32) -> *mut i8 {
    unsafe {
        let mut value = 0 as *mut i8;
        binn_map_get(
            map,
            id,
            0xa0,
            &mut value as *mut *mut i8 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_blob(
    mut map: *mut libc::c_void,
    mut id: i32,
    mut psize: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let mut value = 0 as *mut libc::c_void;
        binn_map_get(
            map,
            id,
            0xc0,
            &mut value as *mut *mut libc::c_void as *mut libc::c_void,
            psize,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_list(mut map: *mut libc::c_void, mut id: i32) -> *mut libc::c_void {
    unsafe {
        let mut value = 0 as *mut libc::c_void;
        binn_map_get(
            map,
            id,
            0xe0,
            &mut value as *mut *mut libc::c_void as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_map(mut map: *mut libc::c_void, mut id: i32) -> *mut libc::c_void {
    unsafe {
        let mut value = 0 as *mut libc::c_void;
        binn_map_get(
            map,
            id,
            0xe1,
            &mut value as *mut *mut libc::c_void as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_map_object(mut map: *mut libc::c_void, mut id: i32) -> *mut libc::c_void {
    unsafe {
        let mut value = 0 as *mut libc::c_void;
        binn_map_get(
            map,
            id,
            0xe2,
            &mut value as *mut *mut libc::c_void as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_int8(mut obj: *mut libc::c_void, mut key: *const i8) -> i8 {
    unsafe {
        let mut value: i8 = 0;
        binn_object_get(
            obj,
            key,
            0x21,
            &mut value as *mut i8 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_int16(mut obj: *mut libc::c_void, mut key: *const i8) -> i16 {
    unsafe {
        let mut value: i16 = 0;
        binn_object_get(
            obj,
            key,
            0x41,
            &mut value as *mut i16 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_int32(mut obj: *mut libc::c_void, mut key: *const i8) -> i32 {
    unsafe {
        let mut value: i32 = 0;
        binn_object_get(
            obj,
            key,
            0x61,
            &mut value as *mut i32 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_int64(mut obj: *mut libc::c_void, mut key: *const i8) -> i64 {
    unsafe {
        let mut value: i64 = 0;
        binn_object_get(
            obj,
            key,
            0x81,
            &mut value as *mut i64 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_uint8(mut obj: *mut libc::c_void, mut key: *const i8) -> u8 {
    unsafe {
        let mut value: u8 = 0;
        binn_object_get(
            obj,
            key,
            0x20,
            &mut value as *mut u8 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_uint16(mut obj: *mut libc::c_void, mut key: *const i8) -> u16 {
    unsafe {
        let mut value: u16 = 0;
        binn_object_get(
            obj,
            key,
            0x40,
            &mut value as *mut u16 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_uint32(mut obj: *mut libc::c_void, mut key: *const i8) -> u32 {
    unsafe {
        let mut value: u32 = 0;
        binn_object_get(
            obj,
            key,
            0x60,
            &mut value as *mut u32 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_uint64(mut obj: *mut libc::c_void, mut key: *const i8) -> u64 {
    unsafe {
        let mut value: u64 = 0;
        binn_object_get(
            obj,
            key,
            0x80,
            &mut value as *mut u64 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_float(
    mut obj: *mut libc::c_void,
    mut key: *const i8,
) -> libc::c_float {
    unsafe {
        let mut value: libc::c_float = 0.;
        binn_object_get(
            obj,
            key,
            0x62,
            &mut value as *mut libc::c_float as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_double(mut obj: *mut libc::c_void, mut key: *const i8) -> f64 {
    unsafe {
        let mut value: f64 = 0.;
        binn_object_get(
            obj,
            key,
            0x82,
            &mut value as *mut f64 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_bool(mut obj: *mut libc::c_void, mut key: *const i8) -> i32 {
    unsafe {
        let mut value: i32 = 0;
        binn_object_get(
            obj,
            key,
            0x80061,
            &mut value as *mut i32 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_null(mut obj: *mut libc::c_void, mut key: *const i8) -> i32 {
    unsafe {
        return binn_object_get(obj, key, 0, 0 as *mut libc::c_void, 0 as *mut i32);
    }
}

#[no_mangle]
pub extern "C" fn binn_object_str(mut obj: *mut libc::c_void, mut key: *const i8) -> *mut i8 {
    unsafe {
        let mut value = 0 as *mut i8;
        binn_object_get(
            obj,
            key,
            0xa0,
            &mut value as *mut *mut i8 as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_blob(
    mut obj: *mut libc::c_void,
    mut key: *const i8,
    mut psize: *mut i32,
) -> *mut libc::c_void {
    unsafe {
        let mut value = 0 as *mut libc::c_void;
        binn_object_get(
            obj,
            key,
            0xc0,
            &mut value as *mut *mut libc::c_void as *mut libc::c_void,
            psize,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_list(
    mut obj: *mut libc::c_void,
    mut key: *const i8,
) -> *mut libc::c_void {
    unsafe {
        let mut value = 0 as *mut libc::c_void;
        binn_object_get(
            obj,
            key,
            0xe0,
            &mut value as *mut *mut libc::c_void as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_map(
    mut obj: *mut libc::c_void,
    mut key: *const i8,
) -> *mut libc::c_void {
    unsafe {
        let mut value = 0 as *mut libc::c_void;
        binn_object_get(
            obj,
            key,
            0xe1,
            &mut value as *mut *mut libc::c_void as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

#[no_mangle]
pub extern "C" fn binn_object_object(
    mut obj: *mut libc::c_void,
    mut key: *const i8,
) -> *mut libc::c_void {
    unsafe {
        let mut value = 0 as *mut libc::c_void;
        binn_object_get(
            obj,
            key,
            0xe2,
            &mut value as *mut *mut libc::c_void as *mut libc::c_void,
            0 as *mut i32,
        );
        return value;
    }
}

extern "C" fn binn_alloc_item() -> *mut binn {
    let mut item = 0 as *mut binn;
    item = binn_malloc(::std::mem::size_of::<binn>() as i32) as *mut binn;
    unsafe {
        if !item.is_null() {
            memset(
                item as *mut libc::c_void,
                0,
                ::std::mem::size_of::<binn>() as u64,
            );
            (*item).header = 0x1f22b11f;
            (*item).allocated = 1;
        }
    }
    return item;
}

#[no_mangle]
pub extern "C" fn binn_value(
    mut type_0: i32,
    mut pvalue: *mut libc::c_void,
    mut size: i32,
    mut freefn: binn_mem_free,
) -> *mut binn {
    unsafe {
        let mut storage_type: i32 = 0;
        let mut item = binn_alloc_item();
        if !item.is_null() {
            (*item).type_0 = type_0;
            binn_get_type_info(type_0, &mut storage_type, 0 as *mut i32);
            let mut current_block_19: u64;
            match storage_type {
                0 => {
                    current_block_19 = 1109700713171191020;
                }
                160 => {
                    if size == 0 {
                        size = (strlen(pvalue as *mut i8)).wrapping_add(1) as i32;
                    }
                    current_block_19 = 3116744561211014279;
                }
                192 | 224 => {
                    current_block_19 = 3116744561211014279;
                }
                _ => {
                    let ref mut fresh48 = (*item).ptr;
                    *fresh48 = &mut (*item).c2rust_unnamed.vint32 as *mut i32 as *mut libc::c_void;
                    copy_raw_value(pvalue, (*item).ptr, storage_type);
                    current_block_19 = 1109700713171191020;
                }
            }
            match current_block_19 {
                3116744561211014279 => {
                    if freefn
                        == ::std::mem::transmute::<libc::intptr_t, binn_mem_free>(
                            -1i32 as libc::intptr_t,
                        )
                    {
                        let ref mut fresh44 = (*item).ptr;
                        *fresh44 = binn_memdup(pvalue, size);
                        if ((*item).ptr).is_null() {
                            free_fn.expect("non-null function pointer")(item as *mut libc::c_void);
                            return 0 as *mut binn;
                        }
                        let ref mut fresh45 = (*item).freefn;
                        *fresh45 = free_fn;
                        if storage_type == 0xa0 {
                            size -= 1;
                        }
                    } else {
                        let ref mut fresh46 = (*item).ptr;
                        *fresh46 = pvalue;
                        let ref mut fresh47 = (*item).freefn;
                        *fresh47 = freefn;
                    };
                    (*item).size = size;
                }
                _ => {}
            }
        }
        return item;
    }
}

#[no_mangle]
pub extern "C" fn binn_set_string(
    mut item: *mut binn,
    mut str: *mut i8,
    mut pfree: binn_mem_free,
) -> i32 {
    unsafe {
        if item.is_null() || str.is_null() {
            return 0;
        }
        if pfree == ::std::mem::transmute::<libc::intptr_t, binn_mem_free>(-1i32 as libc::intptr_t)
        {
            let ref mut fresh49 = (*item).ptr;
            *fresh49 = binn_memdup(
                str as *mut libc::c_void,
                (strlen(str)).wrapping_add(1) as i32,
            );
            if ((*item).ptr).is_null() {
                return 0;
            }
            let ref mut fresh50 = (*item).freefn;
            *fresh50 = free_fn;
        } else {
            let ref mut fresh51 = (*item).ptr;
            *fresh51 = str as *mut libc::c_void;
            let ref mut fresh52 = (*item).freefn;
            *fresh52 = pfree;
        };
        (*item).type_0 = 0xa0;
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn binn_set_blob(
    mut item: *mut binn,
    mut ptr: *mut libc::c_void,
    mut size: i32,
    mut pfree: binn_mem_free,
) -> i32 {
    unsafe {
        if item.is_null() || ptr.is_null() {
            return 0;
        }
        if pfree == ::std::mem::transmute::<libc::intptr_t, binn_mem_free>(-1i32 as libc::intptr_t)
        {
            let ref mut fresh53 = (*item).ptr;
            *fresh53 = binn_memdup(ptr, size);
            if ((*item).ptr).is_null() {
                return 0;
            }
            let ref mut fresh54 = (*item).freefn;
            *fresh54 = free_fn;
        } else {
            let ref mut fresh55 = (*item).ptr;
            *fresh55 = ptr;
            let ref mut fresh56 = (*item).freefn;
            *fresh56 = pfree;
        };
        (*item).type_0 = 0xc0;
        (*item).size = size;
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn atoi64(mut str: *mut i8) -> i64 {
    unsafe {
        let mut retval: i64 = 0;
        let mut is_negative = 0;
        if *str as i32 == '-' as i32 {
            is_negative = 1;
            str = str.offset(1);
        }
        retval = 0;
        while *str != 0 {
            retval = 10 * retval + (*str as i32 - '0' as i32) as i64;
            str = str.offset(1);
        }
        if is_negative != 0 {
            retval *= -1 as i64;
        }
        return retval;
    }
}

extern "C" fn is_integer(mut p: *mut i8) -> i32 {
    unsafe {
        let mut retval: i32 = 0;
        if p.is_null() {
            return 0;
        }
        if *p as i32 == '-' as i32 {
            p = p.offset(1);
        }
        if *p as i32 == 0 {
            return 0;
        }
        retval = 1;
        while *p != 0 {
            if (*p as i32) < '0' as i32 || *p as i32 > '9' as i32 {
                retval = 0;
            }
            p = p.offset(1);
        }
        return retval;
    }
}

extern "C" fn is_float(mut p: *mut i8) -> i32 {
    unsafe {
        let mut retval: i32 = 0;
        let mut number_found = 0;
        if p.is_null() {
            return 0;
        }
        if *p as i32 == '-' as i32 {
            p = p.offset(1);
        }
        if *p as i32 == 0 {
            return 0;
        }
        retval = 1;
        while *p != 0 {
            if *p as i32 == '.' as i32 || *p as i32 == ',' as i32 {
                if number_found == 0 {
                    retval = 0;
                }
            } else if *p as i32 >= '0' as i32 && *p as i32 <= '9' as i32 {
                number_found = 1;
            } else {
                return 0;
            }
            p = p.offset(1);
        }
        return retval;
    }
}

extern "C" fn is_bool_str(mut str: *mut i8, mut pbool: *mut i32) -> i32 {
    unsafe {
        let mut vint: i64 = 0;
        let mut vdouble: f64 = 0.;
        if str.is_null() || pbool.is_null() {
            return 0;
        }
        if !(strcasecmp(str, b"true\0" as *const u8 as *const i8) == 0) {
            if !(strcasecmp(str, b"yes\0" as *const u8 as *const i8) == 0) {
                if !(strcasecmp(str, b"on\0" as *const u8 as *const i8) == 0) {
                    if !(strcasecmp(str, b"false\0" as *const u8 as *const i8) == 0) {
                        if !(strcasecmp(str, b"no\0" as *const u8 as *const i8) == 0) {
                            if !(strcasecmp(str, b"off\0" as *const u8 as *const i8) == 0) {
                                if is_integer(str) != 0 {
                                    vint = atoi64(str);
                                    *pbool = if vint != 0 { 1 } else { 0 };
                                    return 1;
                                } else {
                                    if is_float(str) != 0 {
                                        vdouble = atof(str);
                                        *pbool = if vdouble != 0 as f64 { 1 } else { 0 };
                                        return 1;
                                    }
                                }
                                return 0;
                            }
                        }
                    }
                    *pbool = 0;
                    return 1;
                }
            }
        }
        *pbool = 1;
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn binn_get_int32(mut value: *mut binn, mut pint: *mut i32) -> i32 {
    unsafe {
        if value.is_null() || pint.is_null() {
            return 0;
        }
        if type_family((*value).type_0) == 0xf2 {
            return copy_int_value(
                (*value).ptr,
                pint as *mut libc::c_void,
                (*value).type_0,
                0x61,
            );
        }
        match (*value).type_0 {
            98 => {
                if (*value).c2rust_unnamed.vfloat < (-2147483647i32 - 1i32) as libc::c_float
                    || (*value).c2rust_unnamed.vfloat > 2147483647 as libc::c_float
                {
                    return 0;
                }
                *pint = if (*value).c2rust_unnamed.vfloat as f64 >= 0.0f64 {
                    ((*value).c2rust_unnamed.vfloat as f64 + 0.5f64) as i32
                } else if (*value).c2rust_unnamed.vfloat as f64
                    - (*value).c2rust_unnamed.vfloat as i32 as f64
                    <= -0.5f64
                {
                    (*value).c2rust_unnamed.vfloat as i32
                } else {
                    ((*value).c2rust_unnamed.vfloat as f64 - 0.5f64) as i32
                };
            }
            130 => {
                if (*value).c2rust_unnamed.vdouble < (-2147483647i32 - 1i32) as f64
                    || (*value).c2rust_unnamed.vdouble > 2147483647 as f64
                {
                    return 0;
                }
                *pint = if (*value).c2rust_unnamed.vdouble >= 0.0f64 {
                    ((*value).c2rust_unnamed.vdouble + 0.5f64) as i32
                } else if (*value).c2rust_unnamed.vdouble
                    - (*value).c2rust_unnamed.vdouble as i32 as f64
                    <= -0.5f64
                {
                    (*value).c2rust_unnamed.vdouble as i32
                } else {
                    ((*value).c2rust_unnamed.vdouble - 0.5f64) as i32
                };
            }
            160 => {
                if is_integer((*value).ptr as *mut i8) != 0 {
                    *pint = atoi((*value).ptr as *mut i8);
                } else if is_float((*value).ptr as *mut i8) != 0 {
                    *pint = if atof((*value).ptr as *mut i8) >= 0.0f64 {
                        (atof((*value).ptr as *mut i8) + 0.5f64) as i32
                    } else if atof((*value).ptr as *mut i8)
                        - atof((*value).ptr as *mut i8) as i32 as f64
                        <= -0.5f64
                    {
                        atof((*value).ptr as *mut i8) as i32
                    } else {
                        (atof((*value).ptr as *mut i8) - 0.5f64) as i32
                    };
                } else {
                    return 0;
                }
            }
            524385 => {
                *pint = (*value).c2rust_unnamed.vbool;
            }
            _ => return 0,
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn binn_get_int64(mut value: *mut binn, mut pint: *mut i64) -> i32 {
    unsafe {
        if value.is_null() || pint.is_null() {
            return 0;
        }
        if type_family((*value).type_0) == 0xf2 {
            return copy_int_value(
                (*value).ptr,
                pint as *mut libc::c_void,
                (*value).type_0,
                0x81,
            );
        }
        match (*value).type_0 {
            98 => {
                if (*value).c2rust_unnamed.vfloat
                    < (-9223372036854775807i64 - 1i64) as libc::c_float
                    || (*value).c2rust_unnamed.vfloat > 9223372036854775807 as libc::c_float
                {
                    return 0;
                }
                *pint = (if (*value).c2rust_unnamed.vfloat as f64 >= 0.0f64 {
                    ((*value).c2rust_unnamed.vfloat as f64 + 0.5f64) as i32
                } else if (*value).c2rust_unnamed.vfloat as f64
                    - (*value).c2rust_unnamed.vfloat as i32 as f64
                    <= -0.5f64
                {
                    (*value).c2rust_unnamed.vfloat as i32
                } else {
                    ((*value).c2rust_unnamed.vfloat as f64 - 0.5f64) as i32
                }) as i64;
            }
            130 => {
                if (*value).c2rust_unnamed.vdouble < (-9223372036854775807i64 - 1i64) as f64
                    || (*value).c2rust_unnamed.vdouble > 9223372036854775807 as f64
                {
                    return 0;
                }
                *pint = (if (*value).c2rust_unnamed.vdouble >= 0.0f64 {
                    ((*value).c2rust_unnamed.vdouble + 0.5f64) as i32
                } else if (*value).c2rust_unnamed.vdouble
                    - (*value).c2rust_unnamed.vdouble as i32 as f64
                    <= -0.5f64
                {
                    (*value).c2rust_unnamed.vdouble as i32
                } else {
                    ((*value).c2rust_unnamed.vdouble - 0.5f64) as i32
                }) as i64;
            }
            160 => {
                if is_integer((*value).ptr as *mut i8) != 0 {
                    *pint = atoi64((*value).ptr as *mut i8);
                } else if is_float((*value).ptr as *mut i8) != 0 {
                    *pint = (if atof((*value).ptr as *mut i8) >= 0.0f64 {
                        (atof((*value).ptr as *mut i8) + 0.5f64) as i32
                    } else if atof((*value).ptr as *mut i8)
                        - atof((*value).ptr as *mut i8) as i32 as f64
                        <= -0.5f64
                    {
                        atof((*value).ptr as *mut i8) as i32
                    } else {
                        (atof((*value).ptr as *mut i8) - 0.5f64) as i32
                    }) as i64;
                } else {
                    return 0;
                }
            }
            524385 => {
                *pint = (*value).c2rust_unnamed.vbool as i64;
            }
            _ => return 0,
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn binn_get_double(mut value: *mut binn, mut pfloat: *mut f64) -> i32 {
    unsafe {
        let mut vint: i64 = 0;
        if value.is_null() || pfloat.is_null() {
            return 0;
        }
        if type_family((*value).type_0) == 0xf2 {
            if copy_int_value(
                (*value).ptr,
                &mut vint as *mut i64 as *mut libc::c_void,
                (*value).type_0,
                0x81,
            ) == 0
            {
                return 0;
            }
            *pfloat = vint as f64;
            return 1;
        }
        match (*value).type_0 {
            98 => {
                *pfloat = (*value).c2rust_unnamed.vfloat as f64;
            }
            130 => {
                *pfloat = (*value).c2rust_unnamed.vdouble;
            }
            160 => {
                if is_integer((*value).ptr as *mut i8) != 0 {
                    *pfloat = atoi64((*value).ptr as *mut i8) as f64;
                } else if is_float((*value).ptr as *mut i8) != 0 {
                    *pfloat = atof((*value).ptr as *mut i8);
                } else {
                    return 0;
                }
            }
            524385 => {
                *pfloat = (*value).c2rust_unnamed.vbool as f64;
            }
            _ => return 0,
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn binn_get_bool(mut value: *mut binn, mut pbool: *mut i32) -> i32 {
    unsafe {
        let mut vint: i64 = 0;
        if value.is_null() || pbool.is_null() {
            return 0;
        }
        if type_family((*value).type_0) == 0xf2 {
            if copy_int_value(
                (*value).ptr,
                &mut vint as *mut i64 as *mut libc::c_void,
                (*value).type_0,
                0x81,
            ) == 0
            {
                return 0;
            }
            *pbool = if vint != 0 { 1 } else { 0 };
            return 1;
        }
        match (*value).type_0 {
            524385 => {
                *pbool = (*value).c2rust_unnamed.vbool;
            }
            98 => {
                *pbool = if (*value).c2rust_unnamed.vfloat != 0 as libc::c_float {
                    1
                } else {
                    0
                };
            }
            130 => {
                *pbool = if (*value).c2rust_unnamed.vdouble != 0 as f64 {
                    1
                } else {
                    0
                };
            }
            160 => return is_bool_str((*value).ptr as *mut i8, pbool),
            _ => return 0,
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn binn_get_str(mut value: *mut binn) -> *mut i8 {
    unsafe {
        let mut current_block: u64;
        let mut vint: i64 = 0;
        let mut buf: [i8; 128] = [0; 128];
        if value.is_null() {
            return 0 as *mut i8;
        }
        if type_family((*value).type_0) == 0xf2 {
            if copy_int_value(
                (*value).ptr,
                &mut vint as *mut i64 as *mut libc::c_void,
                (*value).type_0,
                0x81,
            ) == 0
            {
                return 0 as *mut i8;
            }
            snprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 128]>() as u64,
                b"%lli\0" as *const u8 as *const i8,
                vint,
            );
        } else {
            match (*value).type_0 {
                98 => {
                    (*value).c2rust_unnamed.vdouble = (*value).c2rust_unnamed.vfloat as f64;
                    current_block = 18111289606297366774;
                }
                130 => {
                    current_block = 18111289606297366774;
                }
                160 => return (*value).ptr as *mut i8,
                524385 => {
                    if (*value).c2rust_unnamed.vbool != 0 {
                        strcpy(buf.as_mut_ptr(), b"true\0" as *const u8 as *const i8);
                    } else {
                        strcpy(buf.as_mut_ptr(), b"false\0" as *const u8 as *const i8);
                    }
                    current_block = 8192695711153237833;
                }
                _ => return 0 as *mut i8,
            }
            match current_block {
                8192695711153237833 => {}
                _ => {
                    snprintf(
                        buf.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 128]>() as u64,
                        b"%g\0" as *const u8 as *const i8,
                        (*value).c2rust_unnamed.vdouble,
                    );
                }
            }
        }
        let ref mut fresh57 = (*value).ptr;
        *fresh57 = strdup(buf.as_mut_ptr()) as *mut libc::c_void;
        if ((*value).ptr).is_null() {
            return 0 as *mut i8;
        }
        let ref mut fresh58 = (*value).freefn;
        *fresh58 = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
        (*value).type_0 = 0xa0;
        return (*value).ptr as *mut i8;
    }
}

#[no_mangle]
pub extern "C" fn binn_is_container(mut item: *mut binn) -> i32 {
    unsafe {
        if item.is_null() {
            return 0;
        }
        match (*item).type_0 {
            224 | 225 | 226 => return 1,
            _ => return 0,
        };
    }
}
