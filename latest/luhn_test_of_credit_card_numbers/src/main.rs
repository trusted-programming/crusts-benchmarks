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
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn luhn(mut cc: *const i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let m: [i32; 10] = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
        let mut i: i32 = 0;
        let mut odd: i32 = 1;
        let mut sum: i32 = 0;
        i = strlen(cc) as i32;
        loop {
            let fresh0 = i;
            i -= 1_i32;
            if fresh0 == 0_i32 {
                break;
            }
            let mut digit: i32 = i32::from(*cc.offset(i as isize)) - '0' as i32;
            sum += if odd != 0_i32 { digit } else { m[digit as usize] };
            odd = i32::from(odd == 0_i32);
        }
        i32::from(sum % 10_i32 == 0i32)
    }
}

fn main_0() -> i32 {
    let mut cc: [*const i8; 5] = [
        (b"49927398716\0" as *const u8).cast::<i8>(),
        (b"49927398717\0" as *const u8).cast::<i8>(),
        (b"1234567812345678\0" as *const u8).cast::<i8>(),
        (b"1234567812345670\0" as *const u8).cast::<i8>(),
        std::ptr::null::<i8>(),
    ];
    let mut i: i32 = 0;
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while !(cc[i as usize]).is_null() {
            if luhn(cc[i as usize]) != 0_i32 {
                println!(
                    "{:16}	ok\0",
                    build_str_from_raw_ptr(cc[i as usize] as *mut u8)
                )
            } else {
                println!(
                    "{:16}	not ok\0",
                    build_str_from_raw_ptr(cc[i as usize] as *mut u8)
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
