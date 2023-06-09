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
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fgetc(__stream: *mut FILE) -> i32;
    fn ungetc(__c: i32, __stream: *mut FILE) -> i32;
    fn feof(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn exit(_: i32) -> !;
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
static mut width: i32 = 0_i32;
static mut height: i32 = 0_i32;
static mut bitmap: [[u8; 2048]; 2048] = [[0; 2048]; 2048];
static mut oldColor: u8 = 0;
static mut newColor: u8 = 0;
#[no_mangle]
pub extern "C" fn floodFill(mut i: i32, mut j: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        if 0_i32 <= i
            && i < height
            && 0_i32 <= j
            && j < width
            && i32::from(bitmap[i as usize][j as usize]) == i32::from(oldColor)
        {
            bitmap[i as usize][j as usize] = newColor;
            floodFill(i.wrapping_sub(1), j);
            floodFill(i.wrapping_add(1), j);
            floodFill(i, j.wrapping_sub(1));
            floodFill(i, j.wrapping_add(1));
        }
    }
}

#[no_mangle]
pub extern "C" fn skipLine(mut file: *mut FILE) {
// SAFETY: machine generated unsafe code
    unsafe { while ferror(file) == 0_i32 && feof(file) == 0_i32 && fgetc(file) != '\n' as i32 {} }
}

#[no_mangle]
pub extern "C" fn skipCommentLines(mut file: *mut FILE) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut c: i32 = 0;
        let mut comment: i32 = '#' as i32;
        loop {
            c = fgetc(file);
            if c != comment {
                break;
            }
            skipLine(file);
        }
        ungetc(c, file);
    }
}

#[no_mangle]
pub extern "C" fn readPortableBitMap(mut file: *mut FILE) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        skipLine(file);
        skipCommentLines(file);
        fscanf(
            file,
            (b"%d\0" as *const u8).cast::<i8>(),
            &mut width as *mut i32,
        );
        skipCommentLines(file);
        fscanf(
            file,
            (b"%d\0" as *const u8).cast::<i8>(),
            &mut height as *mut i32,
        );
        skipCommentLines(file);
        if width <= 2_048_i32 && height <= 2_048_i32 {
            i = 0_i32;
            while i < height {
                j = 0_i32;
                while j < width {
                    fscanf(
                        file,
                        (b"%1d\0" as *const u8).cast::<i8>(),
                        &mut *(*bitmap.as_mut_ptr().offset(i as isize))
                            .as_mut_ptr()
                            .offset(j as isize) as *mut u8,
                    );
                    j = j.wrapping_add(1);
                    j;
                }
                i = i.wrapping_add(1);
                i;
            }
        } else {
            exit(1);
        }
        panic!("Reached end of non-void function without returning");
    }
}

#[no_mangle]
pub extern "C" fn writePortableBitMap(mut file: *mut FILE) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        fprintf(file, (b"P1\n\0" as *const u8).cast::<i8>());
        fprintf(file, (b"%d %d\n\0" as *const u8).cast::<i8>(), width, height);
        i = 0_i32;
        while i < height {
            j = 0_i32;
            while j < width {
                fprintf(
                    file,
                    (b"%1d\0" as *const u8).cast::<i8>(),
                    i32::from(bitmap[i as usize][j as usize]),
                );
                j = j.wrapping_add(1);
                j;
            }
            fprintf(file, (b"\n\0" as *const u8).cast::<i8>());
            i = i.wrapping_add(1);
            i;
        }
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        oldColor = 1;
        newColor = (if i32::from(oldColor) != 0_i32 { 0_i32 } else { 1_i32 }) as u8;
        readPortableBitMap(stdin);
        floodFill(height.wrapping_div(2), width.wrapping_div(2));
        writePortableBitMap(stdout);
    }
    0_i32
}

pub fn main() {
// SAFETY: machine generated unsafe code
    unsafe {
        ::std::process::exit(main_0());
    }
}
