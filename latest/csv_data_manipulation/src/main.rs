#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
fn build_str_from_raw_ptr(raw_ptr: *mut u8) -> String {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut str_size: usize = 0;
        while *raw_ptr.add(str_size) != 0 {
            str_size = str_size.wrapping_add(1);
        }
        return std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw_ptr, str_size))
            .to_owned();
    }
}


extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strdup(_: *const i8) -> *mut i8;
    fn strtok(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn __ctype_b_loc() -> *mut *const u16;
    fn __errno_location() -> *mut i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
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
pub type FILE = _IO_FILE;
pub const _ISalnum: u32 = 8;
pub const _ISpunct: u32 = 4;
pub const _IScntrl: u32 = 2;
pub const _ISblank: u32 = 1;
pub const _ISgraph: u32 = 32768;
pub const _ISprint: u32 = 16384;
pub const _ISspace: u32 = 8192;
pub const _ISxdigit: u32 = 4096;
pub const _ISdigit: u32 = 2048;
pub const _ISalpha: u32 = 1024;
pub const _ISlower: u32 = 512;
pub const _ISupper: u32 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
#[derive(Debug)]
pub struct CSV {
    pub delim: *mut i8,
    pub rows: u32,
    pub cols: u32,
    pub table: *mut *mut i8,
}
#[no_mangle]
pub extern "C" fn trim(mut str: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut trimmed: i32 = 0;
        let mut n: i32 = 0;
        let mut len: i32 = 0;
        len = strlen(*str) as i32;
        n = len.wrapping_sub(1);
        while n >= 0_i32
            && i32::from(*(*__ctype_b_loc()).offset(i32::from(*(*str).offset(n as isize)) as isize))
                & _ISspace as i32
                != 0_i32
        {
            *(*str).offset(n as isize) = '\0' as i8;
            trimmed = trimmed.wrapping_add(1);
            n = n.wrapping_sub(1);
            n;
        }
        n = 0_i32;
        while n < len
            && i32::from(*(*__ctype_b_loc()).offset(i32::from(*(*str).offset(0_isize)) as isize))
                & _ISspace as i32
                != 0_i32
        {
            *(*str).offset(0_isize) = '\0' as i8;
            *str = (*str).offset(1_isize);
            trimmed = trimmed.wrapping_add(1);
            n = n.wrapping_add(1);
            n;
        }
        trimmed
    }
}

#[no_mangle]
pub extern "C" fn csv_destroy(mut csv: *mut CSV) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        if csv.is_null() {
            return 0_i32;
        }
        if !((*csv).table).is_null() {
            free((*csv).table.cast::<libc::c_void>());
        }
        if !((*csv).delim).is_null() {
            free((*csv).delim.cast::<libc::c_void>());
        }
        free(csv.cast::<libc::c_void>());
        0_i32
    }
}

#[no_mangle]
pub extern "C" fn csv_create(mut cols: u32, mut rows: u32) -> *mut CSV {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut csv: *mut CSV = std::ptr::null_mut::<CSV>();
        csv = malloc(::core::mem::size_of::<CSV>() as u64).cast::<CSV>();
        (*csv).rows = rows;
        (*csv).cols = cols;
        (*csv).delim = strdup((b",\0" as *const u8).cast::<i8>());
        (*csv).table = malloc(
            (::core::mem::size_of::<*mut i8>() as u64)
                .wrapping_mul(u64::from(cols))
                .wrapping_mul(u64::from(rows)),
        ).cast::<*mut i8>();
        if ((*csv).table).is_null() {
            csv_destroy(csv);
            std::ptr::null_mut::<CSV>()
        } else {
            memset(
                (*csv).table.cast::<libc::c_void>(),
                0,
                (::core::mem::size_of::<*mut i8>() as u64)
                    .wrapping_mul(u64::from(cols))
                    .wrapping_mul(u64::from(rows)),
            );
            csv
        }
    }
}

#[no_mangle]
pub extern "C" fn csv_get(mut csv: *mut CSV, mut col: u32, mut row: u32) -> *mut i8 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut idx: u32 = 0;
        idx = col.wrapping_add(row.wrapping_mul((*csv).cols));
        *((*csv).table).offset(idx as isize)
    }
}

