#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use c2rust_out::*;
extern "C" {
    fn printf(_: *const i8, _: ...) -> i32;
}
fn main_0() -> i32 {
    let mut police: i32 = 0;
    let mut sanitation: i32 = 0;
    let mut fire: i32 = 0;
    unsafe {
        printf(b"Police     Sanitation         Fire\n\0" as *const u8 as *const i8);
        printf(b"----------------------------------\0" as *const u8 as *const i8);
    }
    police = 2;
    unsafe {
        while police <= 6 {
            sanitation = 1;
            while sanitation <= 7 {
                fire = 1;
                while fire <= 7 {
                    if police != sanitation
                        && sanitation != fire
                        && fire != police
                        && police + fire + sanitation == 12
                    {
                        printf(
                            b"\n%d\t\t%d\t\t%d\0" as *const u8 as *const i8,
                            police,
                            sanitation,
                            fire,
                        );
                    }
                    fire += 1;
                    fire;
                }
                sanitation += 1;
                sanitation;
            }
            police += 2;
        }
    }
    return 0;
}

pub fn main() {
    ::std::process::exit(main_0() as i32);
}
