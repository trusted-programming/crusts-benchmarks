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
pub static mut alloc: u64 = 0;
#[no_mangle]
pub static mut n: u64 = 1;
#[no_mangle]
pub static mut q: *mut u64 = 0 as *const u64 as *mut u64;
#[no_mangle]
pub extern "C" fn qpush(mut h: u64) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        if alloc <= n {
            alloc = if alloc != 0 {
                alloc.wrapping_mul(2)
            } else {
                16
            };
            q = realloc(
                q.cast::<libc::c_void>(),
                (::core::mem::size_of::<u64>() as u64).wrapping_mul(alloc),
            ).cast::<u64>();
        }
        let fresh0 = n;
        n = n.wrapping_add(1);
        i = fresh0 as i32;
        loop {
            j = i / 2_i32;
            if !(j != 0_i32 && *q.offset(j as isize) > h) {
                break;
            };
            *q.offset(i as isize) = *q.offset(j as isize);
            i = j;
        }
        *q.offset(i as isize) = h;
    }
}

#[no_mangle]
pub extern "C" fn qpop() -> u64 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut r: u64 = 0;
    let mut t: u64 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        r = *q.offset(1_isize);
        while n > 1 && r == *q.offset(1_isize) {
            i = 1_i32;
            n = n.wrapping_sub(1);
            t = *q.offset(n as isize);
            loop {
                j = i * 2_i32;
                if (j as u64) >= n {
                    break;
                }
                if ((j + 1i32) as u64) < n && *q.offset(j as isize) > *q.offset((j + 1i32) as isize)
                {
                    j += 1_i32;
                    j;
                }
                if t <= *q.offset(j as isize) {
                    break;
                };
                *q.offset(i as isize) = *q.offset(j as isize);
                i = j;
            }
            *q.offset(i as isize) = t;
        }
    }
    r
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut h: u64 = 0;
    i = 1_i32;
    qpush(i as u64);
    while i <= 1_691_i32 {
        h = qpop();
        qpush(h.wrapping_mul(2));
        qpush(h.wrapping_mul(3));
        qpush(h.wrapping_mul(5));
        if i <= 20_i32 || i == 1_691_i32 {
            println!("{:6}: {}", i, h);
        }
        i += 1_i32;
        i;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
