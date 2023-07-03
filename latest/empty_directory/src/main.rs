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
    pub type __dirstream;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn opendir(__name: *const i8) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> i32;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [i8; 256],
}
pub type DIR = __dirstream;
#[no_mangle]
pub extern "C" fn dir_empty(mut path: *const i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut ent: *mut dirent = std::ptr::null_mut::<dirent>();
        let mut ret: i32 = 1;
        let mut d: *mut DIR = opendir(path);
        if d.is_null() {
            fprintf(stderr, (b"%s: \0" as *const u8).cast::<i8>(), path);
            perror((b"\0" as *const u8).cast::<i8>());
            return -1_i32;
        }
        loop {
            ent = readdir(d);
            if ent.is_null() {
                break;
            }
            if strcmp(
                ((*ent).d_name).as_mut_ptr(),
                (b".\0" as *const u8).cast::<i8>(),
            ) == 0_i32
                || strcmp(
                    ((*ent).d_name).as_mut_ptr(),
                    (b"..\0" as *const u8).cast::<i8>(),
                ) == 0_i32
            {
                continue;
            }
            ret = 0_i32;
            break;
        }
        closedir(d);
        ret
    }
}

fn main_0(mut c: i32, mut v: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut ret: i32 = 0;
        let mut i: i32 = 0;
        if c < 2_i32 {
            return -1_i32;
        }
        i = 1_i32;
        while i < c {
            ret = dir_empty(*v.offset(i as isize));
            if ret >= 0_i32 {
                if ret != 0_i32 {
                    println!(
                        "{}: \0empty",
                        build_str_from_raw_ptr((*v.offset(i as isize)).cast::<u8>())
                    )
                } else {
                    println!(
                        "{}: not \0empty",
                        build_str_from_raw_ptr((*v.offset(i as isize)).cast::<u8>())
                    )
                };
            }
            i = i.wrapping_add(1);
            i;
        }
        0_i32
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr()));
}
