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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn log2(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn print_headings() {
    print!("{:2}", "N\0");
    print!(" {:10}", "Length\0");
    print!(" {:-20}", "Entropy\0");
    print!(" {:-40}", "Word\0");
    println!();
}

#[no_mangle]
pub extern "C" fn calculate_entropy(mut ones: i32, mut zeros: i32) -> f64 {
    let mut result: f64 = f64::from(0_i32);
    let mut total: i32 = ones + zeros;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        result -= f64::from(ones) / f64::from(total) * log2(f64::from(ones) / f64::from(total));
        result -= f64::from(zeros) / f64::from(total) * log2(f64::from(zeros) / f64::from(total));
    }
    if result != result {
        result = f64::from(0_i32);
    }
    result
}

#[no_mangle]
pub extern "C" fn print_entropy(mut word: *mut i8) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut ones: i32 = 0;
        let mut zeros: i32 = 0;
        let mut i: i32 = 0;
        i = 0_i32;
        while *word.offset(i as isize) != 0 {
            let mut c: i8 = *word.offset(i as isize);
            match i32::from(c) {
                48_i32 => {
                    zeros = zeros.wrapping_add(1);
                    zeros;
                }
                49_i32 => {
                    ones = ones.wrapping_add(1);
                    ones;
                }
                _ => {}
            }
            i = i.wrapping_add(1);
            i;
        }
        let mut entropy: f64 = calculate_entropy(ones, zeros);
        print!(" {:-20.18}", entropy);
    }
}

#[no_mangle]
pub extern "C" fn print_word(mut n: i32, mut word: *mut i8) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        print!("{:2}", n);
        print!(" {:10}", strlen(word));
        print_entropy(word);
        if n < 10_i32 {
            print!(" {:-40}", build_str_from_raw_ptr(word.cast::<u8>()));
        } else {
            print!(" {:-40}", "...\0");
        }
        println!();
    }
}

fn main_0(mut _argc: i32, mut _argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        print_headings();
        let mut last_word: *mut i8 = malloc(2).cast::<i8>();
        strcpy(last_word, (b"1\0" as *const u8).cast::<i8>());
        let mut current_word: *mut i8 = malloc(2).cast::<i8>();
        strcpy(current_word, (b"0\0" as *const u8).cast::<i8>());
        print_word(1, last_word);
        let mut i: i32 = 0;
        i = 2_i32;
        while i <= 37_i32 {
            print_word(i, current_word);
            let mut next_word: *mut i8 = malloc(
                (strlen(current_word))
                    .wrapping_add(strlen(last_word))
                    .wrapping_add(1),
            ).cast::<i8>();
            strcpy(next_word, current_word);
            strcat(next_word, last_word);
            free(last_word.cast::<libc::c_void>());
            last_word = current_word;
            current_word = next_word;
            i = i.wrapping_add(1);
            i;
        }
        free(last_word.cast::<libc::c_void>());
        free(current_word.cast::<libc::c_void>());
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
