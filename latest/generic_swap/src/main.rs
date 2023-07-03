#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct test {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}
#[no_mangle]
pub extern "C" fn swap(mut va: *mut libc::c_void, mut vb: *mut libc::c_void, mut s: u64) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut t: i8 = 0;
        let mut a: *mut i8 = va.cast::<i8>();
        let mut b: *mut i8 = vb.cast::<i8>();
        loop {
            let fresh0 = s;
            s = s.wrapping_sub(1);
            if fresh0 == 0 {
                break;
            }
            t = *a.offset(s as isize);
            *a.offset(s as isize) = *b.offset(s as isize);
            *b.offset(s as isize) = t;
        }
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut t: test = {
            
            test { a: 1, b: 2, c: 3 }
        };
        let mut h: test = {
            
            test { a: 4, b: 5, c: 6 }
        };
        let mut alfa: f64 = 0.45f64;
        let mut omega: f64 = 9.98f64;
        let mut pt: *mut test = &mut t;
        let mut th: *mut test = &mut h;
        println!("{} {} {}", t.a, t.b, t.c);
        std::mem::swap(&mut t, &mut h);
        println!("{} {} {}", t.a, t.b, t.c);
        println!("{} {} {}", h.a, h.b, h.c);
        println!("{}", alfa);
        std::mem::swap(&mut alfa, &mut omega);
        println!("{}", alfa);
        println!("{}", (*pt).a);
        std::mem::swap(&mut pt, &mut th);
        println!("{}", (*pt).a);
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
