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
fn main_0() -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut try_max: i32 = 0;
    let mut count_list: [i32; 3] = [1, 0, 0];
    i = 2_i32;
    while i <= 20_000_i32 {
        try_max = i / 2_i32;
        sum = 1_i32;
        j = 2_i32;
        while j < try_max {
            if i % j == 0_i32 {
                try_max = i / j;
                sum = sum.wrapping_add(j);
                if j != try_max {
                    sum = sum.wrapping_add(try_max);
                }
            }
            j = j.wrapping_add(1);
            j;
        }
        if sum < i {
            count_list[0_usize] += 1_i32;
            count_list[0_usize];
        } else if sum > i {
            count_list[2_usize] += 1_i32;
            count_list[2_usize];
        } else {
            count_list[1_usize] += 1_i32;
            count_list[1_usize];
        }
        i = i.wrapping_add(1);
        i;
    }
    print!("\nThere are {} deficient,", count_list[0_usize]);
    print!(" {} perfect,", count_list[1_usize]);
    println!(
        " {} abundant numbers between 1 and 20000.",
        count_list[2_usize]
    );
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
