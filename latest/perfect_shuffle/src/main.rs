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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
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
pub static mut kDecks: [i32; 7] = [8_i32, 24_i32, 52_i32, 100_i32, 1_020_i32, 1_024_i32, 10_000_i32];
fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut nCards: i32 = 0;
        let mut nShuffles: i32 = 0;
        let mut deck: *mut i32 = std::ptr::null_mut::<i32>();
        i = 0_i32;
        while i < 7_i32 {
            nCards = kDecks[i as usize];
            if CreateDeck(&mut deck, nCards) == 0_i32 {
                fprintf(
                    stderr,
                    (b"Error: malloc() failed!\n\0" as *const u8).cast::<i8>(),
                );
                return 1_i32;
            }
            InitDeck(deck, nCards);
            nShuffles = 0_i32;
            loop {
                ShuffleDeck(deck, nCards);
                nShuffles = nShuffles.wrapping_add(1);
                nShuffles;
                if InitedDeck(deck, nCards) != 0_i32 {
                    break;
                }
            }
            println!(
                "Cards count: {}, shuffles required: {}.",
                nCards, nShuffles
            );
            FreeDeck(&mut deck);
            i = i.wrapping_add(1);
            i;
        }
        0_i32
    }
}

#[no_mangle]
pub extern "C" fn CreateDeck(mut deck: *mut *mut i32, mut nCards: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut tmp: *mut i32 = std::ptr::null_mut::<i32>();
        if !deck.is_null() {
            tmp = malloc((nCards as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64)).cast::<i32>();
        }
        if !tmp.is_null() {
            *deck = tmp;
            i32::from(*deck != std::ptr::null_mut::<libc::c_void>().cast::<i32>())
        } else {
            0_i32
        }
    }
}

#[no_mangle]
pub extern "C" fn InitDeck(mut deck: *mut i32, mut nCards: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        if !deck.is_null() {
            let mut i: i32 = 0;
            i = 0_i32;
            while i < nCards {
                *deck.offset(i as isize) = i;
                i = i.wrapping_add(1);
                i;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn DuplicateDeck(
    mut dest: *mut *mut i32,
    mut orig: *const i32,
    mut nCards: i32,
) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if !orig.is_null() && CreateDeck(dest, nCards) != 0_i32 {
            memcpy(
                (*dest).cast::<libc::c_void>(),
                orig.cast::<libc::c_void>(),
                (nCards as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
            );
            1_i32
        } else {
            0_i32
        }
    }
}

#[no_mangle]
pub extern "C" fn InitedDeck(mut deck: *mut i32, mut nCards: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        i = 0_i32;
        while i < nCards {
            if *deck.offset(i as isize) != i {
                return 0_i32;
            }
            i = i.wrapping_add(1);
            i;
        }
        1_i32
    }
}

#[no_mangle]
pub extern "C" fn ShuffleDeck(mut deck: *mut i32, mut nCards: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut copy: *mut i32 = std::ptr::null_mut::<i32>();
        if DuplicateDeck(&mut copy, deck, nCards) != 0_i32 {
            let mut i: i32 = 0;
            let mut j: i32 = 0;
            j = 0_i32;
            i = j;
            while i < nCards.wrapping_div(2) {
                *deck.offset(j as isize) = *copy.offset(i as isize);
                *deck.offset((j.wrapping_add(1i32)) as isize) =
                    *copy.offset((i + nCards.wrapping_div(2i32)) as isize);
                i = i.wrapping_add(1);
                i;
                j = j.wrapping_add(2);
            }
            FreeDeck(&mut copy);
            1_i32
        } else {
            0_i32
        }
    }
}

#[no_mangle]
pub extern "C" fn FreeDeck(mut deck: *mut *mut i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        if !(*deck).is_null() {
            free((*deck).cast::<libc::c_void>());
            *deck = std::ptr::null_mut::<i32>();
        }
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
