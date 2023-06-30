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
    fn puts(__s: *const i8) -> i32;
    fn free(_: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn string_repeat(mut n: i32, mut s: *const i8) -> *mut i8 {
    unsafe {
        let mut slen: u64 = strlen(s);
        let mut dest: *mut i8 = malloc((n as u64).wrapping_mul(slen).wrapping_add(1)).cast::<i8>();
        let mut i: i32 = 0;
        let mut p: *mut i8 = std::ptr::null_mut::<i8>();
        i = 0_i32;
        p = dest;
        while i < n {
            memcpy(p.cast::<libc::c_void>(), s.cast::<libc::c_void>(), slen);
            i = i.wrapping_add(1);
            i;
            p = p.offset(slen as isize);
        }
        *p = '\0' as i8;
        dest
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut result: *mut i8 = string_repeat(5, (b"ha\0" as *const u8).cast::<i8>());
        puts(result);
        free(result.cast::<libc::c_void>());
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
