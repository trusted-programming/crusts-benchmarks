#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
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
pub type TWord = [i8; 100];
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct Node {
    pub word: TWord,
    pub next: *mut Node,
}
#[no_mangle]
pub extern "C" fn is_ordered_word(mut word: *const i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if !word.is_null() {
        } else {
            __assert_fail(
                (b"word != NULL\0" as *const u8).cast::<i8>(),
                (b"main.c\0" as *const u8).cast::<i8>(),
                18,
                (*::core::mem::transmute::<&[u8; 34], &[i8; 34]>(
                    b"int is_ordered_word(const char *)\0",
                ))
                .as_ptr(),
            );
        }
        if !word.is_null() {
            } else {
                __assert_fail(
                    (b"word != NULL\0" as *const u8).cast::<i8>(),
                    (b"main.c\0" as *const u8).cast::<i8>(),
                    18,
                    (*::core::mem::transmute::<&[u8; 34], &[i8; 34]>(
                        b"int is_ordered_word(const char *)\0",
                    ))
                    .as_ptr(),
                );
            };
        let mut i: i32 = 0;
        i = 0_i32;
        while i32::from(*word.offset(i as isize)) != '\0' as i32 {
            if i32::from(*word.offset(i as isize)) > i32::from(*word.offset((i + 1i32) as isize))
                && i32::from(*word.offset((i + 1i32) as isize)) != '\0' as i32
            {
                return 0_i32;
            }
            i += 1_i32;
            i;
        }
        1_i32
    }
}

#[no_mangle]
pub extern "C" fn list_prepend(mut words_list: *mut Node, mut new_word: *const i8) -> *mut Node {
// SAFETY: machine generated unsafe code
    unsafe {
        if !new_word.is_null() {
        } else {
            __assert_fail(
                (b"new_word != NULL\0" as *const u8).cast::<i8>(),
                (b"main.c\0" as *const u8).cast::<i8>(),
                30,
                (*::core::mem::transmute::<&[u8; 41], &[i8; 41]>(
                    b"Node *list_prepend(Node *, const char *)\0",
                ))
                .as_ptr(),
            );
        }
        if !new_word.is_null() {
            } else {
                __assert_fail(
                    (b"new_word != NULL\0" as *const u8).cast::<i8>(),
                    (b"main.c\0" as *const u8).cast::<i8>(),
                    30,
                    (*::core::mem::transmute::<&[u8; 41], &[i8; 41]>(
                        b"Node *list_prepend(Node *, const char *)\0",
                    ))
                    .as_ptr(),
                );
            };
        let mut new_node: *mut Node = malloc(::core::mem::size_of::<Node>() as u64).cast::<Node>();
        if new_node.is_null() {
            exit(1);
        }
        strcpy(((*new_node).word).as_mut_ptr(), new_word);
        (*new_node).next = words_list;
        new_node
    }
}

#[no_mangle]
pub extern "C" fn list_destroy(mut words_list: *mut Node) -> *mut Node {
// SAFETY: machine generated unsafe code
    unsafe {
        while !words_list.is_null() {
            let mut temp: *mut Node = words_list;
            words_list = (*words_list).next;
            free(temp.cast::<libc::c_void>());
        }
        words_list
    }
}

#[no_mangle]
pub extern "C" fn list_print(mut words_list: *mut Node) {
// SAFETY: machine generated unsafe code
    unsafe {
        while !words_list.is_null() {
            print!(
                "\n{}",
                build_str_from_raw_ptr(((*words_list).word).as_mut_ptr().cast::<u8>())
            );
            words_list = (*words_list).next;
        }
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut fp: *mut FILE = fopen(
            (b"unixdict.txt\0" as *const u8).cast::<i8>(),
            (b"r\0" as *const u8).cast::<i8>(),
        );
        if fp.is_null() {
            return 1_i32;
        }
        let mut words: *mut Node = std::ptr::null_mut::<Node>();
        let mut line: TWord = [0; 100];
        let mut max_len: u32 = 0;
        while fscanf(fp, (b"%99s\n\0" as *const u8).cast::<i8>(), line.as_mut_ptr()) != -1_i32 {
            if strlen(line.as_mut_ptr()) > u64::from(max_len)
                && is_ordered_word(line.as_mut_ptr() as *const i8) != 0_i32
            {
                max_len = strlen(line.as_mut_ptr()) as u32;
                words = list_destroy(words);
                words = list_prepend(words, line.as_mut_ptr() as *const i8);
            } else if strlen(line.as_mut_ptr()) == u64::from(max_len)
                && is_ordered_word(line.as_mut_ptr() as *const i8) != 0_i32
            {
                words = list_prepend(words, line.as_mut_ptr() as *const i8);
            }
        }
        fclose(fp);
        list_print(words);
        0_i32
    }
}

pub fn main() {
// SAFETY: machine generated unsafe code
    unsafe {
        ::std::process::exit(main_0());
    }
}
