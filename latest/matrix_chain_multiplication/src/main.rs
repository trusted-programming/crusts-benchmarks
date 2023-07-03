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
#[no_mangle]
pub static mut m: *mut *mut i32 = 0 as *const *mut i32 as *mut *mut i32;
#[no_mangle]
pub static mut s: *mut *mut i32 = 0 as *const *mut i32 as *mut *mut i32;
#[no_mangle]
pub extern "C" fn optimal_matrix_chain_order(mut dims: *mut i32, mut n: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut len: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut temp: i32 = 0;
        let mut cost: i32 = 0;
        n = n.wrapping_sub(1);
        n;
        m = malloc((n as u64).wrapping_mul(::core::mem::size_of::<*mut i32>() as u64)).cast::<*mut i32>();
        i = 0_i32;
        while i < n {
            let fresh0 = &mut (*m.offset(i as isize));
            *fresh0 = calloc(n as u64, ::core::mem::size_of::<i32>() as u64).cast::<i32>();
            i = i.wrapping_add(1);
            i;
        }
        s = malloc((n as u64).wrapping_mul(::core::mem::size_of::<*mut i32>() as u64)).cast::<*mut i32>();
        i = 0_i32;
        while i < n {
            let fresh1 = &mut (*s.offset(i as isize));
            *fresh1 = calloc(n as u64, ::core::mem::size_of::<i32>() as u64).cast::<i32>();
            i = i.wrapping_add(1);
            i;
        }
        len = 1_i32;
        while len < n {
            i = 0_i32;
            while i < n - len {
                j = i + len;
                *(*m.offset(i as isize)).offset(j as isize) = 2_147_483_647_i32;
                k = i;
                while k < j {
                    temp = *dims.offset(i as isize)
                        * *dims.offset((k + 1i32) as isize)
                        * *dims.offset((j + 1i32) as isize);
                    cost = *(*m.offset(i as isize)).offset(k as isize)
                        + *(*m.offset((k + 1i32) as isize)).offset(j as isize)
                        + temp;
                    if cost < *(*m.offset(i as isize)).offset(j as isize) {
                        *(*m.offset(i as isize)).offset(j as isize) = cost;
                        *(*s.offset(i as isize)).offset(j as isize) = k;
                    }
                    k = k.wrapping_add(1);
                    k;
                }
                i = i.wrapping_add(1);
                i;
            }
            len = len.wrapping_add(1);
            len;
        }
    }
}

#[no_mangle]
pub extern "C" fn print_optimal_chain_order(mut i: i32, mut j: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        if i == j {
            print!("{}", i + 65_i32);
        } else {
            print!("(");
            print_optimal_chain_order(i, *(*s.offset(i as isize)).offset(j as isize));
            print_optimal_chain_order(*(*s.offset(i as isize)).offset(j as isize) + 1, j);
            print!(")");
        };
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut n: i32 = 0;
    let mut a1: [i32; 4] = [5, 6, 3, 1];
    let mut a2: [i32; 13] = [1, 5, 25, 30, 100, 70, 2, 1, 100, 250, 1, 1000, 2];
    let mut a3: [i32; 12] = [1000, 1, 500, 12, 1, 700, 2500, 3, 2, 5, 14, 10];
    let mut dims_list: [*mut i32; 3] = [a1.as_mut_ptr(), a2.as_mut_ptr(), a3.as_mut_ptr()];
    let mut sizes: [i32; 3] = [4, 13, 12];
    i = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 3_i32 {
            print!("Dims  : [");
            n = sizes[i as usize];
            j = 0_i32;
            while j < n {
                print!("{}", *(dims_list[i as usize]).offset(j as isize));
                if j < n - 1_i32 {
                    print!(", ");
                } else {
                    println!("]");
                }
                j = j.wrapping_add(1);
                j;
            }
            optimal_matrix_chain_order(dims_list[i as usize], n);
            print!("Order : ");
            print_optimal_chain_order(0, n - 2);
            print!(
                "\nCost  : {}\n\n",
                *(*m.offset(0_isize)).offset((n - 2i32) as isize)
            );
            j = 0_i32;
            while j <= n - 2_i32 {
                free((*m.offset(j as isize)).cast::<libc::c_void>());
                j = j.wrapping_add(1);
                j;
            }
            free(m.cast::<libc::c_void>());
            j = 0_i32;
            while j <= n - 2_i32 {
                free((*s.offset(j as isize)).cast::<libc::c_void>());
                j = j.wrapping_add(1);
                j;
            }
            free(s.cast::<libc::c_void>());
            i = i.wrapping_add(1);
            i;
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
