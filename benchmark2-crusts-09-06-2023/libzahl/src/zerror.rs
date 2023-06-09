use libc;
extern "C" {
    fn strerror(_: i32) -> *mut i8;
    static mut libzahl_error: i32;
    fn abort() -> !;
    fn __errno_location() -> *mut i32;
}
pub const ZERROR_ERRNO_SET: u32 = 0;
#[no_mangle]
pub extern "C" fn zerror(mut desc: *mut *const i8) -> u32 {
    unsafe {
        if libzahl_error >= 0 {
            if !desc.is_null() {
                *desc = strerror(libzahl_error);
            };
            *__errno_location() = libzahl_error;
            return ZERROR_ERRNO_SET;
        } else {
            if !desc.is_null() {
                abort();
            }
            return -libzahl_error as u32;
        };
    }
}
