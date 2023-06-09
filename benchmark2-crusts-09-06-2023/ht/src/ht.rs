use libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn free(_: *mut libc::c_void);
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht {
    pub entries: *mut ht_entry,
    pub capacity: u64,
    pub length: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ht_entry {
    pub key: *const i8,
    pub value: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hti {
    pub key: *const i8,
    pub value: *mut libc::c_void,
    pub _table: *mut ht,
    pub _index: u64,
}
#[no_mangle]
pub extern "C" fn ht_create() -> *mut ht {
    unsafe {
        let mut table = malloc(::std::mem::size_of::<ht>() as u64) as *mut ht;
        if table.is_null() {
            return 0 as *mut ht;
        };
        (*table).length = 0;
        (*table).capacity = 16;
        let ref mut fresh0 = (*table).entries;
        *fresh0 =
            calloc((*table).capacity, ::std::mem::size_of::<ht_entry>() as u64) as *mut ht_entry;
        if ((*table).entries).is_null() {
            free(table as *mut libc::c_void);
            return 0 as *mut ht;
        }
        return table;
    }
}

#[no_mangle]
pub extern "C" fn ht_destroy(mut table: *mut ht) {
    unsafe {
        let mut i = 0;
        while i < (*table).capacity {
            free((*((*table).entries).offset(i as isize)).key as *mut libc::c_void);
            i = i.wrapping_add(1);
        }
        free((*table).entries as *mut libc::c_void);
        free(table as *mut libc::c_void);
    }
}

extern "C" fn hash_key(mut key: *const i8) -> u64 {
    unsafe {
        let mut hash = 14695981039346656037;
        let mut p = key;
        while *p != 0 {
            hash ^= *p as u64;
            hash = (hash as u64).wrapping_mul(1099511628211) as u64;
            p = p.offset(1);
        }
        return hash;
    }
}

#[no_mangle]
pub extern "C" fn ht_get(mut table: *mut ht, mut key: *const i8) -> *mut libc::c_void {
    unsafe {
        let mut hash = hash_key(key);
        let mut index = hash & ((*table).capacity).wrapping_sub(1);
        while !((*((*table).entries).offset(index as isize)).key).is_null() {
            if strcmp(key, (*((*table).entries).offset(index as isize)).key) == 0 {
                return (*((*table).entries).offset(index as isize)).value;
            }
            index = index.wrapping_add(1);
            if index >= (*table).capacity {
                index = 0;
            }
        }
        return 0 as *mut libc::c_void;
    }
}

extern "C" fn ht_set_entry(
    mut entries: *mut ht_entry,
    mut capacity: u64,
    mut key: *const i8,
    mut value: *mut libc::c_void,
    mut plength: *mut u64,
) -> *const i8 {
    unsafe {
        let mut hash = hash_key(key);
        let mut index = hash & capacity.wrapping_sub(1);
        while !((*entries.offset(index as isize)).key).is_null() {
            if strcmp(key, (*entries.offset(index as isize)).key) == 0 {
                let ref mut fresh1 = (*entries.offset(index as isize)).value;
                *fresh1 = value;
                return (*entries.offset(index as isize)).key;
            }
            index = index.wrapping_add(1);
            if index >= capacity {
                index = 0;
            }
        }
        if !plength.is_null() {
            key = strdup(key);
            if key.is_null() {
                return 0 as *const i8;
            }
            *plength = (*plength).wrapping_add(1);
        }
        let ref mut fresh2 = (*entries.offset(index as isize)).key;
        *fresh2 = key as *mut i8;
        let ref mut fresh3 = (*entries.offset(index as isize)).value;
        *fresh3 = value;
        return key;
    }
}

extern "C" fn ht_expand(mut table: *mut ht) -> bool {
    unsafe {
        let mut new_capacity = ((*table).capacity).wrapping_mul(2);
        if new_capacity < (*table).capacity {
            return 0 != 0;
        }
        let mut new_entries =
            calloc(new_capacity, ::std::mem::size_of::<ht_entry>() as u64) as *mut ht_entry;
        if new_entries.is_null() {
            return 0 != 0;
        }
        let mut i = 0;
        while i < (*table).capacity {
            let mut entry = *((*table).entries).offset(i as isize);
            if !(entry.key).is_null() {
                ht_set_entry(
                    new_entries,
                    new_capacity,
                    entry.key,
                    entry.value,
                    0 as *mut u64,
                );
            }
            i = i.wrapping_add(1);
        }
        free((*table).entries as *mut libc::c_void);
        let ref mut fresh4 = (*table).entries;
        *fresh4 = new_entries;
        (*table).capacity = new_capacity;
        return 1 != 0;
    }
}

#[no_mangle]
pub extern "C" fn ht_set(
    mut table: *mut ht,
    mut key: *const i8,
    mut value: *mut libc::c_void,
) -> *const i8 {
    unsafe {
        if !value.is_null() {
        } else {
            __assert_fail(
                b"value != NULL\0" as *const u8 as *const i8,
                b"ht.c\0" as *const u8 as *const i8,
                154,
                (*::std::mem::transmute::<&[u8; 47], &[i8; 47]>(
                    b"const char *ht_set(ht *, const char *, void *)\0",
                ))
                .as_ptr(),
            );
        }
        if value.is_null() {
            return 0 as *const i8;
        }
        if (*table).length >= ((*table).capacity).wrapping_div(2) {
            if !ht_expand(table) {
                return 0 as *const i8;
            }
        }
        return ht_set_entry(
            (*table).entries,
            (*table).capacity,
            key,
            value,
            &mut (*table).length,
        );
    }
}

#[no_mangle]
pub extern "C" fn ht_length(mut table: *mut ht) -> u64 {
    unsafe {
        return (*table).length;
    }
}

#[no_mangle]
pub extern "C" fn ht_iterator(mut table: *mut ht) -> hti {
    unsafe {
        let mut it = hti {
            key: 0 as *const i8,
            value: 0 as *mut libc::c_void,
            _table: 0 as *mut ht,
            _index: 0,
        };
        it._table = table;
        it._index = 0;
        return it;
    }
}

#[no_mangle]
pub extern "C" fn ht_next(mut it: *mut hti) -> bool {
    unsafe {
        let mut table = (*it)._table;
        while (*it)._index < (*table).capacity {
            let mut i = (*it)._index;
            let ref mut fresh5 = (*it)._index;
            *fresh5 = (*fresh5).wrapping_add(1);
            if !((*((*table).entries).offset(i as isize)).key).is_null() {
                let mut entry = *((*table).entries).offset(i as isize);
                let ref mut fresh6 = (*it).key;
                *fresh6 = entry.key;
                let ref mut fresh7 = (*it).value;
                *fresh7 = entry.value;
                return 1 != 0;
            }
        }
        return 0 != 0;
    }
}
