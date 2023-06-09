use libc;
extern "C" {
    pub type osn_context;
    fn heman_image_create(width: i32, height: i32, nbands: i32) -> *mut heman_image;
    fn heman_image_texel(_: *mut heman_image, x: i32, y: i32) -> *mut libc::c_float;
    fn heman_image_destroy(_: *mut heman_image);
    fn heman_color_to_grayscale(colorimg: *mut heman_image) -> *mut heman_image;
    fn heman_distance_identity_cpcf(width: i32, height: i32) -> *mut heman_image;
    fn open_simplex_noise_free(ctx: *mut osn_context);
    fn open_simplex_noise2(ctx: *mut osn_context, x: f64, y: f64) -> f64;
    fn open_simplex_noise(seed: i64, ctx: *mut *mut osn_context) -> i32;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn pow(_: f64, _: f64) -> f64;
    fn fabsf(_: libc::c_float) -> libc::c_float;
    fn floor(_: f64) -> f64;
    fn kmVec3Lerp(
        pOut: *mut kmVec3,
        pV1: *const kmVec3,
        pV2: *const kmVec3,
        t: libc::c_float,
    ) -> *mut kmVec3;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn omp_get_max_threads() -> i32;
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
pub type heman_points = heman_image_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
}
#[no_mangle]
pub extern "C" fn heman_get_num_threads() -> i32 {
    unsafe {
        return omp_get_max_threads();
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_step(
    mut hmap: *mut heman_image,
    mut threshold: libc::c_float,
) -> *mut heman_image {
    unsafe {
        if (*hmap).nbands == 1 {
        } else {
            __assert_fail(
                b"hmap->nbands == 1\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                17,
                (*::std::mem::transmute::<&[u8; 50], &[i8; 50]>(
                    b"heman_image *heman_ops_step(heman_image *, float)\0",
                ))
                .as_ptr(),
            );
        }
        let mut result = heman_image_create((*hmap).width, (*hmap).height, 1);
        let mut size = (*hmap).height * (*hmap).width;
        let mut src = (*hmap).data;
        let mut dst = (*result).data;
        let mut i = 0;
        while i < size {
            let fresh0 = src;
            src = src.offset(1);
            let fresh1 = dst;
            dst = dst.offset(1);
            *fresh1 = (if *fresh0 >= threshold { 1i32 } else { 0 }) as libc::c_float;
            i += 1;
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_max(
    mut imga: *mut heman_image,
    mut imgb: *mut heman_image,
) -> *mut heman_image {
    unsafe {
        if (*imga).width == (*imgb).width {
        } else {
            __assert_fail(
                b"imga->width == imgb->width\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                30,
                (*::std::mem::transmute::<&[u8; 57], &[i8; 57]>(
                    b"heman_image *heman_ops_max(heman_image *, heman_image *)\0",
                ))
                .as_ptr(),
            );
        }
        if (*imga).height == (*imgb).height {
        } else {
            __assert_fail(
                b"imga->height == imgb->height\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                31,
                (*::std::mem::transmute::<&[u8; 57], &[i8; 57]>(
                    b"heman_image *heman_ops_max(heman_image *, heman_image *)\0",
                ))
                .as_ptr(),
            );
        }
        if (*imga).nbands == (*imgb).nbands {
        } else {
            __assert_fail(
                b"imga->nbands == imgb->nbands\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                32,
                (*::std::mem::transmute::<&[u8; 57], &[i8; 57]>(
                    b"heman_image *heman_ops_max(heman_image *, heman_image *)\0",
                ))
                .as_ptr(),
            );
        }
        let mut result = heman_image_create((*imga).width, (*imga).height, (*imga).nbands);
        let mut size = (*imga).height * (*imga).width * (*imga).nbands;
        let mut srca = (*imga).data;
        let mut srcb = (*imgb).data;
        let mut dst = (*result).data;
        let mut i = 0;
        while i < size {
            *dst = if *srca > *srcb { *srca } else { *srcb };
            i += 1;
            dst = dst.offset(1);
            srca = srca.offset(1);
            srcb = srcb.offset(1);
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_sweep(mut hmap: *mut heman_image) -> *mut heman_image {
    unsafe {
        if (*hmap).nbands == 1 {
        } else {
            __assert_fail(
                b"hmap->nbands == 1\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                47,
                (*::std::mem::transmute::<&[u8; 44], &[i8; 44]>(
                    b"heman_image *heman_ops_sweep(heman_image *)\0",
                ))
                .as_ptr(),
            );
        }
        let mut result = heman_image_create((*hmap).height, 1, 1);
        let mut dst = (*result).data;
        let mut src: *const libc::c_float = (*hmap).data;
        let mut invw = 1.0f32 / (*hmap).width as libc::c_float;
        let mut y = 0;
        while y < (*hmap).height {
            let mut acc = 0 as libc::c_float;
            let mut x = 0;
            while x < (*hmap).width {
                let fresh2 = src;
                src = src.offset(1);
                acc += *fresh2;
                x += 1;
            }
            let fresh3 = dst;
            dst = dst.offset(1);
            *fresh3 = acc * invw;
            y += 1;
        }
        return result;
    }
}

extern "C" fn copy_row(
    mut src: *mut heman_image,
    mut dst: *mut heman_image,
    mut dstx: i32,
    mut y: i32,
) {
    unsafe {
        let mut width = (*src).width;
        if (*src).nbands == 1 {
            let mut x = 0;
            while x < width {
                let mut srcp = heman_image_texel(src, x, y);
                let mut dstp = heman_image_texel(dst, dstx + x, y);
                *dstp = *srcp;
                x += 1;
            }
            return;
        }
        let mut x_0 = 0;
        while x_0 < width {
            let mut srcp_0 = heman_image_texel(src, x_0, y);
            let mut dstp_0 = heman_image_texel(dst, dstx + x_0, y);
            let mut nbands = (*src).nbands;
            loop {
                let fresh4 = nbands;
                nbands = nbands - 1;
                if !(fresh4 != 0) {
                    break;
                }
                let fresh5 = srcp_0;
                srcp_0 = srcp_0.offset(1);
                let fresh6 = dstp_0;
                dstp_0 = dstp_0.offset(1);
                *fresh6 = *fresh5;
            }
            x_0 += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_stitch_horizontal(
    mut images: *mut *mut heman_image,
    mut count: i32,
) -> *mut heman_image {
    unsafe {
        if count > 0 {
        } else {
            __assert_fail(
                b"count > 0\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                85,
                (*::std::mem::transmute::<&[u8; 62], &[i8; 62]>(
                    b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0",
                ))
                .as_ptr(),
            );
        }
        let mut width = (**images.offset(0 as isize)).width;
        let mut height = (**images.offset(0 as isize)).height;
        let mut nbands = (**images.offset(0 as isize)).nbands;
        let mut i = 1;
        while i < count {
            if (**images.offset(i as isize)).width == width {
            } else {
                __assert_fail(
                    b"images[i]->width == width\0" as *const u8 as *const i8,
                    b"../src/ops.c\0" as *const u8 as *const i8,
                    90,
                    (*::std::mem::transmute::<&[u8; 62], &[i8; 62]>(
                        b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0",
                    ))
                    .as_ptr(),
                );
            }
            if (**images.offset(i as isize)).height == height {
            } else {
                __assert_fail(
                    b"images[i]->height == height\0" as *const u8 as *const i8,
                    b"../src/ops.c\0" as *const u8 as *const i8,
                    91,
                    (*::std::mem::transmute::<&[u8; 62], &[i8; 62]>(
                        b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0",
                    ))
                    .as_ptr(),
                );
            }
            if (**images.offset(i as isize)).nbands == nbands {
            } else {
                __assert_fail(
                    b"images[i]->nbands == nbands\0" as *const u8 as *const i8,
                    b"../src/ops.c\0" as *const u8 as *const i8,
                    92,
                    (*::std::mem::transmute::<&[u8; 62], &[i8; 62]>(
                        b"heman_image *heman_ops_stitch_horizontal(heman_image **, int)\0",
                    ))
                    .as_ptr(),
                );
            }
            i += 1;
        }
        let mut result = heman_image_create(width * count, height, nbands);
        let mut y: i32 = 0;
        y = 0;
        while y < height {
            let mut tile = 0;
            while tile < count {
                copy_row(*images.offset(tile as isize), result, tile * width, y);
                tile += 1;
            }
            y += 1;
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_stitch_vertical(
    mut images: *mut *mut heman_image,
    mut count: i32,
) -> *mut heman_image {
    unsafe {
        if count > 0 {
        } else {
            __assert_fail(
                b"count > 0\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                109,
                (*::std::mem::transmute::<&[u8; 60], &[i8; 60]>(
                    b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0",
                ))
                .as_ptr(),
            );
        }
        let mut width = (**images.offset(0 as isize)).width;
        let mut height = (**images.offset(0 as isize)).height;
        let mut nbands = (**images.offset(0 as isize)).nbands;
        let mut i = 1;
        while i < count {
            if (**images.offset(i as isize)).width == width {
            } else {
                __assert_fail(
                    b"images[i]->width == width\0" as *const u8 as *const i8,
                    b"../src/ops.c\0" as *const u8 as *const i8,
                    114,
                    (*::std::mem::transmute::<&[u8; 60], &[i8; 60]>(
                        b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0",
                    ))
                    .as_ptr(),
                );
            }
            if (**images.offset(i as isize)).height == height {
            } else {
                __assert_fail(
                    b"images[i]->height == height\0" as *const u8 as *const i8,
                    b"../src/ops.c\0" as *const u8 as *const i8,
                    115,
                    (*::std::mem::transmute::<&[u8; 60], &[i8; 60]>(
                        b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0",
                    ))
                    .as_ptr(),
                );
            }
            if (**images.offset(i as isize)).nbands == nbands {
            } else {
                __assert_fail(
                    b"images[i]->nbands == nbands\0" as *const u8 as *const i8,
                    b"../src/ops.c\0" as *const u8 as *const i8,
                    116,
                    (*::std::mem::transmute::<&[u8; 60], &[i8; 60]>(
                        b"heman_image *heman_ops_stitch_vertical(heman_image **, int)\0",
                    ))
                    .as_ptr(),
                );
            }
            i += 1;
        }
        let mut result = heman_image_create(width, height * count, nbands);
        let mut size = width * height * nbands;
        let mut dst = (*result).data;
        let mut tile = 0;
        while tile < count {
            memcpy(
                dst as *mut libc::c_void,
                (**images.offset(tile as isize)).data as *const libc::c_void,
                (size as u64).wrapping_mul(::std::mem::size_of::<libc::c_float>() as u64),
            );
            dst = dst.offset(size as isize);
            tile += 1;
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_normalize_f32(
    mut source: *mut heman_image,
    mut minv: libc::c_float,
    mut maxv: libc::c_float,
) -> *mut heman_image {
    unsafe {
        let mut result = heman_image_create((*source).width, (*source).height, (*source).nbands);
        let mut src = (*source).data;
        let mut dst = (*result).data;
        let mut scale = 1.0f32 / (maxv - minv);
        let mut size = (*source).height * (*source).width * (*source).nbands;
        let mut i = 0;
        while i < size {
            let fresh7 = src;
            src = src.offset(1);
            let mut v = (*fresh7 - minv) * scale;
            let fresh8 = dst;
            dst = dst.offset(1);
            *fresh8 = if 0 as libc::c_float
                > (if 1 as libc::c_float > v {
                    v
                } else {
                    1 as libc::c_float
                }) {
                0 as libc::c_float
            } else if 1 as libc::c_float > v {
                v
            } else {
                1 as libc::c_float
            };
            i += 1;
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_laplacian(mut heightmap: *mut heman_image) -> *mut heman_image {
    unsafe {
        if (*heightmap).nbands == 1 {
        } else {
            __assert_fail(
                b"heightmap->nbands == 1\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                146,
                (*::std::mem::transmute::<&[u8; 48], &[i8; 48]>(
                    b"heman_image *heman_ops_laplacian(heman_image *)\0",
                ))
                .as_ptr(),
            );
        }
        let mut width = (*heightmap).width;
        let mut height = (*heightmap).height;
        let mut result = heman_image_create(width, height, 1);
        let mut maxx = width - 1;
        let mut maxy = height - 1;
        let mut y: i32 = 0;
        y = 0;
        while y < height {
            let mut y1 = if y + 1 > maxy { maxy } else { y + 1 };
            let mut dst = ((*result).data).offset((y * width) as isize);
            let mut x = 0;
            while x < width {
                let mut x1 = if x + 1 > maxx { maxx } else { x + 1 };
                let mut p = *heman_image_texel(heightmap, x, y);
                let mut px = *heman_image_texel(heightmap, x1, y);
                let mut py = *heman_image_texel(heightmap, x, y1);
                let fresh9 = dst;
                dst = dst.offset(1);
                *fresh9 = (p - px) * (p - px) + (p - py) * (p - py);
                x += 1;
            }
            y += 1;
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_accumulate(mut dst: *mut heman_image, mut src: *mut heman_image) {
    unsafe {
        if (*dst).nbands == (*src).nbands {
        } else {
            __assert_fail(
                b"dst->nbands == src->nbands\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                172,
                (*::std::mem::transmute::<&[u8; 56], &[i8; 56]>(
                    b"void heman_ops_accumulate(heman_image *, heman_image *)\0",
                ))
                .as_ptr(),
            );
        }
        if (*dst).width == (*src).width {
        } else {
            __assert_fail(
                b"dst->width == src->width\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                173,
                (*::std::mem::transmute::<&[u8; 56], &[i8; 56]>(
                    b"void heman_ops_accumulate(heman_image *, heman_image *)\0",
                ))
                .as_ptr(),
            );
        }
        if (*dst).height == (*src).height {
        } else {
            __assert_fail(
                b"dst->height == src->height\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                174,
                (*::std::mem::transmute::<&[u8; 56], &[i8; 56]>(
                    b"void heman_ops_accumulate(heman_image *, heman_image *)\0",
                ))
                .as_ptr(),
            );
        }
        let mut size = (*dst).height * (*dst).width;
        let mut sdata = (*src).data;
        let mut ddata = (*dst).data;
        let mut i = 0;
        while i < size {
            let fresh10 = sdata;
            sdata = sdata.offset(1);
            let fresh11 = ddata;
            ddata = ddata.offset(1);
            *fresh11 += *fresh10;
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_sobel(mut img: *mut heman_image, mut rgb: u32) -> *mut heman_image {
    unsafe {
        let mut width = (*img).width;
        let mut height = (*img).height;
        if (*img).nbands == 3 {
        } else {
            __assert_fail(
                b"img->nbands == 3\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                187,
                (*::std::mem::transmute::<&[u8; 57], &[i8; 57]>(
                    b"heman_image *heman_ops_sobel(heman_image *, heman_color)\0",
                ))
                .as_ptr(),
            );
        }
        let mut result = heman_image_create(width, height, 3);
        let mut gray = heman_color_to_grayscale(img);
        let mut inv = 1.0f32 / 255.0f32;
        let mut edge_rgb = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        edge_rgb.x = (rgb >> 16i32) as libc::c_float * inv;
        edge_rgb.y = (rgb >> 8 & 0xffu32) as libc::c_float * inv;
        edge_rgb.z = (rgb & 0xffu32) as libc::c_float * inv;
        let mut y: i32 = 0;
        y = 0;
        while y < height {
            let mut dst = ((*result).data as *mut kmVec3).offset((y * width) as isize);
            let mut src: *const kmVec3 = ((*img).data as *mut kmVec3).offset((y * width) as isize);
            let mut x = 0;
            while x < width {
                let mut xm1 = if x - 1 > 0 { x - 1 } else { 0 };
                let mut xp1 = if x + 1 > width - 1 { width - 1 } else { x + 1 };
                let mut ym1 = if y - 1 > 0 { y - 1 } else { 0 };
                let mut yp1 = if y + 1 > height - 1 {
                    height - 1
                } else {
                    y + 1
                };
                let mut t00 = *heman_image_texel(gray, xm1, ym1);
                let mut t10 = *heman_image_texel(gray, x, ym1);
                let mut t20 = *heman_image_texel(gray, xp1, ym1);
                let mut t01 = *heman_image_texel(gray, xm1, 0);
                let mut t21 = *heman_image_texel(gray, xp1, 0);
                let mut t02 = *heman_image_texel(gray, xm1, yp1);
                let mut t12 = *heman_image_texel(gray, x, yp1);
                let mut t22 = *heman_image_texel(gray, xp1, yp1);
                let mut gx = (t00 as f64 + 2.0f64 * t01 as f64 + t02 as f64
                    - t20 as f64
                    - 2.0f64 * t21 as f64
                    - t22 as f64) as libc::c_float;
                let mut gy = (t00 as f64 + 2.0f64 * t10 as f64 + t20 as f64
                    - t02 as f64
                    - 2.0f64 * t12 as f64
                    - t22 as f64) as libc::c_float;
                let mut is_edge = ((gx * gx + gy * gy) as f64 > 1e-5f64) as i32 as libc::c_float;
                let fresh12 = dst;
                dst = dst.offset(1);
                let fresh13 = src;
                src = src.offset(1);
                kmVec3Lerp(fresh12, fresh13, &mut edge_rgb, is_edge);
                x += 1;
            }
            y += 1;
        }
        heman_image_destroy(gray);
        return result;
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_warp_core(
    mut img: *mut heman_image,
    mut secondary: *mut heman_image,
    mut seed: i32,
    mut octaves: i32,
) -> *mut heman_image {
    unsafe {
        let mut ctx = 0 as *mut osn_context;
        open_simplex_noise(seed as i64, &mut ctx);
        let mut width = (*img).width;
        let mut height = (*img).height;
        let mut nbands = (*img).nbands;
        let mut result = heman_image_create(width, height, nbands);
        let mut result2 = if !secondary.is_null() {
            heman_image_create(width, height, (*secondary).nbands)
        } else {
            0 as *mut heman_image
        };
        let mut invw = (1.0f64 / width as f64) as libc::c_float;
        let mut invh = (1.0f64 / height as f64) as libc::c_float;
        let mut inv = if invw > invh { invh } else { invw };
        let mut aspect = width as libc::c_float / height as libc::c_float;
        let mut gain = 0.6f64 as libc::c_float;
        let mut lacunarity = 2.0f64 as libc::c_float;
        let mut initial_amplitude = 0.05f64 as libc::c_float;
        let mut initial_frequency = 8.0f64 as libc::c_float;
        let mut y: i32 = 0;
        y = 0;
        while y < height {
            let mut dst = ((*result).data).offset((y * width * nbands) as isize);
            let mut x = 0;
            while x < width {
                let mut a = initial_amplitude;
                let mut f = initial_frequency;
                let mut src = 0 as *mut libc::c_float;
                if nbands == 4 {
                    src = heman_image_texel(img, x, y);
                    let mut elev = 1 as libc::c_float - *src.offset(3 as isize);
                    a = (a as f64 * pow(elev as f64, 4 as f64)) as libc::c_float;
                }
                let mut s = x as libc::c_float * inv;
                let mut t = y as libc::c_float * inv;
                let mut u = x as libc::c_float * invw;
                let mut v = y as libc::c_float * invh;
                let mut i = 0;
                while i < octaves {
                    u = (u as f64
                        + a as f64 * open_simplex_noise2(ctx, (s * f) as f64, (t * f) as f64))
                        as libc::c_float;
                    v = (v as f64
                        + aspect as f64
                            * (a as f64
                                * open_simplex_noise2(
                                    ctx,
                                    (s * f) as f64 + 0.5f64,
                                    (t * f) as f64,
                                ))) as libc::c_float;
                    a *= gain;
                    f *= lacunarity;
                    i += 1;
                }
                let mut i_0 = (if 0 as libc::c_float
                    > (if (width - 1i32) as libc::c_float > u * width as libc::c_float {
                        u * width as libc::c_float
                    } else {
                        (width - 1i32) as libc::c_float
                    }) {
                    0 as libc::c_float
                } else if (width - 1i32) as libc::c_float > u * width as libc::c_float {
                    u * width as libc::c_float
                } else {
                    (width - 1i32) as libc::c_float
                }) as i32;
                let mut j = (if 0 as libc::c_float
                    > (if (height - 1i32) as libc::c_float > v * height as libc::c_float {
                        v * height as libc::c_float
                    } else {
                        (height - 1i32) as libc::c_float
                    }) {
                    0 as libc::c_float
                } else if (height - 1i32) as libc::c_float > v * height as libc::c_float {
                    v * height as libc::c_float
                } else {
                    (height - 1i32) as libc::c_float
                }) as i32;
                src = heman_image_texel(img, i_0, j);
                let mut n = 0;
                while n < nbands {
                    let fresh14 = src;
                    src = src.offset(1);
                    let fresh15 = dst;
                    dst = dst.offset(1);
                    *fresh15 = *fresh14;
                    n += 1;
                }
                if !secondary.is_null() {
                    src = heman_image_texel(secondary, x, y);
                    let mut dst2 = heman_image_texel(result2, i_0, j);
                    let mut n_0 = 0;
                    while n_0 < (*secondary).nbands {
                        let fresh16 = src;
                        src = src.offset(1);
                        let fresh17 = dst2;
                        dst2 = dst2.offset(1);
                        *fresh17 = *fresh16;
                        n_0 += 1;
                    }
                }
                x += 1;
            }
            y += 1;
        }
        open_simplex_noise_free(ctx);
        if !secondary.is_null() {
            free((*secondary).data as *mut libc::c_void);
            let ref mut fresh18 = (*secondary).data;
            *fresh18 = (*result2).data;
            free(result2 as *mut libc::c_void);
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_warp_points(
    mut img: *mut heman_image,
    mut seed: i32,
    mut octaves: i32,
    mut pts: *mut heman_points,
) -> *mut heman_image {
    unsafe {
        let mut width = (*img).width;
        let mut height = (*img).height;
        let mut mapping = heman_distance_identity_cpcf(width, height);
        let mut retval = heman_ops_warp_core(img, mapping, seed, octaves);
        let mut src = (*pts).data;
        let mut k = 0;
        while k < (*pts).width {
            let mut x = *src.offset(0 as isize);
            let mut y = *src.offset(1 as isize);
            let mut i = (x * (*mapping).width as libc::c_float) as i32;
            let mut j = (y * (*mapping).height as libc::c_float) as i32;
            if !(i < 0 || i >= (*mapping).width || j < 0 || j >= (*mapping).height) {
                let mut texel = heman_image_texel(mapping, i, j);
                *src.offset(0 as isize) =
                    *texel.offset(0 as isize) / (*mapping).width as libc::c_float;
                *src.offset(1 as isize) =
                    *texel.offset(1 as isize) / (*mapping).height as libc::c_float;
            }
            k += 1;
            src = src.offset((*pts).nbands as isize);
        }
        heman_image_destroy(mapping);
        return retval;
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_warp(
    mut img: *mut heman_image,
    mut seed: i32,
    mut octaves: i32,
) -> *mut heman_image {
    unsafe {
        return heman_ops_warp_core(img, 0 as *mut heman_image, seed, octaves);
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_extract_mask(
    mut source: *mut heman_image,
    mut color: u32,
    mut invert: i32,
) -> *mut heman_image {
    unsafe {
        if (*source).nbands == 3 {
        } else {
            __assert_fail(
                b"source->nbands == 3\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                330,
                (*::std::mem::transmute::<&[u8; 69], &[i8; 69]>(
                    b"heman_image *heman_ops_extract_mask(heman_image *, heman_color, int)\0",
                ))
                .as_ptr(),
            );
        }
        let mut inv = 1.0f32 / 255.0f32;
        let mut r = (color >> 16i32) as libc::c_float * inv;
        let mut g = (color >> 8 & 0xffu32) as libc::c_float * inv;
        let mut b = (color & 0xffu32) as libc::c_float * inv;
        let mut height = (*source).height;
        let mut width = (*source).width;
        let mut result = heman_image_create(width, height, 1);
        let mut y: i32 = 0;
        y = 0;
        while y < height {
            let mut dst = ((*result).data).offset((y * width) as isize);
            let mut src = ((*source).data).offset((y * width * 3i32) as isize);
            let mut x = 0;
            while x < width {
                let mut val = (*src.offset(0 as isize) == r
                    && *src.offset(1 as isize) == g
                    && *src.offset(2 as isize) == b) as i32
                    as libc::c_float;
                if invert == 0 {
                    val = 1 as libc::c_float - val;
                }
                let fresh19 = dst;
                dst = dst.offset(1);
                *fresh19 = val;
                x += 1;
                src = src.offset(3 as isize);
            }
            y += 1;
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_replace_color(
    mut source: *mut heman_image,
    mut color: u32,
    mut texture: *mut heman_image,
) -> *mut heman_image {
    unsafe {
        if (*source).nbands == 3 {
        } else {
            __assert_fail (b"source->nbands == 3\0" as * const u8 as * const i8, b"../src/ops.c\0" as * const u8 as * const i8, 359, (* :: std :: mem :: transmute :: < & [u8; 80], & [i8; 80], > (
              b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",)).as_ptr (),);
        }
        if (*texture).nbands == 3 {
        } else {
            __assert_fail (b"texture->nbands == 3\0" as * const u8 as * const i8, b"../src/ops.c\0" as * const u8 as * const i8, 360, (* :: std :: mem :: transmute :: < & [u8; 80], & [i8; 80], > (
              b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",)).as_ptr (),);
        }
        let mut height = (*source).height;
        let mut width = (*source).width;
        if (*texture).width == width {
        } else {
            __assert_fail (b"texture->width == width\0" as * const u8 as * const i8, b"../src/ops.c\0" as * const u8 as * const i8, 363, (* :: std :: mem :: transmute :: < & [u8; 80], & [i8; 80], > (
              b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",)).as_ptr (),);
        }
        if (*texture).height == height {
        } else {
            __assert_fail (b"texture->height == height\0" as * const u8 as * const i8, b"../src/ops.c\0" as * const u8 as * const i8, 364, (* :: std :: mem :: transmute :: < & [u8; 80], & [i8; 80], > (
              b"heman_image *heman_ops_replace_color(heman_image *, heman_color, heman_image *)\0",)).as_ptr (),);
        }
        let mut inv = 1.0f32 / 255.0f32;
        let mut r = (color >> 16i32) as libc::c_float * inv;
        let mut g = (color >> 8 & 0xffu32) as libc::c_float * inv;
        let mut b = (color & 0xffu32) as libc::c_float * inv;
        let mut result = heman_image_create(width, height, 3);
        let mut y: i32 = 0;
        y = 0;
        while y < height {
            let mut dst = ((*result).data).offset((y * width * 3i32) as isize);
            let mut src = ((*source).data).offset((y * width * 3i32) as isize);
            let mut tex = ((*texture).data).offset((y * width * 3i32) as isize);
            let mut x = 0;
            while x < width {
                if *src.offset(0 as isize) == r
                    && *src.offset(1 as isize) == g
                    && *src.offset(2 as isize) == b
                {
                    *dst.offset(0 as isize) = *tex.offset(0 as isize);
                    *dst.offset(1 as isize) = *tex.offset(1 as isize);
                    *dst.offset(2 as isize) = *tex.offset(2 as isize);
                } else {
                    *dst.offset(0 as isize) = *src.offset(0 as isize);
                    *dst.offset(1 as isize) = *src.offset(1 as isize);
                    *dst.offset(2 as isize) = *src.offset(2 as isize);
                }
                x += 1;
                src = src.offset(3 as isize);
                dst = dst.offset(3 as isize);
                tex = tex.offset(3 as isize);
            }
            y += 1;
        }
        return result;
    }
}

extern "C" fn _match(
    mut mask: *mut heman_image,
    mut mask_color: u32,
    mut invert_mask: i32,
    mut pixel_index: i32,
) -> i32 {
    unsafe {
        let mut mcolor = ((*mask).data).offset((pixel_index * 3i32) as isize);
        let mut r1 = (*mcolor.offset(0 as isize) * 255 as libc::c_float) as u8;
        let mut g1 = (*mcolor.offset(1 as isize) * 255 as libc::c_float) as u8;
        let mut b1 = (*mcolor.offset(2 as isize) * 255 as libc::c_float) as u8;
        let mut r2 = (mask_color >> 16i32) as u8;
        let mut g2 = (mask_color >> 8 & 0xffu32) as u8;
        let mut b2 = (mask_color & 0xffu32) as u8;
        let mut retval =
            (r1 as i32 == r2 as i32 && g1 as i32 == g2 as i32 && b1 as i32 == b2 as i32) as i32;
        return if invert_mask != 0 { 1 - retval } else { retval };
    }
}

extern "C" fn qselect(mut v: *mut libc::c_float, mut len: i32, mut k: i32) -> libc::c_float {
    unsafe {
        let mut i: i32 = 0;
        let mut st: i32 = 0;
        i = 0;
        st = i;
        while i < len - 1 {
            if !(*v.offset(i as isize) > *v.offset((len - 1i32) as isize)) {
                let mut f = *v.offset(i as isize);
                *v.offset(i as isize) = *v.offset(st as isize);
                *v.offset(st as isize) = f;
                st += 1;
            }
            i += 1;
        }
        let mut __0 = *v.offset((len - 1i32) as isize);
        *v.offset((len - 1i32) as isize) = *v.offset(st as isize);
        *v.offset(st as isize) = __0;
        return if k == st {
            *v.offset(st as isize)
        } else if st > k {
            qselect(v, st, k)
        } else {
            qselect(v.offset(st as isize), len - st, k - st)
        };
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_percentiles(
    mut hmap: *mut heman_image,
    mut nsteps: i32,
    mut mask: *mut heman_image,
    mut mask_color: u32,
    mut invert_mask: i32,
    mut offset: libc::c_float,
) -> *mut heman_image {
    unsafe {
        if (*hmap).nbands == 1 {
        } else {
            __assert_fail (b"hmap->nbands == 1\0" as * const u8 as * const i8, b"../src/ops.c\0" as * const u8 as * const i8, 427, (* :: std :: mem :: transmute :: < & [u8; 95], & [i8; 95], > (
              b"heman_image *heman_ops_percentiles(heman_image *, int, heman_image *, heman_color, int, float)\0",)).as_ptr (),);
        }
        if mask.is_null() || (*mask).nbands == 3 {
        } else {
            __assert_fail (b"!mask || mask->nbands == 3\0" as * const u8 as * const i8, b"../src/ops.c\0" as * const u8 as * const i8, 428, (* :: std :: mem :: transmute :: < & [u8; 95], & [i8; 95], > (
              b"heman_image *heman_ops_percentiles(heman_image *, int, heman_image *, heman_color, int, float)\0",)).as_ptr (),);
        }
        let mut size = (*hmap).height * (*hmap).width;
        let mut src = (*hmap).data;
        let mut minv = 1000 as libc::c_float;
        let mut maxv = -1000i32 as libc::c_float;
        let mut npixels = 0;
        let mut i = 0;
        while i < size {
            if mask.is_null() || _match(mask, mask_color, invert_mask, i) != 0 {
                minv = if minv > *src.offset(i as isize) {
                    *src.offset(i as isize)
                } else {
                    minv
                };
                maxv = if maxv > *src.offset(i as isize) {
                    maxv
                } else {
                    *src.offset(i as isize)
                };
                npixels += 1;
            }
            i += 1;
        }
        let mut vals =
            malloc((::std::mem::size_of::<libc::c_float>() as u64).wrapping_mul(npixels as u64))
                as *mut libc::c_float;
        npixels = 0;
        let mut i_0 = 0;
        while i_0 < size {
            if mask.is_null() || _match(mask, mask_color, invert_mask, i_0) != 0 {
                let fresh20 = npixels;
                npixels = npixels + 1;
                *vals.offset(fresh20 as isize) = *src.offset(i_0 as isize);
            }
            i_0 += 1;
        }
        let mut percentiles =
            malloc((::std::mem::size_of::<libc::c_float>() as u64).wrapping_mul(nsteps as u64))
                as *mut libc::c_float;
        let mut tier = 0;
        while tier < nsteps {
            let mut height = qselect(vals, npixels, tier * npixels / nsteps);
            *percentiles.offset(tier as isize) = height;
            tier += 1;
        }
        free(vals as *mut libc::c_void);
        let mut i_1 = 0;
        while i_1 < size {
            let mut e = *src;
            if mask.is_null() || _match(mask, mask_color, invert_mask, i_1) != 0 {
                let mut tier_0 = nsteps - 1;
                while tier_0 >= 0 {
                    if e > *percentiles.offset(tier_0 as isize) {
                        e = *percentiles.offset(tier_0 as isize);
                        break;
                    } else {
                        tier_0 -= 1;
                    }
                }
            }
            let fresh21 = src;
            src = src.offset(1);
            *fresh21 = e + offset;
            i_1 += 1;
        }
        free(percentiles as *mut libc::c_void);
        return hmap;
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_stairstep(
    mut hmap: *mut heman_image,
    mut nsteps: i32,
    mut mask: *mut heman_image,
    mut mask_color: u32,
    mut invert_mask: i32,
    mut offset: libc::c_float,
) -> *mut heman_image {
    unsafe {
        if (*hmap).nbands == 1 {
        } else {
            __assert_fail (b"hmap->nbands == 1\0" as * const u8 as * const i8, b"../src/ops.c\0" as * const u8 as * const i8, 477, (* :: std :: mem :: transmute :: < & [u8; 93], & [i8; 93], > (
              b"heman_image *heman_ops_stairstep(heman_image *, int, heman_image *, heman_color, int, float)\0",)).as_ptr (),);
        }
        if mask.is_null() || (*mask).nbands == 3 {
        } else {
            __assert_fail (b"!mask || mask->nbands == 3\0" as * const u8 as * const i8, b"../src/ops.c\0" as * const u8 as * const i8, 478, (* :: std :: mem :: transmute :: < & [u8; 93], & [i8; 93], > (
              b"heman_image *heman_ops_stairstep(heman_image *, int, heman_image *, heman_color, int, float)\0",)).as_ptr (),);
        }
        let mut size = (*hmap).height * (*hmap).width;
        let mut src = (*hmap).data;
        let mut minv = 1000 as libc::c_float;
        let mut maxv = -1000i32 as libc::c_float;
        let mut i = 0;
        while i < size {
            if mask.is_null() || _match(mask, mask_color, invert_mask, i) != 0 {
                minv = if minv > *src.offset(i as isize) {
                    *src.offset(i as isize)
                } else {
                    minv
                };
                maxv = if maxv > *src.offset(i as isize) {
                    maxv
                } else {
                    *src.offset(i as isize)
                };
            }
            i += 1;
        }
        let mut range = maxv - minv;
        let mut i_0 = 0;
        while i_0 < size {
            let mut e = *src;
            if mask.is_null() || _match(mask, mask_color, invert_mask, i_0) != 0 {
                e = e - minv;
                e /= range;
                e = (floor((e * nsteps as libc::c_float) as f64) / nsteps as f64) as libc::c_float;
                e = e * range + minv;
            }
            let fresh22 = src;
            src = src.offset(1);
            *fresh22 = e + offset;
            i_0 += 1;
        }
        return hmap;
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_merge_political(
    mut hmap: *mut heman_image,
    mut cmap: *mut heman_image,
    mut ocean: u32,
) -> *mut heman_image {
    unsafe {
        if (*hmap).nbands == 1 {
        } else {
            __assert_fail (b"hmap->nbands == 1\0" as * const u8 as * const i8, b"../src/ops.c\0" as * const u8 as * const i8, 506, (* :: std :: mem :: transmute :: < & [u8; 82], & [i8; 82], > (
              b"heman_image *heman_ops_merge_political(heman_image *, heman_image *, heman_color)\0",)).as_ptr (),);
        }
        if (*cmap).nbands == 3 {
        } else {
            __assert_fail (b"cmap->nbands == 3\0" as * const u8 as * const i8, b"../src/ops.c\0" as * const u8 as * const i8, 507, (* :: std :: mem :: transmute :: < & [u8; 82], & [i8; 82], > (
              b"heman_image *heman_ops_merge_political(heman_image *, heman_image *, heman_color)\0",)).as_ptr (),);
        }
        let mut result = heman_image_create((*hmap).width, (*hmap).height, 4);
        let mut pheight = (*hmap).data;
        let mut pcolour = (*cmap).data;
        let mut pmerged = (*result).data;
        let mut inv = 1.0f32 / 255.0f32;
        let mut oceanr = (ocean >> 16i32) as libc::c_float * inv;
        let mut oceang = (ocean >> 8 & 0xffu32) as libc::c_float * inv;
        let mut oceanb = (ocean & 0xffu32) as libc::c_float * inv;
        let mut size = (*hmap).height * (*hmap).width;
        let mut minh = 1000 as libc::c_float;
        let mut maxh = -1000i32 as libc::c_float;
        let mut i = 0;
        while i < size {
            minh = if minh > *pheight.offset(i as isize) {
                *pheight.offset(i as isize)
            } else {
                minh
            };
            maxh = if maxh > *pheight.offset(i as isize) {
                *pheight.offset(i as isize)
            } else {
                maxh
            };
            i += 1;
        }
        let mut i_0 = 0;
        while i_0 < size {
            let fresh23 = pheight;
            pheight = pheight.offset(1);
            let mut h = *fresh23;
            if h < 0 as libc::c_float {
                let fresh24 = pmerged;
                pmerged = pmerged.offset(1);
                *fresh24 = oceanr;
                let fresh25 = pmerged;
                pmerged = pmerged.offset(1);
                *fresh25 = oceang;
                let fresh26 = pmerged;
                pmerged = pmerged.offset(1);
                *fresh26 = oceanb;
                pcolour = pcolour.offset(3 as isize);
            } else {
                let fresh27 = pcolour;
                pcolour = pcolour.offset(1);
                let fresh28 = pmerged;
                pmerged = pmerged.offset(1);
                *fresh28 = *fresh27;
                let fresh29 = pcolour;
                pcolour = pcolour.offset(1);
                let fresh30 = pmerged;
                pmerged = pmerged.offset(1);
                *fresh30 = *fresh29;
                let fresh31 = pcolour;
                pcolour = pcolour.offset(1);
                let fresh32 = pmerged;
                pmerged = pmerged.offset(1);
                *fresh32 = *fresh31;
            }
            let fresh33 = pmerged;
            pmerged = pmerged.offset(1);
            *fresh33 = (h - minh) / (maxh - minh);
            i_0 += 1;
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn heman_ops_emboss(mut img: *mut heman_image, mut mode: i32) -> *mut heman_image {
    unsafe {
        let mut seed = 1;
        let mut octaves = 4;
        let mut ctx = 0 as *mut osn_context;
        open_simplex_noise(seed as i64, &mut ctx);
        let mut width = (*img).width;
        let mut height = (*img).height;
        if (*img).nbands == 1 {
        } else {
            __assert_fail(
                b"img->nbands == 1\0" as *const u8 as *const i8,
                b"../src/ops.c\0" as *const u8 as *const i8,
                549,
                (*::std::mem::transmute::<&[u8; 50], &[i8; 50]>(
                    b"heman_image *heman_ops_emboss(heman_image *, int)\0",
                ))
                .as_ptr(),
            );
        }
        let mut result = heman_image_create(width, height, 1);
        let mut invw = (1.0f64 / width as f64) as libc::c_float;
        let mut invh = (1.0f64 / height as f64) as libc::c_float;
        let mut inv = if invw > invh { invh } else { invw };
        let mut gain = 0.6f64 as libc::c_float;
        let mut lacunarity = 2.0f64 as libc::c_float;
        let mut land_amplitude = 0.0005f64 as libc::c_float;
        let mut land_frequency = 256.0f64 as libc::c_float;
        let mut ocean_amplitude = 0.5f64 as libc::c_float;
        let mut ocean_frequency = 1.0f64 as libc::c_float;
        let mut y: i32 = 0;
        y = 0;
        while y < height {
            let mut dst = ((*result).data).offset((y * width) as isize);
            let mut x = 0;
            while x < width {
                let mut z = *heman_image_texel(img, x, y);
                if z > 0 as libc::c_float && mode == 1 {
                    let mut s = x as libc::c_float * inv;
                    let mut t = y as libc::c_float * inv;
                    let mut a = land_amplitude;
                    let mut f = land_frequency;
                    let mut i = 0;
                    while i < octaves {
                        z = (z as f64
                            + a as f64 * open_simplex_noise2(ctx, (s * f) as f64, (t * f) as f64))
                            as libc::c_float;
                        a *= gain;
                        f *= lacunarity;
                        i += 1;
                    }
                } else if z <= 0 as libc::c_float && mode == -1 {
                    z = (if z as f64 > -0.1f64 {
                        z as f64
                    } else {
                        -0.1f64
                    }) as libc::c_float;
                    let mut soften = fabsf(z);
                    let mut s_0 = x as libc::c_float * inv;
                    let mut t_0 = y as libc::c_float * inv;
                    let mut a_0 = ocean_amplitude;
                    let mut f_0 = ocean_frequency;
                    let mut i_0 = 0;
                    while i_0 < octaves {
                        z = (z as f64
                            + soften as f64
                                * (a_0 as f64
                                    * open_simplex_noise2(
                                        ctx,
                                        (s_0 * f_0) as f64,
                                        (t_0 * f_0) as f64,
                                    ))) as libc::c_float;
                        a_0 *= gain;
                        f_0 *= lacunarity;
                        i_0 += 1;
                    }
                }
                let fresh34 = dst;
                dst = dst.offset(1);
                *fresh34 = z;
                x += 1;
            }
            y += 1;
        }
        open_simplex_noise_free(ctx);
        return result;
    }
}
