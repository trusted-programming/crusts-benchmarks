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
use c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const u16;
    fn toupper(_: i32) -> i32;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn strtoul(_: *const i8, _: *mut *mut i8, _: i32) -> u64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
}
pub const _ISalnum: u32 = 8;
pub const _ISpunct: u32 = 4;
pub const _IScntrl: u32 = 2;
pub const _ISblank: u32 = 1;
pub const _ISgraph: u32 = 32768;
pub const _ISprint: u32 = 16384;
pub const _ISspace: u32 = 8192;
pub const _ISxdigit: u32 = 4096;
pub const _ISdigit: u32 = 2048;
pub const _ISalpha: u32 = 1024;
pub const _ISlower: u32 = 512;
pub const _ISupper: u32 = 256;
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
pub struct command_tag {
    pub cmd: *mut i8,
    pub length: u64,
    pub min_len: u64,
    pub next: *mut command_tag,
}
pub type command_t = command_tag;
#[no_mangle]
pub static mut command_table : * const i8 =
b"add 1  alter 3  backup 2  bottom 1  Cappend 2  change 1  Schange  Cinsert 2  Clast 3 compress 4 copy 2 count 3 Coverlay 3 cursor 3  delete 3 Cdelete 2  down 1  duplicate 3 xEdit 1 expand 3 extract 3  find 1 Nfind 2 Nfindup 6 NfUP 3 Cfind 2 findUP 3 fUP 2 forward 2  get  help 1 hexType 4  input 1 powerInput 3  join 1 split 2 spltJOIN load locate 1 Clocate 2 lowerCase 3 upperCase 3 Lprefix 2  macro  merge 2 modify 3 move 2 msg  next 1 overlay 1 parse preserve 4 purge 3 put putD query 1 quit  read recover 3 refresh renum 3 repeat 3 replace 1 Creplace 2 reset 3 restore 4 rgtLEFT right 2 left 2  save  set  shift 2  si  sort  sos  stack 3 status 4 top  transfer 3  type 1  up 1\0"
 as * const u8 as * const i8;
#[no_mangle]
pub extern "C" fn command_match(mut command: *const command_t, mut str: *const i8) -> bool {
    unsafe {
        let mut olen: u64 = strlen(str);
        return olen >= (*command).min_len
            && olen <= (*command).length
            && strncmp(str, (*command).cmd, olen) == 0;
    }
}

#[no_mangle]
pub extern "C" fn uppercase(mut str: *mut i8, mut n: u64) -> *mut i8 {
    unsafe {
        let mut i: u64 = 0;
        while i < n {
            *str.offset(i as isize) = toupper(*str.offset(i as isize) as i32) as i8;
            i = i.wrapping_add(1);
            i;
        }
        return str;
    }
}

#[no_mangle]
pub extern "C" fn fatal(mut message: *const i8) {
    unsafe {
        fprintf(stderr, b"%s\n\0" as *const u8 as *const i8, message);
        exit(1);
    }
}

#[no_mangle]
pub extern "C" fn xmalloc(mut n: u64) -> *mut libc::c_void {
    unsafe {
        let mut ptr: *mut libc::c_void = malloc(n);
        if ptr.is_null() {
            fatal(b"Out of memory\0" as *const u8 as *const i8);
        }
        return ptr;
    }
}

#[no_mangle]
pub extern "C" fn xrealloc(mut p: *mut libc::c_void, mut n: u64) -> *mut libc::c_void {
    unsafe {
        let mut ptr: *mut libc::c_void = realloc(p, n);
        if ptr.is_null() {
            fatal(b"Out of memory\0" as *const u8 as *const i8);
        }
        return ptr;
    }
}

