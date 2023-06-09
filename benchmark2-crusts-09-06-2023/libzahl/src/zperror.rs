use libc;
extern "C" {
    fn abort() -> !;
    static mut libzahl_error: i32;
    fn __errno_location() -> *mut i32;
    fn perror(__s: *const i8);
}
#[no_mangle]
pub extern "C" fn zperror(mut prefix: *const i8) {
    unsafe {
        if libzahl_error >= 0 {
            *__errno_location() = libzahl_error;
            perror(prefix);
        } else {
            abort();
        };
    }
}
