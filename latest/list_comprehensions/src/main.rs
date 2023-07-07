#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]

extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct List {
    pub nx: *mut List,
    pub val: [i8; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct Triple {
    pub _1: i32,
    pub _2: i32,
    pub _3: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct ITERATOR {
    pub l: *mut List,
    pub old: *mut List,
    pub p: *mut ITERATOR,
}
#[no_mangle]
pub static mut FE_var: *mut ITERATOR = 0 as *const ITERATOR as *mut ITERATOR;
#[no_mangle]
pub static mut SEQ_base: ITERATOR = ITERATOR {
    l: 0 as *const List as *mut List,
    old: 0 as *const List as *mut List,
    p: 0 as *const ITERATOR as *mut ITERATOR,
};
#[no_mangle]
// SAFETY: machine generated unsafe code
pub static mut SEQ_var: *mut ITERATOR = unsafe { &SEQ_base as *const ITERATOR as *mut ITERATOR };
#[no_mangle]
pub extern "C" fn listNew(mut sz: i32, mut val: *mut libc::c_void) -> *mut List {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut l: *mut List =
            malloc((::core::mem::size_of::<List>() as u64).wrapping_add(sz as u64)).cast::<List>();
        (*l).nx = std::ptr::null_mut::<List>();
        memcpy(((*l).val).as_mut_ptr().cast::<libc::c_void>(), val, sz as u64);
        l
    }
}

#[no_mangle]
pub extern "C" fn listAppend(
    mut l: *mut List,
    mut sz: i32,
    mut val: *mut libc::c_void,
) -> *mut List {
// SAFETY: machine generated unsafe code
    unsafe {
        while !((*l).nx).is_null() {
            l = (*l).nx;
        }
        (*l).nx = listNew(sz, val);
        l
    }
}

#[no_mangle]
pub extern "C" fn intRangeList(mut f: i32, mut t: i32) -> *mut List {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut l: *mut List = listNew(
            ::core::mem::size_of::<i32>() as i32,
            (&mut f as *mut i32).cast::<libc::c_void>(),
        );
        let mut e: *mut List = l;
        let mut i: i32 = f.wrapping_add(1);
        while i <= t {
            (*e).nx = listNew(
                ::core::mem::size_of::<i32>() as i32,
                (&mut i as *mut i32).cast::<libc::c_void>(),
            );
            e = (*e).nx;
            i = i.wrapping_add(1);
            i;
        }
        l
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut z: i32 = 0;
        let n: i32 = 20;
        SEQ_var = &mut {
            
            ITERATOR {
                l: std::ptr::null_mut::<List>(),
                old: std::ptr::null_mut::<List>(),
                p: SEQ_var,
            }
        } as *mut ITERATOR;
        FE_var = &mut {
            
            ITERATOR {
// SAFETY: machine generated unsafe code
                l: (intRangeList as unsafe extern "C" fn(i32, i32) -> *mut List)(1, n),
                old: std::ptr::null_mut::<List>(),
                p: FE_var,
            }
        } as *mut ITERATOR;
        ::core::ptr::write_volatile(
            &mut x as *mut i32,
            *(&mut (*(*FE_var).l).val as *mut [i8; 0]).cast::<i32>(),
        );
        while (if !((*FE_var).l).is_null() {
            ::core::ptr::write_volatile(
                &mut x as *mut i32,
                *(&mut (*(*FE_var).l).val as *mut [i8; 0]).cast::<i32>(),
            );
            1_i32
        } else {
            0_i32
        }) != 0_i32
        {
            FE_var = &mut {
                
                ITERATOR {
// SAFETY: machine generated unsafe code
                    l: (intRangeList as unsafe extern "C" fn(i32, i32) -> *mut List)(x, n),
                    old: std::ptr::null_mut::<List>(),
                    p: FE_var,
                }
            } as *mut ITERATOR;
            ::core::ptr::write_volatile(
                &mut y as *mut i32,
                *(&mut (*(*FE_var).l).val as *mut [i8; 0]).cast::<i32>(),
            );
            while (if !((*FE_var).l).is_null() {
                ::core::ptr::write_volatile(
                    &mut y as *mut i32,
                    *(&mut (*(*FE_var).l).val as *mut [i8; 0]).cast::<i32>(),
                );
                1_i32
            } else {
                0_i32
            }) != 0_i32
            {
                FE_var = &mut {
                    
                    ITERATOR {
// SAFETY: machine generated unsafe code
                        l: (intRangeList as unsafe extern "C" fn(i32, i32) -> *mut List)(y, n),
                        old: std::ptr::null_mut::<List>(),
                        p: FE_var,
                    }
                } as *mut ITERATOR;
                ::core::ptr::write_volatile(
                    &mut z as *mut i32,
                    *(&mut (*(*FE_var).l).val as *mut [i8; 0]).cast::<i32>(),
                );
                while (if !((*FE_var).l).is_null() {
                    ::core::ptr::write_volatile(
                        &mut z as *mut i32,
                        *(&mut (*(*FE_var).l).val as *mut [i8; 0]).cast::<i32>(),
                    );
                    1_i32
                } else {
                    0_i32
                }) != 0_i32
                {
                    if x * x + y * y == z.wrapping_mul(z) {
                        if !((*SEQ_var).l).is_null() {
                            listAppend(
                                (*SEQ_var).l,
                                ::core::mem::size_of::<Triple>() as i32,
                                (&mut {
                                    
                                    Triple {
                                        _1: x,
                                        _2: y,
                                        _3: z,
                                    }
                                } as *mut Triple).cast::<libc::c_void>(),
                            );
                        } else {
                            (*SEQ_var).l = listNew(::core::mem::size_of::<Triple>() as i32, (&mut {
                                
                                Triple {
                                    _1: x,
                                    _2: y,
                                    _3: z,
                                }
                            } as *mut Triple).cast::<libc::c_void>());
                        };
                    } ;
                    (*FE_var).l = (*(*FE_var).l).nx;
                }
                FE_var = (*FE_var).p;
                (*FE_var).l = (*(*FE_var).l).nx;
            }
            FE_var = (*FE_var).p;
            (*FE_var).l = (*(*FE_var).l).nx;
        }
        FE_var = (*FE_var).p;
        (*(*SEQ_var).p).old = (*SEQ_var).l;
        SEQ_var = (*SEQ_var).p;
        let mut pTriples: *mut List = (*SEQ_var).old;
        let mut t: Triple = Triple {
            _1: 0,
            _2: 0,
            _3: 0,
        };
        FE_var = &mut {
            
            ITERATOR {
                l: pTriples,
                old: std::ptr::null_mut::<List>(),
                p: FE_var,
            }
        } as *mut ITERATOR;
        ::core::ptr::write_volatile(
            &mut t as *mut Triple,
            *(&mut (*(*FE_var).l).val as *mut [i8; 0]).cast::<Triple>(),
        );
        while (if !((*FE_var).l).is_null() {
            ::core::ptr::write_volatile(
                &mut t as *mut Triple,
                *(&mut (*(*FE_var).l).val as *mut [i8; 0]).cast::<Triple>(),
            );
            1_i32
        } else {
            0_i32
        }) != 0_i32
        {
            println!("{}, {}, {}", t._1, t._2, t._3);
            (*FE_var).l = (*(*FE_var).l).nx;
        }
        FE_var = (*FE_var).p;
        0_i32
    }
}

pub fn main() {
    ::std::process::exit(main_0());
}
