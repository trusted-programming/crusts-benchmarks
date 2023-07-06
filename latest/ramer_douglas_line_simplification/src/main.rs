#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]

extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn sqrt(_: f64) -> f64;
    fn fabs(_: f64) -> f64;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct point_tag {
    pub x: f64,
    pub y: f64,
}
pub type point_t = point_tag;
#[no_mangle]
pub extern "C" fn perpendicular_distance(mut p: point_t, mut p1: point_t, mut p2: point_t) -> f64 {
    let mut dx: f64 = p2.x - p1.x;
    let mut dy: f64 = p2.y - p1.y;
// SAFETY: machine generated unsafe code
    unsafe {
        let mut d: f64 = sqrt(dx.mul_add(dx, dy * dy));
        fabs(p2.y.mul_add(-p1.x, p2.x.mul_add(p1.y, p.x.mul_add(dy, -p.y * dx)))) / d
    }
}

#[no_mangle]
pub extern "C" fn douglas_peucker(
    mut points: *const point_t,
    mut n: u64,
    mut epsilon: f64,
    mut dest: *mut point_t,
    mut destlen: u64,
) -> u64 {
// SAFETY: machine generated unsafe code
    unsafe {
        if n >= 2 {
        } else {
            __assert_fail(
                (b"n >= 2\0" as *const u8).cast::<i8>(),
                (b"main.c\0" as *const u8).cast::<i8>(),
                22,
                (*::core::mem::transmute::<&[u8; 75], &[i8; 75]>(
                    b"size_t douglas_peucker(const point_t *, size_t, double, point_t *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
        if n >= 2 {
            } else {
                __assert_fail ((b"n >= 2\0" as * const u8).cast::<i8>(), (b"main.c\0" as * const u8).cast::<i8>(), 22, (* :: core :: mem :: transmute :: < & [u8; 75], & [i8; 75] > (
                  b"size_t douglas_peucker(const point_t *, size_t, double, point_t *, size_t)\0",)).as_ptr (),);
            };
        if epsilon >= f64::from(0_i32) {
        } else {
            __assert_fail(
                (b"epsilon >= 0\0" as *const u8).cast::<i8>(),
                (b"main.c\0" as *const u8).cast::<i8>(),
                23,
                (*::core::mem::transmute::<&[u8; 75], &[i8; 75]>(
                    b"size_t douglas_peucker(const point_t *, size_t, double, point_t *, size_t)\0",
                ))
                .as_ptr(),
            );
        }
        if epsilon >= f64::from(0_i32) {
            } else {
                __assert_fail ((b"epsilon >= 0\0" as * const u8).cast::<i8>(), (b"main.c\0" as * const u8).cast::<i8>(), 23, (* :: core :: mem :: transmute :: < & [u8; 75], & [i8; 75] > (
                  b"size_t douglas_peucker(const point_t *, size_t, double, point_t *, size_t)\0",)).as_ptr (),);
            };
        let mut max_dist: f64 = f64::from(0_i32);
        let mut index: u64 = 0;
        let mut i: u64 = 1;
        while i.wrapping_add(1) < n {
            let mut dist: f64 = perpendicular_distance(
                *points.offset(i as isize),
                *points.offset(0_isize),
                *points.offset(n.wrapping_sub(1) as isize),
            );
            if dist > max_dist {
                max_dist = dist;
                index = i;
            }
            i = i.wrapping_add(1);
            i;
        }
        if max_dist > epsilon {
            let mut n1: u64 =
                douglas_peucker(points, index.wrapping_add(1), epsilon, dest, destlen);
            if destlen >= n1.wrapping_sub(1) {
                destlen = destlen.wrapping_sub(n1.wrapping_sub(1));
                dest = dest.offset(n1.wrapping_sub(1) as isize);
            } else {
                destlen = 0;
            }
            let mut n2: u64 = douglas_peucker(
                points.offset(index as isize),
                n.wrapping_sub(index),
                epsilon,
                dest,
                destlen,
            );
            return n1.wrapping_add(n2).wrapping_sub(1);
        }
        if destlen >= 2 {
            *dest.offset(0_isize) = *points.offset(0_isize);
            *dest.offset(1_isize) = *points.offset(n.wrapping_sub(1) as isize);
        }
        2
    }
}

#[no_mangle]
pub extern "C" fn print_points(mut points: *const point_t, mut n: u64) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: u64 = 0;
        while i < n {
            if i > 0 {
                print!(" ");
            }
            print!(
                "({}, {})",
                (*points.offset(i as isize)).x,
                (*points.offset(i as isize)).y
            );
            i = i.wrapping_add(1);
            i;
        }
        println!();
    }
}

fn main_0() -> i32 {
    let mut points: [point_t; 10] = [
        {
            
            point_tag {
                x: f64::from(0_i32),
                y: f64::from(0_i32),
            }
        },
        {
            
            point_tag {
                x: 1_f64,
                y: 0.1f64,
            }
        },
        {
            
            point_tag {
                x: 2_f64,
                y: -0.1f64,
            }
        },
        {
            
            point_tag {
                x: 3_f64,
                y: 5_f64,
            }
        },
        {
            
            point_tag {
                x: 4_f64,
                y: 6_f64,
            }
        },
        {
            
            point_tag {
                x: 5_f64,
                y: 7_f64,
            }
        },
        {
            
            point_tag {
                x: 6_f64,
                y: 8.1f64,
            }
        },
        {
            
            point_tag {
                x: 7_f64,
                y: 9_f64,
            }
        },
        {
            
            point_tag {
                x: 8_f64,
                y: 9_f64,
            }
        },
        {
            
            point_tag {
                x: 9_f64,
                y: 9_f64,
            }
        },
    ];
    let len: u64 = (::core::mem::size_of::<[point_t; 10]>() as u64)
        .wrapping_div(::core::mem::size_of::<point_t>() as u64);
    let vla = len as usize;
    let mut out: Vec<point_t> = ::std::vec::from_elem(point_t { x: 0., y: 0. }, vla);
    let mut n: u64 = douglas_peucker(points.as_mut_ptr(), len, 1.0f64, out.as_mut_ptr(), len);
    print_points(out.as_mut_ptr(), n);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
