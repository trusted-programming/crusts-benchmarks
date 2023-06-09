use libc;
extern "C" {
    fn zswap(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zseti(_: *mut C2RustUnnamed, _: i64);
    fn zcmpmag(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> i32;
    fn zsub(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    static mut libzahl_tmp_divmod_b: z_t;
    static mut libzahl_tmp_divmod_a: z_t;
    static mut libzahl_tmp_divmod_ds: [z_t; 32];
    fn zrsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: u64);
    fn zbset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: u64, _: i32);
    static mut libzahl_tmp_divmod_d: z_t;
    fn zabs(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zlsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: u64);
    fn zbits(_: *mut C2RustUnnamed) -> u64;
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_error: i32;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
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
pub extern "C" fn zdivmod(
    mut a: *mut C2RustUnnamed,
    mut b: *mut C2RustUnnamed,
    mut c: *mut C2RustUnnamed,
    mut d: *mut C2RustUnnamed,
) {
    unsafe {
        let mut c_bits: u64 = 0;
        let mut d_bits: u64 = 0;
        let mut bit: u64 = 0;
        let mut sign: i32 = 0;
        let mut cmpmag: i32 = 0;
        sign = zsignum(c) * zsignum(d);
        if sign == 0 {
            if zzero(c) != 0 {
                if zzero(d) != 0 {
                    libzahl_error = 33;
                    longjmp(libzahl_jmp_buf.as_mut_ptr(), 1);
                } else {
                    (*a).sign = 0;
                    (*b).sign = 0;
                }
            } else {
                libzahl_error = 33;
                longjmp(libzahl_jmp_buf.as_mut_ptr(), 1);
            }
            return;
        } else {
            cmpmag = zcmpmag(c, d);
            if cmpmag <= 0 {
                if cmpmag == 0 {
                    zseti(a, sign as i64);
                    (*b).sign = 0;
                    return;
                } else {
                    if b != c {
                        zset(b, c);
                    }
                };
                (*b).sign = 1;
                (*a).sign = 0;
                return;
            }
        }
        c_bits = zbits(c);
        d_bits = zbits(d);
        bit = c_bits.wrapping_sub(d_bits);
        zlsh(libzahl_tmp_divmod_d.as_mut_ptr(), d, bit);
        (*libzahl_tmp_divmod_d.as_mut_ptr()).sign = 1;
        if zcmpmag(libzahl_tmp_divmod_d.as_mut_ptr(), c) > 0 {
            zrsh(
                libzahl_tmp_divmod_d.as_mut_ptr(),
                libzahl_tmp_divmod_d.as_mut_ptr(),
                1,
            );
            bit = (bit).wrapping_sub(1) as u64;
        };
        (*libzahl_tmp_divmod_a.as_mut_ptr()).sign = 0;
        zabs(libzahl_tmp_divmod_b.as_mut_ptr(), c);
        if bit < 32 {
            loop {
                if zcmpmag(
                    libzahl_tmp_divmod_d.as_mut_ptr(),
                    libzahl_tmp_divmod_b.as_mut_ptr(),
                ) <= 0
                {
                    zsub(
                        libzahl_tmp_divmod_b.as_mut_ptr(),
                        libzahl_tmp_divmod_b.as_mut_ptr(),
                        libzahl_tmp_divmod_d.as_mut_ptr(),
                    );
                    zbset(
                        libzahl_tmp_divmod_a.as_mut_ptr(),
                        libzahl_tmp_divmod_a.as_mut_ptr(),
                        bit,
                        1,
                    );
                }
                let fresh0 = bit;
                bit = bit.wrapping_sub(1);
                if fresh0 == 0 || zzero(libzahl_tmp_divmod_b.as_mut_ptr()) != 0 {
                    break;
                }
                zrsh(
                    libzahl_tmp_divmod_d.as_mut_ptr(),
                    libzahl_tmp_divmod_d.as_mut_ptr(),
                    1,
                );
            }
        } else {
            let mut i: u64 = 0;
            i = 0;
            while i < 32 {
                zrsh(
                    (libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr(),
                    libzahl_tmp_divmod_d.as_mut_ptr(),
                    i,
                );
                i = i.wrapping_add(1);
            }
            's_253: loop {
                i = 0;
                while i < 32 {
                    if zcmpmag(
                        (libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr(),
                        libzahl_tmp_divmod_b.as_mut_ptr(),
                    ) <= 0
                    {
                        zsub(
                            libzahl_tmp_divmod_b.as_mut_ptr(),
                            libzahl_tmp_divmod_b.as_mut_ptr(),
                            (libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr(),
                        );
                        zbset(
                            libzahl_tmp_divmod_a.as_mut_ptr(),
                            libzahl_tmp_divmod_a.as_mut_ptr(),
                            bit,
                            1,
                        );
                    }
                    let fresh1 = bit;
                    bit = bit.wrapping_sub(1);
                    if fresh1 == 0 || zzero(libzahl_tmp_divmod_b.as_mut_ptr()) != 0 {
                        break 's_253;
                    }
                    i = i.wrapping_add(1);
                }
                i = (if bit < (32 - 1i32) as u64 {
                    bit
                } else {
                    (32 - 1i32) as u64
                })
                .wrapping_add(1);
                loop {
                    let fresh2 = i;
                    i = i.wrapping_sub(1);
                    if !(fresh2 != 0) {
                        break;
                    }
                    zrsh(
                        (libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr(),
                        (libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr(),
                        32,
                    );
                }
            }
        }
        zswap(a, libzahl_tmp_divmod_a.as_mut_ptr());
        zswap(b, libzahl_tmp_divmod_b.as_mut_ptr());
        (*a).sign = sign;
    }
}
