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


extern "C" {}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if argc < 2_i32 {
            println!("Enter an argument. Example 1234 or dcba:");
            return 0_i32;
        }
        let mut x: i32 = 0;
        x = 0_i32;
        while i32::from(*(*argv.offset(1_isize)).offset(x as isize)) != '\0' as i32 {
            x = x.wrapping_add(1);
            x;
        }
        let mut f: i32 = 0;
        let mut v: i32 = 0;
        let mut m: i32 = 0;
        f = 0_i32;
        while f < x {
            v = x.wrapping_sub(1);
            while v > f {
                if i32::from(*(*argv.offset(1_isize)).offset((v.wrapping_sub(1i32)) as isize))
                    > i32::from(*(*argv.offset(1_isize)).offset(v as isize))
                {
                    m = i32::from(*(*argv.offset(1_isize)).offset((v.wrapping_sub(1i32)) as isize));
                    *(*argv.offset(1_isize)).offset((v.wrapping_sub(1i32)) as isize) =
                        *(*argv.offset(1_isize)).offset(v as isize);
                    *(*argv.offset(1_isize)).offset(v as isize) = m as i8;
                }
                v = v.wrapping_sub(1);
                v;
            }
            f = f.wrapping_add(1);
            f;
        }
        let vla = x as usize;
        let mut a: Vec<i8> = ::std::vec::from_elem(0, vla);
        let mut k: i32 = 0;
        let mut fact: i32 = k.wrapping_add(1);
        while k != x {
            *a.as_mut_ptr().offset(k as isize) = *(*argv.offset(1_isize)).offset(k as isize);
            k = k.wrapping_add(1);
            k;
            fact = k.wrapping_mul(fact);
        }
        *a.as_mut_ptr().offset(k as isize) = '\0' as i8;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut y: i32 = 0;
        let mut c: i8 = 0;
        while y != fact {
            println!("{}", build_str_from_raw_ptr(a.as_mut_ptr().cast::<u8>()));
            i = x.wrapping_sub(2);
            while i32::from(*a.as_mut_ptr().offset(i as isize))
                > i32::from(*a.as_mut_ptr().offset((i.wrapping_add(1i32)) as isize))
            {
                i = i.wrapping_sub(1);
                i;
            }
            j = x.wrapping_sub(1);
            while i32::from(*a.as_mut_ptr().offset(j as isize))
                < i32::from(*a.as_mut_ptr().offset(i as isize))
            {
                j = j.wrapping_sub(1);
                j;
            }
            c = *a.as_mut_ptr().offset(j as isize);
            *a.as_mut_ptr().offset(j as isize) = *a.as_mut_ptr().offset(i as isize);
            *a.as_mut_ptr().offset(i as isize) = c;
            i = i.wrapping_add(1);
            i;
            j = x.wrapping_sub(1);
            while j > i {
                c = *a.as_mut_ptr().offset(i as isize);
                *a.as_mut_ptr().offset(i as isize) = *a.as_mut_ptr().offset(j as isize);
                *a.as_mut_ptr().offset(j as isize) = c;
                i = i.wrapping_add(1);
                i;
                j = j.wrapping_sub(1);
                j;
            }
            y = y.wrapping_add(1);
            y;
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
