use libc;
extern "C" {
    fn zsetu(_: *mut C2RustUnnamed, _: u64);
    fn zcmp(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> i32;
    static mut libzahl_tmp_cmp: z_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: i32,
    pub used: u64,
    pub alloced: u64,
    pub chars: *mut u32,
}
pub type z_t = [C2RustUnnamed; 1];
#[inline]
extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> i32 {
    unsafe {
        return (*a).sign;
    }
}

#[no_mangle]
pub extern "C" fn zcmpu(mut a: *mut C2RustUnnamed, mut b: u64) -> i32 {
    unsafe {
        if b == 0 {
            return zsignum(a);
        }
        if zsignum(a) <= 0 {
            return -1;
        }
        zsetu(libzahl_tmp_cmp.as_mut_ptr(), b);
        return zcmp(a, libzahl_tmp_cmp.as_mut_ptr());
    }
}
