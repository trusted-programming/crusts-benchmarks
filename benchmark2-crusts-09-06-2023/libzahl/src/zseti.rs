use libc;
extern "C" {
    fn zsetu(_: *mut C2RustUnnamed, _: u64);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: i32,
    pub used: u64,
    pub alloced: u64,
    pub chars: *mut u32,
}
#[no_mangle]
pub extern "C" fn zseti(mut a: *mut C2RustUnnamed, mut b: i64) {
    unsafe {
        if b >= 0 {
            zsetu(a, b as u64);
        } else {
            zsetu(a, -b as u64);
            (*a).sign = -1;
        };
    }
}
