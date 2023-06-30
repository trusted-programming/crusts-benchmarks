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
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn free(_: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct genome {
    pub strand: *mut i8,
    pub length: i32,
    pub next: *mut genome,
}
#[no_mangle]
pub static mut genomeData: *mut genome = 0 as *const genome as *mut genome;
#[no_mangle]
pub static mut totalLength: i32 = 0_i32;
#[no_mangle]
pub static mut Adenine: i32 = 0_i32;
#[no_mangle]
pub static mut Cytosine: i32 = 0_i32;
#[no_mangle]
pub static mut Guanine: i32 = 0_i32;
#[no_mangle]
pub static mut Thymine: i32 = 0_i32;
#[no_mangle]
pub extern "C" fn numDigits(mut num: i32) -> i32 {
    let mut len: i32 = 1;
    while num > 10_i32 {
        num /= 10_i32;
        len = len.wrapping_add(1);
        len;
    }
    len
}

#[no_mangle]
pub extern "C" fn buildGenome(mut str: *mut i8) {
    unsafe {
        let mut len: i32 = strlen(str as *const i8) as i32;
        let mut i: i32 = 0;
        let mut genomeIterator: *mut genome = std::ptr::null_mut::<genome>();
        let mut newGenome: *mut genome = std::ptr::null_mut::<genome>();
        totalLength = totalLength.wrapping_add(len);
        i = 0_i32;
        while i < len {
            match i32::from(*str.offset(i as isize)) {
                65_i32 => {
                    Adenine = Adenine.wrapping_add(1);
                    Adenine;
                }
                84_i32 => {
                    Thymine = Thymine.wrapping_add(1);
                    Thymine;
                }
                67_i32 => {
                    Cytosine = Cytosine.wrapping_add(1);
                    Cytosine;
                }
                71_i32 => {
                    Guanine = Guanine.wrapping_add(1);
                    Guanine;
                }
                _ => {}
            }
            i = i.wrapping_add(1);
            i;
        }
        if genomeData.is_null() {
            genomeData = malloc(::core::mem::size_of::<genome>() as u64).cast::<genome>();
            (*genomeData).strand =
                malloc((len as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64)).cast::<i8>();
            strcpy((*genomeData).strand, str as *const i8);
            (*genomeData).length = len;
            (*genomeData).next = std::ptr::null_mut::<genome>();
        } else {
            genomeIterator = genomeData;
            while !((*genomeIterator).next).is_null() {
                genomeIterator = (*genomeIterator).next;
            }
            newGenome = malloc(::core::mem::size_of::<genome>() as u64).cast::<genome>();
            (*newGenome).strand =
                malloc((len as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64)).cast::<i8>();
            strcpy((*newGenome).strand, str as *const i8);
            (*newGenome).length = len;
            (*newGenome).next = std::ptr::null_mut::<genome>();
            (*genomeIterator).next = newGenome;
        };
    }
}

#[no_mangle]
pub extern "C" fn printGenome() {
    unsafe {
        let mut genomeIterator: *mut genome = genomeData;
        let mut width: i32 = numDigits(totalLength);
        let mut len: i32 = 0;
        println!("Sequence:");
        while !genomeIterator.is_null() {
            print!(
                "\n{1:0$}{2:3}{3:3}",
                (width + 1).unsigned_abs() as usize,
                len,
                ":\0",
                build_str_from_raw_ptr((*genomeIterator).strand.cast::<u8>())
            );
            len += (*genomeIterator).length;
            genomeIterator = (*genomeIterator).next;
        }
        print!("\n\nBase Count\n----------\n\n");
        println!(
            "{0:3}{1:3}{3:2$}",
            'A' as i32,
            ":\0",
            (width + 1).unsigned_abs() as usize,
            Adenine
        );
        println!(
            "{0:3}{1:3}{3:2$}",
            'T' as i32,
            ":\0",
            (width + 1).unsigned_abs() as usize,
            Thymine
        );
        println!(
            "{0:3}{1:3}{3:2$}",
            'C' as i32,
            ":\0",
            (width + 1).unsigned_abs() as usize,
            Cytosine
        );
        println!(
            "{0:3}{1:3}{3:2$}",
            'G' as i32,
            ":\0",
            (width + 1).unsigned_abs() as usize,
            Guanine
        );
        print!(
            "\n{0:3}{2:1$}\n",
            "Total:\0",
            (width + 1).unsigned_abs() as usize,
            Adenine + Thymine + Cytosine + Guanine
        );
        free(genomeData.cast::<libc::c_void>());
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut str: [i8; 100] = [0; 100];
        let mut _counter: i32 = 0;
        let mut _len: i32 = 0;
        if argc != 2_i32 {
            println!(
                "Usage : {} <Gene file name>",
                build_str_from_raw_ptr((*argv.offset(0_isize)).cast::<u8>())
            );
            return 0_i32;
        }
        let mut fp: *mut FILE = fopen(*argv.offset(1_isize), (b"r\0" as *const u8).cast::<i8>());
        while fscanf(fp, (b"%s\0" as *const u8).cast::<i8>(), str.as_mut_ptr()) != -1_i32 {
            buildGenome(str.as_mut_ptr());
        }
        fclose(fp);
        printGenome();
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
