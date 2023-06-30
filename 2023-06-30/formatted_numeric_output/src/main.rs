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
    let mut r: libc::c_float = 7.125f64 as libc::c_float;
    println!(" {:9.3}", f64::from(-r));
    println!(" {:9.3}", f64::from(r));
    println!(" {:-9.3}", f64::from(r));
    println!(" {:09.3}", f64::from(-r));
    println!(" {:09.3}", f64::from(r));
    println!(" {:-09.3}", f64::from(r));
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
