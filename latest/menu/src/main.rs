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
    static mut stdin: *mut FILE;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn atoi(__nptr: *const i8) -> i32;
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
fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut items: [*const i8; 5] = [
            (b"fee fie\0" as *const u8).cast::<i8>(),
            (b"huff and puff\0" as *const u8).cast::<i8>(),
            (b"mirror mirror\0" as *const u8).cast::<i8>(),
            (b"tick tock\0" as *const u8).cast::<i8>(),
            std::ptr::null::<i8>(),
        ];
        let mut prompt: *const i8 = (b"Which is from the three pigs?\0" as *const u8).cast::<i8>();
        println!(
            "You chose {}.",
            build_str_from_raw_ptr(menu_select(items.as_mut_ptr(), prompt) as *mut u8)
        );
        0_i32
    }
}

#[no_mangle]
pub extern "C" fn menu_select(mut items: *const *const i8, mut prompt: *const i8) -> *const i8 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut buf: [i8; 8192] = [0; 8192];
        let mut i: i32 = 0;
        let mut choice: i32 = 0;
        let mut choice_max: i32 = 0;
        if items.is_null() {
            return std::ptr::null::<i8>();
        }
        loop {
            i = 0_i32;
            while !(*items.offset(i as isize)).is_null() {
                println!(
                    "{}) {}",
                    i + 1_i32,
                    build_str_from_raw_ptr(*items.offset(i as isize) as *mut u8)
                );
                i = i.wrapping_add(1);
                i;
            }
            choice_max = i;
            if !prompt.is_null() {
                print!("{} ", build_str_from_raw_ptr(prompt as *mut u8));
            } else {
                print!("Choice? ");
            }
            if !(fgets(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 8192]>() as i32,
                stdin,
            ))
            .is_null()
            {
                choice = atoi(buf.as_mut_ptr());
            }
            if !(1_i32 > choice || choice > choice_max) {
                break;
            }
        }
        *items.offset((choice.wrapping_sub(1i32)) as isize)
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
