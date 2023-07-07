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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fwrite(_: *const libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn fabs(_: f64) -> f64;
    fn fmod(_: f64, _: f64) -> f64;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
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
pub extern "C" fn hue_to_rgb(mut hue: f64, mut sat: f64, mut p: *mut u8) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut x: f64 = 0.;
        let mut c: i32 = (255_f64 * sat) as i32;
        hue /= 60_f64;
        x = (1_f64 - fabs(fmod(hue, 2_f64) - 1_f64)) * 255_f64;
        match hue as i32 {
            0_i32 => {
                *p.offset(0_isize) = c as u8;
                *p.offset(1_isize) = x as u8;
                *p.offset(2_isize) = 0;
            }
            1_i32 => {
                *p.offset(0_isize) = x as u8;
                *p.offset(1_isize) = c as u8;
                *p.offset(2_isize) = 0;
            }
            2_i32 => {
                *p.offset(0_isize) = 0;
                *p.offset(1_isize) = c as u8;
                *p.offset(2_isize) = x as u8;
            }
            3_i32 => {
                *p.offset(0_isize) = 0;
                *p.offset(1_isize) = x as u8;
                *p.offset(2_isize) = c as u8;
            }
            4_i32 => {
                *p.offset(0_isize) = x as u8;
                *p.offset(1_isize) = 0;
                *p.offset(2_isize) = c as u8;
            }
            5_i32 => {
                *p.offset(0_isize) = c as u8;
                *p.offset(1_isize) = 0;
                *p.offset(2_isize) = x as u8;
            }
            _ => {}
        }
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let size: i32 = 512;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut colors: *mut u8 = malloc((size.wrapping_mul(3i32)) as u64).cast::<u8>();
        let mut pix: *mut u8 = malloc((size * size.wrapping_mul(3i32)) as u64).cast::<u8>();
        let mut p: *mut u8 = std::ptr::null_mut::<u8>();
        let mut fp: *mut FILE = std::ptr::null_mut::<FILE>();
        i = 0_i32;
        while i < size {
            hue_to_rgb(
                f64::from(i) * 240.0f64 / f64::from(size),
                f64::from(i) * 1.0f64 / f64::from(size),
                colors.offset((3 * i) as isize),
            );
            i = i.wrapping_add(1);
            i;
        }
        i = 0_i32;
        p = pix;
        while i < size {
            j = 0_i32;
            while j < size {
                memcpy(
                    p.cast::<libc::c_void>(),
                    colors.offset(((i ^ j) * 3i32) as isize) as *const libc::c_void,
                    3,
                );
                j = j.wrapping_add(1);
                j;
                p = p.offset(3_isize);
            }
            i = i.wrapping_add(1);
            i;
        }
        fp = fopen(
            (b"xor.ppm\0" as *const u8).cast::<i8>(),
            (b"wb\0" as *const u8).cast::<i8>(),
        );
        fprintf(
            fp,
            (b"P6\n%d %d\n255\n\0" as *const u8).cast::<i8>(),
            size,
            size,
        );
        fwrite(
            pix as *const libc::c_void,
            (size * size.wrapping_mul(3i32)) as u64,
            1,
            fp,
        );
        fclose(fp);
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
