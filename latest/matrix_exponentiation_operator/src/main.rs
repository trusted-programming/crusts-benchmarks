#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn cos(_: f64) -> f64;
    fn sin(_: f64) -> f64;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct squareMtxStruct {
    pub dim: i32,
    pub cells: *mut f64,
    pub m: *mut *mut f64,
}
pub type SquareMtx = *mut squareMtxStruct;
// SAFETY: machine generated unsafe code
pub type FillFunc = Option<unsafe extern "C" fn(*mut f64, i32, i32, *mut libc::c_void) -> ()>;
#[no_mangle]
pub extern "C" fn NewSquareMtx(
    mut dim: i32,
    mut fillFunc: FillFunc,
    mut ff_data: *mut libc::c_void,
) -> SquareMtx {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut sm: SquareMtx =
            malloc(::core::mem::size_of::<squareMtxStruct>() as u64).cast::<squareMtxStruct>();
        if !sm.is_null() {
            let mut rw: i32 = 0;
            (*sm).dim = dim;
            (*sm).cells =
                malloc(((dim * dim) as u64).wrapping_mul(::core::mem::size_of::<f64>() as u64)).cast::<f64>();
            (*sm).m = malloc((dim as u64).wrapping_mul(::core::mem::size_of::<*mut f64>() as u64)).cast::<*mut f64>();
            if !((*sm).cells).is_null() && !((*sm).m).is_null() {
                rw = 0_i32;
                while rw < dim {
                    let fresh0 = &mut (*((*sm).m).offset(rw as isize));
                    *fresh0 = ((*sm).cells).offset((dim * rw) as isize);
                    fillFunc.expect("non-null function pointer")(
                        *((*sm).m).offset(rw as isize),
                        rw,
                        dim,
                        ff_data,
                    );
                    rw += 1_i32;
                    rw;
                }
            } else {
                free((*sm).m.cast::<libc::c_void>());
                free((*sm).cells.cast::<libc::c_void>());
                free(sm.cast::<libc::c_void>());
                println!("Square Matrix allocation failure");
                return 0 as SquareMtx;
            }
        } else {
            println!("Malloc failed for square matrix");
        }
        sm
    }
}

#[no_mangle]
pub extern "C" fn ffMatxSquare(mut cells: *mut f64, mut rw: i32, mut dim: i32, mut m0: SquareMtx) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut col: i32 = 0;
        let mut ix: i32 = 0;
        let mut sum: f64 = 0.;
        let mut m0rw: *mut f64 = *((*m0).m).offset(rw as isize);
        col = 0_i32;
        while col < dim {
            sum = 0.0f64;
            ix = 0_i32;
            while ix < dim {
                sum += *m0rw.offset(ix as isize)
                    * *(*((*m0).m).offset(ix as isize)).offset(col as isize);
                ix += 1_i32;
                ix;
            }
            *cells.offset(col as isize) = sum;
            col += 1_i32;
            col;
        }
    }
}

#[no_mangle]
pub extern "C" fn ffMatxMulply(
    mut cells: *mut f64,
    mut rw: i32,
    mut dim: i32,
    mut mplcnds: *mut SquareMtx,
) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut mleft: SquareMtx = *mplcnds.offset(0_isize);
        let mut mrigt: SquareMtx = *mplcnds.offset(1_isize);
        let mut sum: f64 = 0.;
        let mut m0rw: *mut f64 = *((*mleft).m).offset(rw as isize);
        let mut col: i32 = 0;
        let mut ix: i32 = 0;
        col = 0_i32;
        while col < dim {
            sum = 0.0f64;
            ix = 0_i32;
            while ix < dim {
                sum += *m0rw.offset(ix as isize)
                    * *(*((*mrigt).m).offset(ix as isize)).offset(col as isize);
                ix += 1_i32;
                ix;
            }
            *cells.offset(col as isize) = sum;
            col += 1_i32;
            col;
        }
    }
}

