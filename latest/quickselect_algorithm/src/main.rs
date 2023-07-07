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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn qselect(mut v: *mut i32, mut len: i32, mut k: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut st: i32 = 0;
        let mut tmp: i32 = 0;
        i = 0_i32;
        st = i;
        while i < len.wrapping_sub(1) {
            if *v.offset(i as isize) <= *v.offset((len.wrapping_sub(1i32)) as isize) {
                tmp = *v.offset(i as isize);
                *v.offset(i as isize) = *v.offset(st as isize);
                *v.offset(st as isize) = tmp;
                st = st.wrapping_add(1);
                st;
            }
            i = i.wrapping_add(1);
            i;
        }
        tmp = *v.offset((len.wrapping_sub(1i32)) as isize);
        *v.offset((len.wrapping_sub(1i32)) as isize) = *v.offset(st as isize);
        *v.offset(st as isize) = tmp;
        if k == st {
            *v.offset(st as isize)
        } else if st > k {
            qselect(v, st, k)
        } else {
            qselect(v.offset(st as isize), len - st, k - st)
        }
    }
}

fn main_0() -> i32 {
    let mut x: [i32; 10] = [9, 8, 7, 6, 5, 0, 1, 2, 3, 4];
    let mut y: [i32; 10] = [0; 10];
    let mut i: i32 = 0;
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 10_i32 {
            memcpy(
                y.as_mut_ptr().cast::<libc::c_void>(),
                x.as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[i32; 10]>() as u64,
            );
            println!("{}: {}", i, qselect(y.as_mut_ptr(), 10, i));
            i = i.wrapping_add(1);
            i;
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
