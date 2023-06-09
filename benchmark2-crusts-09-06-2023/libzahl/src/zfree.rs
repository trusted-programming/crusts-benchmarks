use libc;
extern "C" {
    static mut libzahl_pool: [*mut *mut u32; 64];
    static mut libzahl_pool_alloc: [u64; 64];
    static mut libzahl_pool_n: [u64; 64];
    fn free(__ptr: *mut libc::c_void);
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
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
pub extern "C" fn zfree(mut a: *mut C2RustUnnamed) {
    unsafe {
        let mut i = 0;
        let mut x: u64 = 0;
        let mut j: u64 = 0;
        let mut new = 0 as *mut *mut u32;
        if ((*a).chars).is_null() {
            return;
        }
        x = (*a).alloced;
        while x != 0 {
            i = (i as u64).wrapping_add(1) as u64;
            x >>= 1;
        }
        let fresh0 = libzahl_pool_n[i as usize];
        libzahl_pool_n[i as usize] = (libzahl_pool_n[i as usize]).wrapping_add(1);
        j = fresh0;
        if j == libzahl_pool_alloc[i as usize] {
            x = if j != 0 { j.wrapping_mul(3) >> 1 } else { 128 };
            new = realloc(
                libzahl_pool[i as usize] as *mut libc::c_void,
                x.wrapping_mul(::std::mem::size_of::<*mut u32>() as u64),
            ) as *mut *mut u32;
            if new.is_null() {
                free((*a).chars as *mut libc::c_void);
                free(libzahl_pool[i as usize] as *mut libc::c_void);
                libzahl_pool_n[i as usize] = 0;
                libzahl_pool[i as usize] = 0 as *mut *mut u32;
                libzahl_pool_alloc[i as usize] = 0;
                return;
            }
            libzahl_pool[i as usize] = new;
            libzahl_pool_alloc[i as usize] = x;
        }
        let ref mut fresh1 = *(libzahl_pool[i as usize]).offset(j as isize);
        *fresh1 = (*a).chars;
    }
}
