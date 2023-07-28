#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn bruteForceProperDivisorSum(
    mut n: libc::c_ulonglong,
) -> libc::c_ulonglong {
    let mut i: libc::c_ulonglong = 0;
    let mut sum: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    i = 1 as libc::c_int as libc::c_ulonglong;
    while i
        < n
            .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
            .wrapping_div(2 as libc::c_int as libc::c_ulonglong)
    {
        if n.wrapping_rem(i) == 0 as libc::c_int as libc::c_ulonglong && n != i {
            sum = sum.wrapping_add(i);
        }
        i = i.wrapping_add(1);
        i;
    }
    return sum;
}
#[no_mangle]
pub unsafe extern "C" fn printSeries(
    mut arr: *mut libc::c_ulonglong,
    mut size: libc::c_int,
    mut type_0: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    printf(
        b"\nInteger : %llu, Type : %s, Series : \0" as *const u8 as *const libc::c_char,
        *arr.offset(0 as libc::c_int as isize),
        type_0,
    );
    i = 0 as libc::c_int;
    while i < size - 1 as libc::c_int {
        printf(b"%llu, \0" as *const u8 as *const libc::c_char, *arr.offset(i as isize));
        i += 1;
        i;
    }
    printf(b"%llu\0" as *const u8 as *const libc::c_char, *arr.offset(i as isize));
}
#[no_mangle]
pub unsafe extern "C" fn aliquotClassifier(mut n: libc::c_ulonglong) {
    let mut arr: [libc::c_ulonglong; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    arr[0 as libc::c_int as usize] = n;
    i = 1 as libc::c_int;
    while i < 16 as libc::c_int {
        arr[i
            as usize] = bruteForceProperDivisorSum(arr[(i - 1 as libc::c_int) as usize]);
        if arr[i as usize] == 0 as libc::c_int as libc::c_ulonglong
            || arr[i as usize] == n
            || arr[i as usize] == arr[(i - 1 as libc::c_int) as usize]
                && arr[i as usize] != n
        {
            printSeries(
                arr.as_mut_ptr(),
                i + 1 as libc::c_int,
                (if arr[i as usize] == 0 as libc::c_int as libc::c_ulonglong {
                    b"Terminating\0" as *const u8 as *const libc::c_char
                } else if arr[i as usize] == n && i == 1 as libc::c_int {
                    b"Perfect\0" as *const u8 as *const libc::c_char
                } else if arr[i as usize] == n && i == 2 as libc::c_int {
                    b"Amicable\0" as *const u8 as *const libc::c_char
                } else if arr[i as usize] == arr[(i - 1 as libc::c_int) as usize]
                    && arr[i as usize] != n
                {
                    b"Aspiring\0" as *const u8 as *const libc::c_char
                } else {
                    b"Sociable\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char,
            );
            return;
        }
        j = 1 as libc::c_int;
        while j < i {
            if arr[j as usize] == arr[i as usize] {
                printSeries(
                    arr.as_mut_ptr(),
                    i + 1 as libc::c_int,
                    b"Cyclic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                return;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    printSeries(
        arr.as_mut_ptr(),
        i + 1 as libc::c_int,
        b"Non-Terminating\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn processFile(mut fileName: *mut libc::c_char) {
    let mut fp: *mut FILE = fopen(fileName, b"r\0" as *const u8 as *const libc::c_char);
    let mut str: [libc::c_char; 21] = [0; 21];
    while !(fgets(str.as_mut_ptr(), 21 as libc::c_int, fp)).is_null() {
        aliquotClassifier(
            strtoull(
                str.as_mut_ptr(),
                0 as *mut libc::c_void as *mut *mut libc::c_char,
                10 as libc::c_int,
            ),
        );
    }
    fclose(fp);
}
unsafe fn main_0(
    mut argC: libc::c_int,
    mut argV: *mut *mut libc::c_char,
) -> libc::c_int {
    if argC != 2 as libc::c_int {
        printf(
            b"Usage : %s <positive integer>\0" as *const u8 as *const libc::c_char,
            *argV.offset(0 as libc::c_int as isize),
        );
    } else if !(strchr(*argV.offset(1 as libc::c_int as isize), '.' as i32)).is_null() {
        processFile(*argV.offset(1 as libc::c_int as isize));
    } else {
        aliquotClassifier(
            strtoull(
                *argV.offset(1 as libc::c_int as isize),
                0 as *mut libc::c_void as *mut *mut libc::c_char,
                10 as libc::c_int,
            ),
        );
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
