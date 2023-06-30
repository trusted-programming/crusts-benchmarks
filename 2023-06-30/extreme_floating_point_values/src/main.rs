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
fn main_0() -> i32 {
    let mut inf: f64 = 1_f64 / 0.0f64;
    let mut minus_inf: f64 = -1_f64 / 0.0f64;
    let mut minus_zero: f64 = -1_f64 / inf;
    let mut nan: f64 = 0.0f64 / 0.0f64;
    println!("positive infinity: {}", inf);
    println!("negative infinity: {}", minus_inf);
    println!("negative zero: {}", minus_zero);
    println!("not a number: {}", nan);
    println!("+inf + 2.0 = {}", inf + 2.0f64);
    println!("+inf - 10.1 = {}", inf - 10.1f64);
    println!("+inf + -inf = {}", inf + minus_inf);
    println!("0.0 * +inf = {}", 0.0f64 * inf);
    println!("1.0/-0.0 = {}", 1.0f64 / minus_zero);
    println!("NaN + 1.0 = {}", nan + 1.0f64);
    println!("NaN + NaN = {}", nan + nan);
    if nan == nan {
        println!("NaN == NaN = true\0")
    } else {
        println!("NaN == NaN = false\0")
    };
    if 0.0f64 == minus_zero {
        println!("0.0 == -0.0 = true\0")
    } else {
        println!("0.0 == -0.0 = false\0")
    };
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
