use libc;
extern "C" {
    fn heman_image_create(width: i32, height: i32, nbands: i32) -> *mut heman_image;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heman_image_s {
    pub width: i32,
    pub height: i32,
    pub nbands: i32,
    pub data: *mut libc::c_float,
}
pub type heman_image = heman_image_s;
#[no_mangle]
pub extern "C" fn heman_import_u8(
    mut width: i32,
    mut height: i32,
    mut nbands: i32,
    mut source: *const u8,
    mut minval: libc::c_float,
    mut maxval: libc::c_float,
) -> *mut heman_image {
    unsafe {
        let mut result = heman_image_create(width, height, nbands);
        let mut inp = source;
        let mut outp = (*result).data;
        let mut scale = (maxval - minval) / 255.0f32;
        let mut size = height * width * nbands;
        let mut i = 0;
        while i < size {
            let fresh0 = inp;
            inp = inp.offset(1);
            let mut v = *fresh0 as i32 as libc::c_float * scale + minval;
            let fresh1 = outp;
            outp = outp.offset(1);
            *fresh1 = if minval > (if maxval > v { v } else { maxval }) {
                minval
            } else if maxval > v {
                v
            } else {
                maxval
            };
            i += 1;
        }
        return result;
    }
}
