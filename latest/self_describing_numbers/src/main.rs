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
pub extern "C" fn self_desc(mut xx: u64) -> i32 {
    let mut d: u32 = 0;
    let mut x: u32 = 0;
    let mut cnt: [u8; 10] = [0; 10];
    let mut dig: [u8; 10] = [0; 10];
    d = 0;
    while xx > !0 {
        let fresh0 = d;
        d = d.wrapping_add(1);
        dig[fresh0 as usize] = xx.wrapping_rem(10) as u8;
        cnt[dig[fresh0 as usize] as usize] = (cnt[dig[fresh0 as usize] as usize]).wrapping_add(1);
        cnt[dig[fresh0 as usize] as usize];
        xx = xx.wrapping_div(10);
    }
    x = xx as u32;
    while x != 0 {
        let fresh1 = d;
        d = d.wrapping_add(1);
        dig[fresh1 as usize] = x.wrapping_rem(10) as u8;
        cnt[dig[fresh1 as usize] as usize] = (cnt[dig[fresh1 as usize] as usize]).wrapping_add(1);
        cnt[dig[fresh1 as usize] as usize];
        x = x.wrapping_div(10);
    }
    loop {
        let fresh2 = d;
        d = d.wrapping_sub(1);
        if !(fresh2 != 0 && {
            let fresh3 = x;
            x = x.wrapping_add(1);
            i32::from(dig[fresh3 as usize]) == i32::from(cnt[d as usize])
        }) {
            break;
        }
    }
    i32::from(d == -1_i32 as u32)
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    i = 1_i32;
    while i < 100_000_000_i32 {
        if self_desc(i as u64) != 0_i32 {
            println!("{}", i);
        }
        i = i.wrapping_add(1);
        i;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
