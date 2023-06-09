use libc;
extern "C" {
    fn zfree(_: *mut C2RustUnnamed);
    fn zadd(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsub(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
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

#[no_mangle]
pub extern "C" fn zmul(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
) {
    unsafe {
        let mut m: u64 = 0;
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
        let mut b_high: z_t = [C2RustUnnamed {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: 0 as *mut u32,
        }; 1];
        let mut b_low: z_t = [C2RustUnnamed {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: 0 as *mut u32,
        }; 1];
        let mut c_high: z_t = [C2RustUnnamed {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: 0 as *mut u32,
        }; 1];
        let mut c_low: z_t = [C2RustUnnamed {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: 0 as *mut u32,
        }; 1];
        let mut b_sign: i32 = 0;
        let mut c_sign: i32 = 0;
        b_sign = zsignum(b);
        c_sign = zsignum(c);
        if b_sign == 0 || c_sign == 0 {
            (*a).sign = 0;
            return;
        }
        m = zbits(b);
        m2 = if b == c { m } else { zbits(c) };
        if m.wrapping_add(m2) <= 32 {
            if (*a).alloced < 1 {
                libzahl_realloc(a, 1);
            };
            (*a).used = 1;
            *((*a).chars).offset(0 as isize) =
                (*((*b).chars).offset(0 as isize)).wrapping_mul(*((*c).chars).offset(0 as isize));
            (*a).sign = b_sign * c_sign;
            return;
        };
        (*b).sign = 1;
        (*c).sign = 1;
        m = if m > m2 { m } else { m2 };
        m2 = m >> 1;
        zinit(z0.as_mut_ptr());
        zinit(z1.as_mut_ptr());
        zinit(z2.as_mut_ptr());
        zinit(b_high.as_mut_ptr());
        zinit(b_low.as_mut_ptr());
        zinit(c_high.as_mut_ptr());
        zinit(c_low.as_mut_ptr());
        zsplit(b_high.as_mut_ptr(), b_low.as_mut_ptr(), b, m2);
        zsplit(c_high.as_mut_ptr(), c_low.as_mut_ptr(), c, m2);
        zmul(z0.as_mut_ptr(), b_low.as_mut_ptr(), c_low.as_mut_ptr());
        zmul(z2.as_mut_ptr(), b_high.as_mut_ptr(), c_high.as_mut_ptr());
        zadd(b_low.as_mut_ptr(), b_low.as_mut_ptr(), b_high.as_mut_ptr());
        zadd(c_low.as_mut_ptr(), c_low.as_mut_ptr(), c_high.as_mut_ptr());
        zmul(z1.as_mut_ptr(), b_low.as_mut_ptr(), c_low.as_mut_ptr());
        zsub(z1.as_mut_ptr(), z1.as_mut_ptr(), z0.as_mut_ptr());
        zsub(z1.as_mut_ptr(), z1.as_mut_ptr(), z2.as_mut_ptr());
        zlsh(z1.as_mut_ptr(), z1.as_mut_ptr(), m2);
        m2 <<= 1;
        zlsh(z2.as_mut_ptr(), z2.as_mut_ptr(), m2);
        zadd(a, z2.as_mut_ptr(), z1.as_mut_ptr());
        zadd(a, a, z0.as_mut_ptr());
        zfree(z0.as_mut_ptr());
        zfree(z1.as_mut_ptr());
        zfree(z2.as_mut_ptr());
        zfree(b_high.as_mut_ptr());
        zfree(b_low.as_mut_ptr());
        zfree(c_high.as_mut_ptr());
        zfree(c_low.as_mut_ptr());
        (*b).sign = b_sign;
        (*c).sign = c_sign;
        (*a).sign = b_sign * c_sign;
    }
}
