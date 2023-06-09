pub const NULL: i32 = 0;
use libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn abs(_: i32) -> i32;
    fn fabs(_: f64) -> f64;
    fn kmVec3Fill(
        pOut: *mut kmVec3,
        x: libc::c_float,
        y: libc::c_float,
        z: libc::c_float,
    ) -> *mut kmVec3;
    fn kmVec3Length(pIn: *const kmVec3) -> libc::c_float;
    fn kmVec3Normalize(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Cross(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Dot(pV1: *const kmVec3, pV2: *const kmVec3) -> libc::c_float;
    fn kmVec3Subtract(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Scale(pOut: *mut kmVec3, pIn: *const kmVec3, s: libc::c_float) -> *mut kmVec3;
    fn kmAlmostEqual(lhs: libc::c_float, rhs: libc::c_float) -> u8;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat4 {
    pub mat: [libc::c_float; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmPlane {
    pub a: libc::c_float,
    pub b: libc::c_float,
    pub c: libc::c_float,
    pub d: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec4 {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
    pub w: libc::c_float,
}
pub const POINT_INFRONT_OF_PLANE: i32 = 1;
pub const POINT_BEHIND_PLANE: i32 = -1;
#[no_mangle]
pub extern "C" fn kmPlaneDot(mut pP: *const kmPlane, mut pV: *const kmVec4) -> libc::c_float {
    unsafe {
        return (*pP).a * (*pV).x + (*pP).b * (*pV).y + (*pP).c * (*pV).z + (*pP).d * (*pV).w;
    }
}

#[no_mangle]
pub extern "C" fn kmPlaneDotCoord(mut pP: *const kmPlane, mut pV: *const kmVec3) -> libc::c_float {
    unsafe {
        return (*pP).a * (*pV).x + (*pP).b * (*pV).y + (*pP).c * (*pV).z + (*pP).d;
    }
}

#[no_mangle]
pub extern "C" fn kmPlaneDotNormal(mut pP: *const kmPlane, mut pV: *const kmVec3) -> libc::c_float {
    unsafe {
        return (*pP).a * (*pV).x + (*pP).b * (*pV).y + (*pP).c * (*pV).z;
    }
}

#[no_mangle]
pub extern "C" fn kmPlaneFromNormalAndDistance(
    mut plane: *mut kmPlane,
    mut normal: *const kmVec3,
    dist: libc::c_float,
) -> *mut kmPlane {
    unsafe {
        (*plane).a = (*normal).x;
        (*plane).b = (*normal).y;
        (*plane).c = (*normal).z;
        (*plane).d = dist;
        return plane;
    }
}

#[no_mangle]
pub extern "C" fn kmPlaneFromPointAndNormal(
    mut pOut: *mut kmPlane,
    mut pPoint: *const kmVec3,
    mut pNormal: *const kmVec3,
) -> *mut kmPlane {
    unsafe {
        (*pOut).a = (*pNormal).x;
        (*pOut).b = (*pNormal).y;
        (*pOut).c = (*pNormal).z;
        (*pOut).d = -kmVec3Dot(pNormal, pPoint);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmPlaneFromPoints(
    mut pOut: *mut kmPlane,
    mut p1: *const kmVec3,
    mut p2: *const kmVec3,
    mut p3: *const kmVec3,
) -> *mut kmPlane {
    unsafe {
        let mut n = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut v1 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut v2 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        kmVec3Subtract(&mut v1, p2, p1);
        kmVec3Subtract(&mut v2, p3, p1);
        kmVec3Cross(&mut n, &mut v1, &mut v2);
        kmVec3Normalize(&mut n, &mut n);
        (*pOut).a = n.x;
        (*pOut).b = n.y;
        (*pOut).c = n.z;
        (*pOut).d = kmVec3Dot(kmVec3Scale(&mut n, &mut n, -1.0f64 as libc::c_float), p1);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmPlaneIntersectLine(
    mut pOut: *mut kmVec3,
    mut pP: *const kmPlane,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
) -> *mut kmVec3 {
    unsafe {
        let mut d = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        kmVec3Subtract(&mut d, pV2, pV1);
        let mut n = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        n.x = (*pP).a;
        n.y = (*pP).b;
        n.z = (*pP).c;
        kmVec3Normalize(&mut n, &mut n);
        let mut nt = -(n.x * (*pV1).x + n.y * (*pV1).y + n.z * (*pV1).z + (*pP).d);
        let mut dt = n.x * d.x + n.y * d.y + n.z * d.z;
        if fabs(dt as f64) < 0.0001f64 {
            pOut = 0 as *mut kmVec3;
            return pOut;
        }
        let mut t = nt / dt;
        (*pOut).x = (*pV1).x + d.x * t;
        (*pOut).y = (*pV1).y + d.y * t;
        (*pOut).z = (*pV1).z + d.z * t;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmPlaneNormalize(mut pOut: *mut kmPlane, mut pP: *const kmPlane) -> *mut kmPlane {
    unsafe {
        let mut n = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut l = 0 as libc::c_float;
        if (*pP).a == 0. && (*pP).b == 0. && (*pP).c == 0. {
            (*pOut).a = (*pP).a;
            (*pOut).b = (*pP).b;
            (*pOut).c = (*pP).c;
            (*pOut).d = (*pP).d;
            return pOut;
        }
        n.x = (*pP).a;
        n.y = (*pP).b;
        n.z = (*pP).c;
        l = 1.0f32 / kmVec3Length(&mut n);
        kmVec3Normalize(&mut n, &mut n);
        (*pOut).a = n.x;
        (*pOut).b = n.y;
        (*pOut).c = n.z;
        (*pOut).d = (*pP).d * l;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmPlaneScale(
    mut pOut: *mut kmPlane,
    mut pP: *const kmPlane,
    mut s: libc::c_float,
) -> *mut kmPlane {
    unsafe {
        if 0 != 0 && !(b"Not implemented\0" as *const u8 as *const i8).is_null() {
        } else {
            __assert_fail(
                b"0 && \"Not implemented\"\0" as *const u8 as *const i8,
                b"../kazmath/plane.c\0" as *const u8 as *const i8,
                179,
                (*::std::mem::transmute::<&[u8; 57], &[i8; 57]>(
                    b"kmPlane *kmPlaneScale(kmPlane *, const kmPlane *, float)\0",
                ))
                .as_ptr(),
            );
        }
        return 0 as *mut kmPlane;
    }
}

#[no_mangle]
pub extern "C" fn kmPlaneClassifyPoint(mut pIn: *const kmPlane, mut pP: *const kmVec3) -> i32 {
    unsafe {
        let mut distance = (*pIn).a * (*pP).x + (*pIn).b * (*pP).y + (*pIn).c * (*pP).z + (*pIn).d;
        if distance as f64 > 0.0001f64 {
            return POINT_INFRONT_OF_PLANE;
        }
        if (distance as f64) < -0.0001f64 {
            return POINT_BEHIND_PLANE;
        }
        return NULL as i32;
    }
}

#[no_mangle]
pub extern "C" fn kmPlaneExtractFromMat4(
    mut pOut: *mut kmPlane,
    mut pIn: *const kmMat4,
    mut row: i32,
) -> *mut kmPlane {
    unsafe {
        let mut scale = if row < 0 { -1 } else { 1 };
        row = abs(row) - 1;
        (*pOut).a = (*pIn).mat[3 as usize] + scale as libc::c_float * (*pIn).mat[row as usize];
        (*pOut).b =
            (*pIn).mat[7 as usize] + scale as libc::c_float * (*pIn).mat[(row + 4i32) as usize];
        (*pOut).c =
            (*pIn).mat[11 as usize] + scale as libc::c_float * (*pIn).mat[(row + 8i32) as usize];
        (*pOut).d =
            (*pIn).mat[15 as usize] + scale as libc::c_float * (*pIn).mat[(row + 12i32) as usize];
        return kmPlaneNormalize(pOut, pOut);
    }
}

#[no_mangle]
pub extern "C" fn kmPlaneGetIntersection(
    mut pOut: *mut kmVec3,
    mut p1: *const kmPlane,
    mut p2: *const kmPlane,
    mut p3: *const kmPlane,
) -> *mut kmVec3 {
    unsafe {
        let mut n1 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut n2 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut n3 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut cross = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut r1 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut r2 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut r3 = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut denom = 0 as f64;
        kmVec3Fill(&mut n1, (*p1).a, (*p1).b, (*p1).c);
        kmVec3Fill(&mut n2, (*p2).a, (*p2).b, (*p2).c);
        kmVec3Fill(&mut n3, (*p3).a, (*p3).b, (*p3).c);
        kmVec3Cross(&mut cross, &mut n2, &mut n3);
        denom = kmVec3Dot(&mut n1, &mut cross) as f64;
        if kmAlmostEqual(denom as libc::c_float, 0.0f64 as libc::c_float) != 0 {
            return 0 as *mut kmVec3;
        }
        kmVec3Cross(&mut r1, &mut n2, &mut n3);
        kmVec3Cross(&mut r2, &mut n3, &mut n1);
        kmVec3Cross(&mut r3, &mut n1, &mut n2);
        kmVec3Scale(&mut r1, &mut r1, -(*p1).d);
        kmVec3Scale(&mut r2, &mut r2, (*p2).d);
        kmVec3Scale(&mut r3, &mut r3, (*p3).d);
        kmVec3Subtract(pOut, &mut r1, &mut r2);
        kmVec3Subtract(pOut, pOut, &mut r3);
        kmVec3Scale(pOut, pOut, (1.0f64 / denom) as libc::c_float);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmPlaneFill(
    mut plane: *mut kmPlane,
    mut a: libc::c_float,
    mut b: libc::c_float,
    mut c: libc::c_float,
    mut d: libc::c_float,
) -> *mut kmPlane {
    unsafe {
        (*plane).a = a;
        (*plane).b = b;
        (*plane).c = c;
        (*plane).d = d;
        return plane;
    }
}
