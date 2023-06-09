use libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_point {
    pub x: f64,
    pub y: f64,
}
pub type quadtree_point_t = quadtree_point;
#[no_mangle]
pub extern "C" fn quadtree_point_new(mut x: f64, mut y: f64) -> *mut quadtree_point_t {
    let mut point = 0 as *mut quadtree_point_t;
    unsafe {
        point = malloc(::std::mem::size_of::<quadtree_point_t>() as u64) as *mut quadtree_point_t;
    }
    if point.is_null() {
        return 0 as *mut quadtree_point_t;
    };
    (*point).x = x;
    (*point).y = y;
    return point;
}

#[no_mangle]
pub extern "C" fn quadtree_point_free(mut point: *mut quadtree_point_t) {
    unsafe {
        free(point as *mut libc::c_void);
    }
}
