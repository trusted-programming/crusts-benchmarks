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
    fn log(_: f64) -> f64;
    fn sqrt(_: f64) -> f64;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn es_check(mut sieve: *mut u32, mut n: u64) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if n != 2 && n & 1 == 0 || n < 2 {
            0_i32
        } else {
            i32::from(*sieve.offset((n >> 6i32) as isize) & (1i32 << (n >> 1_i32 & 31u64)) as u32 == 0)
        }
    }
}

#[no_mangle]
pub extern "C" fn es_sieve(nth: u64, mut es_size: *mut u64) -> *mut u32 {
// SAFETY: machine generated unsafe code
    unsafe {
        *es_size = ((nth as f64).mul_add(log(nth as f64), nth as f64 * (log(log(nth as f64)) - f64::from(0.9385f32)))
            + 1_f64) as u64;
        let mut sieve: *mut u32 = calloc(
            (*es_size >> 6i32).wrapping_add(1),
            ::core::mem::size_of::<u32>() as u64,
        ).cast::<u32>();
        let mut i: u64 = 3;
        while (i as f64) < sqrt(*es_size as f64) + 1_f64 {
            if *sieve.offset((i >> 6i32) as isize) & (1i32 << (i >> 1_i32 & 31)) as u32 == 0 {
                let mut j: u64 = i.wrapping_mul(i);
                while j < *es_size {
                    let fresh0 = &mut (*sieve.offset((j >> 6i32) as isize));
                    *fresh0 |= (1i32 << (j >> 1_i32 & 31)) as u32;
                    j = (j).wrapping_add(i << 1);
                }
            }
            i = (i).wrapping_add(2);
        }
        sieve
    }
}

#[no_mangle]
pub extern "C" fn mpz_number_of_digits(_op: i32) -> u64 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut opstr: *mut i8 = std::ptr::null_mut::<i8>();
        let oplen: u64 = strlen(opstr);
        free(opstr.cast::<libc::c_void>());
        oplen
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut sieve_size: u64 = 0;
        let mut sieve: *mut u32 = es_sieve(100000, &mut sieve_size);
        let mut prime_count: u64 = 0;
        let mut print: i32 = 1;
        let mut _unused: f64 = 0.;
        let mut i: u64 = 2;
        while i < sieve_size && prime_count <= 100000 {
            if print != 0_i32 {
                print = 0_i32;
            }
            if es_check(sieve, i) != 0_i32 {
                prime_count = prime_count.wrapping_add(1);
                prime_count;
                print = 1_i32;
            }
            i = i.wrapping_add(1);
            i;
        }
        free(sieve.cast::<libc::c_void>());
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
