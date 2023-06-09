use libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmul(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmod(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    static mut libzahl_tmp_modmul: z_t;
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
#[no_mangle]
pub extern "C" fn zmodmul(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
    mut d: *mut C2RustUnnamed,
) {
    unsafe {
        if a == d {
            zset(libzahl_tmp_modmul.as_mut_ptr(), d);
            zmul(a, b, c);
            zmod(a, a, libzahl_tmp_modmul.as_mut_ptr());
        } else {
            zmul(a, b, c);
            zmod(a, a, d);
        };
    }
}
