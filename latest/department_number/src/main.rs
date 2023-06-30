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
    let mut police: i32 = 0;
    let mut sanitation: i32 = 0;
    let mut fire: i32 = 0;
    println!("Police     Sanitation         Fire");
    print!("----------------------------------");
    police = 2_i32;
    while police <= 6_i32 {
        sanitation = 1_i32;
        while sanitation <= 7_i32 {
            fire = 1_i32;
            while fire <= 7_i32 {
                if police != sanitation
                    && sanitation != fire
                    && fire != police
                    && police + fire + sanitation == 12_i32
                {
                    print!("\n{}		{}		{}", police, sanitation, fire);
                }
                fire = fire.wrapping_add(1);
                fire;
            }
            sanitation = sanitation.wrapping_add(1);
            sanitation;
        }
        police = police.wrapping_add(2);
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
