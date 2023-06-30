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
    fn malloc(_: u64) -> *mut libc::c_void;
}
fn main_0() -> i32 {
    unsafe {
        let mut i: i32 = 0;
        let mut flip: i32 = 0;
        let mut q: *mut i32 = malloc((::core::mem::size_of::<i32>() as u64).wrapping_mul(100000)).cast::<i32>()
            .offset(-1_isize);
        let fresh0 = &mut (*q.offset(2_isize));
        *fresh0 = 1_i32;
        *q.offset(1_isize) = *fresh0;
        i = 3_i32;
        while i <= 100_000_i32 {
            *q.offset(i as isize) = *q.offset((i - *q.offset((i - 1i32) as isize)) as isize)
                + *q.offset((i - *q.offset((i - 2i32) as isize)) as isize);
            i = i.wrapping_add(1);
            i;
        }
        i = 1_i32;
        while i <= 10_i32 {
            if i == 10_i32 {
                print!("{}{}", *q.offset(i as isize), '\n' as i32)
            } else {
                print!("{}{}", *q.offset(i as isize), ' ' as i32)
            };
            i = i.wrapping_add(1);
            i;
        }
        println!("{}", *q.offset(1000_isize));
        flip = 0_i32;
        i = 1_i32;
        while i < 100_000_i32 {
            flip += i32::from(*q.offset(i as isize) > *q.offset((i + 1i32) as isize));
            i = i.wrapping_add(1);
            i;
        }
        println!("flips: {}", flip);
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
