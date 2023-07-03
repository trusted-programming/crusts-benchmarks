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
#[no_mangle]
pub extern "C" fn transpose(
    mut dest: *mut libc::c_void,
    mut src: *mut libc::c_void,
    mut src_h: i32,
    mut src_w: i32,
) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let vla = src_h as usize;
        let mut d: *mut f64 = dest.cast::<f64>();
        let vla_0 = src_w as usize;
        let mut s: *mut f64 = src.cast::<f64>();
        i = 0_i32;
        while i < src_h {
            j = 0_i32;
            while j < src_w {
                *d.offset(j as isize * vla as isize).offset(i as isize) =
                    *s.offset(i as isize * vla_0 as isize).offset(j as isize);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut a: [[f64; 5]; 3] = [
        [f64::from(0_i32), 1_f64, 2_f64, 3_f64, 4_f64],
        [5_f64, 6_f64, 7_f64, 8_f64, 9_f64],
        [1_f64, f64::from(0_i32), f64::from(0_i32), f64::from(0_i32), 42_f64],
    ];
    let mut b: [[f64; 3]; 5] = [[0.; 3]; 5];
    transpose(
        b.as_mut_ptr().cast::<libc::c_void>(),
        a.as_mut_ptr().cast::<libc::c_void>(),
        3,
        5,
    );
    i = 0_i32;
    while i < 5_i32 {
        j = 0_i32;
        while j < 3_i32 {
            if j == 2_i32 {
                print!("{}{}", b[i as usize][j as usize], '\n' as i32)
            } else {
                print!("{}{}", b[i as usize][j as usize], ' ' as i32)
            };
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
