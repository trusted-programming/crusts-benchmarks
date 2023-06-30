#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

extern "C" {}
extern "C" fn callbackFunction(mut location: i32, mut value: i32) {
    println!("array[{}] = {}", location, value);
}

#[no_mangle]
pub extern "C" fn map(
    mut array: *mut i32,
    mut len: i32,
    mut callback: Option<unsafe extern "C" fn(i32, i32) -> ()>,
) {
    unsafe {
        let mut i: i32 = 0;
        i = 0_i32;
        while i < len {
            callback.expect("non-null function pointer")(i, *array.offset(i as isize));
            i = i.wrapping_add(1);
            i;
        }
    }
}

fn main_0() -> i32 {
    let mut array: [i32; 4] = [1, 2, 3, 4];
    map(
        array.as_mut_ptr(),
        4,
        Some(callbackFunction as unsafe extern "C" fn(i32, i32) -> ()),
    );
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
