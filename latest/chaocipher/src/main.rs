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
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub const DECRYPT: u32 = 1;
pub const ENCRYPT: u32 = 0;
#[no_mangle]
pub static mut l_alphabet: *const i8 = (b"HXUCZVAMDSLKPEFJRIGTWOBNYQ\0" as *const u8).cast::<i8>();
#[no_mangle]
pub static mut r_alphabet: *const i8 = (b"PTLNBQDEOYSFAVZKGJRIHWXUMC\0" as *const u8).cast::<i8>();
#[no_mangle]
pub extern "C" fn chao(mut in_0: *const i8, mut out: *mut i8, mut mode: u32, mut show_steps: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut index: i32 = 0;
        let mut store: i8 = 0;
        let mut len: u64 = strlen(in_0);
        let mut left: [i8; 27] = [0; 27];
        let mut right: [i8; 27] = [0; 27];
        let mut temp: [i8; 27] = [0; 27];
        strcpy(left.as_mut_ptr(), l_alphabet);
        strcpy(right.as_mut_ptr(), r_alphabet);
        temp[26_usize] = '\0' as i8;
        i = 0_i32;
        while (i as u64) < len {
            if show_steps != 0_i32 {
                println!(
                    "{}  {}",
                    build_str_from_raw_ptr(left.as_mut_ptr().cast::<u8>()),
                    build_str_from_raw_ptr(right.as_mut_ptr().cast::<u8>())
                );
            }
            if mode == ENCRYPT {
                index = (strchr(right.as_mut_ptr(), i32::from(*in_0.offset(i as isize))))
                    .offset_from(right.as_mut_ptr()) as i32;
                *out.offset(i as isize) = left[index as usize];
            } else {
                index = (strchr(left.as_mut_ptr(), i32::from(*in_0.offset(i as isize))))
                    .offset_from(left.as_mut_ptr()) as i32;
                *out.offset(i as isize) = right[index as usize];
            }
            if i as u64 == len.wrapping_sub(1) {
                break;
            }
            j = index;
            while j < 26_i32 {
                temp[(j - index) as usize] = left[j as usize];
                j = j.wrapping_add(1);
                j;
            }
            j = 0_i32;
            while j < index {
                temp[(26_i32 - index.wrapping_add(j)) as usize] = left[j as usize];
                j = j.wrapping_add(1);
                j;
            }
            store = temp[1_usize];
            j = 2_i32;
            while j < 14_i32 {
                temp[(j.wrapping_sub(1i32)) as usize] = temp[j as usize];
                j = j.wrapping_add(1);
                j;
            }
            temp[13_usize] = store;
            strcpy(left.as_mut_ptr(), temp.as_mut_ptr());
            j = index;
            while j < 26_i32 {
                temp[(j - index) as usize] = right[j as usize];
                j = j.wrapping_add(1);
                j;
            }
            j = 0_i32;
            while j < index {
                temp[(26_i32 - index.wrapping_add(j)) as usize] = right[j as usize];
                j = j.wrapping_add(1);
                j;
            }
            store = temp[0_usize];
            j = 1_i32;
            while j < 26_i32 {
                temp[(j.wrapping_sub(1i32)) as usize] = temp[j as usize];
                j = j.wrapping_add(1);
                j;
            }
            temp[25_usize] = store;
            store = temp[2_usize];
            j = 3_i32;
            while j < 14_i32 {
                temp[(j.wrapping_sub(1i32)) as usize] = temp[j as usize];
                j = j.wrapping_add(1);
                j;
            }
            temp[13_usize] = store;
            strcpy(right.as_mut_ptr(), temp.as_mut_ptr());
            i = i.wrapping_add(1);
            i;
        }
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut plain_text: *const i8 = (b"WELLDONEISBETTERTHANWELLSAID\0" as *const u8).cast::<i8>();
        let mut cipher_text: *mut i8 = malloc((strlen(plain_text)).wrapping_add(1)).cast::<i8>();
        let mut plain_text2: *mut i8 = malloc((strlen(plain_text)).wrapping_add(1)).cast::<i8>();
        println!(
            "The original plaintext is : {}",
            build_str_from_raw_ptr(plain_text as *mut u8)
        );
        print!("\nThe left and right alphabets after each permutation during encryption are :\n\n");
        chao(plain_text, cipher_text, ENCRYPT, 1);
        print!(
            "\nThe ciphertext is : {}\n",
            build_str_from_raw_ptr(cipher_text.cast::<u8>())
        );
        chao(cipher_text, plain_text2, DECRYPT, 0);
        print!(
            "\nThe recovered plaintext is : {}\n",
            build_str_from_raw_ptr(plain_text2.cast::<u8>())
        );
        free(cipher_text.cast::<libc::c_void>());
        free(plain_text2.cast::<libc::c_void>());
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
