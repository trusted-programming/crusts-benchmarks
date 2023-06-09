use libc;
extern "C" {
    static mut _gamma: libc::c_float;
    fn heman_image_texel(_: *mut heman_image, x: i32, y: i32) -> *mut libc::c_float;
    fn heman_image_destroy(_: *mut heman_image);
    fn heman_image_create(width: i32, height: i32, nbands: i32) -> *mut heman_image;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: i32) -> i32;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn atan(_: f64) -> f64;
    fn pow(_: f64, _: f64) -> f64;
    fn kmMax(lhs: libc::c_float, rhs: libc::c_float) -> libc::c_float;
    fn kmClamp(x: libc::c_float, min: libc::c_float, max: libc::c_float) -> libc::c_float;
    fn kmVec3Lerp(
        pOut: *mut kmVec3,
        pV1: *const kmVec3,
        pV2: *const kmVec3,
        t: libc::c_float,
    ) -> *mut kmVec3;
    fn kmVec3Cross(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Length(pIn: *const kmVec3) -> libc::c_float;
    fn kmVec3Subtract(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Normalize(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Dot(pV1: *const kmVec3, pV2: *const kmVec3) -> libc::c_float;
    static KM_VEC3_POS_Z: kmVec3;
    fn kmVec3Scale(pOut: *mut kmVec3, pIn: *const kmVec3, s: libc::c_float) -> *mut kmVec3;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
}
static mut _occlusion_scale: libc::c_float = 1.0f32;
#[no_mangle]
pub extern "C" fn heman_lighting_set_occlusion_scale(mut s: libc::c_float) {
    unsafe {
        _occlusion_scale = s;
    }
}

#[no_mangle]
pub extern "C" fn heman_lighting_compute_normals(
    mut heightmap: *mut heman_image,
) -> *mut heman_image {
    unsafe {
        if (*heightmap).nbands == 1 {
        } else {
            __assert_fail(
                b"heightmap->nbands == 1\0" as *const u8 as *const i8,
                b"../src/lighting.c\0" as *const u8 as *const i8,
                16,
                (*::std::mem::transmute::<&[u8; 59], &[i8; 59]>(
                    b"heman_image *heman_lighting_compute_normals(heman_image *)\0",
                ))
                .as_ptr(),
            );
        }
        let mut width = (*heightmap).width;
        let mut height = (*heightmap).height;
        let mut result = heman_image_create(width, height, 3);
        let mut invh = 1.0f32 / height as libc::c_float;
        let mut invw = 1.0f32 / width as libc::c_float;
        let mut maxx = width - 1;
        let mut maxy = height - 1;
        let mut normals = (*result).data as *mut kmVec3;
        let mut y: i32 = 0;
        y = 0;
        while y < height {
            let mut v = y as libc::c_float * invh;
            let mut y1 = if y + 1 > maxy { maxy } else { y + 1 };
            let mut p = kmVec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            };
            let mut px = kmVec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            };
            let mut py = kmVec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            };
            let mut n = normals.offset((y * width) as isize);
            let mut x = 0;
            while x < width {
                let mut u = x as libc::c_float * invw;
                let mut x1 = if x + 1 > maxx { maxx } else { x + 1 };
                p.x = u;
                p.y = v;
                p.z = *heman_image_texel(heightmap, x, y);
                px.x = u + invw;
                px.y = v;
                px.z = *heman_image_texel(heightmap, x1, y);
                py.x = u;
                py.y = v + invh;
                py.z = *heman_image_texel(heightmap, x, y1);
                kmVec3Subtract(&mut px, &mut px, &mut p);
                kmVec3Subtract(&mut py, &mut py, &mut p);
                kmVec3Cross(n, &mut px, &mut py);
                kmVec3Normalize(n, n);
                (*n).y *= -1i32 as libc::c_float;
                x += 1;
                n = n.offset(1);
            }
            y += 1;
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn heman_lighting_apply(
    mut heightmap: *mut heman_image,
    mut albedo: *mut heman_image,
    mut occlusion: libc::c_float,
    mut diffuse: libc::c_float,
    mut diffuse_softening: libc::c_float,
    mut light_position: *const libc::c_float,
) -> *mut heman_image {
    unsafe {
        if (*heightmap).nbands == 1 {
        } else {
            __assert_fail (b"heightmap->nbands == 1\0" as * const u8 as * const i8, b"../src/lighting.c\0" as * const u8 as * const i8, 61, (* :: std :: mem :: transmute :: < & [u8; 100], & [i8; 100], > (
              b"heman_image *heman_lighting_apply(heman_image *, heman_image *, float, float, float, const float *)\0",)).as_ptr (),);
        }
        let mut width = (*heightmap).width;
        let mut height = (*heightmap).height;
        let mut final_0 = heman_image_create(width, height, 3);
        let mut normals = heman_lighting_compute_normals(heightmap);
        let mut occ = heman_lighting_compute_occlusion(heightmap);
        if !albedo.is_null() {
            if (*albedo).nbands == 3 {
            } else {
                __assert_fail (b"albedo->nbands == 3\0" as * const u8 as * const i8, b"../src/lighting.c\0" as * const u8 as * const i8, 69, (* :: std :: mem :: transmute :: < & [u8; 100], & [i8; 100], > (
                  b"heman_image *heman_lighting_apply(heman_image *, heman_image *, float, float, float, const float *)\0",)).as_ptr (),);
            }
            if (*albedo).width == width {
            } else {
                __assert_fail (b"albedo->width == width\0" as * const u8 as * const i8, b"../src/lighting.c\0" as * const u8 as * const i8, 70, (* :: std :: mem :: transmute :: < & [u8; 100], & [i8; 100], > (
                  b"heman_image *heman_lighting_apply(heman_image *, heman_image *, float, float, float, const float *)\0",)).as_ptr (),);
            }
            if (*albedo).height == height {
            } else {
                __assert_fail (b"albedo->height == height\0" as * const u8 as * const i8, b"../src/lighting.c\0" as * const u8 as * const i8, 71, (* :: std :: mem :: transmute :: < & [u8; 100], & [i8; 100], > (
                  b"heman_image *heman_lighting_apply(heman_image *, heman_image *, float, float, float, const float *)\0",)).as_ptr (),);
            }
        }
        static mut default_pos: [libc::c_float; 3] = [-0.5f32, 0.5f32, 1.0f32];
        if light_position.is_null() {
            light_position = default_pos.as_mut_ptr();
        }
        let mut colors = (*final_0).data as *mut kmVec3;
        let mut invgamma = 1.0f32 / _gamma;
        let mut L = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        L.x = *light_position.offset(0 as isize);
        L.y = *light_position.offset(1 as isize);
        L.z = *light_position.offset(2 as isize);
        kmVec3Normalize(&mut L, &mut L);
        let mut y: i32 = 0;
        y = 0;
        while y < height {
            let mut color = colors.offset((y * width) as isize);
            let mut x = 0;
            while x < width {
                let mut N = heman_image_texel(normals, x, y) as *mut kmVec3;
                kmVec3Lerp(N, N, &KM_VEC3_POS_Z, diffuse_softening);
                let mut df = 1 as libc::c_float
                    - diffuse
                        * (1 as libc::c_float
                            - kmClamp(
                                kmVec3Dot(N, &mut L),
                                0 as libc::c_float,
                                1 as libc::c_float,
                            ));
                let mut of = 1 as libc::c_float
                    - occlusion * (1 as libc::c_float - *heman_image_texel(occ, x, y));
                if !albedo.is_null() {
                    *color = *(heman_image_texel(albedo, x, y) as *mut kmVec3);
                } else {
                    let ref mut fresh0 = (*color).z;
                    *fresh0 = 1 as libc::c_float;
                    let ref mut fresh1 = (*color).y;
                    *fresh1 = *fresh0;
                    (*color).x = *fresh1;
                };
                (*color).x = pow((*color).x as f64, _gamma as f64) as libc::c_float;
                (*color).y = pow((*color).y as f64, _gamma as f64) as libc::c_float;
                (*color).z = pow((*color).z as f64, _gamma as f64) as libc::c_float;
                kmVec3Scale(color, color, df * of);
                (*color).x = pow((*color).x as f64, invgamma as f64) as libc::c_float;
                (*color).y = pow((*color).y as f64, invgamma as f64) as libc::c_float;
                (*color).z = pow((*color).z as f64, invgamma as f64) as libc::c_float;
                x += 1;
                color = color.offset(1);
            }
            y += 1;
        }
        heman_image_destroy(normals);
        heman_image_destroy(occ);
        return final_0;
    }
}

extern "C" fn azimuth_slope(mut a: kmVec3, mut b: kmVec3) -> libc::c_float {
    let mut d = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    unsafe {
        kmVec3Subtract(&mut d, &mut a, &mut b);
        let mut x = kmVec3Length(&mut d);
        let mut y = b.z - a.z;
        return y / x;
    }
}

extern "C" fn compute_occlusion(mut thispt: kmVec3, mut horizonpt: kmVec3) -> libc::c_float {
    let mut direction = kmVec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    unsafe {
        kmVec3Subtract(&mut direction, &mut horizonpt, &mut thispt);
        kmVec3Normalize(&mut direction, &mut direction);
        let mut dot = kmVec3Dot(&mut direction, &KM_VEC3_POS_Z);
        return (atan((if dot > 0.0f32 { dot } else { 0.0f32 }) as f64) * 0.63661977236f64)
            as libc::c_float;
    }
}

extern "C" fn horizon_scan(
    mut heightmap: *mut heman_image,
    mut result: *mut heman_image,
    mut startpts: *mut i32,
    mut dx: i32,
    mut dy: i32,
) {
    unsafe {
        let mut w = (*heightmap).width;
        let mut h = (*heightmap).height;
        let mut sx = (dx > 0) as i32 - (dx < 0) as i32;
        let mut sy = (dy > 0) as i32 - (dy < 0) as i32;
        let mut ax = abs(dx);
        let mut ay = abs(dy);
        let mut nsweeps = ay * w + ax * h - (ax + ay - 1);
        let mut p = startpts;
        let mut x = -ax;
        while x < w - ax {
            let mut y = -ay;
            while y < h - ay {
                if !(x >= 0 && x < w && y >= 0 && y < h) {
                    let fresh2 = p;
                    p = p.offset(1);
                    *fresh2 = if sx < 0 { w - x - 1 } else { x };
                    let fresh3 = p;
                    p = p.offset(1);
                    *fresh3 = if sy < 0 { h - y - 1 } else { y };
                }
                y += 1;
            }
            x += 1;
        }
        if nsweeps as i64 == p.offset_from(startpts) as i64 / 2 {
        } else {
            __assert_fail(
                b"nsweeps == (p - startpts) / 2\0" as *const u8 as *const i8,
                b"../src/lighting.c\0" as *const u8 as *const i8,
                160,
                (*::std::mem::transmute::<&[u8; 65], &[i8; 65]>(
                    b"void horizon_scan(heman_image *, heman_image *, int *, int, int)\0",
                ))
                .as_ptr(),
            );
        }
        let mut pathlen = 0;
        let mut i = *startpts.offset(0 as isize);
        let mut j = *startpts.offset(1 as isize);
        loop {
            i += dx;
            j += dy;
            pathlen += 1;
            if !(i >= 0 && i < w && j >= 0 && j < h) {
                break;
            }
        }
        let mut cellw = _occlusion_scale / (if w > h { w } else { h }) as libc::c_float;
        let mut cellh = _occlusion_scale / (if w > h { w } else { h }) as libc::c_float;
        let mut hull_buffer = malloc(
            (::std::mem::size_of::<kmVec3>() as u64)
                .wrapping_mul(pathlen as u64)
                .wrapping_mul(nsweeps as u64),
        ) as *mut kmVec3;
        let mut sweep: i32 = 0;
        sweep = 0;
        while sweep < nsweeps {
            let mut convex_hull = hull_buffer.offset((sweep * pathlen) as isize);
            let mut p_0 = startpts.offset((sweep * 2i32) as isize);
            let mut i_0 = *p_0.offset(0 as isize);
            let mut j_0 = *p_0.offset(1 as isize);
            let mut thispt = kmVec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            };
            let mut horizonpt = kmVec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            };
            thispt.x = i_0 as libc::c_float * cellw;
            thispt.y = j_0 as libc::c_float * cellh;
            thispt.z = *heman_image_texel(
                heightmap,
                if 0 > (if w - 1 > i_0 { i_0 } else { w - 1 }) {
                    0
                } else if w - 1 > i_0 {
                    i_0
                } else {
                    w - 1
                },
                if 0 > (if h - 1 > j_0 { j_0 } else { h - 1 }) {
                    0
                } else if h - 1 > j_0 {
                    j_0
                } else {
                    h - 1
                },
            );
            let mut stack_top = 0;
            *convex_hull.offset(0 as isize) = thispt;
            i_0 += dx;
            j_0 += dy;
            while i_0 >= 0 && i_0 < w && j_0 >= 0 && j_0 < h {
                thispt.x = i_0 as libc::c_float * cellw;
                thispt.y = j_0 as libc::c_float * cellh;
                thispt.z = *heman_image_texel(heightmap, i_0, j_0);
                while stack_top > 0 {
                    let mut s1 = azimuth_slope(thispt, *convex_hull.offset(stack_top as isize));
                    let mut s2 =
                        azimuth_slope(thispt, *convex_hull.offset((stack_top - 1i32) as isize));
                    if s1 >= s2 {
                        break;
                    }
                    stack_top -= 1;
                }
                let fresh4 = stack_top;
                stack_top = stack_top + 1;
                horizonpt = *convex_hull.offset(fresh4 as isize);
                if stack_top < pathlen {
                } else {
                    __assert_fail(
                        b"stack_top < pathlen\0" as *const u8 as *const i8,
                        b"../src/lighting.c\0" as *const u8 as *const i8,
                        213,
                        (*::std::mem::transmute::<&[u8; 65], &[i8; 65]>(
                            b"void horizon_scan(heman_image *, heman_image *, int *, int, int)\0",
                        ))
                        .as_ptr(),
                    );
                };
                *convex_hull.offset(stack_top as isize) = thispt;
                let mut occlusion = compute_occlusion(thispt, horizonpt);
                *heman_image_texel(result, i_0, j_0) += 1.0f32 / 16.0f32 * occlusion;
                i_0 += dx;
                j_0 += dy;
            }
            sweep += 1;
        }
        free(hull_buffer as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn heman_lighting_compute_occlusion(
    mut heightmap: *mut heman_image,
) -> *mut heman_image {
    unsafe {
        if (*heightmap).nbands == 1 {
        } else {
            __assert_fail(
                b"heightmap->nbands == 1\0" as *const u8 as *const i8,
                b"../src/lighting.c\0" as *const u8 as *const i8,
                227,
                (*::std::mem::transmute::<&[u8; 61], &[i8; 61]>(
                    b"heman_image *heman_lighting_compute_occlusion(heman_image *)\0",
                ))
                .as_ptr(),
            );
        }
        let mut width = (*heightmap).width;
        let mut height = (*heightmap).height;
        let mut result = heman_image_create(width, height, 1);
        memset(
            (*result).data as *mut libc::c_void,
            0,
            (::std::mem::size_of::<libc::c_float>() as u64)
                .wrapping_mul(width as u64)
                .wrapping_mul(height as u64),
        );
        let scans: [i32; 32] = [
            1, 0, 0, 1, -1, 0, 0, -1, 1, 1, -1, -1, 1, -1, -1, 1, 2, 1, 2, -1, -2, 1, -2, -1, 1, 2,
            1, -2, -1, 2, -1, -2,
        ];
        let mut startpts = malloc(
            ((::std::mem::size_of::<i32>() as u64)
                .wrapping_mul(2u64)
                .wrapping_mul(3) as libc::c_float
                * kmMax(width as libc::c_float, height as libc::c_float)) as u64,
        ) as *mut i32;
        let mut i = 0;
        while i < 16 {
            let mut dx = scans[(i * 2i32) as usize];
            let mut dy = scans[(i * 2 + 1i32) as usize];
            horizon_scan(heightmap, result, startpts, dx, dy);
            i += 1;
        }
        let mut i_0 = 0;
        while i_0 < width * height {
            *((*result).data).offset(i_0 as isize) =
                1.0f32 - *((*result).data).offset(i_0 as isize);
            if *((*result).data).offset(i_0 as isize) as f64 >= 0.0f64
                && *((*result).data).offset(i_0 as isize) <= 1.0f32
            {
            } else {
                __assert_fail(
                    b"result->data[i] >= 0.0 && result->data[i] <= 1.0f\0" as *const u8
                        as *const i8,
                    b"../src/lighting.c\0" as *const u8 as *const i8,
                    253,
                    (*::std::mem::transmute::<&[u8; 61], &[i8; 61]>(
                        b"heman_image *heman_lighting_compute_occlusion(heman_image *)\0",
                    ))
                    .as_ptr(),
                );
            }
            i_0 += 1;
        }
        free(startpts as *mut libc::c_void);
        return result;
    }
}
