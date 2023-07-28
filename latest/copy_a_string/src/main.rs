#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
unsafe fn main_0() -> libc::c_int {
    let mut len: size_t = 0;
    let mut src: [libc::c_char; 6] = *::core::mem::transmute::<
        &[u8; 6],
        &mut [libc::c_char; 6],
    >(b"Hello\0");
    let mut dst1: [libc::c_char; 80] = [0; 80];
    let mut dst2: [libc::c_char; 80] = [0; 80];
    let mut dst3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ref_0: *mut libc::c_char = 0 as *mut libc::c_char;
    strcpy(dst1.as_mut_ptr(), src.as_mut_ptr());
    len = strlen(src.as_mut_ptr());
    if len >= ::core::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong {
        fputs(
            b"The buffer is too small!\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    memcpy(
        dst2.as_mut_ptr() as *mut libc::c_void,
        src.as_mut_ptr() as *const libc::c_void,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    dst3 = strdup(src.as_mut_ptr());
    if dst3.is_null() {
        perror(b"strdup\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    ref_0 = src.as_mut_ptr();
    memset(
        src.as_mut_ptr() as *mut libc::c_void,
        '-' as i32,
        5 as libc::c_int as libc::c_ulong,
    );
    printf(b" src: %s\n\0" as *const u8 as *const libc::c_char, src.as_mut_ptr());
    printf(b"dst1: %s\n\0" as *const u8 as *const libc::c_char, dst1.as_mut_ptr());
    printf(b"dst2: %s\n\0" as *const u8 as *const libc::c_char, dst2.as_mut_ptr());
    printf(b"dst3: %s\n\0" as *const u8 as *const libc::c_char, dst3);
    printf(b" ref: %s\n\0" as *const u8 as *const libc::c_char, ref_0);
    free(dst3 as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
