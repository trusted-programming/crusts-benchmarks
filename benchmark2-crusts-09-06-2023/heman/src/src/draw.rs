use libc;
extern "C" {
    fn heman_image_create(width: i32, height: i32, nbands: i32) -> *mut heman_image;
    fn heman_image_texel(_: *mut heman_image, x: i32, y: i32) -> *mut libc::c_float;
    fn heman_image_clear(_: *mut heman_image, value: libc::c_float);
    fn heman_points_destroy(_: *mut heman_points);
    fn generate_gaussian_splat(target: *mut libc::c_float, fwidth: i32);
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn free(_: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn heman_internal_draw_seeds(target: *mut heman_image, pts: *mut heman_points, filterd: i32);
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
#[no_mangle]
pub extern "C" fn heman_draw_points(
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut val: libc::c_float,
) {
    unsafe {
        let mut src = (*pts).data;
        let mut k = 0;
        while k < (*pts).width {
            let mut x = *src.offset(0 as isize);
            let mut y = *src.offset(1 as isize);
            src = src.offset((*pts).nbands as isize);
            let mut i = (x * (*target).width as libc::c_float) as i32;
            let mut j = (y * (*target).height as libc::c_float) as i32;
            if !(i < 0 || i >= (*target).width || j < 0 || j >= (*target).height) {
                let mut texel = heman_image_texel(target, i, j);
                let mut c = 0;
                while c < (*target).nbands {
                    let fresh0 = texel;
                    texel = texel.offset(1);
                    *fresh0 = val;
                    c += 1;
                }
            }
            k += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn heman_draw_colored_points(
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut colors: *const u32,
) {
    unsafe {
        if (*target).nbands == 3 || (*target).nbands == 4 {
        } else {
            __assert_fail (b"target->nbands == 3 || target->nbands == 4\0" as * const u8 as * const i8, b"../src/draw.c\0" as * const u8 as * const i8, 27, (* :: std :: mem :: transmute :: < & [u8; 83], & [i8; 83], > (
              b"void heman_draw_colored_points(heman_image *, heman_points *, const heman_color *)\0",)).as_ptr (),);
        }
        let mut src = (*pts).data;
        let mut inv = 1.0f32 / 255.0f32;
        let mut k = 0;
        while k < (*pts).width {
            let mut x = *src.offset(0 as isize);
            let mut y = *src.offset(1 as isize);
            src = src.offset((*pts).nbands as isize);
            let mut i = (x * (*target).width as libc::c_float) as i32;
            let mut j = (y * (*target).height as libc::c_float) as i32;
            if !(i < 0 || i >= (*target).width || j < 0 || j >= (*target).height) {
                let mut texel = heman_image_texel(target, i, j);
                let mut rgb = *colors.offset(k as isize);
                let fresh1 = texel;
                texel = texel.offset(1);
                *fresh1 = (rgb >> 16 & 0xffu32) as libc::c_float * inv;
                let fresh2 = texel;
                texel = texel.offset(1);
                *fresh2 = (rgb >> 8 & 0xffu32) as libc::c_float * inv;
                let fresh3 = texel;
                texel = texel.offset(1);
                *fresh3 = (rgb & 0xffu32) as libc::c_float * inv;
                if (*target).nbands == 4 {
                    *texel = (rgb >> 24i32) as libc::c_float * inv;
                }
            }
            k += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn heman_draw_colored_circles(
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut radius: i32,
    mut colors: *const u32,
) {
    unsafe {
        let mut fwidth = radius * 2 + 1;
        let mut radius2 = radius * radius;
        let mut src = (*pts).data;
        let mut inv = 1.0f32 / 255.0f32;
        let mut w = (*target).width;
        let mut h = (*target).height;
        let mut k = 0;
        while k < (*pts).width {
            let mut x = *src.offset(0 as isize);
            let mut y = *src.offset(1 as isize);
            src = src.offset((*pts).nbands as isize);
            let mut ii = (x * w as libc::c_float - radius as libc::c_float) as i32;
            let mut jj = (y * h as libc::c_float - radius as libc::c_float) as i32;
            let mut kj = 0;
            while kj < fwidth {
                let mut ki = 0;
                while ki < fwidth {
                    let mut i = ii + ki;
                    let mut j = jj + kj;
                    let mut r2 = ((i as libc::c_float - x * w as libc::c_float)
                        * (i as libc::c_float - x * w as libc::c_float)
                        + (j as libc::c_float - y * h as libc::c_float)
                            * (j as libc::c_float - y * h as libc::c_float))
                        as i32;
                    if !(r2 > radius2) {
                        let mut texel = heman_image_texel(target, i, j);
                        let mut rgb = *colors.offset(k as isize);
                        let fresh4 = texel;
                        texel = texel.offset(1);
                        *fresh4 = (rgb >> 16i32) as libc::c_float * inv;
                        let fresh5 = texel;
                        texel = texel.offset(1);
                        *fresh5 = (rgb >> 8 & 0xffu32) as libc::c_float * inv;
                        *texel = (rgb & 0xffu32) as libc::c_float * inv;
                    }
                    ki += 1;
                }
                kj += 1;
            }
            k += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn heman_draw_splats(
    mut target: *mut heman_image,
    mut pts: *mut heman_points,
    mut radius: i32,
    mut blend_mode: i32,
) {
    unsafe {
        let mut fwidth = radius * 2 + 1;
        let mut gaussian_splat = malloc(
            ((fwidth * fwidth) as u64).wrapping_mul(::std::mem::size_of::<libc::c_float>() as u64),
        ) as *mut libc::c_float;
        generate_gaussian_splat(gaussian_splat, fwidth);
        let mut src = (*pts).data;
        let mut w = (*target).width;
        let mut h = (*target).height;
        let mut i = 0;
        while i < (*pts).width {
            let fresh6 = src;
            src = src.offset(1);
            let mut x = *fresh6;
            let fresh7 = src;
            src = src.offset(1);
            let mut y = *fresh7;
            let mut ii = (x * w as libc::c_float - radius as libc::c_float) as i32;
            let mut jj = (y * h as libc::c_float - radius as libc::c_float) as i32;
            let mut kj = 0;
            while kj < fwidth {
                let mut ki = 0;
                while ki < fwidth {
                    let mut i_0 = ii + ki;
                    let mut j = jj + kj;
                    if !(i_0 < 0 || i_0 >= w || j < 0 || j >= h) {
                        let mut texel = heman_image_texel(target, i_0, j);
                        let mut c = 0;
                        while c < (*target).nbands {
                            let fresh8 = texel;
                            texel = texel.offset(1);
                            *fresh8 += *gaussian_splat.offset((kj * fwidth + ki) as isize);
                            c += 1;
                        }
                    }
                    ki += 1;
                }
                kj += 1;
            }
            i += 1;
        }
        free(gaussian_splat as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn heman_draw_contour_from_points(
    mut target: *mut heman_image,
    mut coords: *mut heman_points,
    mut rgb: u32,
    mut mind: libc::c_float,
    mut maxd: libc::c_float,
    mut filterd: i32,
) {
    unsafe {
        if (*target).nbands == 3 || (*target).nbands == 4 {
        } else {
            __assert_fail (b"target->nbands == 3 || target->nbands == 4\0" as * const u8 as * const i8, b"../src/draw.c\0" as * const u8 as * const i8, 119, (* :: std :: mem :: transmute :: < & [u8; 99], & [i8; 99], > (
              b"void heman_draw_contour_from_points(heman_image *, heman_points *, heman_color, float, float, int)\0",)).as_ptr (),);
        }
        let mut width = (*target).width;
        let mut height = (*target).height;
        let mut seed = heman_image_create(width, height, 1);
        heman_image_clear(seed, 0 as libc::c_float);
        heman_internal_draw_seeds(seed, coords, filterd);
        let mut inv = 1.0f32 / 255.0f32;
        let mut r = (rgb >> 16 & 0xffu32) as libc::c_float * inv;
        let mut g = (rgb >> 8 & 0xffu32) as libc::c_float * inv;
        let mut b = (rgb & 0xffu32) as libc::c_float * inv;
        let mut a = 1 as libc::c_float;
        if (*target).nbands == 4 {
            a = (rgb >> 24i32) as libc::c_float * inv;
        }
        let mut y: i32 = 0;
        y = 0;
        while y < height {
            let mut dst = ((*target).data).offset((y * width * (*target).nbands) as isize);
            let mut x = 0;
            while x < width {
                let mut dist = *heman_image_texel(seed, x, y);
                if dist > mind && dist < maxd {
                    *dst.offset(0 as isize) = r;
                    *dst.offset(1 as isize) = g;
                    *dst.offset(2 as isize) = b;
                    if (*target).nbands == 4 {
                        *dst.offset(3 as isize) = a;
                    }
                }
                dst = dst.offset((*target).nbands as isize);
                x += 1;
            }
            y += 1;
        }
        heman_points_destroy(seed);
    }
}
