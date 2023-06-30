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
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.add(str_size) != 0 {
            str_size = str_size.wrapping_add(1);
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {}
#[no_mangle]
pub extern "C" fn show_set(mut x: u32, mut name: *const i8) {
    unsafe {
        let mut i: i32 = 0;
        print!("{} is:", build_str_from_raw_ptr(name as *mut u8));
        i = 0_i32;
        while 1 << i <= x {
            if x & 1 << i != 0 {
                print!(" {}", i);
            }
            i = i.wrapping_add(1);
            i;
        }
        print!("{}", '\n' as i32);
    }
}

fn main_0() -> i32 {
    let mut i: i32 = 0;
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    let mut c: u32 = 0;
    a = 0;
    i = 0_i32;
    while i < 10_i32 {
        a |= 1 << i;
        i = i.wrapping_add(3);
    }
    show_set(a, (b"a\0" as *const u8).cast::<i8>());
    i = 0_i32;
    while i < 5_i32 {
        if a & 1 << i != 0 {
            println!("	{}\0 in set a", i)
        } else {
            println!("	{} not\0 in set a", i)
        };
        i = i.wrapping_add(1);
        i;
    }
    b = a;
    b |= 1 << 5_i32;
    b |= 1 << 10_i32;
    b &= !(1 << 0_i32);
    show_set(b, (b"b\0" as *const u8).cast::<i8>());
    show_set(a | b, (b"union(a, b)\0" as *const u8).cast::<i8>());
    c = a & b;
    show_set(c, (b"c = common(a, b)\0" as *const u8).cast::<i8>());
    show_set(a & !b, (b"a - b\0" as *const u8).cast::<i8>());
    show_set(b & !a, (b"b - a\0" as *const u8).cast::<i8>());
    if b & !a == 0 {
        println!("b is\0 a subset of a")
    } else {
        println!("b is not\0 a subset of a")
    };
    if c & !a == 0 {
        println!("c is\0 a subset of a")
    } else {
        println!("c is not\0 a subset of a")
    };
    if (a | b) & !(a & b) == a & !b | b & !a {
        println!(
            "union(a, b) - common(a, b) equals\0 union(a - b, b - a)"
        )
    } else {
        println!(
            "union(a, b) - common(a, b) does not equal\0 union(a - b, b - a)"
        )
    };
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
