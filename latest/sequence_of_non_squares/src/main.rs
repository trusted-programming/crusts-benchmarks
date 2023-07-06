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
    fn sqrt(_: f64) -> f64;
    fn floor(_: f64) -> f64;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
#[no_mangle]
pub extern "C" fn nonsqr(mut n: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        n + (0.5f64 + sqrt(f64::from(n))) as i32
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    i = 1_i32;
    while i < 23_i32 {
        print!("{} ", nonsqr(i));
        i += 1_i32;
        i;
    }
    println!();
    i = 1_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 1_000_000_i32 {
            let mut j: f64 = sqrt(f64::from(nonsqr(i)));
            if j != floor(j) {
            } else {
                __assert_fail(
                    (b"j != floor(j)\0" as *const u8).cast::<i8>(),
                    (b"main.c\0" as *const u8).cast::<i8>(),
                    21,
                    (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"int main()\0")).as_ptr(),
                );
            }
            if j != floor(j) {
                } else {
                    __assert_fail(
                        (b"j != floor(j)\0" as *const u8).cast::<i8>(),
                        (b"main.c\0" as *const u8).cast::<i8>(),
                        21,
                        (*::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"int main()\0")).as_ptr(),
                    );
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
