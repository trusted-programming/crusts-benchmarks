use libc;
extern "C" {
    fn libzahl_realloc(a: *mut C2RustUnnamed, need: u64);
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
pub extern "C" fn zsetu(mut a: *mut C2RustUnnamed, mut b: u64) {
    unsafe {
        if b == 0 {
            (*a).sign = 0;
            return;
        }
        if (*a).alloced
            < (::std::mem::size_of::<u64>() as u64)
                .wrapping_add(::std::mem::size_of::<u32>() as u64)
                .wrapping_sub(1)
                .wrapping_div(::std::mem::size_of::<u32>() as u64)
        {
            libzahl_realloc(
                a,
                (::std::mem::size_of::<u64>() as u64)
                    .wrapping_add(::std::mem::size_of::<u32>() as u64)
                    .wrapping_sub(1)
                    .wrapping_div(::std::mem::size_of::<u32>() as u64),
            );
        };
        (*a).sign = 1;
        (*a).used = 0;
        while b != 0 {
            let ref mut fresh0 = (*a).used;
            let fresh1 = *fresh0;
            *fresh0 = (*fresh0).wrapping_add(1);
            *((*a).chars).offset(fresh1 as isize) = b as u32;
            b >>= 32;
        }
    }
}
