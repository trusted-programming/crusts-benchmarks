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
    fn printf(_: *const i8, _: ...) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
}
#[no_mangle]
pub static mut trans: [i8; 9] =
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe { *::core::mem::transmute::<&[u8; 9], &mut [i8; 9]>(b"___#_##_\0") };
#[no_mangle]
pub extern "C" fn evolve(mut cell: *mut i8, mut backup: *mut i8, mut len: i32) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut diff: i32 = 0;
        i = 0_i32;
        while i < len {
            *backup.offset(i as isize) =
                trans[(i32::from(i32::from(*cell.offset((i - 1i32) as isize)) != '_' as i32) * 4_i32
                    + i32::from(i32::from(*cell.offset(i as isize)) != '_' as i32) * 2_i32
                    + i32::from(i32::from(*cell.offset((i + 1i32) as isize)) != '_' as i32))
                    as usize];
            diff += i32::from(i32::from(*backup.offset(i as isize)) != i32::from(*cell.offset(i as isize)));
            i = i.wrapping_add(1);
            i;
        }
        strcpy(cell, backup as *const i8);
        diff
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        let mut c: [i8; 22] =
            *::core::mem::transmute::<&[u8; 22], &mut [i8; 22]>(b"_###_##_#_#_#_#__#__\n\0");
        let mut b: [i8; 22] =
            *::core::mem::transmute::<&[u8; 22], &mut [i8; 22]>(b"____________________\n\0");
        loop {
            printf(c.as_mut_ptr().offset(1_isize));
            if evolve(
                c.as_mut_ptr().offset(1_isize),
                b.as_mut_ptr().offset(1_isize),
                (::core::mem::size_of::<[i8; 22]>() as u64).wrapping_sub(3) as i32,
            ) == 0_i32
            {
                break;
            }
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
