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
fn main_0() -> i32 {
    let mut inf: f64 = 1 as f64 / 0.0f64;
    let mut minus_inf: f64 = -1i32 as f64 / 0.0f64;
    let mut minus_zero: f64 = -1i32 as f64 / inf;
    let mut nan: f64 = 0.0f64 / 0.0f64;
    unsafe {
        printf(b"positive infinity: %f\n\0" as *const u8 as *const i8, inf);
        printf(
            b"negative infinity: %f\n\0" as *const u8 as *const i8,
            minus_inf,
        );
        printf(
            b"negative zero: %f\n\0" as *const u8 as *const i8,
            minus_zero,
        );
        printf(b"not a number: %f\n\0" as *const u8 as *const i8, nan);
        printf(
            b"+inf + 2.0 = %f\n\0" as *const u8 as *const i8,
            inf + 2.0f64,
        );
        printf(
            b"+inf - 10.1 = %f\n\0" as *const u8 as *const i8,
            inf - 10.1f64,
        );
        printf(
            b"+inf + -inf = %f\n\0" as *const u8 as *const i8,
            inf + minus_inf,
        );
        printf(
            b"0.0 * +inf = %f\n\0" as *const u8 as *const i8,
            0.0f64 * inf,
        );
        printf(
            b"1.0/-0.0 = %f\n\0" as *const u8 as *const i8,
            1.0f64 / minus_zero,
        );
        printf(
            b"NaN + 1.0 = %f\n\0" as *const u8 as *const i8,
            nan + 1.0f64,
        );
        printf(b"NaN + NaN = %f\n\0" as *const u8 as *const i8, nan + nan);
        if nan == nan {
            printf(
                b"NaN == NaN = %s\n\0" as *const u8 as *const i8,
                b"true\0" as *const u8 as *const i8,
            )
        } else {
            printf(
                b"NaN == NaN = %s\n\0" as *const u8 as *const i8,
                b"false\0" as *const u8 as *const i8,
            )
        };
        if 0.0f64 == minus_zero {
            printf(
                b"0.0 == -0.0 = %s\n\0" as *const u8 as *const i8,
                b"true\0" as *const u8 as *const i8,
            )
        } else {
            printf(
                b"0.0 == -0.0 = %s\n\0" as *const u8 as *const i8,
                b"false\0" as *const u8 as *const i8,
            )
        };
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
