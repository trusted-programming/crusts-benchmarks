use libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
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
pub extern "C" fn zbset(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut bit: u64,
    mut action: i32,
) {
    unsafe {
        let mut mask = 1;
        let mut chars: u64 = 0;
        chars = bit >> 5;
        bit = bit & (32 - 1i32) as u64;
        mask <<= bit;
        if a != b {
            zset(a, b);
        }
        if action != 0 {
            if zzero(a) != 0 {
                (*a).used = 0;
                (*a).sign = 1;
            }
            if (*a).used <= chars {
                if (*a).alloced < chars.wrapping_add(1) {
                    libzahl_realloc(a, chars.wrapping_add(1));
                }
                memset(
                    ((*a).chars).offset((*a).used as isize) as *mut libc::c_void,
                    0,
                    chars
                        .wrapping_add(1)
                        .wrapping_sub((*a).used)
                        .wrapping_mul(::std::mem::size_of::<u32>() as u64),
                );
                (*a).used = chars.wrapping_add(1);
            }
        }
        if action > 0 {
            let ref mut fresh0 = *((*a).chars).offset(chars as isize);
            *fresh0 |= mask;
            return;
        } else {
            if action < 0 {
                let ref mut fresh1 = *((*a).chars).offset(chars as isize);
                *fresh1 ^= mask;
            } else if chars < (*a).used {
                let ref mut fresh2 = *((*a).chars).offset(chars as isize);
                *fresh2 &= !mask;
            }
        }
        while (*a).used != 0 && *((*a).chars).offset(((*a).used).wrapping_sub(1) as isize) == 0 {
            let ref mut fresh3 = (*a).used;
            *fresh3 = (*fresh3).wrapping_sub(1);
        }
        if (*a).used == 0 {
            (*a).sign = 0;
        }
    }
}
