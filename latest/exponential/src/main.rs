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
    fn pow(_: f64, _: f64) -> f64;
}
// SAFETY: machine generated unsafe code
pub type seq_func = Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct gen_t {
    pub f: seq_func,
    pub output: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct power_gen_t {
    pub f: seq_func,
    pub output: i32,
    pub pos: i32,
    pub n: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct filter_gen_t {
    pub f: seq_func,
    pub output: i32,
    pub in_0: *mut libc::c_void,
    pub without: *mut libc::c_void,
}
#[no_mangle]
pub extern "C" fn seq_next(mut state: *mut libc::c_void) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let fresh0 = &mut (*state.cast::<gen_t>()).output;
        *fresh0 = (*state.cast::<std::option::Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>>()).expect("non-null function pointer")(state);
        *fresh0
    }
}

#[no_mangle]
pub extern "C" fn power_next(mut s: *mut libc::c_void) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let fresh1 = &mut (*s.cast::<power_gen_t>()).pos;
        *fresh1 += 1_i32;
        pow(f64::from(*fresh1), f64::from((*s.cast::<power_gen_t>()).n)) as i32
    }
}

#[no_mangle]
pub extern "C" fn power_seq(mut n: i32) -> *mut libc::c_void {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut s: *mut power_gen_t =
            malloc(::core::mem::size_of::<power_gen_t>() as u64).cast::<power_gen_t>();
        (*s).output = -1_i32;
// SAFETY: machine generated unsafe code
        (*s).f = Some(power_next as unsafe extern "C" fn(*mut libc::c_void) -> i32);
        (*s).n = n;
        (*s).pos = -1_i32;
        s.cast::<libc::c_void>()
    }
}

#[no_mangle]
pub extern "C" fn filter_next(mut s: *mut libc::c_void) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut in_0: *mut gen_t = (*s.cast::<filter_gen_t>()).in_0.cast::<gen_t>();
        let mut wo: *mut gen_t = (*s.cast::<filter_gen_t>()).without.cast::<gen_t>();
        loop {
            seq_next(in_0.cast::<libc::c_void>());
            while (*wo).output < (*in_0).output {
                seq_next(wo.cast::<libc::c_void>());
            }
            if (*wo).output != (*in_0).output {
                break;
            }
        }
        (*in_0).output
    }
}

#[no_mangle]
pub extern "C" fn filter_seq(mut in_0: *mut gen_t, mut without: *mut gen_t) -> *mut libc::c_void {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut filt: *mut filter_gen_t =
            malloc(::core::mem::size_of::<filter_gen_t>() as u64).cast::<filter_gen_t>();
        (*filt).in_0 = in_0.cast::<libc::c_void>();
        (*filt).without = without.cast::<libc::c_void>();
// SAFETY: machine generated unsafe code
        (*filt).f = Some(filter_next as unsafe extern "C" fn(*mut libc::c_void) -> i32);
        (*filt).output = -1_i32;
        filt.cast::<libc::c_void>()
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut s: *mut libc::c_void =
            filter_seq(power_seq(2).cast::<gen_t>(), power_seq(3).cast::<gen_t>());
        i = 0_i32;
        while i < 20_i32 {
            seq_next(s);
            i = i.wrapping_add(1);
            i;
        }
        i = 0_i32;
        while i < 10_i32 {
            println!("{}", seq_next(s));
            i = i.wrapping_add(1);
            i;
        }
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
