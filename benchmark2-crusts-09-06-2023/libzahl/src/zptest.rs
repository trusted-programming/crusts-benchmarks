use libc;
extern "C" {
    fn zswap(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zcmp(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> i32;
    fn zcmpu(_: *mut C2RustUnnamed, _: u64) -> i32;
    fn zmod(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsqr(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmodpow(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
    );
    fn zadd_unsigned(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsub_unsigned(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zrsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: u64);
    fn zlsb(_: *mut C2RustUnnamed) -> u64;
    static mut libzahl_tmp_ptest_a: z_t;
    static mut libzahl_tmp_ptest_n1: z_t;
    static mut libzahl_tmp_ptest_x: z_t;
    static mut libzahl_const_1: z_t;
    static mut libzahl_tmp_ptest_d: z_t;
    static mut libzahl_const_2: z_t;
    static mut libzahl_tmp_ptest_n4: z_t;
    fn zrand(_: *mut C2RustUnnamed, _: u32, _: u32, _: *mut C2RustUnnamed);
    static mut libzahl_const_4: z_t;
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
pub const PRIME: u32 = 2;
pub const PROBABLY_PRIME: u32 = 1;
pub const NONPRIME: u32 = 0;
pub const SECURE_RANDOM: u32 = 1;
pub const FAST_RANDOM: u32 = 0;
pub const UNIFORM: u32 = 1;
pub const QUASIUNIFORM: u32 = 0;
#[inline]
extern "C" fn zeven(mut a_0: *mut C2RustUnnamed) -> i32 {
    unsafe {
        return ((*a_0).sign == 0 || *((*a_0).chars).offset(0 as isize) & 1 == 0) as i32;
    }
}

#[no_mangle]
pub extern "C" fn zptest(
    mut witness: *mut C2RustUnnamed,
    mut n: *mut C2RustUnnamed,
    mut t: i32,
) -> u32 {
    unsafe {
        let mut i: u64 = 0;
        let mut r: u64 = 0;
        if zcmpu(n, 3) <= 0 {
            if zcmpu(n, 1) <= 0 {
                if !witness.is_null() {
                    if witness != n {
                        zset(witness, n);
                    }
                }
                return NONPRIME;
            } else {
                return PRIME;
            }
        }
        if zeven(n) != 0 {
            if !witness.is_null() {
                if witness != n {
                    zset(witness, n);
                }
            }
            return NONPRIME;
        }
        zsub_unsigned(
            libzahl_tmp_ptest_n1.as_mut_ptr(),
            n,
            libzahl_const_1.as_mut_ptr(),
        );
        zsub_unsigned(
            libzahl_tmp_ptest_n4.as_mut_ptr(),
            n,
            libzahl_const_4.as_mut_ptr(),
        );
        r = zlsb(libzahl_tmp_ptest_n1.as_mut_ptr());
        zrsh(
            libzahl_tmp_ptest_d.as_mut_ptr(),
            libzahl_tmp_ptest_n1.as_mut_ptr(),
            r,
        );
        loop {
            let fresh0 = t;
            t = t - 1;
            if !(fresh0 != 0) {
                break;
            }
            zrand(
                libzahl_tmp_ptest_a.as_mut_ptr(),
                FAST_RANDOM,
                UNIFORM,
                libzahl_tmp_ptest_n4.as_mut_ptr(),
            );
            zadd_unsigned(
                libzahl_tmp_ptest_a.as_mut_ptr(),
                libzahl_tmp_ptest_a.as_mut_ptr(),
                libzahl_const_2.as_mut_ptr(),
            );
            zmodpow(
                libzahl_tmp_ptest_x.as_mut_ptr(),
                libzahl_tmp_ptest_a.as_mut_ptr(),
                libzahl_tmp_ptest_d.as_mut_ptr(),
                n,
            );
            if zcmp(
                libzahl_tmp_ptest_x.as_mut_ptr(),
                libzahl_const_1.as_mut_ptr(),
            ) == 0
                || zcmp(
                    libzahl_tmp_ptest_x.as_mut_ptr(),
                    libzahl_tmp_ptest_n1.as_mut_ptr(),
                ) == 0
            {
                continue;
            }
            i = 1;
            while i < r {
                zsqr(
                    libzahl_tmp_ptest_x.as_mut_ptr(),
                    libzahl_tmp_ptest_x.as_mut_ptr(),
                );
                zmod(
                    libzahl_tmp_ptest_x.as_mut_ptr(),
                    libzahl_tmp_ptest_x.as_mut_ptr(),
                    n,
                );
                if zcmp(
                    libzahl_tmp_ptest_x.as_mut_ptr(),
                    libzahl_const_1.as_mut_ptr(),
                ) == 0
                {
                    if !witness.is_null() {
                        zswap(witness, libzahl_tmp_ptest_a.as_mut_ptr());
                    }
                    return NONPRIME;
                }
                if zcmp(
                    libzahl_tmp_ptest_x.as_mut_ptr(),
                    libzahl_tmp_ptest_n1.as_mut_ptr(),
                ) == 0
                {
                    break;
                }
                i = i.wrapping_add(1);
            }
            if i == r {
                if !witness.is_null() {
                    zswap(witness, libzahl_tmp_ptest_a.as_mut_ptr());
                }
                return NONPRIME;
            }
        }
        return PROBABLY_PRIME;
    }
}
