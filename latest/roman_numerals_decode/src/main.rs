#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.add(str_size) != 0 {
            str_size = str_size.wrapping_add(1);
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {}
#[no_mangle]
pub static mut digits: [i32; 26] = [
    0_i32, 0_i32, 100_i32, 500_i32, 0_i32, 0_i32, 0_i32, 0_i32, 1_i32, 1_i32, 0_i32, 50_i32, 1_000_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 5_i32, 5_i32, 0_i32, 10_i32, 0_i32, 0_i32,
];
#[no_mangle]
pub extern "C" fn decode(mut roman: *const i8) -> i32 {
    unsafe {
        let mut bigger: *const i8 = std::ptr::null::<i8>();
        let mut current: i32 = 0;
        let mut arabic: i32 = 0;
        while i32::from(*roman) != '\0' as i32 {
            current = digits[((!0x20i32 & i32::from(*roman)) - 'A' as i32) as usize];
            bigger = roman;
            while digits[((!0x20i32 & i32::from(*bigger)) - 'A' as i32) as usize] <= current && {
                bigger = bigger.offset(1);
                i32::from(*bigger) != '\0' as i32
            } {}
            if i32::from(*bigger) == '\0' as i32 {
                arabic = arabic.wrapping_add(current);
            } else {
                arabic += digits[((!0x20i32 & i32::from(*bigger)) - 'A' as i32) as usize];
                while roman < bigger {
                    let fresh0 = roman;
                    roman = roman.offset(1);
                    arabic -= digits[((!0x20i32 & i32::from(*fresh0)) - 'A' as i32) as usize];
                }
            }
            roman = roman.offset(1);
            roman;
        }
        arabic
    }
}

fn main_0() -> i32 {
    let mut romans: [*const i8; 4] = [
        (b"MCmxC\0" as *const u8).cast::<i8>(),
        (b"MMVIII\0" as *const u8).cast::<i8>(),
        (b"MDClXVI\0" as *const u8).cast::<i8>(),
        (b"MCXLUJ\0" as *const u8).cast::<i8>(),
    ];
    let mut i: i32 = 0;
    i = 0_i32;
    unsafe {
        while i < 4_i32 {
            println!(
                "{}	{}",
                build_str_from_raw_ptr(romans[i as usize] as *mut u8),
                decode(romans[i as usize])
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
