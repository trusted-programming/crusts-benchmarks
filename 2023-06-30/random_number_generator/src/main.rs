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
pub extern "C" fn evolve(mut state: u64, mut rule: i32) {
    let mut i: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut b: i32 = 0;
    p = 0_i32;
    while p < 10_i32 {
        b = 0_i32;
        q = 8_i32;
        loop {
            let fresh0 = q;
            q = q.wrapping_sub(1);
            if fresh0 == 0_i32 {
                break;
            }
            let mut st: u64 = state;
            b = (b as u64 | (st & 1u64) << q) as i32;
            i = 0_i32;
            state = i as u64;
            while (i as u64) < (::core::mem::size_of::<u64>() as u64).wrapping_mul(8) {
                if rule as u64
                    & 1u64
                        << (7
                            & (st >> (i - 1_i32)
                                | st << (::core::mem::size_of::<u64>() as u64)
                                    .wrapping_mul(8)
                                    .wrapping_add(1)
                                    .wrapping_sub(i as u64)))
                    != 0
                {
                    state |= 1 << i;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        print!(" {}", b);
        p = p.wrapping_add(1);
        p;
    }
    print!("{}", '\n' as i32);
}

fn main_0() -> i32 {
    evolve(1, 30);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
