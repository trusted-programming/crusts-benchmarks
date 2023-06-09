use libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn libzahl_realloc(a: *mut C2RustUnnamed, need: u64);
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
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
pub extern "C" fn zlsh(mut a: *mut C2RustUnnamed, mut b: *mut C2RustUnnamed, mut bits: u64) {
    unsafe {
        let mut i: u64 = 0;
        let mut chars: u64 = 0;
        let mut cbits: u64 = 0;
        let mut carry: [u32; 2] = [0; 2];
        if zzero(b) != 0 {
            (*a).sign = 0;
            return;
        }
        if bits == 0 {
            if a != b {
                zset(a, b);
            }
            return;
        }
        chars = bits >> 5;
        bits = bits & (32 - 1i32) as u64;
        cbits = 32u64.wrapping_sub(bits);
        if (*a).alloced < ((*b).used).wrapping_add(chars) {
            libzahl_realloc(a, ((*b).used).wrapping_add(chars));
        }
        if a == b {
            memmove(
                ((*a).chars).offset(chars as isize) as *mut libc::c_void,
                (*b).chars as *const libc::c_void,
                ((*b).used).wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
        } else {
            memcpy(
                ((*a).chars).offset(chars as isize) as *mut libc::c_void,
                (*b).chars as *const libc::c_void,
                ((*b).used).wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
        }
        memset(
            (*a).chars as *mut libc::c_void,
            0,
            chars.wrapping_mul(::std::mem::size_of::<u32>() as u64),
        );
        (*a).used = ((*b).used).wrapping_add(chars);
        if bits != 0 {
            i = chars;
            while i < (*a).used {
                carry[(!i & 1u64) as usize] = *((*a).chars).offset(i as isize) >> cbits;
                *((*a).chars).offset(i as isize) <<= bits;
                let ref mut fresh0 = *((*a).chars).offset(i as isize);
                *fresh0 |= carry[(i & 1u64) as usize];
                i = i.wrapping_add(1);
            }
            if carry[(i & 1u64) as usize] != 0 {
                if (*a).alloced < ((*a).used).wrapping_add(1) {
                    libzahl_realloc(a, ((*a).used).wrapping_add(1));
                };
                *((*a).chars).offset(i as isize) = carry[(i & 1u64) as usize];
                let ref mut fresh1 = (*a).used;
                *fresh1 = (*fresh1).wrapping_add(1);
            }
        };
        (*a).sign = zsignum(b);
    }
}
