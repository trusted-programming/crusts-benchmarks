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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: i32) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffcode {
    pub nbits: i32,
    pub code: i32,
}
pub type huffcode_t = huffcode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct huffheap {
    pub h: *mut i32,
    pub n: i32,
    pub s: i32,
    pub cs: i32,
    pub f: *mut i64,
}
pub type heap_t = huffheap;
extern "C" fn _heap_create(mut s: i32, mut f: *mut i64) -> *mut heap_t {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut h: *mut heap_t = std::ptr::null_mut::<heap_t>();
        h = malloc(::core::mem::size_of::<heap_t>() as u64).cast::<huffheap>();
        (*h).h = malloc((::core::mem::size_of::<i32>() as u64).wrapping_mul(s as u64)).cast::<i32>();
        (*h).cs = s;
        (*h).s = (*h).cs;
        (*h).n = 0_i32;
        (*h).f = f;
        h
    }
}

extern "C" fn _heap_destroy(mut heap: *mut heap_t) {
// SAFETY: machine generated unsafe code
    unsafe {
        free((*heap).h.cast::<libc::c_void>());
        free(heap.cast::<libc::c_void>());
    }
}

extern "C" fn _heap_sort(mut heap: *mut heap_t) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 1;
        let mut j: i32 = 2;
        let mut a: *mut i32 = (*heap).h;
        while i < (*heap).n {
            if *((*heap).f).offset(*a.offset((i - 1i32) as isize) as isize)
                >= *((*heap).f).offset(*a.offset(i as isize) as isize)
            {
                i = j;
                j = j.wrapping_add(1);
                j;
            } else {
                let mut t_: i32 = 0;
                t_ = *a.offset((i - 1i32) as isize);
                *a.offset((i - 1i32) as isize) = *a.offset(i as isize);
                *a.offset(i as isize) = t_;
                i = i.wrapping_sub(1);
                i;
                i = if i == 0_i32 {
                    let fresh0 = j;
                    j = j.wrapping_add(1);
                    fresh0
                } else {
                    i
                };
            }
        }
    }
}

extern "C" fn _heap_add(mut heap: *mut heap_t, mut c: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        if (*heap).n + 1_i32 > (*heap).s {
            (*heap).h = realloc(
                (*heap).h.cast::<libc::c_void>(),
                ((*heap).s + (*heap).cs) as u64,
            ).cast::<i32>();
            (*heap).s += (*heap).cs;
        };
        *((*heap).h).offset((*heap).n as isize) = c;
        (*heap).n += 1_i32;
        (*heap).n;
        _heap_sort(heap);
    }
}

extern "C" fn _heap_remove(mut heap: *mut heap_t) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if (*heap).n > 0_i32 {
            (*heap).n -= 1_i32;
            (*heap).n;
            return *((*heap).h).offset((*heap).n as isize);
        }
        -1_i32
    }
}

