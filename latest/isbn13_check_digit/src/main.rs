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


extern "C" {}
#[no_mangle]
pub extern "C" fn check_isbn13(mut isbn: *const i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut ch: i32 = i32::from(*isbn);
        let mut count: i32 = 0;
        let mut sum: i32 = 0;
        while ch != 0_i32 {
            if ch == ' ' as i32 || ch == '-' as i32 {
                count -= 1_i32;
                count;
            } else {
                if ch < '0' as i32 || ch > '9' as i32 {
                    return 0_i32;
                }
                if count & 1_i32 != 0_i32 {
                    sum += 3_i32 * (ch - '0' as i32);
                } else {
                    sum += ch - '0' as i32;
                }
            }
            isbn = isbn.offset(1);
            ch = i32::from(*isbn);
            count += 1_i32;
            count;
        }
        if count != 13_i32 {
            return 0_i32;
        }
        i32::from(sum % 10_i32 == 0_i32)
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut isbns: [*const i8; 4] = [
        (b"978-1734314502\0" as *const u8).cast::<i8>(),
        (b"978-1734314509\0" as *const u8).cast::<i8>(),
        (b"978-1788399081\0" as *const u8).cast::<i8>(),
        (b"978-1788399083\0" as *const u8).cast::<i8>(),
    ];
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 4_i32 {
            if check_isbn13(isbns[i as usize]) != 0_i32 {
                println!(
                    "{}: good\0",
                    build_str_from_raw_ptr(isbns[i as usize] as *mut u8)
                )
            } else {
                println!(
                    "{}: bad\0",
                    build_str_from_raw_ptr(isbns[i as usize] as *mut u8)
                )
            };
            i += 1_i32;
            i;
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
