use libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
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
pub extern "C" fn znot(mut a: *mut C2RustUnnamed, mut b: *mut C2RustUnnamed) {
    unsafe {
        let mut bits: u64 = 0;
        let mut n: u64 = 0;
        if zzero(b) != 0 {
            (*a).sign = 0;
            return;
        }
        bits = zbits(b);
        if a != b {
            zset(a, b);
        };
        (*a).sign = -zsignum(a);
        n = (*a).used;
        loop {
            let fresh0 = n;
            n = n.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            };
            *((*a).chars).offset(n as isize) = !*((*a).chars).offset(n as isize);
        }
        bits = bits & (32 - 1i32) as u64;
        if bits != 0 {
            let ref mut fresh1 = *((*a).chars).offset(((*a).used).wrapping_sub(1) as isize);
            *fresh1 &= (1u32 << bits).wrapping_sub(1);
        }
        while (*a).used != 0 && *((*a).chars).offset(((*a).used).wrapping_sub(1) as isize) == 0 {
            let ref mut fresh2 = (*a).used;
            *fresh2 = (*fresh2).wrapping_sub(1);
        }
        if (*a).used == 0 {
            (*a).sign = 0;
        }
    }
}
