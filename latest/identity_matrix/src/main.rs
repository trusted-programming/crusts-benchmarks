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
    fn exit(_: i32) -> !;
    fn abort() -> !;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
}
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        if argc < 2_i32 {
            println!("usage: identitymatrix <number of rows>");
            exit(1);
        }
        let mut rowsize: i32 = atoi(*argv.offset(1_isize));
        if rowsize < 0_i32 {
            println!("Dimensions of matrix cannot be negative");
            exit(1);
        }
        let mut numElements: i32 = rowsize * rowsize;
        if numElements < rowsize {
            println!(
                "Squaring {} caused result to overflow to {}.",
                rowsize, numElements
            );
            abort();
        }
        let mut matrix: *mut *mut i32 = calloc(
            numElements as u64,
            ::core::mem::size_of::<*mut i32>() as u64,
        ).cast::<*mut i32>();
        if matrix.is_null() {
            println!(
                "Failed to allocate {} elements of {} bytes each",
                numElements,
                ::core::mem::size_of::<*mut i32>() as u64
            );
            abort();
        }
        let mut row: u32 = 0;
        while row < rowsize as u32 {
            let fresh0 = &mut (*matrix.offset(row as isize));
            *fresh0 = calloc(numElements as u64, ::core::mem::size_of::<i32>() as u64).cast::<i32>();
            if (*matrix.offset(row as isize)).is_null() {
                println!(
                    "Failed to allocate {} elements of {} bytes each",
                    numElements,
                    ::core::mem::size_of::<i32>() as u64
                );
                abort();
            };
            *(*matrix.offset(row as isize)).offset(row as isize) = 1_i32;
            row = row.wrapping_add(1);
            row;
        }
        println!("Matrix is: ");
        let mut row_0: u32 = 0;
        while row_0 < rowsize as u32 {
            let mut column: u32 = 0;
            while column < rowsize as u32 {
                print!(
                    "{} ",
                    *(*matrix.offset(row_0 as isize)).offset(column as isize)
                );
                column = column.wrapping_add(1);
                column;
            }
            println!();
            row_0 = row_0.wrapping_add(1);
            row_0;
        }
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
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
// SAFETY: machine generated unsafe code
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr()),
        );
    }
}
