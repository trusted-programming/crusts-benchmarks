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
    fn abort() -> !;
}
#[no_mangle]
pub static mut c: [i64; 100] = [0; 100];
#[no_mangle]
pub extern "C" fn coef(mut n: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        if !(0_i32..=63_i32).contains(&n) {
            abort();
        }
    }
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        c[i as usize] = 1;
        while i < n {
            j = i;
            c[(j.wrapping_add(1)) as usize] = 1;
            while j > 0_i32 {
                c[j as usize] = c[(j.wrapping_sub(1i32)) as usize] - c[j as usize];
                j = j.wrapping_sub(1);
                j;
            }
            c[0_usize] = -c[0_usize];
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn is_prime(mut n: i32) -> i32 {
    let mut i: i32 = 0;
    coef(n);
// SAFETY: machine generated unsafe code
    unsafe {
        c[0_usize] += 1;
    }
    i = n;
// SAFETY: machine generated unsafe code
    unsafe {
        c[i as usize] -= 1;
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 != 0_i32 && c[i as usize] % i64::from(n) == 0) {
                break;
            }
        }
    }
    i32::from(i < 0_i32)
}

#[no_mangle]
pub extern "C" fn show(mut n: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        loop {
            print!("{:+}x^{}", c[n as usize], n);
            let fresh1 = n;
            n = n.wrapping_sub(1);
            if fresh1 == 0_i32 {
                break;
            }
        }
    }
}

fn main_0() -> i32 {
    let mut n: i32 = 0;
    n = 0_i32;
    while n < 10_i32 {
        coef(n);
        print!("(x-1)^{} = ", n);
        show(n);
        print!("{}", '\n' as i32);
        n = n.wrapping_add(1);
        n;
    }
    print!("\nprimes (never mind the 1):");
    n = 1_i32;
    while n <= 63_i32 {
        if is_prime(n) != 0_i32 {
            print!(" {}", n);
        }
        n = n.wrapping_add(1);
        n;
    }
    print!("{}", '\n' as i32);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
