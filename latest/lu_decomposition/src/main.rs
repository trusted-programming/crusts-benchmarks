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
    fn printf(_: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fabs(_: f64) -> f64;
}
pub type mat = *mut *mut f64;
#[no_mangle]
pub extern "C" fn mat_zero(mut x: mat, mut n: i32) {
    let mut i: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < n {
            let mut j: i32 = 0;
            while j < n {
                *(*x.offset(i as isize)).offset(j as isize) = f64::from(0_i32);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn mat_new(mut n: i32) -> mat {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut x: mat =
            malloc((::core::mem::size_of::<*mut f64>() as u64).wrapping_mul(n as u64)).cast::<*mut f64>();
        let fresh0 = &mut (*x.offset(0_isize));
        *fresh0 = malloc(
            (::core::mem::size_of::<f64>() as u64)
                .wrapping_mul(n as u64)
                .wrapping_mul(n as u64),
        ).cast::<f64>();
        let mut i: i32 = 0;
        while i < n {
            let fresh1 = &mut (*x.offset(i as isize));
            *fresh1 = (*x.offset(0_isize)).offset((n * i) as isize);
            i = i.wrapping_add(1);
            i;
        }
        mat_zero(x, n);
        x
    }
}

#[no_mangle]
pub extern "C" fn mat_copy(mut s: *mut libc::c_void, mut n: i32) -> mat {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut x: mat = mat_new(n);
        let mut i: i32 = 0;
        while i < n {
            let mut j: i32 = 0;
            while j < n {
                let vla = n as usize;
                *(*x.offset(i as isize)).offset(j as isize) = *s.cast::<f64>()
                    .offset(i as isize * vla as isize)
                    .offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        x
    }
}

#[no_mangle]
pub extern "C" fn mat_del(mut x: mat) {
// SAFETY: machine generated unsafe code
    unsafe {
        free((*x.offset(0_isize)).cast::<libc::c_void>());
        free(x.cast::<libc::c_void>());
    }
}

#[no_mangle]
pub extern "C" fn mat_show(mut x: mat, mut fmt: *mut i8, mut n: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        if fmt.is_null() {
            fmt = (b"%8.4g\0" as *const u8).cast::<i8>() as *mut i8;
        }
        let mut i: i32 = 0;
        while i < n {
            if i != 0_i32 {
                print!("      ")
            } else {
                print!(" [ ")
            };
            let mut j: i32 = 0;
            while j < n {
                printf(fmt, *(*x.offset(i as isize)).offset(j as isize));
                if j < n - 1_i32 {
                    print!("  ")
                } else if i == n - 1_i32 {
                    println!(" ]")
                } else {
                    println!()
                };
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn mat_mul(mut a: mat, mut b: mat, mut n: i32) -> mat {
    let mut c: mat = std::ptr::null_mut::<*mut f64>();
    c = mat_new(n);
    c = c;
    let mut i: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < n {
            let mut j: i32 = 0;
            while j < n {
                let mut k: i32 = 0;
                while k < n {
                    *(*c.offset(i as isize)).offset(j as isize) += *(*a.offset(i as isize))
                        .offset(k as isize)
                        * *(*b.offset(k as isize)).offset(j as isize);
                    k = k.wrapping_add(1);
                    k;
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    c
}

#[no_mangle]
pub extern "C" fn mat_pivot(mut a: mat, mut p: mat, mut n: i32) {
    let mut i: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < n {
            let mut j: i32 = 0;
            while j < n {
                *(*p.offset(i as isize)).offset(j as isize) = f64::from(i32::from(i == j));
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    let mut i_0: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        while i_0 < n {
            let mut max_j: i32 = i_0;
            let mut j_0: i32 = i_0;
            while j_0 < n {
                if fabs(*(*a.offset(j_0 as isize)).offset(i_0 as isize))
                    > fabs(*(*a.offset(max_j as isize)).offset(i_0 as isize))
                {
                    max_j = j_0;
                }
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
            if max_j != i_0 {
                let mut k: i32 = 0;
                while k < n {
                    let mut tmp: f64 = *(*p.offset(i_0 as isize)).offset(k as isize);
                    *(*p.offset(i_0 as isize)).offset(k as isize) =
                        *(*p.offset(max_j as isize)).offset(k as isize);
                    *(*p.offset(max_j as isize)).offset(k as isize) = tmp;
                    k = k.wrapping_add(1);
                    k;
                }
            }
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
}

#[no_mangle]
pub extern "C" fn mat_LU(mut A: mat, mut L: mat, mut U: mat, mut P: mat, mut n: i32) {
    mat_zero(L, n);
    mat_zero(U, n);
    mat_pivot(A, P, n);
    let mut Aprime: mat = mat_mul(P, A, n);
    let mut i: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < n {
            *(*L.offset(i as isize)).offset(i as isize) = 1_f64;
            i = i.wrapping_add(1);
            i;
        }
    }
    let mut i_0: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        while i_0 < n {
            let mut j: i32 = 0;
            while j < n {
                let mut s: f64 = 0.;
                if j <= i_0 {
                    s = f64::from(0_i32);
                    let mut k: i32 = 0;
                    while k < j {
                        s += *(*L.offset(j as isize)).offset(k as isize)
                            * *(*U.offset(k as isize)).offset(i_0 as isize);
                        k = k.wrapping_add(1);
                        k;
                    }
                    *(*U.offset(j as isize)).offset(i_0 as isize) =
                        *(*Aprime.offset(j as isize)).offset(i_0 as isize) - s;
                }
                if j >= i_0 {
                    s = f64::from(0_i32);
                    let mut k_0: i32 = 0;
                    while k_0 < i_0 {
                        s += *(*L.offset(j as isize)).offset(k_0 as isize)
                            * *(*U.offset(k_0 as isize)).offset(i_0 as isize);
                        k_0 = k_0.wrapping_add(1);
                        k_0;
                    }
                    *(*L.offset(j as isize)).offset(i_0 as isize) =
                        (*(*Aprime.offset(j as isize)).offset(i_0 as isize) - s)
                            / *(*U.offset(i_0 as isize)).offset(i_0 as isize);
                }
                j = j.wrapping_add(1);
                j;
            }
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    mat_del(Aprime);
}

#[no_mangle]
pub static mut A3: [[f64; 3]; 3] = [
    [1_f64, 3_f64, 5_f64],
    [2_f64, 4_f64, 7_f64],
    [1_f64, 1_f64, 0_i32 as f64],
];
#[no_mangle]
pub static mut A4: [[f64; 4]; 4] = [
    [11_f64, 9_f64, 24_f64, 2_f64],
    [1_f64, 5_f64, 2_f64, 6_f64],
    [3_f64, 17_f64, 18_f64, 1_f64],
    [2_f64, 5_f64, 7_f64, 1_f64],
];
fn main_0() -> i32 {
    let mut n: i32 = 3;
    let mut A: mat = std::ptr::null_mut::<*mut f64>();
    let mut L: mat = std::ptr::null_mut::<*mut f64>();
    let mut P: mat = std::ptr::null_mut::<*mut f64>();
    let mut U: mat = std::ptr::null_mut::<*mut f64>();
    L = mat_new(n);
    P = mat_new(n);
    U = mat_new(n);
// SAFETY: machine generated unsafe code
    unsafe {
        A = mat_copy(A3.as_mut_ptr().cast::<libc::c_void>(), n);
    }
    mat_LU(A, L, U, P, n);
    print!("A =");
    mat_show(A, std::ptr::null_mut::<i8>(), n);
    print!("L =");
    mat_show(L, std::ptr::null_mut::<i8>(), n);
    print!("U =");
    mat_show(U, std::ptr::null_mut::<i8>(), n);
    print!("P =");
    mat_show(P, std::ptr::null_mut::<i8>(), n);
    mat_del(A);
    mat_del(L);
    mat_del(U);
    mat_del(P);
    println!();
    n = 4_i32;
    L = mat_new(n);
    P = mat_new(n);
    U = mat_new(n);
// SAFETY: machine generated unsafe code
    unsafe {
        A = mat_copy(A4.as_mut_ptr().cast::<libc::c_void>(), n);
    }
    mat_LU(A, L, U, P, n);
    print!("A =");
    mat_show(A, std::ptr::null_mut::<i8>(), n);
    print!("L =");
    mat_show(L, std::ptr::null_mut::<i8>(), n);
    print!("U =");
    mat_show(U, std::ptr::null_mut::<i8>(), n);
    print!("P =");
    mat_show(P, std::ptr::null_mut::<i8>(), n);
    mat_del(A);
    mat_del(L);
    mat_del(U);
    mat_del(P);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
