use libc;
extern "C" {
    fn zrsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: u64);
    fn ztrunc(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: u64);
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
pub extern "C" fn zsplit(
    mut high: *mut C2RustUnnamed,
    mut low: *mut C2RustUnnamed,
    mut a: *mut C2RustUnnamed,
    mut delim: u64,
) {
    unsafe {
        if zzero(a) != 0 {
            (*high).sign = 0;
            (*low).sign = 0;
            return;
        }
        if high == a {
            ztrunc(low, a, delim);
            zrsh(high, a, delim);
        } else {
            zrsh(high, a, delim);
            ztrunc(low, a, delim);
        };
    }
}
