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
pub extern "C" fn zset(mut a: *mut C2RustUnnamed, mut b: *mut C2RustUnnamed) {
    unsafe {
        if zzero(b) != 0 {
            (*a).sign = 0;
        } else {
            if (*a).alloced < (*b).used {
                libzahl_realloc(a, (*b).used);
            };
            (*a).sign = (*b).sign;
            (*a).used = (*b).used;
            memcpy(
                (*a).chars as *mut libc::c_void,
                (*b).chars as *const libc::c_void,
                ((*b).used).wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
        };
    }
}
