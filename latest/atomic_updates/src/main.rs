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
    fn rand() -> i32;
    fn sleep(__seconds: u32) -> u32;
    fn pthread_create(
        __newthread: *mut u64,
        __attr: *const pthread_attr_t,
// SAFETY: machine generated unsafe code
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> i32;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> i32;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [i8; 4],
    pub __align: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [i8; 56],
    pub __align: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [i8; 40],
    pub __align: i64,
}
#[no_mangle]
pub static mut bucket_mutex: [pthread_mutex_t; 15] = [pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
}; 15];
#[no_mangle]
pub static mut buckets: [i32; 15] = [0_i32; 15];
#[no_mangle]
pub static mut equalizer: u64 = 0;
#[no_mangle]
pub static mut randomizer: u64 = 0;
#[no_mangle]
pub extern "C" fn transfer_value(mut from: i32, mut to: i32, mut howmuch: i32) {
    let mut swapped: bool = 0 != 0;
    if from == to || howmuch < 0_i32 || from < 0_i32 || to < 0_i32 || from >= 15_i32 || to >= 15_i32 {
        return;
    }
    if from > to {
        std::mem::swap(&mut from, &mut to);
        swapped = 1_i32 != 0_i32;
        howmuch = -howmuch;
    }
// SAFETY: machine generated unsafe code
    unsafe {
        pthread_mutex_lock(&mut *bucket_mutex.as_mut_ptr().offset(from as isize));
        pthread_mutex_lock(&mut *bucket_mutex.as_mut_ptr().offset(to as isize));
        if howmuch > buckets[from as usize] && !swapped {
            howmuch = buckets[from as usize];
        }
        if -howmuch > buckets[to as usize] && i32::from(swapped) != 0_i32 {
            howmuch = -buckets[to as usize];
        }
        buckets[from as usize] -= howmuch;
        buckets[to as usize] += howmuch;
        pthread_mutex_unlock(&mut *bucket_mutex.as_mut_ptr().offset(from as isize));
        pthread_mutex_unlock(&mut *bucket_mutex.as_mut_ptr().offset(to as isize));
    }
}

#[no_mangle]
pub extern "C" fn print_buckets() {
    let mut i: i32 = 0;
    let mut sum: i32 = 0;
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 15_i32 {
            pthread_mutex_lock(&mut *bucket_mutex.as_mut_ptr().offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
    }
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 15_i32 {
            print!("{:3} ", buckets[i as usize]);
            sum += buckets[i as usize];
            i = i.wrapping_add(1);
            i;
        }
    }
    println!("= {}", sum);
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 15_i32 {
            pthread_mutex_unlock(&mut *bucket_mutex.as_mut_ptr().offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn equalizer_start(mut _t: *mut libc::c_void) -> *mut libc::c_void {
// SAFETY: machine generated unsafe code
    unsafe {
        loop {
            let mut b1: i32 = rand() % 15;
            let mut b2: i32 = rand() % 15;
            let mut diff: i32 = buckets[b1 as usize] - buckets[b2 as usize];
            if diff < 0_i32 {
                transfer_value(b2, b1, -diff / 2);
            } else {
                transfer_value(b1, b2, diff.wrapping_div(2));
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn randomizer_start(mut _t: *mut libc::c_void) -> *mut libc::c_void {
// SAFETY: machine generated unsafe code
    unsafe {
        loop {
            let mut b1: i32 = rand() % 15;
            let mut b2: i32 = rand() % 15;
            let mut diff: i32 = rand() % (buckets[b1 as usize] + 1);
            transfer_value(b1, b2, diff);
        }
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut total: i32 = 0;
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 15_i32 {
            pthread_mutex_init(
                &mut *bucket_mutex.as_mut_ptr().offset(i as isize),
                std::ptr::null::<pthread_mutexattr_t>(),
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 15_i32 {
            buckets[i as usize] = rand() % 100_i32;
            total += buckets[i as usize];
            print!("{:3} ", buckets[i as usize]);
            i = i.wrapping_add(1);
            i;
        }
    }
    println!("= {}", total);
// SAFETY: machine generated unsafe code
    unsafe {
        pthread_create(
            &mut equalizer,
            std::ptr::null::<pthread_attr_t>(),
// SAFETY: machine generated unsafe code
            Some(equalizer_start as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            std::ptr::null_mut::<libc::c_void>(),
        );
        pthread_create(
            &mut randomizer,
            std::ptr::null::<pthread_attr_t>(),
// SAFETY: machine generated unsafe code
            Some(randomizer_start as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            std::ptr::null_mut::<libc::c_void>(),
        );
    }
    let mut i_0: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        while i_0 < 2_i32 {
            sleep(1);
            print_buckets();
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 15_i32 {
            pthread_mutex_destroy(bucket_mutex.as_mut_ptr().offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
