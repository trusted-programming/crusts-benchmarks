#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
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
    fn strlen(_: *const i8) -> u64;
    fn malloc(_: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct positionList {
    pub position: i32,
    pub next: *mut positionList,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct letterList {
    pub letter: i8,
    pub repititions: i32,
    pub positions: *mut positionList,
    pub next: *mut letterList,
}
#[no_mangle]
pub static mut letterSet: *mut letterList = 0 as *const letterList as *mut letterList;
#[no_mangle]
pub static mut duplicatesFound: bool = 0_i32 != 0_i32;
#[no_mangle]
pub extern "C" fn checkAndUpdateLetterList(mut c: i8, mut pos: i32) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut letterOccurs: bool = 0 != 0;
        let mut letterIterator: *mut letterList = std::ptr::null_mut::<letterList>();
        let mut newLetter: *mut letterList = std::ptr::null_mut::<letterList>();
        let mut positionIterator: *mut positionList = std::ptr::null_mut::<positionList>();
        let mut newPosition: *mut positionList = std::ptr::null_mut::<positionList>();
        if letterSet.is_null() {
            letterSet = malloc(::core::mem::size_of::<letterList>() as u64).cast::<letterList>();
            (*letterSet).letter = c;
            (*letterSet).repititions = 0_i32;
            (*letterSet).positions =
                malloc(::core::mem::size_of::<positionList>() as u64).cast::<positionList>();
            (*(*letterSet).positions).position = pos;
            (*(*letterSet).positions).next = std::ptr::null_mut::<positionList>();
            (*letterSet).next = std::ptr::null_mut::<letterList>();
        } else {
            letterIterator = letterSet;
            while !letterIterator.is_null() {
                if i32::from((*letterIterator).letter) == i32::from(c) {
                    letterOccurs = 1_i32 != 0_i32;
                    duplicatesFound = 1_i32 != 0_i32;
                    (*letterIterator).repititions += 1_i32;
                    (*letterIterator).repititions;
                    positionIterator = (*letterIterator).positions;
                    while !((*positionIterator).next).is_null() {
                        positionIterator = (*positionIterator).next;
                    }
                    newPosition =
                        malloc(::core::mem::size_of::<positionList>() as u64).cast::<positionList>();
                    (*newPosition).position = pos;
                    (*newPosition).next = std::ptr::null_mut::<positionList>();
                    (*positionIterator).next = newPosition;
                }
                if i32::from(letterOccurs) == 0_i32 && ((*letterIterator).next).is_null() {
                    break;
                }
                letterIterator = (*letterIterator).next;
            }
            if i32::from(letterOccurs) == 0_i32 {
                newLetter = malloc(::core::mem::size_of::<letterList>() as u64).cast::<letterList>();
                (*newLetter).letter = c;
                (*newLetter).repititions = 0_i32;
                (*newLetter).positions =
                    malloc(::core::mem::size_of::<positionList>() as u64).cast::<positionList>();
                (*(*newLetter).positions).position = pos;
                (*(*newLetter).positions).next = std::ptr::null_mut::<positionList>();
                (*newLetter).next = std::ptr::null_mut::<letterList>();
                (*letterIterator).next = newLetter;
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn printLetterList() {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut positionIterator: *mut positionList = std::ptr::null_mut::<positionList>();
        let mut letterIterator: *mut letterList = letterSet;
        while !letterIterator.is_null() {
            if (*letterIterator).repititions > 0_i32 {
                print!(
                    "\n{} (0x{:x}) at positions :",
                    i32::from((*letterIterator).letter),
                    i32::from((*letterIterator).letter)
                );
                positionIterator = (*letterIterator).positions;
                while !positionIterator.is_null() {
                    print!("{:3}", (*positionIterator).position + 1_i32);
                    positionIterator = (*positionIterator).next;
                }
            }
            letterIterator = (*letterIterator).next;
        }
        println!();
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut len: i32 = 0;
        if argc > 2_i32 {
            println!(
                "Usage : {} <Test string>",
                build_str_from_raw_ptr((*argv.offset(0_isize)).cast::<u8>())
            );
            return 0_i32;
        }
        if argc == 1_i32 || strlen(*argv.offset(1_isize)) == 1 {
            if argc == 1_i32 {
                if argc == 1_i32 {
                    println!(
                        "\"\0\" - Length {} - Contains only unique characters.", 0_i32
                    )
                } else {
                    println!(
                        "\"\0\" - Length {} - Contains only unique characters.", 1_i32
                    )
                }
            } else if argc == 1_i32 {
                println!(
                    "\"{}\" - Length {} - Contains only unique characters.",
                    build_str_from_raw_ptr(*argv.offset(1_isize) as *const i8 as *mut u8),
                    0_i32
                )
            } else {
                println!(
                    "\"{}\" - Length {} - Contains only unique characters.",
                    build_str_from_raw_ptr(*argv.offset(1_isize) as *const i8 as *mut u8),
                    1_i32
                )
            };
            return 0_i32;
        }
        len = strlen(*argv.offset(1_isize)) as i32;
        i = 0_i32;
        while i < len {
            checkAndUpdateLetterList(*(*argv.offset(1_isize)).offset(i as isize), i);
            i = i.wrapping_add(1);
            i;
        }
        if i32::from(duplicatesFound) == 0_i32 {
            print!(
                "\"{}\" - Length {} - Contains only unique characters.\n\0",
                build_str_from_raw_ptr((*argv.offset(1_isize)).cast::<u8>()),
                len
            )
        } else {
            print!(
                "\"{}\" - Length {} - Contains the following duplicate characters :\0",
                build_str_from_raw_ptr((*argv.offset(1_isize)).cast::<u8>()),
                len
            )
        };
        if i32::from(duplicatesFound) == 1_i32 {
            printLetterList();
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
