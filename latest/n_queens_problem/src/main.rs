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
    fn atoi(__nptr: *const i8) -> i32;
    fn abs(_: i32) -> i32;
}
#[no_mangle]
pub static mut count: i32 = 0_i32;
#[no_mangle]
pub extern "C" fn solve(mut n: i32, mut col: i32, mut hist: *mut i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        if col == n {
            count = count.wrapping_add(1);
            print!("\nNo. {}\n-----\n", count);
            let mut i: i32 = 0;
            while i < n {
                let mut j: i32 = 0;
                while j < n {
                    print!(
                        "{}",
                        if j == *hist.offset(i as isize) {
                            'Q' as i32
                        } else if (i + j) & 1_i32 != 0_i32 {
                            ' ' as i32
                        } else {
                            '.' as i32
                        }
                    );
                    j = j.wrapping_add(1);
                    j;
                }
                i = i.wrapping_add(1);
                i;
                print!("{}", '\n' as i32);
            }
            return;
        }
        let mut i_0: i32 = 0;
        let mut j_0: i32 = 0;
        while i_0 < n {
            j_0 = 0_i32;
            while j_0 < col
                && !(*hist.offset(j_0 as isize) == i_0
                    || abs(*hist.offset(j_0 as isize) - i_0) == col - j_0)
            {
                j_0 = j_0.wrapping_add(1);
                j_0;
            }
            if j_0 >= col {
                *hist.offset(col as isize) = i_0;
                solve(n, col.wrapping_add(1), hist);
            }
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
}

fn main_0(mut n: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if n <= 1_i32 || {
            n = atoi(*argv.offset(1_isize));
            n <= 0_i32
        } {
            n = 8_i32;
        }
        let vla = n as usize;
        let mut hist: Vec<i32> = ::std::vec::from_elem(0_i32, vla);
        solve(n, 0, hist.as_mut_ptr());
        0_i32
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr()));
}
