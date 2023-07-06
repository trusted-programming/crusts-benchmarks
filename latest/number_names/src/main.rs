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
            str_size += 1;
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    fn strlen(_: *const i8) -> u64;
}
#[no_mangle]
pub static mut ones: [*const i8; 20] = [
    0 as *const i8,
    (b"one\0" as *const u8).cast::<i8>(),
    (b"two\0" as *const u8).cast::<i8>(),
    (b"three\0" as *const u8).cast::<i8>(),
    (b"four\0" as *const u8).cast::<i8>(),
    (b"five\0" as *const u8).cast::<i8>(),
    (b"six\0" as *const u8).cast::<i8>(),
    (b"seven\0" as *const u8).cast::<i8>(),
    (b"eight\0" as *const u8).cast::<i8>(),
    (b"nine\0" as *const u8).cast::<i8>(),
    (b"ten\0" as *const u8).cast::<i8>(),
    (b"eleven\0" as *const u8).cast::<i8>(),
    (b"twelve\0" as *const u8).cast::<i8>(),
    (b"thirteen\0" as *const u8).cast::<i8>(),
    (b"fourteen\0" as *const u8).cast::<i8>(),
    (b"fifteen\0" as *const u8).cast::<i8>(),
    (b"sixteen\0" as *const u8).cast::<i8>(),
    (b"seventeen\0" as *const u8).cast::<i8>(),
    (b"eighteen\0" as *const u8).cast::<i8>(),
    (b"nineteen\0" as *const u8).cast::<i8>(),
];
#[no_mangle]
pub static mut tens: [*const i8; 10] = [
    0 as *const i8,
    (b"ten\0" as *const u8).cast::<i8>(),
    (b"twenty\0" as *const u8).cast::<i8>(),
    (b"thirty\0" as *const u8).cast::<i8>(),
    (b"forty\0" as *const u8).cast::<i8>(),
    (b"fifty\0" as *const u8).cast::<i8>(),
    (b"sixty\0" as *const u8).cast::<i8>(),
    (b"seventy\0" as *const u8).cast::<i8>(),
    (b"eighty\0" as *const u8).cast::<i8>(),
    (b"ninety\0" as *const u8).cast::<i8>(),
];
#[no_mangle]
pub static mut llions: [*const i8; 5] = [
    0 as *const i8,
    (b"thousand\0" as *const u8).cast::<i8>(),
    (b"million\0" as *const u8).cast::<i8>(),
    (b"billion\0" as *const u8).cast::<i8>(),
    (b"trillion\0" as *const u8).cast::<i8>(),
];
#[no_mangle]
pub static mut maxillion: i32 = 0_i32;
#[no_mangle]
pub extern "C" fn say_hundred(
    mut s: *const i8,
    mut len: i32,
    mut depth: i32,
    mut has_lead: i32,
) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut c: [i32; 3] = [0; 3];
        let mut i: i32 = 0;
        i = -3_i32;
        while i < 0_i32 {
            if len + i >= 0_i32 {
                c[(i + 3i32) as usize] = i32::from(*s.offset((len + i) as isize)) - '0' as i32;
            } else {
                c[(i + 3i32) as usize] = 0_i32;
            }
            i += 1_i32;
            i;
        }
        if c[0_usize] + c[1_usize] + c[2_usize] == 0_i32 {
            return 0_i32;
        }
        if c[0_usize] != 0_i32 {
            print!(
                "{} hundred",
                build_str_from_raw_ptr(ones[c[0_usize] as usize] as *mut u8)
            );
            has_lead = 1_i32;
        }
        if has_lead != 0_i32 && (c[1_usize] != 0_i32 || c[2_usize] != 0_i32) {
            if (depth == 0_i32 || c[0_usize] != 0_i32) && (c[0_usize] == 0_i32 || c[1_usize] == 0_i32) {
                print!("and ")
            } else if c[0_usize] != 0_i32 {
                print!(" ")
            } else {
                print!("")
            };
        }
        if c[1_usize] < 2_i32 {
            if c[1_usize] != 0_i32 || c[2_usize] != 0_i32 {
                print!(
                    "{}",
                    build_str_from_raw_ptr(
                        ones[(c[1_usize] * 10_i32 + c[2_usize]) as usize] as *mut u8
                    )
                );
            }
        } else {
            if c[1_usize] != 0_i32 {
                print!(
                    "{}",
                    build_str_from_raw_ptr(tens[c[1_usize] as usize] as *mut u8)
                );
                if c[2_usize] != 0_i32 {
                    print!("{}", '-' as i32);
                }
            }
            if c[2_usize] != 0_i32 {
                print!(
                    "{}",
                    build_str_from_raw_ptr(ones[c[2_usize] as usize] as *mut u8)
                );
            }
        }
        1_i32
    }
}

