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
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
}
pub type __compar_fn_t =
// SAFETY: machine generated unsafe code
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct replace_info {
    pub n: i32,
    pub text: *mut i8,
}
#[no_mangle]
pub extern "C" fn compare(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut x: *mut replace_info = a as *mut replace_info;
        let mut y: *mut replace_info = b as *mut replace_info;
        (*x).n - (*y).n
    }
}

#[no_mangle]
pub extern "C" fn generic_fizz_buzz(
    mut max: i32,
    mut info: *mut replace_info,
    mut info_length: i32,
) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut it: i32 = 0;
        let mut found_word: i32 = 0;
        i = 1_i32;
        while i < max {
            found_word = 0_i32;
            it = 0_i32;
            while it < info_length {
                if 0_i32 == i % (*info.offset(it as isize)).n {
                    print!(
                        "{}",
                        build_str_from_raw_ptr((*info.offset(it as isize)).text.cast::<u8>())
                    );
                    found_word = 1_i32;
                }
                it += 1_i32;
                it;
            }
            if 0_i32 == found_word {
                print!("{}", i);
            }
            println!();
            i += 1_i32;
            i;
        }
    }
}

fn main_0() -> i32 {
    let mut info: [replace_info; 3] = [
        {
            
            replace_info {
                n: 5,
                text: (b"Buzz\0" as *const u8).cast::<i8>() as *mut i8,
            }
        },
        {
            
            replace_info {
                n: 7,
                text: (b"Baxx\0" as *const u8).cast::<i8>() as *mut i8,
            }
        },
        {
            
            replace_info {
                n: 3,
                text: (b"Fizz\0" as *const u8).cast::<i8>() as *mut i8,
            }
        },
    ];
// SAFETY: machine generated unsafe code
    unsafe {
        qsort(
            info.as_mut_ptr().cast::<libc::c_void>(),
            3,
            ::core::mem::size_of::<replace_info>() as u64,
// SAFETY: machine generated unsafe code
            Some(compare as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
        );
    }
    generic_fizz_buzz(20, info.as_mut_ptr(), 3);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
