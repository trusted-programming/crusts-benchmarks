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
    print!("\ncake.d = {}", cake.d);
    print!("\ncake.e = {}", cake.e);
    print!("\ncake.l1.a = {}", cake.l1.a);
    print!("\ncake.l2.b = {}", f64::from(cake.l2.b));
    print!("\ncake.l2.l1.a = {}", cake.l2.l1.a);
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
    cake1.d = 1_i32;
    cake1.e = 2_i32;
    cake1.l1.a = 3_i32;
    cake1.l2.b = 4_i32 as libc::c_float;
    cake1.l2.l1.a = 5_i32;
    print!("Cake 1 is : ");
    showCake(cake1);
    cake2 = cake1;
    cake2.l2.b += cake2.l2.l1.a as libc::c_float;
    print!("\nCake 2 is : ");
    showCake(cake2);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
