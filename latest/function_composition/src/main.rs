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
    fn asin(_: f64) -> f64;
    fn sin(_: f64) -> f64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct double_to_double {
// SAFETY: machine generated unsafe code
    pub fn_0: Option<unsafe extern "C" fn(*mut double_to_double, f64) -> f64>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compose_functor {
// SAFETY: machine generated unsafe code
    pub fn_0: Option<unsafe extern "C" fn(*mut compose_functor, f64) -> f64>,
    pub f: *mut double_to_double,
    pub g: *mut double_to_double,
}
#[no_mangle]
pub extern "C" fn compose_call(mut this: *mut compose_functor, mut x: f64) -> f64 {
// SAFETY: machine generated unsafe code
    unsafe {
        ((*(*this).f).fn_0).expect("non-null function pointer")(
            (*this).f,
            ((*(*this).g).fn_0).expect("non-null function pointer")((*this).g, x),
        )
    }
}

#[no_mangle]
pub extern "C" fn compose(
    mut f: *mut double_to_double,
    mut g: *mut double_to_double,
) -> *mut double_to_double {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut result: *mut compose_functor =
            malloc(::core::mem::size_of::<compose_functor>() as u64).cast::<compose_functor>();
        (*result).fn_0 =
// SAFETY: machine generated unsafe code
            Some(compose_call as unsafe extern "C" fn(*mut compose_functor, f64) -> f64);
        (*result).f = f;
        (*result).g = g;
        result.cast::<double_to_double>()
    }
}

#[no_mangle]
pub extern "C" fn sin_call(mut _this: *mut double_to_double, mut x: f64) -> f64 {
// SAFETY: machine generated unsafe code
    unsafe {
        sin(x)
    }
}

#[no_mangle]
pub extern "C" fn asin_call(mut _this: *mut double_to_double, mut x: f64) -> f64 {
// SAFETY: machine generated unsafe code
    unsafe {
        asin(x)
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut my_sin: *mut double_to_double =
            malloc(::core::mem::size_of::<double_to_double>() as u64).cast::<double_to_double>();
// SAFETY: machine generated unsafe code
        (*my_sin).fn_0 = Some(sin_call as unsafe extern "C" fn(*mut double_to_double, f64) -> f64);
        let mut my_asin: *mut double_to_double =
            malloc(::core::mem::size_of::<double_to_double>() as u64).cast::<double_to_double>();
        (*my_asin).fn_0 =
// SAFETY: machine generated unsafe code
            Some(asin_call as unsafe extern "C" fn(*mut double_to_double, f64) -> f64);
        let mut sin_asin: *mut double_to_double = compose(my_sin, my_asin);
        println!(
            "{}",
            ((*sin_asin).fn_0).expect("non-null function pointer")(sin_asin, 0.5f64)
        );
        free(sin_asin.cast::<libc::c_void>());
        free(my_sin.cast::<libc::c_void>());
        free(my_asin.cast::<libc::c_void>());
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
