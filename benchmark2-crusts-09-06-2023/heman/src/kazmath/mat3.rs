use libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn cosf(_: libc::c_float) -> libc::c_float;
    fn sinf(_: libc::c_float) -> libc::c_float;
    fn kmVec3Normalize(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Cross(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Subtract(pOut: *mut kmVec3, pV1: *const kmVec3, pV2: *const kmVec3) -> *mut kmVec3;
    fn kmVec3Assign(pOut: *mut kmVec3, pIn: *const kmVec3) -> *mut kmVec3;
    fn kmQuaternionRotationMatrix(pOut: *mut kmQuaternion, pIn: *const kmMat3)
        -> *mut kmQuaternion;
    fn kmQuaternionToAxisAngle(
        pIn: *const kmQuaternion,
        pVector: *mut kmVec3,
        pAngle: *mut libc::c_float,
    );
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat4 {
    pub mat: [libc::c_float; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat3 {
    pub mat: [libc::c_float; 9],
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
pub struct kmQuaternion {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
    pub w: libc::c_float,
}
#[no_mangle]
pub extern "C" fn kmMat3Fill(mut pOut: *mut kmMat3, mut pMat: *const libc::c_float) -> *mut kmMat3 {
    unsafe {
        memcpy(
            ((*pOut).mat).as_mut_ptr() as *mut libc::c_void,
            pMat as *const libc::c_void,
            (::std::mem::size_of::<libc::c_float>() as u64).wrapping_mul(9),
        );
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3Identity(mut pOut: *mut kmMat3) -> *mut kmMat3 {
    unsafe {
        memset(
            ((*pOut).mat).as_mut_ptr() as *mut libc::c_void,
            0,
            (::std::mem::size_of::<libc::c_float>() as u64).wrapping_mul(9),
        );
        let ref mut fresh0 = (*pOut).mat[8 as usize];
        *fresh0 = 1.0f32;
        let ref mut fresh1 = (*pOut).mat[4 as usize];
        *fresh1 = *fresh0;
        (*pOut).mat[0 as usize] = *fresh1;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3Determinant(mut pIn: *const kmMat3) -> libc::c_float {
    unsafe {
        let mut output: libc::c_float = 0.;
        output = (*pIn).mat[0 as usize] * (*pIn).mat[4 as usize] * (*pIn).mat[8 as usize]
            + (*pIn).mat[1 as usize] * (*pIn).mat[5 as usize] * (*pIn).mat[6 as usize]
            + (*pIn).mat[2 as usize] * (*pIn).mat[3 as usize] * (*pIn).mat[7 as usize];
        output -= (*pIn).mat[2 as usize] * (*pIn).mat[4 as usize] * (*pIn).mat[6 as usize]
            + (*pIn).mat[0 as usize] * (*pIn).mat[5 as usize] * (*pIn).mat[7 as usize]
            + (*pIn).mat[1 as usize] * (*pIn).mat[3 as usize] * (*pIn).mat[8 as usize];
        return output;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3Adjugate(mut pOut: *mut kmMat3, mut pIn: *const kmMat3) -> *mut kmMat3 {
    unsafe {
        (*pOut).mat[0 as usize] = (*pIn).mat[4 as usize] * (*pIn).mat[8 as usize]
            - (*pIn).mat[5 as usize] * (*pIn).mat[7 as usize];
        (*pOut).mat[1 as usize] = (*pIn).mat[2 as usize] * (*pIn).mat[7 as usize]
            - (*pIn).mat[1 as usize] * (*pIn).mat[8 as usize];
        (*pOut).mat[2 as usize] = (*pIn).mat[1 as usize] * (*pIn).mat[5 as usize]
            - (*pIn).mat[2 as usize] * (*pIn).mat[4 as usize];
        (*pOut).mat[3 as usize] = (*pIn).mat[5 as usize] * (*pIn).mat[6 as usize]
            - (*pIn).mat[3 as usize] * (*pIn).mat[8 as usize];
        (*pOut).mat[4 as usize] = (*pIn).mat[0 as usize] * (*pIn).mat[8 as usize]
            - (*pIn).mat[2 as usize] * (*pIn).mat[6 as usize];
        (*pOut).mat[5 as usize] = (*pIn).mat[2 as usize] * (*pIn).mat[3 as usize]
            - (*pIn).mat[0 as usize] * (*pIn).mat[5 as usize];
        (*pOut).mat[6 as usize] = (*pIn).mat[3 as usize] * (*pIn).mat[7 as usize]
            - (*pIn).mat[4 as usize] * (*pIn).mat[6 as usize];
        (*pOut).mat[7 as usize] = (*pIn).mat[1 as usize] * (*pIn).mat[6 as usize]
            - (*pIn).mat[0 as usize] * (*pIn).mat[7 as usize];
        (*pOut).mat[8 as usize] = (*pIn).mat[0 as usize] * (*pIn).mat[4 as usize]
            - (*pIn).mat[1 as usize] * (*pIn).mat[3 as usize];
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3Inverse(mut pOut: *mut kmMat3, mut pM: *const kmMat3) -> *mut kmMat3 {
    unsafe {
        let mut determinate = kmMat3Determinant(pM);
        let mut detInv: libc::c_float = 0.;
        let mut adjugate = kmMat3 { mat: [0.; 9] };
        if determinate as f64 == 0.0f64 {
            return 0 as *mut kmMat3;
        }
        detInv = (1.0f64 / determinate as f64) as libc::c_float;
        kmMat3Adjugate(&mut adjugate, pM);
        kmMat3ScalarMultiply(pOut, &mut adjugate, detInv);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3IsIdentity(mut pIn: *const kmMat3) -> i32 {
    unsafe {
        static mut identity: [libc::c_float; 9] = [
            1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32,
        ];
        return (memcmp(
            identity.as_mut_ptr() as *const libc::c_void,
            ((*pIn).mat).as_ptr() as *const libc::c_void,
            (::std::mem::size_of::<libc::c_float>() as u64).wrapping_mul(9),
        ) == 0) as i32;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3Transpose(mut pOut: *mut kmMat3, mut pIn: *const kmMat3) -> *mut kmMat3 {
    unsafe {
        let mut temp: [libc::c_float; 9] = [0.; 9];
        temp[0 as usize] = (*pIn).mat[0 as usize];
        temp[1 as usize] = (*pIn).mat[3 as usize];
        temp[2 as usize] = (*pIn).mat[6 as usize];
        temp[3 as usize] = (*pIn).mat[1 as usize];
        temp[4 as usize] = (*pIn).mat[4 as usize];
        temp[5 as usize] = (*pIn).mat[7 as usize];
        temp[6 as usize] = (*pIn).mat[2 as usize];
        temp[7 as usize] = (*pIn).mat[5 as usize];
        temp[8 as usize] = (*pIn).mat[8 as usize];
        memcpy(
            &mut (*pOut).mat as *mut [libc::c_float; 9] as *mut libc::c_void,
            temp.as_mut_ptr() as *const libc::c_void,
            (::std::mem::size_of::<libc::c_float>() as u64).wrapping_mul(9),
        );
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3Multiply(
    mut pOut: *mut kmMat3,
    mut pM1: *const kmMat3,
    mut pM2: *const kmMat3,
) -> *mut kmMat3 {
    unsafe {
        let mut mat: [libc::c_float; 9] = [0.; 9];
        let mut m1 = ((*pM1).mat).as_ptr();
        let mut m2 = ((*pM2).mat).as_ptr();
        mat[0 as usize] = *m1.offset(0 as isize) * *m2.offset(0 as isize)
            + *m1.offset(3 as isize) * *m2.offset(1 as isize)
            + *m1.offset(6 as isize) * *m2.offset(2 as isize);
        mat[1 as usize] = *m1.offset(1 as isize) * *m2.offset(0 as isize)
            + *m1.offset(4 as isize) * *m2.offset(1 as isize)
            + *m1.offset(7 as isize) * *m2.offset(2 as isize);
        mat[2 as usize] = *m1.offset(2 as isize) * *m2.offset(0 as isize)
            + *m1.offset(5 as isize) * *m2.offset(1 as isize)
            + *m1.offset(8 as isize) * *m2.offset(2 as isize);
        mat[3 as usize] = *m1.offset(0 as isize) * *m2.offset(3 as isize)
            + *m1.offset(3 as isize) * *m2.offset(4 as isize)
            + *m1.offset(6 as isize) * *m2.offset(5 as isize);
        mat[4 as usize] = *m1.offset(1 as isize) * *m2.offset(3 as isize)
            + *m1.offset(4 as isize) * *m2.offset(4 as isize)
            + *m1.offset(7 as isize) * *m2.offset(5 as isize);
        mat[5 as usize] = *m1.offset(2 as isize) * *m2.offset(3 as isize)
            + *m1.offset(5 as isize) * *m2.offset(4 as isize)
            + *m1.offset(8 as isize) * *m2.offset(5 as isize);
        mat[6 as usize] = *m1.offset(0 as isize) * *m2.offset(6 as isize)
            + *m1.offset(3 as isize) * *m2.offset(7 as isize)
            + *m1.offset(6 as isize) * *m2.offset(8 as isize);
        mat[7 as usize] = *m1.offset(1 as isize) * *m2.offset(6 as isize)
            + *m1.offset(4 as isize) * *m2.offset(7 as isize)
            + *m1.offset(7 as isize) * *m2.offset(8 as isize);
        mat[8 as usize] = *m1.offset(2 as isize) * *m2.offset(6 as isize)
            + *m1.offset(5 as isize) * *m2.offset(7 as isize)
            + *m1.offset(8 as isize) * *m2.offset(8 as isize);
        memcpy(
            ((*pOut).mat).as_mut_ptr() as *mut libc::c_void,
            mat.as_mut_ptr() as *const libc::c_void,
            (::std::mem::size_of::<libc::c_float>() as u64).wrapping_mul(9),
        );
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3ScalarMultiply(
    mut pOut: *mut kmMat3,
    mut pM: *const kmMat3,
    pFactor: libc::c_float,
) -> *mut kmMat3 {
    unsafe {
        let mut mat: [libc::c_float; 9] = [0.; 9];
        let mut i: i32 = 0;
        i = 0;
        while i < 9 {
            mat[i as usize] = (*pM).mat[i as usize] * pFactor;
            i += 1;
        }
        memcpy(
            ((*pOut).mat).as_mut_ptr() as *mut libc::c_void,
            mat.as_mut_ptr() as *const libc::c_void,
            (::std::mem::size_of::<libc::c_float>() as u64).wrapping_mul(9),
        );
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3Assign(mut pOut: *mut kmMat3, mut pIn: *const kmMat3) -> *mut kmMat3 {
    unsafe {
        if pOut != pIn as *mut kmMat3 {
        } else {
            __assert_fail(
                b"pOut != pIn\0" as *const u8 as *const i8,
                b"../kazmath/mat3.c\0" as *const u8 as *const i8,
                177,
                (*::std::mem::transmute::<&[u8; 47], &[i8; 47]>(
                    b"kmMat3 *kmMat3Assign(kmMat3 *, const kmMat3 *)\0",
                ))
                .as_ptr(),
            );
        }
        memcpy(
            ((*pOut).mat).as_mut_ptr() as *mut libc::c_void,
            ((*pIn).mat).as_ptr() as *const libc::c_void,
            (::std::mem::size_of::<libc::c_float>() as u64).wrapping_mul(9),
        );
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3AssignMat4(mut pOut: *mut kmMat3, mut pIn: *const kmMat4) -> *mut kmMat3 {
    unsafe {
        (*pOut).mat[0 as usize] = (*pIn).mat[0 as usize];
        (*pOut).mat[1 as usize] = (*pIn).mat[1 as usize];
        (*pOut).mat[2 as usize] = (*pIn).mat[2 as usize];
        (*pOut).mat[3 as usize] = (*pIn).mat[4 as usize];
        (*pOut).mat[4 as usize] = (*pIn).mat[5 as usize];
        (*pOut).mat[5 as usize] = (*pIn).mat[6 as usize];
        (*pOut).mat[6 as usize] = (*pIn).mat[8 as usize];
        (*pOut).mat[7 as usize] = (*pIn).mat[9 as usize];
        (*pOut).mat[8 as usize] = (*pIn).mat[10 as usize];
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3AreEqual(mut pMat1: *const kmMat3, mut pMat2: *const kmMat3) -> i32 {
    unsafe {
        let mut i: i32 = 0;
        if pMat1 == pMat2 {
            return 1;
        }
        i = 0;
        while i < 9 {
            if !((*pMat1).mat[i as usize] as f64 + 0.0001f64 > (*pMat2).mat[i as usize] as f64
                && (*pMat1).mat[i as usize] as f64 - 0.0001f64 < (*pMat2).mat[i as usize] as f64)
            {
                return 0;
            }
            i += 1;
        }
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3Rotation(mut pOut: *mut kmMat3, radians: libc::c_float) -> *mut kmMat3 {
    unsafe {
        (*pOut).mat[0 as usize] = cosf(radians);
        (*pOut).mat[1 as usize] = sinf(radians);
        (*pOut).mat[2 as usize] = 0.0f32;
        (*pOut).mat[3 as usize] = -sinf(radians);
        (*pOut).mat[4 as usize] = cosf(radians);
        (*pOut).mat[5 as usize] = 0.0f32;
        (*pOut).mat[6 as usize] = 0.0f32;
        (*pOut).mat[7 as usize] = 0.0f32;
        (*pOut).mat[8 as usize] = 1.0f32;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3Scaling(
    mut pOut: *mut kmMat3,
    x: libc::c_float,
    y: libc::c_float,
) -> *mut kmMat3 {
    unsafe {
        kmMat3Identity(pOut);
        (*pOut).mat[0 as usize] = x;
        (*pOut).mat[4 as usize] = y;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3Translation(
    mut pOut: *mut kmMat3,
    x: libc::c_float,
    y: libc::c_float,
) -> *mut kmMat3 {
    unsafe {
        kmMat3Identity(pOut);
        (*pOut).mat[6 as usize] = x;
        (*pOut).mat[7 as usize] = y;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3RotationQuaternion(
    mut pOut: *mut kmMat3,
    mut pIn: *const kmQuaternion,
) -> *mut kmMat3 {
    unsafe {
        if pIn.is_null() || pOut.is_null() {
            return 0 as *mut kmMat3;
        };
        (*pOut).mat[0 as usize] = 1.0f32 - 2.0f32 * ((*pIn).y * (*pIn).y + (*pIn).z * (*pIn).z);
        (*pOut).mat[1 as usize] = 2.0f32 * ((*pIn).x * (*pIn).y - (*pIn).w * (*pIn).z);
        (*pOut).mat[2 as usize] = 2.0f32 * ((*pIn).x * (*pIn).z + (*pIn).w * (*pIn).y);
        (*pOut).mat[3 as usize] = 2.0f32 * ((*pIn).x * (*pIn).y + (*pIn).w * (*pIn).z);
        (*pOut).mat[4 as usize] = 1.0f32 - 2.0f32 * ((*pIn).x * (*pIn).x + (*pIn).z * (*pIn).z);
        (*pOut).mat[5 as usize] = 2.0f32 * ((*pIn).y * (*pIn).z - (*pIn).w * (*pIn).x);
        (*pOut).mat[6 as usize] = 2.0f32 * ((*pIn).x * (*pIn).z - (*pIn).w * (*pIn).y);
        (*pOut).mat[7 as usize] = 2.0f32 * ((*pIn).y * (*pIn).z + (*pIn).w * (*pIn).x);
        (*pOut).mat[8 as usize] = 1.0f32 - 2.0f32 * ((*pIn).x * (*pIn).x + (*pIn).y * (*pIn).y);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3RotationAxisAngle(
    mut pOut: *mut kmMat3,
    mut axis: *const kmVec3,
    mut radians: libc::c_float,
) -> *mut kmMat3 {
    unsafe {
        let mut rcos = cosf(radians);
        let mut rsin = sinf(radians);
        (*pOut).mat[0 as usize] = rcos + (*axis).x * (*axis).x * (1 as libc::c_float - rcos);
        (*pOut).mat[1 as usize] =
            (*axis).z * rsin + (*axis).y * (*axis).x * (1 as libc::c_float - rcos);
        (*pOut).mat[2 as usize] =
            -(*axis).y * rsin + (*axis).z * (*axis).x * (1 as libc::c_float - rcos);
        (*pOut).mat[3 as usize] =
            -(*axis).z * rsin + (*axis).x * (*axis).y * (1 as libc::c_float - rcos);
        (*pOut).mat[4 as usize] = rcos + (*axis).y * (*axis).y * (1 as libc::c_float - rcos);
        (*pOut).mat[5 as usize] =
            (*axis).x * rsin + (*axis).z * (*axis).y * (1 as libc::c_float - rcos);
        (*pOut).mat[6 as usize] =
            (*axis).y * rsin + (*axis).x * (*axis).z * (1 as libc::c_float - rcos);
        (*pOut).mat[7 as usize] =
            -(*axis).x * rsin + (*axis).y * (*axis).z * (1 as libc::c_float - rcos);
        (*pOut).mat[8 as usize] = rcos + (*axis).z * (*axis).z * (1 as libc::c_float - rcos);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3RotationToAxisAngle(
    mut pAxis: *mut kmVec3,
    mut radians: *mut libc::c_float,
    mut pIn: *const kmMat3,
) -> *mut kmVec3 {
    unsafe {
        let mut temp = kmQuaternion {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        };
        kmQuaternionRotationMatrix(&mut temp, pIn);
        kmQuaternionToAxisAngle(&mut temp, pAxis, radians);
        return pAxis;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3RotationX(mut pOut: *mut kmMat3, radians: libc::c_float) -> *mut kmMat3 {
    unsafe {
        (*pOut).mat[0 as usize] = 1.0f32;
        (*pOut).mat[1 as usize] = 0.0f32;
        (*pOut).mat[2 as usize] = 0.0f32;
        (*pOut).mat[3 as usize] = 0.0f32;
        (*pOut).mat[4 as usize] = cosf(radians);
        (*pOut).mat[5 as usize] = sinf(radians);
        (*pOut).mat[6 as usize] = 0.0f32;
        (*pOut).mat[7 as usize] = -sinf(radians);
        (*pOut).mat[8 as usize] = cosf(radians);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3RotationY(mut pOut: *mut kmMat3, radians: libc::c_float) -> *mut kmMat3 {
    unsafe {
        (*pOut).mat[0 as usize] = cosf(radians);
        (*pOut).mat[1 as usize] = 0.0f32;
        (*pOut).mat[2 as usize] = -sinf(radians);
        (*pOut).mat[3 as usize] = 0.0f32;
        (*pOut).mat[4 as usize] = 1.0f32;
        (*pOut).mat[5 as usize] = 0.0f32;
        (*pOut).mat[6 as usize] = sinf(radians);
        (*pOut).mat[7 as usize] = 0.0f32;
        (*pOut).mat[8 as usize] = cosf(radians);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3RotationZ(mut pOut: *mut kmMat3, radians: libc::c_float) -> *mut kmMat3 {
    unsafe {
        (*pOut).mat[0 as usize] = cosf(radians);
        (*pOut).mat[1 as usize] = -sinf(radians);
        (*pOut).mat[2 as usize] = 0.0f32;
        (*pOut).mat[3 as usize] = sinf(radians);
        (*pOut).mat[4 as usize] = cosf(radians);
        (*pOut).mat[5 as usize] = 0.0f32;
        (*pOut).mat[6 as usize] = 0.0f32;
        (*pOut).mat[7 as usize] = 0.0f32;
        (*pOut).mat[8 as usize] = 1.0f32;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3GetUpVec3(mut pOut: *mut kmVec3, mut pIn: *const kmMat3) -> *mut kmVec3 {
    unsafe {
        (*pOut).x = (*pIn).mat[3 as usize];
        (*pOut).y = (*pIn).mat[4 as usize];
        (*pOut).z = (*pIn).mat[5 as usize];
        kmVec3Normalize(pOut, pOut);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3GetRightVec3(mut pOut: *mut kmVec3, mut pIn: *const kmMat3) -> *mut kmVec3 {
    unsafe {
        (*pOut).x = (*pIn).mat[0 as usize];
        (*pOut).y = (*pIn).mat[1 as usize];
        (*pOut).z = (*pIn).mat[2 as usize];
        kmVec3Normalize(pOut, pOut);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3GetForwardVec3(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmMat3,
) -> *mut kmVec3 {
    unsafe {
        (*pOut).x = (*pIn).mat[6 as usize];
        (*pOut).y = (*pIn).mat[7 as usize];
        (*pOut).z = (*pIn).mat[8 as usize];
        kmVec3Normalize(pOut, pOut);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmMat3LookAt(
    mut pOut: *mut kmMat3,
    mut pEye: *const kmVec3,
    mut pCenter: *const kmVec3,
    mut pUp: *const kmVec3,
) -> *mut kmMat3 {
    unsafe {
        let mut f = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut up = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut s = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let mut u = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        kmVec3Subtract(&mut f, pCenter, pEye);
        kmVec3Normalize(&mut f, &mut f);
        kmVec3Assign(&mut up, pUp);
        kmVec3Normalize(&mut up, &mut up);
        kmVec3Cross(&mut s, &mut f, &mut up);
        kmVec3Normalize(&mut s, &mut s);
        kmVec3Cross(&mut u, &mut s, &mut f);
        kmVec3Normalize(&mut s, &mut s);
        (*pOut).mat[0 as usize] = s.x;
        (*pOut).mat[3 as usize] = s.y;
        (*pOut).mat[6 as usize] = s.z;
        (*pOut).mat[1 as usize] = u.x;
        (*pOut).mat[4 as usize] = u.y;
        (*pOut).mat[7 as usize] = u.z;
        (*pOut).mat[2 as usize] = -f.x;
        (*pOut).mat[5 as usize] = -f.y;
        (*pOut).mat[8 as usize] = -f.z;
        return pOut;
    }
}
