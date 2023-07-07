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
    fn strlen(_: *const i8) -> u64;
}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut len: i32 = 0;
        let mut reference: i8 = 0;
        if argc > 2_i32 {
            println!(
                "Usage : {} <Test String>",
                build_str_from_raw_ptr((*argv.offset(0_isize)).cast::<u8>())
            );
            return 0_i32;
        }
        if argc == 1_i32 || strlen(*argv.offset(1_isize)) == 1 {
            if argc == 1_i32 {
                if argc == 1_i32 {
                    print!(
                        "Input string : \"\0\"\nLength : {}\nAll characters are identical.\n", 0_i32
                    )
                } else {
                    print!(
                        "Input string : \"\0\"\nLength : {}\nAll characters are identical.\n",
                        strlen(*argv.offset(1_isize)) as i32
                    )
                }
            } else if argc == 1_i32 {
                print!(
                    "Input string : \"{}\"\nLength : {}\nAll characters are identical.\n",
                    build_str_from_raw_ptr(*argv.offset(1_isize) as *const i8 as *mut u8),
                    0_i32
                )
            } else {
                print!(
                    "Input string : \"{}\"\nLength : {}\nAll characters are identical.\n",
                    build_str_from_raw_ptr(*argv.offset(1_isize) as *const i8 as *mut u8),
                    strlen(*argv.offset(1_isize)) as i32
                )
            };
            return 0_i32;
        }
        reference = *(*argv.offset(1_isize)).offset(0_isize);
        len = strlen(*argv.offset(1_isize)) as i32;
        i = 1_i32;
        while i < len {
            if i32::from(*(*argv.offset(1_isize)).offset(i as isize)) != i32::from(reference) {
                print! ("Input string : \"{}\"\nLength : {}\nFirst different character : \"{}\"(0x{:x}) at position : {}\n", build_str_from_raw_ptr ((* argv.offset (1_isize)).cast::<u8>()), len, i32::from(* (* argv.offset (1_isize)).offset (i as isize)), i32::from(*
                  (* argv.offset (1_isize)).offset (i as isize)), i + 1_i32);
                return 0_i32;
            }
            i = i.wrapping_add(1);
            i;
        }
        print!(
            "Input string : \"{}\"\nLength : {}\nAll characters are identical.\n",
            build_str_from_raw_ptr((*argv.offset(1_isize)).cast::<u8>()),
            len
        );
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
