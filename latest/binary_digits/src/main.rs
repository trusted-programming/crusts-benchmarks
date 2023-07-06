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
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    fn log10(_: f64) -> f64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: u64 = 0;
        while i < 20 {
            let mut binstr: *mut i8 = bin(i as u32);
            println!("{}", build_str_from_raw_ptr(binstr.cast::<u8>()));
            free(binstr.cast::<libc::c_void>());
            i = i.wrapping_add(1);
            i;
        }
        0_i32
    }
}

#[no_mangle]
pub extern "C" fn bin(mut x: u32) -> *mut i8 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut bits: u64 = (if x == 0u32 {
            1_f64
        } else {
            log10(f64::from(x)) / log10(2_f64) + 1_f64
        }) as u64;
        let mut ret: *mut i8 = malloc(
            bits.wrapping_add(1)
                .wrapping_mul(::core::mem::size_of::<i8>() as u64),
        ).cast::<i8>();
        let mut i: u64 = 0;
        while i < bits {
            *ret.offset(bits.wrapping_sub(i).wrapping_sub(1) as isize) =
                (if x & 1 != 0 { '1' as i32 } else { '0' as i32 }) as i8;
            x >>= 1_i32;
            i = i.wrapping_add(1);
            i;
        }
        *ret.offset(bits as isize) = '\0' as i8;
        ret
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
