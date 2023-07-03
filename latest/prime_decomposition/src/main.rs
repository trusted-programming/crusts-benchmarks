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
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fread(_: *mut libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn fwrite(_: *const libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn perror(__s: *const i8);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
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
pub static mut pbits: *mut u8 = 0 as *const u8 as *mut u8;
#[no_mangle]
pub static mut bit_pos: [u8; 30] = [
    0,
    (1i32 << 0_i32) as u8,
    0,
    0,
    0,
    0,
    0,
    (1i32 << 1_i32) as u8,
    0,
    0,
    0,
    (1i32 << 2_i32) as u8,
    0,
    (1i32 << 3_i32) as u8,
    0,
    0,
    0,
    (1i32 << 4_i32) as u8,
    0,
    (1i32 << 5_i32) as u8,
    0,
    0,
    0,
    (1i32 << 6_i32) as u8,
    0,
    0,
    0,
    0,
    0,
    (1i32 << 7_i32) as u8,
];
#[no_mangle]
pub static mut rem_num: [u8; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
#[no_mangle]
pub extern "C" fn init_primes() {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut fp: *mut FILE = std::ptr::null_mut::<FILE>();
        let mut s: u32 = 0;
        let mut tgt: u32 = 4;
        pbits = malloc(u64::from((!0u32).wrapping_div(30).wrapping_add(1))).cast::<u8>();
        if pbits.is_null() {
            perror((b"malloc\0" as *const u8).cast::<i8>());
            exit(1);
        }
        fp = fopen(
            (b"primebits\0" as *const u8).cast::<i8>(),
            (b"r\0" as *const u8).cast::<i8>(),
        );
        if !fp.is_null() {
            fread(
                pbits.cast::<libc::c_void>(),
                1,
                u64::from((!0u32).wrapping_div(30).wrapping_add(1)),
                fp,
            );
            fclose(fp);
            return;
        }
        memset(
            pbits.cast::<libc::c_void>(),
            255,
            u64::from((!0u32).wrapping_div(30).wrapping_add(1)),
        );
        s = 7;
        while s <= 65535 {
            if s > tgt {
                tgt = (tgt).wrapping_mul(2);
                fprintf(stderr, (b"sieve %u\n\0" as *const u8).cast::<i8>(), s);
            }
            sieve(s);
            s = next_prime(s);
        }
        fp = fopen(
            (b"primebits\0" as *const u8).cast::<i8>(),
            (b"w\0" as *const u8).cast::<i8>(),
        );
        fwrite(
            pbits as *const libc::c_void,
            1,
            u64::from((!0u32).wrapping_div(30).wrapping_add(1)),
            fp,
        );
        fclose(fp);
    }
}

#[no_mangle]
pub extern "C" fn is_prime(mut x: u64) -> i32 {
    let mut p: u32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        if x > 5 {
            if x < !0 {
                return i32::from(*pbits.offset(x.wrapping_div(30) as isize))
                    & i32::from(bit_pos[x.wrapping_rem(30) as usize]);
            }
            p = 2;
            while p != 0 && u64::from(p).wrapping_mul(u64::from(p)) <= x {
                if x.wrapping_rem(u64::from(p)) == 0 {
                    return 0_i32;
                }
                p = next_prime(p);
            }
            return 1_i32;
        }
    }
    i32::from(x == 2 || x == 3 || x == 5)
}

#[no_mangle]
pub extern "C" fn sieve(mut p: u32) {
    let mut b: [u8; 8] = [0; 8];
    let mut ofs: [i64; 8] = [0; 8];
    let mut i: i32 = 0;
    let mut q: i32 = 0;
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 8_i32 {
            q = u32::from(rem_num[i as usize]).wrapping_mul(p) as i32;
            b[i as usize] = !i32::from(bit_pos[(q % 30i32) as usize]) as u8;
            ofs[i as usize] = i64::from(q / 30i32);
            i = i.wrapping_add(1);
            i;
        }
    }
    q = ofs[1_usize] as i32;
    i = 7_i32;
    while i != 0_i32 {
        ofs[i as usize] -= ofs[(i - 1i32) as usize];
        i = i.wrapping_sub(1);
        i;
    }
    ofs[0_usize] = i64::from(p);
    i = 1_i32;
    while i < 8_i32 {
        ofs[0_usize] -= ofs[i as usize];
        i = i.wrapping_add(1);
        i;
    }
    i = 1_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while (q as u32) < (!0u32).wrapping_div(30).wrapping_add(1) {
            let fresh0 = &mut (*pbits.offset(q as isize));
            *fresh0 = (i32::from(*fresh0) & i32::from(b[i as usize])) as u8;
            i = (i + 1_i32) & 7_i32;
            q = (i64::from(q) + ofs[i as usize]) as i32;
        }
    }
}

