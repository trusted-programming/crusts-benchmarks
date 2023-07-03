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


extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub extern "C" fn move_to_front(mut str: *mut i8, mut c: i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut q: *mut i8 = std::ptr::null_mut::<i8>();
        let mut p: *mut i8 = std::ptr::null_mut::<i8>();
        let mut shift: i32 = 0;
        p = malloc((strlen(str)).wrapping_add(1)).cast::<i8>();
        strcpy(p, str);
        q = strchr(p, i32::from(c));
        shift = q.offset_from(p) as i32;
        strncpy(str.offset(1_isize), p, shift as u64);
        *str.offset(0_isize) = c;
        free(p.cast::<libc::c_void>());
        shift
    }
}

#[no_mangle]
pub extern "C" fn decode(mut pass: *mut i32, mut size: i32, mut sym: *mut i8) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut index: i32 = 0;
        let mut c: i8 = 0;
        let mut table: [i8; 27] =
            *::core::mem::transmute::<&[u8; 27], &mut [i8; 27]>(b"abcdefghijklmnopqrstuvwxyz\0");
        i = 0_i32;
        while i < size {
            c = table[*pass.offset(i as isize) as usize];
            index = move_to_front(table.as_mut_ptr(), c);
            if *pass.offset(i as isize) != index {
                print!("there is an error");
            };
            *sym.offset(i as isize) = c;
            i = i.wrapping_add(1);
            i;
        }
        *sym.offset(size as isize) = '\0' as i8;
    }
}

#[no_mangle]
pub extern "C" fn encode(mut sym: *mut i8, mut size: i32, mut pass: *mut i32) {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut c: i8 = 0;
        let mut table: [i8; 27] =
            *::core::mem::transmute::<&[u8; 27], &mut [i8; 27]>(b"abcdefghijklmnopqrstuvwxyz\0");
        i = 0_i32;
        while i < size {
            c = *sym.offset(i as isize);
            *pass.offset(i as isize) = move_to_front(table.as_mut_ptr(), c);
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn check(mut sym: *mut i8, mut size: i32, mut pass: *mut i32) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut pass2: *mut i32 =
            malloc((::core::mem::size_of::<i32>() as u64).wrapping_mul(size as u64)).cast::<i32>();
        let mut sym2: *mut i8 =
            malloc((::core::mem::size_of::<i8>() as u64).wrapping_mul(size as u64)).cast::<i8>();
        let mut i: i32 = 0;
        let mut val: i32 = 1;
        encode(sym, size, pass2);
        i = 0_i32;
        while i < size && *pass.offset(i as isize) == *pass2.offset(i as isize) {
            i = i.wrapping_add(1);
            i;
        }
        if i != size {
            val = 0_i32;
        }
        decode(pass, size, sym2);
        if strcmp(sym, sym2) != 0_i32 {
            val = 0_i32;
        }
        free(sym2.cast::<libc::c_void>());
        free(pass2.cast::<libc::c_void>());
        val
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut sym : [[i8; 100]; 3] = [* :: core :: mem :: transmute :: < & [u8; 100], & mut [i8; 100], > (
          b"broood\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",), * :: core :: mem :: transmute :: < & [u8; 100
          ], & mut [i8; 100], > (b"bananaaa\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",), * :: core :: mem ::
          transmute :: < & [u8; 100], & mut [i8; 100], > (b"hiphophiphop\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",),];
        let mut pass: [i32; 100] = [0; 100];
        let mut i: i32 = 0;
        let mut len: i32 = 0;
        let mut j: i32 = 0;
        i = 0_i32;
        while i < 3_i32 {
            len = strlen((sym[i as usize]).as_mut_ptr()) as i32;
            encode((sym[i as usize]).as_mut_ptr(), len, pass.as_mut_ptr());
            print!(
                "{} : [",
                build_str_from_raw_ptr((sym[i as usize]).as_mut_ptr().cast::<u8>())
            );
            j = 0_i32;
            while j < len {
                print!("{} ", pass[j as usize]);
                j = j.wrapping_add(1);
                j;
            }
            println!("]");
            if check((sym[i as usize]).as_mut_ptr(), len, pass.as_mut_ptr()) != 0_i32 {
                println!("Correct :)");
            } else {
                println!("Incorrect :(");
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
