use libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn free(_: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn generate_gaussian_row(mut target: *mut i32, mut fwidth: i32) {
    unsafe {
        if fwidth > 0 {
        } else {
            __assert_fail(
                b"fwidth > 0\0" as *const u8 as *const i8,
                b"../src/gaussian.c\0" as *const u8 as *const i8,
                9,
                (*::std::mem::transmute::<&[u8; 39], &[i8; 39]>(
                    b"void generate_gaussian_row(int *, int)\0",
                ))
                .as_ptr(),
            );
        }
        let mut nbytes = (fwidth as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64) as i32;
        let mut tmp = malloc(nbytes as u64) as *mut i32;
        let ref mut fresh0 = *tmp.offset(0 as isize);
        *fresh0 = 1;
        *target.offset(0 as isize) = *fresh0;
        let mut col = 1;
        while col < fwidth {
            *target.offset(col as isize) = 0;
            *tmp.offset(col as isize) = 0;
            col += 1;
        }
        let mut row = 1;
        while row < fwidth {
            let mut col_0 = 1;
            while col_0 <= row {
                *target.offset(col_0 as isize) =
                    *tmp.offset(col_0 as isize) + *tmp.offset((col_0 - 1i32) as isize);
                col_0 += 1;
            }
            let mut col_1 = 1;
            while col_1 <= row {
                *tmp.offset(col_1 as isize) = *target.offset(col_1 as isize);
                col_1 += 1;
            }
            row += 1;
        }
        free(tmp as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn generate_gaussian_splat(mut target: *mut libc::c_float, mut fwidth: i32) {
    unsafe {
        let mut gaussian_row =
            malloc((fwidth as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64)) as *mut i32;
        generate_gaussian_row(gaussian_row, fwidth);
        let mut shift = 1 << fwidth - 1;
        let mut scale = (1.0f64 / (shift * shift) as f64) as libc::c_float;
        let mut gptr = target;
        let mut j = 0;
        while j < fwidth {
            let mut i = 0;
            while i < fwidth {
                let fresh1 = gptr;
                gptr = gptr.offset(1);
                *fresh1 = (*gaussian_row.offset(i as isize) * *gaussian_row.offset(j as isize))
                    as libc::c_float
                    * scale;
                i += 1;
            }
            j += 1;
        }
        free(gaussian_row as *mut libc::c_void);
    }
}
