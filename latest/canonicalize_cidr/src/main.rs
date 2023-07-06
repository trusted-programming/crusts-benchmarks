#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct cidr_tag {
    pub address: u32,
    pub mask_length: u32,
}
pub type cidr_t = cidr_tag;
#[no_mangle]
pub extern "C" fn cidr_parse(mut str: *const i8, mut cidr: *mut cidr_t) -> bool {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;
        let mut d: i32 = 0;
        let mut m: i32 = 0;
        if sscanf(
            str,
            (b"%d.%d.%d.%d/%d\0" as *const u8).cast::<i8>(),
            &mut a as *mut i32,
            &mut b as *mut i32,
            &mut c as *mut i32,
            &mut d as *mut i32,
            &mut m as *mut i32,
        ) != 5_i32
        {
            return 0_i32 != 0_i32;
        }
        if !(1_i32..=32_i32).contains(&m)
            || !(0_i32..=255_i32).contains(&a)
            || !(0_i32..=255_i32).contains(&b)
            || !(0_i32..=255_i32).contains(&c)
            || !(0_i32..=255_i32).contains(&d)
        {
            return 0_i32 != 0_i32;
        }
        let mut mask: u32 = !((1i32 << (32 - m)) - 1) as u32;
        let mut address: u32 = ((a << 24i32) + (b << 16) + (c << 8) + d) as u32;
        address &= mask;
        (*cidr).address = address;
        (*cidr).mask_length = m as u32;
        1_i32 != 0_i32
    }
}

#[no_mangle]
pub extern "C" fn cidr_format(mut cidr: *const cidr_t, mut str: *mut i8, mut size: u64) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut address: u32 = (*cidr).address;
        let mut d: u32 = address & 255;
        address >>= 8_i32;
        let mut c: u32 = address & 255;
        address >>= 8_i32;
        let mut b: u32 = address & 255;
        address >>= 8_i32;
        let mut a: u32 = address & 255;
        snprintf(
            str,
            size,
            (b"%u.%u.%u.%u/%u\0" as *const u8).cast::<i8>(),
            a,
            b,
            c,
            d,
            (*cidr).mask_length,
        );
    }
}

fn main_0(mut _argc: i32, mut _argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut tests: [*const i8; 6] = [
            (b"87.70.141.1/22\0" as *const u8).cast::<i8>(),
            (b"36.18.154.103/12\0" as *const u8).cast::<i8>(),
            (b"62.62.197.11/29\0" as *const u8).cast::<i8>(),
            (b"67.137.119.181/4\0" as *const u8).cast::<i8>(),
            (b"161.214.74.21/24\0" as *const u8).cast::<i8>(),
            (b"184.232.176.184/18\0" as *const u8).cast::<i8>(),
        ];
        let mut i: i32 = 0;
        while (i as u64)
            < (::core::mem::size_of::<[*const i8; 6]>() as u64)
                .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
        {
            let mut cidr: cidr_t = cidr_t {
                address: 0,
                mask_length: 0,
            };
            if cidr_parse(tests[i as usize], &mut cidr) {
                let mut out: [i8; 32] = [0; 32];
                cidr_format(
                    &mut cidr,
                    out.as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 32]>() as u64,
                );
                println!(
                    "{:-18} -> {}",
                    build_str_from_raw_ptr(tests[i as usize] as *mut u8),
                    build_str_from_raw_ptr(out.as_mut_ptr().cast::<u8>())
                );
            } else {
                fprintf(
                    stderr,
                    (b"%s: invalid CIDR\n\0" as *const u8).cast::<i8>(),
                    tests[i as usize],
                );
            }
            i += 1_i32;
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
