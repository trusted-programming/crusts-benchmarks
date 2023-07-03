#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct matrix_t {
    pub h: i32,
    pub w: i32,
    pub x: *mut f64,
}
pub type matrix = *mut matrix_t;
#[no_mangle]
pub extern "C" fn dot(mut a: *mut f64, mut b: *mut f64, mut len: i32, mut step: i32) -> f64 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut r: f64 = f64::from(0_i32);
        loop {
            let fresh0 = len;
            len = len.wrapping_sub(1);
            if fresh0 == 0_i32 {
                break;
            }
            let fresh1 = a;
            a = a.offset(1);
            r += *fresh1 * *b;
            b = b.offset(step as isize);
        }
        r
    }
}

#[no_mangle]
pub extern "C" fn mat_new(mut h: i32, mut w: i32) -> matrix {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut r: matrix = malloc(
            (::core::mem::size_of::<matrix_t>() as u64).wrapping_add(
                (::core::mem::size_of::<f64>() as u64)
                    .wrapping_mul(w as u64)
                    .wrapping_mul(h as u64),
            ),
        ).cast::<matrix_t>();
        (*r).h = h;
        (*r).w = w;
        (*r).x = r.offset(1_isize).cast::<f64>();
        r
    }
}

#[no_mangle]
pub extern "C" fn mat_mul(mut a: matrix, mut b: matrix) -> matrix {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut r: matrix = std::ptr::null_mut::<matrix_t>();
        let mut p: *mut f64 = std::ptr::null_mut::<f64>();
        let mut pa: *mut f64 = std::ptr::null_mut::<f64>();
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        if (*a).w != (*b).h {
            return 0 as matrix;
        }
        r = mat_new((*a).h, (*b).w);
        p = (*r).x;
        pa = (*a).x;
        i = 0_i32;
        while i < (*a).h {
            j = 0_i32;
            while j < (*b).w {
                let fresh2 = p;
                p = p.offset(1);
                *fresh2 = dot(pa, ((*b).x).offset(j as isize), (*a).w, (*b).w);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
            pa = pa.offset((*a).w as isize);
        }
        r
    }
}

#[no_mangle]
pub extern "C" fn mat_show(mut a: matrix) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut p: *mut f64 = (*a).x;
        i = 0_i32;
        while i < (*a).h {
            j = 0_i32;
            while j < (*a).w {
                let fresh3 = p;
                p = p.offset(1);
                print!("	{:7.3}", *fresh3);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
            print!("{}", '\n' as i32);
        }
        print!("{}", '\n' as i32);
    }
}

fn main_0() -> i32 {
    let mut da: [f64; 16] = [
        1_f64, 1_f64, 1_f64, 1_f64, 2_f64, 4_f64, 8_f64, 16_f64, 3_f64,
        9_f64, 27_f64, 81_f64, 4_f64, 16_f64, 64_f64, 256_f64,
    ];
    let mut db: [f64; 12] = [
        4.0f64,
        -3.0f64,
        4.0f64 / 3_f64,
        -13.0f64 / 3_f64,
        19.0f64 / 4_f64,
        -7.0f64 / 3_f64,
        3.0f64 / 2_f64,
        -2.0f64,
        7.0f64 / 6_f64,
        -1.0f64 / 6_f64,
        1.0f64 / 4_f64,
        -1.0f64 / 6_f64,
    ];
    let mut a: matrix_t = {
        
        matrix_t {
            h: 4,
            w: 4,
            x: da.as_mut_ptr(),
        }
    };
    let mut b: matrix_t = {
        
        matrix_t {
            h: 4,
            w: 3,
            x: db.as_mut_ptr(),
        }
    };
    let mut c: matrix = mat_mul(&mut a, &mut b);
    mat_show(c);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
