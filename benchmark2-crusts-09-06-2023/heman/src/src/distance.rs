use libc;
extern "C" {
    fn heman_image_create(width: i32, height: i32, nbands: i32) -> *mut heman_image;
    fn heman_image_destroy(_: *mut heman_image);
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn sqrt(_: f64) -> f64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heman_image_s {
    pub width: i32,
    pub height: i32,
    pub nbands: i32,
    pub data: *mut libc::c_float,
}
pub type heman_image = heman_image_s;
#[no_mangle]
pub static mut INF: libc::c_float = 1E20f64 as libc::c_float;
extern "C" fn edt(
    mut f: *mut libc::c_float,
    mut d: *mut libc::c_float,
    mut z: *mut libc::c_float,
    mut w: *mut u16,
    mut n: i32,
) {
    unsafe {
        let mut k = 0;
        let mut s: libc::c_float = 0.;
        *w.offset(0 as isize) = 0;
        *z.offset(0 as isize) = -INF;
        *z.offset(1 as isize) = INF;
        let mut q = 1;
        while q < n {
            s = (*f.offset(q as isize) + (q * q) as libc::c_float
                - (*f.offset(*w.offset(k as isize) as isize)
                    + (*w.offset(k as isize) as i32 * *w.offset(k as isize) as i32)
                        as libc::c_float))
                / (2 * q - 2 * *w.offset(k as isize) as i32) as libc::c_float;
            while s <= *z.offset(k as isize) {
                k -= 1;
                s = (*f.offset(q as isize) + (q * q) as libc::c_float
                    - (*f.offset(*w.offset(k as isize) as isize)
                        + (*w.offset(k as isize) as i32 * *w.offset(k as isize) as i32)
                            as libc::c_float))
                    / (2 * q - 2 * *w.offset(k as isize) as i32) as libc::c_float;
            }
            k += 1;
            *w.offset(k as isize) = q as u16;
            *z.offset(k as isize) = s;
            *z.offset((k + 1i32) as isize) = INF;
            q += 1;
        }
        k = 0;
        let mut q_0 = 0;
        while q_0 < n {
            while *z.offset((k + 1i32) as isize) < q_0 as libc::c_float {
                k += 1;
            }
            *d.offset(q_0 as isize) = ((q_0 - *w.offset(k as isize) as i32)
                * (q_0 - *w.offset(k as isize) as i32))
                as libc::c_float
                + *f.offset(*w.offset(k as isize) as isize);
            q_0 += 1;
        }
    }
}

extern "C" fn edt_with_payload(
    mut f: *mut libc::c_float,
    mut d: *mut libc::c_float,
    mut z: *mut libc::c_float,
    mut w: *mut u16,
    mut n: i32,
    mut payload_in: *mut libc::c_float,
    mut payload_out: *mut libc::c_float,
) {
    unsafe {
        let mut k = 0;
        let mut s: libc::c_float = 0.;
        *w.offset(0 as isize) = 0;
        *z.offset(0 as isize) = -INF;
        *z.offset(1 as isize) = INF;
        let mut q = 1;
        while q < n {
            s = (*f.offset(q as isize) + (q * q) as libc::c_float
                - (*f.offset(*w.offset(k as isize) as isize)
                    + (*w.offset(k as isize) as i32 * *w.offset(k as isize) as i32)
                        as libc::c_float))
                / (2 * q - 2 * *w.offset(k as isize) as i32) as libc::c_float;
            while s <= *z.offset(k as isize) {
                k -= 1;
                s = (*f.offset(q as isize) + (q * q) as libc::c_float
                    - (*f.offset(*w.offset(k as isize) as isize)
                        + (*w.offset(k as isize) as i32 * *w.offset(k as isize) as i32)
                            as libc::c_float))
                    / (2 * q - 2 * *w.offset(k as isize) as i32) as libc::c_float;
            }
            k += 1;
            *w.offset(k as isize) = q as u16;
            *z.offset(k as isize) = s;
            *z.offset((k + 1i32) as isize) = INF;
            q += 1;
        }
        k = 0;
        let mut q_0 = 0;
        while q_0 < n {
            while *z.offset((k + 1i32) as isize) < q_0 as libc::c_float {
                k += 1;
            }
            *d.offset(q_0 as isize) = ((q_0 - *w.offset(k as isize) as i32)
                * (q_0 - *w.offset(k as isize) as i32))
                as libc::c_float
                + *f.offset(*w.offset(k as isize) as isize);
            *payload_out.offset((q_0 * 2i32) as isize) =
                *payload_in.offset((*w.offset(k as isize) as i32 * 2i32) as isize);
            *payload_out.offset((q_0 * 2 + 1i32) as isize) =
                *payload_in.offset((*w.offset(k as isize) as i32 * 2 + 1i32) as isize);
            q_0 += 1;
        }
    }
}

extern "C" fn transform_to_distance(mut sdf: *mut heman_image) {
    unsafe {
        let mut width = (*sdf).width;
        let mut height = (*sdf).height;
        let mut size = width * height;
        let mut ff = calloc(size as u64, ::std::mem::size_of::<libc::c_float>() as u64)
            as *mut libc::c_float;
        let mut dd = calloc(size as u64, ::std::mem::size_of::<libc::c_float>() as u64)
            as *mut libc::c_float;
        let mut zz = calloc(
            ((height + 1i32) * (width + 1)) as u64,
            ::std::mem::size_of::<libc::c_float>() as u64,
        ) as *mut libc::c_float;
        let mut ww = calloc(size as u64, ::std::mem::size_of::<u16>() as u64) as *mut u16;
        let mut x: i32 = 0;
        x = 0;
        while x < width {
            let mut f = ff.offset((height * x) as isize);
            let mut d = dd.offset((height * x) as isize);
            let mut z = zz.offset(((height + 1i32) * x) as isize);
            let mut w = ww.offset((height * x) as isize);
            let mut y = 0;
            while y < height {
                *f.offset(y as isize) = *((*sdf).data)
                    .offset((y * width) as isize)
                    .offset(x as isize);
                y += 1;
            }
            edt(f, d, z, w, height);
            let mut y_0 = 0;
            while y_0 < height {
                *((*sdf).data)
                    .offset((y_0 * width) as isize)
                    .offset(x as isize) = *d.offset(y_0 as isize);
                y_0 += 1;
            }
            x += 1;
        }
        let mut y_1: i32 = 0;
        y_1 = 0;
        while y_1 < height {
            let mut f_0 = ff.offset((width * y_1) as isize);
            let mut d_0 = dd.offset((width * y_1) as isize);
            let mut z_0 = zz.offset(((width + 1i32) * y_1) as isize);
            let mut w_0 = ww.offset((width * y_1) as isize);
            let mut x_0 = 0;
            while x_0 < width {
                *f_0.offset(x_0 as isize) = *((*sdf).data)
                    .offset((y_1 * width) as isize)
                    .offset(x_0 as isize);
                x_0 += 1;
            }
            edt(f_0, d_0, z_0, w_0, width);
            let mut x_1 = 0;
            while x_1 < width {
                *((*sdf).data)
                    .offset((y_1 * width) as isize)
                    .offset(x_1 as isize) = *d_0.offset(x_1 as isize);
                x_1 += 1;
            }
            y_1 += 1;
        }
        free(ff as *mut libc::c_void);
        free(dd as *mut libc::c_void);
        free(zz as *mut libc::c_void);
        free(ww as *mut libc::c_void);
    }
}

extern "C" fn transform_to_coordfield(mut sdf: *mut heman_image, mut cf: *mut heman_image) {
    unsafe {
        let mut width = (*sdf).width;
        let mut height = (*sdf).height;
        let mut size = width * height;
        let mut ff = calloc(size as u64, ::std::mem::size_of::<libc::c_float>() as u64)
            as *mut libc::c_float;
        let mut dd = calloc(size as u64, ::std::mem::size_of::<libc::c_float>() as u64)
            as *mut libc::c_float;
        let mut zz = calloc(
            ((height + 1i32) * (width + 1)) as u64,
            ::std::mem::size_of::<libc::c_float>() as u64,
        ) as *mut libc::c_float;
        let mut ww = calloc(size as u64, ::std::mem::size_of::<u16>() as u64) as *mut u16;
        let mut x: i32 = 0;
        x = 0;
        while x < width {
            let mut pl1 = calloc(
                (height * 2i32) as u64,
                ::std::mem::size_of::<libc::c_float>() as u64,
            ) as *mut libc::c_float;
            let mut pl2 = calloc(
                (height * 2i32) as u64,
                ::std::mem::size_of::<libc::c_float>() as u64,
            ) as *mut libc::c_float;
            let mut f = ff.offset((height * x) as isize);
            let mut d = dd.offset((height * x) as isize);
            let mut z = zz.offset(((height + 1i32) * x) as isize);
            let mut w = ww.offset((height * x) as isize);
            let mut y = 0;
            while y < height {
                *f.offset(y as isize) = *((*sdf).data)
                    .offset((y * width) as isize)
                    .offset(x as isize);
                *pl1.offset((y * 2i32) as isize) = *((*cf).data)
                    .offset((2 * (y * width + x)) as isize)
                    .offset(0 as isize);
                *pl1.offset((y * 2 + 1i32) as isize) = *((*cf).data)
                    .offset((2 * (y * width + x)) as isize)
                    .offset(1 as isize);
                y += 1;
            }
            edt_with_payload(f, d, z, w, height, pl1, pl2);
            let mut y_0 = 0;
            while y_0 < height {
                *((*sdf).data)
                    .offset((y_0 * width) as isize)
                    .offset(x as isize) = *d.offset(y_0 as isize);
                *((*cf).data)
                    .offset((2 * (y_0 * width + x)) as isize)
                    .offset(0 as isize) = *pl2.offset((2 * y_0) as isize);
                *((*cf).data)
                    .offset((2 * (y_0 * width + x)) as isize)
                    .offset(1 as isize) = *pl2.offset((2 * y_0 + 1i32) as isize);
                y_0 += 1;
            }
            free(pl1 as *mut libc::c_void);
            free(pl2 as *mut libc::c_void);
            x += 1;
        }
        let mut y_1: i32 = 0;
        y_1 = 0;
        while y_1 < height {
            let mut pl1_0 = calloc(
                (width * 2i32) as u64,
                ::std::mem::size_of::<libc::c_float>() as u64,
            ) as *mut libc::c_float;
            let mut pl2_0 = calloc(
                (width * 2i32) as u64,
                ::std::mem::size_of::<libc::c_float>() as u64,
            ) as *mut libc::c_float;
            let mut f_0 = ff.offset((width * y_1) as isize);
            let mut d_0 = dd.offset((width * y_1) as isize);
            let mut z_0 = zz.offset(((width + 1i32) * y_1) as isize);
            let mut w_0 = ww.offset((width * y_1) as isize);
            let mut x_0 = 0;
            while x_0 < width {
                *f_0.offset(x_0 as isize) = *((*sdf).data)
                    .offset((y_1 * width) as isize)
                    .offset(x_0 as isize);
                *pl1_0.offset((x_0 * 2i32) as isize) = *((*cf).data)
                    .offset((2 * (y_1 * width + x_0)) as isize)
                    .offset(0 as isize);
                *pl1_0.offset((x_0 * 2 + 1i32) as isize) = *((*cf).data)
                    .offset((2 * (y_1 * width + x_0)) as isize)
                    .offset(1 as isize);
                x_0 += 1;
            }
            edt_with_payload(f_0, d_0, z_0, w_0, width, pl1_0, pl2_0);
            let mut x_1 = 0;
            while x_1 < width {
                *((*sdf).data)
                    .offset((y_1 * width) as isize)
                    .offset(x_1 as isize) = *d_0.offset(x_1 as isize);
                *((*cf).data)
                    .offset((2 * (y_1 * width + x_1)) as isize)
                    .offset(0 as isize) = *pl2_0.offset((2 * x_1) as isize);
                *((*cf).data)
                    .offset((2 * (y_1 * width + x_1)) as isize)
                    .offset(1 as isize) = *pl2_0.offset((2 * x_1 + 1i32) as isize);
                x_1 += 1;
            }
            free(pl1_0 as *mut libc::c_void);
            free(pl2_0 as *mut libc::c_void);
            y_1 += 1;
        }
        free(ff as *mut libc::c_void);
        free(dd as *mut libc::c_void);
        free(zz as *mut libc::c_void);
        free(ww as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn heman_distance_create_sdf(mut src: *mut heman_image) -> *mut heman_image {
    unsafe {
        if (*src).nbands == 1
            && !(b"Distance field input must have only 1 band.\0" as *const u8 as *const i8)
                .is_null()
        {
        } else {
            __assert_fail(
                b"src->nbands == 1 && \"Distance field input must have only 1 band.\"\0"
                    as *const u8 as *const i8,
                b"../src/distance.c\0" as *const u8 as *const i8,
                183,
                (*::std::mem::transmute::<&[u8; 54], &[i8; 54]>(
                    b"heman_image *heman_distance_create_sdf(heman_image *)\0",
                ))
                .as_ptr(),
            );
        }
        let mut positive = heman_image_create((*src).width, (*src).height, 1);
        let mut negative = heman_image_create((*src).width, (*src).height, 1);
        let mut size = (*src).height * (*src).width;
        let mut pptr = (*positive).data;
        let mut nptr = (*negative).data;
        let mut sptr = (*src).data;
        let mut i = 0;
        while i < size {
            let fresh0 = pptr;
            pptr = pptr.offset(1);
            *fresh0 = if *sptr != 0. { INF } else { 0 as libc::c_float };
            let fresh1 = nptr;
            nptr = nptr.offset(1);
            *fresh1 = if *sptr != 0. { 0 as libc::c_float } else { INF };
            i += 1;
            sptr = sptr.offset(1);
        }
        transform_to_distance(positive);
        transform_to_distance(negative);
        let mut inv = 1.0f32 / (*src).width as libc::c_float;
        pptr = (*positive).data;
        nptr = (*negative).data;
        let mut i_0 = 0;
        while i_0 < size {
            *pptr = ((sqrt(*pptr as f64) - sqrt(*nptr as f64)) * inv as f64) as libc::c_float;
            i_0 += 1;
            pptr = pptr.offset(1);
            nptr = nptr.offset(1);
        }
        heman_image_destroy(negative);
        return positive;
    }
}

#[no_mangle]
pub extern "C" fn heman_distance_create_df(mut src: *mut heman_image) -> *mut heman_image {
    unsafe {
        if (*src).nbands == 1
            && !(b"Distance field input must have only 1 band.\0" as *const u8 as *const i8)
                .is_null()
        {
        } else {
            __assert_fail(
                b"src->nbands == 1 && \"Distance field input must have only 1 band.\"\0"
                    as *const u8 as *const i8,
                b"../src/distance.c\0" as *const u8 as *const i8,
                208,
                (*::std::mem::transmute::<&[u8; 53], &[i8; 53]>(
                    b"heman_image *heman_distance_create_df(heman_image *)\0",
                ))
                .as_ptr(),
            );
        }
        let mut positive = heman_image_create((*src).width, (*src).height, 1);
        let mut size = (*src).height * (*src).width;
        let mut pptr = (*positive).data;
        let mut sptr = (*src).data;
        let mut i = 0;
        while i < size {
            let fresh2 = pptr;
            pptr = pptr.offset(1);
            *fresh2 = if *sptr != 0. { 0 as libc::c_float } else { INF };
            i += 1;
            sptr = sptr.offset(1);
        }
        transform_to_distance(positive);
        let mut inv = 1.0f32 / (*src).width as libc::c_float;
        pptr = (*positive).data;
        let mut i_0 = 0;
        while i_0 < size {
            *pptr = (sqrt(*pptr as f64) * inv as f64) as libc::c_float;
            i_0 += 1;
            pptr = pptr.offset(1);
        }
        return positive;
    }
}

#[no_mangle]
pub extern "C" fn heman_distance_identity_cpcf(
    mut width: i32,
    mut height: i32,
) -> *mut heman_image {
    unsafe {
        let mut retval = heman_image_create(width, height, 2);
        let mut cdata = (*retval).data;
        let mut y = 0;
        while y < height {
            let mut x = 0;
            while x < width {
                let fresh3 = cdata;
                cdata = cdata.offset(1);
                *fresh3 = x as libc::c_float;
                let fresh4 = cdata;
                cdata = cdata.offset(1);
                *fresh4 = y as libc::c_float;
                x += 1;
            }
            y += 1;
        }
        return retval;
    }
}

#[no_mangle]
pub extern "C" fn heman_distance_create_cpcf(mut src: *mut heman_image) -> *mut heman_image {
    unsafe {
        let mut negative = heman_image_create((*src).width, (*src).height, 1);
        let mut size = (*src).height * (*src).width;
        let mut nptr = (*negative).data;
        let mut sptr = (*src).data;
        let mut i = 0;
        while i < size {
            let mut val = 0 as libc::c_float;
            let mut b = 0;
            while b < (*src).nbands {
                let fresh5 = sptr;
                sptr = sptr.offset(1);
                val += *fresh5;
                b += 1;
            }
            let fresh6 = nptr;
            nptr = nptr.offset(1);
            *fresh6 = if val != 0. { 0 as libc::c_float } else { INF };
            i += 1;
        }
        let mut coordfield = heman_distance_identity_cpcf((*src).width, (*src).height);
        transform_to_coordfield(negative, coordfield);
        heman_image_destroy(negative);
        return coordfield;
    }
}

#[no_mangle]
pub extern "C" fn heman_distance_from_cpcf(mut cf: *mut heman_image) -> *mut heman_image {
    unsafe {
        if (*cf).nbands == 2
            && !(b"Coordinate field input must have 2 bands.\0" as *const u8 as *const i8).is_null()
        {
        } else {
            __assert_fail(
                b"cf->nbands == 2 && \"Coordinate field input must have 2 bands.\"\0" as *const u8
                    as *const i8,
                b"../src/distance.c\0" as *const u8 as *const i8,
                259,
                (*::std::mem::transmute::<&[u8; 53], &[i8; 53]>(
                    b"heman_image *heman_distance_from_cpcf(heman_image *)\0",
                ))
                .as_ptr(),
            );
        }
        let mut udf = heman_image_create((*cf).width, (*cf).height, 1);
        let mut dptr = (*udf).data;
        let mut sptr = (*cf).data;
        let mut scale = (1.0f32 as f64
            / sqrt(((*cf).width * (*cf).width + (*cf).height * (*cf).height) as f64))
            as libc::c_float;
        let mut y = 0;
        while y < (*cf).height {
            let mut x = 0;
            while x < (*cf).width {
                let fresh7 = sptr;
                sptr = sptr.offset(1);
                let mut u = *fresh7;
                let fresh8 = sptr;
                sptr = sptr.offset(1);
                let mut v = *fresh8;
                let mut dist = (sqrt(
                    ((u - x as libc::c_float) * (u - x as libc::c_float)
                        + (v - y as libc::c_float) * (v - y as libc::c_float))
                        as f64,
                ) * scale as f64) as libc::c_float;
                let fresh9 = dptr;
                dptr = dptr.offset(1);
                *fresh9 = dist;
                x += 1;
            }
            y += 1;
        }
        return udf;
    }
}
