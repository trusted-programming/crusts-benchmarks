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
pub extern "C" fn zlsb(mut a: *mut C2RustUnnamed) -> u64 {
    unsafe {
        let mut i = 0;
        let mut x: u32 = 0;
        if zzero(a) != 0 {
            return 18446744073709551615;
        }
        loop {
            x = *((*a).chars).offset(i as isize);
            if x != 0 {
                x = !x;
                i = (i as u64).wrapping_mul(32) as u64;
                while x & 1 != 0 {
                    x >>= 1;
                    i = i.wrapping_add(1);
                }
                return i;
            }
            i = i.wrapping_add(1);
        }
    }
}
