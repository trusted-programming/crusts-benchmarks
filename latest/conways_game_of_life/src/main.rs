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
    fn rand() -> i32;
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
pub extern "C" fn show(mut u: *mut libc::c_void, mut w: i32, mut h: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let vla = w as usize;
        let mut univ: *mut i32 = u.cast::<i32>();
        print!("\x1B[H");
        let mut y: i32 = 0;
        while y < h {
            let mut x: i32 = 0;
            while x < w {
                if *univ.offset(y as isize * vla as isize).offset(x as isize) != 0_i32 {
                    print!("\x1B[07m  \x1B[m")
                } else {
                    print!("  ")
                };
                x += 1_i32;
                x;
            }
            print!("\x1B[E");
            y += 1_i32;
            y;
        }
        fflush(stdout);
    }
}

#[no_mangle]
pub extern "C" fn evolve(mut u: *mut libc::c_void, mut w: i32, mut h: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let vla = w as usize;
        let mut univ: *mut u32 = u.cast::<u32>();
        let vla_0 = h as usize;
        let vla_1 = w as usize;
        let mut new: Vec<u32> = ::std::vec::from_elem(0, vla_0 * vla_1);
        let mut y: i32 = 0;
        while y < h {
            let mut x: i32 = 0;
            while x < w {
                let mut n: i32 = 0;
                let mut y1: i32 = y - 1;
                while y1 <= y + 1_i32 {
                    let mut x1: i32 = x - 1;
                    while x1 <= x + 1_i32 {
                        if *univ
                            .offset(((y1 + h) % h) as isize * vla as isize)
                            .offset(((x1 + w) % w) as isize)
                            != 0
                        {
                            n += 1_i32;
                            n;
                        }
                        x1 += 1_i32;
                        x1;
                    }
                    y1 += 1_i32;
                    y1;
                }
                if *univ.offset(y as isize * vla as isize).offset(x as isize) != 0 {
                    n -= 1_i32;
                    n;
                };
                *new.as_mut_ptr()
                    .offset(y as isize * vla_1 as isize)
                    .offset(x as isize) = u32::from(n == 3_i32
                    || n == 2_i32 && *univ.offset(y as isize * vla as isize).offset(x as isize) != 0);
                x += 1_i32;
                x;
            }
            y += 1_i32;
            y;
        }
        let mut y_0: i32 = 0;
        while y_0 < h {
            let mut x_0: i32 = 0;
            while x_0 < w {
                *univ
                    .offset(y_0 as isize * vla as isize)
                    .offset(x_0 as isize) = *new
                    .as_mut_ptr()
                    .offset(y_0 as isize * vla_1 as isize)
                    .offset(x_0 as isize);
                x_0 += 1_i32;
                x_0;
            }
            y_0 += 1_i32;
            y_0;
        }
    }
}

#[no_mangle]
pub extern "C" fn game(mut w: i32, mut h: i32) {
    let vla = h as usize;
    let vla_0 = w as usize;
    let mut univ: Vec<u32> = ::std::vec::from_elem(0, vla * vla_0);
    let mut x: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        while x < w {
            let mut y: i32 = 0;
            while y < h {
                *univ
                    .as_mut_ptr()
                    .offset(y as isize * vla_0 as isize)
                    .offset(x as isize) = (if rand() < 2_147_483_647_i32 / 10i32 { 1_i32 } else { 0_i32 }) as u32;
                y += 1_i32;
                y;
            }
            x += 1_i32;
            x;
        }
        loop {
            show(univ.as_mut_ptr().cast::<libc::c_void>(), w, h);
            evolve(univ.as_mut_ptr().cast::<libc::c_void>(), w, h);
            usleep(200000);
        }
    }
}

fn main_0(mut c: i32, mut v: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut w: i32 = 0;
        let mut h: i32 = 0;
        if c > 1_i32 {
            w = atoi(*v.offset(1_isize));
        }
        if c > 2_i32 {
            h = atoi(*v.offset(2_isize));
        }
        if w <= 0_i32 {
            w = 30_i32;
        }
        if h <= 0_i32 {
            h = 30_i32;
        }
        game(w, h);
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
