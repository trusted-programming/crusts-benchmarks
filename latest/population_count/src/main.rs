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
    let mut n: u64 = 1;
    let mut i: i32 = 0;
    while i < 30_i32 {
        print!("{} ", n.count_ones() as i32);
        n = n.wrapping_mul(3);
        i = i.wrapping_add(1);
        i;
    }
    println!();
    let mut od: [i32; 30] = [0; 30];
    let mut ne: i32 = 0;
    let mut no: i32 = 0;
    print!("evil  : ");
    let mut n_0: i32 = 0;
    while ne + no < 60_i32 {
        if (n_0 as u32).count_ones() as i32 & 1_i32 == 0_i32 {
            if ne < 30_i32 {
                print!("{} ", n_0);
                ne = ne.wrapping_add(1);
                ne;
            }
        } else if no < 30_i32 {
            let fresh0 = no;
            no = no.wrapping_add(1);
            od[fresh0 as usize] = n_0;
        }
        n_0 = n_0.wrapping_add(1);
        n_0;
    }
    println!();
    print!("odious: ");
    let mut i_0: i32 = 0;
    while i_0 < 30_i32 {
        print!("{} ", od[i_0 as usize]);
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    println!();
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
