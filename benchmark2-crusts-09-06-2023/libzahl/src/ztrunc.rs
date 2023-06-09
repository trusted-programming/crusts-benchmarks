use libc;
extern "C" {
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
extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> i32 {
    unsafe {
        return ((*a).sign == 0) as i32;
    }
}

#[no_mangle]
pub extern "C" fn ztrunc(mut a: *mut C2RustUnnamed, mut b: *mut C2RustUnnamed, mut bits: u64) {
    unsafe {
        let mut mask = 1;
        let mut chars: u64 = 0;
        let mut i: u64 = 0;
        if zzero(b) != 0 {
            (*a).sign = 0;
            return;
        }
        chars = bits.wrapping_add((32 - 1i32) as u64) >> 5;
        (*a).sign = (*b).sign;
        (*a).used = if chars < (*b).used { chars } else { (*b).used };
        if (*a).used < chars {
            bits = 0;
        }
        if a != b {
            if (*a).alloced < (*a).used {
                libzahl_realloc(a, (*a).used);
            }
            memcpy(
                (*a).chars as *mut libc::c_void,
                (*b).chars as *const libc::c_void,
                ((*a).used).wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
        }
        bits = bits & (32 - 1i32) as u64;
        if bits != 0 {
            mask <<= bits;
            mask = (mask as u32).wrapping_sub(1) as u32;
            let ref mut fresh0 = *((*a).chars).offset(((*a).used).wrapping_sub(1) as isize);
            *fresh0 &= mask;
        }
        i = (*a).used;
        loop {
            let fresh1 = i;
            i = i.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            if *((*a).chars).offset(i as isize) != 0 {
                return;
            }
        }
        (*a).sign = 0;
    }
}
