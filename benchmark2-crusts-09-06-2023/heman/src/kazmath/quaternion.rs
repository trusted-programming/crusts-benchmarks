use libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn acosf(_: libc::c_float) -> libc::c_float;
    fn acos(_: f64) -> f64;
    fn asin(_: f64) -> f64;
    fn atan2(_: f64, _: f64) -> f64;
    fn cosf(_: libc::c_float) -> libc::c_float;
    fn cos(_: f64) -> f64;
    fn sinf(_: libc::c_float) -> libc::c_float;
    fn sin(_: f64) -> f64;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn sqrt(_: f64) -> f64;
    fn fabs(_: f64) -> f64;
    fn kmClamp(x: libc::c_float, min: libc::c_float, max: libc::c_float) -> libc::c_float;
    fn kmSQR(s: libc::c_float) -> libc::c_float;
    fn kmMat3LookAt(
        pOut: *mut kmMat3,
        pEye: *const kmVec3,
        pCenter: *const kmVec3,
        pUp: *const kmVec3,
    ) -> *mut kmMat3;
    fn kmVec3LengthSq(pIn: *const kmVec3) -> libc::c_float;
    fn kmVec3Normalize(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Cross(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Dot(pV1: *const kmVec3, pV2: *const kmVec3) -> libc::c_float;
    fn kmVec3Add(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Scale(pOut: *mut kmVec3, pIn: *const kmVec3, s: libc::c_float) -> *mut kmVec3;
    fn kmVec3Assign(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    static KM_VEC3_NEG_Z: kmVec3;
    static KM_VEC3_POS_Z: kmVec3;
    static KM_VEC3_POS_Y: kmVec3;
    static KM_VEC3_POS_X: kmVec3;
    static KM_VEC3_ZERO: kmVec3;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmVec3 {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat3 {
    pub mat: [libc::c_float; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmQuaternion {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
    pub w: libc::c_float,
}
#[no_mangle]
pub extern "C" fn kmQuaternionAreEqual(
    mut p1: *const kmQuaternion,
    mut p2: *const kmQuaternion,
) -> i32 {
    unsafe {
        if ((*p1).x as f64) < (*p2).x as f64 + 0.0001f64
            && (*p1).x as f64 > (*p2).x as f64 - 0.0001f64
            && (((*p1).y as f64) < (*p2).y as f64 + 0.0001f64
                && (*p1).y as f64 > (*p2).y as f64 - 0.0001f64)
            && (((*p1).z as f64) < (*p2).z as f64 + 0.0001f64
                && (*p1).z as f64 > (*p2).z as f64 - 0.0001f64)
            && (((*p1).w as f64) < (*p2).w as f64 + 0.0001f64
                && (*p1).w as f64 > (*p2).w as f64 - 0.0001f64)
        {
            return 1;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionFill(
    mut pOut: *mut kmQuaternion,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
    mut w: libc::c_float,
) -> *mut kmQuaternion {
    unsafe {
        (*pOut).x = x;
        (*pOut).y = y;
        (*pOut).z = z;
        (*pOut).w = w;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionDot(
    mut q1: *const kmQuaternion,
    mut q2: *const kmQuaternion,
) -> libc::c_float {
    unsafe {
        return (*q1).w * (*q2).w + (*q1).x * (*q2).x + (*q1).y * (*q2).y + (*q1).z * (*q2).z;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionExp(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    unsafe {
        __assert_fail(
            b"0\0" as *const u8 as *const i8,
            b"../kazmath/quaternion.c\0" as *const u8 as *const i8,
            68,
            (*::std::mem::transmute::<&[u8; 68], &[i8; 68]>(
                b"kmQuaternion *kmQuaternionExp(kmQuaternion *, const kmQuaternion *)\0",
            ))
            .as_ptr(),
        );
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionIdentity(mut pOut: *mut kmQuaternion) -> *mut kmQuaternion {
    unsafe {
        (*pOut).x = 0.0f64 as libc::c_float;
        (*pOut).y = 0.0f64 as libc::c_float;
        (*pOut).z = 0.0f64 as libc::c_float;
        (*pOut).w = 1.0f64 as libc::c_float;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionInverse(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    unsafe {
        let mut l = kmQuaternionLength(pIn);
        if fabs(l as f64) < 0.0001f64 {
            (*pOut).x = 0.0f64 as libc::c_float;
            (*pOut).y = 0.0f64 as libc::c_float;
            (*pOut).z = 0.0f64 as libc::c_float;
            (*pOut).w = 0.0f64 as libc::c_float;
            return pOut;
        };
        (*pOut).x = -(*pIn).x;
        (*pOut).y = -(*pIn).y;
        (*pOut).z = -(*pIn).z;
        (*pOut).w = (*pIn).w;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionIsIdentity(mut pIn: *const kmQuaternion) -> i32 {
    unsafe {
        return ((*pIn).x as f64 == 0.0f64
            && (*pIn).y as f64 == 0.0f64
            && (*pIn).z as f64 == 0.0f64
            && (*pIn).w as f64 == 1.0f64) as i32;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionLength(mut pIn: *const kmQuaternion) -> libc::c_float {
    unsafe {
        return sqrt(kmQuaternionLengthSq(pIn) as f64) as libc::c_float;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionLengthSq(mut pIn: *const kmQuaternion) -> libc::c_float {
    unsafe {
        return (*pIn).x * (*pIn).x
            + (*pIn).y * (*pIn).y
            + (*pIn).z * (*pIn).z
            + (*pIn).w * (*pIn).w;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionLn(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    unsafe {
        __assert_fail(
            b"0\0" as *const u8 as *const i8,
            b"../kazmath/quaternion.c\0" as *const u8 as *const i8,
            137,
            (*::std::mem::transmute::<&[u8; 67], &[i8; 67]>(
                b"kmQuaternion *kmQuaternionLn(kmQuaternion *, const kmQuaternion *)\0",
            ))
            .as_ptr(),
        );
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionMultiply(
    mut pOut: *mut kmQuaternion,
    mut qu1: *const kmQuaternion,
    mut qu2: *const kmQuaternion,
) -> *mut kmQuaternion {
    unsafe {
        let mut tmp1 = kmQuaternion {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        };
        let mut tmp2 = kmQuaternion {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        };
        kmQuaternionAssign(&mut tmp1, qu1);
        kmQuaternionAssign(&mut tmp2, qu2);
        let mut q1: *mut kmQuaternion = &mut tmp1;
        let mut q2: *mut kmQuaternion = &mut tmp2;
        (*pOut).x = (*q1).w * (*q2).x + (*q1).x * (*q2).w + (*q1).y * (*q2).z - (*q1).z * (*q2).y;
        (*pOut).y = (*q1).w * (*q2).y + (*q1).y * (*q2).w + (*q1).z * (*q2).x - (*q1).x * (*q2).z;
        (*pOut).z = (*q1).w * (*q2).z + (*q1).z * (*q2).w + (*q1).x * (*q2).y - (*q1).y * (*q2).x;
        (*pOut).w = (*q1).w * (*q2).w - (*q1).x * (*q2).x - (*q1).y * (*q2).y - (*q1).z * (*q2).z;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionNormalize(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    unsafe {
        let mut length = kmQuaternionLength(pIn);
        if fabs(length as f64) < 0.0001f64 {
            (*pOut).x = 0.0f64 as libc::c_float;
            (*pOut).y = 0.0f64 as libc::c_float;
            (*pOut).z = 0.0f64 as libc::c_float;
            (*pOut).w = 0.0f64 as libc::c_float;
            return pOut;
        }
        kmQuaternionFill(
            pOut,
            (*pOut).x / length,
            (*pOut).y / length,
            (*pOut).z / length,
            (*pOut).w / length,
        );
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionRotationAxisAngle(
    mut pOut: *mut kmQuaternion,
    mut pV: *const kmVec3,
    mut angle: libc::c_float,
) -> *mut kmQuaternion {
    unsafe {
        let mut rad = angle * 0.5f32;
        let mut scale = sinf(rad);
        (*pOut).x = (*pV).x * scale;
        (*pOut).y = (*pV).y * scale;
        (*pOut).z = (*pV).z * scale;
        (*pOut).w = cosf(rad);
        kmQuaternionNormalize(pOut, pOut);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionRotationMatrix(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmMat3,
) -> *mut kmQuaternion {
    unsafe {
        let mut x: libc::c_float = 0.;
        let mut y: libc::c_float = 0.;
        let mut z: libc::c_float = 0.;
        let mut w: libc::c_float = 0.;
        let mut pMatrix = 0 as *mut libc::c_float;
        let mut m4x4: [libc::c_float; 16] = [
            0 as libc::c_float,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
            0.,
        ];
        let mut scale = 0.0f32;
        let mut diagonal = 0.0f32;
        if pIn.is_null() {
            return 0 as *mut kmQuaternion;
        }
        m4x4[0 as usize] = (*pIn).mat[0 as usize];
        m4x4[1 as usize] = (*pIn).mat[3 as usize];
        m4x4[2 as usize] = (*pIn).mat[6 as usize];
        m4x4[4 as usize] = (*pIn).mat[1 as usize];
        m4x4[5 as usize] = (*pIn).mat[4 as usize];
        m4x4[6 as usize] = (*pIn).mat[7 as usize];
        m4x4[8 as usize] = (*pIn).mat[2 as usize];
        m4x4[9 as usize] = (*pIn).mat[5 as usize];
        m4x4[10 as usize] = (*pIn).mat[8 as usize];
        m4x4[15 as usize] = 1 as libc::c_float;
        pMatrix = &mut *m4x4.as_mut_ptr().offset(0 as isize) as *mut libc::c_float;
        diagonal = *pMatrix.offset(0 as isize)
            + *pMatrix.offset(5 as isize)
            + *pMatrix.offset(10 as isize)
            + 1 as libc::c_float;
        if diagonal as f64 > 0.0001f64 {
            scale = sqrt(diagonal as f64) as libc::c_float * 2 as libc::c_float;
            x = (*pMatrix.offset(9 as isize) - *pMatrix.offset(6 as isize)) / scale;
            y = (*pMatrix.offset(2 as isize) - *pMatrix.offset(8 as isize)) / scale;
            z = (*pMatrix.offset(4 as isize) - *pMatrix.offset(1 as isize)) / scale;
            w = 0.25f32 * scale;
        } else if *pMatrix.offset(0 as isize) > *pMatrix.offset(5 as isize)
            && *pMatrix.offset(0 as isize) > *pMatrix.offset(10 as isize)
        {
            scale = sqrt(
                (1.0f32 + *pMatrix.offset(0 as isize)
                    - *pMatrix.offset(5 as isize)
                    - *pMatrix.offset(10 as isize)) as f64,
            ) as libc::c_float
                * 2.0f32;
            x = 0.25f32 * scale;
            y = (*pMatrix.offset(4 as isize) + *pMatrix.offset(1 as isize)) / scale;
            z = (*pMatrix.offset(2 as isize) + *pMatrix.offset(8 as isize)) / scale;
            w = (*pMatrix.offset(9 as isize) - *pMatrix.offset(6 as isize)) / scale;
        } else if *pMatrix.offset(5 as isize) > *pMatrix.offset(10 as isize) {
            scale = sqrt(
                (1.0f32 + *pMatrix.offset(5 as isize)
                    - *pMatrix.offset(0 as isize)
                    - *pMatrix.offset(10 as isize)) as f64,
            ) as libc::c_float
                * 2.0f32;
            x = (*pMatrix.offset(4 as isize) + *pMatrix.offset(1 as isize)) / scale;
            y = 0.25f32 * scale;
            z = (*pMatrix.offset(9 as isize) + *pMatrix.offset(6 as isize)) / scale;
            w = (*pMatrix.offset(2 as isize) - *pMatrix.offset(8 as isize)) / scale;
        } else {
            scale = sqrt(
                (1.0f32 + *pMatrix.offset(10 as isize)
                    - *pMatrix.offset(0 as isize)
                    - *pMatrix.offset(5 as isize)) as f64,
            ) as libc::c_float
                * 2.0f32;
            x = (*pMatrix.offset(2 as isize) + *pMatrix.offset(8 as isize)) / scale;
            y = (*pMatrix.offset(9 as isize) + *pMatrix.offset(6 as isize)) / scale;
            z = 0.25f32 * scale;
            w = (*pMatrix.offset(4 as isize) - *pMatrix.offset(1 as isize)) / scale;
        };
        (*pOut).x = x;
        (*pOut).y = y;
        (*pOut).z = z;
        (*pOut).w = w;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionRotationPitchYawRoll(
    mut pOut: *mut kmQuaternion,
    mut pitch: libc::c_float,
    mut yaw: libc::c_float,
    mut roll: libc::c_float,
) -> *mut kmQuaternion {
    unsafe {
        if pitch <= 2 as libc::c_float * 3.14159265358979323846f32 {
        } else {
            __assert_fail (b"pitch <= 2*kmPI\0" as * const u8 as * const i8, b"../kazmath/quaternion.c\0" as * const u8 as * const i8, 334, (* :: std :: mem :: transmute :: < & [u8; 84], & [i8; 84], > (
              b"kmQuaternion *kmQuaternionRotationPitchYawRoll(kmQuaternion *, float, float, float)\0",)).as_ptr (),);
        }
        if yaw <= 2 as libc::c_float * 3.14159265358979323846f32 {
        } else {
            __assert_fail (b"yaw <= 2*kmPI\0" as * const u8 as * const i8, b"../kazmath/quaternion.c\0" as * const u8 as * const i8, 335, (* :: std :: mem :: transmute :: < & [u8; 84], & [i8; 84], > (
              b"kmQuaternion *kmQuaternionRotationPitchYawRoll(kmQuaternion *, float, float, float)\0",)).as_ptr (),);
        }
        if roll <= 2 as libc::c_float * 3.14159265358979323846f32 {
        } else {
            __assert_fail (b"roll <= 2*kmPI\0" as * const u8 as * const i8, b"../kazmath/quaternion.c\0" as * const u8 as * const i8, 336, (* :: std :: mem :: transmute :: < & [u8; 84], & [i8; 84], > (
              b"kmQuaternion *kmQuaternionRotationPitchYawRoll(kmQuaternion *, float, float, float)\0",)).as_ptr (),);
        }
        let mut sY = sinf((yaw as f64 * 0.5f64) as libc::c_float);
        let mut cY = cosf((yaw as f64 * 0.5f64) as libc::c_float);
        let mut sZ = sinf((roll as f64 * 0.5f64) as libc::c_float);
        let mut cZ = cosf((roll as f64 * 0.5f64) as libc::c_float);
        let mut sX = sinf((pitch as f64 * 0.5f64) as libc::c_float);
        let mut cX = cosf((pitch as f64 * 0.5f64) as libc::c_float);
        (*pOut).w = cY * cZ * cX - sY * sZ * sX;
        (*pOut).x = sY * sZ * cX + cY * cZ * sX;
        (*pOut).y = sY * cZ * cX + cY * sZ * sX;
        (*pOut).z = cY * sZ * cX - sY * cZ * sX;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionSlerp(
    mut pOut: *mut kmQuaternion,
    mut q1: *const kmQuaternion,
    mut q2: *const kmQuaternion,
    mut t: libc::c_float,
) -> *mut kmQuaternion {
    unsafe {
        let mut dot = kmQuaternionDot(q1, q2);
        let DOT_THRESHOLD = 0.9995f64;
        if dot as f64 > DOT_THRESHOLD {
            let mut diff = kmQuaternion {
                x: 0.,
                y: 0.,
                z: 0.,
                w: 0.,
            };
            kmQuaternionSubtract(&mut diff, q2, q1);
            kmQuaternionScale(&mut diff, &mut diff, t);
            kmQuaternionAdd(pOut, q1, &mut diff);
            kmQuaternionNormalize(pOut, pOut);
            return pOut;
        }
        dot = kmClamp(dot, -1i32 as libc::c_float, 1 as libc::c_float);
        let mut theta_0 = acos(dot as f64) as libc::c_float;
        let mut theta = theta_0 * t;
        let mut tmp = kmQuaternion {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        };
        kmQuaternionScale(&mut tmp, q1, dot);
        kmQuaternionSubtract(&mut tmp, q2, &mut tmp);
        kmQuaternionNormalize(&mut tmp, &mut tmp);
        let mut t1 = kmQuaternion {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        };
        let mut t2 = kmQuaternion {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        };
        kmQuaternionScale(&mut t1, q1, cos(theta as f64) as libc::c_float);
        kmQuaternionScale(&mut t2, &mut tmp, sin(theta as f64) as libc::c_float);
        kmQuaternionAdd(pOut, &mut t1, &mut t2);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionToAxisAngle(
    mut pIn: *const kmQuaternion,
    mut pAxis: *mut kmVec3,
    mut pAngle: *mut libc::c_float,
) {
    unsafe {
        let mut tempAngle: libc::c_float = 0.;
        let mut scale: libc::c_float = 0.;
        tempAngle = acosf((*pIn).w);
        scale = sqrtf(kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z));
        if scale as f64 > -0.0001f64 && (scale as f64) < 0.0001f64
            || (scale as f64) < (2 as libc::c_float * 3.14159265358979323846f32) as f64 + 0.0001f64
                && scale as f64
                    > (2 as libc::c_float * 3.14159265358979323846f32) as f64 - 0.0001f64
        {
            *pAngle = 0.0f32;
            (*pAxis).x = 0.0f32;
            (*pAxis).y = 0.0f32;
            (*pAxis).z = 1.0f32;
        } else {
            *pAngle = tempAngle * 2.0f32;
            (*pAxis).x = (*pIn).x / scale;
            (*pAxis).y = (*pIn).y / scale;
            (*pAxis).z = (*pIn).z / scale;
            kmVec3Normalize(pAxis, pAxis);
        };
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionScale(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
    mut s: libc::c_float,
) -> *mut kmQuaternion {
    unsafe {
        (*pOut).x = (*pIn).x * s;
        (*pOut).y = (*pIn).y * s;
        (*pOut).z = (*pIn).z * s;
        (*pOut).w = (*pIn).w * s;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionAssign(
    mut pOut: *mut kmQuaternion,
    mut pIn: *const kmQuaternion,
) -> *mut kmQuaternion {
    unsafe {
        memcpy(
            pOut as *mut libc::c_void,
            pIn as *const libc::c_void,
            (::std::mem::size_of::<libc::c_float>() as u64).wrapping_mul(4),
        );
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionSubtract(
    mut pOut: *mut kmQuaternion,
    mut pQ1: *const kmQuaternion,
    mut pQ2: *const kmQuaternion,
) -> *mut kmQuaternion {
    unsafe {
        (*pOut).x = (*pQ1).x - (*pQ2).x;
        (*pOut).y = (*pQ1).y - (*pQ2).y;
        (*pOut).z = (*pQ1).z - (*pQ2).z;
        (*pOut).w = (*pQ1).w - (*pQ2).w;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionAdd(
    mut pOut: *mut kmQuaternion,
    mut pQ1: *const kmQuaternion,
    mut pQ2: *const kmQuaternion,
) -> *mut kmQuaternion {
    unsafe {
        (*pOut).x = (*pQ1).x + (*pQ2).x;
        (*pOut).y = (*pQ1).y + (*pQ2).y;
        (*pOut).z = (*pQ1).z + (*pQ2).z;
        (*pOut).w = (*pQ1).w + (*pQ2).w;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionRotationBetweenVec3(
    mut pOut: *mut kmQuaternion,
    mut vec1: *const kmVec3,
    mut vec2: *const kmVec3,
    mut fallback: *const kmVec3,
) -> *mut kmQuaternion {
    unsafe {
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
        let mut a: libc::c_float = 0.;
        kmVec3Assign(&mut v1, vec1);
        kmVec3Assign(&mut v2, vec2);
        kmVec3Normalize(&mut v1, &mut v1);
        kmVec3Normalize(&mut v2, &mut v2);
        a = kmVec3Dot(&mut v1, &mut v2);
        if a as f64 >= 1.0f64 {
            kmQuaternionIdentity(pOut);
            return pOut;
        }
        if a < 1e-6f32 - 1.0f32 {
            if fabs(kmVec3LengthSq(fallback) as f64) < 0.0001f64 {
                kmQuaternionRotationAxisAngle(pOut, fallback, 3.14159265358979323846f32);
            } else {
                let mut axis = kmVec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                };
                let mut X = kmVec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                };
                X.x = 1.0f64 as libc::c_float;
                X.y = 0.0f64 as libc::c_float;
                X.z = 0.0f64 as libc::c_float;
                kmVec3Cross(&mut axis, &mut X, vec1);
                if fabs(kmVec3LengthSq(&mut axis) as f64) < 0.0001f64 {
                    let mut Y = kmVec3 {
                        x: 0.,
                        y: 0.,
                        z: 0.,
                    };
                    Y.x = 0.0f64 as libc::c_float;
                    Y.y = 1.0f64 as libc::c_float;
                    Y.z = 0.0f64 as libc::c_float;
                    kmVec3Cross(&mut axis, &mut Y, vec1);
                }
                kmVec3Normalize(&mut axis, &mut axis);
                kmQuaternionRotationAxisAngle(pOut, &mut axis, 3.14159265358979323846f32);
            }
        } else {
            let mut s = sqrtf((1 as libc::c_float + a) * 2 as libc::c_float);
            let mut invs = 1 as libc::c_float / s;
            let mut c = kmVec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            };
            kmVec3Cross(&mut c, &mut v1, &mut v2);
            (*pOut).x = c.x * invs;
            (*pOut).y = c.y * invs;
            (*pOut).z = c.z * invs;
            (*pOut).w = s * 0.5f32;
            kmQuaternionNormalize(pOut, pOut);
        }
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionMultiplyVec3(
    mut pOut: *mut kmVec3,
    mut q: *const kmQuaternion,
    mut v: *const kmVec3,
) -> *mut kmVec3 {
    unsafe {
        let mut uv = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut uuv = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut qvec = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        qvec.x = (*q).x;
        qvec.y = (*q).y;
        qvec.z = (*q).z;
        kmVec3Cross(&mut uv, &mut qvec, v);
        kmVec3Cross(&mut uuv, &mut qvec, &mut uv);
        kmVec3Scale(&mut uv, &mut uv, 2.0f32 * (*q).w);
        kmVec3Scale(&mut uuv, &mut uuv, 2.0f32);
        kmVec3Add(pOut, v, &mut uv);
        kmVec3Add(pOut, pOut, &mut uuv);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionGetUpVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmQuaternion,
) -> *mut kmVec3 {
    unsafe {
        return kmQuaternionMultiplyVec3(pOut, pIn, &KM_VEC3_POS_Y);
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionGetRightVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmQuaternion,
) -> *mut kmVec3 {
    unsafe {
        return kmQuaternionMultiplyVec3(pOut, pIn, &KM_VEC3_POS_X);
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionGetForwardVec3RH(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmQuaternion,
) -> *mut kmVec3 {
    unsafe {
        return kmQuaternionMultiplyVec3(pOut, pIn, &KM_VEC3_NEG_Z);
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionGetForwardVec3LH(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmQuaternion,
) -> *mut kmVec3 {
    unsafe {
        return kmQuaternionMultiplyVec3(pOut, pIn, &KM_VEC3_POS_Z);
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionGetPitch(mut q: *const kmQuaternion) -> libc::c_float {
    unsafe {
        let mut result = atan2(
            (2 as libc::c_float * ((*q).y * (*q).z + (*q).w * (*q).x)) as f64,
            ((*q).w * (*q).w - (*q).x * (*q).x - (*q).y * (*q).y + (*q).z * (*q).z) as f64,
        ) as libc::c_float;
        return result;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionGetYaw(mut q: *const kmQuaternion) -> libc::c_float {
    unsafe {
        let mut result = asin((-2i32 as libc::c_float * ((*q).x * (*q).z - (*q).w * (*q).y)) as f64)
            as libc::c_float;
        return result;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionGetRoll(mut q: *const kmQuaternion) -> libc::c_float {
    unsafe {
        let mut result = atan2(
            (2 as libc::c_float * ((*q).x * (*q).y + (*q).w * (*q).z)) as f64,
            ((*q).w * (*q).w + (*q).x * (*q).x - (*q).y * (*q).y - (*q).z * (*q).z) as f64,
        ) as libc::c_float;
        return result;
    }
}

#[no_mangle]
pub extern "C" fn kmQuaternionLookRotation(
    mut pOut: *mut kmQuaternion,
    mut direction: *const kmVec3,
    mut up: *const kmVec3,
) -> *mut kmQuaternion {
    unsafe {
        let mut tmp = kmMat3 { mat: [0.; 9] };
        kmMat3LookAt(&mut tmp, &KM_VEC3_ZERO, direction, up);
        return kmQuaternionNormalize(pOut, kmQuaternionRotationMatrix(pOut, &mut tmp));
    }
}
