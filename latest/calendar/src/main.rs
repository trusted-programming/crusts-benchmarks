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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    fn atoi(__nptr: *const i8) -> i32;
    fn exit(_: i32) -> !;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
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
pub struct months {
    pub name: *const i8,
    pub days: i32,
    pub start_wday: i32,
    pub at: i32,
}
#[no_mangle]
pub static mut width: i32 = 80_i32;
#[no_mangle]
pub static mut year: i32 = 1_969_i32;
#[no_mangle]
pub static mut cols: i32 = 0_i32;
#[no_mangle]
pub static mut lead: i32 = 0_i32;
#[no_mangle]
pub static mut gap: i32 = 0_i32;
#[no_mangle]
pub static mut wdays: [*const i8; 7] = [
    (b"Su\0" as *const u8).cast::<i8>(),
    (b"Mo\0" as *const u8).cast::<i8>(),
    (b"Tu\0" as *const u8).cast::<i8>(),
    (b"We\0" as *const u8).cast::<i8>(),
    (b"Th\0" as *const u8).cast::<i8>(),
    (b"Fr\0" as *const u8).cast::<i8>(),
    (b"Sa\0" as *const u8).cast::<i8>(),
];
#[no_mangle]
pub static mut months: [months; 12] = [
    {
        
        months {
            name: (b"January\0" as *const u8).cast::<i8>(),
            days: 31,
            start_wday: 0,
            at: 0,
        }
    },
    {
        
        months {
            name: (b"February\0" as *const u8).cast::<i8>(),
            days: 28,
            start_wday: 0,
            at: 0,
        }
    },
    {
        
        months {
            name: (b"March\0" as *const u8).cast::<i8>(),
            days: 31,
            start_wday: 0,
            at: 0,
        }
    },
    {
        
        months {
            name: (b"April\0" as *const u8).cast::<i8>(),
            days: 30,
            start_wday: 0,
            at: 0,
        }
    },
    {
        
        months {
            name: (b"May\0" as *const u8).cast::<i8>(),
            days: 31,
            start_wday: 0,
            at: 0,
        }
    },
    {
        
        months {
            name: (b"June\0" as *const u8).cast::<i8>(),
            days: 30,
            start_wday: 0,
            at: 0,
        }
    },
    {
        
        months {
            name: (b"July\0" as *const u8).cast::<i8>(),
            days: 31,
            start_wday: 0,
            at: 0,
        }
    },
    {
        
        months {
            name: (b"August\0" as *const u8).cast::<i8>(),
            days: 31,
            start_wday: 0,
            at: 0,
        }
    },
    {
        
        months {
            name: (b"September\0" as *const u8).cast::<i8>(),
            days: 30,
            start_wday: 0,
            at: 0,
        }
    },
    {
        
        months {
            name: (b"October\0" as *const u8).cast::<i8>(),
            days: 31,
            start_wday: 0,
            at: 0,
        }
    },
    {
        
        months {
            name: (b"November\0" as *const u8).cast::<i8>(),
            days: 30,
            start_wday: 0,
            at: 0,
        }
    },
    {
        
        months {
            name: (b"December\0" as *const u8).cast::<i8>(),
            days: 31,
            start_wday: 0,
            at: 0,
        }
    },
];
#[no_mangle]
pub extern "C" fn space(mut n: i32) {
    loop {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if fresh0 <= 0_i32 {
            break;
        }
        print!("{}", ' ' as i32);
    }
}

#[no_mangle]
pub extern "C" fn init_months() {
    let mut i: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        if year % 4_i32 == 0_i32 && year % 100_i32 != 0_i32 || year % 400_i32 == 0_i32 {
            months[1_usize].days = 29_i32;
        }
        year = year.wrapping_sub(1);
        year;
        months[0_usize].start_wday = (year * 365_i32 + year / 4_i32 - year / 100_i32 + year / 400_i32 + 1_i32) % 7_i32;
    }
    i = 1_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while i < 12_i32 {
            months[i as usize].start_wday = (months[(i.wrapping_sub(1i32)) as usize].start_wday
                + months[(i.wrapping_sub(1i32)) as usize].days)
                % 7_i32;
            i = i.wrapping_add(1);
            i;
        }
        cols = (width.wrapping_add(2)) / 22_i32;
        while 12_i32 % cols != 0_i32 {
            cols = cols.wrapping_sub(1);
            cols;
        }
        gap = if cols - 1_i32 != 0_i32 {
            (width - 20_i32 * cols) / (cols.wrapping_sub(1))
        } else {
            0_i32
        };
        if gap > 4_i32 {
            gap = 4_i32;
        }
        lead = (width - (gap.wrapping_add(20)) * cols + gap.wrapping_add(1)) / 2_i32;
        year = year.wrapping_add(1);
        year;
    }
}

