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
pub static mut total: u64 = 0;
#[no_mangle]
pub static mut prim: u64 = 0;
#[no_mangle]
pub static mut max_peri: u64 = 0;
#[no_mangle]
pub static mut U: [[u64; 9]; 3] = [
    [
        1,
        -2_i64 as u64,
        2,
        2,
        -1_i64 as u64,
        2,
        2,
        -2_i64 as u64,
        3,
    ],
    [1, 2, 2, 2, 1, 2, 2, 2, 3],
    [
        -1_i64 as u64,
        2,
        2,
        -2_i64 as u64,
        1,
        2,
        -2_i64 as u64,
        2,
        3,
    ],
];
#[no_mangle]
pub extern "C" fn new_tri(mut in_0: *mut u64) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut t: [u64; 3] = [0; 3];
        let mut p: u64 = (*in_0.offset(0_isize))
            .wrapping_add(*in_0.offset(1_isize))
            .wrapping_add(*in_0.offset(2_isize));
        if p > max_peri {
            return;
        }
        prim = prim.wrapping_add(1);
        prim;
        total = total.wrapping_add(max_peri.wrapping_div(p));
        i = 0_i32;
        while i < 3_i32 {
            t[0_usize] = (U[i as usize][0_usize])
                .wrapping_mul(*in_0.offset(0_isize))
                .wrapping_add((U[i as usize][1_usize]).wrapping_mul(*in_0.offset(1_isize)))
                .wrapping_add((U[i as usize][2_usize]).wrapping_mul(*in_0.offset(2_isize)));
            t[1_usize] = (U[i as usize][3_usize])
                .wrapping_mul(*in_0.offset(0_isize))
                .wrapping_add((U[i as usize][4_usize]).wrapping_mul(*in_0.offset(1_isize)))
                .wrapping_add((U[i as usize][5_usize]).wrapping_mul(*in_0.offset(2_isize)));
            t[2_usize] = (U[i as usize][6_usize])
                .wrapping_mul(*in_0.offset(0_isize))
                .wrapping_add((U[i as usize][7_usize]).wrapping_mul(*in_0.offset(1_isize)))
                .wrapping_add((U[i as usize][8_usize]).wrapping_mul(*in_0.offset(2_isize)));
            new_tri(t.as_mut_ptr());
            i += 1_i32;
            i;
        }
    }
}

fn main_0() -> i32 {
    let mut seed: [u64; 3] = [3, 4, 5];
// SAFETY: machine generated unsafe code
    unsafe {
        max_peri = 10;
        while max_peri <= 100000000 {
            prim = 0;
            total = prim;
            new_tri(seed.as_mut_ptr());
            println!(
                "Up to {}: {} triples, {} primitives.",
                max_peri, total, prim
            );
            max_peri = max_peri.wrapping_mul(10);
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
