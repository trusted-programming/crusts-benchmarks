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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const i8) -> u64;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct edit_s {
    pub c1: i8,
    pub c2: i8,
    pub n: i32,
    pub next: edit,
}
pub type edit = *mut edit_s;
pub type edit_t = edit_s;
#[no_mangle]
pub extern "C" fn leven(mut a: *mut i8, mut b: *mut i8) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut la: i32 = strlen(a) as i32;
        let mut lb: i32 = strlen(b) as i32;
        let mut tbl: *mut edit = malloc(
            (::core::mem::size_of::<edit>() as u64).wrapping_mul((la.wrapping_add(1)) as u64),
        ).cast::<*mut edit_s>();
        let fresh0 = &mut (*tbl.offset(0_isize));
        *fresh0 = calloc(
            ((la.wrapping_add(1)) * (lb.wrapping_add(1))) as u64,
            ::core::mem::size_of::<edit_t>() as u64,
        ).cast::<edit_s>();
        i = 1_i32;
        while i <= la {
            let fresh1 = &mut (*tbl.offset(i as isize));
            *fresh1 = (*tbl.offset((i.wrapping_sub(1i32)) as isize))
                .offset((lb.wrapping_add(1)) as isize);
            i = i.wrapping_add(1);
            i;
        }
        i = la;
        while i >= 0_i32 {
            let mut aa: *mut i8 = a.offset(i as isize);
            j = lb;
            while j >= 0_i32 {
                let mut bb: *mut i8 = b.offset(j as isize);
                if !(*aa == 0 && *bb == 0) {
                    let mut e: edit =
                        &mut *(*tbl.offset(i as isize)).offset(j as isize) as *mut edit_s;
                    let mut repl: edit = &mut *(*tbl.offset((i.wrapping_add(1i32)) as isize))
                        .offset((j.wrapping_add(1i32)) as isize)
                        as *mut edit_s;
                    let mut dela: edit = &mut *(*tbl.offset((i.wrapping_add(1i32)) as isize))
                        .offset(j as isize) as *mut edit_s;
                    let mut delb: edit = &mut *(*tbl.offset(i as isize))
                        .offset((j.wrapping_add(1i32)) as isize)
                        as *mut edit_s;
                    (*e).c1 = *aa;
                    (*e).c2 = *bb;
                    if *aa == 0 {
                        (*e).next = delb;
                        (*e).n = (*(*e).next).n + 1_i32;
                    } else if *bb == 0 {
                        (*e).next = dela;
                        (*e).n = (*(*e).next).n + 1_i32;
                    } else {
                        (*e).next = repl;
                        if i32::from(*aa) == i32::from(*bb) {
                            (*e).n = (*(*e).next).n;
                        } else {
                            if (*(*e).next).n > (*delb).n {
                                (*e).next = delb;
                                (*e).c1 = 0;
                            }
                            if (*(*e).next).n > (*dela).n {
                                (*e).next = dela;
                                (*e).c1 = *aa;
                                (*e).c2 = 0;
                            };
                            (*e).n = (*(*e).next).n + 1_i32;
                        }
                    }
                }
                j = j.wrapping_sub(1);
                j;
            }
            i = i.wrapping_sub(1);
            i;
        }
        let mut p: edit = *tbl.offset(0_isize);
        println!(
            "{} -> {}: {} edits",
            build_str_from_raw_ptr(a.cast::<u8>()),
            build_str_from_raw_ptr(b.cast::<u8>()),
            (*p).n
        );
        while !((*p).next).is_null() {
            if i32::from((*p).c1) == i32::from((*p).c2) {
                print!("{}", i32::from((*p).c1));
            } else {
                print!("{}", '(' as i32);
                if (*p).c1 != 0 {
                    print!("{}", i32::from((*p).c1));
                }
                print!("{}", ',' as i32);
                if (*p).c2 != 0 {
                    print!("{}", i32::from((*p).c2));
                }
                print!("{}", ')' as i32);
            }
            p = (*p).next;
        }
        print!("{}", '\n' as i32);
        free((*tbl.offset(0_isize)).cast::<libc::c_void>());
        free(tbl.cast::<libc::c_void>());
    }
}

fn main_0() -> i32 {
    leven(
        (b"raisethysword\0" as *const u8).cast::<i8>() as *mut i8,
        (b"rosettacode\0" as *const u8).cast::<i8>() as *mut i8,
    );
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
