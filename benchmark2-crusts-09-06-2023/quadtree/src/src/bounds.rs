use libc;
extern "C" {
    fn fabs(_: f64) -> f64;
    fn quadtree_point_free(point: *mut quadtree_point_t);
    fn quadtree_point_new(x: f64, y: f64) -> *mut quadtree_point_t;
    fn free(_: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn fmax(_: f64, _: f64) -> f64;
    fn fmin(_: f64, _: f64) -> f64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_point {
    pub x: f64,
    pub y: f64,
}
pub type quadtree_point_t = quadtree_point;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quadtree_bounds {
    pub nw: *mut quadtree_point_t,
    pub se: *mut quadtree_point_t,
    pub width: f64,
    pub height: f64,
}
pub type quadtree_bounds_t = quadtree_bounds;
#[no_mangle]
pub extern "C" fn quadtree_bounds_extend(
    mut bounds: *mut quadtree_bounds_t,
    mut x: f64,
    mut y: f64,
) {
    unsafe {
        (*(*bounds).nw).x = fmin(x, (*(*bounds).nw).x);
        (*(*bounds).nw).y = fmax(y, (*(*bounds).nw).y);
        (*(*bounds).se).x = fmax(x, (*(*bounds).se).x);
        (*(*bounds).se).y = fmin(y, (*(*bounds).se).y);
        (*bounds).width = fabs((*(*bounds).nw).x - (*(*bounds).se).x);
        (*bounds).height = fabs((*(*bounds).nw).y - (*(*bounds).se).y);
    }
}

#[no_mangle]
pub extern "C" fn quadtree_bounds_free(mut bounds: *mut quadtree_bounds_t) {
    unsafe {
        quadtree_point_free((*bounds).nw);
        quadtree_point_free((*bounds).se);
        free(bounds as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn quadtree_bounds_new() -> *mut quadtree_bounds_t {
    let mut bounds = 0 as *mut quadtree_bounds_t;
    unsafe {
        bounds =
            malloc(::std::mem::size_of::<quadtree_bounds_t>() as u64) as *mut quadtree_bounds_t;
    }
    if bounds.is_null() {
        return 0 as *mut quadtree_bounds_t;
    }
    let ref mut fresh0 = (*bounds).nw;
    unsafe {
        *fresh0 = quadtree_point_new(::std::f32::INFINITY as f64, -::std::f32::INFINITY as f64);
    }
    let ref mut fresh1 = (*bounds).se;
    unsafe {
        *fresh1 = quadtree_point_new(-::std::f32::INFINITY as f64, ::std::f32::INFINITY as f64);
    }
    (*bounds).width = 0 as f64;
    (*bounds).height = 0 as f64;
    return bounds;
}
