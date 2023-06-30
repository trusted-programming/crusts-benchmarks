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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct i128_0 {
    pub x: [u64; 2],
}
#[no_mangle]
pub extern "C" fn show(mut v: i128_0) {
    let mut x: [u32; 4] = [
        v.x[0_usize] as u32,
        (v.x[0_usize] >> 32i32) as u32,
        v.x[1_usize] as u32,
        (v.x[1_usize] >> 32i32) as u32,
    ];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut len: i32 = 4;
    let mut buf: [i8; 100] = [0; 100];
    loop {
        let mut c: u64 = 0;
        i = len;
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if fresh0 == 0_i32 {
                break;
            }
            c = (c << 32i32).wrapping_add(u64::from(x[i as usize]));
            x[i as usize] = c.wrapping_div(10) as u32;
            c = (c).wrapping_rem(10);
        }
        let fresh1 = j;
        j = j.wrapping_add(1);
        buf[fresh1 as usize] = c.wrapping_add('0' as u64) as i8;
        len = 4_i32;
        while x[(len - 1i32) as usize] == 0 {
            len = len.wrapping_sub(1);
            len;
        }
        if len == 0_i32 {
            break;
        }
    }
    loop {
        let fresh2 = j;
        j = j.wrapping_sub(1);
        if fresh2 == 0_i32 {
            break;
        }
        print!("{}", i32::from(buf[j as usize]));
    }
    print!("{}", '\n' as i32);
}

#[no_mangle]
pub extern "C" fn count(mut sum: i32, mut coins: *mut i32) -> i128_0 {
    unsafe {
        let mut n: i32 = 0;
        let mut i: i32 = 0;
        let mut k: i32 = 0;
        n = 0_i32;
        while *coins.offset(n as isize) != 0_i32 {
            n = n.wrapping_add(1);
            n;
        }
        let mut v: *mut *mut i128_0 =
            malloc((::core::mem::size_of::<*mut i32>() as u64).wrapping_mul(n as u64)).cast::<*mut i128_0>();
        let mut idx: *mut i32 =
            malloc((::core::mem::size_of::<i32>() as u64).wrapping_mul(n as u64)).cast::<i32>();
        i = 0_i32;
        while i < n {
            *idx.offset(i as isize) = *coins.offset(i as isize);
            let fresh3 = &mut (*v.offset(i as isize));
            *fresh3 = calloc(
                ::core::mem::size_of::<i128_0>() as u64,
                *coins.offset(i as isize) as u64,
            ).cast::<i128_0>();
            i = i.wrapping_add(1);
            i;
        }
        *(*v.offset(0_isize)).offset((*coins.offset(0_isize) - 1i32) as isize) = {
            
            i128_0 { x: [1, 0] }
        };
        k = 0_i32;
        while k <= sum {
            i = 0_i32;
            while i < n {
                let fresh4 = &mut (*idx.offset(i as isize));
                let fresh5 = *fresh4;
                *fresh4 -= 1_i32;
                if fresh5 == 0_i32 {
                    *idx.offset(i as isize) = *coins.offset(i as isize) - 1_i32;
                }
                i = i.wrapping_add(1);
                i;
            }
            let mut c: i128_0 = *(*v.offset(0_isize)).offset(*idx.offset(0_isize) as isize);
            i = 1_i32;
            while i < n {
                let mut p: *mut i128_0 =
                    (*v.offset(i as isize)).offset(*idx.offset(i as isize) as isize);
                (*p).x[0_usize] =
                    ((*p).x[0_usize] as u64).wrapping_add(c.x[0_usize]) as u64;
                (*p).x[1_usize] =
                    ((*p).x[1_usize] as u64).wrapping_add(c.x[1_usize]) as u64;
                if (*p).x[0_usize] < c.x[0_usize] {
                    (*p).x[1_usize] = ((*p).x[1_usize]).wrapping_add(1);
                    (*p).x[1_usize];
                }
                c = *p;
                i = i.wrapping_add(1);
                i;
            }
            k = k.wrapping_add(1);
            k;
        }
        let mut r: i128_0 =
            *(*v.offset((n - 1i32) as isize)).offset(*idx.offset((n - 1i32) as isize) as isize);
        i = 0_i32;
        while i < n {
            free((*v.offset(i as isize)).cast::<libc::c_void>());
            i = i.wrapping_add(1);
            i;
        }
        free(v.cast::<libc::c_void>());
        free(idx.cast::<libc::c_void>());
        r
    }
}

#[no_mangle]
pub extern "C" fn count2(mut sum: i32, mut coins: *mut i32) -> i32 {
    unsafe {
        if *coins == 0_i32 || sum < 0_i32 {
            return 0_i32;
        }
        if sum == 0_i32 {
            return 1_i32;
        }
        count2(sum - *coins, coins) + count2(sum, coins.offset(1_isize))
    }
}

fn main_0() -> i32 {
    let mut us_coins: [i32; 7] = [100, 50, 25, 10, 5, 1, 0];
    let mut eu_coins: [i32; 9] = [200, 100, 50, 20, 10, 5, 2, 1, 0];
    unsafe {
        show(count(100, us_coins.as_mut_ptr().offset(2_isize)));
    }
    show(count(1000, us_coins.as_mut_ptr()));
    show(count(1000 * 100, us_coins.as_mut_ptr()));
    show(count(10000 * 100, us_coins.as_mut_ptr()));
    show(count(100000 * 100, us_coins.as_mut_ptr()));
    print!("{}", '\n' as i32);
    show(count(100, eu_coins.as_mut_ptr()));
    show(count(1000 * 100, eu_coins.as_mut_ptr()));
    show(count(10000 * 100, eu_coins.as_mut_ptr()));
    show(count(100000 * 100, eu_coins.as_mut_ptr()));
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
