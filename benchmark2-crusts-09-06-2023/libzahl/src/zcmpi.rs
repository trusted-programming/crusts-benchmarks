use libc;
extern "C" {
    fn zseti(_: *mut C2RustUnnamed, _: i64);
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
extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> i32 {
    unsafe {
        return ((*a).sign == 0) as i32;
    }
}

#[inline]
extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> i32 {
    unsafe {
        return (*a).sign;
    }
}

#[no_mangle]
pub extern "C" fn zcmpi(mut a: *mut C2RustUnnamed, mut b: i64) -> i32 {
    unsafe {
        if b == 0 {
            return zsignum(a);
        }
        if zzero(a) != 0 {
            return if b > 0 { -1 } else { (b < 0) as i32 };
        }
        zseti(libzahl_tmp_cmp.as_mut_ptr(), b);
        return zcmp(a, libzahl_tmp_cmp.as_mut_ptr());
    }
}
