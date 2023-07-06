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
    fn rand() -> i32;
    fn sqrt(_: f64) -> f64;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct vec {
    pub x: f64,
    pub y: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct polygon_t {
    pub n: i32,
    pub v: *mut vec,
}
pub type polygon = *mut polygon_t;
#[no_mangle]
pub extern "C" fn vsub(mut a: vec, mut b: vec) -> vec {
    let mut c: vec = vec { x: 0., y: 0. };
    c.x = a.x - b.x;
    c.y = a.y - b.y;
    c
}

#[no_mangle]
pub extern "C" fn vadd(mut a: vec, mut b: vec) -> vec {
    let mut c: vec = vec { x: 0., y: 0. };
    c.x = a.x + b.x;
    c.y = a.y + b.y;
    c
}

#[no_mangle]
pub extern "C" fn vdot(mut a: vec, mut b: vec) -> f64 {
    a.x.mul_add(b.x, a.y * b.y)
}

#[no_mangle]
pub extern "C" fn vcross(mut a: vec, mut b: vec) -> f64 {
    a.x.mul_add(b.y, -a.y * b.x)
}

#[no_mangle]
pub extern "C" fn vmadd(mut a: vec, mut s: f64, mut b: vec) -> vec {
    let mut c: vec = vec { x: 0., y: 0. };
    c.x = s.mul_add(b.x, a.x);
    c.y = s.mul_add(b.y, a.y);
    c
}

#[no_mangle]
pub extern "C" fn intersect(
    mut x0: vec,
    mut x1: vec,
    mut y0: vec,
    mut y1: vec,
    mut tol: f64,
    mut sect: *mut vec,
) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut dx: vec = vsub(x1, x0);
        let mut dy: vec = vsub(y1, y0);
        let mut d: f64 = vcross(dy, dx);
        let mut a: f64 = 0.;
        if d == 0.0_f64 {
            return 0_i32;
        }
        a = (vcross(x0, dx) - vcross(y0, dx)) / d;
        if !sect.is_null() {
            *sect = vmadd(y0, a, dy);
        }
        if a < -tol || a > 1_f64 + tol {
            return -1_i32;
        }
        if a < tol || a > 1_f64 - tol {
            return 0_i32;
        }
        a = (vcross(x0, dy) - vcross(y0, dy)) / d;
        if a < f64::from(0_i32) || a > 1_f64 {
            return -1_i32;
        }
        1_i32
    }
}

#[no_mangle]
pub extern "C" fn dist(mut x: vec, mut y0: vec, mut y1: vec, mut tol: f64) -> f64 {
    let mut dy: vec = vsub(y1, y0);
    let mut x1: vec = vec { x: 0., y: 0. };
    let mut s: vec = vec { x: 0., y: 0. };
    let mut r: i32 = 0;
    x1.x = x.x + dy.y;
    x1.y = x.y - dy.x;
    r = intersect(x, x1, y0, y1, tol, &mut s);
    if r == -1_i32 {
        return ::core::f64::INFINITY;
    }
    s = vsub(s, x);
// SAFETY: machine generated unsafe code
    unsafe {
        sqrt(vdot(s, s))
    }
}

#[no_mangle]
pub extern "C" fn inside(mut v: vec, mut p: polygon, mut tol: f64) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut k: i32 = 0;
        let mut crosses: i32 = 0;
        let mut intersectResult: i32 = 0;
        let mut pv: *mut vec = std::ptr::null_mut::<vec>();
        let mut min_x: f64 = 0.;
        let mut max_x: f64 = 0.;
        let mut min_y: f64 = 0.;
        let mut max_y: f64 = 0.;
        i = 0_i32;
        while i < (*p).n {
            k = (i + 1_i32) % (*p).n;
            min_x = dist(
                v,
                *((*p).v).offset(i as isize),
                *((*p).v).offset(k as isize),
                tol,
            );
            if min_x < tol {
                return 0_i32;
            }
            i += 1_i32;
            i;
        }
        max_x = (*((*p).v).offset(0_isize)).x;
        min_x = max_x;
        max_y = (*((*p).v).offset(1_isize)).y;
        min_y = max_y;
        i = 0_i32;
        pv = (*p).v;
        while i < (*p).n {
            if (*pv).x > max_x {
                max_x = (*pv).x;
            }
            if (*pv).x < min_x {
                min_x = (*pv).x;
            }
            if (*pv).y > max_y {
                max_y = (*pv).y;
            }
            if (*pv).y < min_y {
                min_y = (*pv).y;
            }
            i += 1_i32;
            i;
            pv = pv.offset(1);
            pv;
        }
        if v.x < min_x || v.x > max_x || v.y < min_y || v.y > max_y {
            return -1_i32;
        }
        max_x -= min_x;
        max_x *= 2_f64;
        max_y -= min_y;
        max_y *= 2_f64;
        max_x += max_y;
        let mut e: vec = vec { x: 0., y: 0. };
        loop {
            crosses = 0_i32;
            e.x = (1_f64 + f64::from(rand()) / (2147483647_f64 + 1.0f64)).mul_add(max_x, v.x);
            e.y = (1_f64 + f64::from(rand()) / (2147483647_f64 + 1.0f64)).mul_add(max_x, v.y);
            i = 0_i32;
            while i < (*p).n {
                k = (i + 1_i32) % (*p).n;
                intersectResult = intersect(
                    v,
                    e,
                    *((*p).v).offset(i as isize),
                    *((*p).v).offset(k as isize),
                    tol,
                    std::ptr::null_mut::<vec>(),
                );
                if intersectResult == 0_i32 {
                    break;
                }
                if intersectResult == 1_i32 {
                    crosses += 1_i32;
                    crosses;
                }
                i += 1_i32;
                i;
            }
            if i == (*p).n {
                break;
            }
        }
        if crosses & 1_i32 != 0_i32 { 1_i32 } else { -1_i32 }
    }
}

fn main_0() -> i32 {
    let mut vsq: [vec; 8] = [
        {
            
            vec {
                x: f64::from(0_i32),
                y: f64::from(0_i32),
            }
        },
        {
            
            vec {
                x: 10_f64,
                y: f64::from(0_i32),
            }
        },
        {
            
            vec {
                x: 10_f64,
                y: 10_f64,
            }
        },
        {
            
            vec {
                x: f64::from(0_i32),
                y: 10_f64,
            }
        },
        {
            
            vec {
                x: 2.5f64,
                y: 2.5f64,
            }
        },
        {
            
            vec {
                x: 7.5f64,
                y: 0.1f64,
            }
        },
        {
            
            vec {
                x: 7.5f64,
                y: 7.5f64,
            }
        },
        {
            
            vec {
                x: 2.5f64,
                y: 7.5f64,
            }
        },
    ];
    let mut sq: polygon_t = {
        
        polygon_t {
            n: 4,
            v: vsq.as_mut_ptr(),
        }
    };
    let mut sq_hole: polygon_t = {
        
        polygon_t {
            n: 8,
            v: vsq.as_mut_ptr(),
        }
    };
    let mut c: vec = {
        
        vec {
            x: 10_f64,
            y: 5_f64,
        }
    };
    let mut d: vec = {
        
        vec {
            x: 5_f64,
            y: 5_f64,
        }
    };
    println!("{}", inside(c, &mut sq, 1e-10f64));
    println!("{}", inside(c, &mut sq_hole, 1e-10f64));
    println!("{}", inside(d, &mut sq, 1e-10f64));
    println!("{}", inside(d, &mut sq_hole, 1e-10f64));
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
