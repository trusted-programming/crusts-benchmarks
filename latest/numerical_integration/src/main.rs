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
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    fn log(_: f64) -> f64;
}
pub type pfunc =
// SAFETY: machine generated unsafe code
    Option<unsafe extern "C" fn(f64, f64, f64, Option<unsafe extern "C" fn() -> f64>) -> f64>;
// SAFETY: machine generated unsafe code
pub type rfunc = Option<unsafe extern "C" fn(f64) -> f64>;
#[no_mangle]
pub extern "C" fn int_leftrect(
    mut from: f64,
    mut to: f64,
    mut n: f64,
// SAFETY: machine generated unsafe code
    mut func: Option<unsafe extern "C" fn() -> f64>,
) -> f64 {
    let mut h: f64 = (to - from) / n;
    let mut sum: f64 = 0.0f64;
    let mut x: f64 = 0.;
    x = from;
// SAFETY: machine generated unsafe code
    unsafe {
        while x <= to - h {
            sum += ::core::mem::transmute::<_, fn(_) -> f64>(
                func.expect("non-null function pointer"),
            )(x);
            x += h;
        }
    }
    h * sum
}

#[no_mangle]
pub extern "C" fn int_rightrect(
    mut from: f64,
    mut to: f64,
    mut n: f64,
// SAFETY: machine generated unsafe code
    mut func: Option<unsafe extern "C" fn() -> f64>,
) -> f64 {
    let mut h: f64 = (to - from) / n;
    let mut sum: f64 = 0.0f64;
    let mut x: f64 = 0.;
    x = from;
// SAFETY: machine generated unsafe code
    unsafe {
        while x <= to - h {
            sum += ::core::mem::transmute::<_, fn(_) -> f64>(
                func.expect("non-null function pointer"),
            )(x + h);
            x += h;
        }
    }
    h * sum
}

#[no_mangle]
pub extern "C" fn int_midrect(
    mut from: f64,
    mut to: f64,
    mut n: f64,
// SAFETY: machine generated unsafe code
    mut func: Option<unsafe extern "C" fn() -> f64>,
) -> f64 {
    let mut h: f64 = (to - from) / n;
    let mut sum: f64 = 0.0f64;
    let mut x: f64 = 0.;
    x = from;
// SAFETY: machine generated unsafe code
    unsafe {
        while x <= to - h {
            sum += ::core::mem::transmute::<_, fn(_) -> f64>(
                func.expect("non-null function pointer"),
            )(x + h / 2.0f64);
            x += h;
        }
    }
    h * sum
}

#[no_mangle]
pub extern "C" fn int_trapezium(
    mut from: f64,
    mut to: f64,
    mut n: f64,
// SAFETY: machine generated unsafe code
    mut func: Option<unsafe extern "C" fn() -> f64>,
) -> f64 {
    let mut h: f64 = (to - from) / n;
// SAFETY: machine generated unsafe code
    unsafe {
        let mut sum: f64 = ::core::mem::transmute::<_, fn(_) -> f64>(
            func.expect("non-null function pointer"),
        )(from)
            + ::core::mem::transmute::<_, fn(_) -> f64>(func.expect("non-null function pointer"))(
                to,
            );
        let mut i: i32 = 0;
        i = 1_i32;
        while f64::from(i) < n {
            sum += 2.0f64
                * ::core::mem::transmute::<_, fn(_) -> f64>(
                    func.expect("non-null function pointer"),
                )(f64::from(i).mul_add(h, from));
            i += 1_i32;
            i;
        }
        h * sum / 2.0f64
    }
}

#[no_mangle]
pub extern "C" fn int_simpson(
    mut from: f64,
    mut to: f64,
    mut n: f64,
// SAFETY: machine generated unsafe code
    mut func: Option<unsafe extern "C" fn() -> f64>,
) -> f64 {
    let mut h: f64 = (to - from) / n;
    let mut sum1: f64 = 0.0f64;
    let mut sum2: f64 = 0.0f64;
    let mut i: i32 = 0;
    let mut _x: f64 = 0.;
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while f64::from(i) < n {
            sum1 += ::core::mem::transmute::<_, fn(_) -> f64>(
                func.expect("non-null function pointer"),
            )(h.mul_add(f64::from(i), from) + h / 2.0f64);
            i += 1_i32;
            i;
        }
    }
    i = 1_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while f64::from(i) < n {
            sum2 += ::core::mem::transmute::<_, fn(_) -> f64>(
                func.expect("non-null function pointer"),
            )(h.mul_add(f64::from(i), from));
            i += 1_i32;
            i;
        }
        h / 6.0f64
            * 2.0f64.mul_add(sum2, 4.0f64.mul_add(sum1, ::core::mem::transmute::<_, fn(_) -> f64>(
                func.expect("non-null function pointer"),
            )(from) + ::core::mem::transmute::<_, fn(_) -> f64>(
                    func.expect("non-null function pointer"),
                )(to)))
    }
}

#[no_mangle]
pub extern "C" fn f3(mut x: f64) -> f64 {
    x
}

#[no_mangle]
pub extern "C" fn f3a(mut x: f64) -> f64 {
    x * x / 2.0f64
}

