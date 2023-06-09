use libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn libzahl_realloc(a: *mut C2RustUnnamed, need: u64);
    fn zabs(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsub_unsigned(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zlsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: u64);
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
pub extern "C" fn zadd_unsigned(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
) {
    unsafe {
        let mut i: u64 = 0;
        let mut size: u64 = 0;
        let mut n: u64 = 0;
        let mut carry: [u32; 2] = [0; 2];
        let mut addend = 0 as *mut u32;
        if zzero(b) != 0 {
            zabs(a, c);
            return;
        } else {
            if zzero(c) != 0 {
                zabs(a, b);
                return;
            }
        }
        size = if (*b).used > (*c).used {
            (*b).used
        } else {
            (*c).used
        };
        n = ((*b).used).wrapping_add((*c).used).wrapping_sub(size);
        if (*a).alloced < size.wrapping_add(1) {
            libzahl_realloc(a, size.wrapping_add(1));
        };
        *((*a).chars).offset(size as isize) = 0;
        if a == b {
            if (*a).used < (*c).used {
                n = (*c).used;
                memset(
                    ((*a).chars).offset((*a).used as isize) as *mut libc::c_void,
                    0,
                    n.wrapping_sub((*a).used)
                        .wrapping_mul(::std::mem::size_of::<u32>() as u64),
                );
            }
            addend = (*c).chars;
        } else if a == c {
            if (*a).used < (*b).used {
                n = (*b).used;
                memset(
                    ((*a).chars).offset((*a).used as isize) as *mut libc::c_void,
                    0,
                    n.wrapping_sub((*a).used)
                        .wrapping_mul(::std::mem::size_of::<u32>() as u64),
                );
            }
            addend = (*b).chars;
        } else if (*b).used > (*c).used {
            memcpy(
                (*a).chars as *mut libc::c_void,
                (*b).chars as *const libc::c_void,
                ((*b).used).wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
            (*a).used = (*b).used;
            addend = (*c).chars;
        } else {
            memcpy(
                (*a).chars as *mut libc::c_void,
                (*c).chars as *const libc::c_void,
                ((*c).used).wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
            (*a).used = (*c).used;
            addend = (*b).chars;
        }
        i = 0;
        while i < n {
            if carry[(i & 1u64) as usize] != 0 {
                carry[(!i & 1u64) as usize] =
                    (4294967295u32.wrapping_sub(*((*a).chars).offset(i as isize))
                        <= *addend.offset(i as isize)) as u32;
            } else {
                carry[(!i & 1u64) as usize] =
                    (4294967295u32.wrapping_sub(*((*a).chars).offset(i as isize))
                        < *addend.offset(i as isize)) as u32;
            }
            let ref mut fresh0 = *((*a).chars).offset(i as isize);
            *fresh0 = (*fresh0 as u32)
                .wrapping_add((*addend.offset(i as isize)).wrapping_add(carry[(i & 1u64) as usize]))
                as u32;
            i = i.wrapping_add(1);
        }
        while carry[(i & 1u64) as usize] != 0 {
            carry[(!i & 1u64) as usize] = (*((*a).chars).offset(i as isize) == 4294967295) as u32;
            let fresh1 = i;
            i = i.wrapping_add(1);
            let ref mut fresh2 = *((*a).chars).offset(fresh1 as isize);
            *fresh2 = (*fresh2 as u32).wrapping_add(1) as u32;
        }
        if (*a).used < i {
            (*a).used = i;
        };
        (*a).sign = 1;
    }
}

#[no_mangle]
pub extern "C" fn zadd(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
) {
    unsafe {
        if zzero(b) != 0 {
            if a != c {
                zset(a, c);
            }
        } else if zzero(c) != 0 {
            if a != b {
                zset(a, b);
            }
        } else if b == c {
            zlsh(a, b, 1);
        } else if zsignum(b) | zsignum(c) < 0 {
            if zsignum(b) < 0 {
                if zsignum(c) < 0 {
                    zadd_unsigned(a, b, c);
                    (*a).sign = -zsignum(a);
                } else {
                    zsub_unsigned(a, c, b);
                }
            } else {
                zsub_unsigned(a, b, c);
            }
        } else {
            zadd_unsigned(a, b, c);
        };
    }
}
