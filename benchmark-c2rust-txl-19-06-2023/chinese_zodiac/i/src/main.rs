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
    fn floor(_: f64) -> f64;
    fn printf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub static mut animals: [*const i8; 12] = [
    b"Rat\0" as *const u8 as *const i8,
    b"Ox\0" as *const u8 as *const i8,
    b"Tiger\0" as *const u8 as *const i8,
    b"Rabbit\0" as *const u8 as *const i8,
    b"Dragon\0" as *const u8 as *const i8,
    b"Snake\0" as *const u8 as *const i8,
    b"Horse\0" as *const u8 as *const i8,
    b"Goat\0" as *const u8 as *const i8,
    b"Monkey\0" as *const u8 as *const i8,
    b"Rooster\0" as *const u8 as *const i8,
    b"Dog\0" as *const u8 as *const i8,
    b"Pig\0" as *const u8 as *const i8,
];
#[no_mangle]
pub static mut elements: [*const i8; 5] = [
    b"Wood\0" as *const u8 as *const i8,
    b"Fire\0" as *const u8 as *const i8,
    b"Earth\0" as *const u8 as *const i8,
    b"Metal\0" as *const u8 as *const i8,
    b"Water\0" as *const u8 as *const i8,
];
#[no_mangle]
pub extern "C" fn getElement(mut year: i32) -> *const i8 {
    unsafe {
        let mut element: i32 = floor(((year - 4i32) % 10 / 2i32) as f64) as i32;
        return elements[element as usize];
    }
}

#[no_mangle]
pub extern "C" fn getAnimal(mut year: i32) -> *const i8 {
    unsafe {
        return animals[((year - 4i32) % 12i32) as usize];
    }
}

#[no_mangle]
pub extern "C" fn getYY(mut year: i32) -> *const i8 {
    if year % 2 == 0 {
        return b"yang\0" as *const u8 as *const i8;
    } else {
        return b"yin\0" as *const u8 as *const i8;
    };
}

fn main_0() -> i32 {
    let mut years: [i32; 6] = [1935, 1938, 1968, 1972, 1976, 2017];
    let mut i: i32 = 0;
    i = 0;
    unsafe {
        while i < 6 {
            let mut year: i32 = years[i as usize];
            printf(
                b"%d is the year of the %s %s (%s).\n\0" as *const u8 as *const i8,
                year,
                getElement(year),
                getAnimal(year),
                getYY(year),
            );
            i += 1;
            i;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
