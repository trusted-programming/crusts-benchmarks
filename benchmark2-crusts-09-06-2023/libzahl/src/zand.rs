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
pub extern "C" fn zand(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
) {
    unsafe {
        let mut current_block: u64;
        let mut n: u64 = 0;
        if zzero(b) != 0 || zzero(c) != 0 {
            (*a).sign = 0;
            return;
        }
        n = if (*b).used < (*c).used {
            (*b).used
        } else {
            (*c).used
        };
        loop {
            let fresh0 = n;
            n = n.wrapping_sub(1);
            if !(fresh0 != 0) {
                current_block = 2473556513754201174;
                break;
            }
            if *((*b).chars).offset(n as isize) & *((*c).chars).offset(n as isize) != 0 {
                current_block = 8876057388401302592;
                break;
            }
        }
        match current_block {
            2473556513754201174 => {
                (*a).sign = 0;
                return;
            }
            _ => {
                n = n.wrapping_add(1);
                (*a).used = n;
                if a == b {
                    loop {
                        let fresh1 = n;
                        n = n.wrapping_sub(1);
                        if !(fresh1 != 0) {
                            break;
                        }
                        let ref mut fresh2 = *((*a).chars).offset(n as isize);
                        *fresh2 &= *((*c).chars).offset(n as isize);
                    }
                } else if a == c {
                    loop {
                        let fresh3 = n;
                        n = n.wrapping_sub(1);
                        if !(fresh3 != 0) {
                            break;
                        }
                        let ref mut fresh4 = *((*a).chars).offset(n as isize);
                        *fresh4 &= *((*b).chars).offset(n as isize);
                    }
                } else {
                    if (*a).alloced < (*a).used {
                        libzahl_realloc(a, (*a).used);
                    }
                    memcpy(
                        (*a).chars as *mut libc::c_void,
                        (*c).chars as *const libc::c_void,
                        ((*a).used).wrapping_mul(::std::mem::size_of::<u32>() as u64),
                    );
                    loop {
                        let fresh5 = n;
                        n = n.wrapping_sub(1);
                        if !(fresh5 != 0) {
                            break;
                        }
                        let ref mut fresh6 = *((*a).chars).offset(n as isize);
                        *fresh6 &= *((*b).chars).offset(n as isize);
                    }
                };
                (*a).sign = (zsignum(b) > 0 || zsignum(c) > 0) as i32 * 2 - 1;
                return;
            }
        };
    }
}
