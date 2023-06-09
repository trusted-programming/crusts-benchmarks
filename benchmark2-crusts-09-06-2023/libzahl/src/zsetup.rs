use libc;
extern "C" {
    fn zsetu(_: *mut C2RustUnnamed, _: u64);
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
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
pub type z_t = [C2RustUnnamed; 1];
#[inline]
extern "C" fn zinit(mut a: *mut C2RustUnnamed) {
    unsafe {
        (*a).alloced = 0;
        let ref mut fresh0 = (*a).chars;
        *fresh0 = 0 as *mut u32;
    }
}

#[no_mangle]
pub static mut libzahl_tmp_modmul: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_d: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_div: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_num: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_n4: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_b: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_cmp: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_mag: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_pow_c: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_div: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_str_rem: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_gcd_u: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_gcd_v: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_sub: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_mod: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_d: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_a: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_x: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_d: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_ptest_n1: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_modsqr: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_a: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_b: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_const_1e9: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_const_1e19: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_const_1: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_const_2: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_const_4: z_t = [C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1];
#[no_mangle]
pub static mut libzahl_tmp_divmod_ds: [z_t; 32] = [[C2RustUnnamed {
    sign: 0,
    used: 0,
    alloced: 0,
    chars: 0 as *const u32 as *mut u32,
}; 1]; 32];
#[no_mangle]
pub static mut libzahl_jmp_buf: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut libzahl_set_up: i32 = 0;
#[no_mangle]
pub static mut libzahl_error: i32 = 0;
#[no_mangle]
pub static mut libzahl_pool: [*mut *mut u32; 64] = [0 as *const *mut u32 as *mut *mut u32; 64];
#[no_mangle]
pub static mut libzahl_pool_n: [u64; 64] = [0; 64];
#[no_mangle]
pub static mut libzahl_pool_alloc: [u64; 64] = [0; 64];
#[no_mangle]
pub extern "C" fn zsetup(mut env: *mut __jmp_buf_tag) {
    unsafe {
        let mut i: u64 = 0;
        *libzahl_jmp_buf.as_mut_ptr() = *env;
        if libzahl_set_up == 0 {
            libzahl_set_up = 1;
            memset(
                libzahl_pool.as_mut_ptr() as *mut libc::c_void,
                0,
                ::std::mem::size_of::<[*mut *mut u32; 64]>() as u64,
            );
            memset(
                libzahl_pool_n.as_mut_ptr() as *mut libc::c_void,
                0,
                ::std::mem::size_of::<[u64; 64]>() as u64,
            );
            memset(
                libzahl_pool_alloc.as_mut_ptr() as *mut libc::c_void,
                0,
                ::std::mem::size_of::<[u64; 64]>() as u64,
            );
            zinit(libzahl_tmp_cmp.as_mut_ptr());
            zinit(libzahl_tmp_str_num.as_mut_ptr());
            zinit(libzahl_tmp_str_mag.as_mut_ptr());
            zinit(libzahl_tmp_str_div.as_mut_ptr());
            zinit(libzahl_tmp_str_rem.as_mut_ptr());
            zinit(libzahl_tmp_gcd_u.as_mut_ptr());
            zinit(libzahl_tmp_gcd_v.as_mut_ptr());
            zinit(libzahl_tmp_sub.as_mut_ptr());
            zinit(libzahl_tmp_modmul.as_mut_ptr());
            zinit(libzahl_tmp_div.as_mut_ptr());
            zinit(libzahl_tmp_mod.as_mut_ptr());
            zinit(libzahl_tmp_pow_b.as_mut_ptr());
            zinit(libzahl_tmp_pow_c.as_mut_ptr());
            zinit(libzahl_tmp_pow_d.as_mut_ptr());
            zinit(libzahl_tmp_modsqr.as_mut_ptr());
            zinit(libzahl_tmp_divmod_a.as_mut_ptr());
            zinit(libzahl_tmp_divmod_b.as_mut_ptr());
            zinit(libzahl_tmp_divmod_d.as_mut_ptr());
            zinit(libzahl_tmp_ptest_x.as_mut_ptr());
            zinit(libzahl_tmp_ptest_a.as_mut_ptr());
            zinit(libzahl_tmp_ptest_d.as_mut_ptr());
            zinit(libzahl_tmp_ptest_n1.as_mut_ptr());
            zinit(libzahl_tmp_ptest_n4.as_mut_ptr());
            zinit(libzahl_const_1e19.as_mut_ptr());
            zsetu(libzahl_const_1e19.as_mut_ptr(), 10000000000000000000);
            zinit(libzahl_const_1e9.as_mut_ptr());
            zsetu(libzahl_const_1e9.as_mut_ptr(), 1000000000);
            zinit(libzahl_const_1.as_mut_ptr());
            zsetu(libzahl_const_1.as_mut_ptr(), 1);
            zinit(libzahl_const_2.as_mut_ptr());
            zsetu(libzahl_const_2.as_mut_ptr(), 2);
            zinit(libzahl_const_4.as_mut_ptr());
            zsetu(libzahl_const_4.as_mut_ptr(), 4);
            i = 32;
            loop {
                let fresh1 = i;
                i = i.wrapping_sub(1);
                if !(fresh1 != 0) {
                    break;
                }
                zinit((libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr());
            }
        }
    }
}
