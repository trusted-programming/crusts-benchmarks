use libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
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
pub extern "C" fn zsave(mut a: *mut C2RustUnnamed, mut buffer: *mut libc::c_void) -> u64 {
    unsafe {
        if !buffer.is_null() {
            let mut buf = buffer as *mut i8;
            *(buf as *mut i32) = (*a).sign;
            buf = buf.offset(::std::mem::size_of::<i32>() as u64 as isize);
            *(buf as *mut u64) = (*a).used;
            buf = buf.offset(::std::mem::size_of::<u64>() as u64 as isize);
            if zzero(a) == 0 {
                memcpy(
                    buf as *mut libc::c_void,
                    (*a).chars as *const libc::c_void,
                    ((*a).used).wrapping_mul(::std::mem::size_of::<u32>() as u64),
                );
            }
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
