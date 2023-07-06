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
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub extern "C" fn hailstone(mut n: i32, mut arry: *mut i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut hs: i32 = 1;
        while n != 1_i32 {
            hs += 1_i32;
            hs;
            if !arry.is_null() {
                let fresh0 = arry;
                arry = arry.offset(1);
                *fresh0 = n;
            }
            n = if n & 1_i32 != 0_i32 { 3_i32 * n + 1_i32 } else { n / 2_i32 };
        }
        if !arry.is_null() {
            let fresh1 = arry;
            arry = arry.offset(1);
            *fresh1 = n;
        }
        hs
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut j: i32 = 0;
        let mut hmax: i32 = 0;
        let mut jatmax: i32 = 0;
        let mut n: i32 = 0;
        let mut arry: *mut i32 = std::ptr::null_mut::<i32>();
        j = 1_i32;
        while j < 100_000_i32 {
            n = hailstone(j, std::ptr::null_mut::<i32>());
            if hmax < n {
                hmax = n;
                jatmax = j;
            }
            j += 1_i32;
            j;
        }
        n = hailstone(27, std::ptr::null_mut::<i32>());
        arry = malloc((n as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64)).cast::<i32>();
        n = hailstone(27, arry);
        println!(
            "[ {}, {}, {}, {}, ...., {}, {}, {}, {}] len={}",
            *arry.offset(0_isize),
            *arry.offset(1_isize),
            *arry.offset(2_isize),
            *arry.offset(3_isize),
            *arry.offset((n - 4i32) as isize),
            *arry.offset((n - 3i32) as isize),
            *arry.offset((n - 2i32) as isize),
            *arry.offset((n - 1i32) as isize),
            n
        );
        println!("Max {} at j= {}", hmax, jatmax);
        free(arry.cast::<libc::c_void>());
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
