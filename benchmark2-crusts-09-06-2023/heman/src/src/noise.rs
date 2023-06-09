use libc;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct osn_context {
    pub perm: *mut i16,
    pub permGradIndex3D: *mut i16,
}
static mut gradients2D: [i8; 16] = [
    5, 2, 2, 5, -5 as i8, 2, -2 as i8, 5, 5, -2 as i8, 2, -5 as i8, -5 as i8, -2 as i8, -2 as i8,
    -5 as i8,
];
static mut gradients3D: [i8; 72] = [
    -11 as i8, 4, 4, -4 as i8, 11, 4, -4 as i8, 4, 11, 11, 4, 4, 4, 11, 4, 4, 4, 11, -11 as i8,
    -4 as i8, 4, -4 as i8, -11 as i8, 4, -4 as i8, -4 as i8, 11, 11, -4 as i8, 4, 4, -11 as i8, 4,
    4, -4 as i8, 11, -11 as i8, 4, -4 as i8, -4 as i8, 11, -4 as i8, -4 as i8, 4, -11 as i8, 11, 4,
    -4 as i8, 4, 11, -4 as i8, 4, 4, -11 as i8, -11 as i8, -4 as i8, -4 as i8, -4 as i8, -11 as i8,
    -4 as i8, -4 as i8, -4 as i8, -11 as i8, 11, -4 as i8, -4 as i8, 4, -11 as i8, -4 as i8, 4,
    -4 as i8, -11 as i8,
];
static mut gradients4D: [i8; 256] = [
    3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, 1, 1, 1, 1, 3, -3 as i8, 1, 1, 1, -1 as i8, 3, 1, 1, -1 as i8,
    1, 3, 1, -1 as i8, 1, 1, 3, 3, -1 as i8, 1, 1, 1, -3 as i8, 1, 1, 1, -1 as i8, 3, 1, 1,
    -1 as i8, 1, 3, -3 as i8, -1 as i8, 1, 1, -1 as i8, -3 as i8, 1, 1, -1 as i8, -1 as i8, 3, 1,
    -1 as i8, -1 as i8, 1, 3, 3, 1, -1 as i8, 1, 1, 3, -1 as i8, 1, 1, 1, -3 as i8, 1, 1, 1,
    -1 as i8, 3, -3 as i8, 1, -1 as i8, 1, -1 as i8, 3, -1 as i8, 1, -1 as i8, 1, -3 as i8, 1,
    -1 as i8, 1, -1 as i8, 3, 3, -1 as i8, -1 as i8, 1, 1, -3 as i8, -1 as i8, 1, 1, -1 as i8,
    -3 as i8, 1, 1, -1 as i8, -1 as i8, 3, -3 as i8, -1 as i8, -1 as i8, 1, -1 as i8, -3 as i8,
    -1 as i8, 1, -1 as i8, -1 as i8, -3 as i8, 1, -1 as i8, -1 as i8, -1 as i8, 3, 3, 1, 1,
    -1 as i8, 1, 3, 1, -1 as i8, 1, 1, 3, -1 as i8, 1, 1, 1, -3 as i8, -3 as i8, 1, 1, -1 as i8,
    -1 as i8, 3, 1, -1 as i8, -1 as i8, 1, 3, -1 as i8, -1 as i8, 1, 1, -3 as i8, 3, -1 as i8, 1,
    -1 as i8, 1, -3 as i8, 1, -1 as i8, 1, -1 as i8, 3, -1 as i8, 1, -1 as i8, 1, -3 as i8,
    -3 as i8, -1 as i8, 1, -1 as i8, -1 as i8, -3 as i8, 1, -1 as i8, -1 as i8, -1 as i8, 3,
    -1 as i8, -1 as i8, -1 as i8, 1, -3 as i8, 3, 1, -1 as i8, -1 as i8, 1, 3, -1 as i8, -1 as i8,
    1, 1, -3 as i8, -1 as i8, 1, 1, -1 as i8, -3 as i8, -3 as i8, 1, -1 as i8, -1 as i8, -1 as i8,
    3, -1 as i8, -1 as i8, -1 as i8, 1, -3 as i8, -1 as i8, -1 as i8, 1, -1 as i8, -3 as i8, 3,
    -1 as i8, -1 as i8, -1 as i8, 1, -3 as i8, -1 as i8, -1 as i8, 1, -1 as i8, -3 as i8, -1 as i8,
    1, -1 as i8, -1 as i8, -3 as i8, -3 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8, -3 as i8,
    -1 as i8, -1 as i8, -1 as i8, -1 as i8, -3 as i8, -1 as i8, -1 as i8, -1 as i8, -1 as i8,
    -3 as i8,
];
extern "C" fn extrapolate2(
    mut ctx: *mut osn_context,
    mut xsb: i32,
    mut ysb: i32,
    mut dx: f64,
    mut dy: f64,
) -> f64 {
    unsafe {
        let mut perm = (*ctx).perm;
        let mut index = *perm
            .offset((*perm.offset((xsb & 0xffi32) as isize) as i32 + ysb & 0xffi32) as isize)
            as i32
            & 0xe;
        return gradients2D[index as usize] as i32 as f64 * dx
            + gradients2D[(index + 1i32) as usize] as i32 as f64 * dy;
    }
}

extern "C" fn extrapolate3(
    mut ctx: *mut osn_context,
    mut xsb: i32,
    mut ysb: i32,
    mut zsb: i32,
    mut dx: f64,
    mut dy: f64,
    mut dz: f64,
) -> f64 {
    unsafe {
        let mut perm = (*ctx).perm;
        let mut permGradIndex3D = (*ctx).permGradIndex3D;
        let mut index = *permGradIndex3D.offset(
            (*perm.offset((*perm.offset((xsb & 0xffi32) as isize) as i32 + ysb & 0xffi32) as isize)
                as i32
                + zsb
                & 0xffi32) as isize,
        ) as i32;
        return gradients3D[index as usize] as i32 as f64 * dx
            + gradients3D[(index + 1i32) as usize] as i32 as f64 * dy
            + gradients3D[(index + 2i32) as usize] as i32 as f64 * dz;
    }
}

extern "C" fn extrapolate4(
    mut ctx: *mut osn_context,
    mut xsb: i32,
    mut ysb: i32,
    mut zsb: i32,
    mut wsb: i32,
    mut dx: f64,
    mut dy: f64,
    mut dz: f64,
    mut dw: f64,
) -> f64 {
    unsafe {
        let mut perm = (*ctx).perm;
        let mut index = *perm.offset(
            (*perm.offset(
                (*perm.offset(
                    (*perm.offset((xsb & 0xffi32) as isize) as i32 + ysb & 0xffi32) as isize,
                ) as i32
                    + zsb
                    & 0xffi32) as isize,
            ) as i32
                + wsb
                & 0xff) as isize,
        ) as i32
            & 0xfc;
        return gradients4D[index as usize] as i32 as f64 * dx
            + gradients4D[(index + 1i32) as usize] as i32 as f64 * dy
            + gradients4D[(index + 2i32) as usize] as i32 as f64 * dz
            + gradients4D[(index + 3i32) as usize] as i32 as f64 * dw;
    }
}

#[inline]
extern "C" fn fastFloor(mut x: f64) -> i32 {
    let mut xi = x as i32;
    return if x < xi as f64 { xi - 1 } else { xi };
}

