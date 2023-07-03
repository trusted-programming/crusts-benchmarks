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
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fwrite(_: *const libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn atoi(__nptr: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn cos(_: f64) -> f64;
    fn fabs(_: f64) -> f64;
    fn fmod(_: f64, _: f64) -> f64;
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
pub struct rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
#[no_mangle]
pub static mut x: i64 = 0;
#[no_mangle]
pub static mut y: i64 = 0;
#[no_mangle]
pub static mut dx: i64 = 0;
#[no_mangle]
pub static mut dy: i64 = 0;
#[no_mangle]
pub static mut scale: i64 = 0;
#[no_mangle]
pub static mut clen: i64 = 0;
#[no_mangle]
pub static mut pix: *mut *mut rgb = 0 as *const *mut rgb as *mut *mut rgb;
#[no_mangle]
pub extern "C" fn sc_up() {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut tmp: i64 = dx - dy;
        dy += dx;
        dx = tmp;
        scale *= 2;
        x *= 2;
        y *= 2;
    }
}

#[no_mangle]
pub extern "C" fn h_rgb(mut x_0: i64, mut y_0: i64) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut p: *mut rgb = &mut *(*pix.offset(y_0 as isize)).offset(x_0 as isize) as *mut rgb;
        let mut h: f64 = 6.0f64 * clen as f64 / scale as f64;
        let mut VAL: f64 = 1_f64
            - (cos(3.141592653579f64 * 64_f64 * clen as f64 / scale as f64) - 1_f64)
                / 4_f64;
        let mut c: f64 = 1_f64 * VAL;
        let mut X: f64 = c * (1_f64 - fabs(fmod(h, 2_f64) - 1_f64));
        match h as i32 {
            0_i32 => {
                (*p).r += c;
                (*p).g += X;
            }
            1_i32 => {
                (*p).r += X;
                (*p).g += c;
            }
            2_i32 => {
                (*p).g += c;
                (*p).b += X;
            }
            3_i32 => {
                (*p).g += X;
                (*p).b += c;
            }
            4_i32 => {
                (*p).r += X;
                (*p).b += c;
            }
            _ => {
                (*p).r += c;
                (*p).b += X;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn iter_string(mut str: *const i8, mut d: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut tmp: i64 = 0;
        while i32::from(*str) != '\0' as i32 {
            let fresh0 = str;
            str = str.offset(1);
            match i32::from(*fresh0) {
                88_i32 => {
                    if d != 0_i32 {
                        iter_string((b"X+YF+\0" as *const u8).cast::<i8>(), d - 1);
                    }
                }
                89_i32 => {
                    if d != 0_i32 {
                        iter_string((b"-FX-Y\0" as *const u8).cast::<i8>(), d - 1);
                    }
                }
                43_i32 => {
                    tmp = dy;
                    dy = -dx;
                    dx = tmp;
                }
                45_i32 => {
                    tmp = -dy;
                    dy = dx;
                    dx = tmp;
                }
                70_i32 => {
                    clen = clen.wrapping_add(1);
                    clen;
                    h_rgb(x / scale, y / scale);
                    x = x.wrapping_add(dx);
                    y = y.wrapping_add(dy);
                }
                _ => {}
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn dragon(mut leng: i64, mut depth: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i64 = 0;
        let mut d: i64 = leng / 3 + 1;
        let mut h: i64 = leng + 3;
        let mut w: i64 = leng + d * 3 / 2 + 2;
        let mut buf: *mut rgb = malloc(
            (::core::mem::size_of::<rgb>() as u64)
                .wrapping_mul(w as u64)
                .wrapping_mul(h as u64),
        ).cast::<rgb>();
        pix = malloc((::core::mem::size_of::<*mut rgb>() as u64).wrapping_mul(h as u64)).cast::<*mut rgb>();
        i = 0;
        while i < h {
            let fresh1 = &mut (*pix.offset(i as isize));
            *fresh1 = buf.offset((w * i) as isize);
            i = i.wrapping_add(1);
            i;
        }
        memset(
            buf.cast::<libc::c_void>(),
            0,
            (::core::mem::size_of::<rgb>() as u64)
                .wrapping_mul(w as u64)
                .wrapping_mul(h as u64),
        );
        y = d;
        x = y;
        dx = leng;
        dy = 0;
        scale = 1;
        clen = 0;
        i = 0;
        while i < i64::from(depth) {
            sc_up();
            i = i.wrapping_add(1);
            i;
        }
        iter_string((b"FX\0" as *const u8).cast::<i8>(), depth);
        let mut fpix: *mut u8 = malloc((w * h * 3i64) as u64).cast::<u8>();
        let mut maxv: f64 = f64::from(0_i32);
        let mut dbuf: *mut f64 = buf.cast::<f64>();
        i = 3 * w * h - 1;
        while i >= 0 {
            if *dbuf.offset(i as isize) > maxv {
                maxv = *dbuf.offset(i as isize);
            }
            i = i.wrapping_sub(1);
            i;
        }
        i = 3 * h * w - 1;
        while i >= 0 {
            *fpix.offset(i as isize) = (255_f64 * *dbuf.offset(i as isize) / maxv) as u8;
            i = i.wrapping_sub(1);
            i;
        }
        print!("P6\n{} {}\n255\n", w, h);
        fflush(stdout);
        fwrite(
            fpix as *const libc::c_void,
            (h * w * 3i64) as u64,
            1,
            stdout,
        );
    }
}

fn main_0(mut c: i32, mut v: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut size: i32 = 0;
        let mut depth: i32 = 0;
        depth = if c > 1_i32 {
            atoi(*v.offset(1_isize))
        } else {
            10_i32
        };
        size = 1_i32 << depth;
        fprintf(
            stderr,
            (b"size: %d depth: %d\n\0" as *const u8).cast::<i8>(),
            size,
            depth,
        );
        dragon(i64::from(size), depth * 2);
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
