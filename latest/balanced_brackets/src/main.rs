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
    fn rand() -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn isBal(mut s: *const i8, mut l: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut c: i32 = 0;
        loop {
            let fresh0 = l;
            l = l.wrapping_sub(1);
            if fresh0 == 0_i32 {
                break;
            }
            if i32::from(*s.offset(l as isize)) == ']' as i32 {
                c = c.wrapping_add(1);
                c;
            } else {
                if i32::from(*s.offset(l as isize)) != '[' as i32 {
                    continue;
                }
                c = c.wrapping_sub(1);
                if c < 0_i32 {
                    break;
                }
            }
        }
        i32::from(c == 0_i32)
    }
}

#[no_mangle]
pub extern "C" fn shuffle(mut s: *mut i8, mut h: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut x: i32 = 0;
        let mut t: i32 = 0;
        let mut i: i32 = h;
        loop {
            let fresh1 = i;
            i = i.wrapping_sub(1);
            if fresh1 == 0_i32 {
                break;
            }
            x = rand() % h;
            t = i32::from(*s.offset(x as isize));
            *s.offset(x as isize) = *s.offset(i as isize);
            *s.offset(i as isize) = t as i8;
        }
    }
}

#[no_mangle]
pub extern "C" fn genSeq(mut s: *mut i8, mut n: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        if n != 0_i32 {
            memset(s.cast::<libc::c_void>(), '[' as i32, n as u64);
            memset(
                s.offset(n as isize).cast::<libc::c_void>(),
                ']' as i32,
                n as u64,
            );
            shuffle(s, n.wrapping_mul(2));
        };
        *s.offset((n.wrapping_mul(2i32)) as isize) = 0;
    }
}

#[no_mangle]
pub extern "C" fn doSeq(mut n: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut s: [i8; 64] = [0; 64];
        let mut o: *const i8 = (b"False\0" as *const u8).cast::<i8>();
        genSeq(s.as_mut_ptr(), n);
        if isBal(s.as_mut_ptr(), n.wrapping_mul(2)) != 0_i32 {
            o = (b"True\0" as *const u8).cast::<i8>();
        }
        println!(
            "{}: {}",
            build_str_from_raw_ptr(s.as_mut_ptr().cast::<u8>()),
            build_str_from_raw_ptr(o as *mut u8)
        );
    }
}

fn main_0() -> i32 {
    let mut n: i32 = 0;
    while n < 9_i32 {
        let fresh2 = n;
        n = n.wrapping_add(1);
        doSeq(fresh2);
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
