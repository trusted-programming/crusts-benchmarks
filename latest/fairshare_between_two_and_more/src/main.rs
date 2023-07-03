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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub extern "C" fn turn(mut base: i32, mut n: i32) -> i32 {
    let mut sum: i32 = 0;
    while n != 0_i32 {
        let mut rem: i32 = n % base;
        n /= base;
        sum = sum.wrapping_add(rem);
    }
    sum % base
}

#[no_mangle]
pub extern "C" fn fairshare(mut base: i32, mut count: i32) {
    let mut i: i32 = 0;
    print!("Base {:2}:", base);
    i = 0_i32;
    while i < count {
        let mut t: i32 = turn(base, i);
        print!(" {:2}", t);
        i = i.wrapping_add(1);
        i;
    }
    println!();
}

#[no_mangle]
pub extern "C" fn turnCount(mut base: i32, mut count: i32) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut cnt: *mut i32 =
            calloc(base as u64, ::core::mem::size_of::<i32>() as u64).cast::<i32>();
        let mut i: i32 = 0;
        let mut minTurn: i32 = 0;
        let mut maxTurn: i32 = 0;
        let mut portion: i32 = 0;
        if cnt.is_null() {
            println!("Failed to allocate space to determine the spread of turns.");
            return;
        }
        i = 0_i32;
        while i < count {
            let mut t: i32 = turn(base, i);
            let fresh0 = &mut (*cnt.offset(t as isize));
            *fresh0 += 1_i32;
            *fresh0;
            i = i.wrapping_add(1);
            i;
        }
        minTurn = 2_147_483_647_i32;
        maxTurn = -2_147_483_647_i32 - 1_i32;
        portion = 0_i32;
        i = 0_i32;
        while i < base {
            if *cnt.offset(i as isize) > 0_i32 {
                portion = portion.wrapping_add(1);
                portion;
            }
            if *cnt.offset(i as isize) < minTurn {
                minTurn = *cnt.offset(i as isize);
            }
            if *cnt.offset(i as isize) > maxTurn {
                maxTurn = *cnt.offset(i as isize);
            }
            i = i.wrapping_add(1);
            i;
        }
        print!("  With {} people: ", base);
        if 0_i32 == minTurn {
            println!("Only {} have a turn", portion);
        } else if minTurn == maxTurn {
            println!("{}", minTurn);
        } else {
            println!("{} or {}", minTurn, maxTurn);
        }
        free(cnt.cast::<libc::c_void>());
    }
}

fn main_0() -> i32 {
    fairshare(2, 25);
    fairshare(3, 25);
    fairshare(5, 25);
    fairshare(11, 25);
    println!("How many times does each get a turn in 50000 iterations?");
    turnCount(191, 50000);
    turnCount(1377, 50000);
    turnCount(49999, 50000);
    turnCount(50000, 50000);
    turnCount(50001, 50000);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
