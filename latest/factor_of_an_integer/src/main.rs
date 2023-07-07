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
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct Factors {
    pub list: *mut i32,
    pub count: i16,
}
#[no_mangle]
pub extern "C" fn xferFactors(mut fctrs: *mut Factors, mut flist: *mut i32, mut flix: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut ix: i32 = 0;
        let mut ij: i32 = 0;
        let mut newSize: i32 = i32::from((*fctrs).count) + flix;
        if newSize > flix {
            (*fctrs).list = realloc(
                (*fctrs).list.cast::<libc::c_void>(),
                (newSize as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
            ).cast::<i32>();
        } else {
            (*fctrs).list =
                malloc((newSize as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64)).cast::<i32>();
        }
        ij = 0_i32;
        ix = i32::from((*fctrs).count);
        while ix < newSize {
            *((*fctrs).list).offset(ix as isize) = *flist.offset(ij as isize);
            ij = ij.wrapping_add(1);
            ij;
            ix = ix.wrapping_add(1);
            ix;
        }
        (*fctrs).count = newSize as i16;
    }
}

#[no_mangle]
pub extern "C" fn factor(mut num: i32, mut fctrs: *mut Factors) -> *mut Factors {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut flist: [i32; 301] = [0; 301];
        let mut flix: i32 = 0;
        let mut dvsr: i32 = 0;
        flix = 0_i32;
        (*fctrs).count = 0;
        free((*fctrs).list.cast::<libc::c_void>());
        (*fctrs).list = std::ptr::null_mut::<i32>();
        dvsr = 1_i32;
        while dvsr * dvsr < num {
            if num % dvsr == 0_i32 {
                if flix == 300_i32 {
                    xferFactors(fctrs, flist.as_mut_ptr(), flix);
                    flix = 0_i32;
                }
                let fresh0 = flix;
                flix = flix.wrapping_add(1);
                flist[fresh0 as usize] = dvsr;
                let fresh1 = flix;
                flix = flix.wrapping_add(1);
                flist[fresh1 as usize] = num / dvsr;
            }
            dvsr = dvsr.wrapping_add(1);
            dvsr;
        }
        if dvsr * dvsr == num {
            let fresh2 = flix;
            flix = flix.wrapping_add(1);
            flist[fresh2 as usize] = dvsr;
        }
        if flix > 0_i32 {
            xferFactors(fctrs, flist.as_mut_ptr(), flix);
        }
        fctrs
    }
}

fn main_0(mut _argc: i32, mut _argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut nums2factor: [i32; 4] = [2059, 223092870, 3135, 45];
        let mut ftors: Factors = {
            
            Factors {
                list: std::ptr::null_mut::<i32>(),
                count: 0,
            }
        };
        let mut sep: i8 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        i = 0_i32;
        while i < 4_i32 {
            factor(nums2factor[i as usize], &mut ftors);
            print!("\nfactors of {} are:\n  ", nums2factor[i as usize]);
            sep = ' ' as i8;
            j = 0_i32;
            while j < i32::from(ftors.count) {
                print!("{} {}", i32::from(sep), *(ftors.list).offset(j as isize));
                sep = ',' as i8;
                j = j.wrapping_add(1);
                j;
            }
            println!();
            i = i.wrapping_add(1);
            i;
        }
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
