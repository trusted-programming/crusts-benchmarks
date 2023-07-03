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
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
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
pub extern "C" fn cusipCheck(mut str: *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut sum: i32 = 0;
        let mut i: i32 = 0;
        let mut v: i32 = 0;
        i = 0_i32;
        while i < 8_i32 {
            if i32::from(*str.offset(i as isize)) >= '0' as i32
                && i32::from(*str.offset(i as isize)) <= '9' as i32
            {
                v = i32::from(*str.offset(i as isize)) - '0' as i32;
            } else if i32::from(*str.offset(i as isize)) >= 'A' as i32
                && i32::from(*str.offset(i as isize)) <= 'Z' as i32
            {
                v = i32::from(*str.offset(i as isize)) - 'A' as i32 + 10_i32;
            } else if i32::from(*str.offset(i as isize)) == '*' as i32 {
                v = 36_i32;
            } else if i32::from(*str.offset(i as isize)) == '@' as i32 {
                v = 37_i32;
            } else if i32::from(*str.offset(i as isize)) == '#' as i32 {
                v = 38_i32;
            }
            if i % 2_i32 != 0_i32 {
                v *= 2_i32;
            }
            sum += v / 10_i32 + v % 10_i32;
            i = i.wrapping_add(1);
            i;
        }
        (10_i32 - sum % 10_i32) % 10_i32
    }
}

fn main_0(mut argC: i32, mut argV: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut cusipStr: [i8; 10] = [0; 10];
        let mut i: i32 = 0;
        let mut numLines: i32 = 0;
        if argC == 1_i32 {
            print!(
                "Usage : {} <full path of CUSIP Data file>",
                build_str_from_raw_ptr((*argV.offset(0_isize)).cast::<u8>())
            );
        } else {
            let mut fp: *mut FILE =
                fopen(*argV.offset(1_isize), (b"r\0" as *const u8).cast::<i8>());
            fscanf(
                fp,
                (b"%d\0" as *const u8).cast::<i8>(),
                &mut numLines as *mut i32,
            );
            println!("CUSIP       Verdict");
            print!("-------------------");
            i = 0_i32;
            while i < numLines {
                fscanf(fp, (b"%s\0" as *const u8).cast::<i8>(), cusipStr.as_mut_ptr());
                if cusipCheck(cusipStr.as_mut_ptr()) == i32::from(cusipStr[8_usize]) - '0' as i32 {
                    print!(
                        "\n{} : Valid\0",
                        build_str_from_raw_ptr(cusipStr.as_mut_ptr().cast::<u8>())
                    )
                } else {
                    print!(
                        "\n{} : Invalid\0",
                        build_str_from_raw_ptr(cusipStr.as_mut_ptr().cast::<u8>())
                    )
                };
                i = i.wrapping_add(1);
                i;
            }
            fclose(fp);
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
