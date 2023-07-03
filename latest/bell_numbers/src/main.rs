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
    fn printf(_: *const i8, _: ...) -> i32;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub extern "C" fn bellIndex(mut row: i32, mut col: i32) -> u64 {
    (row * (row - 1i32) / 2_i32 + col) as u64
}

#[no_mangle]
pub extern "C" fn getBell(mut bellTri: *mut i32, mut row: i32, mut col: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut index: u64 = bellIndex(row, col);
        *bellTri.offset(index as isize)
    }
}

#[no_mangle]
pub extern "C" fn setBell(mut bellTri: *mut i32, mut row: i32, mut col: i32, mut value: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut index: u64 = bellIndex(row, col);
        *bellTri.offset(index as isize) = value;
    }
}

#[no_mangle]
pub extern "C" fn bellTriangle(mut n: i32) -> *mut i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut length: u64 = (n * (n + 1i32) / 2) as u64;
        let mut tri: *mut i32 = calloc(length, ::core::mem::size_of::<i32>() as u64).cast::<i32>();
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        setBell(tri, 1, 0, 1);
        i = 2_i32;
        while i <= n {
            setBell(tri, i, 0, getBell(tri, i - 1, i - 2));
            j = 1_i32;
            while j < i {
                let mut value: i32 = getBell(tri, i, j - 1) + getBell(tri, i - 1, j - 1);
                setBell(tri, i, j, value);
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        tri
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let rows: i32 = 15;
        let mut bt: *mut i32 = bellTriangle(rows);
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        println!("First fifteen Bell numbers:");
        i = 1_i32;
        while i <= rows {
            println!("{:2}: {}", i, getBell(bt, i, 0));
            i = i.wrapping_add(1);
            i;
        }
        printf((b"\nThe first ten rows of Bell's triangle:\n\0" as *const u8).cast::<i8>());
        i = 1_i32;
        while i <= 10_i32 {
            print!("{}", getBell(bt, i, 0));
            j = 1_i32;
            while j < i {
                print!(", {}", getBell(bt, i, j));
                j = j.wrapping_add(1);
                j;
            }
            println!();
            i = i.wrapping_add(1);
            i;
        }
        free(bt.cast::<libc::c_void>());
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
