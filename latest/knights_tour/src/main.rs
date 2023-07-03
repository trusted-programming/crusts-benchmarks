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
use std::io::Read;
fn rust_getchar() -> u8 {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .unwrap()
}


extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn atoi(__nptr: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn usleep(__useconds: u32) -> i32;
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
pub static mut dx: [i32; 8] = [-2_i32, -2_i32, -1_i32, 1_i32, 2_i32, 2_i32, 1_i32, -1_i32];
#[no_mangle]
pub static mut dy: [i32; 8] = [-1_i32, 1_i32, 2_i32, 2_i32, 1_i32, -1_i32, -2_i32, -2_i32];
#[no_mangle]
pub extern "C" fn init_board(mut w: i32, mut h: i32, mut a: *mut *mut u8, mut b: *mut *mut u8) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut p: i32 = w + 4;
        let mut q: i32 = h + 4;
        let fresh0 = &mut (*a.offset(0_isize));
        *fresh0 = a.offset(q as isize).cast::<u8>();
        let fresh1 = &mut (*b.offset(0_isize));
        *fresh1 = (*a.offset(0_isize)).offset(2_isize);
        i = 1_i32;
        while i < q {
            let fresh2 = &mut (*a.offset(i as isize));
            *fresh2 = (*a.offset((i - 1i32) as isize)).offset(p as isize);
            let fresh3 = &mut (*b.offset(i as isize));
            *fresh3 = (*a.offset(i as isize)).offset(2_isize);
            i = i.wrapping_add(1);
            i;
        }
        memset(
            (*a.offset(0_isize)).cast::<libc::c_void>(),
            255,
            (p * q) as u64,
        );
        i = 0_i32;
        while i < h {
            j = 0_i32;
            while j < w {
                k = 0_i32;
                while k < 8_i32 {
                    x = j + dx[k as usize];
                    y = i + dy[k as usize];
                    if i32::from(*(*b.offset((i + 2i32) as isize)).offset(j as isize)) == 255_i32 {
                        *(*b.offset((i + 2i32) as isize)).offset(j as isize) = 0;
                    }
                    let fresh4 = &mut (*(*b.offset((i + 2i32) as isize)).offset(j as isize));
                    *fresh4 = (i32::from(*fresh4) + i32::from(x >= 0_i32 && x < w && y >= 0_i32 && y < h)) as u8;
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn walk_board(
    mut w: i32,
    mut h: i32,
    mut x: i32,
    mut y: i32,
    mut b: *mut *mut u8,
) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut nx: i32 = 0;
        let mut ny: i32 = 0;
        let mut least: i32 = 0;
        let mut steps: i32 = 0;
        print!("\x1B[H\x1B[J\x1B[{};{}H\x1B[32m[]\x1B[m", y + 1_i32, 1_i32 + 2_i32 * x);
        loop {
            *(*b.offset(y as isize)).offset(x as isize) = 255;
            i = 0_i32;
            while i < 8_i32 {
                let fresh5 = &mut (*(*b.offset((y + dy[i as usize]) as isize))
                    .offset((x + dx[i as usize]) as isize));
                *fresh5 = (*fresh5).wrapping_sub(1);
                *fresh5;
                i = i.wrapping_add(1);
                i;
            }
            least = 255_i32;
            i = 0_i32;
            while i < 8_i32 {
                if i32::from(*(*b.offset((y + dy[i as usize]) as isize))
                    .offset((x + dx[i as usize]) as isize))
                    < least
                {
                    nx = x + dx[i as usize];
                    ny = y + dy[i as usize];
                    least = i32::from(*(*b.offset(ny as isize)).offset(nx as isize));
                }
                i = i.wrapping_add(1);
                i;
            }
            if least > 7_i32 {
                print!("\x1B[{}H", h + 2_i32);
                return i32::from(steps == w * h - 1i32);
            }
            let fresh6 = steps;
            steps = steps.wrapping_add(1);
            if fresh6 != 0_i32 {
                print!("\x1B[{};{}H[]", y + 1_i32, 1_i32 + 2_i32 * x);
            }
            x = nx;
            y = ny;
            print!("\x1B[{};{}H\x1B[31m[]\x1B[m", y + 1_i32, 1_i32 + 2_i32 * x);
            fflush(stdout);
            usleep(120000);
        }
    }
}

#[no_mangle]
pub extern "C" fn solve(mut w: i32, mut h: i32) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut a: *mut *mut u8 = std::ptr::null_mut::<*mut u8>();
        let mut b: *mut *mut u8 = std::ptr::null_mut::<*mut u8>();
        a = malloc((((w + 4i32) * (h + 4i32)) as u64).wrapping_add(
            (::core::mem::size_of::<*mut u8>() as u64).wrapping_mul((h + 4i32) as u64),
        )).cast::<*mut u8>();
        b = malloc(((h + 4i32) as u64).wrapping_mul(::core::mem::size_of::<*mut u8>() as u64)).cast::<*mut u8>();
        loop {
            init_board(w, h, a, b);
            if walk_board(w, h, x, y, b.offset(2_isize)) != 0_i32 {
                println!("Success!");
                return 1_i32;
            }
            x = x.wrapping_add(1);
            if x >= w {
                x = 0_i32;
                y = y.wrapping_add(1);
                y;
            }
            if y >= h {
                println!("Failed to find a solution");
                return 0_i32;
            }
            print!("Any key to try next start position");
            i32::from(rust_getchar());
        }
    }
}

fn main_0(mut c: i32, mut v: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut w: i32 = 0;
        let mut h: i32 = 0;
        if c < 2_i32 || {
            w = atoi(*v.offset(1_isize));
            w <= 0_i32
        } {
            w = 8_i32;
        }
        if c < 3_i32 || {
            h = atoi(*v.offset(2_isize));
            h <= 0_i32
        } {
            h = w;
        }
        solve(w, h);
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
