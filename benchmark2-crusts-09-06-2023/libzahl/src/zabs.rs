use libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
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
pub extern "C" fn zabs(mut a: *mut C2RustUnnamed, mut b: *mut C2RustUnnamed) {
    unsafe {
        if a != b {
            zset(a, b);
        };
        (*a).sign = (zzero(a) == 0) as i32;
    }
}