#[no_mangle]
pub extern "C" fn MatxMul(mut mr: SquareMtx, mut left: SquareMtx, mut rigt: SquareMtx) {
    let mut rw: i32 = 0;
    let mut mplcnds: [SquareMtx; 2] = [std::ptr::null_mut::<squareMtxStruct>(); 2];
    mplcnds[0_usize] = left;
    mplcnds[1_usize] = rigt;
    rw = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while rw < (*left).dim {
            ffMatxMulply(
                *((*mr).m).offset(rw as isize),
                rw,
                (*left).dim,
                mplcnds.as_mut_ptr(),
            );
            rw += 1_i32;
            rw;
        }
    }
}

#[no_mangle]
pub extern "C" fn ffIdentity(
    mut cells: *mut f64,
    mut rw: i32,
    mut dim: i32,
    mut _v: *mut libc::c_void,
) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut col: i32 = 0;
        col = 0_i32;
        while col < dim {
            *cells.offset(col as isize) = 0.0f64;
            col += 1_i32;
            col;
        }
        *cells.offset(rw as isize) = 1.0f64;
    }
}

#[no_mangle]
pub extern "C" fn ffCopy(mut cells: *mut f64, mut rw: i32, mut dim: i32, mut m1: SquareMtx) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut col: i32 = 0;
        col = 0_i32;
        while col < dim {
            *cells.offset(col as isize) = *(*((*m1).m).offset(rw as isize)).offset(col as isize);
            col += 1_i32;
            col;
        }
    }
}

#[no_mangle]
pub extern "C" fn FreeSquareMtx(mut m: SquareMtx) {
// SAFETY: machine generated unsafe code
    unsafe {
        free((*m).m.cast::<libc::c_void>());
        free((*m).cells.cast::<libc::c_void>());
        free(m.cast::<libc::c_void>());
    }
}

#[no_mangle]
pub extern "C" fn SquareMtxPow(mut m0: SquareMtx, mut exp: i32) -> SquareMtx {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut v0: SquareMtx = NewSquareMtx(
            (*m0).dim,
// SAFETY: machine generated unsafe code
            Some(ffIdentity as unsafe extern "C" fn(*mut f64, i32, i32, *mut libc::c_void) -> ()),
            std::ptr::null_mut::<libc::c_void>(),
        );
        let mut v1: SquareMtx = 0 as SquareMtx;
        let mut base0: SquareMtx = NewSquareMtx(
            (*m0).dim,
            ::core::mem::transmute::<
// SAFETY: machine generated unsafe code
                Option<unsafe extern "C" fn(*mut f64, i32, i32, SquareMtx) -> ()>,
                FillFunc,
            >(Some(
// SAFETY: machine generated unsafe code
                ffCopy as unsafe extern "C" fn(*mut f64, i32, i32, SquareMtx) -> (),
            )),
            m0.cast::<libc::c_void>(),
        );
        let mut base1: SquareMtx = 0 as SquareMtx;
        let mut mplcnds: [SquareMtx; 2] = [std::ptr::null_mut::<squareMtxStruct>(); 2];
        let mut t: SquareMtx = std::ptr::null_mut::<squareMtxStruct>();
        while exp != 0_i32 {
            if exp % 2_i32 != 0_i32 {
                if !v1.is_null() {
                    MatxMul(v1, v0, base0);
                } else {
                    mplcnds[0_usize] = v0;
                    mplcnds[1_usize] = base0;
                    v1 = NewSquareMtx(
                        (*m0).dim,
                        ::core::mem::transmute::<
// SAFETY: machine generated unsafe code
                            Option<unsafe extern "C" fn(*mut f64, i32, i32, *mut SquareMtx) -> ()>,
                            FillFunc,
                        >(Some(
                            ffMatxMulply
// SAFETY: machine generated unsafe code
                                as unsafe extern "C" fn(*mut f64, i32, i32, *mut SquareMtx) -> (),
                        )),
                        mplcnds.as_mut_ptr().cast::<libc::c_void>(),
                    );
                }
                t = v0;
                v0 = v1;
                v1 = t;
            }
            if !base1.is_null() {
                MatxMul(base1, base0, base0);
            } else {
                base1 = NewSquareMtx(
                    (*m0).dim,
                    ::core::mem::transmute::<
// SAFETY: machine generated unsafe code
                        Option<unsafe extern "C" fn(*mut f64, i32, i32, SquareMtx) -> ()>,
                        FillFunc,
                    >(Some(
// SAFETY: machine generated unsafe code
                        ffMatxSquare as unsafe extern "C" fn(*mut f64, i32, i32, SquareMtx) -> (),
                    )),
                    base0.cast::<libc::c_void>(),
                );
            }
            t = base0;
            base0 = base1;
            base1 = t;
            exp /= 2_i32;
        }
        if !base0.is_null() {
            FreeSquareMtx(base0);
        }
        if !base1.is_null() {
            FreeSquareMtx(base1);
        }
        if !v1.is_null() {
            FreeSquareMtx(v1);
        }
        v0
    }
}

