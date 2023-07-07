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
#[no_mangle]
pub static mut a1: [i8; 3] = ['a' as i8, 'b' as i8, 'c' as i8];
#[no_mangle]
pub static mut a2: [i8; 3] = ['A' as i8, 'B' as i8, 'C' as i8];
#[no_mangle]
pub static mut a3: [i32; 3] = [1_i32, 2_i32, 3_i32];
fn main_0() -> i32 {
    let mut i: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 3_i32 {
            println!(
                "{}{}{}",
                i32::from(a1[i as usize]), i32::from(a2[i as usize]), a3[i as usize]
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
