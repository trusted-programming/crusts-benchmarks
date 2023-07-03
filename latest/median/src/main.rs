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
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
}
pub type __compar_fn_t =
// SAFETY: machine generated unsafe code
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floatList {
    pub list: *mut libc::c_float,
    pub size: i32,
}
pub type FloatList = *mut floatList;
#[no_mangle]
pub extern "C" fn floatcmp(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if *a.cast::<f32>() < *b.cast::<f32>() {
            -1_i32
        } else {
            i32::from(*a.cast::<f32>() > *b.cast::<f32>())
        }
    }
}

#[no_mangle]
pub extern "C" fn median(mut fl: FloatList) -> libc::c_float {
// SAFETY: machine generated unsafe code
    unsafe {
        qsort(
            (*fl).list.cast::<libc::c_void>(),
            (*fl).size as u64,
            ::core::mem::size_of::<libc::c_float>() as u64,
// SAFETY: machine generated unsafe code
            Some(floatcmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
        );
        (0.5f64
            * f64::from(*((*fl).list).offset(((*fl).size / 2i32) as isize)
                + *((*fl).list).offset((((*fl).size - 1i32) / 2i32) as isize)))
            as libc::c_float
    }
}

fn main_0() -> i32 {
    let mut floats1: [libc::c_float; 6] = [
        5.1f64 as libc::c_float,
        2.6f64 as libc::c_float,
        6.2f64 as libc::c_float,
        8.8f64 as libc::c_float,
        4.6f64 as libc::c_float,
        4.1f64 as libc::c_float,
    ];
    let mut flist1: floatList = {
        
        floatList {
            list: floats1.as_mut_ptr(),
            size: (::core::mem::size_of::<[libc::c_float; 6]>() as u64)
                .wrapping_div(::core::mem::size_of::<libc::c_float>() as u64)
                as i32,
        }
    };
    let mut floats2: [libc::c_float; 5] = [
        5.1f64 as libc::c_float,
        2.6f64 as libc::c_float,
        8.8f64 as libc::c_float,
        4.6f64 as libc::c_float,
        4.1f64 as libc::c_float,
    ];
    let mut flist2: floatList = {
        
        floatList {
            list: floats2.as_mut_ptr(),
            size: (::core::mem::size_of::<[libc::c_float; 5]>() as u64)
                .wrapping_div(::core::mem::size_of::<libc::c_float>() as u64)
                as i32,
        }
    };
    println!("flist1 median is {:7.2}", f64::from(median(&mut flist1)));
    println!("flist2 median is {:7.2}", f64::from(median(&mut flist2)));
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
