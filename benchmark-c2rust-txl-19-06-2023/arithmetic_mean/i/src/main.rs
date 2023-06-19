#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn printf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub extern "C" fn mean(mut v: *mut f64, mut len: i32) -> f64 {
    unsafe {
        let mut sum: f64 = 0 as f64;
        let mut i: i32 = 0;
        i = 0;
        while i < len {
            sum += *v.offset(i as isize);
            i += 1;
            i;
        }
        return sum / len as f64;
    }
}

fn main_0() -> i32 {
    let mut v: [f64; 5] = [1 as f64, 2 as f64, 2.718f64, 3 as f64, 3.142f64];
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    len = 5;
    unsafe {
        while len >= 0 {
            printf(b"mean[\0" as *const u8 as *const i8);
            i = 0;
            while i < len {
                if i != 0 {
                    printf(b", %g\0" as *const u8 as *const i8, v[i as usize])
                } else {
                    printf(b"%g\0" as *const u8 as *const i8, v[i as usize])
                };
                i += 1;
                i;
            }
            printf(
                b"] = %g\n\0" as *const u8 as *const i8,
                mean(v.as_mut_ptr(), len),
            );
            len -= 1;
            len;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
