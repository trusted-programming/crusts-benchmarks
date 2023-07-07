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
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
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
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct sDictWord {
    pub word: *const i8,
    pub next: DictWord,
}
pub type DictWord = *mut sDictWord;
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct sHashEntry {
    pub key: *const i8,
    pub next: HashEntry,
    pub words: DictWord,
    pub link: HashEntry,
    pub wordCount: i16,
}
pub type HashEntry = *mut sHashEntry;
#[no_mangle]
pub extern "C" fn sortedWord(mut word: *const i8, mut wbuf: *mut i8) -> *mut i8 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut p1: *mut i8 = std::ptr::null_mut::<i8>();
        let mut p2: *mut i8 = std::ptr::null_mut::<i8>();
        let mut endwrd: *mut i8 = std::ptr::null_mut::<i8>();
        let mut t: i8 = 0;
        let mut swaps: i32 = 0;
        strcpy(wbuf, word);
        endwrd = wbuf.offset(strlen(wbuf) as isize);
        loop {
            swaps = 0_i32;
            p1 = wbuf;
            p2 = endwrd.offset(-1_isize);
            while p1 < p2 {
                if i32::from(*p2) > i32::from(*p1) {
                    t = *p2;
                    *p2 = *p1;
                    *p1 = t;
                    swaps = 1_i32;
                }
                p1 = p1.offset(1);
                p1;
                p2 = p2.offset(-1);
                p2;
            }
            p1 = wbuf;
            p2 = p1.offset(1_isize);
            while p2 < endwrd {
                if i32::from(*p2) > i32::from(*p1) {
                    t = *p2;
                    *p2 = *p1;
                    *p1 = t;
                    swaps = 1_i32;
                }
                p1 = p1.offset(1);
                p1;
                p2 = p2.offset(1);
                p2;
            }
            if swaps == 0_i32 {
                break;
            }
        }
        wbuf
    }
}

static mut cxmap: [i16; 96] = [
    0x6, 0x1f, 0x4d, 0xc, 0x5c, 0x28, 0x5d, 0xe, 0x9, 0x33, 0x31, 0x56, 0x52, 0x19, 0x29, 0x53,
    0x32, 0x48, 0x35, 0x55, 0x5e, 0x14, 0x27, 0x24, 0x2, 0x3e, 0x18, 0x4a, 0x3f, 0x4c, 0x45, 0x30,
    0x8, 0x2c, 0x1a, 0x3, 0xb, 0xd, 0x4f, 0x7, 0x20, 0x1d, 0x51, 0x3b, 0x11, 0x58, 0, 0x49, 0x15,
    0x2d, 0x41, 0x17, 0x5f, 0x39, 0x16, 0x42, 0x37, 0x22, 0x1c, 0xf, 0x43, 0x5b, 0x46, 0x4b, 0xa,
    0x26, 0x2e, 0x40, 0x12, 0x21, 0x3c, 0x36, 0x38, 0x1e, 0x1, 0x1b, 0x5, 0x4e, 0x44, 0x3d, 0x4,
    0x10, 0x5a, 0x2a, 0x23, 0x34, 0x25, 0x2f, 0x2b, 0x50, 0x3a, 0x54, 0x47, 0x59, 0x13, 0x57,
];
#[no_mangle]
pub extern "C" fn Str_Hash(mut key: *const i8, mut ix_max: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut cp: *const i8 = std::ptr::null::<i8>();
        let mut mash: i16 = 0;
        let mut hash: i32 = 33501551;
        cp = key;
        while *cp != 0 {
            mash = cxmap[(*cp as u64).wrapping_rem(
                (::core::mem::size_of::<[i16; 96]>() as u64)
                    .wrapping_div(::core::mem::size_of::<i16>() as u64),
            ) as usize];
            hash = hash >> 4_i32 ^ 0x05c5_cf5c_i32 ^ ((hash << 1_i32) + (i32::from(mash) << 5_i32));
            hash &= 0x3fff_ffff_i32;
            cp = cp.offset(1);
            cp;
        }
        hash % ix_max
    }
}

