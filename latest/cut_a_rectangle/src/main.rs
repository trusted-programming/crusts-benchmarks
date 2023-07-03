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
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub static mut grid: *mut u8 = 0 as *const u8 as *mut u8;
#[no_mangle]
pub static mut w: i32 = 0_i32;
#[no_mangle]
pub static mut h: i32 = 0_i32;
#[no_mangle]
pub static mut len: i32 = 0_i32;
#[no_mangle]
pub static mut cnt: u64 = 0;
static mut next: [i32; 4] = [0_i32; 4];
static mut dir: [[i32; 2]; 4] = [[0_i32, -1_i32], [-1_i32, 0_i32], [0_i32, 1_i32], [1_i32, 0_i32]];
#[no_mangle]
pub extern "C" fn walk(mut y: i32, mut x: i32) {
    let mut i: i32 = 0;
    let mut t: i32 = 0;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        if y == 0_i32 || y == h || x == 0_i32 || x == w {
            cnt = cnt.wrapping_add(2);
            return;
        }
        t = y * (w + 1_i32) + x;
        let fresh0 = &mut (*grid.offset(t as isize));
        *fresh0 = (*fresh0).wrapping_add(1);
        *fresh0;
        let fresh1 = &mut (*grid.offset((len - t) as isize));
        *fresh1 = (*fresh1).wrapping_add(1);
        *fresh1;
    }
    i = 0_i32;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 4_i32 {
            if *grid.offset((t + next[i as usize]) as isize) == 0 {
                walk(
                    y + dir[i as usize][0_usize],
                    x + dir[i as usize][1_usize],
                );
            }
            i = i.wrapping_add(1);
            i;
        }
        let fresh2 = &mut (*grid.offset(t as isize));
        *fresh2 = (*fresh2).wrapping_sub(1);
        *fresh2;
        let fresh3 = &mut (*grid.offset((len - t) as isize));
        *fresh3 = (*fresh3).wrapping_sub(1);
        *fresh3;
    }
}

#[no_mangle]
pub extern "C" fn solve(mut hh: i32, mut ww: i32, mut recur: i32) -> u64 {
    let mut t: i32 = 0;
    let mut cx: i32 = 0;
    let mut cy: i32 = 0;
    let mut x: i32 = 0;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        h = hh;
        w = ww;
        if h & 1_i32 != 0_i32 {
            t = w;
            w = h;
            h = t;
        }
        if h & 1_i32 != 0_i32 {
            return 0;
        }
        if w == 1_i32 {
            return 1;
        }
        if w == 2_i32 {
            return h as u64;
        }
        if h == 2_i32 {
            return w as u64;
        }
        cy = h / 2_i32;
        cx = w / 2_i32;
        len = (h + 1_i32) * (w + 1_i32);
        grid = realloc(grid.cast::<libc::c_void>(), len as u64).cast::<u8>();
        let fresh4 = len;
        len = len.wrapping_sub(1);
        memset(grid.cast::<libc::c_void>(), 0, fresh4 as u64);
        next[0_usize] = -1_i32;
        next[1_usize] = -w - 1_i32;
        next[2_usize] = 1_i32;
        next[3_usize] = w + 1_i32;
        if recur != 0_i32 {
            cnt = 0;
        }
    }
    x = cx.wrapping_add(1);
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        while x < w {
            t = cy * (w + 1_i32) + x;
            *grid.offset(t as isize) = 1;
            *grid.offset((len - t) as isize) = 1;
            walk(cy - 1, x);
            x = x.wrapping_add(1);
            x;
        }
        cnt = cnt.wrapping_add(1);
        cnt;
        if h == w {
            cnt = cnt.wrapping_mul(2);
        } else if w & 1_i32 == 0_i32 && recur != 0_i32 {
            solve(w, h, 0);
        }
        cnt
    }
}

fn main_0() -> i32 {
    let mut y: i32 = 0;
    let mut x: i32 = 0;
    y = 1_i32;
    while y <= 10_i32 {
        x = 1_i32;
        while x <= y {
            if x & 1_i32 == 0_i32 || y & 1_i32 == 0_i32 {
                println!("{} x {}: {}", y, x, solve(y, x, 1));
            }
            x = x.wrapping_add(1);
            x;
        }
        y = y.wrapping_add(1);
        y;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