#[no_mangle]
pub extern "C" fn next_prime(mut p: u32) -> u32 {
    let mut addr: i64 = 0;
    let mut bits: u8 = 0;
    let mut rem: u8 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        if p > 5 {
            addr = i64::from(p.wrapping_div(30));
            bits = (i32::from(bit_pos[p.wrapping_rem(30u32) as usize]) << 1_i32) as u8;
            rem = 0;
            while (1_i32 << i32::from(rem)) < i32::from(bits) {
                rem = rem.wrapping_add(1);
                rem;
            }
            while i32::from(*pbits.offset(addr as isize)) < i32::from(bits) || bits == 0 {
                addr = addr.wrapping_add(1);
                if addr >= i64::from((!0u32).wrapping_div(30).wrapping_add(1)) {
                    return 0;
                }
                bits = 1;
                rem = 0;
            }
            if addr >= i64::from((!0u32).wrapping_div(30).wrapping_add(1)) {
                return 0;
            }
            while i32::from(*pbits.offset(addr as isize)) & i32::from(bits) == 0_i32 {
                rem = rem.wrapping_add(1);
                rem;
                bits = (i32::from(bits) << 1i32) as u8;
            }
            p = (addr * 30 + i64::from(rem_num[rem as usize])) as u32;
            return p;
        }
    }
    match p {
        2 => return 3,
        3 => return 5,
        5 => return 7,
        _ => {}
    }
    2
}

#[no_mangle]
pub extern "C" fn decompose(mut n: u64, mut f: *mut u64) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut p: u32 = 0;
        let mut i: i32 = 0;
        if n <= !0 && is_prime(n) != 0_i32 {
            *f.offset(0_isize) = n;
            return 1_i32;
        }
        while n >= u64::from(p).wrapping_mul(u64::from(p)) {
            p = next_prime(p);
            if p == 0 {
                break;
            }
            while n.wrapping_rem(u64::from(p)) == 0 {
                n = n.wrapping_div(u64::from(p));
                let fresh1 = i;
                i = i.wrapping_add(1);
                *f.offset(fresh1 as isize) = u64::from(p);
            }
        }
        if n > 1 {
            let fresh2 = i;
            i = i.wrapping_add(1);
            *f.offset(fresh2 as isize) = n;
        }
        i
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    let mut p: u32 = 0;
    let mut f: [u64; 63] = [0; 63];
    let mut po: u64 = 0;
    init_primes();
    p = 1;
// SAFETY: machine generated unsafe code
    unsafe {
        while p < 64 {
            po = (1u64 << p).wrapping_sub(1);
            print!("2^{} - 1 = {}", p, po);
            fflush(stdout);
            len = decompose(po, f.as_mut_ptr());
            if len > 1_i32 {
                i = 0_i32;
                while i < len {
                    if i != 0_i32 {
                        print!(" {} {}", 'x' as i32, f[i as usize])
                    } else {
                        print!(" {} {}", '=' as i32, f[i as usize])
                    };
                    i = i.wrapping_add(1);
                    i;
                }
            }
            print!("{}", '\n' as i32);
            p = p.wrapping_add(1);
            p;
        }
    }
    0_i32
}

pub fn main() {
// SAFETY: machine generated unsafe code
    unsafe {
        ::std::process::exit(main_0());
    }
}
