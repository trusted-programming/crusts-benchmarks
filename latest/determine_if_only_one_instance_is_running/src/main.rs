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
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
// SAFETY: machine generated unsafe code
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn perror(__s: *const i8);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn sleep(__seconds: u32) -> u32;
    fn unlink(__name: *const i8) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct flock {
    pub l_type: i16,
    pub l_whence: i16,
    pub l_start: i64,
    pub l_len: i64,
    pub l_pid: i32,
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
pub extern "C" fn fail(mut message: *const i8) {
// SAFETY: machine generated unsafe code
    unsafe {
        perror(message);
        exit(1);
    }
}

static mut ooi_path: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub extern "C" fn ooi_unlink() {
// SAFETY: machine generated unsafe code
    unsafe {
        unlink(ooi_path);
    }
}

#[no_mangle]
pub extern "C" fn only_one_instance() {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut fl: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        let mut dirlen: u64 = 0;
        let mut fd: i32 = 0;
        let mut dir: *mut i8 = std::ptr::null_mut::<i8>();
        dir = getenv((b"HOME\0" as *const u8).cast::<i8>());
        if dir.is_null() || i32::from(*dir.offset(0_isize)) != '/' as i32 {
            fputs((b"Bad home directory.\n\0" as *const u8).cast::<i8>(), stderr);
            exit(1);
        }
        dirlen = strlen(dir);
        ooi_path =
            malloc(dirlen.wrapping_add(::core::mem::size_of::<[i8; 19]>() as u64)).cast::<i8>();
        if ooi_path.is_null() {
            fail((b"malloc\0" as *const u8).cast::<i8>());
        }
        memcpy(
            ooi_path.cast::<libc::c_void>(),
            dir as *const libc::c_void,
            dirlen,
        );
        memcpy(
            ooi_path.offset(dirlen as isize).cast::<libc::c_void>(),
            (b"/rosetta-code-lock\0" as *const u8).cast::<i8>().cast::<libc::c_void>(),
            ::core::mem::size_of::<[i8; 19]>() as u64,
        );
        fd = open(ooi_path, 0o2 | 0o100, 0o600);
        if fd < 0_i32 {
            fail(ooi_path);
        }
        fl.l_start = 0;
        fl.l_len = 0;
        fl.l_type = 1;
        fl.l_whence = 0;
        if fcntl(fd, 6, &mut fl as *mut flock) < 0_i32 {
            fputs(
                (b"Another instance of this program is running.\n\0" as *const u8).cast::<i8>(),
                stderr,
            );
            exit(1);
        }
// SAFETY: machine generated unsafe code
        atexit(Some(ooi_unlink as unsafe extern "C" fn() -> ()));
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    only_one_instance();
    i = 10_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i > 0_i32 {
            if i % 5_i32 == 1_i32 {
                print!("{}...\n\0", i)
            } else {
                print!("{}... \0", i)
            };
            fflush(stdout);
            sleep(1);
            i = i.wrapping_sub(1);
            i;
        }
        puts((b"Fin!\0" as *const u8).cast::<i8>());
    }
    0_i32
}

pub fn main() {
// SAFETY: machine generated unsafe code
    unsafe {
        ::std::process::exit(main_0());
    }
}
