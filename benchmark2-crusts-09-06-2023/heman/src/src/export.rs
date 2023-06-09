use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn heman_image_texel(_: *mut heman_image, x: i32, y: i32) -> *mut libc::c_float;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn fwrite(_: *const libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn free(_: *mut libc::c_void);
    fn malloc(_: u64) -> *mut libc::c_void;
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
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: i64,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: i64,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[no_mangle]
pub extern "C" fn heman_export_ply(mut img: *mut heman_image, mut filename: *const i8) {
    unsafe {
        if (*img).nbands == 1 {
        } else {
            __assert_fail(
                b"img->nbands == 1\0" as *const u8 as *const i8,
                b"../src/export.c\0" as *const u8 as *const i8,
                8,
                (*::std::mem::transmute::<&[u8; 51], &[i8; 51]>(
                    b"void heman_export_ply(heman_image *, const char *)\0",
                ))
                .as_ptr(),
            );
        }
        let mut fout = fopen(filename, b"wb\0" as *const u8 as *const i8);
        let mut ncols = (*img).width - 1;
        let mut nrows = (*img).height - 1;
        let mut ncells = ncols * nrows;
        let mut nverts = (*img).width * (*img).height;
        fprintf (fout, b"ply\nformat binary_little_endian 1.0\ncomment heman\nelement vertex %d\nproperty float32 x\nproperty float32 y\nproperty float32 z\nelement face %d\nproperty list int32 int32 vertex_indices\nend_header\n\0" as * const u8 as * const
          i8, nverts, ncells,);
        let mut invw = 2.0f32 / (*img).width as libc::c_float;
        let mut invh = 2.0f32 / (*img).height as libc::c_float;
        let mut vert: [libc::c_float; 3] = [0.; 3];
        let mut j = 0;
        while j < (*img).height {
            let mut i = 0;
            while i < (*img).width {
                vert[0 as usize] = -1i32 as libc::c_float + i as libc::c_float * invw;
                vert[1 as usize] = -1i32 as libc::c_float + j as libc::c_float * invh;
                vert[2 as usize] = *heman_image_texel(img, i, j);
                fwrite(
                    vert.as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<[libc::c_float; 3]>() as u64,
                    1,
                    fout,
                );
                i += 1;
            }
            j += 1;
        }
        let mut face: [i32; 5] = [0; 5];
        face[0 as usize] = 4;
        let mut j_0 = 0;
        while j_0 < nrows {
            let mut p = j_0 * (*img).width;
            let mut i_0 = 0;
            while i_0 < ncols {
                face[1 as usize] = p;
                face[2 as usize] = p + 1;
                face[3 as usize] = p + (*img).width + 1;
                face[4 as usize] = p + (*img).width;
                fwrite(
                    face.as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<[i32; 5]>() as u64,
                    1,
                    fout,
                );
                i_0 += 1;
                p += 1;
            }
            j_0 += 1;
        }
        fclose(fout);
    }
}

#[no_mangle]
pub extern "C" fn heman_export_with_colors_ply(
    mut hmap: *mut heman_image,
    mut colors: *mut heman_image,
    mut filename: *const i8,
) {
    unsafe {
        let mut width = (*hmap).width;
        let mut height = (*hmap).height;
        if (*hmap).nbands == 1 {
        } else {
            __assert_fail (b"hmap->nbands == 1\0" as * const u8 as * const i8, b"../src/export.c\0" as * const u8 as * const i8, 57, (* :: std :: mem :: transmute :: < & [u8; 78], & [i8; 78] > (
              b"void heman_export_with_colors_ply(heman_image *, heman_image *, const char *)\0",)).as_ptr (),);
        }
        if (*colors).nbands == 3 {
        } else {
            __assert_fail (b"colors->nbands == 3\0" as * const u8 as * const i8, b"../src/export.c\0" as * const u8 as * const i8, 58, (* :: std :: mem :: transmute :: < & [u8; 78], & [i8; 78] > (
              b"void heman_export_with_colors_ply(heman_image *, heman_image *, const char *)\0",)).as_ptr (),);
        }
        if (*colors).width == width {
        } else {
            __assert_fail (b"colors->width == width\0" as * const u8 as * const i8, b"../src/export.c\0" as * const u8 as * const i8, 59, (* :: std :: mem :: transmute :: < & [u8; 78], & [i8; 78] > (
              b"void heman_export_with_colors_ply(heman_image *, heman_image *, const char *)\0",)).as_ptr (),);
        }
        if (*colors).height == height {
        } else {
            __assert_fail (b"colors->height == height\0" as * const u8 as * const i8, b"../src/export.c\0" as * const u8 as * const i8, 60, (* :: std :: mem :: transmute :: < & [u8; 78], & [i8; 78] > (
              b"void heman_export_with_colors_ply(heman_image *, heman_image *, const char *)\0",)).as_ptr (),);
        }
        let mut fout = fopen(filename, b"wb\0" as *const u8 as *const i8);
        let mut ncols = (*hmap).width - 1;
        let mut nrows = (*hmap).height - 1;
        let mut ncells = ncols * nrows;
        let mut nverts = (*hmap).width * (*hmap).height;
        let mut colordata = malloc((width * height * 3i32) as u64) as *mut u8;
        heman_export_u8(
            colors,
            0.0f64 as libc::c_float,
            1.0f64 as libc::c_float,
            colordata,
        );
        fprintf (fout,
b"ply\nformat binary_little_endian 1.0\ncomment heman\nelement vertex %d\nproperty float32 x\nproperty float32 y\nproperty float32 z\nproperty uchar red\nproperty uchar green\nproperty uchar blue\nproperty uchar alpha\nelement face %d\nproperty list int32 int32 vertex_indices\nend_header\n\0"
         as * const u8 as * const i8, nverts, ncells,);
        let mut invw = 2.0f32 / width as libc::c_float;
        let mut invh = 2.0f32 / height as libc::c_float;
        let mut pcolor = colordata;
        let mut vert: [libc::c_float; 3] = [0.; 3];
        let mut j = 0;
        while j < height {
            let mut i = 0;
            while i < width {
                vert[0 as usize] = -1i32 as libc::c_float + i as libc::c_float * invw;
                vert[1 as usize] = -1i32 as libc::c_float + j as libc::c_float * invh;
                vert[2 as usize] = *heman_image_texel(hmap, i, j);
                fwrite(
                    vert.as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<[libc::c_float; 3]>() as u64,
                    1,
                    fout,
                );
                fwrite(pcolor as *const libc::c_void, 3, 1, fout);
                pcolor = pcolor.offset(3 as isize);
                fputc(255, fout);
                i += 1;
            }
            j += 1;
        }
        let mut face: [i32; 5] = [0; 5];
        face[0 as usize] = 4;
        let mut j_0 = 0;
        while j_0 < nrows {
            let mut p = j_0 * width;
            let mut i_0 = 0;
            while i_0 < ncols {
                face[1 as usize] = p;
                face[2 as usize] = p + 1;
                face[3 as usize] = p + (*hmap).width + 1;
                face[4 as usize] = p + (*hmap).width;
                fwrite(
                    face.as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<[i32; 5]>() as u64,
                    1,
                    fout,
                );
                i_0 += 1;
                p += 1;
            }
            j_0 += 1;
        }
        fclose(fout);
        free(colordata as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn heman_export_u8(
    mut source: *mut heman_image,
    mut minv: libc::c_float,
    mut maxv: libc::c_float,
    mut outp: *mut u8,
) {
    unsafe {
        let mut inp: *const libc::c_float = (*source).data;
        let mut scale = 1.0f32 / (maxv - minv);
        let mut size = (*source).height * (*source).width * (*source).nbands;
        let mut i = 0;
        while i < size {
            let fresh0 = inp;
            inp = inp.offset(1);
            let mut v = 255 as libc::c_float * (*fresh0 - minv) * scale;
            let fresh1 = outp;
            outp = outp.offset(1);
            *fresh1 = (if 0 as libc::c_float
                > (if 255 as libc::c_float > v {
                    v
                } else {
                    255 as libc::c_float
                }) {
                0 as libc::c_float
            } else if 255 as libc::c_float > v {
                v
            } else {
                255 as libc::c_float
            }) as u8;
            i += 1;
        }
    }
}
