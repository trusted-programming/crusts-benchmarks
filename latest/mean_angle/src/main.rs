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
    fn atan2(_: f64, _: f64) -> f64;
    fn cos(_: f64) -> f64;
    fn sin(_: f64) -> f64;
}
#[no_mangle]
pub extern "C" fn meanAngle(mut angles: *mut f64, mut size: i32) -> f64 {
    unsafe {
        let mut y_part: f64 = f64::from(0_i32);
        let mut x_part: f64 = f64::from(0_i32);
        let mut i: i32 = 0;
        i = 0_i32;
        while i < size {
            x_part += cos((*angles.offset(i as isize)).to_radians());
            y_part += sin((*angles.offset(i as isize)).to_radians());
            i = i.wrapping_add(1);
            i;
        }
        atan2(y_part / f64::from(size), x_part / f64::from(size)).to_degrees()
    }
}

fn main_0() -> i32 {
    let mut angleSet1: [f64; 2] = [350_f64, 10_f64];
    let mut angleSet2: [f64; 4] = [90_f64, 180_f64, 270_f64, 360_f64];
    let mut angleSet3: [f64; 3] = [10_f64, 20_f64, 30_f64];
    print!(
        "\nMean Angle for 1st set : {} degrees",
        meanAngle(angleSet1.as_mut_ptr(), 2)
    );
    print!(
        "\nMean Angle for 2nd set : {} degrees",
        meanAngle(angleSet2.as_mut_ptr(), 4)
    );
    print!(
        "\nMean Angle for 3rd set : {} degrees\n",
        meanAngle(angleSet3.as_mut_ptr(), 3)
    );
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
