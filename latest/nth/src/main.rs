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


extern "C" {
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
}
#[no_mangle]
pub extern "C" fn addSuffix(mut num: i32, mut buf: *mut i8, mut len: u64) -> *mut i8 {
    unsafe {
        let mut suffixes: [*mut i8; 4] = [
            (b"th\0" as *const u8).cast::<i8>() as *mut i8,
            (b"st\0" as *const u8).cast::<i8>() as *mut i8,
            (b"nd\0" as *const u8).cast::<i8>() as *mut i8,
            (b"rd\0" as *const u8).cast::<i8>() as *mut i8,
        ];
        let mut i: i32 = 0;
        match num % 10_i32 {
            1_i32 => {
                i = if num % 100_i32 == 11_i32 { 0_i32 } else { 1_i32 };
            }
            2_i32 => {
                i = if num % 100_i32 == 12_i32 { 0_i32 } else { 2_i32 };
            }
            3_i32 => {
                i = if num % 100_i32 == 13_i32 { 0_i32 } else { 3_i32 };
            }
            _ => {
                i = 0_i32;
            }
        }
        snprintf(
            buf,
            len,
            (b"%d%s\0" as *const u8).cast::<i8>(),
            num,
            suffixes[i as usize],
        );
        buf
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    println!("Set [0,25]:");
    i = 0_i32;
    unsafe {
        while i < 26_i32 {
            let mut s: [i8; 5] = [0; 5];
            print!(
                "{} ",
                build_str_from_raw_ptr(addSuffix(i, s.as_mut_ptr(), 5).cast::<u8>())
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    print!("{}", '\n' as i32);
    println!("Set [250,265]:");
    i = 250_i32;
    unsafe {
        while i < 266_i32 {
            let mut s_0: [i8; 6] = [0; 6];
            print!(
                "{} ",
                build_str_from_raw_ptr(addSuffix(i, s_0.as_mut_ptr(), 6).cast::<u8>())
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    print!("{}", '\n' as i32);
    println!("Set [1000,1025]:");
    i = 1_000_i32;
    unsafe {
        while i < 1_026_i32 {
            let mut s_1: [i8; 7] = [0; 7];
            print!(
                "{} ",
                build_str_from_raw_ptr(addSuffix(i, s_1.as_mut_ptr(), 7).cast::<u8>())
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    print!("{}", '\n' as i32);
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
