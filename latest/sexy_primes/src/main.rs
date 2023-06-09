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
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
}
#[no_mangle]
pub extern "C" fn sieve(mut c: *mut u8, mut limit: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut p: i32 = 3;
        let mut p2: i32 = 0;
        *c.offset(0_isize) = 1;
        *c.offset(1_isize) = 1;
        loop {
            p2 = p.wrapping_mul(p);
            if p2 >= limit {
                break;
            }
            i = p2;
            while i < limit {
                *c.offset(i as isize) = 1;
                i += 2_i32 * p;
            }
            loop {
                p = p.wrapping_add(2);
                if *c.offset(p as isize) == 0 {
                    break;
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn printHelper(mut cat: *const i8, mut len: i32, mut lim: i32, mut n: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut sp: *const i8 = if strcmp(cat, (b"unsexy primes\0" as *const u8).cast::<i8>()) != 0 {
            (b"sexy prime \0" as *const u8).cast::<i8>()
        } else {
            (b"\0" as *const u8).cast::<i8>()
        };
        let mut verb: *const i8 = if len == 1 {
            (b"is\0" as *const u8).cast::<i8>()
        } else {
            (b"are\0" as *const u8).cast::<i8>()
        };
        println!(
            "Number of {}{} less than {} = {}",
            build_str_from_raw_ptr(sp as *mut u8),
            build_str_from_raw_ptr(cat as *mut u8),
            lim,
            len
        );
        println!(
            "The last {} {}:",
            n,
            build_str_from_raw_ptr(verb as *mut u8)
        );
    }
}

#[no_mangle]
pub extern "C" fn printArray(mut a: *mut i32, mut len: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        print!("[");
        i = 0_i32;
        while i < len {
            print!("{} ", *a.offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
        print!("\x08]");
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut ix: i32 = 0;
        let mut _n: i32 = 0;
        let mut lim: i32 = 1000035;
        let mut pairs: i32 = 0;
        let mut trips: i32 = 0;
        let mut quads: i32 = 0;
        let mut quins: i32 = 0;
        let mut unsexy: i32 = 2;
        let mut pr: i32 = 0;
        let mut tr: i32 = 0;
        let mut qd: i32 = 0;
        let mut qn: i32 = 0;
        let mut un: i32 = 2;
        let mut lpr: i32 = 5;
        let mut ltr: i32 = 5;
        let mut lqd: i32 = 5;
        let mut lqn: i32 = 5;
        let mut lun: i32 = 10;
        let mut last_pr: [[i32; 2]; 5] = [[0; 2]; 5];
        let mut last_tr: [[i32; 3]; 5] = [[0; 3]; 5];
        let mut last_qd: [[i32; 4]; 5] = [[0; 4]; 5];
        let mut last_qn: [[i32; 5]; 5] = [[0; 5]; 5];
        let mut last_un: [i32; 10] = [0; 10];
        let mut sv: *mut u8 = calloc(
            (lim.wrapping_sub(1i32)) as u64,
            ::core::mem::size_of::<u8>() as u64,
        ).cast::<u8>();
        setlocale(1, (b"\0" as *const u8).cast::<i8>());
        sieve(sv, lim);
        i = 3_i32;
        while i < lim {
            if i > 5_i32
                && i < lim - 6_i32
                && *sv.offset(i as isize) == 0
                && i32::from(*sv.offset((i.wrapping_sub(6i32)) as isize)) != 0_i32
                && i32::from(*sv.offset((i.wrapping_add(6i32)) as isize)) != 0_i32
            {
                unsexy = unsexy.wrapping_add(1);
                unsexy;
            } else if i < lim - 6_i32
                && *sv.offset(i as isize) == 0
                && *sv.offset((i.wrapping_add(6i32)) as isize) == 0
            {
                pairs = pairs.wrapping_add(1);
                pairs;
                if i < lim - 12_i32 && *sv.offset((i.wrapping_add(12i32)) as isize) == 0 {
                    trips = trips.wrapping_add(1);
                    trips;
                    if i < lim - 18_i32 && *sv.offset((i.wrapping_add(18i32)) as isize) == 0 {
                        quads = quads.wrapping_add(1);
                        quads;
                        if i < lim - 24_i32 && *sv.offset((i.wrapping_add(24i32)) as isize) == 0 {
                            quins = quins.wrapping_add(1);
                            quins;
                        }
                    }
                }
            }
            i = i.wrapping_add(2);
        }
        if pairs < lpr {
            lpr = pairs;
        }
        if trips < ltr {
            ltr = trips;
        }
        if quads < lqd {
            lqd = quads;
        }
        if quins < lqn {
            lqn = quins;
        }
        if unsexy < lun {
            lun = unsexy;
        }
        i = 3_i32;
        while i < lim {
            if i > 5_i32
                && i < lim - 6_i32
                && *sv.offset(i as isize) == 0
                && i32::from(*sv.offset((i.wrapping_sub(6i32)) as isize)) != 0_i32
                && i32::from(*sv.offset((i.wrapping_add(6i32)) as isize)) != 0_i32
            {
                un = un.wrapping_add(1);
                un;
                if un > unsexy - lun {
                    last_un[(un + lun - 1_i32 - unsexy) as usize] = i;
                }
            } else if i < lim - 6_i32
                && *sv.offset(i as isize) == 0
                && *sv.offset((i.wrapping_add(6i32)) as isize) == 0
            {
                pr = pr.wrapping_add(1);
                pr;
                if pr > pairs - lpr {
                    ix = pr + lpr - 1_i32 - pairs;
                    last_pr[ix as usize][0_usize] = i;
                    last_pr[ix as usize][1_usize] = i.wrapping_add(6);
                }
                if i < lim - 12_i32 && *sv.offset((i.wrapping_add(12i32)) as isize) == 0 {
                    tr = tr.wrapping_add(1);
                    tr;
                    if tr > trips - ltr {
                        ix = tr + ltr - 1_i32 - trips;
                        last_tr[ix as usize][0_usize] = i;
                        last_tr[ix as usize][1_usize] = i.wrapping_add(6);
                        last_tr[ix as usize][2_usize] = i.wrapping_add(12);
                    }
                    if i < lim - 18_i32 && *sv.offset((i.wrapping_add(18i32)) as isize) == 0 {
                        qd = qd.wrapping_add(1);
                        qd;
                        if qd > quads - lqd {
                            ix = qd + lqd - 1_i32 - quads;
                            last_qd[ix as usize][0_usize] = i;
                            last_qd[ix as usize][1_usize] = i.wrapping_add(6);
                            last_qd[ix as usize][2_usize] = i.wrapping_add(12);
                            last_qd[ix as usize][3_usize] = i.wrapping_add(18);
                        }
                        if i < lim - 24_i32 && *sv.offset((i.wrapping_add(24i32)) as isize) == 0 {
                            qn = qn.wrapping_add(1);
                            qn;
                            if qn > quins - lqn {
                                ix = qn + lqn - 1_i32 - quins;
                                last_qn[ix as usize][0_usize] = i;
                                last_qn[ix as usize][1_usize] = i.wrapping_add(6);
                                last_qn[ix as usize][2_usize] = i.wrapping_add(12);
                                last_qn[ix as usize][3_usize] = i.wrapping_add(18);
                                last_qn[ix as usize][4_usize] = i.wrapping_add(24);
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(2);
        }
        printHelper((b"pairs\0" as *const u8).cast::<i8>(), pairs, lim, lpr);
        print!("  [");
        i = 0_i32;
        while i < lpr {
            printArray((last_pr[i as usize]).as_mut_ptr(), 2);
            print!("\x08] ");
            i = i.wrapping_add(1);
            i;
        }
        print!("\x08]\n\n");
        printHelper((b"triplets\0" as *const u8).cast::<i8>(), trips, lim, ltr);
        print!("  [");
        i = 0_i32;
        while i < ltr {
            printArray((last_tr[i as usize]).as_mut_ptr(), 3);
            print!("\x08] ");
            i = i.wrapping_add(1);
            i;
        }
        print!("\x08]\n\n");
        printHelper((b"quadruplets\0" as *const u8).cast::<i8>(), quads, lim, lqd);
        print!("  [");
        i = 0_i32;
        while i < lqd {
            printArray((last_qd[i as usize]).as_mut_ptr(), 4);
            print!("\x08] ");
            i = i.wrapping_add(1);
            i;
        }
        print!("\x08]\n\n");
        printHelper((b"quintuplets\0" as *const u8).cast::<i8>(), quins, lim, lqn);
        print!("  [");
        i = 0_i32;
        while i < lqn {
            printArray((last_qn[i as usize]).as_mut_ptr(), 5);
            print!("\x08] ");
            i = i.wrapping_add(1);
            i;
        }
        print!("\x08]\n\n");
        printHelper(
            (b"unsexy primes\0" as *const u8).cast::<i8>(),
            unsexy,
            lim,
            lun,
        );
        print!("  [");
        printArray(last_un.as_mut_ptr(), lun);
        println!("\x08]");
        free(sv.cast::<libc::c_void>());
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
