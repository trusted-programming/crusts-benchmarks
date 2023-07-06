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
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atof(__nptr: *const i8) -> f64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strtok(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
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
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct vector {
    pub values: *mut libc::c_float,
    pub size: i32,
}
#[no_mangle]
pub extern "C" fn extractVector(mut str: *mut i8) -> vector {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut coeff: vector = vector {
            values: std::ptr::null_mut::<libc::c_float>(),
            size: 0,
        };
        let mut i: i32 = 0;
        let mut count: i32 = 1;
        let mut token: *mut i8 = std::ptr::null_mut::<i8>();
        while i32::from(*str.offset(i as isize)) != 0_i32 {
            let fresh0 = i;
            i += 1_i32;
            if i32::from(*str.offset(fresh0 as isize)) == ' ' as i32 {
                count += 1_i32;
                count;
            }
        }
        coeff.values =
            malloc((count as u64).wrapping_mul(::core::mem::size_of::<libc::c_float>() as u64)).cast::<f32>();
        coeff.size = count;
        token = strtok(str, (b" \0" as *const u8).cast::<i8>());
        i = 0_i32;
        while !token.is_null() {
            let fresh1 = i;
            i += 1_i32;
            *(coeff.values).offset(fresh1 as isize) = atof(token) as libc::c_float;
            token = strtok(std::ptr::null_mut::<i8>(), (b" \0" as *const u8).cast::<i8>());
        }
        coeff
    }
}

#[no_mangle]
pub extern "C" fn processSignalFile(mut fileName: *mut i8) -> vector {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut sum: libc::c_float = 0.;
        let mut str: [i8; 1000] = [0; 1000];
        let mut coeff1: vector = vector {
            values: std::ptr::null_mut::<libc::c_float>(),
            size: 0,
        };
        let mut coeff2: vector = vector {
            values: std::ptr::null_mut::<libc::c_float>(),
            size: 0,
        };
        let mut signal: vector = vector {
            values: std::ptr::null_mut::<libc::c_float>(),
            size: 0,
        };
        let mut filteredSignal: vector = vector {
            values: std::ptr::null_mut::<libc::c_float>(),
            size: 0,
        };
        let mut fp: *mut FILE = fopen(fileName, (b"r\0" as *const u8).cast::<i8>());
        fgets(str.as_mut_ptr(), 1000, fp);
        coeff1 = extractVector(str.as_mut_ptr());
        fgets(str.as_mut_ptr(), 1000, fp);
        coeff2 = extractVector(str.as_mut_ptr());
        fgets(str.as_mut_ptr(), 1000, fp);
        signal = extractVector(str.as_mut_ptr());
        fclose(fp);
        filteredSignal.values = calloc(
            signal.size as u64,
            ::core::mem::size_of::<libc::c_float>() as u64,
        ).cast::<f32>();
        filteredSignal.size = signal.size;
        i = 0_i32;
        while i < signal.size {
            sum = 0_i32 as libc::c_float;
            j = 0_i32;
            while j < coeff2.size {
                if i - j >= 0_i32 {
                    sum += *(coeff2.values).offset(j as isize)
                        * *(signal.values).offset((i - j) as isize);
                }
                j += 1_i32;
                j;
            }
            j = 0_i32;
            while j < coeff1.size {
                if i - j >= 0_i32 {
                    sum -= *(coeff1.values).offset(j as isize)
                        * *(filteredSignal.values).offset((i - j) as isize);
                }
                j += 1_i32;
                j;
            }
            sum /= *(coeff1.values).offset(0_isize);
            *(filteredSignal.values).offset(i as isize) = sum;
            i += 1_i32;
            i;
        }
        filteredSignal
    }
}

#[no_mangle]
pub extern "C" fn printVector(mut v: vector, mut outputFile: *mut i8) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        if outputFile.is_null() {
            print!("[");
            i = 0_i32;
            while i < v.size {
                print!("{:.12}, ", f64::from(*(v.values).offset(i as isize)));
                i += 1_i32;
                i;
            }
            print!("\x08\x08]");
        } else {
            let mut fp: *mut FILE = fopen(outputFile, (b"w\0" as *const u8).cast::<i8>());
            i = 0_i32;
            while i < v.size - 1_i32 {
                fprintf(
                    fp,
                    (b"%.12f, \0" as *const u8).cast::<i8>(),
                    f64::from(*(v.values).offset(i as isize)),
                );
                i += 1_i32;
                i;
            }
            fprintf(
                fp,
                (b"%.12f\0" as *const u8).cast::<i8>(),
                f64::from(*(v.values).offset(i as isize)),
            );
            fclose(fp);
        };
    }
}

fn main_0(mut argC: i32, mut argV: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut str: *mut i8 = std::ptr::null_mut::<i8>();
        if !(2_i32..=3_i32).contains(&argC) {
            print!(
                "Usage : {} <name of signal data file and optional output file.>",
                build_str_from_raw_ptr((*argV.offset(0_isize)).cast::<u8>())
            );
        } else {
            if argC != 2_i32 {
                str = malloc(
                    (strlen(*argV.offset(2_isize)))
                        .wrapping_add(strlen(str))
                        .wrapping_add(1)
                        .wrapping_mul(::core::mem::size_of::<i8>() as u64),
                ).cast::<i8>();
                strcpy(str, (b"written to \0" as *const u8).cast::<i8>());
            }
            if argC == 2_i32 {
                print!("Filtered signal is:\n\0")
            } else {
                print!(
                    "Filtered signal {}",
                    build_str_from_raw_ptr(
                        strcat(str, *argV.offset(2_isize)) as *const i8 as *mut u8
                    )
                )
            };
            printVector(
                processSignalFile(*argV.offset(1_isize)),
                *argV.offset(2_isize),
            );
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
