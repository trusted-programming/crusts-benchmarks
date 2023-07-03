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

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn perror(__s: *const i8);
    fn puts(__s: *const i8) -> i32;
    fn getc(__stream: *mut FILE) -> i32;
    fn exit(_: i32) -> !;
    fn log10f(_: libc::c_float) -> libc::c_float;
}
#[derive(Copy, Clone)]
#[repr(C)]
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
#[no_mangle]
pub extern "C" fn benford_distribution() -> *mut libc::c_float {
// SAFETY: machine generated unsafe code
    unsafe {
        static mut prob: [libc::c_float; 9] = [0.; 9];
        let mut i: i32 = 1;
        while i < 10_i32 {
            prob[(i - 1i32) as usize] = log10f((1_f64 + 1.0f64 / f64::from(i)) as libc::c_float);
            i = i.wrapping_add(1);
            i;
        }
        prob.as_mut_ptr()
    }
}

#[no_mangle]
pub extern "C" fn get_actual_distribution(mut fn_0: *mut i8) -> *mut libc::c_float {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut input: *mut FILE = fopen(fn_0, (b"r\0" as *const u8).cast::<i8>());
        if input.is_null() {
            perror((b"Can't open file\0" as *const u8).cast::<i8>());
            exit(1);
        }
        let mut tally: [i32; 9] = [0; 9];
        let mut c: i8 = 0;
        let mut total: i32 = 0;
        loop {
            c = getc(input) as i8;
            if i32::from(c) == -1_i32 {
                break;
            }
            while i32::from(c) < '1' as i32 || i32::from(c) > '9' as i32 {
                c = getc(input) as i8;
            }
            tally[(i32::from(c) - '1' as i32) as usize] += 1_i32;
            tally[(i32::from(c) - '1' as i32) as usize];
            total = total.wrapping_add(1);
            total;
            loop {
                c = getc(input) as i8;
                if !(i32::from(c) != '\n' as i32 && i32::from(c) != -1_i32) {
                    break;
                }
            }
        }
        fclose(input);
        static mut freq: [libc::c_float; 9] = [0.; 9];
        let mut i: i32 = 0;
        while i < 9_i32 {
            freq[i as usize] = tally[i as usize] as libc::c_float / total as libc::c_float;
            i = i.wrapping_add(1);
            i;
        }
        freq.as_mut_ptr()
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if argc != 2_i32 {
            println!("Usage: benford <file>");
            return 1_i32;
        }
        let mut actual: *mut libc::c_float = get_actual_distribution(*argv.offset(1_isize));
        let mut expected: *mut libc::c_float = benford_distribution();
        puts((b"digit\tactual\texpected\0" as *const u8).cast::<i8>());
        let mut i: i32 = 0;
        while i < 9_i32 {
            println!(
                "{}	{:.3}	{:.3}",
                i + 1_i32,
                f64::from(*actual.offset(i as isize)),
                f64::from(*expected.offset(i as isize))
            );
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
// SAFETY: machine generated unsafe code
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr()),
        );
    }
}
