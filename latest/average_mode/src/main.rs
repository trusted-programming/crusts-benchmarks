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
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vcount {
    pub v: f64,
    pub c: i32,
}
#[no_mangle]
pub extern "C" fn cmp_dbl(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    unsafe {
        let mut x: f64 = *a.cast::<f64>() - *b.cast::<f64>();
        if x < f64::from(0_i32) {
            -1_i32
        } else {
            i32::from(x > f64::from(0_i32))
        }
    }
}

#[no_mangle]
pub extern "C" fn vc_cmp(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    unsafe {
        (*b.cast::<vcount>()).c - (*a.cast::<vcount>()).c
    }
}

#[no_mangle]
pub extern "C" fn get_mode(mut x: *mut f64, mut len_0: i32, mut list: *mut *mut vcount) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut vc: *mut vcount = std::ptr::null_mut::<vcount>();
        qsort(
            x.cast::<libc::c_void>(),
            len_0 as u64,
            ::core::mem::size_of::<f64>() as u64,
            Some(cmp_dbl as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
        );
        i = 0_i32;
        j = 1_i32;
        while i < len_0 - 1_i32 {
            i = i.wrapping_add(1);
            i;
            j += i32::from(*x.offset(i as isize) != *x.offset((i + 1i32) as isize));
        }
        vc =
            malloc((::core::mem::size_of::<vcount>() as u64).wrapping_mul(j as u64)).cast::<vcount>();
        *list = vc;
        (*vc.offset(0_isize)).v = *x.offset(0_isize);
        (*vc.offset(0_isize)).c = 1_i32;
        j = 0_i32;
        i = j;
        while i < len_0 - 1_i32 {
            if *x.offset(i as isize) != *x.offset((i + 1i32) as isize) {
                j = j.wrapping_add(1);
                (*vc.offset(j as isize)).v = *x.offset((i + 1i32) as isize);
            }
            i = i.wrapping_add(1);
            i;
            let fresh0 = &mut (*vc.offset(j as isize)).c;
            *fresh0 += 1_i32;
            *fresh0;
        }
        qsort(
            vc.cast::<libc::c_void>(),
            (j + 1i32) as u64,
            ::core::mem::size_of::<vcount>() as u64,
            Some(vc_cmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
        );
        i = 0_i32;
        while i <= j && (*vc.offset(i as isize)).c == (*vc.offset(0_isize)).c {
            i = i.wrapping_add(1);
            i;
        }
        i
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut values: [f64; 13] = [
            1_f64, 3_f64, 6_f64, 6_f64, 6_f64, 6_f64, 7_f64, 7_f64,
            12_f64, 12_f64, 12_f64, 12_f64, 17_f64,
        ];
        let mut vc: *mut vcount = std::ptr::null_mut::<vcount>();
        let mut i: i32 = 0;
        let mut n_modes: i32 = get_mode(
            values.as_mut_ptr(),
            (::core::mem::size_of::<[f64; 13]>() as u64)
                .wrapping_div(::core::mem::size_of::<f64>() as u64) as i32,
            &mut vc,
        );
        println!("got {} modes:", n_modes);
        i = 0_i32;
        while i < n_modes {
            println!(
                "	value = {}, count = {}",
                (*vc.offset(i as isize)).v,
                (*vc.offset(i as isize)).c
            );
            i = i.wrapping_add(1);
            i;
        }
        free(vc.cast::<libc::c_void>());
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
