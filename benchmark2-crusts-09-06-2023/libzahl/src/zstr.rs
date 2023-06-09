use libc;
extern "C" {
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    fn zdivmod(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
    );
    fn zabs(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    static mut libzahl_tmp_str_rem: z_t;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_const_1e9: z_t;
    static mut libzahl_jmp_buf: jmp_buf;
    fn __errno_location() -> *mut i32;
    static mut libzahl_error: i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn zstr_length(_: *mut C2RustUnnamed, _: u64) -> u64;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
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
pub extern "C" fn zstr(mut a: *mut C2RustUnnamed, mut b: *mut i8) -> *mut i8 {
    unsafe {
        let mut buf: [i8; 10] = [0; 10];
        let mut n: u64 = 0;
        let mut len: u64 = 0;
        let mut overridden = 0;
        let mut neg: i32 = 0;
        if zzero(a) != 0 {
            if b.is_null() {
                b = malloc(2) as *mut i8;
                if b.is_null() {
                    libzahl_error = *__errno_location();
                    longjmp(libzahl_jmp_buf.as_mut_ptr(), 1);
                }
            };
            *b.offset(0 as isize) = '0' as i8;
            *b.offset(1 as isize) = 0;
            return b;
        }
        n = zstr_length(a, 10);
        if b.is_null() {
            b = malloc(n.wrapping_add(1)) as *mut i8;
            if b.is_null() {
                libzahl_error = *__errno_location();
                longjmp(libzahl_jmp_buf.as_mut_ptr(), 1);
            }
        }
        neg = (zsignum(a) < 0) as i32;
        zabs(libzahl_tmp_str_num.as_mut_ptr(), a);
        *b.offset(0 as isize) = '-' as i8;
        b = b.offset(neg as isize);
        n = (n).wrapping_sub(neg as u64) as u64;
        n = if n > 9 { n.wrapping_sub(9) } else { 0 };
        loop {
            zdivmod(
                libzahl_tmp_str_num.as_mut_ptr(),
                libzahl_tmp_str_rem.as_mut_ptr(),
                libzahl_tmp_str_num.as_mut_ptr(),
                libzahl_const_1e9.as_mut_ptr(),
            );
            if zzero(libzahl_tmp_str_num.as_mut_ptr()) == 0 {
                sprintf(
                    b.offset(n as isize),
                    b"%09lu\0" as *const u8 as *const i8,
                    if zzero(libzahl_tmp_str_rem.as_mut_ptr()) != 0 {
                        0
                    } else {
                        *((*libzahl_tmp_str_rem.as_mut_ptr()).chars).offset(0 as isize) as u64
                    },
                );
                *b.offset(n.wrapping_add(9) as isize) = overridden;
                overridden = *b.offset(n as isize);
                n = if n > 9 { n.wrapping_sub(9) } else { 0 };
            } else {
                len = sprintf(
                    buf.as_mut_ptr(),
                    b"%lu\0" as *const u8 as *const i8,
                    *((*libzahl_tmp_str_rem.as_mut_ptr()).chars).offset(0 as isize) as u64,
                ) as u64;
                if overridden != 0 {
                    buf[len as usize] = *b.offset(n.wrapping_add(len) as isize);
                }
                memcpy(
                    b.offset(n as isize) as *mut libc::c_void,
                    buf.as_mut_ptr() as *const libc::c_void,
                    len.wrapping_add(1),
                );
                break;
            }
        }
        return b.offset(-(neg as isize));
    }
}
