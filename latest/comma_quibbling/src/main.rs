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
    fn perror(__s: *const i8);
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
}
#[no_mangle]
pub extern "C" fn quib(mut strs: *mut *const i8, mut size: u64) -> *mut i8 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut len: u64 = 3u64.wrapping_add(
            if size > 1 {
                2u64.wrapping_mul(size).wrapping_add(1)
            } else {
                0
            },
        );
        let mut i: u64 = 0;
        i = 0;
        while i < size {
            len = (len).wrapping_add(strlen(*strs.offset(i as isize)));
            i = i.wrapping_add(1);
            i;
        }
        let mut s: *mut i8 =
            malloc(len.wrapping_mul(::core::mem::size_of::<i8>() as u64)).cast::<i8>();
        if s.is_null() {
            perror((b"Can't allocate memory!\n\0" as *const u8).cast::<i8>());
            exit(1);
        }
        strcpy(s, (b"{\0" as *const u8).cast::<i8>());
        match size {
            0 => {}
            1 => {
                strcat(s, *strs.offset(0_isize));
            }
            _ => {
                i = 0;
                while i < size.wrapping_sub(1) {
                    strcat(s, *strs.offset(i as isize));
                    if i < size.wrapping_sub(2) {
                        strcat(s, (b", \0" as *const u8).cast::<i8>());
                    } else {
                        strcat(s, (b" and \0" as *const u8).cast::<i8>());
                    }
                    i = i.wrapping_add(1);
                    i;
                }
                strcat(s, *strs.offset(i as isize));
            }
        }
        strcat(s, (b"}\0" as *const u8).cast::<i8>());
        s
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut test: [*const i8; 4] = [
            (b"ABC\0" as *const u8).cast::<i8>(),
            (b"DEF\0" as *const u8).cast::<i8>(),
            (b"G\0" as *const u8).cast::<i8>(),
            (b"H\0" as *const u8).cast::<i8>(),
        ];
        let mut s: *mut i8 = std::ptr::null_mut::<i8>();
        let mut i: u64 = 0;
        while i < 5 {
            s = quib(test.as_mut_ptr(), i);
            println!("{}", build_str_from_raw_ptr(s.cast::<u8>()));
            free(s.cast::<libc::c_void>());
            i = i.wrapping_add(1);
            i;
        }
        0_i32
    }
}

pub fn main() {
// SAFETY: machine generated unsafe code
    unsafe {
        ::std::process::exit(main_0());
    }
}
