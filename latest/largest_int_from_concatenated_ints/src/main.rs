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
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
    fn strcmp(_: *const i8, _: *const i8) -> i32;
}
pub type __compar_fn_t =
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
#[no_mangle]
pub extern "C" fn catcmp(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut ab: [i8; 32] = [0; 32];
        let mut ba: [i8; 32] = [0; 32];
        sprintf(
            ab.as_mut_ptr(),
            (b"%d%d\0" as *const u8).cast::<i8>(),
            *(a as *mut i32),
            *(b as *mut i32),
        );
        sprintf(
            ba.as_mut_ptr(),
            (b"%d%d\0" as *const u8).cast::<i8>(),
            *(b as *mut i32),
            *(a as *mut i32),
        );
        strcmp(ba.as_mut_ptr(), ab.as_mut_ptr())
    }
}

#[no_mangle]
pub extern "C" fn maxcat(mut a: *mut i32, mut len: i32) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        qsort(
            a.cast::<libc::c_void>(),
            len as u64,
            ::core::mem::size_of::<i32>() as u64,
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
            Some(catcmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
        );
        i = 0_i32;
        while i < len {
            print!("{}", *a.offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        print!("{}", '\n' as i32);
    }
}

fn main_0() -> i32 {
    let mut x: [i32; 8] = [1, 34, 3, 98, 9, 76, 45, 4];
    let mut y: [i32; 4] = [54, 546, 548, 60];
    maxcat(
        x.as_mut_ptr(),
        (::core::mem::size_of::<[i32; 8]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32,
    );
    maxcat(
        y.as_mut_ptr(),
        (::core::mem::size_of::<[i32; 4]>() as u64)
            .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32,
    );
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
