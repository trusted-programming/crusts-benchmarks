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

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fwrite(_: *const libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
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
fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let dimx: i32 = 800;
        let dimy: i32 = 800;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut fp: *mut FILE = fopen(
            (b"first.ppm\0" as *const u8).cast::<i8>(),
            (b"wb\0" as *const u8).cast::<i8>(),
        );
        fprintf(
            fp,
            (b"P6\n%d %d\n255\n\0" as *const u8).cast::<i8>(),
            dimx,
            dimy,
        );
        j = 0_i32;
        while j < dimy {
            i = 0_i32;
            while i < dimx {
                static mut color: [u8; 3] = [0; 3];
                color[0_usize] = (i % 256i32) as u8;
                color[1_usize] = (j % 256i32) as u8;
                color[2_usize] = (i * j % 256i32) as u8;
                fwrite(color.as_mut_ptr() as *const libc::c_void, 1, 3, fp);
                i = i.wrapping_add(1);
                i;
            }
            j = j.wrapping_add(1);
            j;
        }
        fclose(fp);
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