#[no_mangle]
pub extern "C" fn split_into_words(mut str: *const i8, mut count: *mut u64) -> *mut *mut i8 {
    unsafe {
        let mut size: u64 = 0;
        let mut capacity: u64 = 16;
        let mut words: *mut *mut i8 =
            xmalloc(capacity.wrapping_mul(::core::mem::size_of::<*mut i8>() as u64))
                as *mut *mut i8;
        let mut len: u64 = strlen(str);
        let mut begin: u64 = 0;
        while begin < len {
            let mut i: u64 = begin;
            while i < len
                && *(*__ctype_b_loc()).offset(*str.offset(i as isize) as i32 as isize) as i32
                    & _ISspace as i32
                    != 0
            {
                i = i.wrapping_add(1);
                i;
            }
            begin = i;
            while i < len
                && *(*__ctype_b_loc()).offset(*str.offset(i as isize) as i32 as isize) as i32
                    & _ISspace as i32
                    == 0
            {
                i = i.wrapping_add(1);
                i;
            }
            let mut word_len: u64 = i.wrapping_sub(begin);
            if word_len == 0 {
                break;
            }
            let mut word: *mut i8 = xmalloc(word_len.wrapping_add(1)) as *mut i8;
            memcpy(
                word as *mut libc::c_void,
                str.offset(begin as isize) as *const libc::c_void,
                word_len,
            );
            *word.offset(word_len as isize) = 0;
            begin = (begin).wrapping_add(word_len) as u64;
            if capacity == size {
                capacity = (capacity).wrapping_mul(2) as u64;
                words = xrealloc(
                    words as *mut libc::c_void,
                    capacity.wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
                ) as *mut *mut i8;
            }
            let fresh0 = size;
            size = size.wrapping_add(1);
            let ref mut fresh1 = *words.offset(fresh0 as isize);
            *fresh1 = word;
        }
        *count = size;
        return words;
    }
}

#[no_mangle]
pub extern "C" fn make_command_list(mut table: *const i8) -> *mut command_t {
    unsafe {
        let mut cmd: *mut command_t = 0 as *mut command_t;
        let mut count: u64 = 0;
        let mut words: *mut *mut i8 = split_into_words(table, &mut count);
        let mut i: u64 = 0;
        while i < count {
            let mut word: *mut i8 = *words.offset(i as isize);
            let mut new_cmd: *mut command_t =
                xmalloc(::core::mem::size_of::<command_t>() as u64) as *mut command_t;
            let mut word_len: u64 = strlen(word);
            (*new_cmd).length = word_len;
            (*new_cmd).min_len = word_len;
            (*new_cmd).cmd = uppercase(word, word_len);
            if i.wrapping_add(1) < count {
                let mut eptr: *mut i8 = 0 as *mut i8;
                let mut min_len: u64 =
                    strtoul(*words.offset(i.wrapping_add(1) as isize), &mut eptr, 10);
                if min_len > 0 && *eptr as i32 == 0 {
                    free(*words.offset(i.wrapping_add(1) as isize) as *mut libc::c_void);
                    (*new_cmd).min_len = min_len;
                    i = i.wrapping_add(1);
                    i;
                }
            };
            (*new_cmd).next = cmd;
            cmd = new_cmd;
            i = i.wrapping_add(1);
            i;
        }
        free(words as *mut libc::c_void);
        return cmd;
    }
}

#[no_mangle]
pub extern "C" fn free_command_list(mut cmd: *mut command_t) {
    unsafe {
        while !cmd.is_null() {
            let mut next: *mut command_t = (*cmd).next;
            free((*cmd).cmd as *mut libc::c_void);
            free(cmd as *mut libc::c_void);
            cmd = next;
        }
    }
}

#[no_mangle]
pub extern "C" fn find_command(
    mut commands: *const command_t,
    mut word: *const i8,
) -> *const command_t {
    unsafe {
        let mut cmd: *const command_t = commands;
        while !cmd.is_null() {
            if command_match(cmd, word) {
                return cmd;
            }
            cmd = (*cmd).next;
        }
        return 0 as *const command_t;
    }
}

#[no_mangle]
pub extern "C" fn test(mut commands: *const command_t, mut input: *const i8) {
    unsafe {
        printf(b" input: %s\n\0" as *const u8 as *const i8, input);
        printf(b"output:\0" as *const u8 as *const i8);
        let mut count: u64 = 0;
        let mut words: *mut *mut i8 = split_into_words(input, &mut count);
        let mut i: u64 = 0;
        while i < count {
            let mut word: *mut i8 = *words.offset(i as isize);
            uppercase(word, strlen(word));
            let mut cmd_ptr: *const command_t = find_command(commands, word);
            if !cmd_ptr.is_null() {
                printf(
                    b" %s\0" as *const u8 as *const i8,
                    (*cmd_ptr).cmd as *const i8,
                )
            } else {
                printf(
                    b" %s\0" as *const u8 as *const i8,
                    b"*error*\0" as *const u8 as *const i8,
                )
            };
            free(word as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        free(words as *mut libc::c_void);
        printf(b"\n\0" as *const u8 as *const i8);
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut commands: *mut command_t = make_command_list(command_table);
        let mut input: *const i8 =
            b"riG   rePEAT copies  put mo   rest    types   fup.    6       poweRin\0" as *const u8
                as *const i8;
        test(commands, input);
        free_command_list(commands);
        return 0;
    }
}

pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32);
    }
}
