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
pub extern "C" fn lcs(sa: *const i8, sb: *const i8, beg: *mut *mut i8, end: *mut *mut i8) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut apos: u64 = 0;
        let mut bpos: u64 = 0;
        let mut len: i32 = 0;
        *beg = std::ptr::null_mut::<i8>();
        *end = std::ptr::null_mut::<i8>();
        len = 0_i32;
        apos = 0;
        while i32::from(*sa.offset(apos as isize)) != 0_i32 {
            bpos = 0;
            while i32::from(*sb.offset(bpos as isize)) != 0_i32 {
                if i32::from(*sa.offset(apos as isize)) == i32::from(*sb.offset(bpos as isize)) {
                    len = 1_i32;
                    while i32::from(*sa.offset(apos.wrapping_add(len as u64) as isize)) != 0_i32
                        && i32::from(*sb.offset(bpos.wrapping_add(len as u64) as isize)) != 0_i32
                        && i32::from(*sa.offset(apos.wrapping_add(len as u64) as isize))
                            == i32::from(*sb.offset(bpos.wrapping_add(len as u64) as isize))
                    {
                        len = len.wrapping_add(1);
                        len;
                    }
                }
                if i64::from(len) > (*end).offset_from(*beg) as i64 {
                    *beg = sa.offset(apos as isize) as *mut i8;
                    *end = (*beg).offset(len as isize);
                    len = 0_i32;
                }
                bpos = bpos.wrapping_add(1);
                bpos;
            }
            apos = apos.wrapping_add(1);
            apos;
        }
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut s1: *mut i8 = (b"thisisatest\0" as *const u8).cast::<i8>() as *mut i8;
        let mut s2: *mut i8 = (b"testing123testing\0" as *const u8).cast::<i8>() as *mut i8;
        let mut beg: *mut i8 = std::ptr::null_mut::<i8>();
        let mut end: *mut i8 = std::ptr::null_mut::<i8>();
        let mut it: *mut i8 = std::ptr::null_mut::<i8>();
        lcs(s1, s2, &mut beg, &mut end);
        it = beg;
        while it != end {
            print!("{}", i32::from(*it));
            it = it.offset(1);
            it;
        }
        println!();
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
