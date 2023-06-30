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
}
#[no_mangle]
pub extern "C" fn even_sel(mut x: i32) -> i32 {
    i32::from(x & 1_i32 == 0_i32)
}

#[no_mangle]
pub extern "C" fn tri_sel(mut x: i32) -> i32 {
    x % 3_i32
}

#[no_mangle]
pub extern "C" fn grep(
    mut in_0: *mut i32,
    mut len: i32,
    mut outlen: *mut i32,
    mut sel: Option<unsafe extern "C" fn(i32) -> i32>,
    mut inplace: i32,
) -> *mut i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut out: *mut i32 = std::ptr::null_mut::<i32>();
        if inplace != 0_i32 {
            out = in_0;
        } else {
            out =
                malloc((::core::mem::size_of::<i32>() as u64).wrapping_mul(len as u64)).cast::<i32>();
        }
        j = 0_i32;
        i = j;
        while i < len {
            if sel.expect("non-null function pointer")(*in_0.offset(i as isize)) != 0_i32 {
                let fresh0 = j;
                j = j.wrapping_add(1);
                *out.offset(fresh0 as isize) = *in_0.offset(i as isize);
            }
            i = i.wrapping_add(1);
            i;
        }
        if inplace == 0_i32 && j < len {
            out = realloc(
                out.cast::<libc::c_void>(),
                (::core::mem::size_of::<i32>() as u64).wrapping_mul(j as u64),
            ).cast::<i32>();
        }
        *outlen = j;
        out
    }
}

fn main_0() -> i32 {
    unsafe {
        let mut in_0: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut i: i32 = 0;
        let mut len: i32 = 0;
        let mut even: *mut i32 = grep(
            in_0.as_mut_ptr(),
            10,
            &mut len,
            Some(even_sel as unsafe extern "C" fn(i32) -> i32),
            0,
        );
        print!("Filtered even:");
        i = 0_i32;
        while i < len {
            print!(" {}", *even.offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        println!();
        grep(
            in_0.as_mut_ptr(),
            8,
            &mut len,
            Some(tri_sel as unsafe extern "C" fn(i32) -> i32),
            1,
        );
        print!("In-place filtered not multiple of 3:");
        i = 0_i32;
        while i < len {
            print!(" {}", in_0[i as usize]);
            i = i.wrapping_add(1);
            i;
        }
        println!();
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
