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
    fn scanf(_: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub extern "C" fn leonardo(mut a: i32, mut b: i32, mut step: i32, mut num: i32) {
    let mut i: i32 = 0;
    let mut temp: i32 = 0;
    println!("First 25 Leonardo numbers : ");
    i = 1_i32;
    while i <= num {
        if i == 1_i32 {
            print!(" {}", a);
        } else if i == 2_i32 {
            print!(" {}", b);
        } else {
            print!(" {}", a + b + step);
            temp = a;
            a = b;
            b = temp + b.wrapping_add(step);
        }
        i = i.wrapping_add(1);
        i;
    }
}

fn main_0() -> i32 {
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut step: i32 = 0;
    print!("Enter first two Leonardo numbers and increment step : ");
// SAFETY: machine generated unsafe code
    unsafe {
        scanf(
            (b"%d%d%d\0" as *const u8).cast::<i8>(),
            &mut a as *mut i32,
            &mut b as *mut i32,
            &mut step as *mut i32,
        );
    }
    leonardo(a, b, step, 25);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
