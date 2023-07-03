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
    fn scanf(_: *const i8, _: ...) -> i32;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn log2(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn makehist(mut S: *mut i8, mut hist: *mut i32, mut len: i32) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut wherechar: [i32; 256] = [0; 256];
        let mut i: i32 = 0;
        let mut histlen: i32 = 0;
        histlen = 0_i32;
        i = 0_i32;
        while i < 256_i32 {
            wherechar[i as usize] = -1_i32;
            i = i.wrapping_add(1);
            i;
        }
        i = 0_i32;
        while i < len {
            if wherechar[i32::from(*S.offset(i as isize)) as usize] == -1_i32 {
                wherechar[i32::from(*S.offset(i as isize)) as usize] = histlen;
                histlen = histlen.wrapping_add(1);
                histlen;
            }
            let fresh0 = &mut (*hist.offset(wherechar[i32::from(*S.offset(i as isize)) as usize] as isize));
            *fresh0 += 1_i32;
            *fresh0;
            i = i.wrapping_add(1);
            i;
        }
        histlen
    }
}

#[no_mangle]
pub extern "C" fn entropy(mut hist: *mut i32, mut histlen: i32, mut len: i32) -> f64 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut H: f64 = 0.;
        H = f64::from(0_i32);
        i = 0_i32;
        while i < histlen {
            H -= f64::from(*hist.offset(i as isize)) / f64::from(len)
                * log2(f64::from(*hist.offset(i as isize)) / f64::from(len));
            i = i.wrapping_add(1);
            i;
        }
        H
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut S: [i8; 100] = [0; 100];
        let mut len: i32 = 0;
        let mut hist: *mut i32 = std::ptr::null_mut::<i32>();
        let mut histlen: i32 = 0;
        let mut H: f64 = 0.;
        scanf((b"%[^\n]\0" as *const u8).cast::<i8>(), S.as_mut_ptr());
        len = strlen(S.as_mut_ptr()) as i32;
        hist = calloc(len as u64, ::core::mem::size_of::<i32>() as u64).cast::<i32>();
        histlen = makehist(S.as_mut_ptr(), hist, len);
        H = entropy(hist, histlen, len);
        println!("{}", H);
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
