#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]

extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct sma_obj {
    pub sma: f64,
    pub sum: f64,
    pub period: i32,
    pub values: *mut f64,
    pub lv: i32,
}
pub type sma_obj_t = sma_obj;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sma_result {
    pub handle: *mut sma_obj_t,
    pub sma: f64,
    pub values: *mut f64,
}
pub type sma_result_t = sma_result;
pub const SMA_MEAN: u32 = 4;
pub const SMA_ADD: u32 = 3;
pub const SMA_VALUES: u32 = 2;
pub const SMA_FREE: u32 = 1;
pub const SMA_NEW: u32 = 0;
#[no_mangle]
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
pub unsafe extern "C" fn sma(mut action: u32, mut args: ...) -> sma_result_t {
    let mut vl: ::core::ffi::VaListImpl;
    let mut r: sma_result_t = sma_result {
        handle: std::ptr::null_mut::<sma_obj_t>(),
    };
    let mut o: *mut sma_obj_t = std::ptr::null_mut::<sma_obj_t>();
    let mut v_0: f64 = 0.;
    vl = args.clone();
    match action {
        0 => {
            r.handle = malloc(::core::mem::size_of::<sma_obj_t>() as u64).cast::<sma_obj>();
            (*r.handle).sma = 0.0f64;
            (*r.handle).period = vl.arg::<i32>();
            (*r.handle).values = malloc(
                ((*r.handle).period as u64).wrapping_mul(::core::mem::size_of::<f64>() as u64),
            ).cast::<f64>();
            (*r.handle).lv = 0_i32;
            (*r.handle).sum = 0.0f64;
        }
        1 => {
            r.handle = vl.arg::<*mut sma_obj_t>();
            free((*r.handle).values.cast::<libc::c_void>());
            free(r.handle.cast::<libc::c_void>());
            r.handle = std::ptr::null_mut::<sma_obj_t>();
        }
        2 => {
            o = vl.arg::<*mut sma_obj_t>();
            r.values = (*o).values;
        }
        4 => {
            o = vl.arg::<*mut sma_obj_t>();
            r.sma = (*o).sma;
        }
        3 => {
            o = vl.arg::<*mut sma_obj_t>();
            v_0 = vl.arg::<f64>();
            if (*o).lv < (*o).period {
                let fresh0 = (*o).lv;
                (*o).lv += 1_i32;
                *((*o).values).offset(fresh0 as isize) = v_0;
                (*o).sum += v_0;
                (*o).sma = (*o).sum / f64::from((*o).lv);
            } else {
                (*o).sum -= *((*o).values).offset(((*o).lv % (*o).period) as isize);
                (*o).sum += v_0;
                (*o).sma = (*o).sum / f64::from((*o).period);
                *((*o).values).offset(((*o).lv % (*o).period) as isize) = v_0;
                (*o).lv += 1_i32;
                (*o).lv;
            }
            r.sma = (*o).sma;
        }
        _ => {}
    }
    r
}

#[no_mangle]
pub static mut v: [f64; 10] = [
    1_f64, 2_f64, 3_f64, 4_f64, 5_f64, 5_f64, 4_f64, 3_f64, 2_f64,
    1_f64,
];
fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut h3: *mut sma_obj_t = (sma(SMA_NEW, 3)).handle;
        let mut h5: *mut sma_obj_t = (sma(SMA_NEW, 5)).handle;
        i = 0_i32;
        while (i as u64)
            < (::core::mem::size_of::<[f64; 10]>() as u64)
                .wrapping_div(::core::mem::size_of::<f64>() as u64)
        {
            println!(
                "next number {}, SMA_3 = {}, SMA_5 = {}",
                v[i as usize],
                (sma(SMA_ADD, h3, v[i as usize])).sma,
                (sma(SMA_ADD, h5, v[i as usize])).sma
            );
            i = i.wrapping_add(1);
            i;
        }
        sma(SMA_FREE, h3);
        sma(SMA_FREE, h5);
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
