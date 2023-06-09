use libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zcmpmag(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> i32;
    static mut libzahl_tmp_sub: z_t;
    fn zabs(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zneg(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zadd_unsigned(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
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
pub extern "C" fn zsub_unsigned(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
) {
    unsafe {
        let mut carry: [u32; 2] = [0; 2];
        let mut s = 0 as *mut u32;
        let mut i: u64 = 0;
        let mut n: u64 = 0;
        let mut magcmp: i32 = 0;
        if zzero(b) != 0 {
            zabs(a, c);
            zneg(a, a);
            return;
        } else {
            if zzero(c) != 0 {
                zabs(a, b);
                return;
            }
        }
        magcmp = zcmpmag(b, c);
        if magcmp <= 0 {
            if magcmp == 0 {
                (*a).sign = 0;
                return;
            }
            n = if (*b).used < (*c).used {
                (*b).used
            } else {
                (*c).used
            };
            if a == b {
                zset(libzahl_tmp_sub.as_mut_ptr(), b);
                s = (*libzahl_tmp_sub.as_mut_ptr()).chars;
            } else {
                s = (*b).chars;
            }
            if a != c {
                zset(a, c);
            }
        } else {
            n = if (*b).used < (*c).used {
                (*b).used
            } else {
                (*c).used
            };
            if a == c {
                zset(libzahl_tmp_sub.as_mut_ptr(), c);
                s = (*libzahl_tmp_sub.as_mut_ptr()).chars;
            } else {
                s = (*c).chars;
            }
            if a != b {
                zset(a, b);
            }
        }
        i = 0;
        while i < n {
            carry[(!i & 1u64) as usize] = (if carry[(i & 1u64) as usize] != 0 {
                (*((*a).chars).offset(i as isize) <= *s.offset(i as isize)) as i32
            } else {
                (*((*a).chars).offset(i as isize) < *s.offset(i as isize)) as i32
            }) as u32;
            let ref mut fresh0 = *((*a).chars).offset(i as isize);
            *fresh0 = (*fresh0 as u32).wrapping_sub(*s.offset(i as isize)) as u32;
            let ref mut fresh1 = *((*a).chars).offset(i as isize);
            *fresh1 = (*fresh1 as u32).wrapping_sub(carry[(i & 1u64) as usize]) as u32;
            i = i.wrapping_add(1);
        }
        if carry[(i & 1u64) as usize] != 0 {
            while *((*a).chars).offset(i as isize) == 0 {
                let fresh2 = i;
                i = i.wrapping_add(1);
                *((*a).chars).offset(fresh2 as isize) = 4294967295;
            }
            let ref mut fresh3 = *((*a).chars).offset(i as isize);
            *fresh3 = (*fresh3 as u32).wrapping_sub(1) as u32;
        };
        (*a).sign = magcmp;
    }
}

#[no_mangle]
pub extern "C" fn zsub(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
) {
    unsafe {
        if b == c {
            (*a).sign = 0;
        } else if zzero(b) != 0 {
            zneg(a, c);
        } else if zzero(c) != 0 {
            if a != b {
                zset(a, b);
            }
        } else if zsignum(b) | zsignum(c) < 0 {
            if zsignum(b) < 0 {
                if zsignum(c) < 0 {
                    zsub_unsigned(a, c, b);
                } else {
                    zadd_unsigned(a, b, c);
                    (*a).sign = -zsignum(a);
                }
            } else {
                zadd_unsigned(a, b, c);
            }
        } else {
            zsub_unsigned(a, b, c);
        };
    }
}
