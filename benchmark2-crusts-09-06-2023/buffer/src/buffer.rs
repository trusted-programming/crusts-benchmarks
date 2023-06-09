use libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncat(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn vsnprintf(_: *mut i8, _: u64, _: *const i8, _: ::std::ffi::VaList) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __ctype_b_loc() -> *mut *const u16;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub const _ISalnum: u32 = 8;
pub const _ISpunct: u32 = 4;
pub const _IScntrl: u32 = 2;
pub const _ISblank: u32 = 1;
pub const _ISgraph: u32 = 32768;
pub const _ISprint: u32 = 16384;
pub const _ISspace: u32 = 8192;
pub const _ISxdigit: u32 = 4096;
pub const _ISdigit: u32 = 2048;
pub const _ISalpha: u32 = 1024;
pub const _ISlower: u32 = 512;
pub const _ISupper: u32 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer_t {
    pub len: u64,
    pub alloc: *mut i8,
    pub data: *mut i8,
}
#[no_mangle]
pub extern "C" fn buffer_new() -> *mut buffer_t {
    return buffer_new_with_size(64);
}

#[no_mangle]
pub extern "C" fn buffer_new_with_size(mut n: u64) -> *mut buffer_t {
    unsafe {
        let mut self_0 = malloc(::std::mem::size_of::<buffer_t>() as u64) as *mut buffer_t;
        if self_0.is_null() {
            return 0 as *mut buffer_t;
        };
        (*self_0).len = n;
        let ref mut fresh0 = (*self_0).alloc;
        *fresh0 = calloc(n.wrapping_add(1), 1) as *mut i8;
        let ref mut fresh1 = (*self_0).data;
        *fresh1 = *fresh0;
        return self_0;
    }
}

#[no_mangle]
pub extern "C" fn buffer_new_with_string(mut str: *mut i8) -> *mut buffer_t {
    unsafe {
        return buffer_new_with_string_length(str, strlen(str));
    }
}

#[no_mangle]
pub extern "C" fn buffer_new_with_string_length(mut str: *mut i8, mut len: u64) -> *mut buffer_t {
    unsafe {
        let mut self_0 = malloc(::std::mem::size_of::<buffer_t>() as u64) as *mut buffer_t;
        if self_0.is_null() {
            return 0 as *mut buffer_t;
        };
        (*self_0).len = len;
        let ref mut fresh2 = (*self_0).alloc;
        *fresh2 = str;
        let ref mut fresh3 = (*self_0).data;
        *fresh3 = *fresh2;
        return self_0;
    }
}

#[no_mangle]
pub extern "C" fn buffer_new_with_copy(mut str: *mut i8) -> *mut buffer_t {
    unsafe {
        let mut len = strlen(str);
        let mut self_0 = buffer_new_with_size(len);
        if self_0.is_null() {
            return 0 as *mut buffer_t;
        }
        memcpy(
            (*self_0).alloc as *mut libc::c_void,
            str as *const libc::c_void,
            len,
        );
        let ref mut fresh4 = (*self_0).data;
        *fresh4 = (*self_0).alloc;
        return self_0;
    }
}

#[no_mangle]
pub extern "C" fn buffer_compact(mut self_0: *mut buffer_t) -> i64 {
    unsafe {
        let mut len = buffer_length(self_0);
        let mut rem = ((*self_0).len).wrapping_sub(len);
        let mut buf = calloc(len.wrapping_add(1), 1) as *mut i8;
        if buf.is_null() {
            return -1 as i64;
        }
        memcpy(
            buf as *mut libc::c_void,
            (*self_0).data as *const libc::c_void,
            len,
        );
        free((*self_0).alloc as *mut libc::c_void);
        (*self_0).len = len;
        let ref mut fresh5 = (*self_0).alloc;
        *fresh5 = buf;
        let ref mut fresh6 = (*self_0).data;
        *fresh6 = *fresh5;
        return rem as i64;
    }
}

#[no_mangle]
pub extern "C" fn buffer_free(mut self_0: *mut buffer_t) {
    unsafe {
        free((*self_0).alloc as *mut libc::c_void);
        free(self_0 as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn buffer_size(mut self_0: *mut buffer_t) -> u64 {
    unsafe {
        return (*self_0).len;
    }
}

#[no_mangle]
pub extern "C" fn buffer_length(mut self_0: *mut buffer_t) -> u64 {
    unsafe {
        return strlen((*self_0).data);
    }
}

#[no_mangle]
pub extern "C" fn buffer_resize(mut self_0: *mut buffer_t, mut n: u64) -> i32 {
    unsafe {
        n = n.wrapping_add((1024 - 1i32) as u64) & !(1024 - 1i32) as u64;
        (*self_0).len = n;
        let ref mut fresh7 = (*self_0).data;
        *fresh7 = realloc((*self_0).alloc as *mut libc::c_void, n.wrapping_add(1)) as *mut i8;
        let ref mut fresh8 = (*self_0).alloc;
        *fresh8 = *fresh7;
        if ((*self_0).alloc).is_null() {
            return -1;
        };
        *((*self_0).alloc).offset(n as isize) = '\0' as i8;
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn buffer_append(mut self_0: *mut buffer_t, mut str: *const i8) -> i32 {
    unsafe {
        return buffer_append_n(self_0, str, strlen(str));
    }
}

#[no_mangle]
pub extern "C" fn buffer_append_n(
    mut self_0: *mut buffer_t,
    mut str: *const i8,
    mut len: u64,
) -> i32 {
    unsafe {
        let mut prev = strlen((*self_0).data);
        let mut needed = len.wrapping_add(prev);
        if (*self_0).len > needed {
            strncat((*self_0).data, str, len);
            return 0;
        }
        let mut ret = buffer_resize(self_0, needed);
        if -1 == ret {
            return -1;
        }
        strncat((*self_0).data, str, len);
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn buffer_prepend(mut self_0: *mut buffer_t, mut str: *mut i8) -> i32 {
    unsafe {
        let mut ret: i32 = 0;
        let mut len = strlen(str);
        let mut prev = strlen((*self_0).data);
        let mut needed = len.wrapping_add(prev);
        if !((*self_0).len > needed) {
            ret = buffer_resize(self_0, needed);
            if -1 == ret {
                return -1;
            }
        }
        memmove(
            ((*self_0).data).offset(len as isize) as *mut libc::c_void,
            (*self_0).data as *const libc::c_void,
            len.wrapping_add(1),
        );
        memcpy(
            (*self_0).data as *mut libc::c_void,
            str as *const libc::c_void,
            len,
        );
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn buffer_slice(
    mut buf: *mut buffer_t,
    mut from: u64,
    mut to: i64,
) -> *mut buffer_t {
    unsafe {
        let mut len = strlen((*buf).data);
        if (to as u64) < from {
            return 0 as *mut buffer_t;
        }
        if to < 0 {
            to = len.wrapping_sub(!to as u64) as i64;
        }
        if to as u64 > len {
            to = len as i64;
        }
        let mut n = (to as u64).wrapping_sub(from);
        let mut self_0 = buffer_new_with_size(n);
        memcpy(
            (*self_0).data as *mut libc::c_void,
            ((*buf).data).offset(from as isize) as *const libc::c_void,
            n,
        );
        return self_0;
    }
}

#[no_mangle]
pub extern "C" fn buffer_equals(mut self_0: *mut buffer_t, mut other: *mut buffer_t) -> i32 {
    unsafe {
        return (0 == strcmp((*self_0).data, (*other).data)) as i32;
    }
}

#[no_mangle]
pub extern "C" fn buffer_indexof(mut self_0: *mut buffer_t, mut str: *mut i8) -> i64 {
    unsafe {
        let mut sub = strstr((*self_0).data, str);
        if sub.is_null() {
            return -1 as i64;
        }
        return sub.offset_from((*self_0).data) as i64;
    }
}

#[no_mangle]
pub extern "C" fn buffer_trim_left(mut self_0: *mut buffer_t) {
    unsafe {
        let mut c: i32 = 0;
        loop {
            c = *(*self_0).data as i32;
            if !(c != 0 && *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISspace as i32 != 0) {
                break;
            }
            let ref mut fresh9 = (*self_0).data;
            *fresh9 = (*fresh9).offset(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn buffer_trim_right(mut self_0: *mut buffer_t) {
    unsafe {
        let mut c: i32 = 0;
        let mut i = (buffer_length(self_0)).wrapping_sub(1);
        loop {
            c = *((*self_0).data).offset(i as isize) as i32;
            if !(c != 0 && *(*__ctype_b_loc()).offset(c as isize) as i32 & _ISspace as i32 != 0) {
                break;
            }
            let fresh10 = i;
            i = i.wrapping_sub(1);
            *((*self_0).data).offset(fresh10 as isize) = 0;
        }
    }
}

#[no_mangle]
pub extern "C" fn buffer_trim(mut self_0: *mut buffer_t) {
    unsafe {
        buffer_trim_left(self_0);
        buffer_trim_right(self_0);
    }
}

#[no_mangle]
pub extern "C" fn buffer_fill(mut self_0: *mut buffer_t, mut c: i32) {
    unsafe {
        memset((*self_0).data as *mut libc::c_void, c, (*self_0).len);
    }
}

#[no_mangle]
pub extern "C" fn buffer_clear(mut self_0: *mut buffer_t) {
    unsafe {
        buffer_fill(self_0, 0);
    }
}

#[no_mangle]
pub extern "C" fn buffer_print(mut self_0: *mut buffer_t) {
    unsafe {
        let mut len = (*self_0).len;
        print!("\n ");
        let mut i = 0;
        while (i as u64) < len {
            print!(" {:02x},9999");
            if (i + 1) % 8 == 0 {
                print!("\n ");
            }
            i += 1;
        }
        print!("\n");
    }
}
