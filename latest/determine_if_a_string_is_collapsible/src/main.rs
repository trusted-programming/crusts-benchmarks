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
    fn free(_: *mut libc::c_void);
    fn scanf(_: *const i8, _: ...) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct charList {
    pub c: i8,
    pub next: *mut charList,
}
#[no_mangle]
pub extern "C" fn strcmpi(mut str1: *mut i8, mut str2: *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut len1: i32 = strlen(str1) as i32;
        let mut len2: i32 = strlen(str2) as i32;
        let mut i: i32 = 0;
        if len1 != len2 {
            return 1_i32;
        } else {
            i = 0_i32;
            while i < len1 {
                if i32::from(*str1.offset(i as isize)) >= 'A' as i32
                    && i32::from(*str1.offset(i as isize)) <= 'Z' as i32
                    && (i32::from(*str2.offset(i as isize)) >= 'a' as i32
                        && i32::from(*str2.offset(i as isize)) <= 'z' as i32)
                    && i32::from(*str2.offset(i as isize)) - 65_i32 != i32::from(*str1.offset(i as isize))
                {
                    return 1_i32;
                } else if i32::from(*str2.offset(i as isize)) >= 'A' as i32
                    && i32::from(*str2.offset(i as isize)) <= 'Z' as i32
                    && (i32::from(*str1.offset(i as isize)) >= 'a' as i32
                        && i32::from(*str1.offset(i as isize)) <= 'z' as i32)
                    && i32::from(*str1.offset(i as isize)) - 65_i32 != i32::from(*str2.offset(i as isize))
                {
                    return 1_i32;
                } else if i32::from(*str1.offset(i as isize)) != i32::from(*str2.offset(i as isize)) {
                    return 1_i32;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        0_i32
    }
}

#[no_mangle]
pub extern "C" fn strToCharList(mut str: *mut i8) -> *mut charList {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut len: i32 = strlen(str) as i32;
        let mut i: i32 = 0;
        let mut list: *mut charList = std::ptr::null_mut::<charList>();
        let mut iterator: *mut charList = std::ptr::null_mut::<charList>();
        let mut nextChar: *mut charList = std::ptr::null_mut::<charList>();
        list = malloc(::core::mem::size_of::<charList>() as u64).cast::<charList>();
        (*list).c = *str.offset(0_isize);
        (*list).next = std::ptr::null_mut::<charList>();
        iterator = list;
        i = 1_i32;
        while i < len {
            nextChar = malloc(::core::mem::size_of::<charList>() as u64).cast::<charList>();
            (*nextChar).c = *str.offset(i as isize);
            (*nextChar).next = std::ptr::null_mut::<charList>();
            (*iterator).next = nextChar;
            iterator = nextChar;
            i = i.wrapping_add(1);
            i;
        }
        list
    }
}

#[no_mangle]
pub extern "C" fn charListToString(mut list: *mut charList) -> *mut i8 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut iterator: *mut charList = list;
        let mut count: i32 = 0;
        let mut i: i32 = 0;
        let mut str: *mut i8 = std::ptr::null_mut::<i8>();
        while !iterator.is_null() {
            count = count.wrapping_add(1);
            count;
            iterator = (*iterator).next;
        }
        str = malloc(((count + 1i32) as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64)).cast::<i8>();
        iterator = list;
        i = 0_i32;
        while i < count {
            *str.offset(i as isize) = (*iterator).c;
            iterator = (*iterator).next;
            i = i.wrapping_add(1);
            i;
        }
        free(list.cast::<libc::c_void>());
        *str.offset(i as isize) = '\0' as i8;
        str
    }
}

#[no_mangle]
pub extern "C" fn processString(
    mut str: *mut i8,
    mut operation: i32,
    mut squeezeChar: i8,
) -> *mut i8 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut strList: *mut charList = strToCharList(str);
        let mut iterator: *mut charList = strList;
        let mut scout: *mut charList = std::ptr::null_mut::<charList>();
        if operation == 1_i32 {
            while !iterator.is_null() {
                if i32::from((*iterator).c) == i32::from(squeezeChar) {
                    scout = (*iterator).next;
                    while !scout.is_null() && i32::from((*scout).c) == i32::from(squeezeChar) {
                        (*iterator).next = (*scout).next;
                        (*scout).next = std::ptr::null_mut::<charList>();
                        free(scout.cast::<libc::c_void>());
                        scout = (*iterator).next;
                    }
                }
                iterator = (*iterator).next;
            }
        } else {
            while !iterator.is_null() && !((*iterator).next).is_null() {
                if i32::from((*iterator).c) == i32::from((*(*iterator).next).c) {
                    scout = (*iterator).next;
                    squeezeChar = (*iterator).c;
                    while !scout.is_null() && i32::from((*scout).c) == i32::from(squeezeChar) {
                        (*iterator).next = (*scout).next;
                        (*scout).next = std::ptr::null_mut::<charList>();
                        free(scout.cast::<libc::c_void>());
                        scout = (*iterator).next;
                    }
                }
                iterator = (*iterator).next;
            }
        }
        charListToString(strList)
    }
}

#[no_mangle]
pub extern "C" fn printResults(
    mut originalString: *mut i8,
    mut finalString: *mut i8,
    mut operation: i32,
    mut squeezeChar: i8,
) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        if operation == 1_i32 {
            print!(
                "Specified Operation : SQUEEZE\nTarget Character : {}",
                i32::from(squeezeChar)
            );
        } else {
            print!("Specified Operation : COLLAPSE");
        }
        print!(
            "\nOriginal {}{}{}{}{}{}{}\nLength : {}",
            174_i32,
            174_i32,
            174_i32,
            build_str_from_raw_ptr(originalString.cast::<u8>()),
            175_i32,
            175_i32,
            175_i32,
            strlen(originalString as *const i8) as i32
        );
        print!(
            "\nFinal    {}{}{}{}{}{}{}\nLength : {}\n",
            174_i32,
            174_i32,
            174_i32,
            build_str_from_raw_ptr(finalString.cast::<u8>()),
            175_i32,
            175_i32,
            175_i32,
            strlen(finalString as *const i8) as i32
        );
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut operation: i32 = 0;
        let mut squeezeChar: i8 = 0;
        if !(3_i32..=4_i32).contains(&argc) {
            println! ("Usage : {} <SQUEEZE|COLLAPSE> <String to be processed> <Character to be squeezed, if operation is SQUEEZE>", build_str_from_raw_ptr ((* argv.offset (0_isize)).cast::<u8>()));
            return 0_i32;
        }
        if strcmpi(
            *argv.offset(1_isize),
            (b"SQUEEZE\0" as *const u8).cast::<i8>() as *mut i8,
        ) == 0_i32
            && argc != 4_i32
        {
            scanf(
                (b"Please enter characted to be squeezed : %c\0" as *const u8).cast::<i8>(),
                &mut squeezeChar as *mut i8,
            );
            operation = 1_i32;
        } else if argc == 4_i32 {
            operation = 1_i32;
            squeezeChar = *(*argv.offset(3_isize)).offset(0_isize);
        } else if strcmpi(
            *argv.offset(1_isize),
            (b"COLLAPSE\0" as *const u8).cast::<i8>() as *mut i8,
        ) == 0_i32
        {
            operation = 0_i32;
        }
        if strlen(*argv.offset(2_isize)) < 2 {
            printResults(
                *argv.offset(2_isize),
                *argv.offset(2_isize),
                operation,
                squeezeChar,
            );
        } else {
            printResults(
                *argv.offset(2_isize),
                processString(*argv.offset(2_isize), operation, squeezeChar),
                operation,
                squeezeChar,
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
