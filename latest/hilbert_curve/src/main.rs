#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct point {
    pub x: i32,
    pub y: i32,
}
#[no_mangle]
pub extern "C" fn rot(mut n: i32, mut p: *mut point, mut rx: i32, mut ry: i32) {
    unsafe {
        let mut t: i32 = 0;
        if ry == 0_i32 {
            if rx == 1_i32 {
                (*p).x = n - 1_i32 - (*p).x;
                (*p).y = n - 1_i32 - (*p).y;
            }
            t = (*p).x;
            (*p).x = (*p).y;
            (*p).y = t;
        }
    }
}

#[no_mangle]
pub extern "C" fn d2pt(mut n: i32, mut d: i32, mut p: *mut point) {
    unsafe {
        let mut s: i32 = 1;
        let mut t: i32 = d;
        let mut rx: i32 = 0;
        let mut ry: i32 = 0;
        (*p).x = 0_i32;
        (*p).y = 0_i32;
        while s < n {
            rx = 1_i32 & (t / 2_i32);
            ry = 1_i32 & (t ^ rx);
            rot(s, p, rx, ry);
            (*p).x += s * rx;
            (*p).y += s * ry;
            t = t.wrapping_div(4);
            s *= 2_i32;
        }
    }
}

fn main_0() -> i32 {
    let mut d: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut cx: i32 = 0;
    let mut cy: i32 = 0;
    let mut px: i32 = 0;
    let mut py: i32 = 0;
    let mut pts: [[i8; 96]; 96] = [[0; 96]; 96];
    let mut curr: point = point { x: 0, y: 0 };
    let mut prev: point = point { x: 0, y: 0 };
    x = 0_i32;
    while x < 32_i32 * 3_i32 {
        y = 0_i32;
        while y < 32_i32 * 3_i32 {
            pts[x as usize][y as usize] = ' ' as i8;
            y = y.wrapping_add(1);
            y;
        }
        x = x.wrapping_add(1);
        x;
    }
    prev.y = 0_i32;
    prev.x = prev.y;
    pts[0_usize][0_usize] = '.' as i8;
    d = 1_i32;
    while d < 32_i32 * 32_i32 {
        d2pt(32, d, &mut curr);
        cx = curr.x * 3_i32;
        cy = curr.y * 3_i32;
        px = prev.x * 3_i32;
        py = prev.y * 3_i32;
        pts[cx as usize][cy as usize] = '.' as i8;
        if cx == px {
            if py < cy {
                y = py.wrapping_add(1);
                while y < cy {
                    pts[cx as usize][y as usize] = '|' as i8;
                    y = y.wrapping_add(1);
                    y;
                }
            } else {
                y = cy.wrapping_add(1);
                while y < py {
                    pts[cx as usize][y as usize] = '|' as i8;
                    y = y.wrapping_add(1);
                    y;
                }
            }
        } else if px < cx {
            x = px.wrapping_add(1);
            while x < cx {
                pts[x as usize][cy as usize] = '_' as i8;
                x = x.wrapping_add(1);
                x;
            }
        } else {
            x = cx.wrapping_add(1);
            while x < px {
                pts[x as usize][cy as usize] = '_' as i8;
                x = x.wrapping_add(1);
                x;
            }
        }
        prev = curr;
        d = d.wrapping_add(1);
        d;
    }
    x = 0_i32;
    while x < 32_i32 * 3_i32 {
        y = 0_i32;
        while y < 32_i32 * 3_i32 {
            print!("{}", i32::from(pts[y as usize][x as usize]));
            y = y.wrapping_add(1);
            y;
        }
        println!();
        x = x.wrapping_add(1);
        x;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
