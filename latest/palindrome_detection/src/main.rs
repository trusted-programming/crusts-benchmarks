#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {
    fn strlen(_: *const i8) -> u64;
    fn printf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub extern "C" fn palindrome(mut s: *const i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut l: i32 = 0;
        l = strlen(s) as i32;
        i = 0_i32;
        while i < l.wrapping_div(2) {
            if i32::from(*s.offset(i as isize)) != i32::from(*s.offset((l - i.wrapping_sub(1i32)) as isize))
            {
                return 0_i32;
            }
            i = i.wrapping_add(1);
            i;
        }
        1_i32
    }
}

#[no_mangle]
pub extern "C" fn palindrome_r(mut s: *const i8, mut b: i32, mut e: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if e - 1_i32 <= b {
            return 1_i32;
        }
        if i32::from(*s.offset(b as isize)) != i32::from(*s.offset((e.wrapping_sub(1i32)) as isize)) {
            return 0_i32;
        }
        palindrome_r(s, b.wrapping_add(1), e.wrapping_sub(1))
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut t: *const i8 = (b"ingirumimusnocteetconsumimurigni\0" as *const u8).cast::<i8>();
        let mut template: *const i8 =
            (b"sequence \"%s\" is%s palindrome\n\0" as *const u8).cast::<i8>();
        let mut l: i32 = strlen(t) as i32;
        if palindrome(t) != 0_i32 {
            printf(template, t, (b"\0" as *const u8).cast::<i8>())
        } else {
            printf(template, t, (b"n't\0" as *const u8).cast::<i8>())
        };
        if palindrome_r(t, 0, l) != 0_i32 {
            printf(template, t, (b"\0" as *const u8).cast::<i8>())
        } else {
            printf(template, t, (b"n't\0" as *const u8).cast::<i8>())
        };
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
