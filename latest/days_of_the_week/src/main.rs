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
}
#[no_mangle]
pub extern "C" fn wday(mut year: i32, mut month: i32, mut day: i32) -> i32 {
    let mut adjustment: i32 = 0;
    let mut mm: i32 = 0;
    let mut yy: i32 = 0;
    adjustment = (14_i32 - month) / 12_i32;
    mm = month + 12_i32 * adjustment - 2_i32;
    yy = year - adjustment;
    (day + (13_i32 * mm - 1_i32) / 5_i32 + yy + yy / 4_i32 - yy / 100_i32 + yy / 400_i32) % 7_i32
}

fn main_0() -> i32 {
    let mut y: i32 = 0;
    y = 2_008_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while y <= 2_121_i32 {
            if wday(y, 12, 25) == 0_i32 {
                printf((b"%04d-12-25\n\0" as *const u8).cast::<i8>(), y);
            }
            y += 1_i32;
            y;
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
