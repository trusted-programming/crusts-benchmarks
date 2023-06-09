use libc;
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
pub extern "C" fn zcmpmag(mut a: *mut C2RustUnnamed, mut b: *mut C2RustUnnamed) -> i32 {
    unsafe {
        let mut i: u64 = 0;
        let mut j: u64 = 0;
        if zzero(a) != 0 {
            return -((zzero(b) == 0) as i32);
        }
        if zzero(b) != 0 {
            return 1;
        }
        i = ((*a).used).wrapping_sub(1);
        j = ((*b).used).wrapping_sub(1);
        while i > j {
            if *((*a).chars).offset(i as isize) != 0 {
                return 1;
            }
            let ref mut fresh0 = (*a).used;
            *fresh0 = (*fresh0).wrapping_sub(1);
            i = i.wrapping_sub(1);
        }
        while j > i {
            if *((*b).chars).offset(j as isize) != 0 {
                return -1;
            }
            let ref mut fresh1 = (*b).used;
            *fresh1 = (*fresh1).wrapping_sub(1);
            j = j.wrapping_sub(1);
        }
        while i != 0 {
            if *((*a).chars).offset(i as isize) != *((*b).chars).offset(i as isize) {
                return (*((*a).chars).offset(i as isize) > *((*b).chars).offset(i as isize))
                    as i32
                    * 2
                    - 1;
            }
            i = i.wrapping_sub(1);
        }
        return if *((*a).chars).offset(0 as isize) < *((*b).chars).offset(0 as isize) {
            -1
        } else {
            (*((*a).chars).offset(0 as isize) > *((*b).chars).offset(0 as isize)) as i32
        };
    }
}
