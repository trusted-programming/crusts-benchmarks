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
    fn exp(_: f64) -> f64;
}
// SAFETY: machine generated unsafe code
pub type deriv_f = Option<unsafe extern "C" fn(f64, f64) -> f64>;
#[no_mangle]
pub extern "C" fn ivp_euler(mut f: deriv_f, mut y: f64, mut step: i32, mut end_t: i32) {
    let mut t: i32 = 0;
    print!(" Step {:2}: ", step);
// SAFETY: machine generated unsafe code
    unsafe {
        loop {
            if t % 10_i32 == 0_i32 {
                print!(" {:7.3}", y);
            }
            y += f64::from(step) * f.expect("non-null function pointer")(f64::from(t), y);
            t += step;
            if t > end_t {
                break;
            }
        }
    }
    println!();
}

#[no_mangle]
pub extern "C" fn analytic() {
    let mut t: f64 = 0.;
    print!("    Time: ");
    t = f64::from(0_i32);
    while t <= 100_f64 {
        print!(" {:7}", t);
        t += 10_f64;
    }
    print!("\nAnalytic: ");
    t = f64::from(0_i32);
// SAFETY: machine generated unsafe code
    unsafe {
        while t <= 100_f64 {
            print!(" {:7.3}", 80_f64.mul_add(exp(-0.07f64 * t), 20_f64));
            t += 10_f64;
        }
    }
    println!();
}

#[no_mangle]
pub extern "C" fn cooling(mut _t: f64, mut temp: f64) -> f64 {
    -0.07f64 * (temp - 20_f64)
}

fn main_0() -> i32 {
    analytic();
    ivp_euler(
// SAFETY: machine generated unsafe code
        Some(cooling as unsafe extern "C" fn(f64, f64) -> f64),
        100_f64,
        2,
        100,
    );
    ivp_euler(
// SAFETY: machine generated unsafe code
        Some(cooling as unsafe extern "C" fn(f64, f64) -> f64),
        100_f64,
        5,
        100,
    );
    ivp_euler(
// SAFETY: machine generated unsafe code
        Some(cooling as unsafe extern "C" fn(f64, f64) -> f64),
        100_f64,
        10,
        100,
    );
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
