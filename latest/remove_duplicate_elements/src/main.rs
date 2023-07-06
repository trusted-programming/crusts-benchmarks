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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn elem(mut a: *mut i32, mut n: u64, mut e: i32) -> bool {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: u64 = 0;
        while i < n {
            if *a.offset(i as isize) == e {
                return 1_i32 != 0_i32;
            }
            i = i.wrapping_add(1);
            i;
        }
        0_i32 != 0_i32
    }
}

#[no_mangle]
pub extern "C" fn nub(mut a: *mut i32, mut n: u64) -> u64 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut m: u64 = 0;
        let mut i: u64 = 0;
        while i < n {
            if !elem(a, m, *a.offset(i as isize)) {
                let fresh0 = m;
                m = m.wrapping_add(1);
                *a.offset(fresh0 as isize) = *a.offset(i as isize);
            }
            i = i.wrapping_add(1);
            i;
        }
        m
    }
}

#[no_mangle]
pub extern "C" fn nub_new(mut b: *mut *mut i32, mut a: *mut i32, mut n: u64) -> u64 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut c: *mut i32 =
            malloc(n.wrapping_mul(::core::mem::size_of::<i32>() as u64)).cast::<i32>();
        memcpy(
            c.cast::<libc::c_void>(),
            a as *const libc::c_void,
            n.wrapping_mul(::core::mem::size_of::<i32>() as u64),
        );
        let mut m: i32 = nub(c, n) as i32;
        *b = malloc((m as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64)).cast::<i32>();
        memcpy(
            (*b).cast::<libc::c_void>(),
            c as *const libc::c_void,
            (m as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
        );
        free(c.cast::<libc::c_void>());
        m as u64
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut a: [i32; 10] = [1, 2, 1, 4, 5, 2, 15, 1, 3, 4];
        let mut b: *mut i32 = std::ptr::null_mut::<i32>();
        let mut n: u64 = nub_new(
            &mut b,
            a.as_mut_ptr(),
            (::core::mem::size_of::<[i32; 10]>() as u64)
                .wrapping_div(::core::mem::size_of::<i32>() as u64),
        );
        let mut i: u64 = 0;
        while i < n {
            print!("{} ", *b.offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        puts((b"\0" as *const u8).cast::<i8>());
        free(b.cast::<libc::c_void>());
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
