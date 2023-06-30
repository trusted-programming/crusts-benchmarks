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
    fn fflush(__stream: *mut FILE) -> i32;
    fn atoi(__nptr: *const i8) -> i32;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn usleep(__useconds: u32) -> i32;
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
#[no_mangle]
pub static mut w: i32 = 0_i32;
#[no_mangle]
pub static mut h: i32 = 0_i32;
#[no_mangle]
pub static mut pix: *mut u8 = 0 as *const u8 as *mut u8;
#[no_mangle]
pub extern "C" fn refresh(mut _x: i32, mut _y: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    print!("\x1B[H");
    k = 0_i32;
    i = k;
    unsafe {
        while i < h {
            j = 0_i32;
            while j < w {
                print!(
                    "{}",
                    if i32::from(*pix.offset(k as isize)) != 0_i32 {
                        '#' as i32
                    } else {
                        ' ' as i32
                    }
                );
                j = j.wrapping_add(1);
                j;
                k = k.wrapping_add(1);
                k;
            }
            print!("{}", '\n' as i32);
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn walk() {
    let mut dx: i32 = 0;
    let mut dy: i32 = 1;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    unsafe {
        let mut x: i32 = w / 2;
        let mut y: i32 = h / 2;
        pix = calloc(1, (w * h) as u64).cast::<u8>();
        print!("\x1B[H\x1B[J");
        loop {
            i = y * w + x;
            if *pix.offset(i as isize) != 0 {
                k = dx;
                dx = -dy;
                dy = k;
            } else {
                k = dy;
                dy = -dx;
                dx = k;
            };
            *pix.offset(i as isize) = u8::from(*pix.offset(i as isize) == 0);
            if i32::from(*pix.offset(i as isize)) != 0_i32 {
                print!("\x1B[{};{}H{}", y + 1_i32, x + 1_i32, '#' as i32)
            } else {
                print!("\x1B[{};{}H{}", y + 1_i32, x + 1_i32, ' ' as i32)
            };
            x = x.wrapping_add(dx);
            y = y.wrapping_add(dy);
            k = 0_i32;
            if x < 0_i32 {
                memmove(
                    pix.offset(1_isize).cast::<libc::c_void>(),
                    pix as *const libc::c_void,
                    (w * h - 1i32) as u64,
                );
                i = 0_i32;
                while i < w * h {
                    *pix.offset(i as isize) = 0;
                    i = i.wrapping_add(w);
                }
                x = x.wrapping_add(1);
                x;
                k = 1_i32;
            } else if x >= w {
                memmove(
                    pix.cast::<libc::c_void>(),
                    pix.offset(1_isize) as *const libc::c_void,
                    (w * h - 1i32) as u64,
                );
                i = w.wrapping_sub(1);
                while i < w * h {
                    *pix.offset(i as isize) = 0;
                    i = i.wrapping_add(w);
                }
                x = x.wrapping_sub(1);
                x;
                k = 1_i32;
            }
            if y >= h {
                memmove(
                    pix.cast::<libc::c_void>(),
                    pix.offset(w as isize) as *const libc::c_void,
                    (w * (h - 1i32)) as u64,
                );
                memset(
                    pix.offset((w * (h - 1i32)) as isize).cast::<libc::c_void>(),
                    0,
                    w as u64,
                );
                y = y.wrapping_sub(1);
                y;
                k = 1_i32;
            } else if y < 0_i32 {
                memmove(
                    pix.offset(w as isize).cast::<libc::c_void>(),
                    pix as *const libc::c_void,
                    (w * (h - 1i32)) as u64,
                );
                memset(pix.cast::<libc::c_void>(), 0, w as u64);
                y = y.wrapping_add(1);
                y;
                k = 1_i32;
            }
            if k != 0_i32 {
                refresh(x, y);
            }
            print!("\x1B[{};{}H\x1B[31m@\x1B[m", y + 1_i32, x + 1_i32);
            fflush(stdout);
            usleep(10000);
        }
    }
}

fn main_0(mut c: i32, mut v: *mut *mut i8) -> i32 {
    unsafe {
        if c > 1_i32 {
            w = atoi(*v.offset(1_isize));
        }
        if c > 2_i32 {
            h = atoi(*v.offset(2_isize));
        }
        if w < 40_i32 {
            w = 40_i32;
        }
        if h < 25_i32 {
            h = 25_i32;
        }
        walk();
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
