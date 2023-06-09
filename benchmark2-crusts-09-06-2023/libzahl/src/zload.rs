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
pub extern "C" fn zload(mut a: *mut C2RustUnnamed, mut buffer: *const libc::c_void) -> u64 {
    unsafe {
        let mut buf = buffer as *const i8;
        (*a).sign = *(buf as *const i32);
        buf = buf.offset(::std::mem::size_of::<i32>() as u64 as isize);
        (*a).used = *(buf as *const u64);
        buf = buf.offset(::std::mem::size_of::<u64>() as u64 as isize);
        if (*a).sign != 0 {
            if (*a).alloced < (*a).used {
                libzahl_realloc(a, (*a).used);
            }
            memcpy(
                (*a).chars as *mut libc::c_void,
                buf as *const libc::c_void,
                ((*a).used).wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
        }
        return (::std::mem::size_of::<i32>() as u64)
            .wrapping_add(::std::mem::size_of::<u64>() as u64)
            .wrapping_add(
                (if zzero(a) != 0 {
                    0
                } else {
                    ((*a).used).wrapping_mul(::std::mem::size_of::<u32>() as u64)
                }),
            );
    }
}
