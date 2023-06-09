use libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn libzahl_realloc(a: *mut C2RustUnnamed, need: u64);
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
pub extern "C" fn zor(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
) {
    unsafe {
        let mut n: u64 = 0;
        let mut m: u64 = 0;
        if zzero(b) != 0 {
            if zzero(c) != 0 {
                (*a).sign = 0;
            } else if a != c {
                zset(a, c);
            }
            return;
        } else {
            if zzero(c) != 0 {
                if a != b {
                    zset(a, b);
                }
                return;
            }
        }
        m = if (*b).used > (*c).used {
            (*b).used
        } else {
            (*c).used
        };
        n = ((*b).used).wrapping_add((*c).used).wrapping_sub(m);
        if (*a).alloced < m {
            libzahl_realloc(a, m);
        }
        if a == b {
            if (*b).used < (*c).used {
                memcpy(
                    ((*a).chars).offset(n as isize) as *mut libc::c_void,
                    ((*c).chars).offset(n as isize) as *const libc::c_void,
                    m.wrapping_sub(n)
                        .wrapping_mul(::std::mem::size_of::<u32>() as u64),
                );
            }
            loop {
                let fresh0 = n;
                n = n.wrapping_sub(1);
                if !(fresh0 != 0) {
                    break;
                }
                let ref mut fresh1 = *((*a).chars).offset(n as isize);
                *fresh1 |= *((*c).chars).offset(n as isize);
            }
        } else if a == c {
            if (*c).used < (*b).used {
                memcpy(
                    ((*a).chars).offset(n as isize) as *mut libc::c_void,
                    ((*b).chars).offset(n as isize) as *const libc::c_void,
                    m.wrapping_sub(n)
                        .wrapping_mul(::std::mem::size_of::<u32>() as u64),
                );
            }
            loop {
                let fresh2 = n;
                n = n.wrapping_sub(1);
                if !(fresh2 != 0) {
                    break;
                }
                let ref mut fresh3 = *((*a).chars).offset(n as isize);
                *fresh3 |= *((*b).chars).offset(n as isize);
            }
        } else if m == (*b).used {
            memcpy(
                (*a).chars as *mut libc::c_void,
                (*b).chars as *const libc::c_void,
                m.wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
            loop {
                let fresh4 = n;
                n = n.wrapping_sub(1);
                if !(fresh4 != 0) {
                    break;
                }
                let ref mut fresh5 = *((*a).chars).offset(n as isize);
                *fresh5 |= *((*c).chars).offset(n as isize);
            }
        } else {
            memcpy(
                (*a).chars as *mut libc::c_void,
                (*c).chars as *const libc::c_void,
                m.wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
            loop {
                let fresh6 = n;
                n = n.wrapping_sub(1);
                if !(fresh6 != 0) {
                    break;
                }
                let ref mut fresh7 = *((*a).chars).offset(n as isize);
                *fresh7 |= *((*b).chars).offset(n as isize);
            }
        };
        (*a).used = m;
        (*a).sign = (zsignum(b) > 0 && zsignum(c) > 0) as i32 * 2 - 1;
    }
}
