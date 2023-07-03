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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn array_concat(
    mut a_0: *const libc::c_void,
    mut an: u64,
    mut b_0: *const libc::c_void,
    mut bn: u64,
    mut s: u64,
) -> *mut libc::c_void {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut p: *mut i8 = malloc(s.wrapping_mul(an.wrapping_add(bn))).cast::<i8>();
        memcpy(p.cast::<libc::c_void>(), a_0, an.wrapping_mul(s));
        memcpy(
            p.offset(an.wrapping_mul(s) as isize).cast::<libc::c_void>(),
            b_0,
            bn.wrapping_mul(s),
        );
        p.cast::<libc::c_void>()
    }
}

#[no_mangle]
pub static mut a: [i32; 5] = [1_i32, 2_i32, 3_i32, 4_i32, 5_i32];
#[no_mangle]
pub static mut b: [i32; 5] = [6_i32, 7_i32, 8_i32, 9_i32, 0_i32];
fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: u32 = 0;
        let mut c: *mut i32 = array_concat(
            a.as_ptr().cast::<libc::c_void>(),
            5,
            b.as_ptr().cast::<libc::c_void>(),
            5,
            ::core::mem::size_of::<i32>() as u64,
        ).cast::<i32>();
        i = 0;
        while i < 10 {
            println!("{}", *c.offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        free(c.cast::<libc::c_void>());
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
