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
            str_size = str_size.wrapping_add(1);
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    fn atoi(__nptr: *const i8) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct functionPair {
    pub x: i32,
// SAFETY: machine generated unsafe code
    pub funcPtr: Option<unsafe extern "C" fn(i32) -> i32>,
}
#[no_mangle]
pub extern "C" fn factorial(mut num: i32) -> i32 {
    if num == 0_i32 || num == 1_i32 {
        1_i32
    } else {
        num * factorial(num.wrapping_sub(1))
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut response: functionPair = functionPair {
            x: 0,
            funcPtr: None,
        };
        if argc != 2_i32 {
            let str_to_print = format!(
                "Usage : {} <non negative integer>",
                build_str_from_raw_ptr((*argv.offset(0_isize)).cast::<u8>())
            );
            print!("{}", str_to_print);
            return str_to_print.chars().count() as i32;
        } else {
            response = {
                
                functionPair {
                    x: atoi(*argv.offset(1_isize)),
// SAFETY: machine generated unsafe code
                    funcPtr: Some(factorial as unsafe extern "C" fn(i32) -> i32),
                }
            };
            match response.funcPtr {
                Some(temp_m) => print!("\nFactorial of {} is {}\n", response.x, temp_m(response.x)),
                None => panic!("non-null function pointer"),
            }
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
