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
pub extern "C" fn zbtest(mut a: *mut C2RustUnnamed, mut bit: u64) -> i32 {
    unsafe {
        let mut chars: u64 = 0;
        if zzero(a) != 0 {
            return 0;
        }
        chars = bit >> 5;
        if chars >= (*a).used {
            return 0;
        }
        bit = bit & (32 - 1i32) as u64;
        return (*((*a).chars).offset(chars as isize) >> bit & 1u32) as i32;
    }
}
