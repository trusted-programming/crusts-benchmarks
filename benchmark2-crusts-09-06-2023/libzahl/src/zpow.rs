use libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsetu(_: *mut C2RustUnnamed, _: u64);
    fn zmul(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsqr(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    static mut libzahl_tmp_pow_b: z_t;
    static mut libzahl_tmp_pow_c: z_t;
    fn zbits(_: *mut C2RustUnnamed) -> u64;
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: i32;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
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
#[inline]
extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> i32 {
    unsafe {
        return ((*a).sign == 0) as i32;
    }
}

#[inline]
extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> i32 {
    unsafe {
        return (*a).sign;
    }
}

#[no_mangle]
pub extern "C" fn zpow(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
) {
    unsafe {
        let mut i: u64 = 0;
        let mut j: u64 = 0;
        let mut n: u64 = 0;
        let mut bits: u64 = 0;
        let mut x: u32 = 0;
        if zsignum(c) <= 0 {
            if zzero(c) != 0 {
                if zzero(b) != 0 {
                    libzahl_error = 33;
                    longjmp(libzahl_jmp_buf.as_mut_ptr(), 1);
                }
                zsetu(a, 1);
            } else if zzero(b) != 0 {
                libzahl_error = 33;
                longjmp(libzahl_jmp_buf.as_mut_ptr(), 1);
            } else {
                (*a).sign = 0;
            }
            return;
        } else {
            if zzero(b) != 0 {
                (*a).sign = 0;
                return;
            }
        }
        bits = zbits(c);
        n = bits >> 5;
        zset(libzahl_tmp_pow_b.as_mut_ptr(), b);
        zset(libzahl_tmp_pow_c.as_mut_ptr(), c);
        zsetu(a, 1);
        i = 0;
        while i < n {
            x = *((*libzahl_tmp_pow_c.as_mut_ptr()).chars).offset(i as isize);
            j = 32;
            loop {
                let fresh0 = j;
                j = j.wrapping_sub(1);
                if !(fresh0 != 0) {
                    break;
                }
                if x & 1 != 0 {
                    zmul(a, a, libzahl_tmp_pow_b.as_mut_ptr());
                }
                zsqr(
                    libzahl_tmp_pow_b.as_mut_ptr(),
                    libzahl_tmp_pow_b.as_mut_ptr(),
                );
                x >>= 1;
            }
            i = i.wrapping_add(1);
        }
        x = *((*libzahl_tmp_pow_c.as_mut_ptr()).chars).offset(i as isize);
        while x != 0 {
            if x & 1 != 0 {
                zmul(a, a, libzahl_tmp_pow_b.as_mut_ptr());
            }
            zsqr(
                libzahl_tmp_pow_b.as_mut_ptr(),
                libzahl_tmp_pow_b.as_mut_ptr(),
            );
            x >>= 1;
        }
    }
}
