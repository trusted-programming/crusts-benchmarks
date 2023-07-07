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
pub static mut one: u64 = 1;
#[no_mangle]
pub extern "C" fn comb(mut pool: i32, mut need: i32, mut chosen: u64, mut at: i32) {
    if pool < need.wrapping_add(at) {
        return;
    }
// SAFETY: machine generated unsafe code
    unsafe {
        if need == 0_i32 {
            at = 0_i32;
            while at < pool {
                if chosen & one << at != 0 {
                    print!("{} ", at);
                }
                at = at.wrapping_add(1);
                at;
            }
            println!();
            return;
        }
        comb(
            pool,
            need.wrapping_sub(1),
            chosen | one << at,
            at.wrapping_add(1),
        );
    }
    comb(pool, need, chosen, at.wrapping_add(1));
}

fn main_0() -> i32 {
    comb(5, 3, 0, 0);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
