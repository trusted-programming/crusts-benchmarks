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
pub extern "C" fn t(mut n: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut c: i32 = 0;
    let mut len: i32 = 0;
    i = n * (n.wrapping_sub(1)) / 2_i32;
    c = 1_i32;
    len = c;
    while c < i {
        c = c.wrapping_mul(10);
        len = len.wrapping_add(1);
        len;
    }
    c = c.wrapping_sub(i);
    let mut num: i32 = 0;
    i = 1_i32;
    num = i;
    while i <= n {
        j = 1_i32;
        while j <= i {
            let fresh0 = num;
            num = num.wrapping_add(1);
            if i - j != 0_i32 {
                print!(
                    "{1:0$}{2:}",
                    (len - i32::from(j < c)).unsigned_abs() as usize,
                    fresh0,
                    ' ' as i32
                )
            } else {
                print!(
                    "{1:0$}{2:}",
                    (len - i32::from(j < c)).unsigned_abs() as usize,
                    fresh0,
                    '\n' as i32
                )
            };
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}

fn main_0() -> i32 {
    t(5);
    t(14);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
