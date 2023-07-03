#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.add(str_size) != 0 {
            str_size = str_size.wrapping_add(1);
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    fn rand() -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
}
#[no_mangle]
pub static mut target: [i8; 29] =
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe { *::core::mem::transmute::<&[u8; 29], &[i8; 29]>(b"METHINKS IT IS LIKE A WEASEL\0") };
#[no_mangle]
pub static mut tbl: [i8; 28] =
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe { *::core::mem::transmute::<&[u8; 28], &[i8; 28]>(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ \0") };
#[no_mangle]
pub extern "C" fn irand(mut n: i32) -> i32 {
    let mut r: i32 = 0;
    let mut rand_max: i32 = 2147483647 - 2147483647 % n;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        loop {
            r = rand();
            if r < rand_max {
                break;
            }
        }
    }
    r / (rand_max / n)
}

#[no_mangle]
pub extern "C" fn unfitness(mut a: *const i8, mut b: *const i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut sum: i32 = 0;
        i = 0_i32;
        while *a.offset(i as isize) != 0 {
            sum += i32::from(i32::from(*a.offset(i as isize)) != i32::from(*b.offset(i as isize)));
            i = i.wrapping_add(1);
            i;
        }
        sum
    }
}

#[no_mangle]
pub extern "C" fn mutate(mut a: *const i8, mut b: *mut i8) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        i = 0_i32;
        while *a.offset(i as isize) != 0 {
            *b.offset(i as isize) = (if irand(15) != 0_i32 {
                i32::from(*a.offset(i as isize))
            } else {
                i32::from(tbl[irand((::core::mem::size_of::<[i8; 28]>() as u64).wrapping_sub(1) as i32)
                    as usize])
            }) as i8;
            i = i.wrapping_add(1);
            i;
        }
        *b.offset(i as isize) = '\0' as i8;
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut best_i: i32 = 0;
    let mut unfit: i32 = 0;
    let mut best: i32 = 0;
    let mut iters: i32 = 0;
    let mut specimen: [[i8; 29]; 30] = [[0; 29]; 30];
    i = 0_i32;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        while target[i as usize] != 0 {
            specimen[0_usize][i as usize] =
                tbl[irand((::core::mem::size_of::<[i8; 28]>() as u64).wrapping_sub(1) as i32)
                    as usize];
            i = i.wrapping_add(1);
            i;
        }
    }
    specimen[0_usize][i as usize] = 0;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        loop {
            i = 1_i32;
            while i < 30_i32 {
                mutate(
                    (specimen[0_usize]).as_mut_ptr(),
                    (specimen[i as usize]).as_mut_ptr(),
                );
                i = i.wrapping_add(1);
                i;
            }
            i = 0_i32;
            best_i = i;
            while i < 30_i32 {
                unfit = unfitness(target.as_ptr(), (specimen[i as usize]).as_mut_ptr());
                if unfit < best || i == 0_i32 {
                    best = unfit;
                    best_i = i;
                }
                i = i.wrapping_add(1);
                i;
            }
            if best_i != 0_i32 {
                strcpy(
                    (specimen[0_usize]).as_mut_ptr(),
                    (specimen[best_i as usize]).as_mut_ptr(),
                );
            }
            let fresh0 = iters;
            iters = iters.wrapping_add(1);
            println!(
                "iter {}, score {}: {}",
                fresh0,
                best,
                build_str_from_raw_ptr((specimen[0_usize]).as_mut_ptr().cast::<u8>())
            );
            if best == 0_i32 {
                break;
            }
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
