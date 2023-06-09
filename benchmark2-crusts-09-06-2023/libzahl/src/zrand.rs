use libc;
extern "C" {
    fn abort() -> !;
    fn zcmpmag(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> i32;
    fn zadd(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmul(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zrsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: u64);
    fn zbits(_: *mut C2RustUnnamed) -> u64;
    static mut libzahl_jmp_buf: jmp_buf;
    fn __errno_location() -> *mut i32;
    static mut libzahl_error: i32;
    fn libzahl_realloc(a: *mut C2RustUnnamed, need: u64);
    static mut libzahl_const_1: z_t;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: u64) -> i64;
}
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: i32,
    pub used: u64,
    pub alloced: u64,
    pub chars: *mut u32,
}
pub type z_t = [C2RustUnnamed; 1];
pub const SECURE_RANDOM: u32 = 1;
pub const FAST_RANDOM: u32 = 0;
pub const UNIFORM: u32 = 1;
pub const QUASIUNIFORM: u32 = 0;
#[inline]
extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> i32 {
    unsafe {
        return (*a).sign;
    }
}

#[inline]
extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> i32 {
    unsafe {
        return ((*a).sign == 0) as i32;
    }
}

extern "C" fn zrand_get_random_bits(mut r: *mut C2RustUnnamed, mut bits: u64, mut fd: i32) {
    unsafe {
        let mut read_total = 0;
        let mut n: u64 = 0;
        let mut chars = bits.wrapping_add((32 - 1i32) as u64) >> 5;
        let mut read_just: i64 = 0;
        let mut mask = 1;
        let mut buf = 0 as *mut i8;
        if (*r).alloced < chars {
            libzahl_realloc(r, chars);
        }
        buf = (*r).chars as *mut i8;
        n = chars.wrapping_mul(::std::mem::size_of::<u32>() as u64);
        while n != 0 {
            read_just = read(fd, buf.offset(read_total as isize) as *mut libc::c_void, n);
            if read_just < 0 {
                libzahl_error = *__errno_location();
                longjmp(libzahl_jmp_buf.as_mut_ptr(), 1);
            }
            read_total = (read_total as u64).wrapping_add(read_just as u64) as u64;
            n = (n).wrapping_sub(read_just as u64) as u64;
        }
        bits = bits & (32 - 1i32) as u64;
        mask <<= bits;
        mask = (mask as u32).wrapping_sub(1) as u32;
        let ref mut fresh0 = *((*r).chars).offset(chars.wrapping_sub(1) as isize);
        *fresh0 &= mask;
        n = chars;
        loop {
            let fresh1 = n;
            n = n.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            if *((*r).chars).offset(n as isize) != 0 {
                (*r).used = n.wrapping_add(1);
                (*r).sign = 1;
                return;
            }
        }
        (*r).sign = 0;
    }
}

#[no_mangle]
pub extern "C" fn zrand(
    mut r: *mut C2RustUnnamed,
    mut dev: u32,
    mut dist: u32,
    mut n: *mut C2RustUnnamed,
) {
    unsafe {
        let mut pathname = 0 as *const i8;
        let mut bits: u64 = 0;
        let mut fd: i32 = 0;
        match dev as u32 {
            0 => {
                pathname = b"/dev/urandom\0" as *const u8 as *const i8;
            }
            1 => {
                pathname = b"/dev/random\0" as *const u8 as *const i8;
            }
            _ => {
                abort();
            }
        }
        if zzero(n) != 0 {
            (*r).sign = 0;
            return;
        }
        fd = open(pathname, 0);
        if fd < 0 {
            libzahl_error = *__errno_location();
            longjmp(libzahl_jmp_buf.as_mut_ptr(), 1);
        }
        match dist as u32 {
            0 => {
                if zsignum(n) < 0 {
                    libzahl_error = 33;
                    longjmp(libzahl_jmp_buf.as_mut_ptr(), 1);
                }
                bits = zbits(n);
                zrand_get_random_bits(r, bits, fd);
                zadd(r, r, libzahl_const_1.as_mut_ptr());
                zmul(r, r, n);
                zrsh(r, r, bits);
            }
            1 => {
                if zsignum(n) < 0 {
                    libzahl_error = 33;
                    longjmp(libzahl_jmp_buf.as_mut_ptr(), 1);
                }
                bits = zbits(n);
                loop {
                    zrand_get_random_bits(r, bits, fd);
                    if !(zcmpmag(r, n) > 0) {
                        break;
                    }
                }
            }
            _ => {
                abort();
            }
        }
        close(fd);
    }
}
