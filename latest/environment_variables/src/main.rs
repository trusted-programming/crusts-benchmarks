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
    fn getenv(__name: *const i8) -> *mut i8;
    fn puts(__s: *const i8) -> i32;
}
fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        puts(getenv((b"HOME\0" as *const u8).cast::<i8>()));
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
