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
    fn puts(__s: *const i8) -> i32;
}
#[no_mangle]
pub extern "C" fn binomial(mut m: u64, mut n: u64) -> u64 {
    let mut r: u64 = 1;
    let mut d: u64 = m.wrapping_sub(n);
    if d > n {
        n = d;
        d = m.wrapping_sub(n);
    }
    while m > n {
        let fresh0 = m;
        m = m.wrapping_sub(1);
        r = (r).wrapping_mul(fresh0);
        while d > 1 && r.wrapping_rem(d) == 0 {
            let fresh1 = d;
            d = d.wrapping_sub(1);
            r = (r).wrapping_div(fresh1);
        }
    }
    r
}

#[no_mangle]
pub extern "C" fn catalan1(mut n: i32) -> u64 {
    (binomial((2 * n) as u64, n as u64)).wrapping_div((n.wrapping_add(1)) as u64)
}

#[no_mangle]
pub extern "C" fn catalan2(mut n: i32) -> u64 {
    let mut i: i32 = 0;
    let mut r: u64 = u64::from(n == 0_i32);
    i = 0_i32;
    while i < n {
        r = (r).wrapping_add((catalan2(i)).wrapping_mul(catalan2(n - 1 - i)));
        i = i.wrapping_add(1);
        i;
    }
    r
}

#[no_mangle]
pub extern "C" fn catalan3(mut n: i32) -> u64 {
    if n != 0_i32 {
        ((2 * (2 * n.wrapping_sub(1i32))) as u64)
            .wrapping_mul(catalan3(n.wrapping_sub(1)))
            .wrapping_div((n.wrapping_add(1)) as u64)
    } else {
        1
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        puts((b"\tdirect\tsumming\tfrac\0" as *const u8).cast::<i8>());
    }
    i = 0_i32;
    while i < 16_i32 {
        println!("{}	{}	{}	{}", i, catalan1(i), catalan2(i), catalan3(i));
        i = i.wrapping_add(1);
        i;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