extern "C" fn allocate_perm(mut ctx: *mut osn_context, mut nperm: i32, mut ngrad: i32) -> i32 {
    unsafe {
        if !((*ctx).perm).is_null() {
            free((*ctx).perm as *mut libc::c_void);
        }
        if !((*ctx).permGradIndex3D).is_null() {
            free((*ctx).permGradIndex3D as *mut libc::c_void);
        }
        let ref mut fresh0 = (*ctx).perm;
        *fresh0 =
            malloc((::std::mem::size_of::<i16>() as u64).wrapping_mul(nperm as u64)) as *mut i16;
        if ((*ctx).perm).is_null() {
            return -12;
        }
        let ref mut fresh1 = (*ctx).permGradIndex3D;
        *fresh1 =
            malloc((::std::mem::size_of::<i16>() as u64).wrapping_mul(ngrad as u64)) as *mut i16;
        if ((*ctx).permGradIndex3D).is_null() {
            free((*ctx).perm as *mut libc::c_void);
            return -12;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn open_simplex_noise_init_perm(
    mut ctx: *mut osn_context,
    mut p: *mut i16,
    mut nelements: i32,
) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut rc: i32 = 0;
        rc = allocate_perm(ctx, nelements, 256);
        if rc != 0 {
            return rc;
        }
        memcpy(
            (*ctx).perm as *mut libc::c_void,
            p as *const libc::c_void,
            (::std::mem::size_of::<i16>() as u64).wrapping_mul(nelements as u64),
        );
        i = 0;
        while i < 256 {
            *((*ctx).permGradIndex3D).offset(i as isize) = (*((*ctx).perm).offset(i as isize)
                as u64)
                .wrapping_rem(
                    (::std::mem::size_of::<[i8; 72]>() as u64)
                        .wrapping_div(::std::mem::size_of::<i8>() as u64)
                        .wrapping_div(3),
                )
                .wrapping_mul(3) as i16;
            i += 1;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn open_simplex_noise(mut seed: i64, mut ctx: *mut *mut osn_context) -> i32 {
    unsafe {
        let mut rc: i32 = 0;
        let mut source: [i16; 256] = [0; 256];
        let mut i: i32 = 0;
        let mut perm = 0 as *mut i16;
        let mut permGradIndex3D = 0 as *mut i16;
        *ctx = malloc(::std::mem::size_of::<osn_context>() as u64) as *mut osn_context;
        if (*ctx).is_null() {
            return -12;
        }
        let ref mut fresh2 = (**ctx).perm;
        *fresh2 = 0 as *mut i16;
        let ref mut fresh3 = (**ctx).permGradIndex3D;
        *fresh3 = 0 as *mut i16;
        rc = allocate_perm(*ctx, 256, 256);
        if rc != 0 {
            free(*ctx as *mut libc::c_void);
            return rc;
        }
        perm = (**ctx).perm;
        permGradIndex3D = (**ctx).permGradIndex3D;
        i = 0;
        while i < 256 {
            source[i as usize] = i as i16;
            i += 1;
        }
        seed = (seed as i64 * 6364136223846793005 + 1442695040888963407i64) as i64;
        seed = (seed as i64 * 6364136223846793005 + 1442695040888963407i64) as i64;
        seed = (seed as i64 * 6364136223846793005 + 1442695040888963407i64) as i64;
        i = 255;
        while i >= 0 {
            seed = (seed as i64 * 6364136223846793005 + 1442695040888963407i64) as i64;
            let mut r = ((seed + 31i64) % (i + 1i32) as i64) as i32;
            if r < 0 {
                r += i + 1;
            };
            *perm.offset(i as isize) = source[r as usize];
            *permGradIndex3D.offset(i as isize) = (*perm.offset(i as isize) as u64)
                .wrapping_rem(
                    (::std::mem::size_of::<[i8; 72]>() as u64)
                        .wrapping_div(::std::mem::size_of::<i8>() as u64)
                        .wrapping_div(3),
                )
                .wrapping_mul(3) as i16;
            source[r as usize] = source[i as usize];
            i -= 1;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn open_simplex_noise_free(mut ctx: *mut osn_context) {
    unsafe {
        if ctx.is_null() {
            return;
        }
        if !((*ctx).perm).is_null() {
            free((*ctx).perm as *mut libc::c_void);
            let ref mut fresh4 = (*ctx).perm;
            *fresh4 = 0 as *mut i16;
        }
        if !((*ctx).permGradIndex3D).is_null() {
            free((*ctx).permGradIndex3D as *mut libc::c_void);
            let ref mut fresh5 = (*ctx).permGradIndex3D;
            *fresh5 = 0 as *mut i16;
        }
        free(ctx as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn open_simplex_noise2(mut ctx: *mut osn_context, mut x: f64, mut y: f64) -> f64 {
    unsafe {
        let mut stretchOffset = (x + y) * -0.211324865405187f64;
        let mut xs = x + stretchOffset;
        let mut ys = y + stretchOffset;
        let mut xsb = fastFloor(xs);
        let mut ysb = fastFloor(ys);
        let mut squishOffset = (xsb + ysb) as f64 * 0.366025403784439f64;
        let mut xb = xsb as f64 + squishOffset;
        let mut yb = ysb as f64 + squishOffset;
        let mut xins = xs - xsb as f64;
        let mut yins = ys - ysb as f64;
        let mut inSum = xins + yins;
        let mut dx0 = x - xb;
        let mut dy0 = y - yb;
        let mut dx_ext: f64 = 0.;
        let mut dy_ext: f64 = 0.;
        let mut xsv_ext: i32 = 0;
        let mut ysv_ext: i32 = 0;
        let mut value = 0 as f64;
        let mut dx1 = dx0 - 1 as f64 - 0.366025403784439f64;
        let mut dy1 = dy0 - 0 as f64 - 0.366025403784439f64;
        let mut attn1 = 2 as f64 - dx1 * dx1 - dy1 * dy1;
        if attn1 > 0 as f64 {
            attn1 *= attn1;
            value += attn1 * attn1 * extrapolate2(ctx, xsb + 1, ysb + 0, dx1, dy1);
        }
        let mut dx2 = dx0 - 0 as f64 - 0.366025403784439f64;
        let mut dy2 = dy0 - 1 as f64 - 0.366025403784439f64;
        let mut attn2 = 2 as f64 - dx2 * dx2 - dy2 * dy2;
        if attn2 > 0 as f64 {
            attn2 *= attn2;
            value += attn2 * attn2 * extrapolate2(ctx, xsb + 0, ysb + 1, dx2, dy2);
        }
        if inSum <= 1 as f64 {
            let mut zins = 1 as f64 - inSum;
            if zins > xins || zins > yins {
                if xins > yins {
                    xsv_ext = xsb + 1;
                    ysv_ext = ysb - 1;
                    dx_ext = dx0 - 1 as f64;
                    dy_ext = dy0 + 1 as f64;
                } else {
                    xsv_ext = xsb - 1;
                    ysv_ext = ysb + 1;
                    dx_ext = dx0 + 1 as f64;
                    dy_ext = dy0 - 1 as f64;
                }
            } else {
                xsv_ext = xsb + 1;
                ysv_ext = ysb + 1;
                dx_ext = dx0 - 1 as f64 - 2 as f64 * 0.366025403784439f64;
                dy_ext = dy0 - 1 as f64 - 2 as f64 * 0.366025403784439f64;
            }
        } else {
            let mut zins_0 = 2 as f64 - inSum;
            if zins_0 < xins || zins_0 < yins {
                if xins > yins {
                    xsv_ext = xsb + 2;
                    ysv_ext = ysb + 0;
                    dx_ext = dx0 - 2 as f64 - 2 as f64 * 0.366025403784439f64;
                    dy_ext = dy0 + 0 as f64 - 2 as f64 * 0.366025403784439f64;
                } else {
                    xsv_ext = xsb + 0;
                    ysv_ext = ysb + 2;
                    dx_ext = dx0 + 0 as f64 - 2 as f64 * 0.366025403784439f64;
                    dy_ext = dy0 - 2 as f64 - 2 as f64 * 0.366025403784439f64;
                }
            } else {
                dx_ext = dx0;
                dy_ext = dy0;
                xsv_ext = xsb;
                ysv_ext = ysb;
            }
            xsb += 1;
            ysb += 1;
            dx0 = dx0 - 1 as f64 - 2 as f64 * 0.366025403784439f64;
            dy0 = dy0 - 1 as f64 - 2 as f64 * 0.366025403784439f64;
        }
        let mut attn0 = 2 as f64 - dx0 * dx0 - dy0 * dy0;
        if attn0 > 0 as f64 {
            attn0 *= attn0;
            value += attn0 * attn0 * extrapolate2(ctx, xsb, ysb, dx0, dy0);
        }
        let mut attn_ext = 2 as f64 - dx_ext * dx_ext - dy_ext * dy_ext;
        if attn_ext > 0 as f64 {
            attn_ext *= attn_ext;
            value += attn_ext * attn_ext * extrapolate2(ctx, xsv_ext, ysv_ext, dx_ext, dy_ext);
        }
        return value / 47.0f64;
    }
}

#[no_mangle]
pub extern "C" fn open_simplex_noise3(
    mut ctx: *mut osn_context,
    mut x: f64,
    mut y: f64,
    mut z: f64,
) -> f64 {
    unsafe {
        let mut stretchOffset = (x + y + z) * (-1.0f64 / 6.0f64);
        let mut xs = x + stretchOffset;
        let mut ys = y + stretchOffset;
        let mut zs = z + stretchOffset;
        let mut xsb = fastFloor(xs);
        let mut ysb = fastFloor(ys);
        let mut zsb = fastFloor(zs);
        let mut squishOffset = (xsb + ysb + zsb) as f64 * (1.0f64 / 3.0f64);
        let mut xb = xsb as f64 + squishOffset;
        let mut yb = ysb as f64 + squishOffset;
        let mut zb = zsb as f64 + squishOffset;
        let mut xins = xs - xsb as f64;
        let mut yins = ys - ysb as f64;
        let mut zins = zs - zsb as f64;
        let mut inSum = xins + yins + zins;
        let mut dx0 = x - xb;
        let mut dy0 = y - yb;
        let mut dz0 = z - zb;
        let mut dx_ext0: f64 = 0.;
        let mut dy_ext0: f64 = 0.;
        let mut dz_ext0: f64 = 0.;
        let mut dx_ext1: f64 = 0.;
        let mut dy_ext1: f64 = 0.;
        let mut dz_ext1: f64 = 0.;
        let mut xsv_ext0: i32 = 0;
        let mut ysv_ext0: i32 = 0;
        let mut zsv_ext0: i32 = 0;
        let mut xsv_ext1: i32 = 0;
        let mut ysv_ext1: i32 = 0;
        let mut zsv_ext1: i32 = 0;
        let mut value = 0 as f64;
        if inSum <= 1 as f64 {
            let mut aPoint = 0x1;
            let mut aScore = xins;
            let mut bPoint = 0x2;
            let mut bScore = yins;
            if aScore >= bScore && zins > bScore {
                bScore = zins;
                bPoint = 0x4;
            } else if aScore < bScore && zins > aScore {
                aScore = zins;
                aPoint = 0x4;
            }
            let mut wins = 1 as f64 - inSum;
            if wins > aScore || wins > bScore {
                let mut c = (if bScore > aScore {
                    bPoint as i32
                } else {
                    aPoint as i32
                }) as i8;
                if c as i32 & 0x1 == 0 {
                    xsv_ext0 = xsb - 1;
                    xsv_ext1 = xsb;
                    dx_ext0 = dx0 + 1 as f64;
                    dx_ext1 = dx0;
                } else {
                    xsv_ext1 = xsb + 1;
                    xsv_ext0 = xsv_ext1;
                    dx_ext1 = dx0 - 1 as f64;
                    dx_ext0 = dx_ext1;
                }
                if c as i32 & 0x2 == 0 {
                    ysv_ext1 = ysb;
                    ysv_ext0 = ysv_ext1;
                    dy_ext1 = dy0;
                    dy_ext0 = dy_ext1;
                    if c as i32 & 0x1 == 0 {
                        ysv_ext1 -= 1;
                        dy_ext1 += 1 as f64;
                    } else {
                        ysv_ext0 -= 1;
                        dy_ext0 += 1 as f64;
                    }
                } else {
                    ysv_ext1 = ysb + 1;
                    ysv_ext0 = ysv_ext1;
                    dy_ext1 = dy0 - 1 as f64;
                    dy_ext0 = dy_ext1;
                }
                if c as i32 & 0x4 == 0 {
                    zsv_ext0 = zsb;
                    zsv_ext1 = zsb - 1;
                    dz_ext0 = dz0;
                    dz_ext1 = dz0 + 1 as f64;
                } else {
                    zsv_ext1 = zsb + 1;
                    zsv_ext0 = zsv_ext1;
                    dz_ext1 = dz0 - 1 as f64;
                    dz_ext0 = dz_ext1;
                }
            } else {
                let mut c_0 = (aPoint as i32 | bPoint as i32) as i8;
                if c_0 as i32 & 0x1 == 0 {
                    xsv_ext0 = xsb;
                    xsv_ext1 = xsb - 1;
                    dx_ext0 = dx0 - 2 as f64 * (1.0f64 / 3.0f64);
                    dx_ext1 = dx0 + 1 as f64 - 1.0f64 / 3.0f64;
                } else {
                    xsv_ext1 = xsb + 1;
                    xsv_ext0 = xsv_ext1;
                    dx_ext0 = dx0 - 1 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
                    dx_ext1 = dx0 - 1 as f64 - 1.0f64 / 3.0f64;
                }
                if c_0 as i32 & 0x2 == 0 {
                    ysv_ext0 = ysb;
                    ysv_ext1 = ysb - 1;
                    dy_ext0 = dy0 - 2 as f64 * (1.0f64 / 3.0f64);
                    dy_ext1 = dy0 + 1 as f64 - 1.0f64 / 3.0f64;
                } else {
                    ysv_ext1 = ysb + 1;
                    ysv_ext0 = ysv_ext1;
                    dy_ext0 = dy0 - 1 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
                    dy_ext1 = dy0 - 1 as f64 - 1.0f64 / 3.0f64;
                }
                if c_0 as i32 & 0x4 == 0 {
                    zsv_ext0 = zsb;
                    zsv_ext1 = zsb - 1;
                    dz_ext0 = dz0 - 2 as f64 * (1.0f64 / 3.0f64);
                    dz_ext1 = dz0 + 1 as f64 - 1.0f64 / 3.0f64;
                } else {
                    zsv_ext1 = zsb + 1;
                    zsv_ext0 = zsv_ext1;
                    dz_ext0 = dz0 - 1 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
                    dz_ext1 = dz0 - 1 as f64 - 1.0f64 / 3.0f64;
                }
            }
            let mut attn0 = 2 as f64 - dx0 * dx0 - dy0 * dy0 - dz0 * dz0;
            if attn0 > 0 as f64 {
                attn0 *= attn0;
                value +=
                    attn0 * attn0 * extrapolate3(ctx, xsb + 0, ysb + 0, zsb + 0, dx0, dy0, dz0);
            }
            let mut dx1 = dx0 - 1 as f64 - 1.0f64 / 3.0f64;
            let mut dy1 = dy0 - 0 as f64 - 1.0f64 / 3.0f64;
            let mut dz1 = dz0 - 0 as f64 - 1.0f64 / 3.0f64;
            let mut attn1 = 2 as f64 - dx1 * dx1 - dy1 * dy1 - dz1 * dz1;
            if attn1 > 0 as f64 {
                attn1 *= attn1;
                value +=
                    attn1 * attn1 * extrapolate3(ctx, xsb + 1, ysb + 0, zsb + 0, dx1, dy1, dz1);
            }
            let mut dx2 = dx0 - 0 as f64 - 1.0f64 / 3.0f64;
            let mut dy2 = dy0 - 1 as f64 - 1.0f64 / 3.0f64;
            let mut dz2 = dz1;
            let mut attn2 = 2 as f64 - dx2 * dx2 - dy2 * dy2 - dz2 * dz2;
            if attn2 > 0 as f64 {
                attn2 *= attn2;
                value +=
                    attn2 * attn2 * extrapolate3(ctx, xsb + 0, ysb + 1, zsb + 0, dx2, dy2, dz2);
            }
            let mut dx3 = dx2;
            let mut dy3 = dy1;
            let mut dz3 = dz0 - 1 as f64 - 1.0f64 / 3.0f64;
            let mut attn3 = 2 as f64 - dx3 * dx3 - dy3 * dy3 - dz3 * dz3;
            if attn3 > 0 as f64 {
                attn3 *= attn3;
                value +=
                    attn3 * attn3 * extrapolate3(ctx, xsb + 0, ysb + 0, zsb + 1, dx3, dy3, dz3);
            }
        } else if inSum >= 2 as f64 {
            let mut aPoint_0 = 0x6;
            let mut aScore_0 = xins;
            let mut bPoint_0 = 0x5;
            let mut bScore_0 = yins;
            if aScore_0 <= bScore_0 && zins < bScore_0 {
                bScore_0 = zins;
                bPoint_0 = 0x3;
            } else if aScore_0 > bScore_0 && zins < aScore_0 {
                aScore_0 = zins;
                aPoint_0 = 0x3;
            }
            let mut wins_0 = 3 as f64 - inSum;
            if wins_0 < aScore_0 || wins_0 < bScore_0 {
                let mut c_1 = (if bScore_0 < aScore_0 {
                    bPoint_0 as i32
                } else {
                    aPoint_0 as i32
                }) as i8;
                if c_1 as i32 & 0x1 != 0 {
                    xsv_ext0 = xsb + 2;
                    xsv_ext1 = xsb + 1;
                    dx_ext0 = dx0 - 2 as f64 - 3 as f64 * (1.0f64 / 3.0f64);
                    dx_ext1 = dx0 - 1 as f64 - 3 as f64 * (1.0f64 / 3.0f64);
                } else {
                    xsv_ext1 = xsb;
                    xsv_ext0 = xsv_ext1;
                    dx_ext1 = dx0 - 3 as f64 * (1.0f64 / 3.0f64);
                    dx_ext0 = dx_ext1;
                }
                if c_1 as i32 & 0x2 != 0 {
                    ysv_ext1 = ysb + 1;
                    ysv_ext0 = ysv_ext1;
                    dy_ext1 = dy0 - 1 as f64 - 3 as f64 * (1.0f64 / 3.0f64);
                    dy_ext0 = dy_ext1;
                    if c_1 as i32 & 0x1 != 0 {
                        ysv_ext1 += 1;
                        dy_ext1 -= 1 as f64;
                    } else {
                        ysv_ext0 += 1;
                        dy_ext0 -= 1 as f64;
                    }
                } else {
                    ysv_ext1 = ysb;
                    ysv_ext0 = ysv_ext1;
                    dy_ext1 = dy0 - 3 as f64 * (1.0f64 / 3.0f64);
                    dy_ext0 = dy_ext1;
                }
                if c_1 as i32 & 0x4 != 0 {
                    zsv_ext0 = zsb + 1;
                    zsv_ext1 = zsb + 2;
                    dz_ext0 = dz0 - 1 as f64 - 3 as f64 * (1.0f64 / 3.0f64);
                    dz_ext1 = dz0 - 2 as f64 - 3 as f64 * (1.0f64 / 3.0f64);
                } else {
                    zsv_ext1 = zsb;
                    zsv_ext0 = zsv_ext1;
                    dz_ext1 = dz0 - 3 as f64 * (1.0f64 / 3.0f64);
                    dz_ext0 = dz_ext1;
                }
            } else {
                let mut c_2 = (aPoint_0 as i32 & bPoint_0 as i32) as i8;
                if c_2 as i32 & 0x1 != 0 {
                    xsv_ext0 = xsb + 1;
                    xsv_ext1 = xsb + 2;
                    dx_ext0 = dx0 - 1 as f64 - 1.0f64 / 3.0f64;
                    dx_ext1 = dx0 - 2 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
                } else {
                    xsv_ext1 = xsb;
                    xsv_ext0 = xsv_ext1;
                    dx_ext0 = dx0 - 1.0f64 / 3.0f64;
                    dx_ext1 = dx0 - 2 as f64 * (1.0f64 / 3.0f64);
                }
                if c_2 as i32 & 0x2 != 0 {
                    ysv_ext0 = ysb + 1;
                    ysv_ext1 = ysb + 2;
                    dy_ext0 = dy0 - 1 as f64 - 1.0f64 / 3.0f64;
                    dy_ext1 = dy0 - 2 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
                } else {
                    ysv_ext1 = ysb;
                    ysv_ext0 = ysv_ext1;
                    dy_ext0 = dy0 - 1.0f64 / 3.0f64;
                    dy_ext1 = dy0 - 2 as f64 * (1.0f64 / 3.0f64);
                }
                if c_2 as i32 & 0x4 != 0 {
                    zsv_ext0 = zsb + 1;
                    zsv_ext1 = zsb + 2;
                    dz_ext0 = dz0 - 1 as f64 - 1.0f64 / 3.0f64;
                    dz_ext1 = dz0 - 2 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
                } else {
                    zsv_ext1 = zsb;
                    zsv_ext0 = zsv_ext1;
                    dz_ext0 = dz0 - 1.0f64 / 3.0f64;
                    dz_ext1 = dz0 - 2 as f64 * (1.0f64 / 3.0f64);
                }
            }
            let mut dx3_0 = dx0 - 1 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
            let mut dy3_0 = dy0 - 1 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
            let mut dz3_0 = dz0 - 0 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
            let mut attn3_0 = 2 as f64 - dx3_0 * dx3_0 - dy3_0 * dy3_0 - dz3_0 * dz3_0;
            if attn3_0 > 0 as f64 {
                attn3_0 *= attn3_0;
                value += attn3_0
                    * attn3_0
                    * extrapolate3(ctx, xsb + 1, ysb + 1, zsb + 0, dx3_0, dy3_0, dz3_0);
            }
            let mut dx2_0 = dx3_0;
            let mut dy2_0 = dy0 - 0 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
            let mut dz2_0 = dz0 - 1 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
            let mut attn2_0 = 2 as f64 - dx2_0 * dx2_0 - dy2_0 * dy2_0 - dz2_0 * dz2_0;
            if attn2_0 > 0 as f64 {
                attn2_0 *= attn2_0;
                value += attn2_0
                    * attn2_0
                    * extrapolate3(ctx, xsb + 1, ysb + 0, zsb + 1, dx2_0, dy2_0, dz2_0);
            }
            let mut dx1_0 = dx0 - 0 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
            let mut dy1_0 = dy3_0;
            let mut dz1_0 = dz2_0;
            let mut attn1_0 = 2 as f64 - dx1_0 * dx1_0 - dy1_0 * dy1_0 - dz1_0 * dz1_0;
            if attn1_0 > 0 as f64 {
                attn1_0 *= attn1_0;
                value += attn1_0
                    * attn1_0
                    * extrapolate3(ctx, xsb + 0, ysb + 1, zsb + 1, dx1_0, dy1_0, dz1_0);
            }
            dx0 = dx0 - 1 as f64 - 3 as f64 * (1.0f64 / 3.0f64);
            dy0 = dy0 - 1 as f64 - 3 as f64 * (1.0f64 / 3.0f64);
            dz0 = dz0 - 1 as f64 - 3 as f64 * (1.0f64 / 3.0f64);
            let mut attn0_0 = 2 as f64 - dx0 * dx0 - dy0 * dy0 - dz0 * dz0;
            if attn0_0 > 0 as f64 {
                attn0_0 *= attn0_0;
                value +=
                    attn0_0 * attn0_0 * extrapolate3(ctx, xsb + 1, ysb + 1, zsb + 1, dx0, dy0, dz0);
            }
        } else {
            let mut aScore_1: f64 = 0.;
            let mut aPoint_1: i8 = 0;
            let mut aIsFurtherSide: i32 = 0;
            let mut bScore_1: f64 = 0.;
            let mut bPoint_1: i8 = 0;
            let mut bIsFurtherSide: i32 = 0;
            let mut p1 = xins + yins;
            if p1 > 1 as f64 {
                aScore_1 = p1 - 1 as f64;
                aPoint_1 = 0x3;
                aIsFurtherSide = 1;
            } else {
                aScore_1 = 1 as f64 - p1;
                aPoint_1 = 0x4;
                aIsFurtherSide = 0;
            }
            let mut p2 = xins + zins;
            if p2 > 1 as f64 {
                bScore_1 = p2 - 1 as f64;
                bPoint_1 = 0x5;
                bIsFurtherSide = 1;
            } else {
                bScore_1 = 1 as f64 - p2;
                bPoint_1 = 0x2;
                bIsFurtherSide = 0;
            }
            let mut p3 = yins + zins;
            if p3 > 1 as f64 {
                let mut score = p3 - 1 as f64;
                if aScore_1 <= bScore_1 && aScore_1 < score {
                    aScore_1 = score;
                    aPoint_1 = 0x6;
                    aIsFurtherSide = 1;
                } else if aScore_1 > bScore_1 && bScore_1 < score {
                    bScore_1 = score;
                    bPoint_1 = 0x6;
                    bIsFurtherSide = 1;
                }
            } else {
                let mut score_0 = 1 as f64 - p3;
                if aScore_1 <= bScore_1 && aScore_1 < score_0 {
                    aScore_1 = score_0;
                    aPoint_1 = 0x1;
                    aIsFurtherSide = 0;
                } else if aScore_1 > bScore_1 && bScore_1 < score_0 {
                    bScore_1 = score_0;
                    bPoint_1 = 0x1;
                    bIsFurtherSide = 0;
                }
            }
            if aIsFurtherSide == bIsFurtherSide {
                if aIsFurtherSide != 0 {
                    dx_ext0 = dx0 - 1 as f64 - 3 as f64 * (1.0f64 / 3.0f64);
                    dy_ext0 = dy0 - 1 as f64 - 3 as f64 * (1.0f64 / 3.0f64);
                    dz_ext0 = dz0 - 1 as f64 - 3 as f64 * (1.0f64 / 3.0f64);
                    xsv_ext0 = xsb + 1;
                    ysv_ext0 = ysb + 1;
                    zsv_ext0 = zsb + 1;
                    let mut c_3 = (aPoint_1 as i32 & bPoint_1 as i32) as i8;
                    if c_3 as i32 & 0x1 != 0 {
                        dx_ext1 = dx0 - 2 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
                        dy_ext1 = dy0 - 2 as f64 * (1.0f64 / 3.0f64);
                        dz_ext1 = dz0 - 2 as f64 * (1.0f64 / 3.0f64);
                        xsv_ext1 = xsb + 2;
                        ysv_ext1 = ysb;
                        zsv_ext1 = zsb;
                    } else if c_3 as i32 & 0x2 != 0 {
                        dx_ext1 = dx0 - 2 as f64 * (1.0f64 / 3.0f64);
                        dy_ext1 = dy0 - 2 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
                        dz_ext1 = dz0 - 2 as f64 * (1.0f64 / 3.0f64);
                        xsv_ext1 = xsb;
                        ysv_ext1 = ysb + 2;
                        zsv_ext1 = zsb;
                    } else {
                        dx_ext1 = dx0 - 2 as f64 * (1.0f64 / 3.0f64);
                        dy_ext1 = dy0 - 2 as f64 * (1.0f64 / 3.0f64);
                        dz_ext1 = dz0 - 2 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
                        xsv_ext1 = xsb;
                        ysv_ext1 = ysb;
                        zsv_ext1 = zsb + 2;
                    }
                } else {
                    dx_ext0 = dx0;
                    dy_ext0 = dy0;
                    dz_ext0 = dz0;
                    xsv_ext0 = xsb;
                    ysv_ext0 = ysb;
                    zsv_ext0 = zsb;
                    let mut c_4 = (aPoint_1 as i32 | bPoint_1 as i32) as i8;
                    if c_4 as i32 & 0x1 == 0 {
                        dx_ext1 = dx0 + 1 as f64 - 1.0f64 / 3.0f64;
                        dy_ext1 = dy0 - 1 as f64 - 1.0f64 / 3.0f64;
                        dz_ext1 = dz0 - 1 as f64 - 1.0f64 / 3.0f64;
                        xsv_ext1 = xsb - 1;
                        ysv_ext1 = ysb + 1;
                        zsv_ext1 = zsb + 1;
                    } else if c_4 as i32 & 0x2 == 0 {
                        dx_ext1 = dx0 - 1 as f64 - 1.0f64 / 3.0f64;
                        dy_ext1 = dy0 + 1 as f64 - 1.0f64 / 3.0f64;
                        dz_ext1 = dz0 - 1 as f64 - 1.0f64 / 3.0f64;
                        xsv_ext1 = xsb + 1;
                        ysv_ext1 = ysb - 1;
                        zsv_ext1 = zsb + 1;
                    } else {
                        dx_ext1 = dx0 - 1 as f64 - 1.0f64 / 3.0f64;
                        dy_ext1 = dy0 - 1 as f64 - 1.0f64 / 3.0f64;
                        dz_ext1 = dz0 + 1 as f64 - 1.0f64 / 3.0f64;
                        xsv_ext1 = xsb + 1;
                        ysv_ext1 = ysb + 1;
                        zsv_ext1 = zsb - 1;
                    }
                }
            } else {
                let mut c1: i8 = 0;
                let mut c2: i8 = 0;
                if aIsFurtherSide != 0 {
                    c1 = aPoint_1;
                    c2 = bPoint_1;
                } else {
                    c1 = bPoint_1;
                    c2 = aPoint_1;
                }
                if c1 as i32 & 0x1 == 0 {
                    dx_ext0 = dx0 + 1 as f64 - 1.0f64 / 3.0f64;
                    dy_ext0 = dy0 - 1 as f64 - 1.0f64 / 3.0f64;
                    dz_ext0 = dz0 - 1 as f64 - 1.0f64 / 3.0f64;
                    xsv_ext0 = xsb - 1;
                    ysv_ext0 = ysb + 1;
                    zsv_ext0 = zsb + 1;
                } else if c1 as i32 & 0x2 == 0 {
                    dx_ext0 = dx0 - 1 as f64 - 1.0f64 / 3.0f64;
                    dy_ext0 = dy0 + 1 as f64 - 1.0f64 / 3.0f64;
                    dz_ext0 = dz0 - 1 as f64 - 1.0f64 / 3.0f64;
                    xsv_ext0 = xsb + 1;
                    ysv_ext0 = ysb - 1;
                    zsv_ext0 = zsb + 1;
                } else {
                    dx_ext0 = dx0 - 1 as f64 - 1.0f64 / 3.0f64;
                    dy_ext0 = dy0 - 1 as f64 - 1.0f64 / 3.0f64;
                    dz_ext0 = dz0 + 1 as f64 - 1.0f64 / 3.0f64;
                    xsv_ext0 = xsb + 1;
                    ysv_ext0 = ysb + 1;
                    zsv_ext0 = zsb - 1;
                }
                dx_ext1 = dx0 - 2 as f64 * (1.0f64 / 3.0f64);
                dy_ext1 = dy0 - 2 as f64 * (1.0f64 / 3.0f64);
                dz_ext1 = dz0 - 2 as f64 * (1.0f64 / 3.0f64);
                xsv_ext1 = xsb;
                ysv_ext1 = ysb;
                zsv_ext1 = zsb;
                if c2 as i32 & 0x1 != 0 {
                    dx_ext1 -= 2 as f64;
                    xsv_ext1 += 2;
                } else if c2 as i32 & 0x2 != 0 {
                    dy_ext1 -= 2 as f64;
                    ysv_ext1 += 2;
                } else {
                    dz_ext1 -= 2 as f64;
                    zsv_ext1 += 2;
                }
            }
            let mut dx1_1 = dx0 - 1 as f64 - 1.0f64 / 3.0f64;
            let mut dy1_1 = dy0 - 0 as f64 - 1.0f64 / 3.0f64;
            let mut dz1_1 = dz0 - 0 as f64 - 1.0f64 / 3.0f64;
            let mut attn1_1 = 2 as f64 - dx1_1 * dx1_1 - dy1_1 * dy1_1 - dz1_1 * dz1_1;
            if attn1_1 > 0 as f64 {
                attn1_1 *= attn1_1;
                value += attn1_1
                    * attn1_1
                    * extrapolate3(ctx, xsb + 1, ysb + 0, zsb + 0, dx1_1, dy1_1, dz1_1);
            }
            let mut dx2_1 = dx0 - 0 as f64 - 1.0f64 / 3.0f64;
            let mut dy2_1 = dy0 - 1 as f64 - 1.0f64 / 3.0f64;
            let mut dz2_1 = dz1_1;
            let mut attn2_1 = 2 as f64 - dx2_1 * dx2_1 - dy2_1 * dy2_1 - dz2_1 * dz2_1;
            if attn2_1 > 0 as f64 {
                attn2_1 *= attn2_1;
                value += attn2_1
                    * attn2_1
                    * extrapolate3(ctx, xsb + 0, ysb + 1, zsb + 0, dx2_1, dy2_1, dz2_1);
            }
            let mut dx3_1 = dx2_1;
            let mut dy3_1 = dy1_1;
            let mut dz3_1 = dz0 - 1 as f64 - 1.0f64 / 3.0f64;
            let mut attn3_1 = 2 as f64 - dx3_1 * dx3_1 - dy3_1 * dy3_1 - dz3_1 * dz3_1;
            if attn3_1 > 0 as f64 {
                attn3_1 *= attn3_1;
                value += attn3_1
                    * attn3_1
                    * extrapolate3(ctx, xsb + 0, ysb + 0, zsb + 1, dx3_1, dy3_1, dz3_1);
            }
            let mut dx4 = dx0 - 1 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
            let mut dy4 = dy0 - 1 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
            let mut dz4 = dz0 - 0 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
            let mut attn4 = 2 as f64 - dx4 * dx4 - dy4 * dy4 - dz4 * dz4;
            if attn4 > 0 as f64 {
                attn4 *= attn4;
                value +=
                    attn4 * attn4 * extrapolate3(ctx, xsb + 1, ysb + 1, zsb + 0, dx4, dy4, dz4);
            }
            let mut dx5 = dx4;
            let mut dy5 = dy0 - 0 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
            let mut dz5 = dz0 - 1 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
            let mut attn5 = 2 as f64 - dx5 * dx5 - dy5 * dy5 - dz5 * dz5;
            if attn5 > 0 as f64 {
                attn5 *= attn5;
                value +=
                    attn5 * attn5 * extrapolate3(ctx, xsb + 1, ysb + 0, zsb + 1, dx5, dy5, dz5);
            }
            let mut dx6 = dx0 - 0 as f64 - 2 as f64 * (1.0f64 / 3.0f64);
            let mut dy6 = dy4;
            let mut dz6 = dz5;
            let mut attn6 = 2 as f64 - dx6 * dx6 - dy6 * dy6 - dz6 * dz6;
            if attn6 > 0 as f64 {
                attn6 *= attn6;
                value +=
                    attn6 * attn6 * extrapolate3(ctx, xsb + 0, ysb + 1, zsb + 1, dx6, dy6, dz6);
            }
        }
        let mut attn_ext0 = 2 as f64 - dx_ext0 * dx_ext0 - dy_ext0 * dy_ext0 - dz_ext0 * dz_ext0;
        if attn_ext0 > 0 as f64 {
            attn_ext0 *= attn_ext0;
            value += attn_ext0
                * attn_ext0
                * extrapolate3(ctx, xsv_ext0, ysv_ext0, zsv_ext0, dx_ext0, dy_ext0, dz_ext0);
        }
        let mut attn_ext1 = 2 as f64 - dx_ext1 * dx_ext1 - dy_ext1 * dy_ext1 - dz_ext1 * dz_ext1;
        if attn_ext1 > 0 as f64 {
            attn_ext1 *= attn_ext1;
            value += attn_ext1
                * attn_ext1
                * extrapolate3(ctx, xsv_ext1, ysv_ext1, zsv_ext1, dx_ext1, dy_ext1, dz_ext1);
        }
        return value / 103.0f64;
    }
}

#[no_mangle]
pub extern "C" fn open_simplex_noise4(
    mut ctx: *mut osn_context,
    mut x: f64,
    mut y: f64,
    mut z: f64,
    mut w: f64,
) -> f64 {
    unsafe {
        let mut stretchOffset = (x + y + z + w) * -0.138196601125011f64;
        let mut xs = x + stretchOffset;
        let mut ys = y + stretchOffset;
        let mut zs = z + stretchOffset;
        let mut ws = w + stretchOffset;
        let mut xsb = fastFloor(xs);
        let mut ysb = fastFloor(ys);
        let mut zsb = fastFloor(zs);
        let mut wsb = fastFloor(ws);
        let mut squishOffset = (xsb + ysb + zsb + wsb) as f64 * 0.309016994374947f64;
        let mut xb = xsb as f64 + squishOffset;
        let mut yb = ysb as f64 + squishOffset;
        let mut zb = zsb as f64 + squishOffset;
        let mut wb = wsb as f64 + squishOffset;
        let mut xins = xs - xsb as f64;
        let mut yins = ys - ysb as f64;
        let mut zins = zs - zsb as f64;
        let mut wins = ws - wsb as f64;
        let mut inSum = xins + yins + zins + wins;
        let mut dx0 = x - xb;
        let mut dy0 = y - yb;
        let mut dz0 = z - zb;
        let mut dw0 = w - wb;
        let mut dx_ext0: f64 = 0.;
        let mut dy_ext0: f64 = 0.;
        let mut dz_ext0: f64 = 0.;
        let mut dw_ext0: f64 = 0.;
        let mut dx_ext1: f64 = 0.;
        let mut dy_ext1: f64 = 0.;
        let mut dz_ext1: f64 = 0.;
        let mut dw_ext1: f64 = 0.;
        let mut dx_ext2: f64 = 0.;
        let mut dy_ext2: f64 = 0.;
        let mut dz_ext2: f64 = 0.;
        let mut dw_ext2: f64 = 0.;
        let mut xsv_ext0: i32 = 0;
        let mut ysv_ext0: i32 = 0;
        let mut zsv_ext0: i32 = 0;
        let mut wsv_ext0: i32 = 0;
        let mut xsv_ext1: i32 = 0;
        let mut ysv_ext1: i32 = 0;
        let mut zsv_ext1: i32 = 0;
        let mut wsv_ext1: i32 = 0;
        let mut xsv_ext2: i32 = 0;
        let mut ysv_ext2: i32 = 0;
        let mut zsv_ext2: i32 = 0;
        let mut wsv_ext2: i32 = 0;
        let mut value = 0 as f64;
        if inSum <= 1 as f64 {
            let mut aPoint = 0x1;
            let mut aScore = xins;
            let mut bPoint = 0x2;
            let mut bScore = yins;
            if aScore >= bScore && zins > bScore {
                bScore = zins;
                bPoint = 0x4;
            } else if aScore < bScore && zins > aScore {
                aScore = zins;
                aPoint = 0x4;
            }
            if aScore >= bScore && wins > bScore {
                bScore = wins;
                bPoint = 0x8;
            } else if aScore < bScore && wins > aScore {
                aScore = wins;
                aPoint = 0x8;
            }
            let mut uins = 1 as f64 - inSum;
            if uins > aScore || uins > bScore {
                let mut c = (if bScore > aScore {
                    bPoint as i32
                } else {
                    aPoint as i32
                }) as i8;
                if c as i32 & 0x1 == 0 {
                    xsv_ext0 = xsb - 1;
                    xsv_ext2 = xsb;
                    xsv_ext1 = xsv_ext2;
                    dx_ext0 = dx0 + 1 as f64;
                    dx_ext2 = dx0;
                    dx_ext1 = dx_ext2;
                } else {
                    xsv_ext2 = xsb + 1;
                    xsv_ext1 = xsv_ext2;
                    xsv_ext0 = xsv_ext1;
                    dx_ext2 = dx0 - 1 as f64;
                    dx_ext1 = dx_ext2;
                    dx_ext0 = dx_ext1;
                }
                if c as i32 & 0x2 == 0 {
                    ysv_ext2 = ysb;
                    ysv_ext1 = ysv_ext2;
                    ysv_ext0 = ysv_ext1;
                    dy_ext2 = dy0;
                    dy_ext1 = dy_ext2;
                    dy_ext0 = dy_ext1;
                    if c as i32 & 0x1 == 0x1 {
                        ysv_ext0 -= 1;
                        dy_ext0 += 1 as f64;
                    } else {
                        ysv_ext1 -= 1;
                        dy_ext1 += 1 as f64;
                    }
                } else {
                    ysv_ext2 = ysb + 1;
                    ysv_ext1 = ysv_ext2;
                    ysv_ext0 = ysv_ext1;
                    dy_ext2 = dy0 - 1 as f64;
                    dy_ext1 = dy_ext2;
                    dy_ext0 = dy_ext1;
                }
                if c as i32 & 0x4 == 0 {
                    zsv_ext2 = zsb;
                    zsv_ext1 = zsv_ext2;
                    zsv_ext0 = zsv_ext1;
                    dz_ext2 = dz0;
                    dz_ext1 = dz_ext2;
                    dz_ext0 = dz_ext1;
                    if c as i32 & 0x3 != 0 {
                        if c as i32 & 0x3 == 0x3 {
                            zsv_ext0 -= 1;
                            dz_ext0 += 1 as f64;
                        } else {
                            zsv_ext1 -= 1;
                            dz_ext1 += 1 as f64;
                        }
                    } else {
                        zsv_ext2 -= 1;
                        dz_ext2 += 1 as f64;
                    }
                } else {
                    zsv_ext2 = zsb + 1;
                    zsv_ext1 = zsv_ext2;
                    zsv_ext0 = zsv_ext1;
                    dz_ext2 = dz0 - 1 as f64;
                    dz_ext1 = dz_ext2;
                    dz_ext0 = dz_ext1;
                }
                if c as i32 & 0x8 == 0 {
                    wsv_ext1 = wsb;
                    wsv_ext0 = wsv_ext1;
                    wsv_ext2 = wsb - 1;
                    dw_ext1 = dw0;
                    dw_ext0 = dw_ext1;
                    dw_ext2 = dw0 + 1 as f64;
                } else {
                    wsv_ext2 = wsb + 1;
                    wsv_ext1 = wsv_ext2;
                    wsv_ext0 = wsv_ext1;
                    dw_ext2 = dw0 - 1 as f64;
                    dw_ext1 = dw_ext2;
                    dw_ext0 = dw_ext1;
                }
            } else {
                let mut c_0 = (aPoint as i32 | bPoint as i32) as i8;
                if c_0 as i32 & 0x1 == 0 {
                    xsv_ext2 = xsb;
                    xsv_ext0 = xsv_ext2;
                    xsv_ext1 = xsb - 1;
                    dx_ext0 = dx0 - 2 as f64 * 0.309016994374947f64;
                    dx_ext1 = dx0 + 1 as f64 - 0.309016994374947f64;
                    dx_ext2 = dx0 - 0.309016994374947f64;
                } else {
                    xsv_ext2 = xsb + 1;
                    xsv_ext1 = xsv_ext2;
                    xsv_ext0 = xsv_ext1;
                    dx_ext0 = dx0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    dx_ext2 = dx0 - 1 as f64 - 0.309016994374947f64;
                    dx_ext1 = dx_ext2;
                }
                if c_0 as i32 & 0x2 == 0 {
                    ysv_ext2 = ysb;
                    ysv_ext1 = ysv_ext2;
                    ysv_ext0 = ysv_ext1;
                    dy_ext0 = dy0 - 2 as f64 * 0.309016994374947f64;
                    dy_ext2 = dy0 - 0.309016994374947f64;
                    dy_ext1 = dy_ext2;
                    if c_0 as i32 & 0x1 == 0x1 {
                        ysv_ext1 -= 1;
                        dy_ext1 += 1 as f64;
                    } else {
                        ysv_ext2 -= 1;
                        dy_ext2 += 1 as f64;
                    }
                } else {
                    ysv_ext2 = ysb + 1;
                    ysv_ext1 = ysv_ext2;
                    ysv_ext0 = ysv_ext1;
                    dy_ext0 = dy0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    dy_ext2 = dy0 - 1 as f64 - 0.309016994374947f64;
                    dy_ext1 = dy_ext2;
                }
                if c_0 as i32 & 0x4 == 0 {
                    zsv_ext2 = zsb;
                    zsv_ext1 = zsv_ext2;
                    zsv_ext0 = zsv_ext1;
                    dz_ext0 = dz0 - 2 as f64 * 0.309016994374947f64;
                    dz_ext2 = dz0 - 0.309016994374947f64;
                    dz_ext1 = dz_ext2;
                    if c_0 as i32 & 0x3 == 0x3 {
                        zsv_ext1 -= 1;
                        dz_ext1 += 1 as f64;
                    } else {
                        zsv_ext2 -= 1;
                        dz_ext2 += 1 as f64;
                    }
                } else {
                    zsv_ext2 = zsb + 1;
                    zsv_ext1 = zsv_ext2;
                    zsv_ext0 = zsv_ext1;
                    dz_ext0 = dz0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    dz_ext2 = dz0 - 1 as f64 - 0.309016994374947f64;
                    dz_ext1 = dz_ext2;
                }
                if c_0 as i32 & 0x8 == 0 {
                    wsv_ext1 = wsb;
                    wsv_ext0 = wsv_ext1;
                    wsv_ext2 = wsb - 1;
                    dw_ext0 = dw0 - 2 as f64 * 0.309016994374947f64;
                    dw_ext1 = dw0 - 0.309016994374947f64;
                    dw_ext2 = dw0 + 1 as f64 - 0.309016994374947f64;
                } else {
                    wsv_ext2 = wsb + 1;
                    wsv_ext1 = wsv_ext2;
                    wsv_ext0 = wsv_ext1;
                    dw_ext0 = dw0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    dw_ext2 = dw0 - 1 as f64 - 0.309016994374947f64;
                    dw_ext1 = dw_ext2;
                }
            }
            let mut attn0 = 2 as f64 - dx0 * dx0 - dy0 * dy0 - dz0 * dz0 - dw0 * dw0;
            if attn0 > 0 as f64 {
                attn0 *= attn0;
                value += attn0
                    * attn0
                    * extrapolate4(ctx, xsb + 0, ysb + 0, zsb + 0, wsb + 0, dx0, dy0, dz0, dw0);
            }
            let mut dx1 = dx0 - 1 as f64 - 0.309016994374947f64;
            let mut dy1 = dy0 - 0 as f64 - 0.309016994374947f64;
            let mut dz1 = dz0 - 0 as f64 - 0.309016994374947f64;
            let mut dw1 = dw0 - 0 as f64 - 0.309016994374947f64;
            let mut attn1 = 2 as f64 - dx1 * dx1 - dy1 * dy1 - dz1 * dz1 - dw1 * dw1;
            if attn1 > 0 as f64 {
                attn1 *= attn1;
                value += attn1
                    * attn1
                    * extrapolate4(ctx, xsb + 1, ysb + 0, zsb + 0, wsb + 0, dx1, dy1, dz1, dw1);
            }
            let mut dx2 = dx0 - 0 as f64 - 0.309016994374947f64;
            let mut dy2 = dy0 - 1 as f64 - 0.309016994374947f64;
            let mut dz2 = dz1;
            let mut dw2 = dw1;
            let mut attn2 = 2 as f64 - dx2 * dx2 - dy2 * dy2 - dz2 * dz2 - dw2 * dw2;
            if attn2 > 0 as f64 {
                attn2 *= attn2;
                value += attn2
                    * attn2
                    * extrapolate4(ctx, xsb + 0, ysb + 1, zsb + 0, wsb + 0, dx2, dy2, dz2, dw2);
            }
            let mut dx3 = dx2;
            let mut dy3 = dy1;
            let mut dz3 = dz0 - 1 as f64 - 0.309016994374947f64;
            let mut dw3 = dw1;
            let mut attn3 = 2 as f64 - dx3 * dx3 - dy3 * dy3 - dz3 * dz3 - dw3 * dw3;
            if attn3 > 0 as f64 {
                attn3 *= attn3;
                value += attn3
                    * attn3
                    * extrapolate4(ctx, xsb + 0, ysb + 0, zsb + 1, wsb + 0, dx3, dy3, dz3, dw3);
            }
            let mut dx4 = dx2;
            let mut dy4 = dy1;
            let mut dz4 = dz1;
            let mut dw4 = dw0 - 1 as f64 - 0.309016994374947f64;
            let mut attn4 = 2 as f64 - dx4 * dx4 - dy4 * dy4 - dz4 * dz4 - dw4 * dw4;
            if attn4 > 0 as f64 {
                attn4 *= attn4;
                value += attn4
                    * attn4
                    * extrapolate4(ctx, xsb + 0, ysb + 0, zsb + 0, wsb + 1, dx4, dy4, dz4, dw4);
            }
        } else if inSum >= 3 as f64 {
            let mut aPoint_0 = 0xe;
            let mut aScore_0 = xins;
            let mut bPoint_0 = 0xd;
            let mut bScore_0 = yins;
            if aScore_0 <= bScore_0 && zins < bScore_0 {
                bScore_0 = zins;
                bPoint_0 = 0xb;
            } else if aScore_0 > bScore_0 && zins < aScore_0 {
                aScore_0 = zins;
                aPoint_0 = 0xb;
            }
            if aScore_0 <= bScore_0 && wins < bScore_0 {
                bScore_0 = wins;
                bPoint_0 = 0x7;
            } else if aScore_0 > bScore_0 && wins < aScore_0 {
                aScore_0 = wins;
                aPoint_0 = 0x7;
            }
            let mut uins_0 = 4 as f64 - inSum;
            if uins_0 < aScore_0 || uins_0 < bScore_0 {
                let mut c_1 = (if bScore_0 < aScore_0 {
                    bPoint_0 as i32
                } else {
                    aPoint_0 as i32
                }) as i8;
                if c_1 as i32 & 0x1 != 0 {
                    xsv_ext0 = xsb + 2;
                    xsv_ext2 = xsb + 1;
                    xsv_ext1 = xsv_ext2;
                    dx_ext0 = dx0 - 2 as f64 - 4 as f64 * 0.309016994374947f64;
                    dx_ext2 = dx0 - 1 as f64 - 4 as f64 * 0.309016994374947f64;
                    dx_ext1 = dx_ext2;
                } else {
                    xsv_ext2 = xsb;
                    xsv_ext1 = xsv_ext2;
                    xsv_ext0 = xsv_ext1;
                    dx_ext2 = dx0 - 4 as f64 * 0.309016994374947f64;
                    dx_ext1 = dx_ext2;
                    dx_ext0 = dx_ext1;
                }
                if c_1 as i32 & 0x2 != 0 {
                    ysv_ext2 = ysb + 1;
                    ysv_ext1 = ysv_ext2;
                    ysv_ext0 = ysv_ext1;
                    dy_ext2 = dy0 - 1 as f64 - 4 as f64 * 0.309016994374947f64;
                    dy_ext1 = dy_ext2;
                    dy_ext0 = dy_ext1;
                    if c_1 as i32 & 0x1 != 0 {
                        ysv_ext1 += 1;
                        dy_ext1 -= 1 as f64;
                    } else {
                        ysv_ext0 += 1;
                        dy_ext0 -= 1 as f64;
                    }
                } else {
                    ysv_ext2 = ysb;
                    ysv_ext1 = ysv_ext2;
                    ysv_ext0 = ysv_ext1;
                    dy_ext2 = dy0 - 4 as f64 * 0.309016994374947f64;
                    dy_ext1 = dy_ext2;
                    dy_ext0 = dy_ext1;
                }
                if c_1 as i32 & 0x4 != 0 {
                    zsv_ext2 = zsb + 1;
                    zsv_ext1 = zsv_ext2;
                    zsv_ext0 = zsv_ext1;
                    dz_ext2 = dz0 - 1 as f64 - 4 as f64 * 0.309016994374947f64;
                    dz_ext1 = dz_ext2;
                    dz_ext0 = dz_ext1;
                    if c_1 as i32 & 0x3 != 0x3 {
                        if c_1 as i32 & 0x3 == 0 {
                            zsv_ext0 += 1;
                            dz_ext0 -= 1 as f64;
                        } else {
                            zsv_ext1 += 1;
                            dz_ext1 -= 1 as f64;
                        }
                    } else {
                        zsv_ext2 += 1;
                        dz_ext2 -= 1 as f64;
                    }
                } else {
                    zsv_ext2 = zsb;
                    zsv_ext1 = zsv_ext2;
                    zsv_ext0 = zsv_ext1;
                    dz_ext2 = dz0 - 4 as f64 * 0.309016994374947f64;
                    dz_ext1 = dz_ext2;
                    dz_ext0 = dz_ext1;
                }
                if c_1 as i32 & 0x8 != 0 {
                    wsv_ext1 = wsb + 1;
                    wsv_ext0 = wsv_ext1;
                    wsv_ext2 = wsb + 2;
                    dw_ext1 = dw0 - 1 as f64 - 4 as f64 * 0.309016994374947f64;
                    dw_ext0 = dw_ext1;
                    dw_ext2 = dw0 - 2 as f64 - 4 as f64 * 0.309016994374947f64;
                } else {
                    wsv_ext2 = wsb;
                    wsv_ext1 = wsv_ext2;
                    wsv_ext0 = wsv_ext1;
                    dw_ext2 = dw0 - 4 as f64 * 0.309016994374947f64;
                    dw_ext1 = dw_ext2;
                    dw_ext0 = dw_ext1;
                }
            } else {
                let mut c_2 = (aPoint_0 as i32 & bPoint_0 as i32) as i8;
                if c_2 as i32 & 0x1 != 0 {
                    xsv_ext2 = xsb + 1;
                    xsv_ext0 = xsv_ext2;
                    xsv_ext1 = xsb + 2;
                    dx_ext0 = dx0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    dx_ext1 = dx0 - 2 as f64 - 3 as f64 * 0.309016994374947f64;
                    dx_ext2 = dx0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                } else {
                    xsv_ext2 = xsb;
                    xsv_ext1 = xsv_ext2;
                    xsv_ext0 = xsv_ext1;
                    dx_ext0 = dx0 - 2 as f64 * 0.309016994374947f64;
                    dx_ext2 = dx0 - 3 as f64 * 0.309016994374947f64;
                    dx_ext1 = dx_ext2;
                }
                if c_2 as i32 & 0x2 != 0 {
                    ysv_ext2 = ysb + 1;
                    ysv_ext1 = ysv_ext2;
                    ysv_ext0 = ysv_ext1;
                    dy_ext0 = dy0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    dy_ext2 = dy0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                    dy_ext1 = dy_ext2;
                    if c_2 as i32 & 0x1 != 0 {
                        ysv_ext2 += 1;
                        dy_ext2 -= 1 as f64;
                    } else {
                        ysv_ext1 += 1;
                        dy_ext1 -= 1 as f64;
                    }
                } else {
                    ysv_ext2 = ysb;
                    ysv_ext1 = ysv_ext2;
                    ysv_ext0 = ysv_ext1;
                    dy_ext0 = dy0 - 2 as f64 * 0.309016994374947f64;
                    dy_ext2 = dy0 - 3 as f64 * 0.309016994374947f64;
                    dy_ext1 = dy_ext2;
                }
                if c_2 as i32 & 0x4 != 0 {
                    zsv_ext2 = zsb + 1;
                    zsv_ext1 = zsv_ext2;
                    zsv_ext0 = zsv_ext1;
                    dz_ext0 = dz0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    dz_ext2 = dz0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                    dz_ext1 = dz_ext2;
                    if c_2 as i32 & 0x3 != 0 {
                        zsv_ext2 += 1;
                        dz_ext2 -= 1 as f64;
                    } else {
                        zsv_ext1 += 1;
                        dz_ext1 -= 1 as f64;
                    }
                } else {
                    zsv_ext2 = zsb;
                    zsv_ext1 = zsv_ext2;
                    zsv_ext0 = zsv_ext1;
                    dz_ext0 = dz0 - 2 as f64 * 0.309016994374947f64;
                    dz_ext2 = dz0 - 3 as f64 * 0.309016994374947f64;
                    dz_ext1 = dz_ext2;
                }
                if c_2 as i32 & 0x8 != 0 {
                    wsv_ext1 = wsb + 1;
                    wsv_ext0 = wsv_ext1;
                    wsv_ext2 = wsb + 2;
                    dw_ext0 = dw0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    dw_ext1 = dw0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                    dw_ext2 = dw0 - 2 as f64 - 3 as f64 * 0.309016994374947f64;
                } else {
                    wsv_ext2 = wsb;
                    wsv_ext1 = wsv_ext2;
                    wsv_ext0 = wsv_ext1;
                    dw_ext0 = dw0 - 2 as f64 * 0.309016994374947f64;
                    dw_ext2 = dw0 - 3 as f64 * 0.309016994374947f64;
                    dw_ext1 = dw_ext2;
                }
            }
            let mut dx4_0 = dx0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
            let mut dy4_0 = dy0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
            let mut dz4_0 = dz0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
            let mut dw4_0 = dw0 - 3 as f64 * 0.309016994374947f64;
            let mut attn4_0 =
                2 as f64 - dx4_0 * dx4_0 - dy4_0 * dy4_0 - dz4_0 * dz4_0 - dw4_0 * dw4_0;
            if attn4_0 > 0 as f64 {
                attn4_0 *= attn4_0;
                value += attn4_0
                    * attn4_0
                    * extrapolate4(
                        ctx,
                        xsb + 1,
                        ysb + 1,
                        zsb + 1,
                        wsb + 0,
                        dx4_0,
                        dy4_0,
                        dz4_0,
                        dw4_0,
                    );
            }
            let mut dx3_0 = dx4_0;
            let mut dy3_0 = dy4_0;
            let mut dz3_0 = dz0 - 3 as f64 * 0.309016994374947f64;
            let mut dw3_0 = dw0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
            let mut attn3_0 =
                2 as f64 - dx3_0 * dx3_0 - dy3_0 * dy3_0 - dz3_0 * dz3_0 - dw3_0 * dw3_0;
            if attn3_0 > 0 as f64 {
                attn3_0 *= attn3_0;
                value += attn3_0
                    * attn3_0
                    * extrapolate4(
                        ctx,
                        xsb + 1,
                        ysb + 1,
                        zsb + 0,
                        wsb + 1,
                        dx3_0,
                        dy3_0,
                        dz3_0,
                        dw3_0,
                    );
            }
            let mut dx2_0 = dx4_0;
            let mut dy2_0 = dy0 - 3 as f64 * 0.309016994374947f64;
            let mut dz2_0 = dz4_0;
            let mut dw2_0 = dw3_0;
            let mut attn2_0 =
                2 as f64 - dx2_0 * dx2_0 - dy2_0 * dy2_0 - dz2_0 * dz2_0 - dw2_0 * dw2_0;
            if attn2_0 > 0 as f64 {
                attn2_0 *= attn2_0;
                value += attn2_0
                    * attn2_0
                    * extrapolate4(
                        ctx,
                        xsb + 1,
                        ysb + 0,
                        zsb + 1,
                        wsb + 1,
                        dx2_0,
                        dy2_0,
                        dz2_0,
                        dw2_0,
                    );
            }
            let mut dx1_0 = dx0 - 3 as f64 * 0.309016994374947f64;
            let mut dz1_0 = dz4_0;
            let mut dy1_0 = dy4_0;
            let mut dw1_0 = dw3_0;
            let mut attn1_0 =
                2 as f64 - dx1_0 * dx1_0 - dy1_0 * dy1_0 - dz1_0 * dz1_0 - dw1_0 * dw1_0;
            if attn1_0 > 0 as f64 {
                attn1_0 *= attn1_0;
                value += attn1_0
                    * attn1_0
                    * extrapolate4(
                        ctx,
                        xsb + 0,
                        ysb + 1,
                        zsb + 1,
                        wsb + 1,
                        dx1_0,
                        dy1_0,
                        dz1_0,
                        dw1_0,
                    );
            }
            dx0 = dx0 - 1 as f64 - 4 as f64 * 0.309016994374947f64;
            dy0 = dy0 - 1 as f64 - 4 as f64 * 0.309016994374947f64;
            dz0 = dz0 - 1 as f64 - 4 as f64 * 0.309016994374947f64;
            dw0 = dw0 - 1 as f64 - 4 as f64 * 0.309016994374947f64;
            let mut attn0_0 = 2 as f64 - dx0 * dx0 - dy0 * dy0 - dz0 * dz0 - dw0 * dw0;
            if attn0_0 > 0 as f64 {
                attn0_0 *= attn0_0;
                value += attn0_0
                    * attn0_0
                    * extrapolate4(ctx, xsb + 1, ysb + 1, zsb + 1, wsb + 1, dx0, dy0, dz0, dw0);
            }
        } else if inSum <= 2 as f64 {
            let mut aScore_1: f64 = 0.;
            let mut aPoint_1: i8 = 0;
            let mut aIsBiggerSide = 1;
            let mut bScore_1: f64 = 0.;
            let mut bPoint_1: i8 = 0;
            let mut bIsBiggerSide = 1;
            if xins + yins > zins + wins {
                aScore_1 = xins + yins;
                aPoint_1 = 0x3;
            } else {
                aScore_1 = zins + wins;
                aPoint_1 = 0xc;
            }
            if xins + zins > yins + wins {
                bScore_1 = xins + zins;
                bPoint_1 = 0x5;
            } else {
                bScore_1 = yins + wins;
                bPoint_1 = 0xa;
            }
            if xins + wins > yins + zins {
                let mut score = xins + wins;
                if aScore_1 >= bScore_1 && score > bScore_1 {
                    bScore_1 = score;
                    bPoint_1 = 0x9;
                } else if aScore_1 < bScore_1 && score > aScore_1 {
                    aScore_1 = score;
                    aPoint_1 = 0x9;
                }
            } else {
                let mut score_0 = yins + zins;
                if aScore_1 >= bScore_1 && score_0 > bScore_1 {
                    bScore_1 = score_0;
                    bPoint_1 = 0x6;
                } else if aScore_1 < bScore_1 && score_0 > aScore_1 {
                    aScore_1 = score_0;
                    aPoint_1 = 0x6;
                }
            }
            let mut p1 = 2 as f64 - inSum + xins;
            if aScore_1 >= bScore_1 && p1 > bScore_1 {
                bScore_1 = p1;
                bPoint_1 = 0x1;
                bIsBiggerSide = 0;
            } else if aScore_1 < bScore_1 && p1 > aScore_1 {
                aScore_1 = p1;
                aPoint_1 = 0x1;
                aIsBiggerSide = 0;
            }
            let mut p2 = 2 as f64 - inSum + yins;
            if aScore_1 >= bScore_1 && p2 > bScore_1 {
                bScore_1 = p2;
                bPoint_1 = 0x2;
                bIsBiggerSide = 0;
            } else if aScore_1 < bScore_1 && p2 > aScore_1 {
                aScore_1 = p2;
                aPoint_1 = 0x2;
                aIsBiggerSide = 0;
            }
            let mut p3 = 2 as f64 - inSum + zins;
            if aScore_1 >= bScore_1 && p3 > bScore_1 {
                bScore_1 = p3;
                bPoint_1 = 0x4;
                bIsBiggerSide = 0;
            } else if aScore_1 < bScore_1 && p3 > aScore_1 {
                aScore_1 = p3;
                aPoint_1 = 0x4;
                aIsBiggerSide = 0;
            }
            let mut p4 = 2 as f64 - inSum + wins;
            if aScore_1 >= bScore_1 && p4 > bScore_1 {
                bScore_1 = p4;
                bPoint_1 = 0x8;
                bIsBiggerSide = 0;
            } else if aScore_1 < bScore_1 && p4 > aScore_1 {
                aScore_1 = p4;
                aPoint_1 = 0x8;
                aIsBiggerSide = 0;
            }
            if aIsBiggerSide == bIsBiggerSide {
                if aIsBiggerSide != 0 {
                    let mut c1 = (aPoint_1 as i32 | bPoint_1 as i32) as i8;
                    let mut c2 = (aPoint_1 as i32 & bPoint_1 as i32) as i8;
                    if c1 as i32 & 0x1 == 0 {
                        xsv_ext0 = xsb;
                        xsv_ext1 = xsb - 1;
                        dx_ext0 = dx0 - 3 as f64 * 0.309016994374947f64;
                        dx_ext1 = dx0 + 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    } else {
                        xsv_ext1 = xsb + 1;
                        xsv_ext0 = xsv_ext1;
                        dx_ext0 = dx0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                        dx_ext1 = dx0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    }
                    if c1 as i32 & 0x2 == 0 {
                        ysv_ext0 = ysb;
                        ysv_ext1 = ysb - 1;
                        dy_ext0 = dy0 - 3 as f64 * 0.309016994374947f64;
                        dy_ext1 = dy0 + 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    } else {
                        ysv_ext1 = ysb + 1;
                        ysv_ext0 = ysv_ext1;
                        dy_ext0 = dy0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                        dy_ext1 = dy0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    }
                    if c1 as i32 & 0x4 == 0 {
                        zsv_ext0 = zsb;
                        zsv_ext1 = zsb - 1;
                        dz_ext0 = dz0 - 3 as f64 * 0.309016994374947f64;
                        dz_ext1 = dz0 + 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    } else {
                        zsv_ext1 = zsb + 1;
                        zsv_ext0 = zsv_ext1;
                        dz_ext0 = dz0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                        dz_ext1 = dz0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    }
                    if c1 as i32 & 0x8 == 0 {
                        wsv_ext0 = wsb;
                        wsv_ext1 = wsb - 1;
                        dw_ext0 = dw0 - 3 as f64 * 0.309016994374947f64;
                        dw_ext1 = dw0 + 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    } else {
                        wsv_ext1 = wsb + 1;
                        wsv_ext0 = wsv_ext1;
                        dw_ext0 = dw0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                        dw_ext1 = dw0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    }
                    xsv_ext2 = xsb;
                    ysv_ext2 = ysb;
                    zsv_ext2 = zsb;
                    wsv_ext2 = wsb;
                    dx_ext2 = dx0 - 2 as f64 * 0.309016994374947f64;
                    dy_ext2 = dy0 - 2 as f64 * 0.309016994374947f64;
                    dz_ext2 = dz0 - 2 as f64 * 0.309016994374947f64;
                    dw_ext2 = dw0 - 2 as f64 * 0.309016994374947f64;
                    if c2 as i32 & 0x1 != 0 {
                        xsv_ext2 += 2;
                        dx_ext2 -= 2 as f64;
                    } else if c2 as i32 & 0x2 != 0 {
                        ysv_ext2 += 2;
                        dy_ext2 -= 2 as f64;
                    } else if c2 as i32 & 0x4 != 0 {
                        zsv_ext2 += 2;
                        dz_ext2 -= 2 as f64;
                    } else {
                        wsv_ext2 += 2;
                        dw_ext2 -= 2 as f64;
                    }
                } else {
                    xsv_ext2 = xsb;
                    ysv_ext2 = ysb;
                    zsv_ext2 = zsb;
                    wsv_ext2 = wsb;
                    dx_ext2 = dx0;
                    dy_ext2 = dy0;
                    dz_ext2 = dz0;
                    dw_ext2 = dw0;
                    let mut c_3 = (aPoint_1 as i32 | bPoint_1 as i32) as i8;
                    if c_3 as i32 & 0x1 == 0 {
                        xsv_ext0 = xsb - 1;
                        xsv_ext1 = xsb;
                        dx_ext0 = dx0 + 1 as f64 - 0.309016994374947f64;
                        dx_ext1 = dx0 - 0.309016994374947f64;
                    } else {
                        xsv_ext1 = xsb + 1;
                        xsv_ext0 = xsv_ext1;
                        dx_ext1 = dx0 - 1 as f64 - 0.309016994374947f64;
                        dx_ext0 = dx_ext1;
                    }
                    if c_3 as i32 & 0x2 == 0 {
                        ysv_ext1 = ysb;
                        ysv_ext0 = ysv_ext1;
                        dy_ext1 = dy0 - 0.309016994374947f64;
                        dy_ext0 = dy_ext1;
                        if c_3 as i32 & 0x1 == 0x1 {
                            ysv_ext0 -= 1;
                            dy_ext0 += 1 as f64;
                        } else {
                            ysv_ext1 -= 1;
                            dy_ext1 += 1 as f64;
                        }
                    } else {
                        ysv_ext1 = ysb + 1;
                        ysv_ext0 = ysv_ext1;
                        dy_ext1 = dy0 - 1 as f64 - 0.309016994374947f64;
                        dy_ext0 = dy_ext1;
                    }
                    if c_3 as i32 & 0x4 == 0 {
                        zsv_ext1 = zsb;
                        zsv_ext0 = zsv_ext1;
                        dz_ext1 = dz0 - 0.309016994374947f64;
                        dz_ext0 = dz_ext1;
                        if c_3 as i32 & 0x3 == 0x3 {
                            zsv_ext0 -= 1;
                            dz_ext0 += 1 as f64;
                        } else {
                            zsv_ext1 -= 1;
                            dz_ext1 += 1 as f64;
                        }
                    } else {
                        zsv_ext1 = zsb + 1;
                        zsv_ext0 = zsv_ext1;
                        dz_ext1 = dz0 - 1 as f64 - 0.309016994374947f64;
                        dz_ext0 = dz_ext1;
                    }
                    if c_3 as i32 & 0x8 == 0 {
                        wsv_ext0 = wsb;
                        wsv_ext1 = wsb - 1;
                        dw_ext0 = dw0 - 0.309016994374947f64;
                        dw_ext1 = dw0 + 1 as f64 - 0.309016994374947f64;
                    } else {
                        wsv_ext1 = wsb + 1;
                        wsv_ext0 = wsv_ext1;
                        dw_ext1 = dw0 - 1 as f64 - 0.309016994374947f64;
                        dw_ext0 = dw_ext1;
                    }
                }
            } else {
                let mut c1_0: i8 = 0;
                let mut c2_0: i8 = 0;
                if aIsBiggerSide != 0 {
                    c1_0 = aPoint_1;
                    c2_0 = bPoint_1;
                } else {
                    c1_0 = bPoint_1;
                    c2_0 = aPoint_1;
                }
                if c1_0 as i32 & 0x1 == 0 {
                    xsv_ext0 = xsb - 1;
                    xsv_ext1 = xsb;
                    dx_ext0 = dx0 + 1 as f64 - 0.309016994374947f64;
                    dx_ext1 = dx0 - 0.309016994374947f64;
                } else {
                    xsv_ext1 = xsb + 1;
                    xsv_ext0 = xsv_ext1;
                    dx_ext1 = dx0 - 1 as f64 - 0.309016994374947f64;
                    dx_ext0 = dx_ext1;
                }
                if c1_0 as i32 & 0x2 == 0 {
                    ysv_ext1 = ysb;
                    ysv_ext0 = ysv_ext1;
                    dy_ext1 = dy0 - 0.309016994374947f64;
                    dy_ext0 = dy_ext1;
                    if c1_0 as i32 & 0x1 == 0x1 {
                        ysv_ext0 -= 1;
                        dy_ext0 += 1 as f64;
                    } else {
                        ysv_ext1 -= 1;
                        dy_ext1 += 1 as f64;
                    }
                } else {
                    ysv_ext1 = ysb + 1;
                    ysv_ext0 = ysv_ext1;
                    dy_ext1 = dy0 - 1 as f64 - 0.309016994374947f64;
                    dy_ext0 = dy_ext1;
                }
                if c1_0 as i32 & 0x4 == 0 {
                    zsv_ext1 = zsb;
                    zsv_ext0 = zsv_ext1;
                    dz_ext1 = dz0 - 0.309016994374947f64;
                    dz_ext0 = dz_ext1;
                    if c1_0 as i32 & 0x3 == 0x3 {
                        zsv_ext0 -= 1;
                        dz_ext0 += 1 as f64;
                    } else {
                        zsv_ext1 -= 1;
                        dz_ext1 += 1 as f64;
                    }
                } else {
                    zsv_ext1 = zsb + 1;
                    zsv_ext0 = zsv_ext1;
                    dz_ext1 = dz0 - 1 as f64 - 0.309016994374947f64;
                    dz_ext0 = dz_ext1;
                }
                if c1_0 as i32 & 0x8 == 0 {
                    wsv_ext0 = wsb;
                    wsv_ext1 = wsb - 1;
                    dw_ext0 = dw0 - 0.309016994374947f64;
                    dw_ext1 = dw0 + 1 as f64 - 0.309016994374947f64;
                } else {
                    wsv_ext1 = wsb + 1;
                    wsv_ext0 = wsv_ext1;
                    dw_ext1 = dw0 - 1 as f64 - 0.309016994374947f64;
                    dw_ext0 = dw_ext1;
                }
                xsv_ext2 = xsb;
                ysv_ext2 = ysb;
                zsv_ext2 = zsb;
                wsv_ext2 = wsb;
                dx_ext2 = dx0 - 2 as f64 * 0.309016994374947f64;
                dy_ext2 = dy0 - 2 as f64 * 0.309016994374947f64;
                dz_ext2 = dz0 - 2 as f64 * 0.309016994374947f64;
                dw_ext2 = dw0 - 2 as f64 * 0.309016994374947f64;
                if c2_0 as i32 & 0x1 != 0 {
                    xsv_ext2 += 2;
                    dx_ext2 -= 2 as f64;
                } else if c2_0 as i32 & 0x2 != 0 {
                    ysv_ext2 += 2;
                    dy_ext2 -= 2 as f64;
                } else if c2_0 as i32 & 0x4 != 0 {
                    zsv_ext2 += 2;
                    dz_ext2 -= 2 as f64;
                } else {
                    wsv_ext2 += 2;
                    dw_ext2 -= 2 as f64;
                }
            }
            let mut dx1_1 = dx0 - 1 as f64 - 0.309016994374947f64;
            let mut dy1_1 = dy0 - 0 as f64 - 0.309016994374947f64;
            let mut dz1_1 = dz0 - 0 as f64 - 0.309016994374947f64;
            let mut dw1_1 = dw0 - 0 as f64 - 0.309016994374947f64;
            let mut attn1_1 =
                2 as f64 - dx1_1 * dx1_1 - dy1_1 * dy1_1 - dz1_1 * dz1_1 - dw1_1 * dw1_1;
            if attn1_1 > 0 as f64 {
                attn1_1 *= attn1_1;
                value += attn1_1
                    * attn1_1
                    * extrapolate4(
                        ctx,
                        xsb + 1,
                        ysb + 0,
                        zsb + 0,
                        wsb + 0,
                        dx1_1,
                        dy1_1,
                        dz1_1,
                        dw1_1,
                    );
            }
            let mut dx2_1 = dx0 - 0 as f64 - 0.309016994374947f64;
            let mut dy2_1 = dy0 - 1 as f64 - 0.309016994374947f64;
            let mut dz2_1 = dz1_1;
            let mut dw2_1 = dw1_1;
            let mut attn2_1 =
                2 as f64 - dx2_1 * dx2_1 - dy2_1 * dy2_1 - dz2_1 * dz2_1 - dw2_1 * dw2_1;
            if attn2_1 > 0 as f64 {
                attn2_1 *= attn2_1;
                value += attn2_1
                    * attn2_1
                    * extrapolate4(
                        ctx,
                        xsb + 0,
                        ysb + 1,
                        zsb + 0,
                        wsb + 0,
                        dx2_1,
                        dy2_1,
                        dz2_1,
                        dw2_1,
                    );
            }
            let mut dx3_1 = dx2_1;
            let mut dy3_1 = dy1_1;
            let mut dz3_1 = dz0 - 1 as f64 - 0.309016994374947f64;
            let mut dw3_1 = dw1_1;
            let mut attn3_1 =
                2 as f64 - dx3_1 * dx3_1 - dy3_1 * dy3_1 - dz3_1 * dz3_1 - dw3_1 * dw3_1;
            if attn3_1 > 0 as f64 {
                attn3_1 *= attn3_1;
                value += attn3_1
                    * attn3_1
                    * extrapolate4(
                        ctx,
                        xsb + 0,
                        ysb + 0,
                        zsb + 1,
                        wsb + 0,
                        dx3_1,
                        dy3_1,
                        dz3_1,
                        dw3_1,
                    );
            }
            let mut dx4_1 = dx2_1;
            let mut dy4_1 = dy1_1;
            let mut dz4_1 = dz1_1;
            let mut dw4_1 = dw0 - 1 as f64 - 0.309016994374947f64;
            let mut attn4_1 =
                2 as f64 - dx4_1 * dx4_1 - dy4_1 * dy4_1 - dz4_1 * dz4_1 - dw4_1 * dw4_1;
            if attn4_1 > 0 as f64 {
                attn4_1 *= attn4_1;
                value += attn4_1
                    * attn4_1
                    * extrapolate4(
                        ctx,
                        xsb + 0,
                        ysb + 0,
                        zsb + 0,
                        wsb + 1,
                        dx4_1,
                        dy4_1,
                        dz4_1,
                        dw4_1,
                    );
            }
            let mut dx5 = dx0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dy5 = dy0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dz5 = dz0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dw5 = dw0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut attn5 = 2 as f64 - dx5 * dx5 - dy5 * dy5 - dz5 * dz5 - dw5 * dw5;
            if attn5 > 0 as f64 {
                attn5 *= attn5;
                value += attn5
                    * attn5
                    * extrapolate4(ctx, xsb + 1, ysb + 1, zsb + 0, wsb + 0, dx5, dy5, dz5, dw5);
            }
            let mut dx6 = dx0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dy6 = dy0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dz6 = dz0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dw6 = dw0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut attn6 = 2 as f64 - dx6 * dx6 - dy6 * dy6 - dz6 * dz6 - dw6 * dw6;
            if attn6 > 0 as f64 {
                attn6 *= attn6;
                value += attn6
                    * attn6
                    * extrapolate4(ctx, xsb + 1, ysb + 0, zsb + 1, wsb + 0, dx6, dy6, dz6, dw6);
            }
            let mut dx7 = dx0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dy7 = dy0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dz7 = dz0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dw7 = dw0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut attn7 = 2 as f64 - dx7 * dx7 - dy7 * dy7 - dz7 * dz7 - dw7 * dw7;
            if attn7 > 0 as f64 {
                attn7 *= attn7;
                value += attn7
                    * attn7
                    * extrapolate4(ctx, xsb + 1, ysb + 0, zsb + 0, wsb + 1, dx7, dy7, dz7, dw7);
            }
            let mut dx8 = dx0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dy8 = dy0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dz8 = dz0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dw8 = dw0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut attn8 = 2 as f64 - dx8 * dx8 - dy8 * dy8 - dz8 * dz8 - dw8 * dw8;
            if attn8 > 0 as f64 {
                attn8 *= attn8;
                value += attn8
                    * attn8
                    * extrapolate4(ctx, xsb + 0, ysb + 1, zsb + 1, wsb + 0, dx8, dy8, dz8, dw8);
            }
            let mut dx9 = dx0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dy9 = dy0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dz9 = dz0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dw9 = dw0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut attn9 = 2 as f64 - dx9 * dx9 - dy9 * dy9 - dz9 * dz9 - dw9 * dw9;
            if attn9 > 0 as f64 {
                attn9 *= attn9;
                value += attn9
                    * attn9
                    * extrapolate4(ctx, xsb + 0, ysb + 1, zsb + 0, wsb + 1, dx9, dy9, dz9, dw9);
            }
            let mut dx10 = dx0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dy10 = dy0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dz10 = dz0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dw10 = dw0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut attn10 = 2 as f64 - dx10 * dx10 - dy10 * dy10 - dz10 * dz10 - dw10 * dw10;
            if attn10 > 0 as f64 {
                attn10 *= attn10;
                value += attn10
                    * attn10
                    * extrapolate4(
                        ctx,
                        xsb + 0,
                        ysb + 0,
                        zsb + 1,
                        wsb + 1,
                        dx10,
                        dy10,
                        dz10,
                        dw10,
                    );
            }
        } else {
            let mut aScore_2: f64 = 0.;
            let mut aPoint_2: i8 = 0;
            let mut aIsBiggerSide_0 = 1;
            let mut bScore_2: f64 = 0.;
            let mut bPoint_2: i8 = 0;
            let mut bIsBiggerSide_0 = 1;
            if xins + yins < zins + wins {
                aScore_2 = xins + yins;
                aPoint_2 = 0xc;
            } else {
                aScore_2 = zins + wins;
                aPoint_2 = 0x3;
            }
            if xins + zins < yins + wins {
                bScore_2 = xins + zins;
                bPoint_2 = 0xa;
            } else {
                bScore_2 = yins + wins;
                bPoint_2 = 0x5;
            }
            if xins + wins < yins + zins {
                let mut score_1 = xins + wins;
                if aScore_2 <= bScore_2 && score_1 < bScore_2 {
                    bScore_2 = score_1;
                    bPoint_2 = 0x6;
                } else if aScore_2 > bScore_2 && score_1 < aScore_2 {
                    aScore_2 = score_1;
                    aPoint_2 = 0x6;
                }
            } else {
                let mut score_2 = yins + zins;
                if aScore_2 <= bScore_2 && score_2 < bScore_2 {
                    bScore_2 = score_2;
                    bPoint_2 = 0x9;
                } else if aScore_2 > bScore_2 && score_2 < aScore_2 {
                    aScore_2 = score_2;
                    aPoint_2 = 0x9;
                }
            }
            let mut p1_0 = 3 as f64 - inSum + xins;
            if aScore_2 <= bScore_2 && p1_0 < bScore_2 {
                bScore_2 = p1_0;
                bPoint_2 = 0xe;
                bIsBiggerSide_0 = 0;
            } else if aScore_2 > bScore_2 && p1_0 < aScore_2 {
                aScore_2 = p1_0;
                aPoint_2 = 0xe;
                aIsBiggerSide_0 = 0;
            }
            let mut p2_0 = 3 as f64 - inSum + yins;
            if aScore_2 <= bScore_2 && p2_0 < bScore_2 {
                bScore_2 = p2_0;
                bPoint_2 = 0xd;
                bIsBiggerSide_0 = 0;
            } else if aScore_2 > bScore_2 && p2_0 < aScore_2 {
                aScore_2 = p2_0;
                aPoint_2 = 0xd;
                aIsBiggerSide_0 = 0;
            }
            let mut p3_0 = 3 as f64 - inSum + zins;
            if aScore_2 <= bScore_2 && p3_0 < bScore_2 {
                bScore_2 = p3_0;
                bPoint_2 = 0xb;
                bIsBiggerSide_0 = 0;
            } else if aScore_2 > bScore_2 && p3_0 < aScore_2 {
                aScore_2 = p3_0;
                aPoint_2 = 0xb;
                aIsBiggerSide_0 = 0;
            }
            let mut p4_0 = 3 as f64 - inSum + wins;
            if aScore_2 <= bScore_2 && p4_0 < bScore_2 {
                bScore_2 = p4_0;
                bPoint_2 = 0x7;
                bIsBiggerSide_0 = 0;
            } else if aScore_2 > bScore_2 && p4_0 < aScore_2 {
                aScore_2 = p4_0;
                aPoint_2 = 0x7;
                aIsBiggerSide_0 = 0;
            }
            if aIsBiggerSide_0 == bIsBiggerSide_0 {
                if aIsBiggerSide_0 != 0 {
                    let mut c1_1 = (aPoint_2 as i32 & bPoint_2 as i32) as i8;
                    let mut c2_1 = (aPoint_2 as i32 | bPoint_2 as i32) as i8;
                    xsv_ext1 = xsb;
                    xsv_ext0 = xsv_ext1;
                    ysv_ext1 = ysb;
                    ysv_ext0 = ysv_ext1;
                    zsv_ext1 = zsb;
                    zsv_ext0 = zsv_ext1;
                    wsv_ext1 = wsb;
                    wsv_ext0 = wsv_ext1;
                    dx_ext0 = dx0 - 0.309016994374947f64;
                    dy_ext0 = dy0 - 0.309016994374947f64;
                    dz_ext0 = dz0 - 0.309016994374947f64;
                    dw_ext0 = dw0 - 0.309016994374947f64;
                    dx_ext1 = dx0 - 2 as f64 * 0.309016994374947f64;
                    dy_ext1 = dy0 - 2 as f64 * 0.309016994374947f64;
                    dz_ext1 = dz0 - 2 as f64 * 0.309016994374947f64;
                    dw_ext1 = dw0 - 2 as f64 * 0.309016994374947f64;
                    if c1_1 as i32 & 0x1 != 0 {
                        xsv_ext0 += 1;
                        dx_ext0 -= 1 as f64;
                        xsv_ext1 += 2;
                        dx_ext1 -= 2 as f64;
                    } else if c1_1 as i32 & 0x2 != 0 {
                        ysv_ext0 += 1;
                        dy_ext0 -= 1 as f64;
                        ysv_ext1 += 2;
                        dy_ext1 -= 2 as f64;
                    } else if c1_1 as i32 & 0x4 != 0 {
                        zsv_ext0 += 1;
                        dz_ext0 -= 1 as f64;
                        zsv_ext1 += 2;
                        dz_ext1 -= 2 as f64;
                    } else {
                        wsv_ext0 += 1;
                        dw_ext0 -= 1 as f64;
                        wsv_ext1 += 2;
                        dw_ext1 -= 2 as f64;
                    }
                    xsv_ext2 = xsb + 1;
                    ysv_ext2 = ysb + 1;
                    zsv_ext2 = zsb + 1;
                    wsv_ext2 = wsb + 1;
                    dx_ext2 = dx0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    dy_ext2 = dy0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    dz_ext2 = dz0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    dw_ext2 = dw0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                    if c2_1 as i32 & 0x1 == 0 {
                        xsv_ext2 -= 2;
                        dx_ext2 += 2 as f64;
                    } else if c2_1 as i32 & 0x2 == 0 {
                        ysv_ext2 -= 2;
                        dy_ext2 += 2 as f64;
                    } else if c2_1 as i32 & 0x4 == 0 {
                        zsv_ext2 -= 2;
                        dz_ext2 += 2 as f64;
                    } else {
                        wsv_ext2 -= 2;
                        dw_ext2 += 2 as f64;
                    }
                } else {
                    xsv_ext2 = xsb + 1;
                    ysv_ext2 = ysb + 1;
                    zsv_ext2 = zsb + 1;
                    wsv_ext2 = wsb + 1;
                    dx_ext2 = dx0 - 1 as f64 - 4 as f64 * 0.309016994374947f64;
                    dy_ext2 = dy0 - 1 as f64 - 4 as f64 * 0.309016994374947f64;
                    dz_ext2 = dz0 - 1 as f64 - 4 as f64 * 0.309016994374947f64;
                    dw_ext2 = dw0 - 1 as f64 - 4 as f64 * 0.309016994374947f64;
                    let mut c_4 = (aPoint_2 as i32 & bPoint_2 as i32) as i8;
                    if c_4 as i32 & 0x1 != 0 {
                        xsv_ext0 = xsb + 2;
                        xsv_ext1 = xsb + 1;
                        dx_ext0 = dx0 - 2 as f64 - 3 as f64 * 0.309016994374947f64;
                        dx_ext1 = dx0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                    } else {
                        xsv_ext1 = xsb;
                        xsv_ext0 = xsv_ext1;
                        dx_ext1 = dx0 - 3 as f64 * 0.309016994374947f64;
                        dx_ext0 = dx_ext1;
                    }
                    if c_4 as i32 & 0x2 != 0 {
                        ysv_ext1 = ysb + 1;
                        ysv_ext0 = ysv_ext1;
                        dy_ext1 = dy0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                        dy_ext0 = dy_ext1;
                        if c_4 as i32 & 0x1 == 0 {
                            ysv_ext0 += 1;
                            dy_ext0 -= 1 as f64;
                        } else {
                            ysv_ext1 += 1;
                            dy_ext1 -= 1 as f64;
                        }
                    } else {
                        ysv_ext1 = ysb;
                        ysv_ext0 = ysv_ext1;
                        dy_ext1 = dy0 - 3 as f64 * 0.309016994374947f64;
                        dy_ext0 = dy_ext1;
                    }
                    if c_4 as i32 & 0x4 != 0 {
                        zsv_ext1 = zsb + 1;
                        zsv_ext0 = zsv_ext1;
                        dz_ext1 = dz0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                        dz_ext0 = dz_ext1;
                        if c_4 as i32 & 0x3 == 0 {
                            zsv_ext0 += 1;
                            dz_ext0 -= 1 as f64;
                        } else {
                            zsv_ext1 += 1;
                            dz_ext1 -= 1 as f64;
                        }
                    } else {
                        zsv_ext1 = zsb;
                        zsv_ext0 = zsv_ext1;
                        dz_ext1 = dz0 - 3 as f64 * 0.309016994374947f64;
                        dz_ext0 = dz_ext1;
                    }
                    if c_4 as i32 & 0x8 != 0 {
                        wsv_ext0 = wsb + 1;
                        wsv_ext1 = wsb + 2;
                        dw_ext0 = dw0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                        dw_ext1 = dw0 - 2 as f64 - 3 as f64 * 0.309016994374947f64;
                    } else {
                        wsv_ext1 = wsb;
                        wsv_ext0 = wsv_ext1;
                        dw_ext1 = dw0 - 3 as f64 * 0.309016994374947f64;
                        dw_ext0 = dw_ext1;
                    }
                }
            } else {
                let mut c1_2: i8 = 0;
                let mut c2_2: i8 = 0;
                if aIsBiggerSide_0 != 0 {
                    c1_2 = aPoint_2;
                    c2_2 = bPoint_2;
                } else {
                    c1_2 = bPoint_2;
                    c2_2 = aPoint_2;
                }
                if c1_2 as i32 & 0x1 != 0 {
                    xsv_ext0 = xsb + 2;
                    xsv_ext1 = xsb + 1;
                    dx_ext0 = dx0 - 2 as f64 - 3 as f64 * 0.309016994374947f64;
                    dx_ext1 = dx0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                } else {
                    xsv_ext1 = xsb;
                    xsv_ext0 = xsv_ext1;
                    dx_ext1 = dx0 - 3 as f64 * 0.309016994374947f64;
                    dx_ext0 = dx_ext1;
                }
                if c1_2 as i32 & 0x2 != 0 {
                    ysv_ext1 = ysb + 1;
                    ysv_ext0 = ysv_ext1;
                    dy_ext1 = dy0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                    dy_ext0 = dy_ext1;
                    if c1_2 as i32 & 0x1 == 0 {
                        ysv_ext0 += 1;
                        dy_ext0 -= 1 as f64;
                    } else {
                        ysv_ext1 += 1;
                        dy_ext1 -= 1 as f64;
                    }
                } else {
                    ysv_ext1 = ysb;
                    ysv_ext0 = ysv_ext1;
                    dy_ext1 = dy0 - 3 as f64 * 0.309016994374947f64;
                    dy_ext0 = dy_ext1;
                }
                if c1_2 as i32 & 0x4 != 0 {
                    zsv_ext1 = zsb + 1;
                    zsv_ext0 = zsv_ext1;
                    dz_ext1 = dz0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                    dz_ext0 = dz_ext1;
                    if c1_2 as i32 & 0x3 == 0 {
                        zsv_ext0 += 1;
                        dz_ext0 -= 1 as f64;
                    } else {
                        zsv_ext1 += 1;
                        dz_ext1 -= 1 as f64;
                    }
                } else {
                    zsv_ext1 = zsb;
                    zsv_ext0 = zsv_ext1;
                    dz_ext1 = dz0 - 3 as f64 * 0.309016994374947f64;
                    dz_ext0 = dz_ext1;
                }
                if c1_2 as i32 & 0x8 != 0 {
                    wsv_ext0 = wsb + 1;
                    wsv_ext1 = wsb + 2;
                    dw_ext0 = dw0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
                    dw_ext1 = dw0 - 2 as f64 - 3 as f64 * 0.309016994374947f64;
                } else {
                    wsv_ext1 = wsb;
                    wsv_ext0 = wsv_ext1;
                    dw_ext1 = dw0 - 3 as f64 * 0.309016994374947f64;
                    dw_ext0 = dw_ext1;
                }
                xsv_ext2 = xsb + 1;
                ysv_ext2 = ysb + 1;
                zsv_ext2 = zsb + 1;
                wsv_ext2 = wsb + 1;
                dx_ext2 = dx0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                dy_ext2 = dy0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                dz_ext2 = dz0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                dw_ext2 = dw0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
                if c2_2 as i32 & 0x1 == 0 {
                    xsv_ext2 -= 2;
                    dx_ext2 += 2 as f64;
                } else if c2_2 as i32 & 0x2 == 0 {
                    ysv_ext2 -= 2;
                    dy_ext2 += 2 as f64;
                } else if c2_2 as i32 & 0x4 == 0 {
                    zsv_ext2 -= 2;
                    dz_ext2 += 2 as f64;
                } else {
                    wsv_ext2 -= 2;
                    dw_ext2 += 2 as f64;
                }
            }
            let mut dx4_2 = dx0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
            let mut dy4_2 = dy0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
            let mut dz4_2 = dz0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
            let mut dw4_2 = dw0 - 3 as f64 * 0.309016994374947f64;
            let mut attn4_2 =
                2 as f64 - dx4_2 * dx4_2 - dy4_2 * dy4_2 - dz4_2 * dz4_2 - dw4_2 * dw4_2;
            if attn4_2 > 0 as f64 {
                attn4_2 *= attn4_2;
                value += attn4_2
                    * attn4_2
                    * extrapolate4(
                        ctx,
                        xsb + 1,
                        ysb + 1,
                        zsb + 1,
                        wsb + 0,
                        dx4_2,
                        dy4_2,
                        dz4_2,
                        dw4_2,
                    );
            }
            let mut dx3_2 = dx4_2;
            let mut dy3_2 = dy4_2;
            let mut dz3_2 = dz0 - 3 as f64 * 0.309016994374947f64;
            let mut dw3_2 = dw0 - 1 as f64 - 3 as f64 * 0.309016994374947f64;
            let mut attn3_2 =
                2 as f64 - dx3_2 * dx3_2 - dy3_2 * dy3_2 - dz3_2 * dz3_2 - dw3_2 * dw3_2;
            if attn3_2 > 0 as f64 {
                attn3_2 *= attn3_2;
                value += attn3_2
                    * attn3_2
                    * extrapolate4(
                        ctx,
                        xsb + 1,
                        ysb + 1,
                        zsb + 0,
                        wsb + 1,
                        dx3_2,
                        dy3_2,
                        dz3_2,
                        dw3_2,
                    );
            }
            let mut dx2_2 = dx4_2;
            let mut dy2_2 = dy0 - 3 as f64 * 0.309016994374947f64;
            let mut dz2_2 = dz4_2;
            let mut dw2_2 = dw3_2;
            let mut attn2_2 =
                2 as f64 - dx2_2 * dx2_2 - dy2_2 * dy2_2 - dz2_2 * dz2_2 - dw2_2 * dw2_2;
            if attn2_2 > 0 as f64 {
                attn2_2 *= attn2_2;
                value += attn2_2
                    * attn2_2
                    * extrapolate4(
                        ctx,
                        xsb + 1,
                        ysb + 0,
                        zsb + 1,
                        wsb + 1,
                        dx2_2,
                        dy2_2,
                        dz2_2,
                        dw2_2,
                    );
            }
            let mut dx1_2 = dx0 - 3 as f64 * 0.309016994374947f64;
            let mut dz1_2 = dz4_2;
            let mut dy1_2 = dy4_2;
            let mut dw1_2 = dw3_2;
            let mut attn1_2 =
                2 as f64 - dx1_2 * dx1_2 - dy1_2 * dy1_2 - dz1_2 * dz1_2 - dw1_2 * dw1_2;
            if attn1_2 > 0 as f64 {
                attn1_2 *= attn1_2;
                value += attn1_2
                    * attn1_2
                    * extrapolate4(
                        ctx,
                        xsb + 0,
                        ysb + 1,
                        zsb + 1,
                        wsb + 1,
                        dx1_2,
                        dy1_2,
                        dz1_2,
                        dw1_2,
                    );
            }
            let mut dx5_0 = dx0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dy5_0 = dy0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dz5_0 = dz0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dw5_0 = dw0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut attn5_0 =
                2 as f64 - dx5_0 * dx5_0 - dy5_0 * dy5_0 - dz5_0 * dz5_0 - dw5_0 * dw5_0;
            if attn5_0 > 0 as f64 {
                attn5_0 *= attn5_0;
                value += attn5_0
                    * attn5_0
                    * extrapolate4(
                        ctx,
                        xsb + 1,
                        ysb + 1,
                        zsb + 0,
                        wsb + 0,
                        dx5_0,
                        dy5_0,
                        dz5_0,
                        dw5_0,
                    );
            }
            let mut dx6_0 = dx0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dy6_0 = dy0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dz6_0 = dz0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dw6_0 = dw0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut attn6_0 =
                2 as f64 - dx6_0 * dx6_0 - dy6_0 * dy6_0 - dz6_0 * dz6_0 - dw6_0 * dw6_0;
            if attn6_0 > 0 as f64 {
                attn6_0 *= attn6_0;
                value += attn6_0
                    * attn6_0
                    * extrapolate4(
                        ctx,
                        xsb + 1,
                        ysb + 0,
                        zsb + 1,
                        wsb + 0,
                        dx6_0,
                        dy6_0,
                        dz6_0,
                        dw6_0,
                    );
            }
            let mut dx7_0 = dx0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dy7_0 = dy0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dz7_0 = dz0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dw7_0 = dw0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut attn7_0 =
                2 as f64 - dx7_0 * dx7_0 - dy7_0 * dy7_0 - dz7_0 * dz7_0 - dw7_0 * dw7_0;
            if attn7_0 > 0 as f64 {
                attn7_0 *= attn7_0;
                value += attn7_0
                    * attn7_0
                    * extrapolate4(
                        ctx,
                        xsb + 1,
                        ysb + 0,
                        zsb + 0,
                        wsb + 1,
                        dx7_0,
                        dy7_0,
                        dz7_0,
                        dw7_0,
                    );
            }
            let mut dx8_0 = dx0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dy8_0 = dy0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dz8_0 = dz0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dw8_0 = dw0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut attn8_0 =
                2 as f64 - dx8_0 * dx8_0 - dy8_0 * dy8_0 - dz8_0 * dz8_0 - dw8_0 * dw8_0;
            if attn8_0 > 0 as f64 {
                attn8_0 *= attn8_0;
                value += attn8_0
                    * attn8_0
                    * extrapolate4(
                        ctx,
                        xsb + 0,
                        ysb + 1,
                        zsb + 1,
                        wsb + 0,
                        dx8_0,
                        dy8_0,
                        dz8_0,
                        dw8_0,
                    );
            }
            let mut dx9_0 = dx0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dy9_0 = dy0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dz9_0 = dz0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dw9_0 = dw0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut attn9_0 =
                2 as f64 - dx9_0 * dx9_0 - dy9_0 * dy9_0 - dz9_0 * dz9_0 - dw9_0 * dw9_0;
            if attn9_0 > 0 as f64 {
                attn9_0 *= attn9_0;
                value += attn9_0
                    * attn9_0
                    * extrapolate4(
                        ctx,
                        xsb + 0,
                        ysb + 1,
                        zsb + 0,
                        wsb + 1,
                        dx9_0,
                        dy9_0,
                        dz9_0,
                        dw9_0,
                    );
            }
            let mut dx10_0 = dx0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dy10_0 = dy0 - 0 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dz10_0 = dz0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut dw10_0 = dw0 - 1 as f64 - 2 as f64 * 0.309016994374947f64;
            let mut attn10_0 =
                2 as f64 - dx10_0 * dx10_0 - dy10_0 * dy10_0 - dz10_0 * dz10_0 - dw10_0 * dw10_0;
            if attn10_0 > 0 as f64 {
                attn10_0 *= attn10_0;
                value += attn10_0
                    * attn10_0
                    * extrapolate4(
                        ctx,
                        xsb + 0,
                        ysb + 0,
                        zsb + 1,
                        wsb + 1,
                        dx10_0,
                        dy10_0,
                        dz10_0,
                        dw10_0,
                    );
            }
        }
        let mut attn_ext0 = 2 as f64
            - dx_ext0 * dx_ext0
            - dy_ext0 * dy_ext0
            - dz_ext0 * dz_ext0
            - dw_ext0 * dw_ext0;
        if attn_ext0 > 0 as f64 {
            attn_ext0 *= attn_ext0;
            value += attn_ext0
                * attn_ext0
                * extrapolate4(
                    ctx, xsv_ext0, ysv_ext0, zsv_ext0, wsv_ext0, dx_ext0, dy_ext0, dz_ext0, dw_ext0,
                );
        }
        let mut attn_ext1 = 2 as f64
            - dx_ext1 * dx_ext1
            - dy_ext1 * dy_ext1
            - dz_ext1 * dz_ext1
            - dw_ext1 * dw_ext1;
        if attn_ext1 > 0 as f64 {
            attn_ext1 *= attn_ext1;
            value += attn_ext1
                * attn_ext1
                * extrapolate4(
                    ctx, xsv_ext1, ysv_ext1, zsv_ext1, wsv_ext1, dx_ext1, dy_ext1, dz_ext1, dw_ext1,
                );
        }
        let mut attn_ext2 = 2 as f64
            - dx_ext2 * dx_ext2
            - dy_ext2 * dy_ext2
            - dz_ext2 * dz_ext2
            - dw_ext2 * dw_ext2;
        if attn_ext2 > 0 as f64 {
            attn_ext2 *= attn_ext2;
            value += attn_ext2
                * attn_ext2
                * extrapolate4(
                    ctx, xsv_ext2, ysv_ext2, zsv_ext2, wsv_ext2, dx_ext2, dy_ext2, dz_ext2, dw_ext2,
                );
        }
        return value / 30.0f64;
    }
}
