#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.add(str_size) != 0 {
            str_size = str_size.wrapping_add(1);
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut stdin: *mut FILE;
    fn fgetc(__stream: *mut FILE) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
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
#[no_mangle]
pub extern "C" fn get_line(mut fp: *mut FILE) -> *mut i8 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut len: i32 = 0;
        let mut got: i32 = 0;
        let mut c: i32 = 0;
        let mut buf: *mut i8 = std::ptr::null_mut::<i8>();
        loop {
            c = fgetc(fp);
            if c == -1_i32 {
                break;
            }
            if got + 1_i32 >= len {
                len *= 2_i32;
                if len < 4_i32 {
                    len = 4_i32;
                }
                buf = realloc(buf.cast::<libc::c_void>(), len as u64).cast::<i8>();
            }
            let fresh0 = got;
            got = got.wrapping_add(1);
            *buf.offset(fresh0 as isize) = c as i8;
            if c == '\n' as i32 {
                break;
            }
        }
        if c == -1_i32 && got == 0_i32 {
            return std::ptr::null_mut::<i8>();
        }
        let fresh1 = got;
        got = got.wrapping_add(1);
        *buf.offset(fresh1 as isize) = '\0' as i8;
        buf
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut s: *mut i8 = std::ptr::null_mut::<i8>();
        loop {
            s = get_line(stdin);
            if s.is_null() {
                break;
            }
            print!("{}", build_str_from_raw_ptr(s.cast::<u8>()));
            free(s.cast::<libc::c_void>());
        }
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
