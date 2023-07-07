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
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
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
pub extern "C" fn process(mut lineNum: i32, mut buffer: *mut i8) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut days: [[i8; 64]; 7] = [[0; 64]; 7];
        let mut i: i32 = 0;
        let mut d: i32 = 0;
        let mut j: i32 = 0;
        while i32::from(*buffer.offset(i as isize)) != 0_i32 {
            if i32::from(*buffer.offset(i as isize)) == ' ' as i32 {
                days[d as usize][j as usize] = '\0' as i8;
                d = d.wrapping_add(1);
                d;
                j = 0_i32;
            } else if i32::from(*buffer.offset(i as isize)) == '\n' as i32
                || i32::from(*buffer.offset(i as isize)) == '\r' as i32
            {
                days[d as usize][j as usize] = '\0' as i8;
                d = d.wrapping_add(1);
                d;
                break;
            } else {
                days[d as usize][j as usize] = *buffer.offset(i as isize);
                j = j.wrapping_add(1);
                j;
            }
            if d >= 7_i32 {
                printf(
                    (b"There aren't 7 days in line %d\n\0" as *const u8).cast::<i8>(),
                    lineNum,
                );
                return;
            }
            i = i.wrapping_add(1);
            i;
        }
        if i32::from(*buffer.offset(i as isize)) == '\0' as i32 {
            days[d as usize][j as usize] = '\0' as i8;
            d = d.wrapping_add(1);
            d;
        }
        if d < 7_i32 {
            printf(
                (b"There aren't 7 days in line %d\n\0" as *const u8).cast::<i8>(),
                lineNum,
            );
            return;
        } else {
            let mut len: i32 = 0;
            len = 1_i32;
            while len < 64_i32 {
                let mut current_block_35: u64;
                let mut d1: i32 = 0;
                d1 = 0_i32;
                's_113: loop {
                    if d1 >= 7_i32 {
                        current_block_35 = 18153031941552419006;
                        break;
                    }
                    let mut d2: i32 = 0;
                    d2 = d1.wrapping_add(1);
                    while d2 < 7_i32 {
                        let mut unique: i32 = 0;
                        i = 0_i32;
                        while i < len {
                            if i32::from(days[d1 as usize][i as usize])
                                != i32::from(days[d2 as usize][i as usize])
                            {
                                unique = 1_i32;
                                break;
                            } else {
                                i = i.wrapping_add(1);
                                i;
                            }
                        }
                        if unique == 0_i32 {
                            current_block_35 = 10891380440665537214;
                            break 's_113;
                        }
                        d2 = d2.wrapping_add(1);
                        d2;
                    }
                    d1 = d1.wrapping_add(1);
                    d1;
                }
                match current_block_35 {
                    10891380440665537214 => {}
                    _ => {
                        print!("{:2} ", len);
                        i = 0_i32;
                        while i < 7_i32 {
                            print!(
                                " {}",
                                build_str_from_raw_ptr((days[i as usize]).as_mut_ptr().cast::<u8>())
                            );
                            i = i.wrapping_add(1);
                            i;
                        }
                        println!();
                        return;
                    }
                }
                len = len.wrapping_add(1);
                len;
            }
        }
        print!("Failed to find uniqueness within the bounds.");
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut buffer: [i8; 1024] = [0; 1024];
        let mut lineNum: i32 = 1;
        let mut len: i32 = 0;
        let mut fp: *mut FILE = std::ptr::null_mut::<FILE>();
        fp = fopen(
            (b"days_of_week.txt\0" as *const u8).cast::<i8>(),
            (b"r\0" as *const u8).cast::<i8>(),
        );
        loop {
            memset(
                buffer.as_mut_ptr().cast::<libc::c_void>(),
                0,
                ::core::mem::size_of::<[i8; 1024]>() as u64,
            );
            fgets(
                buffer.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 1024]>() as i32,
                fp,
            );
            len = strlen(buffer.as_mut_ptr()) as i32;
            if len == 0_i32 || i32::from(buffer[(len.wrapping_sub(1i32)) as usize]) == '\0' as i32 {
                break;
            }
            let fresh0 = lineNum;
            lineNum = lineNum.wrapping_add(1);
            process(fresh0, buffer.as_mut_ptr());
        }
        fclose(fp);
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
