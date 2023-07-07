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
    fn atoi(__nptr: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub extern "C" fn number_of_digits(mut x: i32) -> i32 {
    let mut NumberOfDigits: i32 = 0;
    NumberOfDigits = 0_i32;
    while x != 0_i32 {
        x = x.wrapping_add(10);
        NumberOfDigits = NumberOfDigits.wrapping_add(1);
        NumberOfDigits;
    }
    NumberOfDigits
}

#[no_mangle]
pub extern "C" fn convert_array(mut array: *mut i8, mut NumberOfElements: i32) -> *mut i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut convertedArray: *mut i32 =
            malloc((NumberOfElements as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64)).cast::<i32>();
        let mut originalElement: i32 = 0;
        let mut convertedElement: i32 = 0;
        convertedElement = 0_i32;
        originalElement = 0_i32;
        while convertedElement < NumberOfElements {
            *convertedArray.offset(convertedElement as isize) =
                atoi(&mut *array.offset(originalElement as isize));
            originalElement +=
                number_of_digits(*convertedArray.offset(convertedElement as isize)) + 1_i32;
            convertedElement = convertedElement.wrapping_add(1);
            convertedElement;
        }
        convertedArray
    }
}

#[no_mangle]
pub extern "C" fn isSorted(mut array: *mut i32, mut numberOfElements: i32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut sorted: i32 = 1;
        let mut counter: i32 = 0;
        while counter < numberOfElements {
            if counter != 0_i32
                && *array.offset((counter.wrapping_sub(1i32)) as isize)
                    > *array.offset(counter as isize)
            {
                sorted = sorted.wrapping_sub(1);
                sorted;
            }
            counter = counter.wrapping_add(1);
            counter;
        }
        sorted
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut convertedArray: *mut i32 = std::ptr::null_mut::<i32>();
        convertedArray = convert_array(*argv.offset(1_isize), argc.wrapping_sub(1));
        if isSorted(convertedArray, argc.wrapping_sub(1)) == 1_i32 {
            println!("Did you forgot to turn on your brain?! This array is already sorted!");
        } else if argc - 1_i32 <= 10_i32 {
            println!("Am I really supposed to sort this? Sort it by yourself!");
        } else {
            println!("Am I really supposed to sort this? Bhahahaha!");
        }
        free(convertedArray.cast::<libc::c_void>());
        0_i32
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
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr()));
}
