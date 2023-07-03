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
    fn atoi(__nptr: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fwrite(_: *const libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
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
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut sandPileEdge: i32 = 0;
        let mut centerPileHeight: i32 = 0;
        let mut processAgain: i32 = 1;
        let mut top: i32 = 0;
        let mut down: i32 = 0;
        let mut left: i32 = 0;
        let mut right: i32 = 0;
        let mut sandPile: *mut *mut i32 = std::ptr::null_mut::<*mut i32>();
        let mut fileName: *mut i8 = std::ptr::null_mut::<i8>();
        static mut colour: [u8; 3] = [0; 3];
        if argc != 3_i32 {
            print!(
                "Usage: {} <Sand pile side> <Center pile height>",
                build_str_from_raw_ptr((*argv.offset(0_isize)).cast::<u8>())
            );
            return 0_i32;
        }
        sandPileEdge = atoi(*argv.offset(1_isize));
        centerPileHeight = atoi(*argv.offset(2_isize));
        if sandPileEdge <= 0_i32 || centerPileHeight <= 0_i32 {
            print!("Sand pile and center pile dimensions must be positive integers.");
            return 0_i32;
        }
        sandPile =
            malloc((sandPileEdge as u64).wrapping_mul(::core::mem::size_of::<*mut i32>() as u64)).cast::<*mut i32>();
        i = 0_i32;
        while i < sandPileEdge {
            let fresh0 = &mut (*sandPile.offset(i as isize));
            *fresh0 = calloc(sandPileEdge as u64, ::core::mem::size_of::<i32>() as u64).cast::<i32>();
            i = i.wrapping_add(1);
            i;
        }
        *(*sandPile.offset((sandPileEdge / 2i32) as isize))
            .offset((sandPileEdge / 2i32) as isize) = centerPileHeight;
        print!("Initial sand pile :\n\n");
        i = 0_i32;
        while i < sandPileEdge {
            j = 0_i32;
            while j < sandPileEdge {
                print!("{:3}", *(*sandPile.offset(i as isize)).offset(j as isize));
                j = j.wrapping_add(1);
                j;
            }
            println!();
            i = i.wrapping_add(1);
            i;
        }
        while processAgain == 1_i32 {
            processAgain = 0_i32;
            top = 0_i32;
            down = 0_i32;
            left = 0_i32;
            right = 0_i32;
            i = 0_i32;
            while i < sandPileEdge {
                j = 0_i32;
                while j < sandPileEdge {
                    if *(*sandPile.offset(i as isize)).offset(j as isize) >= 4_i32 {
                        if i > 0_i32 {
                            top = 1_i32;
                            *(*sandPile.offset((i - 1i32) as isize)).offset(j as isize) += 1_i32;
                            if *(*sandPile.offset((i - 1i32) as isize)).offset(j as isize) >= 4_i32 {
                                processAgain = 1_i32;
                            }
                        }
                        if (i + 1_i32) < sandPileEdge {
                            down = 1_i32;
                            *(*sandPile.offset((i + 1i32) as isize)).offset(j as isize) += 1_i32;
                            if *(*sandPile.offset((i + 1i32) as isize)).offset(j as isize) >= 4_i32 {
                                processAgain = 1_i32;
                            }
                        }
                        if j > 0_i32 {
                            left = 1_i32;
                            *(*sandPile.offset(i as isize)).offset((j - 1i32) as isize) += 1_i32;
                            if *(*sandPile.offset(i as isize)).offset((j - 1i32) as isize) >= 4_i32 {
                                processAgain = 1_i32;
                            }
                        }
                        if (j + 1_i32) < sandPileEdge {
                            right = 1_i32;
                            *(*sandPile.offset(i as isize)).offset((j + 1i32) as isize) += 1_i32;
                            if *(*sandPile.offset(i as isize)).offset((j + 1i32) as isize) >= 4_i32 {
                                processAgain = 1_i32;
                            }
                        };
                        *(*sandPile.offset(i as isize)).offset(j as isize) -=
                            top + down + left + right;
                        if *(*sandPile.offset(i as isize)).offset(j as isize) >= 4_i32 {
                            processAgain = 1_i32;
                        }
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        print!("Final sand pile : \n\n");
        i = 0_i32;
        while i < sandPileEdge {
            j = 0_i32;
            while j < sandPileEdge {
                print!("{:3}", *(*sandPile.offset(i as isize)).offset(j as isize));
                j = j.wrapping_add(1);
                j;
            }
            println!();
            i = i.wrapping_add(1);
            i;
        }
        fileName = malloc(
            (strlen(*argv.offset(1_isize)))
                .wrapping_add(strlen(*argv.offset(2_isize)))
                .wrapping_add(23)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
        ).cast::<i8>();
        strcpy(fileName, (b"Final_Sand_Pile_\0" as *const u8).cast::<i8>());
        strcat(fileName, *argv.offset(1_isize));
        strcat(fileName, (b"_\0" as *const u8).cast::<i8>());
        strcat(fileName, *argv.offset(2_isize));
        strcat(fileName, (b".ppm\0" as *const u8).cast::<i8>());
        let mut fp: *mut FILE = fopen(fileName, (b"wb\0" as *const u8).cast::<i8>());
        fprintf(
            fp,
            (b"P6\n%d %d\n255\n\0" as *const u8).cast::<i8>(),
            sandPileEdge,
            sandPileEdge,
        );
        i = 0_i32;
        while i < sandPileEdge {
            j = 0_i32;
            while j < sandPileEdge {
                colour[0_usize] =
                    ((*(*sandPile.offset(i as isize)).offset(j as isize) + i) % 256i32) as u8;
                colour[1_usize] =
                    ((*(*sandPile.offset(i as isize)).offset(j as isize) + j) % 256i32) as u8;
                colour[2_usize] =
                    ((*(*sandPile.offset(i as isize)).offset(j as isize) + i * j) % 256i32) as u8;
                fwrite(colour.as_mut_ptr() as *const libc::c_void, 1, 3, fp);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        fclose(fp);
        print!(
            "\nImage file written to {}\n",
            build_str_from_raw_ptr(fileName.cast::<u8>())
        );
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
