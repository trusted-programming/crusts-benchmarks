use libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn libzahl_realloc(a: *mut C2RustUnnamed, need: u64);
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
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
pub extern "C" fn zrsh(mut a: *mut C2RustUnnamed, mut b: *mut C2RustUnnamed, mut bits: u64) {
    unsafe {
        let mut i: u64 = 0;
        let mut chars: u64 = 0;
        let mut cbits: u64 = 0;
        if bits == 0 {
            if a != b {
                zset(a, b);
            }
            return;
        }
        chars = bits >> 5;
        if zzero(b) != 0 || chars >= (*b).used || zbits(b) <= bits {
            (*a).sign = 0;
            return;
        }
        bits = bits & (32 - 1i32) as u64;
        cbits = 32u64.wrapping_sub(bits);
        if chars != 0 && a == b {
            let ref mut fresh0 = (*a).used;
            *fresh0 = (*fresh0 as u64).wrapping_sub(chars) as u64;
            memmove(
                (*a).chars as *mut libc::c_void,
                ((*a).chars).offset(chars as isize) as *const libc::c_void,
                ((*a).used).wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
        } else if a != b {
            (*a).used = ((*b).used).wrapping_sub(chars);
            if (*a).alloced < (*a).used {
                libzahl_realloc(a, (*a).used);
            }
            memcpy(
                (*a).chars as *mut libc::c_void,
                ((*b).chars).offset(chars as isize) as *const libc::c_void,
                ((*a).used).wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
        }
        if bits != 0 {
            *((*a).chars).offset(0 as isize) >>= bits;
            i = 1;
            while i < (*a).used {
                let ref mut fresh1 = *((*a).chars).offset(i.wrapping_sub(1) as isize);
                *fresh1 |= *((*a).chars).offset(i as isize) << cbits;
                *((*a).chars).offset(i as isize) >>= bits;
                i = i.wrapping_add(1);
            }
            while *((*a).chars).offset(((*a).used).wrapping_sub(1) as isize) == 0 {
                let ref mut fresh2 = (*a).used;
                *fresh2 = (*fresh2).wrapping_sub(1);
            }
        };
        (*a).sign = zsignum(b);
    }
}
