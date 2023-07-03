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
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
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
static mut animals: [*const i8; 8] = [
    (b"fly\0" as *const u8).cast::<i8>(),
    (b"spider\0" as *const u8).cast::<i8>(),
    (b"bird\0" as *const u8).cast::<i8>(),
    (b"cat\0" as *const u8).cast::<i8>(),
    (b"dog\0" as *const u8).cast::<i8>(),
    (b"goat\0" as *const u8).cast::<i8>(),
    (b"cow\0" as *const u8).cast::<i8>(),
    (b"horse\0" as *const u8).cast::<i8>(),
];
static mut verses: [*const i8; 8] = [
    (b"I don't know why she swallowed that fly.\nPerhaps she'll die\n\0" as *const u8).cast::<i8>(),
    (b"That wiggled and jiggled and tickled inside her\0" as *const u8).cast::<i8>(),
    (b"How absurd, to swallow a bird\0" as *const u8).cast::<i8>(),
    (b"Imagine that. She swallowed a cat\0" as *const u8).cast::<i8>(),
    (b"What a hog to swallow a dog\0" as *const u8).cast::<i8>(),
    (b"She just opened her throat and swallowed that goat\0" as *const u8).cast::<i8>(),
    (b"I don't know how she swallowed that cow\0" as *const u8).cast::<i8>(),
    (b"She's dead of course\0" as *const u8).cast::<i8>(),
];
fn main_0() -> i32 {
    let mut i: u64 = 0;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        while i
            < (::core::mem::size_of::<[*const i8; 8]>() as u64)
                .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
        {
            print!(
                "There was an old lady who swallowed a {}\n{}\n",
                build_str_from_raw_ptr(animals[i as usize] as *mut u8),
                build_str_from_raw_ptr(verses[i as usize] as *mut u8)
            );
            let mut j: u64 = i;
            while j > 0
                && i < (::core::mem::size_of::<[*const i8; 8]>() as u64)
                    .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
                    .wrapping_sub(1)
            {
                println!(
                    "She swallowed the {} to catch the {}",
                    build_str_from_raw_ptr(animals[j as usize] as *mut u8),
                    build_str_from_raw_ptr(animals[j.wrapping_sub(1) as usize] as *mut u8)
                );
                if j == 1 {
                    println!(
                        "{}",
                        build_str_from_raw_ptr(verses[0_usize] as *mut u8)
                    );
                }
                j = j.wrapping_sub(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
