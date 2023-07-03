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
pub extern "C" fn mean(mut v: *mut f64, mut len: i32) -> f64 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut sum: f64 = f64::from(0_i32);
        let mut i: i32 = 0;
        i = 0_i32;
        while i < len {
            sum += *v.offset(i as isize);
            i = i.wrapping_add(1);
            i;
        }
        sum / f64::from(len)
    }
}

fn main_0() -> i32 {
    let mut v: [f64; 5] = [1_f64, 2_f64, 2.718f64, 3_f64, 3.142f64];
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    len = 5_i32;
    while len >= 0_i32 {
        print!("mean[");
        i = 0_i32;
        while i < len {
            if i != 0_i32 {
                print!(", {}", v[i as usize])
            } else {
                print!("{}", v[i as usize])
            };
            i = i.wrapping_add(1);
            i;
        }
        println!("] = {}", mean(v.as_mut_ptr(), len));
        len = len.wrapping_sub(1);
        len;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
