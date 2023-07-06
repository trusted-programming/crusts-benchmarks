#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]

extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
fn main_0() -> i32 {
    let mut a: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        if a == 42_i32 {
        } else {
            __assert_fail(
                (b"a == 42\0" as *const u8).cast::<i8>(),
                (b"main.c\0" as *const u8).cast::<i8>(),
                6,
                (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"int main()\0")).as_ptr(),
            );
        }
        if a == 42_i32 {
            } else {
                __assert_fail(
                    (b"a == 42\0" as *const u8).cast::<i8>(),
                    (b"main.c\0" as *const u8).cast::<i8>(),
                    6,
                    (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"int main()\0")).as_ptr(),
                );
            };
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
