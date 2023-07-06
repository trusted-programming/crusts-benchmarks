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
    fn strchr(_: *const i8, _: i32) -> *mut i8;
}
#[no_mangle]
pub extern "C" fn is_pangram(mut s: *const i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut alpha: *const i8 =
            (b"abcdefghjiklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ\0" as *const u8).cast::<i8>();
        let mut ch: i8 = 0;
        let mut wasused: [i8; 26] = [0; 26];
        let mut total: i32 = 0;
        loop {
            let fresh0 = s;
            s = s.offset(1);
            ch = *fresh0;
            if i32::from(ch) == '\0' as i32 {
                break;
            }
            let mut p: *const i8 = std::ptr::null::<i8>();
            let mut idx: i32 = 0;
            p = strchr(alpha, i32::from(ch));
            if p.is_null() {
                continue;
            }
            idx = (p.offset_from(alpha) as i64 % 26i64) as i32;
            total += i32::from(wasused[idx as usize] == 0);
            wasused[idx as usize] = 1;
            if total == 26_i32 {
                return 1_i32;
            }
        }
        0_i32
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut tests: [*const i8; 2] = [
        (b"The quick brown fox jumps over the lazy dog.\0" as *const u8).cast::<i8>(),
        (b"The qu1ck brown fox jumps over the lazy d0g.\0" as *const u8).cast::<i8>(),
    ];
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 2_i32 {
            if is_pangram(tests[i as usize]) != 0_i32 {
                println!(
                    "\"{}\" is \0a pangram",
                    build_str_from_raw_ptr(tests[i as usize] as *mut u8)
                )
            } else {
                println!(
                    "\"{}\" is not \0a pangram",
                    build_str_from_raw_ptr(tests[i as usize] as *mut u8)
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
