#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.add(str_size) != 0 {
            str_size = str_size.wrapping_add(1);
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    fn floor(_: f64) -> f64;
}
#[no_mangle]
pub static mut animals: [*const i8; 12] = [
    (b"Rat\0" as *const u8).cast::<i8>(),
    (b"Ox\0" as *const u8).cast::<i8>(),
    (b"Tiger\0" as *const u8).cast::<i8>(),
    (b"Rabbit\0" as *const u8).cast::<i8>(),
    (b"Dragon\0" as *const u8).cast::<i8>(),
    (b"Snake\0" as *const u8).cast::<i8>(),
    (b"Horse\0" as *const u8).cast::<i8>(),
    (b"Goat\0" as *const u8).cast::<i8>(),
    (b"Monkey\0" as *const u8).cast::<i8>(),
    (b"Rooster\0" as *const u8).cast::<i8>(),
    (b"Dog\0" as *const u8).cast::<i8>(),
    (b"Pig\0" as *const u8).cast::<i8>(),
];
#[no_mangle]
pub static mut elements: [*const i8; 5] = [
    (b"Wood\0" as *const u8).cast::<i8>(),
    (b"Fire\0" as *const u8).cast::<i8>(),
    (b"Earth\0" as *const u8).cast::<i8>(),
    (b"Metal\0" as *const u8).cast::<i8>(),
    (b"Water\0" as *const u8).cast::<i8>(),
];
#[no_mangle]
pub extern "C" fn getElement(mut year: i32) -> *const i8 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut element: i32 = floor(f64::from((year.wrapping_sub(4i32)) % 10_i32 / 2i32)) as i32;
        elements[element as usize]
    }
}

#[no_mangle]
pub extern "C" fn getAnimal(mut year: i32) -> *const i8 {
// SAFETY: machine generated unsafe code
    unsafe {
        animals[((year.wrapping_sub(4i32)) % 12i32) as usize]
    }
}

#[no_mangle]
pub extern "C" fn getYY(mut year: i32) -> *const i8 {
    if year % 2_i32 == 0_i32 {
        (b"yang\0" as *const u8).cast::<i8>()
    } else {
        (b"yin\0" as *const u8).cast::<i8>()
    }
}

fn main_0() -> i32 {
    let mut years: [i32; 6] = [1935, 1938, 1968, 1972, 1976, 2017];
    let mut i: i32 = 0;
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 6_i32 {
            let mut year: i32 = years[i as usize];
            println!(
                "{} is the year of the {} {} ({}).",
                year,
                build_str_from_raw_ptr(getElement(year) as *mut u8),
                build_str_from_raw_ptr(getAnimal(year) as *mut u8),
                build_str_from_raw_ptr(getYY(year) as *mut u8)
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
