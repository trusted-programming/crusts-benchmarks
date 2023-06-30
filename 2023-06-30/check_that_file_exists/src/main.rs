#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {
    fn stat(__file: *const i8, __buf: *mut stat) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [i64; 3],
}
#[no_mangle]
pub extern "C" fn check_reg(mut path: *const i8) -> i32 {
    unsafe {
        let mut sb: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        i32::from(stat(path, &mut sb) == 0_i32 && sb.st_mode & 0o170000 == 0o100000u32)
    }
}

#[no_mangle]
pub extern "C" fn check_dir(mut path: *const i8) -> i32 {
    unsafe {
        let mut sb: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        i32::from(stat(path, &mut sb) == 0_i32 && sb.st_mode & 0o170000 == 0o40000u32)
    }
}

fn main_0() -> i32 {
    if check_reg((b"input.txt\0" as *const u8).cast::<i8>()) != 0_i32 {
        println!("input.txt is a regular file? yes\0")
    } else {
        println!("input.txt is a regular file? no\0")
    };
    if check_dir((b"docs\0" as *const u8).cast::<i8>()) != 0_i32 {
        println!("docs is a directory? yes\0")
    } else {
        println!("docs is a directory? no\0")
    };
    if check_reg((b"/input.txt\0" as *const u8).cast::<i8>()) != 0_i32 {
        println!("/input.txt is a regular file? yes\0")
    } else {
        println!("/input.txt is a regular file? no\0")
    };
    if check_dir((b"/docs\0" as *const u8).cast::<i8>()) != 0_i32 {
        println!("/docs is a directory? yes\0")
    } else {
        println!("/docs is a directory? no\0")
    };
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
