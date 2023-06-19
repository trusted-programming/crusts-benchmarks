#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn printf(_: *const i8, _: ...) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layer1 {
    pub a: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layer2 {
    pub l1: layer1,
    pub b: libc::c_float,
    pub c: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct layer3 {
    pub l2: layer2,
    pub l1: layer1,
    pub d: i32,
    pub e: i32,
}
#[no_mangle]
pub extern "C" fn showCake(mut cake: layer3) {
    unsafe {
        printf(b"\ncake.d = %d\0" as *const u8 as *const i8, cake.d);
        printf(b"\ncake.e = %d\0" as *const u8 as *const i8, cake.e);
        printf(b"\ncake.l1.a = %d\0" as *const u8 as *const i8, cake.l1.a);
        printf(
            b"\ncake.l2.b = %f\0" as *const u8 as *const i8,
            cake.l2.b as f64,
        );
        printf(
            b"\ncake.l2.l1.a = %d\0" as *const u8 as *const i8,
            cake.l2.l1.a,
        );
    }
}

fn main_0() -> i32 {
    let mut cake1: layer3 = layer3 {
        l2: layer2 {
            l1: layer1 { a: 0 },
            b: 0.,
            c: 0.,
        },
        l1: layer1 { a: 0 },
        d: 0,
        e: 0,
    };
    let mut cake2: layer3 = layer3 {
        l2: layer2 {
            l1: layer1 { a: 0 },
            b: 0.,
            c: 0.,
        },
        l1: layer1 { a: 0 },
        d: 0,
        e: 0,
    };
    cake1.d = 1;
    cake1.e = 2;
    cake1.l1.a = 3;
    cake1.l2.b = 4 as libc::c_float;
    cake1.l2.l1.a = 5;
    unsafe {
        printf(b"Cake 1 is : \0" as *const u8 as *const i8);
    }
    showCake(cake1);
    cake2 = cake1;
    cake2.l2.b += cake2.l2.l1.a as libc::c_float;
    unsafe {
        printf(b"\nCake 2 is : \0" as *const u8 as *const i8);
    }
    showCake(cake2);
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
