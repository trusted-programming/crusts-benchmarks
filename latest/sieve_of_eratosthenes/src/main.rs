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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn sqrt(_: f64) -> f64;
    fn printf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub extern "C" fn eratosthenes(mut n: i32, mut c: *mut i32) -> *mut i8 {
    unsafe {
        let mut sieve_0: *mut i8 = std::ptr::null_mut::<i8>();
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut m: i32 = 0;
        if n < 2_i32 {
            return std::ptr::null_mut::<i8>();
        }
        *c = n - 1_i32;
        m = sqrt(f64::from(n)) as i32;
        sieve_0 = calloc((n + 1i32) as u64, ::core::mem::size_of::<i8>() as u64).cast::<i8>();
        *sieve_0.offset(0_isize) = 1;
        *sieve_0.offset(1_isize) = 1;
        i = 2_i32;
        while i <= m {
            if *sieve_0.offset(i as isize) == 0 {
                j = i * i;
                while j <= n {
                    if *sieve_0.offset(j as isize) == 0 {
                        *sieve_0.offset(j as isize) = 1;
                        *c -= 1_i32;
                        *c;
                    }
                    j = j.wrapping_add(i);
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        sieve_0
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut array: *mut i32 = std::ptr::null_mut::<i32>();
        let mut n: i32 = 10;
        array = malloc(((n + 1i32) as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64)).cast::<i32>();
        sieve(array, n);
        0_i32
    }
}

#[no_mangle]
pub extern "C" fn sieve(mut a: *mut i32, mut n: i32) {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        i = 2_i32;
        while i <= n {
            *a.offset(i as isize) = 1_i32;
            i = i.wrapping_add(1);
            i;
        }
        i = 2_i32;
        while i <= n {
            print!("\ni:{}", i);
            if *a.offset(i as isize) == 1_i32 {
                j = i;
                while i * j <= n {
                    print!("\nj:{}", j);
                    printf(
                        (b"\nBefore a[%d*%d]: %d\0" as *const u8).cast::<i8>(),
                        i,
                        j,
                        *a.offset((i * j) as isize),
                    );
                    *a.offset((i * j) as isize) = 0_i32;
                    printf(
                        (b"\nAfter a[%d*%d]: %d\0" as *const u8).cast::<i8>(),
                        i,
                        j,
                        *a.offset((i * j) as isize),
                    );
                    j = j.wrapping_add(1);
                    j;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        print!("\nPrimes numbers from 1 to {} are : ", n);
        i = 2_i32;
        while i <= n {
            if *a.offset(i as isize) == 1_i32 {
                print!("{}, ", i);
            }
            i = i.wrapping_add(1);
            i;
        }
        print!("\n\n");
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
