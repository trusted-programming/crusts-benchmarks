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
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn get_prime(mut idx: i32) -> u64 {
// SAFETY: machine generated unsafe code
    unsafe {
        static mut n_primes: i64 = 0;
        static mut alloc: i64 = 0;
        static mut primes: *mut u64 = 0 as *const u64 as *mut u64;
        let mut last: u64 = 0;
        let mut p: u64 = 0;
        let mut i: i32 = 0;
        if i64::from(idx) >= n_primes {
            if n_primes >= alloc {
                alloc = alloc.wrapping_add(16);
                primes = realloc(
                    primes.cast::<libc::c_void>(),
                    (::core::mem::size_of::<u64>() as u64).wrapping_mul(alloc as u64),
                ).cast::<u64>();
            }
            if n_primes == 0 {
                *primes.offset(0_isize) = 2;
                *primes.offset(1_isize) = 3;
                n_primes = 2;
            }
            last = *primes.offset((n_primes.wrapping_sub(1i64)) as isize);
            while i64::from(idx) >= n_primes {
                last = (last).wrapping_add(2);
                i = 0_i32;
                while i64::from(i) < n_primes {
                    p = *primes.offset(i as isize);
                    if p.wrapping_mul(p) > last {
                        let fresh0 = n_primes;
                        n_primes = n_primes.wrapping_add(1);
                        *primes.offset(fresh0 as isize) = last;
                        break;
                    } else {
                        if last.wrapping_rem(p) == 0 {
                            break;
                        }
                        i = i.wrapping_add(1);
                        i;
                    }
                }
            }
        }
        *primes.offset(idx as isize)
    }
}

fn main_0() -> i32 {
    let mut n: u64 = 0;
    let mut x: u64 = 0;
    let mut p: u64 = 0;
    let mut i: i32 = 0;
    let mut first: i32 = 0;
    x = 1;
    while x < 1000 {
        n = x;
        print!("{} = ", n);
        i = 0_i32;
        first = 1_i32;
        loop {
            p = get_prime(i);
            while n.wrapping_rem(p) == 0 {
                n = (n).wrapping_div(p);
                if first == 0_i32 {
                    print!(" x ");
                }
                first = 0_i32;
                print!("{}", p);
            }
            if n <= p.wrapping_mul(p) {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
        if first != 0_i32 {
            println!("{}", n);
        } else if n > 1 {
            println!(" x {}", n);
        } else {
            println!();
        }
        x = x.wrapping_add(1);
        x;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