#[no_mangle]
pub extern "C" fn print_row(mut row: i32) {
    let mut c: i32 = 0;
    let mut i: i32 = 0;
// SAFETY: machine generated unsafe code
    unsafe {
        let mut from: i32 = row.wrapping_mul(cols);
        let mut to: i32 = from.wrapping_add(cols);
        space(lead);
        c = from;
        while c < to {
            i = strlen(months[c as usize].name) as i32;
            space((20 - i) / 2);
            print!(
                "{}",
                build_str_from_raw_ptr(months[c as usize].name as *mut u8)
            );
            space(20 - i - (20 - i) / 2 + (if c == to.wrapping_sub(1) { 0 } else { gap }));
            c = c.wrapping_add(1);
            c;
        }
        print!("{}", '\n' as i32);
        space(lead);
        c = from;
        while c < to {
            i = 0_i32;
            while i < 7_i32 {
                if i == 6_i32 {
                    print!(
                        "{}\0",
                        build_str_from_raw_ptr(wdays[i as usize] as *mut u8)
                    )
                } else {
                    print!(
                        "{} \0",
                        build_str_from_raw_ptr(wdays[i as usize] as *mut u8)
                    )
                };
                i = i.wrapping_add(1);
                i;
            }
            if c < to.wrapping_sub(1) {
                space(gap);
            } else {
                print!("{}", '\n' as i32);
            }
            c = c.wrapping_add(1);
            c;
        }
        loop {
            c = from;
            while c < to {
                if months[c as usize].at < months[c as usize].days {
                    break;
                }
                c = c.wrapping_add(1);
                c;
            }
            if c == to {
                break;
            }
            space(lead);
            c = from;
            while c < to {
                i = 0_i32;
                while i < months[c as usize].start_wday {
                    space(3);
                    i = i.wrapping_add(1);
                    i;
                }
                loop {
                    let fresh1 = i;
                    i = i.wrapping_add(1);
                    if !(fresh1 < 7_i32 && months[c as usize].at < months[c as usize].days) {
                        break;
                    }
                    months[c as usize].at += 1_i32;
                    print!("{:2}", months[c as usize].at);
                    if i < 7_i32 || c < to.wrapping_sub(1) {
                        print!("{}", ' ' as i32);
                    }
                }
                loop {
                    let fresh2 = i;
                    i = i.wrapping_add(1);
                    if !(fresh2 <= 7_i32 && c < to.wrapping_sub(1)) {
                        break;
                    }
                    space(3);
                }
                if c < to.wrapping_sub(1) {
                    space(gap.wrapping_sub(1));
                }
                months[c as usize].start_wday = 0_i32;
                c = c.wrapping_add(1);
                c;
            }
            print!("{}", '\n' as i32);
        }
    }
    print!("{}", '\n' as i32);
}

#[no_mangle]
pub extern "C" fn print_year() {
    let mut row: i32 = 0;
    let mut buf: [i8; 32] = [0; 32];
// SAFETY: machine generated unsafe code
    unsafe {
        sprintf(buf.as_mut_ptr(), (b"%d\0" as *const u8).cast::<i8>(), year);
        space(
            (width as u64)
                .wrapping_sub(strlen(buf.as_mut_ptr()))
                .wrapping_div(2) as i32,
        );
        print!(
            "{}\n\n",
            build_str_from_raw_ptr(buf.as_mut_ptr().cast::<u8>())
        );
    }
    row = 0_i32;
// SAFETY: machine generated unsafe code
    unsafe {
        while row * cols < 12_i32 {
            print_row(row);
            row = row.wrapping_add(1);
            row;
        }
    }
}

fn main_0(mut c: i32, mut v: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut current_block: u64;
        let mut i: i32 = 0;
        let mut year_set: i32 = 0;
        i = 1_i32;
        loop {
            if i >= c {
                current_block = 1394248824506584008;
                break;
            }
            if strcmp(*v.offset(i as isize), (b"-w\0" as *const u8).cast::<i8>()) == 0_i32 {
                i = i.wrapping_add(1);
                if i == c || {
                    width = atoi(*v.offset(i as isize));
                    width < 20_i32
                } {
                    current_block = 2002819764153645144;
                    break;
                }
            } else {
                if year_set != 0_i32 {
                    current_block = 2002819764153645144;
                    break;
                }
                if sscanf(
                    *v.offset(i as isize),
                    (b"%d\0" as *const u8).cast::<i8>(),
                    &mut year as *mut i32,
                ) == 0_i32
                    || year <= 0_i32
                {
                    year = 1_969_i32;
                }
                year_set = 1_i32;
            }
            i = i.wrapping_add(1);
            i;
        }
        match current_block {
            2002819764153645144 => {
                fprintf(
                    stderr,
                    (b"bad args\nUsage: %s year [-w width (>= 20)]\n\0" as *const u8).cast::<i8>(),
                    *v.offset(0_isize),
                );
                exit(1);
            }
            _ => {
                init_months();
                print_year();
                0_i32
            }
        }
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
// SAFETY: machine generated unsafe code
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr()),
        );
    }
}
