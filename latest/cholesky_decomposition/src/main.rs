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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn sqrt(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn cholesky(mut A: *mut f64, mut n: i32) -> *mut f64 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut L: *mut f64 =
            calloc((n * n) as u64, ::core::mem::size_of::<f64>() as u64).cast::<f64>();
        if L.is_null() {
            exit(1);
        }
        let mut i: i32 = 0;
        while i < n {
            let mut j: i32 = 0;
            while j < i + 1_i32 {
                let mut s: f64 = f64::from(0_i32);
                let mut k: i32 = 0;
                while k < j {
                    s += *L.offset((i * n + k) as isize) * *L.offset((j * n + k) as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                *L.offset((i * n + j) as isize) = if i == j {
                    sqrt(*A.offset((i * n + i) as isize) - s)
                } else {
                    1.0f64 / *L.offset((j * n + j) as isize) * (*A.offset((i * n + j) as isize) - s)
                };
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        L
    }
}

#[no_mangle]
pub extern "C" fn show_matrix(mut A: *mut f64, mut n: i32) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        while i < n {
            let mut j: i32 = 0;
            while j < n {
                print!("{:2.5} ", *A.offset((i * n + j) as isize));
                j = j.wrapping_add(1);
                j;
            }
            println!();
            i = i.wrapping_add(1);
            i;
        }
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut n: i32 = 3;
        let mut m1: [f64; 9] = [
            25_f64,
            15_f64,
            -5_f64,
            15_f64,
            18_f64,
            f64::from(0_i32),
            -5_f64,
            f64::from(0_i32),
            11_f64,
        ];
        let mut c1: *mut f64 = cholesky(m1.as_mut_ptr(), n);
        show_matrix(c1, n);
        println!();
        free(c1.cast::<libc::c_void>());
        n = 4_i32;
        let mut m2: [f64; 16] = [
            18_f64, 22_f64, 54_f64, 42_f64, 22_f64, 70_f64, 86_f64, 62_f64,
            54_f64, 86_f64, 174_f64, 134_f64, 42_f64, 62_f64, 134_f64,
            106_f64,
        ];
        let mut c2: *mut f64 = cholesky(m2.as_mut_ptr(), n);
        show_matrix(c2, n);
        free(c2.cast::<libc::c_void>());
        0_i32
    }
}

pub fn main() {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        ::std::process::exit(main_0());
    }
}