#[no_mangle]
pub extern "C" fn f2(mut x: f64) -> f64 {
    1.0f64 / x
}

#[no_mangle]
pub extern "C" fn f2a(mut x: f64) -> f64 {
// SAFETY: machine generated unsafe code
    unsafe {
        log(x)
    }
}

#[no_mangle]
pub extern "C" fn f1(mut x: f64) -> f64 {
    x * x * x
}

#[no_mangle]
pub extern "C" fn f1a(mut x: f64) -> f64 {
    x * x * x * x / 4.0f64
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ic: f64 = 0.;
// SAFETY: machine generated unsafe code
    unsafe {
        let mut f: [pfunc; 5] = [
            Some(
                int_leftrect
// SAFETY: machine generated unsafe code
                    as unsafe extern "C" fn(
                        f64,
                        f64,
                        f64,
// SAFETY: machine generated unsafe code
                        Option<unsafe extern "C" fn() -> f64>,
                    ) -> f64,
            ),
            Some(
                int_rightrect
// SAFETY: machine generated unsafe code
                    as unsafe extern "C" fn(
                        f64,
                        f64,
                        f64,
// SAFETY: machine generated unsafe code
                        Option<unsafe extern "C" fn() -> f64>,
                    ) -> f64,
            ),
            Some(
                int_midrect
// SAFETY: machine generated unsafe code
                    as unsafe extern "C" fn(
                        f64,
                        f64,
                        f64,
// SAFETY: machine generated unsafe code
                        Option<unsafe extern "C" fn() -> f64>,
                    ) -> f64,
            ),
            Some(
                int_trapezium
// SAFETY: machine generated unsafe code
                    as unsafe extern "C" fn(
                        f64,
                        f64,
                        f64,
// SAFETY: machine generated unsafe code
                        Option<unsafe extern "C" fn() -> f64>,
                    ) -> f64,
            ),
            Some(
                int_simpson
// SAFETY: machine generated unsafe code
                    as unsafe extern "C" fn(
                        f64,
                        f64,
                        f64,
// SAFETY: machine generated unsafe code
                        Option<unsafe extern "C" fn() -> f64>,
                    ) -> f64,
            ),
        ];
        let mut names: [*const i8; 5] = [
            (b"leftrect\0" as *const u8).cast::<i8>(),
            (b"rightrect\0" as *const u8).cast::<i8>(),
            (b"midrect\0" as *const u8).cast::<i8>(),
            (b"trapezium\0" as *const u8).cast::<i8>(),
            (b"simpson\0" as *const u8).cast::<i8>(),
        ];
        let mut rf: [rfunc; 4] = [
// SAFETY: machine generated unsafe code
            Some(f1 as unsafe extern "C" fn(f64) -> f64),
// SAFETY: machine generated unsafe code
            Some(f2 as unsafe extern "C" fn(f64) -> f64),
// SAFETY: machine generated unsafe code
            Some(f3 as unsafe extern "C" fn(f64) -> f64),
// SAFETY: machine generated unsafe code
            Some(f3 as unsafe extern "C" fn(f64) -> f64),
        ];
        let mut If: [rfunc; 4] = [
// SAFETY: machine generated unsafe code
            Some(f1a as unsafe extern "C" fn(f64) -> f64),
// SAFETY: machine generated unsafe code
            Some(f2a as unsafe extern "C" fn(f64) -> f64),
// SAFETY: machine generated unsafe code
            Some(f3a as unsafe extern "C" fn(f64) -> f64),
// SAFETY: machine generated unsafe code
            Some(f3a as unsafe extern "C" fn(f64) -> f64),
        ];
        let mut ivals: [f64; 8] = [
            0.0f64, 1.0f64, 1.0f64, 100.0f64, 0.0f64, 5000.0f64, 0.0f64, 6000.0f64,
        ];
        let mut approx: [f64; 4] = [100.0f64, 1000.0f64, 5000000.0f64, 6000000.0f64];
        j = 0_i32;
        while (j as u64)
            < (::core::mem::size_of::<[rfunc; 4]>() as u64)
                .wrapping_div(::core::mem::size_of::<rfunc>() as u64)
        {
            i = 0_i32;
            while i < 5_i32 {
                ic = (*f.as_mut_ptr().offset(i as isize)).expect("non-null function pointer")(
                    ivals[(2 * j) as usize],
                    ivals[(2 * j + 1i32) as usize],
                    approx[j as usize],
// SAFETY: machine generated unsafe code
                    ::core::mem::transmute::<rfunc, Option<unsafe extern "C" fn() -> f64>>(
                        rf[j as usize],
                    ),
                );
                println!(
                    "{:10} [ 0,1] num: {:+}, an: {}",
                    build_str_from_raw_ptr(names[i as usize] as *mut u8),
                    ic,
                    (*If.as_mut_ptr().offset(j as isize)).expect("non-null function pointer")(
                        ivals[(2 * j + 1i32) as usize]
                    ) - (*If.as_mut_ptr().offset(j as isize)).expect("non-null function pointer")(
                        ivals[(2 * j) as usize]
                    )
                );
                i += 1_i32;
                i;
            }
            println!();
            j += 1_i32;
            j;
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
