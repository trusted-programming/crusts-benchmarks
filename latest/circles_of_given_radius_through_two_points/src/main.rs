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
    fn pow(_: f64, _: f64) -> f64;
    fn sqrt(_: f64) -> f64;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct point {
    pub x: f64,
    pub y: f64,
}
#[no_mangle]
pub extern "C" fn distance(mut p1: point, mut p2: point) -> f64 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        sqrt((p1.x - p2.x).mul_add(p1.x - p2.x, (p1.y - p2.y) * (p1.y - p2.y)))
    }
}

#[no_mangle]
pub extern "C" fn findCircles(mut p1: point, mut p2: point, mut radius: f64) {
    let mut separation: f64 = distance(p1, p2);
    let mut mirrorDistance: f64 = 0.;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        if separation == 0.0f64 {
            if radius == 0.0f64 {
                print!(
                    "\nNo circles can be drawn through ({:.4},{:.4})",
                    p1.x, p1.y
                );
            } else {
                print!(
                    "\nInfinitely many circles can be drawn through ({:.4},{:.4})",
                    p1.x, p1.y
                );
            };
        } else if separation == 2_f64 * radius {
            print! ("\nGiven points are opposite ends of a diameter of the circle with center ({:.4},{:.4}) and radius {:.4}", (p1.x + p2.x) / 2_f64, (p1.y + p2.y) / 2_f64, radius);
        } else if separation > 2_f64 * radius {
            print! ("\nGiven points are farther away from each other than a diameter of a circle with radius {:.4}", radius);
        } else {
            mirrorDistance = sqrt(pow(radius, 2_f64) - pow(separation / 2_f64, 2_f64));
            print!("\nTwo circles are possible.");
            print! ("\nCircle C1 with center ({:.4},{:.4}), radius {:.4} and Circle C2 with center ({:.4},{:.4}), radius {:.4}", (p1.x + p2.x) / 2_f64 + mirrorDistance * (p1.y - p2.y) / separation, (p1.y + p2.y) / 2_f64 + mirrorDistance * (p2.x - p1
              .x) / separation, radius, (p1.x + p2.x) / 2_f64 - mirrorDistance * (p1.y - p2.y) / separation, (p1.y + p2.y) / 2_f64 - mirrorDistance * (p2.x - p1.x) / separation, radius);
        };
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut cases: [point; 10] = [
        {
            
            point {
                x: 0.1234f64,
                y: 0.9876f64,
            }
        },
        {
            
            point {
                x: 0.8765f64,
                y: 0.2345f64,
            }
        },
        {
            
            point {
                x: 0.0000f64,
                y: 2.0000f64,
            }
        },
        {
            
            point {
                x: 0.0000f64,
                y: 0.0000f64,
            }
        },
        {
            
            point {
                x: 0.1234f64,
                y: 0.9876f64,
            }
        },
        {
            
            point {
                x: 0.1234f64,
                y: 0.9876f64,
            }
        },
        {
            
            point {
                x: 0.1234f64,
                y: 0.9876f64,
            }
        },
        {
            
            point {
                x: 0.8765f64,
                y: 0.2345f64,
            }
        },
        {
            
            point {
                x: 0.1234f64,
                y: 0.9876f64,
            }
        },
        {
            
            point {
                x: 0.1234f64,
                y: 0.9876f64,
            }
        },
    ];
    let mut radii: [f64; 5] = [2.0f64, 1.0f64, 2.0f64, 0.5f64, 0.0f64];
    i = 0_i32;
    while i < 5_i32 {
        print!("\nCase {})", i + 1_i32);
        findCircles(
            cases[(2_i32 * i) as usize],
            cases[(2_i32 * i + 1i32) as usize],
            radii[i as usize],
        );
        i = i.wrapping_add(1);
        i;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