#[no_mangle]
pub extern "C" fn csv_set(
    mut csv: *mut CSV,
    mut col: u32,
    mut row: u32,
    mut value: *mut i8,
) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut idx: u32 = 0;
        idx = col.wrapping_add(row.wrapping_mul((*csv).cols));
        let fresh0 = &mut (*((*csv).table).offset(idx as isize));
        *fresh0 = value;
        0_i32
    }
}

#[no_mangle]
pub extern "C" fn csv_display(mut csv: *mut CSV) {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut row: i32 = 0;
        let mut col: i32 = 0;
        let mut content: *mut i8 = std::ptr::null_mut::<i8>();
        if (*csv).rows == 0 || (*csv).cols == 0 {
            println!("[Empty table]");
            return;
        }
        print!("\n[Table cols={} rows={}]\n", (*csv).cols, (*csv).rows);
        row = 0_i32;
        while (row as u32) < (*csv).rows {
            print!("[|");
            col = 0_i32;
            while (col as u32) < (*csv).cols {
                content = csv_get(csv, col as u32, row as u32);
                print!("{}	|", build_str_from_raw_ptr(content.cast::<u8>()));
                col = col.wrapping_add(1);
                col;
            }
            println!("]");
            row = row.wrapping_add(1);
            row;
        }
        println!();
    }
}

#[no_mangle]
pub extern "C" fn csv_resize(mut old_csv: *mut CSV, mut new_cols: u32, mut new_rows: u32) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut cur_col: u32 = 0;
        let mut cur_row: u32 = 0;
        let mut max_cols: u32 = 0;
        let mut max_rows: u32 = 0;
        let mut new_csv: *mut CSV = std::ptr::null_mut::<CSV>();
        let mut content: *mut i8 = std::ptr::null_mut::<i8>();
        let mut in_old: i32 = 0;
        let mut in_new: i32 = 0;
        new_csv = csv_create(new_cols, new_rows);
        if new_csv.is_null() {
            println!(
                "Unable to resize CSV table: error {} - {}",
                *__errno_location(),
                build_str_from_raw_ptr(strerror(*__errno_location()).cast::<u8>())
            );
            -1_i32
        } else {
            (*new_csv).rows = new_rows;
            (*new_csv).cols = new_cols;
            max_cols = if new_cols > (*old_csv).cols {
                new_cols
            } else {
                (*old_csv).cols
            };
            max_rows = if new_rows > (*old_csv).rows {
                new_rows
            } else {
                (*old_csv).rows
            };
            cur_col = 0;
            while cur_col < max_cols {
                cur_row = 0;
                while cur_row < max_rows {
                    in_old = i32::from(cur_col < (*old_csv).cols && cur_row < (*old_csv).rows);
                    in_new = i32::from(cur_col < (*new_csv).cols && cur_row < (*new_csv).rows);
                    if in_old != 0_i32 && in_new != 0_i32 {
                        content = csv_get(old_csv, cur_col, cur_row);
                        csv_set(new_csv, cur_col, cur_row, content);
                    } else if in_old != 0_i32 {
                        content = csv_get(old_csv, cur_col, cur_row);
                        free(content.cast::<libc::c_void>());
                    }
                    cur_row = cur_row.wrapping_add(1);
                    cur_row;
                }
                cur_col = cur_col.wrapping_add(1);
                cur_col;
            }
            free((*old_csv).table.cast::<libc::c_void>());
            (*old_csv).rows = new_rows;
            (*old_csv).cols = new_cols;
            (*old_csv).table = (*new_csv).table;
            (*new_csv).table = std::ptr::null_mut::<*mut i8>();
            csv_destroy(new_csv);
            0_i32
        }
    }
}

