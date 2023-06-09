use libc;
extern "C" {
    fn atan2(_: f64, _: f64) -> f64;
    fn cos(_: f64) -> f64;
    fn sin(_: f64) -> f64;
    fn sqrtf(_: libc::c_float) -> libc::c_float;
    fn sqrt(_: f64) -> f64;
    fn kmRadiansToDegrees(radians: libc::c_float) -> libc::c_float;
    fn kmDegreesToRadians(degrees: libc::c_float) -> libc::c_float;
    fn kmSQR(s: libc::c_float) -> libc::c_float;
    fn kmVec4Fill(
        pOut: *mut kmVec4,
        x: libc::c_float,
        y: libc::c_float,
        z: libc::c_float,
        w: libc::c_float,
    ) -> *mut kmVec4;
    fn kmVec4Transform(pOut: *mut kmVec4, pV: *const kmVec4, pM: *const kmMat4) -> *mut kmVec4;
    fn kmRay3IntersectPlane(pOut: *mut kmVec3, ray: *const kmRay3, plane: *const kmPlane) -> u8;
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
pub struct kmPlane {
    pub a: libc::c_float,
    pub b: libc::c_float,
    pub c: libc::c_float,
    pub d: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kmRay3 {
    pub start: kmVec3,
    pub dir: kmVec3,
}
#[no_mangle]
pub static mut KM_VEC3_POS_Z: kmVec3 = {
    let mut init = kmVec3 {
        x: 0 as libc::c_float,
        y: 0 as libc::c_float,
        z: 1 as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC3_NEG_Z: kmVec3 = {
    let mut init = kmVec3 {
        x: 0 as libc::c_float,
        y: 0 as libc::c_float,
        z: -1i32 as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC3_POS_Y: kmVec3 = {
    let mut init = kmVec3 {
        x: 0 as libc::c_float,
        y: 1 as libc::c_float,
        z: 0 as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC3_NEG_Y: kmVec3 = {
    let mut init = kmVec3 {
        x: 0 as libc::c_float,
        y: -1i32 as libc::c_float,
        z: 0 as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC3_NEG_X: kmVec3 = {
    let mut init = kmVec3 {
        x: -1i32 as libc::c_float,
        y: 0 as libc::c_float,
        z: 0 as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC3_POS_X: kmVec3 = {
    let mut init = kmVec3 {
        x: 1 as libc::c_float,
        y: 0 as libc::c_float,
        z: 0 as libc::c_float,
    };
    init
};
#[no_mangle]
pub static mut KM_VEC3_ZERO: kmVec3 = {
    let mut init = kmVec3 {
        x: 0 as libc::c_float,
        y: 0 as libc::c_float,
        z: 0 as libc::c_float,
    };
    init
};
#[no_mangle]
pub extern "C" fn kmVec3Fill(
    mut pOut: *mut kmVec3,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
) -> *mut kmVec3 {
    unsafe {
        (*pOut).x = x;
        (*pOut).y = y;
        (*pOut).z = z;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Length(mut pIn: *const kmVec3) -> libc::c_float {
    unsafe {
        return sqrtf(kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z));
    }
}

#[no_mangle]
pub extern "C" fn kmVec3LengthSq(mut pIn: *const kmVec3) -> libc::c_float {
    unsafe {
        return kmSQR((*pIn).x) + kmSQR((*pIn).y) + kmSQR((*pIn).z);
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Lerp(
    mut pOut: *mut kmVec3,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
    mut t: libc::c_float,
) -> *mut kmVec3 {
    unsafe {
        (*pOut).x = (*pV1).x + t * ((*pV2).x - (*pV1).x);
        (*pOut).y = (*pV1).y + t * ((*pV2).y - (*pV1).y);
        (*pOut).z = (*pV1).z + t * ((*pV2).z - (*pV1).z);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Normalize(mut pOut: *mut kmVec3, mut pIn: *const kmVec3) -> *mut kmVec3 {
    unsafe {
        if (*pIn).x == 0. && (*pIn).y == 0. && (*pIn).z == 0. {
            return kmVec3Assign(pOut, pIn);
        }
        let mut l = 1.0f32 / kmVec3Length(pIn);
        let mut v = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        v.x = (*pIn).x * l;
        v.y = (*pIn).y * l;
        v.z = (*pIn).z * l;
        (*pOut).x = v.x;
        (*pOut).y = v.y;
        (*pOut).z = v.z;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Cross(
    mut pOut: *mut kmVec3,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
) -> *mut kmVec3 {
    unsafe {
        let mut v = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        v.x = (*pV1).y * (*pV2).z - (*pV1).z * (*pV2).y;
        v.y = (*pV1).z * (*pV2).x - (*pV1).x * (*pV2).z;
        v.z = (*pV1).x * (*pV2).y - (*pV1).y * (*pV2).x;
        (*pOut).x = v.x;
        (*pOut).y = v.y;
        (*pOut).z = v.z;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Dot(mut pV1: *const kmVec3, mut pV2: *const kmVec3) -> libc::c_float {
    unsafe {
        return (*pV1).x * (*pV2).x + (*pV1).y * (*pV2).y + (*pV1).z * (*pV2).z;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Add(
    mut pOut: *mut kmVec3,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
) -> *mut kmVec3 {
    unsafe {
        let mut v = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        v.x = (*pV1).x + (*pV2).x;
        v.y = (*pV1).y + (*pV2).y;
        v.z = (*pV1).z + (*pV2).z;
        (*pOut).x = v.x;
        (*pOut).y = v.y;
        (*pOut).z = v.z;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Subtract(
    mut pOut: *mut kmVec3,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
) -> *mut kmVec3 {
    unsafe {
        let mut v = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        v.x = (*pV1).x - (*pV2).x;
        v.y = (*pV1).y - (*pV2).y;
        v.z = (*pV1).z - (*pV2).z;
        (*pOut).x = v.x;
        (*pOut).y = v.y;
        (*pOut).z = v.z;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Mul(
    mut pOut: *mut kmVec3,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
) -> *mut kmVec3 {
    unsafe {
        (*pOut).x = (*pV1).x * (*pV2).x;
        (*pOut).y = (*pV1).y * (*pV2).y;
        (*pOut).z = (*pV1).z * (*pV2).z;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Div(
    mut pOut: *mut kmVec3,
    mut pV1: *const kmVec3,
    mut pV2: *const kmVec3,
) -> *mut kmVec3 {
    unsafe {
        if (*pV2).x != 0. && (*pV2).y != 0. && (*pV2).z != 0. {
            (*pOut).x = (*pV1).x / (*pV2).x;
            (*pOut).y = (*pV1).y / (*pV2).y;
            (*pOut).z = (*pV1).z / (*pV2).z;
        }
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3MultiplyMat3(
    mut pOut: *mut kmVec3,
    mut pV: *const kmVec3,
    mut pM: *const kmMat3,
) -> *mut kmVec3 {
    unsafe {
        let mut v = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        v.x = (*pV).x * (*pM).mat[0 as usize]
            + (*pV).y * (*pM).mat[3 as usize]
            + (*pV).z * (*pM).mat[6 as usize];
        v.y = (*pV).x * (*pM).mat[1 as usize]
            + (*pV).y * (*pM).mat[4 as usize]
            + (*pV).z * (*pM).mat[7 as usize];
        v.z = (*pV).x * (*pM).mat[2 as usize]
            + (*pV).y * (*pM).mat[5 as usize]
            + (*pV).z * (*pM).mat[8 as usize];
        (*pOut).x = v.x;
        (*pOut).y = v.y;
        (*pOut).z = v.z;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3MultiplyMat4(
    mut pOut: *mut kmVec3,
    mut pV: *const kmVec3,
    mut pM: *const kmMat4,
) -> *mut kmVec3 {
    unsafe {
        let mut v = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        v.x = (*pV).x * (*pM).mat[0 as usize]
            + (*pV).y * (*pM).mat[4 as usize]
            + (*pV).z * (*pM).mat[8 as usize]
            + (*pM).mat[12 as usize];
        v.y = (*pV).x * (*pM).mat[1 as usize]
            + (*pV).y * (*pM).mat[5 as usize]
            + (*pV).z * (*pM).mat[9 as usize]
            + (*pM).mat[13 as usize];
        v.z = (*pV).x * (*pM).mat[2 as usize]
            + (*pV).y * (*pM).mat[6 as usize]
            + (*pV).z * (*pM).mat[10 as usize]
            + (*pM).mat[14 as usize];
        (*pOut).x = v.x;
        (*pOut).y = v.y;
        (*pOut).z = v.z;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Transform(
    mut pOut: *mut kmVec3,
    mut pV: *const kmVec3,
    mut pM: *const kmMat4,
) -> *mut kmVec3 {
    unsafe {
        return kmVec3MultiplyMat4(pOut, pV, pM);
    }
}

#[no_mangle]
pub extern "C" fn kmVec3InverseTransform(
    mut pOut: *mut kmVec3,
    mut pVect: *const kmVec3,
    mut pM: *const kmMat4,
) -> *mut kmVec3 {
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
        v1.x = (*pVect).x - (*pM).mat[12 as usize];
        v1.y = (*pVect).y - (*pM).mat[13 as usize];
        v1.z = (*pVect).z - (*pM).mat[14 as usize];
        v2.x = v1.x * (*pM).mat[0 as usize]
            + v1.y * (*pM).mat[1 as usize]
            + v1.z * (*pM).mat[2 as usize];
        v2.y = v1.x * (*pM).mat[4 as usize]
            + v1.y * (*pM).mat[5 as usize]
            + v1.z * (*pM).mat[6 as usize];
        v2.z = v1.x * (*pM).mat[8 as usize]
            + v1.y * (*pM).mat[9 as usize]
            + v1.z * (*pM).mat[10 as usize];
        (*pOut).x = v2.x;
        (*pOut).y = v2.y;
        (*pOut).z = v2.z;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3InverseTransformNormal(
    mut pOut: *mut kmVec3,
    mut pVect: *const kmVec3,
    mut pM: *const kmMat4,
) -> *mut kmVec3 {
    unsafe {
        let mut v = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        v.x = (*pVect).x * (*pM).mat[0 as usize]
            + (*pVect).y * (*pM).mat[1 as usize]
            + (*pVect).z * (*pM).mat[2 as usize];
        v.y = (*pVect).x * (*pM).mat[4 as usize]
            + (*pVect).y * (*pM).mat[5 as usize]
            + (*pVect).z * (*pM).mat[6 as usize];
        v.z = (*pVect).x * (*pM).mat[8 as usize]
            + (*pVect).y * (*pM).mat[9 as usize]
            + (*pVect).z * (*pM).mat[10 as usize];
        (*pOut).x = v.x;
        (*pOut).y = v.y;
        (*pOut).z = v.z;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3TransformCoord(
    mut pOut: *mut kmVec3,
    mut pV: *const kmVec3,
    mut pM: *const kmMat4,
) -> *mut kmVec3 {
    unsafe {
        let mut v = kmVec4 {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        };
        let mut inV = kmVec4 {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        };
        kmVec4Fill(&mut inV, (*pV).x, (*pV).y, (*pV).z, 1.0f64 as libc::c_float);
        kmVec4Transform(&mut v, &mut inV, pM);
        (*pOut).x = v.x / v.w;
        (*pOut).y = v.y / v.w;
        (*pOut).z = v.z / v.w;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3TransformNormal(
    mut pOut: *mut kmVec3,
    mut pV: *const kmVec3,
    mut pM: *const kmMat4,
) -> *mut kmVec3 {
    unsafe {
        let mut v = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        v.x = (*pV).x * (*pM).mat[0 as usize]
            + (*pV).y * (*pM).mat[4 as usize]
            + (*pV).z * (*pM).mat[8 as usize];
        v.y = (*pV).x * (*pM).mat[1 as usize]
            + (*pV).y * (*pM).mat[5 as usize]
            + (*pV).z * (*pM).mat[9 as usize];
        v.z = (*pV).x * (*pM).mat[2 as usize]
            + (*pV).y * (*pM).mat[6 as usize]
            + (*pV).z * (*pM).mat[10 as usize];
        (*pOut).x = v.x;
        (*pOut).y = v.y;
        (*pOut).z = v.z;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Scale(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmVec3,
    s: libc::c_float,
) -> *mut kmVec3 {
    unsafe {
        (*pOut).x = (*pIn).x * s;
        (*pOut).y = (*pIn).y * s;
        (*pOut).z = (*pIn).z * s;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3AreEqual(mut p1: *const kmVec3, mut p2: *const kmVec3) -> i32 {
    unsafe {
        if ((*p1).x as f64) < (*p2).x as f64 + 0.0001f64
            && (*p1).x as f64 > (*p2).x as f64 - 0.0001f64
            && (((*p1).y as f64) < (*p2).y as f64 + 0.0001f64
                && (*p1).y as f64 > (*p2).y as f64 - 0.0001f64)
            && (((*p1).z as f64) < (*p2).z as f64 + 0.0001f64
                && (*p1).z as f64 > (*p2).z as f64 - 0.0001f64)
        {
            return 1;
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Assign(mut pOut: *mut kmVec3, mut pIn: *const kmVec3) -> *mut kmVec3 {
    unsafe {
        if pOut == pIn as *mut kmVec3 {
            return pOut;
        };
        (*pOut).x = (*pIn).x;
        (*pOut).y = (*pIn).y;
        (*pOut).z = (*pIn).z;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Zero(mut pOut: *mut kmVec3) -> *mut kmVec3 {
    unsafe {
        (*pOut).x = 0.0f32;
        (*pOut).y = 0.0f32;
        (*pOut).z = 0.0f32;
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3GetHorizontalAngle(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmVec3,
) -> *mut kmVec3 {
    unsafe {
        let z1 = sqrt(((*pIn).x * (*pIn).x + (*pIn).z * (*pIn).z) as f64) as libc::c_float;
        (*pOut).y = kmRadiansToDegrees(atan2((*pIn).x as f64, (*pIn).z as f64) as libc::c_float);
        if (*pOut).y < 0 as libc::c_float {
            (*pOut).y += 360 as libc::c_float;
        }
        if (*pOut).y >= 360 as libc::c_float {
            (*pOut).y -= 360 as libc::c_float;
        };
        (*pOut).x = (kmRadiansToDegrees(atan2(z1 as f64, (*pIn).y as f64) as libc::c_float) as f64
            - 90.0f64) as libc::c_float;
        if (*pOut).x < 0 as libc::c_float {
            (*pOut).x += 360 as libc::c_float;
        }
        if (*pOut).x >= 360 as libc::c_float {
            (*pOut).x -= 360 as libc::c_float;
        }
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3RotationToDirection(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmVec3,
    mut forwards: *const kmVec3,
) -> *mut kmVec3 {
    unsafe {
        let xr = kmDegreesToRadians((*pIn).x);
        let yr = kmDegreesToRadians((*pIn).y);
        let zr = kmDegreesToRadians((*pIn).z);
        let cr = cos(xr as f64) as libc::c_float;
        let sr = sin(xr as f64) as libc::c_float;
        let cp = cos(yr as f64) as libc::c_float;
        let sp = sin(yr as f64) as libc::c_float;
        let cy = cos(zr as f64) as libc::c_float;
        let sy = sin(zr as f64) as libc::c_float;
        let srsp = sr * sp;
        let crsp = cr * sp;
        let pseudoMatrix: [libc::c_float; 9] = [
            cp * cy,
            cp * sy,
            -sp,
            srsp * cy - cr * sy,
            srsp * sy + cr * cy,
            sr * cp,
            crsp * cy + sr * sy,
            crsp * sy - sr * cy,
            cr * cp,
        ];
        (*pOut).x = (*forwards).x * pseudoMatrix[0 as usize]
            + (*forwards).y * pseudoMatrix[3 as usize]
            + (*forwards).z * pseudoMatrix[6 as usize];
        (*pOut).y = (*forwards).x * pseudoMatrix[1 as usize]
            + (*forwards).y * pseudoMatrix[4 as usize]
            + (*forwards).z * pseudoMatrix[7 as usize];
        (*pOut).z = (*forwards).x * pseudoMatrix[2 as usize]
            + (*forwards).y * pseudoMatrix[5 as usize]
            + (*forwards).z * pseudoMatrix[8 as usize];
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3ProjectOnToPlane(
    mut pOut: *mut kmVec3,
    mut point: *const kmVec3,
    mut plane: *const kmPlane,
) -> *mut kmVec3 {
    unsafe {
        let mut ray = kmRay3 {
            start: kmVec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            dir: kmVec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
        };
        kmVec3Assign(&mut ray.start, point);
        ray.dir.x = -(*plane).a;
        ray.dir.y = -(*plane).b;
        ray.dir.z = -(*plane).c;
        kmRay3IntersectPlane(pOut, &mut ray, plane);
        return pOut;
    }
}

#[no_mangle]
pub extern "C" fn kmVec3Reflect(
    mut pOut: *mut kmVec3,
    mut pIn: *const kmVec3,
    mut normal: *const kmVec3,
) -> *mut kmVec3 {
    unsafe {
        let mut tmp = kmVec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        kmVec3Scale(&mut tmp, normal, 2.0f32 * kmVec3Dot(pIn, normal));
        kmVec3Subtract(pOut, pIn, &mut tmp);
        return pOut;
    }
}
