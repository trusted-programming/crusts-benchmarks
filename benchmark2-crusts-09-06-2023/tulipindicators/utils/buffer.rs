extern "C" {
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    #[no_mangle]
    fn free(_: *mut std::os::raw::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_buffer {
    pub size: std::os::raw::c_int,
    pub pushes: std::os::raw::c_int,
    pub index: std::os::raw::c_int,
    pub sum: std::os::raw::c_double,
    pub vals: [std::os::raw::c_double; 1],
}
#[no_mangle]
pub extern "C" fn ti_buffer_new(mut size: std::os::raw::c_int) -> *mut ti_buffer {
    unsafe {
        let s: std::os::raw::c_int = ::std::mem::size_of::<ti_buffer>() as std::os::raw::c_ulong
            as std::os::raw::c_int
            + (size - 1 as std::os::raw::c_int)
                * ::std::mem::size_of::<std::os::raw::c_double>() as std::os::raw::c_ulong
                    as std::os::raw::c_int;
        let mut ret: *mut ti_buffer =
            malloc(s as std::os::raw::c_uint as std::os::raw::c_ulong) as *mut ti_buffer;
        (*ret).size = size;
        (*ret).pushes = 0 as std::os::raw::c_int;
        (*ret).index = 0 as std::os::raw::c_int;
        (*ret).sum = 0 as std::os::raw::c_int as std::os::raw::c_double;
        return ret;
    }
}

#[no_mangle]
pub extern "C" fn ti_buffer_free(mut buffer: *mut ti_buffer) {
    unsafe {
        free(buffer as *mut std::os::raw::c_void);
    }
}