#[no_mangle]
pub extern "C" fn csv_open(mut csv: *mut CSV, mut filename: *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut fp: *mut FILE = std::ptr::null_mut::<FILE>();
        let mut m_rows: u32 = 0;
        let mut m_cols: u32 = 0;
        let mut cols: u32 = 0;
        let mut line: [i8; 2048] = [0; 2048];
        let mut lineptr: *mut i8 = std::ptr::null_mut::<i8>();
        let mut token: *mut i8 = std::ptr::null_mut::<i8>();
        fp = fopen(filename, (b"r\0" as *const u8).cast::<i8>());
        if fp.is_null() {
            fclose(fp);
            print!(
                "Unable to open {} for reading.",
                build_str_from_raw_ptr(filename.cast::<u8>())
            );
            -1_i32
        } else {
            m_rows = 0;
            m_cols = 0;
            while !(fgets(
                line.as_mut_ptr(),
                ::core::mem::size_of::<[i8; 2048]>() as i32,
                fp,
            ))
            .is_null()
            {
                m_rows = m_rows.wrapping_add(1);
                cols = 0;
                lineptr = line.as_mut_ptr();
                loop {
                    token = strtok(lineptr, (*csv).delim);
                    if token.is_null() {
                        break;
                    }
                    lineptr = std::ptr::null_mut::<i8>();
                    trim(&mut token);
                    cols = cols.wrapping_add(1);
                    if cols > m_cols {
                        m_cols = cols;
                    }
                    csv_resize(csv, m_cols, m_rows);
                    csv_set(
                        csv,
                        cols.wrapping_sub(1),
                        m_rows.wrapping_sub(1),
                        strdup(token),
                    );
                }
            }
            fclose(fp);
            (*csv).rows = m_rows;
            (*csv).cols = m_cols;
            0_i32
        }
    }
}

#[no_mangle]
pub extern "C" fn csv_save(mut csv: *mut CSV, mut filename: *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut fp: *mut FILE = std::ptr::null_mut::<FILE>();
        let mut row: i32 = 0;
        let mut col: i32 = 0;
        let mut content: *mut i8 = std::ptr::null_mut::<i8>();
        fp = fopen(filename, (b"w\0" as *const u8).cast::<i8>());
        row = 0_i32;
        while (row as u32) < (*csv).rows {
            col = 0_i32;
            while (col as u32) < (*csv).cols {
                content = csv_get(csv, col as u32, row as u32);
                fprintf(
                    fp,
                    (b"%s%s\0" as *const u8).cast::<i8>(),
                    content,
                    if col as u32 == ((*csv).cols).wrapping_sub(1) {
                        (b"\0" as *const u8).cast::<i8>()
                    } else {
                        (*csv).delim as *const i8
                    },
                );
                col = col.wrapping_add(1);
                col;
            }
            fprintf(fp, (b"\n\0" as *const u8).cast::<i8>());
            row = row.wrapping_add(1);
            row;
        }
        fclose(fp);
        0_i32
    }
}

fn main_0(mut _argc: i32, mut _argv: *mut *mut i8) -> i32 {
// SAFETY: machine generated unsafe code
    unsafe {
        let mut csv: *mut CSV = std::ptr::null_mut::<CSV>();
        print!(
            "CSV data manipulation\0\nhttp://rosettacode.org/wiki/CSV_data_manipulation\0\n\n"
        );
        csv = csv_create(0, 0);
        csv_open(csv, (b"input.csv\0" as *const u8).cast::<i8>() as *mut i8);
        csv_display(csv);
        csv_set(csv, 0, 0, (b"Column0\0" as *const u8).cast::<i8>() as *mut i8);
        csv_set(csv, 1, 1, (b"100\0" as *const u8).cast::<i8>() as *mut i8);
        csv_set(csv, 2, 2, (b"200\0" as *const u8).cast::<i8>() as *mut i8);
        csv_set(csv, 3, 3, (b"300\0" as *const u8).cast::<i8>() as *mut i8);
        csv_set(csv, 4, 4, (b"400\0" as *const u8).cast::<i8>() as *mut i8);
        csv_display(csv);
        csv_save(csv, (b"output.csv\0" as *const u8).cast::<i8>() as *mut i8);
        csv_destroy(csv);
        0_i32
    }
}

pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    ::std::process::exit(main_0((args.len() - 1) as i32, args.as_mut_ptr()));
}