#[no_mangle]
pub static mut hashTable: [HashEntry; 8192] = [0 as *const sHashEntry as *mut sHashEntry; 8192];
#[no_mangle]
pub static mut mostPerms: HashEntry = 0 as *const sHashEntry as HashEntry;
#[no_mangle]
pub extern "C" fn buildAnagrams(mut fin: *mut FILE) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut buffer: [i8; 40] = [0; 40];
        let mut bufr2: [i8; 40] = [0; 40];
        let mut hkey: *mut i8 = std::ptr::null_mut::<i8>();
        let mut hix: i32 = 0;
        let mut he: HashEntry = std::ptr::null_mut::<sHashEntry>();
        let mut hep: *mut HashEntry = std::ptr::null_mut::<HashEntry>();
        let mut we: DictWord = std::ptr::null_mut::<sDictWord>();
        let mut maxPC: i32 = 2;
        let mut numWords: i32 = 0;
        while !(fgets(buffer.as_mut_ptr(), 40, fin)).is_null() {
            hkey = buffer.as_mut_ptr();
            while i32::from(*hkey) != 0_i32 && i32::from(*hkey) != '\n' as i32 {
                hkey = hkey.offset(1);
                hkey;
            }
            *hkey = 0;
            hkey = sortedWord(buffer.as_mut_ptr(), bufr2.as_mut_ptr());
            hix = Str_Hash(hkey, 8192);
            he = hashTable[hix as usize];
            hep = &mut *hashTable.as_mut_ptr().offset(hix as isize) as *mut HashEntry;
            while !he.is_null() && strcmp((*he).key, hkey) != 0_i32 {
                hep = &mut (*he).next;
                he = (*he).next;
            }
            if he.is_null() {
                he = malloc(::core::mem::size_of::<sHashEntry>() as u64).cast::<sHashEntry>();
                (*he).next = 0 as HashEntry;
                (*he).key = strdup(hkey);
                (*he).wordCount = 0;
                (*he).words = 0 as DictWord;
                (*he).link = 0 as HashEntry;
                *hep = he;
            }
            we = malloc(::core::mem::size_of::<sDictWord>() as u64).cast::<sDictWord>();
            (*we).word = strdup(buffer.as_mut_ptr());
            (*we).next = (*he).words;
            (*he).words = we;
            (*he).wordCount += 1;
            (*he).wordCount;
            if maxPC < i32::from((*he).wordCount) {
                maxPC = i32::from((*he).wordCount);
                mostPerms = he;
                (*he).link = 0 as HashEntry;
            } else if maxPC == i32::from((*he).wordCount) {
                (*he).link = mostPerms;
                mostPerms = he;
            }
            numWords = numWords.wrapping_add(1);
            numWords;
        }
        println!("{} words in dictionary max ana={}", numWords, maxPC);
        maxPC
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut he: HashEntry = std::ptr::null_mut::<sHashEntry>();
        let mut we: DictWord = std::ptr::null_mut::<sDictWord>();
        let mut f1: *mut FILE = std::ptr::null_mut::<FILE>();
        f1 = fopen(
            (b"unixdict.txt\0" as *const u8).cast::<i8>(),
            (b"r\0" as *const u8).cast::<i8>(),
        );
        buildAnagrams(f1);
        fclose(f1);
        f1 = fopen(
            (b"anaout.txt\0" as *const u8).cast::<i8>(),
            (b"w\0" as *const u8).cast::<i8>(),
        );
        he = mostPerms;
        while !he.is_null() {
            fprintf(
                f1,
                (b"%d:\0" as *const u8).cast::<i8>(),
                i32::from((*he).wordCount),
            );
            we = (*he).words;
            while !we.is_null() {
                fprintf(f1, (b"%s, \0" as *const u8).cast::<i8>(), (*we).word);
                we = (*we).next;
            }
            fprintf(f1, (b"\n\0" as *const u8).cast::<i8>());
            he = (*he).link;
        }
        fclose(f1);
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
