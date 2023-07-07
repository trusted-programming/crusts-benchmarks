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
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 12;
    j = 1_i32;
    while j <= n {
        if j != n {
            print!("{:3}{}", j, ' ' as i32)
        } else {
            print!("{:3}{}", j, '\n' as i32)
        };
        j = j.wrapping_add(1);
        j;
    }
    j = 0_i32;
    while j <= n {
        if j != n {
            print!("----")
        } else {
            println!("+")
        };
        j = j.wrapping_add(1);
        j;
    }
    i = 1_i32;
    while i <= n {
        j = 1_i32;
        while j <= n {
            if j < i {
                print!("    ")
            } else {
                print!("{:3} ", i * j)
            };
            j = j.wrapping_add(1);
            j;
        }
        println!("| {}", i);
        i = i.wrapping_add(1);
        i;
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
