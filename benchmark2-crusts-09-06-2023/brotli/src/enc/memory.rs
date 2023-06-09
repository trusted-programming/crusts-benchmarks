use libc;
extern "C" {
    fn BrotliDefaultAllocFunc(opaque: *mut libc::c_void, size: u64) -> *mut libc::c_void;
    fn BrotliDefaultFreeFunc(opaque: *mut libc::c_void, address: *mut libc::c_void);
    fn exit(_: i32) -> !;
}
pub type brotli_alloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func: brotli_alloc_func,
    pub free_func: brotli_free_func,
    pub opaque: *mut libc::c_void,
}
#[no_mangle]
pub extern "C" fn BrotliInitMemoryManager(
    mut m: *mut MemoryManager,
    mut alloc_func: brotli_alloc_func,
    mut free_func: brotli_free_func,
    mut opaque: *mut libc::c_void,
) {
    unsafe {
        if alloc_func.is_none() {
            let ref mut fresh0 = (*m).alloc_func;
            *fresh0 = Some(
                BrotliDefaultAllocFunc
                    as unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void,
            );
            let ref mut fresh1 = (*m).free_func;
            *fresh1 = Some(
                BrotliDefaultFreeFunc
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            );
            let ref mut fresh2 = (*m).opaque;
            *fresh2 = 0 as *mut libc::c_void;
        } else {
            let ref mut fresh3 = (*m).alloc_func;
            *fresh3 = alloc_func;
            let ref mut fresh4 = (*m).free_func;
            *fresh4 = free_func;
            let ref mut fresh5 = (*m).opaque;
            *fresh5 = opaque;
        };
    }
}

#[no_mangle]
pub extern "C" fn BrotliAllocate(mut m: *mut MemoryManager, mut n: u64) -> *mut libc::c_void {
    unsafe {
        let mut result = ((*m).alloc_func).expect("non-null function pointer")((*m).opaque, n);
        if result.is_null() {
            exit(1);
        }
        return result;
    }
}

#[no_mangle]
pub extern "C" fn BrotliFree(mut m: *mut MemoryManager, mut p: *mut libc::c_void) {
    unsafe {
        ((*m).free_func).expect("non-null function pointer")((*m).opaque, p);
    }
}

#[no_mangle]
pub extern "C" fn BrotliWipeOutMemoryManager(mut m: *mut MemoryManager) {}
