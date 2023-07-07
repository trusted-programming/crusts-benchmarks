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
    fn atoi(__nptr: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn __ctype_b_loc() -> *mut *const u16;
}
pub const _ISalnum: u32 = 8;
pub const _ISpunct: u32 = 4;
pub const _IScntrl: u32 = 2;
pub const _ISblank: u32 = 1;
pub const _ISgraph: u32 = 32768;
pub const _ISprint: u32 = 16384;
pub const _ISspace: u32 = 8192;
pub const _ISxdigit: u32 = 4096;
pub const _ISdigit: u32 = 2048;
pub const _ISalpha: u32 = 1024;
pub const _ISlower: u32 = 512;
pub const _ISupper: u32 = 256;
#[no_mangle]
pub extern "C" fn doublyEvenMagicSquare(mut n: i32) -> *mut *mut i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if n < 4_i32 || n % 4_i32 != 0_i32 {
            return std::ptr::null_mut::<*mut i32>();
        }
        let mut bits: i32 = 38505;
        let mut size: i32 = n.wrapping_mul(n);
        let mut mult: i32 = n.wrapping_div(4);
        let mut i: i32 = 0;
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        let mut bitPos: i32 = 0;
        let mut result: *mut *mut i32 =
            malloc((n as u64).wrapping_mul(::core::mem::size_of::<*mut i32>() as u64)).cast::<*mut i32>();
        i = 0_i32;
        while i < n {
            let fresh0 = &mut (*result.offset(i as isize));
            *fresh0 =
                malloc((n as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64)).cast::<i32>();
            i = i.wrapping_add(1);
            i;
        }
        r = 0_i32;
        i = 0_i32;
        while r < n {
            c = 0_i32;
            while c < n {
                bitPos = c / mult + r / mult.wrapping_mul(4);
                *(*result.offset(r as isize)).offset(c as isize) = if bits & 1_i32 << bitPos != 0_i32 {
                    i.wrapping_add(1)
                } else {
                    size - i
                };
                c = c.wrapping_add(1);
                c;
                i = i.wrapping_add(1);
                i;
            }
            r = r.wrapping_add(1);
            r;
        }
        result
    }
}

#[no_mangle]
pub extern "C" fn numDigits(mut n: i32) -> i32 {
    let mut count: i32 = 1;
    while n >= 10_i32 {
        n = n.wrapping_div(10);
        count = count.wrapping_add(1);
        count;
    }
    count
}

#[no_mangle]
pub extern "C" fn printMagicSquare(mut square: *mut *mut i32, mut rows: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut baseWidth: i32 = numDigits(rows.wrapping_mul(rows)) + 3;
        print!(
            "Doubly Magic Square of Order : {} and Magic Constant : {}\n\n",
            rows,
            (rows * rows + 1_i32) * rows / 2_i32
        );
        i = 0_i32;
        while i < rows {
            j = 0_i32;
            while j < rows {
                print!(
                    "{1:0$}{2:}",
                    (baseWidth - numDigits(*(*square.offset(i as isize)).offset(j as isize))).unsigned_abs()
                        as usize,
                    "\0",
                    *(*square.offset(i as isize)).offset(j as isize)
                );
                j = j.wrapping_add(1);
                j;
            }
            println!();
            i = i.wrapping_add(1);
            i;
        }
    }
}

fn main_0(mut argC: i32, mut argV: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut n: i32 = 0;
        if argC != 2_i32
            || i32::from(*(*__ctype_b_loc())
                .offset(i32::from(*(*argV.offset(1_isize)).offset(0_isize)) as isize))
                & _ISdigit as i32
                == 0_i32
        {
            print!(
                "Usage : {} <integer specifying rows in magic square>",
                build_str_from_raw_ptr((*argV.offset(0_isize)).cast::<u8>())
            );
        } else {
            n = atoi(*argV.offset(1_isize));
            printMagicSquare(doublyEvenMagicSquare(n), n);
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
