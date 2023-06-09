use libc;
extern "C" {
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn zfree(_: *mut C2RustUnnamed);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    static mut libzahl_pool_n: [u64; 64];
    static mut libzahl_pool: [*mut *mut u32; 64];
    static mut libzahl_error: i32;
    static mut libzahl_jmp_buf: jmp_buf;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn __errno_location() -> *mut i32;
}
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: i32,
    pub used: u64,
    pub alloced: u64,
    pub chars: *mut u32,
}
#[no_mangle]
pub extern "C" fn libzahl_realloc(mut a: *mut C2RustUnnamed, mut need: u64) {
    unsafe {
        let mut i: u64 = 0;
        let mut x: u64 = 0;
        let mut new = 0 as *mut u32;
        if need & (!need).wrapping_add(1) != need {
            need |= need >> 1;
            need |= need >> 2;
            need |= need >> 4;
            i = ::std::mem::size_of::<u64>() as u64;
            x = 8;
            while i != 0 {
                need |= need >> x;
                i >>= 1;
                x <<= 1;
            }
            need = (need as u64).wrapping_add(1) as u64;
        }
        i = 0;
        x = need;
        while x != 0 {
            i = (i).wrapping_add(1) as u64;
            x >>= 1;
        }
        if libzahl_pool_n[i as usize] != 0 {
            libzahl_pool_n[i as usize] = (libzahl_pool_n[i as usize]).wrapping_sub(1);
            new = *(libzahl_pool[i as usize]).offset(libzahl_pool_n[i as usize] as isize);
            memcpy(
                new as *mut libc::c_void,
                (*a).chars as *const libc::c_void,
                ((*a).alloced).wrapping_mul(::std::mem::size_of::<u32>() as u64),
            );
            zfree(a);
            let ref mut fresh0 = (*a).chars;
            *fresh0 = new;
        } else {
            let ref mut fresh1 = (*a).chars;
            *fresh1 = realloc(
                (*a).chars as *mut libc::c_void,
                need.wrapping_mul(::std::mem::size_of::<u32>() as u64),
            ) as *mut u32;
            if ((*a).chars).is_null() {
                if *__errno_location() == 0 {
                    *__errno_location() = 12;
                }
                libzahl_error = *__errno_location();
                longjmp(libzahl_jmp_buf.as_mut_ptr(), 1);
            }
        };
        (*a).alloced = need;
    }
}