#[no_mangle]
pub static mut fout: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub extern "C" fn SquareMtxPrint(mut mtx: SquareMtx, mut mn: *const i8) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut rw: i32 = 0;
        let mut col: i32 = 0;
        let mut d: i32 = (*mtx).dim;
        fprintf(
            fout,
            (b"%s dim:%d =\n\0" as *const u8).cast::<i8>(),
            mn,
            (*mtx).dim,
        );
        rw = 0_i32;
        while rw < d {
            fprintf(fout, (b" |\0" as *const u8).cast::<i8>());
            col = 0_i32;
            while col < d {
                fprintf(
                    fout,
                    (b"%8.5f \0" as *const u8).cast::<i8>(),
                    *(*((*mtx).m).offset(rw as isize)).offset(col as isize),
                );
                col += 1_i32;
                col;
            }
            fprintf(fout, (b" |\n\0" as *const u8).cast::<i8>());
            rw += 1_i32;
            rw;
        }
        fprintf(fout, (b"\n\0" as *const u8).cast::<i8>());
    }
}

#[no_mangle]
pub extern "C" fn fillInit(
    mut cells: *mut f64,
    mut rw: i32,
    mut _dim: i32,
    mut _data: *mut libc::c_void,
) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut theta: f64 = 3.1415926536f64 / 6.0f64;
        let mut c1: f64 = cos(theta);
        let mut s1: f64 = sin(theta);
        match rw {
            0_i32 => {
                *cells.offset(0_isize) = c1;
                *cells.offset(1_isize) = s1;
                *cells.offset(2_isize) = 0.0f64;
            }
            1_i32 => {
                *cells.offset(0_isize) = -s1;
                *cells.offset(1_isize) = c1;
                *cells.offset(2_isize) = f64::from(0_i32);
            }
            2_i32 => {
                *cells.offset(0_isize) = 0.0f64;
                *cells.offset(1_isize) = 0.0f64;
                *cells.offset(2_isize) = 1.0f64;
            }
            _ => {}
        };
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut m0: SquareMtx = NewSquareMtx(
            3,
// SAFETY: machine generated unsafe code
            Some(fillInit as unsafe extern "C" fn(*mut f64, i32, i32, *mut libc::c_void) -> ()),
            std::ptr::null_mut::<libc::c_void>(),
        );
        let mut m1: SquareMtx = SquareMtxPow(m0, 5);
        let mut m2: SquareMtx = SquareMtxPow(m0, 9);
        let mut m3: SquareMtx = SquareMtxPow(m0, 2);
        fout = fopen(
            (b"matrx_exp.txt\0" as *const u8).cast::<i8>(),
            (b"w\0" as *const u8).cast::<i8>(),
        );
        SquareMtxPrint(m0, (b"m0\0" as *const u8).cast::<i8>());
        FreeSquareMtx(m0);
        SquareMtxPrint(m1, (b"m0^5\0" as *const u8).cast::<i8>());
        FreeSquareMtx(m1);
        SquareMtxPrint(m2, (b"m0^9\0" as *const u8).cast::<i8>());
        FreeSquareMtx(m2);
        SquareMtxPrint(m3, (b"m0^2\0" as *const u8).cast::<i8>());
        FreeSquareMtx(m3);
        fclose(fout);
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
