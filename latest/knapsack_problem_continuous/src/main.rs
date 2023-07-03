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
    fn qsort(__base: *mut libc::c_void, __nmemb: u64, __size: u64, __compar: __compar_fn_t);
}
pub type __compar_fn_t =
// SAFETY: machine generated unsafe code
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct item {
    pub w: f64,
    pub v: f64,
    pub name: *const i8,
}
#[no_mangle]
pub static mut items: [item; 9] = [
    {
        
        item {
            w: 3.8f64,
            v: 36_f64,
            name: (b"beef\0" as *const u8).cast::<i8>(),
        }
    },
    {
        
        item {
            w: 5.4f64,
            v: 43_f64,
            name: (b"pork\0" as *const u8).cast::<i8>(),
        }
    },
    {
        
        item {
            w: 3.6f64,
            v: 90_f64,
            name: (b"ham\0" as *const u8).cast::<i8>(),
        }
    },
    {
        
        item {
            w: 2.4f64,
            v: 45_f64,
            name: (b"greaves\0" as *const u8).cast::<i8>(),
        }
    },
    {
        
        item {
            w: 4.0f64,
            v: 30_f64,
            name: (b"flitch\0" as *const u8).cast::<i8>(),
        }
    },
    {
        
        item {
            w: 2.5f64,
            v: 56_f64,
            name: (b"brawn\0" as *const u8).cast::<i8>(),
        }
    },
    {
        
        item {
            w: 3.7f64,
            v: 67_f64,
            name: (b"welt\0" as *const u8).cast::<i8>(),
        }
    },
    {
        
        item {
            w: 3.0f64,
            v: 95_f64,
            name: (b"salami\0" as *const u8).cast::<i8>(),
        }
    },
    {
        
        item {
            w: 5.9f64,
            v: 98_f64,
            name: (b"sausage\0" as *const u8).cast::<i8>(),
        }
    },
];
#[no_mangle]
pub extern "C" fn item_cmp(mut aa: *const libc::c_void, mut bb: *const libc::c_void) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut a: *const item = aa.cast::<item>();
        let mut b: *const item = bb.cast::<item>();
        let mut ua: f64 = (*a).v / (*a).w;
        let mut ub: f64 = (*b).v / (*b).w;
        if ua < ub { -1_i32 } else { i32::from(ua > ub) }
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut it: *mut item = std::ptr::null_mut::<item>();
        let mut space: f64 = 15_f64;
        qsort(
            items.as_mut_ptr().cast::<libc::c_void>(),
            9,
            ::core::mem::size_of::<item>() as u64,
// SAFETY: machine generated unsafe code
            Some(item_cmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32),
        );
        it = items.as_mut_ptr().offset(9_isize);
        loop {
            let fresh0 = it;
            it = it.offset(-1);
            if !(fresh0.offset_from(items.as_mut_ptr()) as i64 != 0 && space > f64::from(0_i32)) {
                break;
            }
            if space >= (*it).w {
                println!(
                    "take all {}",
                    build_str_from_raw_ptr((*it).name as *mut u8)
                );
            } else {
                println!(
                    "take {}kg of {} kg of {}",
                    space,
                    (*it).w,
                    build_str_from_raw_ptr((*it).name as *mut u8)
                );
            }
            space -= (*it).w;
        }
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