#[no_mangle]
pub extern "C" fn say_maxillion(
    mut s: *const i8,
    mut len: i32,
    mut depth: i32,
    mut has_lead: i32,
) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut n: i32 = len / 3;
        let mut r: i32 = len % 3;
        if r == 0_i32 {
            n -= 1_i32;
            n;
            r = 3_i32;
        }
        let mut e: *const i8 = s.offset(r as isize);
        loop {
            if say_hundred(s, r, n, has_lead) != 0_i32 && n != 0_i32 {
                has_lead = 1_i32;
                print!(" {}", build_str_from_raw_ptr(llions[n as usize] as *mut u8));
                if depth == 0_i32 {
                    print!(", ");
                } else {
                    print!(" ");
                }
            }
            s = e;
            e = e.offset(3_isize);
            r = 3_i32;
            let fresh0 = n;
            n -= 1_i32;
            if fresh0 == 0_i32 {
                break;
            }
        }
        1_i32
    }
}

#[no_mangle]
pub extern "C" fn say_number(mut s: *const i8) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut n: i32 = 0;
        let mut r: i32 = 0;
        let mut end: *const i8 = std::ptr::null::<i8>();
        let mut has_lead: i32 = 0;
        let mut current_block: u64;
        let mut len: i32 = 0;
        let mut i: i32 = 0;
        let mut got_sign: i32 = 0;
        while i32::from(*s) == ' ' as i32 {
            s = s.offset(1);
            s;
        }
        if i32::from(*s) < '0' as i32 || i32::from(*s) > '9' as i32 {
            if i32::from(*s) == '-' as i32 {
                got_sign = -1_i32;
                current_block = 15619007995458559411;
            } else if i32::from(*s) == '+' as i32 {
                got_sign = 1_i32;
                current_block = 15619007995458559411;
            } else {
                current_block = 1693057788147685417;
            }
            match current_block {
                1693057788147685417 => {}
                _ => {
                    s = s.offset(1);
                    s;
                    current_block = 14523784380283086299;
                }
            }
        } else {
            got_sign = 1_i32;
            current_block = 14523784380283086299;
        }
        match current_block {
            14523784380283086299 => {
                while i32::from(*s) == '0' as i32 {
                    s = s.offset(1);
                    s;
                    if i32::from(*s) == '\0' as i32 {
                        println!("zero");
                        return;
                    }
                }
                len = strlen(s) as i32;
                if len != 0_i32 {
                    i = 0_i32;
                    while i < len {
                        if i32::from(*s.offset(i as isize)) < '0' as i32
                            || i32::from(*s.offset(i as isize)) > '9' as i32
                        {
                            print!("(not a number)");
                            return;
                        }
                        i += 1_i32;
                        i;
                    }
                    if got_sign == -1_i32 {
                        print!("minus ");
                    }
                    n = len / maxillion;
                    r = len % maxillion;
                    if r == 0_i32 {
                        r = maxillion;
                        n -= 1_i32;
                        n;
                    }
                    end = s.offset(len as isize).offset(-((n * maxillion) as isize));
                    has_lead = 0_i32;
                    loop {
                        has_lead = say_maxillion(s, r, n, has_lead);
                        if has_lead != 0_i32 {
                            i = 0_i32;
                            while i < n {
                                print!(
                                    " {}",
                                    build_str_from_raw_ptr(
                                        llions[(maxillion / 3i32) as usize] as *mut u8
                                    )
                                );
                                i += 1_i32;
                                i;
                            }
                            if n != 0_i32 {
                                print!(", ");
                            }
                        }
                        n -= 1_i32;
                        n;
                        r = maxillion;
                        s = end;
                        end = end.offset(r as isize);
                        if n < 0_i32 {
                            break;
                        }
                    }
                    println!();
                    return;
                }
            }
            _ => {}
        }
        println!("not a number");
    }
}

fn main_0() -> i32 {
    say_number((b"-42\0" as *const u8).cast::<i8>());
    say_number((b"1984\0" as *const u8).cast::<i8>());
    say_number((b"10000\0" as *const u8).cast::<i8>());
    say_number((b"1024\0" as *const u8).cast::<i8>());
    say_number((b"1001001001001\0" as *const u8).cast::<i8>());
    say_number(
        (b"123456789012345678901234567890123456789012345678900000001\0" as *const u8).cast::<i8>(),
    );
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}

extern "C" fn run_static_initializers() {
// SAFETY: machine generated unsafe code
    unsafe {
        maxillion = (::core::mem::size_of::<[*const i8; 5]>() as u64)
            .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
            .wrapping_mul(3)
            .wrapping_sub(3) as i32;
    }
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
// SAFETY: machine generated unsafe code
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
