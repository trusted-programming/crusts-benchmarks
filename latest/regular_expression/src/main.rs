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
            str_size = str_size.wrapping_add(1);
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}

#[macro_use]
extern crate c2rust_bitfields;


extern "C" {
    pub type re_dfa_t;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn regcomp(__preg: *mut regex_t, __pattern: *const i8, __cflags: i32) -> i32;
    fn regexec(
        __preg: *const regex_t,
        __String: *const i8,
        __nmatch: u64,
        __pmatch: *mut regmatch_t,
        __eflags: i32,
    ) -> i32;
    fn regfree(__preg: *mut regex_t);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
#[derive(Debug)]
pub struct re_pattern_buffer {
    pub __buffer: *mut re_dfa_t,
    pub __allocated: u64,
    pub __used: u64,
    pub __syntax: u64,
    pub __fastmap: *mut i8,
    pub __translate: *mut u8,
    pub re_nsub: u64,
    #[bitfield(name = "__can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "__regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "__fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "__no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "__not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "__not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "__newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor:
        [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct regmatch_t {
    pub rm_so: i32,
    pub rm_eo: i32,
}
fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut preg : regex_t = regex_t {
            __buffer : std::ptr::null_mut::<re_dfa_t>(),
            __allocated : 0,
            __used : 0,
            __syntax : 0,
            __fastmap : std::ptr::null_mut::<i8>(),
            __translate : std::ptr::null_mut::<u8>(),
            re_nsub : 0,
            __can_be_null___regs_allocated___fastmap_accurate___no_sub___not_bol___not_eol___newline_anchor : [0; 1],
            c2rust_padding : [0; 7],
        };
        let mut substmatch: [regmatch_t; 1] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 1];
        let mut tp: *const i8 = (b"string$\0" as *const u8).cast::<i8>();
        let mut t1: *const i8 = (b"this is a matching string\0" as *const u8).cast::<i8>();
        let mut t2: *const i8 = (b"this is not a matching string!\0" as *const u8).cast::<i8>();
        let mut ss: *const i8 = (b"istyfied\0" as *const u8).cast::<i8>();
        regcomp(&mut preg, (b"string$\0" as *const u8).cast::<i8>(), 1);
        if regexec(&mut preg, t1, 0, std::ptr::null_mut::<regmatch_t>(), 0) == 0_i32 {
            println!(
                "{} \0matched with {}",
                build_str_from_raw_ptr(t1 as *mut u8),
                build_str_from_raw_ptr(tp as *mut u8)
            )
        } else {
            println!(
                "{} did not \0matched with {}",
                build_str_from_raw_ptr(t1 as *mut u8),
                build_str_from_raw_ptr(tp as *mut u8)
            )
        };
        if regexec(&mut preg, t2, 0, std::ptr::null_mut::<regmatch_t>(), 0) == 0_i32 {
            println!(
                "{} \0matched with {}",
                build_str_from_raw_ptr(t2 as *mut u8),
                build_str_from_raw_ptr(tp as *mut u8)
            )
        } else {
            println!(
                "{} did not \0matched with {}",
                build_str_from_raw_ptr(t2 as *mut u8),
                build_str_from_raw_ptr(tp as *mut u8)
            )
        };
        regfree(&mut preg);
        regcomp(&mut preg, (b"a[a-z]+\0" as *const u8).cast::<i8>(), 1);
        if regexec(&mut preg, t1, 1, substmatch.as_mut_ptr(), 0) == 0_i32 {
            let mut ns: *mut i8 = malloc(
                ((substmatch[0_usize].rm_so + 1i32) as u64)
                    .wrapping_add(strlen(ss))
                    .wrapping_add((strlen(t1)).wrapping_sub(substmatch[0_usize].rm_eo as u64))
                    .wrapping_add(2),
            ).cast::<i8>();
            memcpy(
                ns.cast::<libc::c_void>(),
                t1.cast::<libc::c_void>(),
                (substmatch[0_usize].rm_so + 1i32) as u64,
            );
            memcpy(
                (&mut *ns.offset((*substmatch.as_mut_ptr().offset(0_isize)).rm_so as isize) as *mut i8).cast::<libc::c_void>(),
                ss.cast::<libc::c_void>(),
                strlen(ss),
            );
            memcpy(
                (&mut *ns.offset(
                    ((*substmatch.as_mut_ptr().offset(0_isize)).rm_so as u64)
// SAFETY: machine generated unsafe code
                        .wrapping_add((strlen as unsafe extern "C" fn(*const i8) -> u64)(ss))
                        as isize,
                ) as *mut i8).cast::<libc::c_void>(),
                (&*t1.offset((*substmatch.as_mut_ptr().offset(0_isize)).rm_eo as isize) as *const i8).cast::<libc::c_void>(),
                strlen(&*t1.offset((*substmatch.as_mut_ptr().offset(0_isize)).rm_eo as isize)),
            );
            *ns.offset(
                (substmatch[0_usize].rm_so as u64)
                    .wrapping_add(strlen(ss))
                    .wrapping_add(strlen(
                        &*t1.offset((*substmatch.as_mut_ptr().offset(0_isize)).rm_eo as isize),
                    )) as isize,
            ) = 0;
            println!("mod string: {}", build_str_from_raw_ptr(ns.cast::<u8>()));
            free(ns.cast::<libc::c_void>());
        } else {
            println!(
                "the string {} is the same: no matching!",
                build_str_from_raw_ptr(t1 as *mut u8)
            );
        }
        regfree(&mut preg);
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
