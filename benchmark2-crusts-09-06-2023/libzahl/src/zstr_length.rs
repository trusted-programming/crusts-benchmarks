use libc;
extern "C" {
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsetu(_: *mut C2RustUnnamed, _: u64);
    fn zcmpmag(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> i32;
    fn zdiv(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsqr(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    static mut libzahl_tmp_str_div: z_t;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_tmp_str_mag: z_t;
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
#[inline]
extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> i32 {
    unsafe {
        return (*a).sign;
    }
}

#[inline]
extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> i32 {
    unsafe {
        return ((*a).sign == 0) as i32;
    }
}

#[no_mangle]
pub extern "C" fn zstr_length(mut a: *mut C2RustUnnamed, mut radix: u64) -> u64 {
    unsafe {
        let mut size_total = 1;
        let mut size_temp: u64 = 0;
        zset(libzahl_tmp_str_num.as_mut_ptr(), a);
        while zzero(libzahl_tmp_str_num.as_mut_ptr()) == 0 {
            zsetu(libzahl_tmp_str_mag.as_mut_ptr(), radix);
            zset(
                libzahl_tmp_str_div.as_mut_ptr(),
                libzahl_tmp_str_mag.as_mut_ptr(),
            );
            size_temp = 1;
            while zcmpmag(
                libzahl_tmp_str_mag.as_mut_ptr(),
                libzahl_tmp_str_num.as_mut_ptr(),
            ) <= 0
            {
                zset(
                    libzahl_tmp_str_div.as_mut_ptr(),
                    libzahl_tmp_str_mag.as_mut_ptr(),
                );
                zsqr(
                    libzahl_tmp_str_mag.as_mut_ptr(),
                    libzahl_tmp_str_mag.as_mut_ptr(),
                );
                size_temp <<= 1;
            }
            size_temp >>= 1;
            size_total = (size_total as u64).wrapping_add(size_temp) as u64;
            zdiv(
                libzahl_tmp_str_num.as_mut_ptr(),
                libzahl_tmp_str_num.as_mut_ptr(),
                libzahl_tmp_str_div.as_mut_ptr(),
            );
        }
        return size_total.wrapping_add((zsignum(a) < 0) as u64);
    }
}
