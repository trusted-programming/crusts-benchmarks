use libc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: i32,
    pub used: u64,
    pub alloced: u64,
    pub chars: *mut u32,
}
pub type z_t = [C2RustUnnamed; 1];
#[no_mangle]
pub extern "C" fn zswap(mut a: *mut C2RustUnnamed, mut b: *mut C2RustUnnamed) {
    unsafe {
        let mut t: z_t = [C2RustUnnamed {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: 0 as *mut u32,
        }; 1];
        *t.as_mut_ptr() = *a;
        *a = *b;
        *b = *t.as_mut_ptr();
    }
}
