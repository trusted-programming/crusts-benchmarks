#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sqrt(_: f64) -> f64;
    fn ceil(_: f64) -> f64;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct PrimeArray {
    pub ptr: *mut i64,
    pub size: u64,
    pub capacity: u64,
}
#[no_mangle]
pub extern "C" fn allocate() -> PrimeArray {
    let mut primes: PrimeArray = PrimeArray {
        ptr: std::ptr::null_mut::<i64>(),
        size: 0,
        capacity: 0,
    };
    primes.size = 0;
    primes.capacity = 10;
// SAFETY: machine generated unsafe code
    unsafe {
        primes.ptr = malloc((primes.capacity).wrapping_mul(::core::mem::size_of::<i64>() as u64)).cast::<i64>();
    }
    primes
}

#[no_mangle]
pub extern "C" fn deallocate(mut primes: *mut PrimeArray) {
// SAFETY: machine generated unsafe code
    unsafe {
        free((*primes).ptr.cast::<libc::c_void>());
        (*primes).ptr = std::ptr::null_mut::<i64>();
    }
}

#[no_mangle]
pub extern "C" fn push_back(mut primes: *mut PrimeArray, mut p: i64) {
// SAFETY: machine generated unsafe code
    unsafe {
        if (*primes).size >= (*primes).capacity {
            let mut new_capacity: u64 = 3u64
                .wrapping_mul((*primes).capacity)
                .wrapping_div(2)
                .wrapping_add(1);
            let mut temp: *mut i64 = realloc(
                (*primes).ptr.cast::<libc::c_void>(),
                new_capacity.wrapping_mul(::core::mem::size_of::<i64>() as u64),
            ).cast::<i64>();
            if temp.is_null() {
                fprintf(
                    stderr,
                    (b"Failed to reallocate the prime array.\0" as *const u8).cast::<i8>(),
                );
                exit(1);
            } else {
                (*primes).ptr = temp;
                (*primes).capacity = new_capacity;
            }
        }
        let fresh0 = (*primes).size;
        (*primes).size = ((*primes).size).wrapping_add(1);
        *((*primes).ptr).offset(fresh0 as isize) = p;
    }
}

fn main_0() -> i32 {
    let cutOff: i32 = 200;
    let bigUn: i32 = 100000;
    let chunks: i32 = 50;
    let little: i32 = bigUn / chunks;
    let mut primes: PrimeArray = allocate();
    let mut c: i32 = 0;
    let mut showEach: bool = 1 != 0;
    let mut u: i64 = 0;
    let mut v: i64 = 1;
    let mut i: i64 = 0;
    push_back(&mut primes, 3);
    push_back(&mut primes, 5);
    println!("The first {} cuban primes:", cutOff);
    i = 1;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 9223372036854775807 {
            let mut found: bool = 0 != 0;
            u = u.wrapping_add(6);
            v = v.wrapping_add(u);
            let mut mx: i64 = ceil(sqrt(v as f64)) as i64;
            let mut j: i64 = 0;
            j = 0;
            while (j as u64) < primes.size as u64 {
                if *(primes.ptr).offset(j as isize) > mx {
                    break;
                }
                if v % *(primes.ptr).offset(j as isize) == 0 {
                    found = 1_i32 != 0_i32;
                    break;
                } else {
                    j = j.wrapping_add(1);
                    j;
                }
            }
            if !found {
                c = c.wrapping_add(1);
                if showEach {
                    let mut z: i64 = 0;
                    z = *(primes.ptr).offset((primes.size).wrapping_sub(1) as isize) + 2;
                    while z <= v.wrapping_sub(2) {
                        let mut fnd: bool = 0 != 0;
                        j = 0;
                        while (j as u64) < primes.size as u64 {
                            if *(primes.ptr).offset(j as isize) > mx {
                                break;
                            }
                            if z % *(primes.ptr).offset(j as isize) == 0 {
                                fnd = 1_i32 != 0_i32;
                                break;
                            } else {
                                j = j.wrapping_add(1);
                                j;
                            }
                        }
                        if !fnd {
                            push_back(&mut primes, z);
                        }
                        z = z.wrapping_add(2);
                    }
                    push_back(&mut primes, v);
                    print!("{:11}", v);
                    if c % 10_i32 == 0_i32 {
                        println!();
                    }
                    if c == cutOff {
                        showEach = 0_i32 != 0_i32;
                        print!("\nProgress to the {}th cuban prime: ", bigUn);
                    }
                }
                if c % little == 0_i32 {
                    print!(".");
                    if c == bigUn {
                        break;
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    print!("\nThe {}th cuban prime is {}\n", c, v);
    deallocate(&mut primes);
    0_i32
}

pub fn main() {
// SAFETY: machine generated unsafe code
    unsafe {
        ::std::process::exit(main_0());
    }
}
