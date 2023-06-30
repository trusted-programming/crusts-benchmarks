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
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct frac {
    pub d: i32,
    pub n: i32,
}
#[no_mangle]
pub extern "C" fn farey(mut n: i32) {
    let mut f1: frac = {
        
        frac { d: 0, n: 1 }
    };
    let mut f2: frac = {
        
        frac { d: 1, n }
    };
    let mut t: frac = frac { d: 0, n: 0 };
    let mut k: i32 = 0;
    print!("{}/{} {}/{}", 0_i32, 1_i32, 1_i32, n);
    while f2.n > 1_i32 {
        k = (n + f1.n) / f2.n;
        t = f1;
        f1 = f2;
        f2 = {
            
            frac {
                d: f2.d * k - t.d,
                n: f2.n * k - t.n,
            }
        };
        print!(" {}/{}", f2.d, f2.n);
    }
    print!("{}", '\n' as i32);
}

#[no_mangle]
pub static mut cache: *mut u64 = 0 as *const u64 as *mut u64;
#[no_mangle]
pub static mut ccap: u64 = 0;
#[no_mangle]
pub extern "C" fn farey_len(mut n: i32) -> u64 {
    unsafe {
        if n as u64 >= ccap {
            let mut old: u64 = ccap;
            if ccap == 0 {
                ccap = 16;
            }
            while ccap <= n as u64 {
                ccap = ccap.wrapping_mul(2);
            }
            cache = realloc(
                cache.cast::<libc::c_void>(),
                (::core::mem::size_of::<u64>() as u64).wrapping_mul(ccap),
            ).cast::<u64>();
            memset(
                cache.offset(old as isize).cast::<libc::c_void>(),
                0,
                (::core::mem::size_of::<u64>() as u64).wrapping_mul(ccap.wrapping_sub(old)),
            );
        } else if *cache.offset(n as isize) != 0 {
            return *cache.offset(n as isize);
        }
    }
    let mut len: u64 = (n as u64).wrapping_mul((n + 3i32) as u64).wrapping_div(2);
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    p = 2_i32;
    unsafe {
        while p <= n {
            q = n / (n / p) + 1_i32;
            len = (len).wrapping_sub((farey_len(n / p)).wrapping_mul((q - p) as u64));
            p = q;
        }
        *cache.offset(n as isize) = len;
    }
    len
}

fn main_0() -> i32 {
    let mut n: i32 = 0;
    n = 1_i32;
    while n <= 11_i32 {
        print!("{}: ", n);
        farey(n);
        n = n.wrapping_add(1);
        n;
    }
    n = 100_i32;
    while n <= 1_000_i32 {
        println!("{}: {} items", n, farey_len(n));
        n = n.wrapping_add(100);
    }
    n = 10_000_000_i32;
    print!("\n{}: {} items\n", n, farey_len(n));
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
