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
    fn scanf(_: *const i8, _: ...) -> i32;
}
fn main_0() -> i32 {
    let mut curr: [[i32; 5]; 5] = [[0; 5]; 5];
    let mut max_claim: [[i32; 5]; 5] = [[0; 5]; 5];
    let mut avl: [i32; 5] = [0; 5];
    let mut alloc: [i32; 5] = [0; 5];
    let mut max_res: [i32; 5] = [0; 5];
    let mut running: [i32; 5] = [0; 5];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut exec: i32 = 0;
    let mut r: i32 = 0;
    let mut p: i32 = 0;
    let mut count: i32 = 0;
    let mut safe: bool = 0 != 0;
    print!("\nEnter the number of resources: ");
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        scanf((b"%d\0" as *const u8).cast::<i8>(), &mut r as *mut i32);
    }
    print!("\nEnter the number of processes: ");
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        scanf((b"%d\0" as *const u8).cast::<i8>(), &mut p as *mut i32);
    }
    i = 0_i32;
    while i < p {
        running[i as usize] = 1_i32;
        count = count.wrapping_add(1);
        count;
        i = i.wrapping_add(1);
        i;
    }
    print!("\nEnter Claim Vector: ");
    i = 0_i32;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        while i < r {
            scanf(
                (b"%d\0" as *const u8).cast::<i8>(),
                &mut *max_res.as_mut_ptr().offset(i as isize) as *mut i32,
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    print!("\nEnter Allocated Resource Table: ");
    i = 0_i32;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        while i < p {
            j = 0_i32;
            while j < r {
                scanf(
                    (b"%d\0" as *const u8).cast::<i8>(),
                    &mut *(*curr.as_mut_ptr().offset(i as isize))
                        .as_mut_ptr()
                        .offset(j as isize) as *mut i32,
                );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    print!("\nEnter Maximum Claim table: ");
    i = 0_i32;
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        while i < p {
            j = 0_i32;
            while j < r {
                scanf(
                    (b"%d\0" as *const u8).cast::<i8>(),
                    &mut *(*max_claim.as_mut_ptr().offset(i as isize))
                        .as_mut_ptr()
                        .offset(j as isize) as *mut i32,
                );
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    print!("\nThe Claim Vector is: ");
    i = 0_i32;
    while i < r {
        print!("{} ", max_res[i as usize]);
        i = i.wrapping_add(1);
        i;
    }
    print!("\nThe Allocated Resource Table:\n");
    i = 0_i32;
    while i < p {
        j = 0_i32;
        while j < r {
            print!("	{}", curr[i as usize][j as usize]);
            j = j.wrapping_add(1);
            j;
        }
        println!();
        i = i.wrapping_add(1);
        i;
    }
    print!("\nThe Maximum Claim Table:\n");
    i = 0_i32;
    while i < p {
        j = 0_i32;
        while j < r {
            print!("	{}", max_claim[i as usize][j as usize]);
            j = j.wrapping_add(1);
            j;
        }
        println!();
        i = i.wrapping_add(1);
        i;
    }
    i = 0_i32;
    while i < p {
        j = 0_i32;
        while j < r {
            alloc[j as usize] += curr[i as usize][j as usize];
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    print!("\nAllocated resources: ");
    i = 0_i32;
    while i < r {
        print!("{} ", alloc[i as usize]);
        i = i.wrapping_add(1);
        i;
    }
    i = 0_i32;
    while i < r {
        avl[i as usize] = max_res[i as usize] - alloc[i as usize];
        i = i.wrapping_add(1);
        i;
    }
    print!("\nAvailable resources: ");
    i = 0_i32;
    while i < r {
        print!("{} ", avl[i as usize]);
        i = i.wrapping_add(1);
        i;
    }
    println!();
    while count != 0_i32 {
        safe = 0_i32 != 0_i32;
        i = 0_i32;
        while i < p {
            if running[i as usize] != 0_i32 {
                exec = 1_i32;
                j = 0_i32;
                while j < r {
                    if max_claim[i as usize][j as usize] - curr[i as usize][j as usize]
                        > avl[j as usize]
                    {
                        exec = 0_i32;
                        break;
                    } else {
                        j = j.wrapping_add(1);
                        j;
                    }
                }
                if exec != 0_i32 {
                    print!("\nProcess{} is executing.\n", i + 1_i32);
                    running[i as usize] = 0_i32;
                    count = count.wrapping_sub(1);
                    count;
                    safe = 1_i32 != 0_i32;
                    j = 0_i32;
                    while j < r {
                        avl[j as usize] += curr[i as usize][j as usize];
                        j = j.wrapping_add(1);
                        j;
                    }
                    break;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        if !safe {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
            print!("\nThe processes are in unsafe state.");
            break;
        } else {
            if safe {
                print!("\nThe process is in safe state.");
            }
            print!("\nAvailable vector: ");
            i = 0_i32;
            while i < r {
                print!("{} ", avl[i as usize]);
                i = i.wrapping_add(1);
                i;
            }
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
