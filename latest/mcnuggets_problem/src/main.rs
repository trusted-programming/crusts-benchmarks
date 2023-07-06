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
    let mut max: i32 = 0;
    let mut i: i32 = 0;
    let mut sixes: i32 = 0;
    let mut nines: i32 = 0;
    let mut twenties: i32 = 0;
    's_3: while i < 100_i32 {
        sixes = 0_i32;
        while (sixes * 6_i32) < i {
            if sixes * 6_i32 == i {
                i += 1_i32;
                i;
                continue 's_3;
            } else {
                nines = 0_i32;
                while (nines * 9_i32) < i {
                    if sixes * 6_i32 + nines * 9_i32 == i {
                        i += 1_i32;
                        i;
                        continue 's_3;
                    } else {
                        twenties = 0_i32;
                        while (twenties * 20_i32) < i {
                            if sixes * 6_i32 + nines * 9_i32 + twenties * 20_i32 == i {
                                i += 1_i32;
                                i;
                                continue 's_3;
                            } else {
                                twenties += 1_i32;
                                twenties;
                            }
                        }
                        nines += 1_i32;
                        nines;
                    }
                }
                sixes += 1_i32;
                sixes;
            }
        }
        max = i;
        i += 1_i32;
        i;
    }
    println!("Maximum non-McNuggets number is {}", max);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
