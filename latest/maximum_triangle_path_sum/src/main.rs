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
    fn sqrt(_: f64) -> f64;
}
#[no_mangle]
pub static mut tri: [i32; 171] = [
    55_i32, 94_i32, 48_i32, 95_i32, 30_i32, 96_i32, 77_i32, 71_i32, 26_i32, 67_i32, 97_i32, 13_i32, 76_i32, 38_i32, 45_i32, 7_i32, 36_i32, 79_i32, 16_i32, 37_i32, 68_i32, 48_i32, 7_i32, 9_i32,
    18_i32, 70_i32, 26_i32, 6_i32, 18_i32, 72_i32, 79_i32, 46_i32, 59_i32, 79_i32, 29_i32, 90_i32, 20_i32, 76_i32, 87_i32, 11_i32, 32_i32, 7_i32, 7_i32, 49_i32, 18_i32, 27_i32, 83_i32, 58_i32,
    35_i32, 71_i32, 11_i32, 25_i32, 57_i32, 29_i32, 85_i32, 14_i32, 64_i32, 36_i32, 96_i32, 27_i32, 11_i32, 58_i32, 56_i32, 92_i32, 18_i32, 55_i32, 2_i32, 90_i32, 3_i32, 60_i32, 48_i32, 49_i32,
    41_i32, 46_i32, 33_i32, 36_i32, 47_i32, 23_i32, 92_i32, 50_i32, 48_i32, 2_i32, 36_i32, 59_i32, 42_i32, 79_i32, 72_i32, 20_i32, 82_i32, 77_i32, 42_i32, 56_i32, 78_i32, 38_i32, 80_i32, 39_i32,
    75_i32, 2_i32, 71_i32, 66_i32, 66_i32, 1_i32, 3_i32, 55_i32, 72_i32, 44_i32, 25_i32, 67_i32, 84_i32, 71_i32, 67_i32, 11_i32, 61_i32, 40_i32, 57_i32, 58_i32, 89_i32, 40_i32, 56_i32, 36_i32,
    85_i32, 32_i32, 25_i32, 85_i32, 57_i32, 48_i32, 84_i32, 35_i32, 47_i32, 62_i32, 17_i32, 1_i32, 1_i32, 99_i32, 89_i32, 52_i32, 6_i32, 71_i32, 28_i32, 75_i32, 94_i32, 48_i32, 37_i32, 10_i32,
    23_i32, 51_i32, 6_i32, 48_i32, 53_i32, 18_i32, 74_i32, 98_i32, 15_i32, 27_i32, 2_i32, 92_i32, 23_i32, 8_i32, 71_i32, 76_i32, 84_i32, 15_i32, 52_i32, 92_i32, 63_i32, 81_i32, 10_i32, 44_i32,
    10_i32, 69_i32, 93_i32,
];
fn main_0() -> i32 {
    let len: i32 = (::core::mem::size_of::<[i32; 171]>() as u64)
        .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32;
// SAFETY: machine generated unsafe code
    unsafe {
        let base: i32 = ((sqrt(f64::from(8_i32 * len.wrapping_add(1i32))) - 1_f64) / 2_f64) as i32;
        let mut step: i32 = base.wrapping_sub(1);
        let mut stepc: i32 = 0;
        let mut i: i32 = 0;
        i = len - base.wrapping_sub(1);
        while i >= 0_i32 {
            tri[i as usize] += if tri[(i.wrapping_add(step)) as usize]
                > tri[(i + step.wrapping_add(1i32)) as usize]
            {
                tri[(i.wrapping_add(step)) as usize]
            } else {
                tri[(i + step.wrapping_add(1i32)) as usize]
            };
            stepc = stepc.wrapping_add(1);
            if stepc == step {
                step = step.wrapping_sub(1);
                step;
                stepc = 0_i32;
            }
            i = i.wrapping_sub(1);
            i;
        }
        println!("{}", tri[0_usize]);
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
