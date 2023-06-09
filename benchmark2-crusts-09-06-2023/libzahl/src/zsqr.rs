use libc;
extern "C" {
    fn zfree(_: *mut C2RustUnnamed);
    fn zadd(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmul(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zlsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: u64);
    fn zsplit(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: u64);
    fn libzahl_realloc(a: *mut C2RustUnnamed, need: u64);
    fn zbits(_: *mut C2RustUnnamed) -> u64;
}
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
extern "C" fn zinit(mut a: *mut C2RustUnnamed) {
    unsafe {
        (*a).alloced = 0;
        let ref mut fresh0 = (*a).chars;
        *fresh0 = 0 as *mut u32;
    }
}

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

#[no_mangle]
pub extern "C" fn zsqr(mut a: *mut C2RustUnnamed, mut b: *mut C2RustUnnamed) {
    unsafe {
        let mut m2: u64 = 0;
        let mut z0: z_t = [C2RustUnnamed {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: 0 as *mut u32,
        }; 1];
        let mut z1: z_t = [C2RustUnnamed {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: 0 as *mut u32,
        }; 1];
        let mut z2: z_t = [C2RustUnnamed {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: 0 as *mut u32,
        }; 1];
        let mut high: z_t = [C2RustUnnamed {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: 0 as *mut u32,
        }; 1];
        let mut low: z_t = [C2RustUnnamed {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: 0 as *mut u32,
        }; 1];
        let mut sign: i32 = 0;
        if zzero(b) != 0 {
            (*a).sign = 0;
            return;
        }
        m2 = zbits(b);
        if m2 <= (32 / 2i32) as u64 {
            if (*a).alloced < 1 {
                libzahl_realloc(a, 1);
            };
            (*a).used = 1;
            *((*a).chars).offset(0 as isize) =
                (*((*b).chars).offset(0 as isize)).wrapping_mul(*((*b).chars).offset(0 as isize));
            (*a).sign = 1;
            return;
        }
        sign = zsignum(b);
        (*b).sign = 1;
        m2 >>= 1;
        zinit(z0.as_mut_ptr());
        zinit(z1.as_mut_ptr());
        zinit(z2.as_mut_ptr());
        zinit(high.as_mut_ptr());
        zinit(low.as_mut_ptr());
        zsplit(high.as_mut_ptr(), low.as_mut_ptr(), b, m2);
        zsqr(z0.as_mut_ptr(), low.as_mut_ptr());
        zsqr(z2.as_mut_ptr(), high.as_mut_ptr());
        zmul(z1.as_mut_ptr(), low.as_mut_ptr(), high.as_mut_ptr());
        zlsh(z1.as_mut_ptr(), z1.as_mut_ptr(), m2.wrapping_add(1));
        m2 <<= 1;
        zlsh(z2.as_mut_ptr(), z2.as_mut_ptr(), m2);
        zadd(a, z2.as_mut_ptr(), z1.as_mut_ptr());
        zadd(a, a, z0.as_mut_ptr());
        zfree(z0.as_mut_ptr());
        zfree(z1.as_mut_ptr());
        zfree(z2.as_mut_ptr());
        zfree(high.as_mut_ptr());
        zfree(low.as_mut_ptr());
        (*b).sign = sign;
        (*a).sign = 1;
    }
}
