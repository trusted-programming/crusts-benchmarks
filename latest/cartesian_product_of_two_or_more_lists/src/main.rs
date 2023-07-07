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
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strtok(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn atoi(__nptr: *const i8) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
}
#[no_mangle]
pub extern "C" fn cartesianProduct(
    mut sets: *mut *mut i32,
    mut setLengths: *mut i32,
    mut currentSet: *mut i32,
    mut numSets: i32,
    mut times: i32,
) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        if times == numSets {
            print!("(");
            i = 0_i32;
            while i < times {
                print!("{},", *currentSet.offset(i as isize));
                i = i.wrapping_add(1);
                i;
            }
            print!("\x08),");
        } else {
            j = 0_i32;
            while j < *setLengths.offset(times as isize) {
                *currentSet.offset(times as isize) =
                    *(*sets.offset(times as isize)).offset(j as isize);
                cartesianProduct(sets, setLengths, currentSet, numSets, times.wrapping_add(1));
                j = j.wrapping_add(1);
                j;
            }
        };
    }
}

#[no_mangle]
pub extern "C" fn printSets(mut sets: *mut *mut i32, mut setLengths: *mut i32, mut numSets: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        print!("\nNumber of sets : {}", numSets);
        i = 0_i32;
        while i < numSets.wrapping_add(1) {
            print!("\nSet {} : ", i + 1_i32);
            j = 0_i32;
            while j < *setLengths.offset(i as isize) {
                print!(" {} ", *(*sets.offset(i as isize)).offset(j as isize));
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
}

#[no_mangle]
pub extern "C" fn processInputString(mut str: *mut i8) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut sets: *mut *mut i32 = std::ptr::null_mut::<*mut i32>();
        let mut currentSet: *mut i32 = std::ptr::null_mut::<i32>();
        let mut setLengths: *mut i32 = std::ptr::null_mut::<i32>();
        let mut setLength: i32 = 0;
        let mut numSets: i32 = 0;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut l: i32 = 0;
        let mut start: i32 = 0;
        let mut counter: i32 = 0;
        let mut token: *mut i8 = std::ptr::null_mut::<i8>();
        let mut holder: *mut i8 = std::ptr::null_mut::<i8>();
        let mut holderToken: *mut i8 = std::ptr::null_mut::<i8>();
        i = 0_i32;
        while i32::from(*str.offset(i as isize)) != 0_i32 {
            if i32::from(*str.offset(i as isize)) == 'x' as i32 {
                numSets = numSets.wrapping_add(1);
                numSets;
            }
            i = i.wrapping_add(1);
            i;
        }
        if numSets == 0_i32 {
            print!("\n{}", build_str_from_raw_ptr(str.cast::<u8>()));
            return;
        }
        currentSet = calloc(
            ::core::mem::size_of::<i32>() as u64,
            (numSets.wrapping_add(1i32)) as u64,
        ).cast::<i32>();
        setLengths = calloc(
            ::core::mem::size_of::<i32>() as u64,
            (numSets.wrapping_add(1i32)) as u64,
        ).cast::<i32>();
        sets = malloc(
            ((numSets.wrapping_add(1i32)) as u64)
                .wrapping_mul(::core::mem::size_of::<*mut i32>() as u64),
        ).cast::<*mut i32>();
        token = strtok(str, (b"x\0" as *const u8).cast::<i8>());
        while !token.is_null() {
            holder = malloc((strlen(token)).wrapping_mul(::core::mem::size_of::<i8>() as u64)).cast::<i8>();
            j = 0_i32;
            i = 0_i32;
            while i32::from(*token.offset(i as isize)) != 0_i32 {
                if i32::from(*token.offset(i as isize)) >= '0' as i32
                    && i32::from(*token.offset(i as isize)) <= '9' as i32
                {
                    let fresh0 = j;
                    j = j.wrapping_add(1);
                    *holder.offset(fresh0 as isize) = *token.offset(i as isize);
                } else if i32::from(*token.offset(i as isize)) == ',' as i32 {
                    let fresh1 = j;
                    j = j.wrapping_add(1);
                    *holder.offset(fresh1 as isize) = ' ' as i8;
                }
                i = i.wrapping_add(1);
                i;
            }
            *holder.offset(j as isize) = 0;
            setLength = 0_i32;
            i = 0_i32;
            while i32::from(*holder.offset(i as isize)) != 0_i32 {
                if i32::from(*holder.offset(i as isize)) == ' ' as i32 {
                    setLength = setLength.wrapping_add(1);
                    setLength;
                }
                i = i.wrapping_add(1);
                i;
            }
            if setLength == 0_i32 && strlen(holder) == 0 {
                print!("\n{{}}");
                return;
            };
            *setLengths.offset(counter as isize) = setLength.wrapping_add(1);
            let fresh2 = &mut (*sets.offset(counter as isize));
            *fresh2 = malloc(
                ((setLength.wrapping_add(1)) as u64)
                    .wrapping_mul(::core::mem::size_of::<i32>() as u64),
            ).cast::<i32>();
            k = 0_i32;
            start = 0_i32;
            l = 0_i32;
            while i32::from(*holder.offset(l as isize)) != 0_i32 {
                if i32::from(*holder.offset((l.wrapping_add(1i32)) as isize)) == ' ' as i32
                    || i32::from(*holder.offset((l.wrapping_add(1i32)) as isize)) == 0_i32
                {
                    holderToken = malloc(
                        ((l + 1 - start) as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
                    ).cast::<i8>();
                    strncpy(
                        holderToken,
                        holder.offset(start as isize),
                        (l + 1 - start) as u64,
                    );
                    let fresh3 = k;
                    k = k.wrapping_add(1);
                    *(*sets.offset(counter as isize)).offset(fresh3 as isize) = atoi(holderToken);
                    start = l.wrapping_add(2);
                }
                l = l.wrapping_add(1);
                l;
            }
            counter = counter.wrapping_add(1);
            counter;
            token = strtok(std::ptr::null_mut::<i8>(), (b"x\0" as *const u8).cast::<i8>());
        }
        print!("\n{{");
        cartesianProduct(sets, setLengths, currentSet, numSets.wrapping_add(1), 0);
        print!("\x08}}");
    }
}

fn main_0(mut argC: i32, mut argV: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if argC != 2_i32 {
            print!(
                "Usage : {} <Set product expression enclosed in double quotes>",
                build_str_from_raw_ptr((*argV.offset(0_isize)).cast::<u8>())
            );
        } else {
            processInputString(*argV.offset(1_isize));
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
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr()));
}
