use libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn kmSQR(s: libc::c_float) -> libc::c_float;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmMat4 {
    pub mat: [libc::c_float; 16],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct kmVec4 {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
    pub w: libc::c_float,
}
#[no_mangle]
pub extern "C" fn kmVec4Fill(
    mut pOut: *mut kmVec4,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
    mut w: libc::c_float,
) -> *mut kmVec4 {
    unsafe {
        (*pOut).x = x;
        (*pOut).y = y;
        (*pOut).z = z;
        (*pOut).w = w;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec4Add(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> *mut kmVec4 {
    unsafe {
        (*pOut).x = (*pV1).x + (*pV2).x;
        (*pOut).y = (*pV1).y + (*pV2).y;
        (*pOut).z = (*pV1).z + (*pV2).z;
        (*pOut).w = (*pV1).w + (*pV2).w;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec4Dot(mut pV1: *const kmVec4, mut pV2: *const kmVec4) -> libc::c_float {
    unsafe {
        return (*pV1).x * (*pV2).x
            + (*pV1).y * (*pV2).y
            + (*pV1).z * (*pV2).z
            + (*pV1).w * (*pV2).w;
    }
}

#[no_mangle]
pub extern "C" fn kmVec4Length(mut pIn: *const kmVec4) -> libc::c_float {
    unsafe {
        return sqrtf(kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z) + kmSQR((*pIn).w));
    }
}

#[no_mangle]
pub extern "C" fn kmVec4LengthSq(mut pIn: *const kmVec4) -> libc::c_float {
    unsafe {
        return kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z) + kmSQR((*pIn).w);
    }
}

#[no_mangle]
pub extern "C" fn kmVec4Lerp(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
    mut t: libc::c_float,
) -> *mut kmVec4 {
    unsafe {
        (*pOut).x = (*pV1).x + t * ((*pV2).x - (*pV1).x);
        (*pOut).y = (*pV1).y + t * ((*pV2).y - (*pV1).y);
        (*pOut).z = (*pV1).z + t * ((*pV2).z - (*pV1).z);
        (*pOut).w = (*pV1).w + t * ((*pV2).w - (*pV1).w);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec4Normalize(mut pOut: *mut kmVec4, mut pIn: *const kmVec4) -> *mut kmVec4 {
    unsafe {
        if (*pIn).x == 0. && (*pIn).y == 0. && (*pIn).z == 0. && (*pIn).w == 0. {
            return kmVec4Assign(pOut, pIn);
        }
        let mut l = 1.0f32 / kmVec4Length(pIn);
        (*pOut).x = (*pIn).x * l;
        (*pOut).y = (*pIn).y * l;
        (*pOut).z = (*pIn).z * l;
        (*pOut).w = (*pIn).w * l;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec4Scale(
    mut pOut: *mut kmVec4,
    mut pIn: *const kmVec4,
    s: libc::c_float,
) -> *mut kmVec4 {
    unsafe {
        kmVec4Normalize(pOut, pIn);
        (*pOut).x *= s;
        (*pOut).y *= s;
        (*pOut).z *= s;
        (*pOut).w *= s;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec4Subtract(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> *mut kmVec4 {
    unsafe {
        (*pOut).x = (*pV1).x - (*pV2).x;
        (*pOut).y = (*pV1).y - (*pV2).y;
        (*pOut).z = (*pV1).z - (*pV2).z;
        (*pOut).w = (*pV1).w - (*pV2).w;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec4Mul(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> *mut kmVec4 {
    unsafe {
        (*pOut).x = (*pV1).x * (*pV2).x;
        (*pOut).y = (*pV1).y * (*pV2).y;
        (*pOut).z = (*pV1).z * (*pV2).z;
        (*pOut).w = (*pV1).w * (*pV2).w;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec4Div(
    mut pOut: *mut kmVec4,
    mut pV1: *const kmVec4,
    mut pV2: *const kmVec4,
) -> *mut kmVec4 {
    unsafe {
        if (*pV2).x != 0. && (*pV2).y != 0. && (*pV2).z != 0. && (*pV2).w != 0. {
            (*pOut).x = (*pV1).x / (*pV2).x;
            (*pOut).y = (*pV1).y / (*pV2).y;
            (*pOut).z = (*pV1).z / (*pV2).z;
            (*pOut).w = (*pV1).w / (*pV2).w;
        }
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec4MultiplyMat4(
    mut pOut: *mut kmVec4,
    mut pV: *const kmVec4,
    mut pM: *const kmMat4,
) -> *mut kmVec4 {
    unsafe {
        (*pOut).x = (*pV).x * (*pM).mat[0 as usize]
            + (*pV).y * (*pM).mat[4 as usize]
            + (*pV).z * (*pM).mat[8 as usize]
            + (*pV).w * (*pM).mat[12 as usize];
        (*pOut).y = (*pV).x * (*pM).mat[1 as usize]
            + (*pV).y * (*pM).mat[5 as usize]
            + (*pV).z * (*pM).mat[9 as usize]
            + (*pV).w * (*pM).mat[13 as usize];
        (*pOut).z = (*pV).x * (*pM).mat[2 as usize]
            + (*pV).y * (*pM).mat[6 as usize]
            + (*pV).z * (*pM).mat[10 as usize]
            + (*pV).w * (*pM).mat[14 as usize];
        (*pOut).w = (*pV).x * (*pM).mat[3 as usize]
            + (*pV).y * (*pM).mat[7 as usize]
            + (*pV).z * (*pM).mat[11 as usize]
            + (*pV).w * (*pM).mat[15 as usize];
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec4Transform(
    mut pOut: *mut kmVec4,
    mut pV: *const kmVec4,
    mut pM: *const kmMat4,
) -> *mut kmVec4 {
    unsafe {
        return kmVec4MultiplyMat4(pOut, pV, pM);
    }
}

#[no_mangle]
pub extern "C" fn kmVec4TransformArray(
    mut pOut: *mut kmVec4,
    mut outStride: u32,
    mut pV: *const kmVec4,
    mut vStride: u32,
    mut pM: *const kmMat4,
    mut count: u32,
) -> *mut kmVec4 {
    unsafe {
        let mut i = 0;
        while i < count {
            let mut in_0 = pV.offset(i.wrapping_mul(vStride) as isize);
            let mut out = pOut.offset(i.wrapping_mul(outStride) as isize);
            kmVec4Transform(out, in_0, pM);
            i = i.wrapping_add(1);
        }
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec4AreEqual(mut p1: *const kmVec4, mut p2: *const kmVec4) -> i32 {
    unsafe {
        return (((*p1).x as f64) < (*p2).x as f64 + 0.0001f64
            && (*p1).x as f64 > (*p2).x as f64 - 0.0001f64
            && (((*p1).y as f64) < (*p2).y as f64 + 0.0001f64
                && (*p1).y as f64 > (*p2).y as f64 - 0.0001f64)
            && (((*p1).z as f64) < (*p2).z as f64 + 0.0001f64
                && (*p1).z as f64 > (*p2).z as f64 - 0.0001f64)
            && (((*p1).w as f64) < (*p2).w as f64 + 0.0001f64
                && (*p1).w as f64 > (*p2).w as f64 - 0.0001f64)) as i32;
    }
}

#[no_mangle]
pub extern "C" fn kmVec4Assign(mut pOut: *mut kmVec4, mut pIn: *const kmVec4) -> *mut kmVec4 {
    unsafe {
        if pOut != pIn as *mut kmVec4 {
        } else {
            __assert_fail(
                b"pOut != pIn\0" as *const u8 as *const i8,
                b"../kazmath/vec4.c\0" as *const u8 as *const i8,
                176,
                (*::std::mem::transmute::<&[u8; 47], &[i8; 47]>(
                    b"kmVec4 *kmVec4Assign(kmVec4 *, const kmVec4 *)\0",
                ))
                .as_ptr(),
            );
        }
        memcpy(
            pOut as *mut libc::c_void,
            pIn as *const libc::c_void,
            (::std::mem::size_of::<libc::c_float>() as u64).wrapping_mul(4),
        );
        return pOut;
    }
}
