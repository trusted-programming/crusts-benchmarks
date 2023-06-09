use libc;
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub extern "C" fn BrotliDefaultAllocFunc(
    mut opaque: *mut libc::c_void,
    mut size: u64,
) -> *mut libc::c_void {
    unsafe {
        return malloc(size);
    }
}

#[no_mangle]
pub extern "C" fn BrotliDefaultFreeFunc(
    mut opaque: *mut libc::c_void,
    mut address: *mut libc::c_void,
) {
    unsafe {
        free(address);
    }
}
