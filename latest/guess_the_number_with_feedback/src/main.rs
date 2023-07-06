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
    fn scanf(_: *const i8, _: ...) -> i32;
}
fn main_0() -> i32 {
    let mut bounds: [i32; 2] = [1, 100];
// SAFETY: machine generated unsafe code
    unsafe {
        let mut input: [i8; 2] = *::core::mem::transmute::<&[u8; 2], &mut [i8; 2]>(b"  ");
        let mut choice: i32 = (bounds[0_usize] + bounds[1_usize]) / 2;
        println!(
            "Choose a number between {} and {}.",
            bounds[0_usize], bounds[1_usize]
        );
        loop {
            match i32::from(input[0_usize]) {
                72_i32 => {
                    bounds[1_usize] = choice;
                }
                76_i32 => {
                    bounds[0_usize] = choice;
                }
                89_i32 => {
                    print!("\nAwwwright\n");
                    return 0_i32;
                }
                _ => {}
            }
            choice = (bounds[0_usize] + bounds[1_usize]) / 2_i32;
            print!("Is the number {}? (Y/H/L) ", choice);
            if scanf((b"%1s\0" as *const u8).cast::<i8>(), input.as_mut_ptr()) != 1_i32 {
                break;
            }
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