#[no_mangle]
pub extern "C" fn create_huffman_codes(mut freqs: *mut i64) -> *mut *mut huffcode_t {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut codes: *mut *mut huffcode_t = std::ptr::null_mut::<*mut huffcode_t>();
        let mut heap: *mut heap_t = std::ptr::null_mut::<heap_t>();
        let mut efreqs: [i64; 512] = [0; 512];
        let mut preds: [i32; 512] = [0; 512];
        let mut i: i32 = 0;
        let mut extf: i32 = 256;
        let mut r1: i32 = 0;
        let mut r2: i32 = 0;
        memcpy(
            efreqs.as_mut_ptr().cast::<libc::c_void>(),
            freqs as *const libc::c_void,
            (::core::mem::size_of::<i64>() as u64).wrapping_mul(256),
        );
        memset(
            (&mut *efreqs.as_mut_ptr().offset(256_isize) as *mut i64).cast::<libc::c_void>(),
            0,
            (::core::mem::size_of::<i64>() as u64).wrapping_mul(256),
        );
        heap = _heap_create(256 * 2, efreqs.as_mut_ptr());
        if heap.is_null() {
            return std::ptr::null_mut::<*mut huffcode_t>();
        }
        i = 0_i32;
        while i < 256_i32 {
            if efreqs[i as usize] > 0 {
                _heap_add(heap, i);
            }
            i = i.wrapping_add(1);
            i;
        }
        while (*heap).n > 1_i32 {
            r1 = _heap_remove(heap);
            r2 = _heap_remove(heap);
            efreqs[extf as usize] = efreqs[r1 as usize] + efreqs[r2 as usize];
            _heap_add(heap, extf);
            preds[r1 as usize] = extf;
            preds[r2 as usize] = -extf;
            extf = extf.wrapping_add(1);
            extf;
        }
        r1 = _heap_remove(heap);
        preds[r1 as usize] = r1;
        _heap_destroy(heap);
        codes = malloc((::core::mem::size_of::<*mut huffcode_t>() as u64).wrapping_mul(256)).cast::<*mut huffcode>();
        let mut bc: i32 = 0;
        let mut bn: i32 = 0;
        let mut ix: i32 = 0;
        i = 0_i32;
        while i < 256_i32 {
            bc = 0_i32;
            bn = 0_i32;
            if efreqs[i as usize] == 0 {
                let fresh1 = &mut (*codes.offset(i as isize));
                *fresh1 = std::ptr::null_mut::<huffcode_t>();
            } else {
                ix = i;
                while abs(preds[ix as usize]) != ix {
                    bc |= (if preds[ix as usize] >= 0_i32 { 1_i32 } else { 0_i32 }) << bn;
                    ix = abs(preds[ix as usize]);
                    bn = bn.wrapping_add(1);
                    bn;
                }
                let fresh2 = &mut (*codes.offset(i as isize));
                *fresh2 = malloc(::core::mem::size_of::<huffcode_t>() as u64).cast::<huffcode>();
                (**codes.offset(i as isize)).nbits = bn;
                (**codes.offset(i as isize)).code = bc;
            }
            i = i.wrapping_add(1);
            i;
        }
        codes
    }
}

#[no_mangle]
pub extern "C" fn free_huffman_codes(mut c: *mut *mut huffcode_t) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut i: i32 = 0;
        i = 0_i32;
        while i < 256_i32 {
            free((*c.offset(i as isize)).cast::<libc::c_void>());
            i = i.wrapping_add(1);
            i;
        }
        free(c.cast::<libc::c_void>());
    }
}

#[no_mangle]
pub extern "C" fn inttobits(mut c: i32, mut n: i32, mut s: *mut i8) {
// SAFETY: machine generated unsafe code
    unsafe {
        *s.offset(n as isize) = 0;
        while n > 0_i32 {
            *s.offset((n - 1i32) as isize) = (c % 2_i32 + '0' as i32) as i8;
            c >>= 1_i32;
            n = n.wrapping_sub(1);
            n;
        }
    }
}

#[no_mangle]
pub static mut test: *const i8 =
    (b"this is an example for huffman encoding\0" as *const u8).cast::<i8>();
fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut r: *mut *mut huffcode_t = std::ptr::null_mut::<*mut huffcode_t>();
        let mut i: i32 = 0;
        let mut strbit: [i8; 100] = [0; 100];
        let mut p: *const i8 = std::ptr::null::<i8>();
        let mut freqs: [i64; 256] = [0; 256];
        memset(
            freqs.as_mut_ptr().cast::<libc::c_void>(),
            0,
            ::core::mem::size_of::<[i64; 256]>() as u64,
        );
        p = test;
        while i32::from(*p) != '\0' as i32 {
            let fresh3 = p;
            p = p.offset(1);
            freqs[*fresh3 as usize] += 1;
            freqs[*fresh3 as usize];
        }
        r = create_huffman_codes(freqs.as_mut_ptr());
        i = 0_i32;
        while i < 256_i32 {
            if !(*r.offset(i as isize)).is_null() {
                inttobits(
                    (**r.offset(i as isize)).code,
                    (**r.offset(i as isize)).nbits,
                    strbit.as_mut_ptr(),
                );
                println!(
                    "{} ({}) {}",
                    i,
                    (**r.offset(i as isize)).code,
                    build_str_from_raw_ptr(strbit.as_mut_ptr().cast::<u8>())
                );
            }
            i = i.wrapping_add(1);
            i;
        }
        free_huffman_codes(r);
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
