use libc;
extern "C" {
    fn zcmpmag(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> i32;
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

#[no_mangle]
pub extern "C" fn zcmp(mut a: *mut C2RustUnnamed, mut b: *mut C2RustUnnamed) -> i32 {
    unsafe {
        if zsignum(a) != zsignum(b) {
            return if zsignum(a) < zsignum(b) {
                -1
            } else {
                (zsignum(a) > zsignum(b)) as i32
            };
        }
        return zsignum(a) * zcmpmag(a, b);
    }
}
