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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sHme {
    pub key: *const i8,
    pub value: i32,
    pub link: *mut sHme,
}
pub type MapEntry = *mut sHme;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct he {
    pub first: MapEntry,
    pub last: MapEntry,
}
pub type HashElement = he;
// SAFETY: machine generated unsafe code
pub type KeyCopyF = Option<unsafe extern "C" fn(*mut *const i8, *const i8) -> ()>;
// SAFETY: machine generated unsafe code
pub type ValCopyF = Option<unsafe extern "C" fn(*mut i32, i32) -> ()>;
// SAFETY: machine generated unsafe code
pub type KeyHashF = Option<unsafe extern "C" fn(*const i8, i32) -> u32>;
// SAFETY: machine generated unsafe code
pub type KeyCmprF = Option<unsafe extern "C" fn(*const i8, *const i8) -> i32>;
#[no_mangle]
pub extern "C" fn strhashkey(mut key: *const i8, mut max: i32) -> u32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut h: u32 = 0;
        let mut hl: u32 = 0;
        let mut hr: u32 = 0;
        while *key != 0 {
            h = h.wrapping_add(*key as u32);
            hl = 0x5c5 ^ (h & 0xfff00000) >> 18_i32;
            hr = h & 0xfffff;
            let fresh0 = key;
            key = key.offset(1);
            h = hl ^ hr ^ *fresh0 as u32;
        }
        h.wrapping_rem(max as u32)
    }
}

#[no_mangle]
pub static mut hash: [HashElement; 4096] = [HashElement {
    first: 0 as *const sHme as *mut sHme,
    last: 0 as *const sHme as *mut sHme,
}; 4096];
#[no_mangle]
pub extern "C" fn HashAddH(
    mut key: *const i8,
    mut value: i32,
    mut copyKey: KeyCopyF,
    mut copyVal: ValCopyF,
    mut hashKey: KeyHashF,
    mut keySame: KeyCmprF,
) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut hix: u32 = hashKey.expect("non-null function pointer")(key, 4096);
        let mut m_ent: MapEntry = std::ptr::null_mut::<sHme>();
        m_ent = hash[hix as usize].first;
        while !m_ent.is_null()
            && keySame.expect("non-null function pointer")((*m_ent).key, key)
                == 0_i32
        {
            m_ent = (*m_ent).link;
        }
        if !m_ent.is_null() {
            copyVal.expect("non-null function pointer")(
                &mut (*m_ent).value,
                value,
            );
        } else {
            let mut last: MapEntry = std::ptr::null_mut::<sHme>();
            let mut hme: MapEntry = malloc(::core::mem::size_of::<sHme>() as u64).cast::<sHme>();
            copyKey.expect("non-null function pointer")(
                &mut (*hme).key,
                key,
            );
            copyVal.expect("non-null function pointer")(
                &mut (*hme).value,
                value,
            );
            (*hme).link = std::ptr::null_mut::<sHme>();
            last = hash[hix as usize].last;
            if !last.is_null() {
                (*last).link = hme;
            } else {
                hash[hix as usize].first = hme;
            }
            hash[hix as usize].last = hme;
        };
    }
}

#[no_mangle]
pub extern "C" fn HashGetH(
    mut val: *mut i32,
    mut key: *const i8,
    mut hashKey: KeyHashF,
    mut keySame: KeyCmprF,
) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut hix: u32 = hashKey.expect("non-null function pointer")(key, 4096);
        let mut m_ent: MapEntry = std::ptr::null_mut::<sHme>();
        m_ent = hash[hix as usize].first;
        while !m_ent.is_null()
            && keySame.expect("non-null function pointer")((*m_ent).key, key)
                == 0_i32
        {
            m_ent = (*m_ent).link;
        }
        if !m_ent.is_null() {
            *val = (*m_ent).value;
        }
        i32::from(m_ent != std::ptr::null_mut::<libc::c_void>().cast::<sHme>())
    }
}

#[no_mangle]
pub extern "C" fn copyStr(mut dest: *mut *const i8, mut src: *const i8) {
// SAFETY: machine generated unsafe code
    unsafe {
        *dest = strdup(src);
    }
}

#[no_mangle]
pub extern "C" fn copyInt(mut dest: *mut i32, mut src: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        *dest = src;
    }
}

#[no_mangle]
pub extern "C" fn strCompare(mut key1: *const i8, mut key2: *const i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        i32::from(strcmp(key1, key2) == 0_i32)
    }
}

#[no_mangle]
pub extern "C" fn HashAdd(mut key: *const i8, mut value: i32) {
// SAFETY: machine generated unsafe code
    unsafe {
        HashAddH(
            key,
            value,
// SAFETY: machine generated unsafe code
            Some(copyStr as unsafe extern "C" fn(*mut *const i8, *const i8) -> ()),
// SAFETY: machine generated unsafe code
            Some(copyInt as unsafe extern "C" fn(*mut i32, i32) -> ()),
// SAFETY: machine generated unsafe code
            Some(strhashkey as unsafe extern "C" fn(*const i8, i32) -> u32),
// SAFETY: machine generated unsafe code
            Some(strCompare as unsafe extern "C" fn(*const i8, *const i8) -> i32),
        );
    }
}

#[no_mangle]
pub extern "C" fn HashGet(mut val: *mut i32, mut key: *const i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        HashGetH(
            val,
            key,
// SAFETY: machine generated unsafe code
            Some(strhashkey as unsafe extern "C" fn(*const i8, i32) -> u32),
// SAFETY: machine generated unsafe code
            Some(strCompare as unsafe extern "C" fn(*const i8, *const i8) -> i32),
        )
    }
}

fn main_0() -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        static mut keyList: [*const i8; 6] = [
            (b"red\0" as *const u8).cast::<i8>(),
            (b"orange\0" as *const u8).cast::<i8>(),
            (b"yellow\0" as *const u8).cast::<i8>(),
            (b"green\0" as *const u8).cast::<i8>(),
            (b"blue\0" as *const u8).cast::<i8>(),
            (b"violet\0" as *const u8).cast::<i8>(),
        ];
        static mut valuList: [i32; 6] = [1_i32, 43_i32, 640_i32, 747_i32, 42_i32, 42_i32];
        let mut ix: i32 = 0;
        ix = 0_i32;
        while ix < 6_i32 {
            HashAdd(keyList[ix as usize], valuList[ix as usize]);
            ix = ix.wrapping_add(1);
            ix;
        }
    }
    0_i32
}

pub fn main() {
    ::std::process::exit(main_0());
}
