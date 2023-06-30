#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {}
#[no_mangle]
pub extern "C" fn halve(mut x: *mut i32) {
    unsafe {
        *x >>= 1_i32;
    }
}

#[no_mangle]
pub extern "C" fn doublit(mut x: *mut i32) {
    unsafe {
        *x <<= 1_i32;
    }
}

#[no_mangle]
pub extern "C" fn iseven(x: i32) -> bool {
    x & 1_i32 == 0_i32
}

#[no_mangle]
pub extern "C" fn ethiopian(mut plier: i32, mut plicand: i32, tutor: bool) -> i32 {
    let mut result: i32 = 0;
    if tutor {
        println!("ethiopian multiplication of {} by {}", plier, plicand);
    }
    while plier >= 1_i32 {
        if iseven(plier) {
            if tutor {
                println!("{:4} {:6} struck", plier, plicand);
            }
        } else {
            if tutor {
                println!("{:4} {:6} kept", plier, plicand);
            }
            result = result.wrapping_add(plicand);
        }
        halve(&mut plier);
        doublit(&mut plicand);
    }
    result
}

fn main_0() -> i32 {
    println!("{}", ethiopian(17, 34, 1_i32 != 0_i32));
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
