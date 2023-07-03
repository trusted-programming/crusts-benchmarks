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
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
}
#[no_mangle]
pub extern "C" fn digit_sum(mut n: u64, mut sum: u64) -> u64 {
    sum = sum.wrapping_add(1);
    sum;
    while n > 0 && n.wrapping_rem(10) == 0 {
        sum = sum.wrapping_sub(9);
        n = n.wrapping_div(10);
    }
    sum
}

#[no_mangle]
pub extern "C" fn divisible(mut n: u64, mut d: u64) -> bool {
    if d & 1 == 0 && n & 1 == 1 {
        return 0_i32 != 0_i32;
    }
    n.wrapping_rem(d) == 0
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        setlocale(6, (b"\0" as *const u8).cast::<i8>());
    }
    let mut previous: u64 = 1;
    let mut gap: u64 = 0;
    let mut sum: u64 = 0;
    let mut niven_index: i32 = 0;
    let mut gap_index: i32 = 1;
    println!("Gap index  Gap    Niven index    Niven number");
    let mut niven: u64 = 1;
    while gap_index <= 32_i32 {
        sum = digit_sum(niven, sum);
        if divisible(niven, sum) {
            if niven > previous.wrapping_add(gap) {
                gap = niven.wrapping_sub(previous);
                let fresh0 = gap_index;
                gap_index = gap_index.wrapping_add(1);
                println!(
                    "{:9} {:4} {:14} {:15}",
                    fresh0, gap, niven_index, previous
                );
            }
            previous = niven;
            niven_index = niven_index.wrapping_add(1);
            niven_index;
        }
        niven = niven.wrapping_add(1);
        niven;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
