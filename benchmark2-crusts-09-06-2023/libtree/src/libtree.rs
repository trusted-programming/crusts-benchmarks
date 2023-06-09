use libc;
extern "C" {
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn _IO_getc(__fp: *mut _IO_FILE) -> i32;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn fread(__ptr: *mut libc::c_void, __size: u64, __n: u64, __stream: *mut FILE) -> u64;
    fn fwrite(__ptr: *const libc::c_void, __size: u64, __n: u64, __s: *mut FILE) -> u64;
    fn fseek(__stream: *mut FILE, __off: i64, __whence: i32) -> i32;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn __ctype_b_loc() -> *mut *const u16;
    fn glob(
        __pattern: *const i8,
        __flags: i32,
        __errfunc: Option<unsafe extern "C" fn(*const i8, i32) -> i32>,
        __pglob: *mut glob_t,
    ) -> i32;
    fn globfree(__pglob: *mut glob_t);
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn uname(__name: *mut utsname) -> i32;
    fn isatty(__fd: i32) -> i32;
}
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
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: u64,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
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
pub struct glob_t {
    pub gl_pathc: u64,
    pub gl_pathv: *mut *mut i8,
    pub gl_offs: u64,
    pub gl_flags: i32,
    pub gl_closedir: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub gl_opendir: Option<unsafe extern "C" fn(*const i8) -> *mut libc::c_void>,
    pub gl_lstat: Option<unsafe extern "C" fn(*const i8, *mut libc::c_void) -> i32>,
    pub gl_stat: Option<unsafe extern "C" fn(*const i8, *mut libc::c_void) -> i32>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atimensec: u64,
    pub st_mtime: i64,
    pub st_mtimensec: u64,
    pub st_ctime: i64,
    pub st_ctimensec: u64,
    pub __glibc_reserved: [i64; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [i8; 65],
    pub nodename: [i8; 65],
    pub release: [i8; 65],
    pub version: [i8; 65],
    pub machine: [i8; 65],
    pub __domainname: [i8; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct header_64_t {
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct header_32_t {
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prog_64_t {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct prog_32_t {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dyn_64_t {
    pub d_tag: i64,
    pub d_val: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dyn_32_t {
    pub d_tag: i32,
    pub d_val: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct compat_t {
    pub any: i8,
    pub class: u8,
    pub machine: u16,
}
pub const DEFAULT: u32 = 6;
pub const LD_SO_CONF: u32 = 5;
pub const RUNPATH: u32 = 4;
pub const LD_LIBRARY_PATH: u32 = 3;
pub const RPATH: u32 = 2;
pub const DIRECT: u32 = 1;
pub const INPUT: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct found_t {
    pub how: u32,
    pub depth: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_table_t {
    pub arr: *mut i8,
    pub n: u64,
    pub capacity: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct visited_file_t {
    pub st_dev: u64,
    pub st_ino: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct visited_file_array_t {
    pub arr: *mut visited_file_t,
    pub n: u64,
    pub capacity: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libtree_state_t {
    pub verbosity: i32,
    pub path: i32,
    pub color: i32,
    pub ld_conf_file: *mut i8,
    pub max_depth: u64,
    pub string_table: string_table_t,
    pub visited: visited_file_array_t,
    pub PLATFORM: *mut i8,
    pub LIB: *mut i8,
    pub OSNAME: *mut i8,
    pub OSREL: *mut i8,
    pub rpath_offsets: [u64; 32],
    pub ld_library_path_offset: u64,
    pub default_paths_offset: u64,
    pub ld_so_conf_offset: u64,
    pub found_all_needed: [i8; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct small_vec_u64_t {
    pub buf: [u64; 16],
    pub p: *mut u64,
    pub n: u64,
    pub capacity: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub p64: prog_64_t,
    pub p32: prog_32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub h64: header_64_t,
    pub h32: header_32_t,
}
#[inline]
extern "C" fn putchar(mut __c: i32) -> i32 {
    unsafe {
        return _IO_putc(__c, stdout);
    }
}

#[inline]
extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    unsafe {
        return __xstat(1, __path, __statbuf);
    }
}

#[no_mangle]
pub static mut exclude_list: [*const i8; 14] = [
    b"ld-linux-aarch64.so\0" as *const u8 as *const i8,
    b"ld-linux-armhf.so\0" as *const u8 as *const i8,
    b"ld-linux-x86-64.so\0" as *const u8 as *const i8,
    b"ld-linux.so\0" as *const u8 as *const i8,
    b"ld64.so\0" as *const u8 as *const i8,
    b"libc.musl-aarch64.so\0" as *const u8 as *const i8,
    b"libc.musl-armhf.so\0" as *const u8 as *const i8,
    b"libc.musl-i386.so\0" as *const u8 as *const i8,
    b"libc.musl-x86_64.so\0" as *const u8 as *const i8,
    b"libc.so\0" as *const u8 as *const i8,
    b"libdl.so\0" as *const u8 as *const i8,
    b"libgcc_s.so\0" as *const u8 as *const i8,
    b"libm.so\0" as *const u8 as *const i8,
    b"libstdc++.so\0" as *const u8 as *const i8,
];
#[inline]
extern "C" fn utoa(mut str: *mut i8, mut v: u64) {
    unsafe {
        let mut p = str;
        loop {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = ('0' as u64).wrapping_add(v.wrapping_rem(10)) as i8;
            v = (v as u64).wrapping_div(10) as u64;
            if !(v > 0) {
                break;
            }
        }
        let mut len = p.offset_from(str) as u64;
        let mut i = 0;
        while i < len.wrapping_div(2) {
            let mut tmp = *str.offset(i as isize);
            *str.offset(i as isize) = *str.offset(len.wrapping_sub(i).wrapping_sub(1) as isize);
            *str.offset(len.wrapping_sub(i).wrapping_sub(1) as isize) = tmp;
            i = i.wrapping_add(1);
        }
        *str.offset(len as isize) = '\0' as i8;
    }
}

#[inline]
extern "C" fn small_vec_u64_init(mut v: *mut small_vec_u64_t) {
    unsafe {
        memset(
            v as *mut libc::c_void,
            0,
            ::std::mem::size_of::<small_vec_u64_t>() as u64,
        );
        let ref mut fresh1 = (*v).p;
        *fresh1 = ((*v).buf).as_mut_ptr();
    }
}

extern "C" fn small_vec_u64_append(mut v: *mut small_vec_u64_t, mut val: u64) {
    unsafe {
        if (*v).n < 16 {
            let ref mut fresh2 = (*v).n;
            let fresh3 = *fresh2;
            *fresh2 = (*fresh2).wrapping_add(1);
            *((*v).p).offset(fresh3 as isize) = val;
            return;
        }
        if (*v).n == 16 {
            (*v).capacity = (2 * 16i32) as u64;
            let ref mut fresh4 = (*v).p;
            *fresh4 = malloc(((*v).capacity).wrapping_mul(::std::mem::size_of::<u64>() as u64))
                as *mut u64;
            if ((*v).p).is_null() {
                exit(1);
            }
            memcpy(
                (*v).p as *mut libc::c_void,
                ((*v).buf).as_mut_ptr() as *const libc::c_void,
                16u64.wrapping_mul(::std::mem::size_of::<u64>() as u64),
            );
        } else if (*v).n == (*v).capacity {
            let ref mut fresh5 = (*v).capacity;
            *fresh5 = (*fresh5 as u64).wrapping_mul(2) as u64;
            let mut p = realloc(
                (*v).p as *mut libc::c_void,
                ((*v).capacity).wrapping_mul(::std::mem::size_of::<u64>() as u64),
            ) as *mut u64;
            if p.is_null() {
                exit(1);
            }
            let ref mut fresh6 = (*v).p;
            *fresh6 = p;
        }
        let ref mut fresh7 = (*v).n;
        let fresh8 = *fresh7;
        *fresh7 = (*fresh7).wrapping_add(1);
        *((*v).p).offset(fresh8 as isize) = val;
    }
}

extern "C" fn small_vec_u64_free(mut v: *mut small_vec_u64_t) {
    unsafe {
        if (*v).n <= 16 {
            return;
        }
        free((*v).p as *mut libc::c_void);
        let ref mut fresh9 = (*v).p;
        *fresh9 = 0 as *mut u64;
    }
}

#[inline]
extern "C" fn host_is_little_endian() -> i32 {
    let mut test = 1;
    let mut bytes = &mut test as *mut i32 as *mut i8;
    unsafe {
        return (*bytes.offset(0 as isize) as i32 == 1) as i32;
    }
}

extern "C" fn is_ascending_order(mut v: *mut u64, mut n: u64) -> i32 {
    unsafe {
        let mut j = 1;
        while j < n {
            if *v.offset(j.wrapping_sub(1) as isize) >= *v.offset(j as isize) {
                return 0;
            }
            j = j.wrapping_add(1);
        }
        return 1;
    }
}

extern "C" fn string_table_maybe_grow(mut t: *mut string_table_t, mut n: u64) {
    unsafe {
        if ((*t).n).wrapping_add(n) <= (*t).capacity {
            return;
        };
        (*t).capacity = 2u64.wrapping_mul(((*t).n).wrapping_add(n));
        let mut arr = realloc(
            (*t).arr as *mut libc::c_void,
            ((*t).capacity).wrapping_mul(::std::mem::size_of::<i8>() as u64),
        ) as *mut i8;
        if arr.is_null() {
            exit(1);
        }
        let ref mut fresh10 = (*t).arr;
        *fresh10 = arr;
    }
}

extern "C" fn string_table_store(mut t: *mut string_table_t, mut str: *const i8) {
    unsafe {
        let mut n = (strlen(str)).wrapping_add(1);
        string_table_maybe_grow(t, n);
        memcpy(
            ((*t).arr).offset((*t).n as isize) as *mut libc::c_void,
            str as *const libc::c_void,
            n,
        );
        let ref mut fresh11 = (*t).n;
        *fresh11 = (*fresh11 as u64).wrapping_add(n) as u64;
    }
}

extern "C" fn string_table_copy_from_file(mut t: *mut string_table_t, mut fptr: *mut FILE) {
    unsafe {
        let mut c: i32 = 0;
        loop {
            c = _IO_getc(fptr);
            if !(c != '\0' as i32 && c != -1) {
                break;
            }
            string_table_maybe_grow(t, 1);
            let ref mut fresh12 = (*t).n;
            let fresh13 = *fresh12;
            *fresh12 = (*fresh12).wrapping_add(1);
            *((*t).arr).offset(fresh13 as isize) = c as i8;
        }
        string_table_maybe_grow(t, 1);
        let ref mut fresh14 = (*t).n;
        let fresh15 = *fresh14;
        *fresh14 = (*fresh14).wrapping_add(1);
        *((*t).arr).offset(fresh15 as isize) = '\0' as i8;
    }
}

extern "C" fn is_in_exclude_list(mut soname: *mut i8) -> i32 {
    unsafe {
        let mut start = soname;
        let mut end = strrchr(start, '\0' as i32);
        if start == end {
            return 0;
        }
        end = end.offset(-1);
        while end != start
            && (*end as i32 >= '0' as i32 && *end as i32 <= '9' as i32 || *end as i32 == '.' as i32)
        {
            end = end.offset(-1);
        }
        let mut j = 0;
        while j
            < (::std::mem::size_of::<[*const i8; 14]>() as u64)
                .wrapping_div(::std::mem::size_of::<*mut i8>() as u64)
        {
            let mut len = strlen(exclude_list[j as usize]);
            if strncmp(start, exclude_list[j as usize], len) != 0 {
                j = j.wrapping_add(1);
            } else {
                return 1;
            }
        }
        return 0;
    }
}

extern "C" fn tree_preamble(mut s: *mut libtree_state_t, mut depth: u64) {
    unsafe {
        if depth == 0 {
            return;
        }
        let mut i = 0;
        while i < depth.wrapping_sub(1) {
            fputs(
                if (*s).found_all_needed[i as usize] as i32 != 0 {
                    b"    \0" as *const u8 as *const i8
                } else {
                    b"\xE2\x94\x82   \0" as *const u8 as *const i8
                },
                stdout,
            );
            i = i.wrapping_add(1);
        }
        fputs(
            if (*s).found_all_needed[depth.wrapping_sub(1) as usize] as i32 != 0 {
                b"\xE2\x94\x94\xE2\x94\x80\xE2\x94\x80 \0" as *const u8 as *const i8
            } else {
                b"\xE2\x94\x9C\xE2\x94\x80\xE2\x94\x80 \0" as *const u8 as *const i8
            },
            stdout,
        );
    }
}

extern "C" fn apply_exclude_list(
    mut needed_not_found: *mut u64,
    mut needed_buf_offsets: *mut small_vec_u64_t,
    mut s: *mut libtree_state_t,
) {
    unsafe {
        let mut i = 0;
        while i < *needed_not_found {
            if is_in_exclude_list(
                ((*s).string_table.arr)
                    .offset(*((*needed_buf_offsets).p).offset(i as isize) as isize),
            ) != 0
            {
                let mut tmp = *((*needed_buf_offsets).p).offset(i as isize);
                *((*needed_buf_offsets).p).offset(i as isize) =
                    *((*needed_buf_offsets).p).offset((*needed_not_found).wrapping_sub(1) as isize);
                *needed_not_found = (*needed_not_found).wrapping_sub(1);
                *((*needed_buf_offsets).p).offset(*needed_not_found as isize) = tmp;
            } else {
                i = i.wrapping_add(1);
            }
        }
    }
}

extern "C" fn check_absolute_paths(
    mut needed_not_found: *mut u64,
    mut needed_buf_offsets: *mut small_vec_u64_t,
    mut depth: u64,
    mut s: *mut libtree_state_t,
    mut compat: compat_t,
) -> i32 {
    unsafe {
        let mut exit_code = 0;
        let mut i = 0;
        while i < *needed_not_found {
            let mut st: *const string_table_t = &mut (*s).string_table;
            if (strchr(
                ((*st).arr).offset(*((*needed_buf_offsets).p).offset(i as isize) as isize),
                '/' as i32,
            ))
            .is_null()
            {
                i = i.wrapping_add(1);
            } else {
                let mut path: [i8; 4096] = [0; 4096];
                let mut len = strlen(
                    ((*st).arr).offset(*((*needed_buf_offsets).p).offset(i as isize) as isize),
                );
                if len >= 4096 {
                    continue;
                }
                memcpy(
                    path.as_mut_ptr() as *mut libc::c_void,
                    ((*st).arr).offset(*((*needed_buf_offsets).p).offset(i as isize) as isize)
                        as *const libc::c_void,
                    len.wrapping_add(1),
                );
                (*s).found_all_needed[depth as usize] = (*needed_not_found <= 1) as i8;
                let mut err = 0 as *mut i8;
                if path[0 as usize] as i32 != '/' as i32 {
                    err = b" is not absolute\0" as *const u8 as *const i8 as *mut i8;
                    exit_code = 28;
                } else {
                    let mut code = recurse(path.as_mut_ptr(), depth.wrapping_add(1), s, compat, {
                        let mut init = found_t {
                            how: DIRECT,
                            depth: 0,
                        };
                        init
                    });
                    if code == 28 {
                        exit_code = 28;
                    }
                    if code != 0 && code != 28 {
                        err = b" not found\0" as *const u8 as *const i8 as *mut i8;
                    }
                }
                if !err.is_null() {
                    tree_preamble(s, depth.wrapping_add(1));
                    if (*s).color != 0 {
                        fputs(b"\x1B[1;31m\0" as *const u8 as *const i8, stdout);
                    }
                    fputs(path.as_mut_ptr(), stdout);
                    fputs(b" is not absolute\0" as *const u8 as *const i8, stdout);
                    fputs(
                        if (*s).color != 0 {
                            b"\x1B[0m\n\0" as *const u8 as *const i8
                        } else {
                            b"\n\0" as *const u8 as *const i8
                        },
                        stdout,
                    );
                }
                let mut tmp = *((*needed_buf_offsets).p).offset(i as isize);
                *((*needed_buf_offsets).p).offset(i as isize) =
                    *((*needed_buf_offsets).p).offset((*needed_not_found).wrapping_sub(1) as isize);
                *needed_not_found = (*needed_not_found).wrapping_sub(1);
                *((*needed_buf_offsets).p).offset(*needed_not_found as isize) = tmp;
            }
        }
        return exit_code;
    }
}

extern "C" fn check_search_paths(
    mut reason: found_t,
    mut offset: u64,
    mut needed_not_found: *mut u64,
    mut needed_buf_offsets: *mut small_vec_u64_t,
    mut depth: u64,
    mut s: *mut libtree_state_t,
    mut compat: compat_t,
) -> i32 {
    unsafe {
        let mut exit_code = 0;
        let mut path: [i8; 4096] = [0; 4096];
        let mut path_end = path.as_mut_ptr().offset(4096 as isize);
        let mut st: *const string_table_t = &mut (*s).string_table;
        while *((*st).arr).offset(offset as isize) as i32 != '\0' as i32 {
            while *((*st).arr).offset(offset as isize) as i32 == ':' as i32
                && *((*st).arr).offset(offset as isize) as i32 != '\0' as i32
            {
                offset = offset.wrapping_add(1);
            }
            if *((*st).arr).offset(offset as isize) as i32 == '\0' as i32 {
                return exit_code;
            }
            let mut dest = path.as_mut_ptr();
            while *((*st).arr).offset(offset as isize) as i32 != '\0' as i32
                && *((*st).arr).offset(offset as isize) as i32 != ':' as i32
                && dest != path_end
            {
                let fresh16 = offset;
                offset = offset.wrapping_add(1);
                let fresh17 = dest;
                dest = dest.offset(1);
                *fresh17 = *((*st).arr).offset(fresh16 as isize);
            }
            if dest.offset(1 as isize) >= path_end {
                continue;
            }
            if *dest.offset(-(1 as isize)) as i32 != '/' as i32 {
                let fresh18 = dest;
                dest = dest.offset(1);
                *fresh18 = '/' as i8;
            }
            let mut search_path_end = dest;
            let mut i = 0;
            while i < *needed_not_found {
                let mut soname_len = strlen(
                    ((*st).arr).offset(*((*needed_buf_offsets).p).offset(i as isize) as isize),
                );
                if search_path_end
                    .offset(soname_len as isize)
                    .offset(1 as isize)
                    >= path_end
                {
                    continue;
                }
                memcpy(
                    search_path_end as *mut libc::c_void,
                    ((*st).arr).offset(*((*needed_buf_offsets).p).offset(i as isize) as isize)
                        as *const libc::c_void,
                    soname_len.wrapping_add(1),
                );
                (*s).found_all_needed[depth as usize] = (*needed_not_found <= 1) as i8;
                let mut code = recurse(path.as_mut_ptr(), depth.wrapping_add(1), s, compat, reason);
                if code == 28 {
                    exit_code = 28;
                }
                if code == 0 || code == 28 {
                    let mut tmp = *((*needed_buf_offsets).p).offset(i as isize);
                    *((*needed_buf_offsets).p).offset(i as isize) = *((*needed_buf_offsets).p)
                        .offset((*needed_not_found).wrapping_sub(1) as isize);
                    *needed_not_found = (*needed_not_found).wrapping_sub(1);
                    *((*needed_buf_offsets).p).offset(*needed_not_found as isize) = tmp;
                } else {
                    i = i.wrapping_add(1);
                }
            }
        }
        return exit_code;
    }
}

extern "C" fn interpolate_variables(
    mut s: *mut libtree_state_t,
    mut src: u64,
    mut ORIGIN: *const i8,
) -> i32 {
    unsafe {
        let mut prev_src = src;
        let mut curr_src = src;
        let mut st: *mut string_table_t = &mut (*s).string_table;
        loop {
            let mut dollar = strchr(((*st).arr).offset(curr_src as isize), '$' as i32);
            if dollar.is_null() {
                break;
            }
            curr_src = dollar.offset_from((*st).arr) as u64;
            let mut bytes_to_dollar = curr_src.wrapping_sub(prev_src);
            curr_src = curr_src.wrapping_add(1);
            let mut curly = 0;
            if *((*st).arr).offset(curr_src as isize) as i32 == '{' as i32 {
                curly = 1;
                curr_src = curr_src.wrapping_add(1);
            }
            let mut var_val = 0 as *const i8;
            if strncmp(
                &mut *((*st).arr).offset(curr_src as isize),
                b"ORIGIN\0" as *const u8 as *const i8,
                6,
            ) == 0
            {
                var_val = ORIGIN;
                curr_src = (curr_src as u64).wrapping_add(6) as u64;
            } else if strncmp(
                &mut *((*st).arr).offset(curr_src as isize),
                b"LIB\0" as *const u8 as *const i8,
                3,
            ) == 0
            {
                var_val = (*s).LIB;
                curr_src = (curr_src as u64).wrapping_add(3) as u64;
            } else if strncmp(
                &mut *((*st).arr).offset(curr_src as isize),
                b"PLATFORM\0" as *const u8 as *const i8,
                8,
            ) == 0
            {
                var_val = (*s).PLATFORM;
                curr_src = (curr_src as u64).wrapping_add(8) as u64;
            } else if strncmp(
                &mut *((*st).arr).offset(curr_src as isize),
                b"OSNAME\0" as *const u8 as *const i8,
                6,
            ) == 0
            {
                var_val = (*s).OSNAME;
                curr_src = (curr_src as u64).wrapping_add(6) as u64;
            } else {
                if !(strncmp(
                    &mut *((*st).arr).offset(curr_src as isize),
                    b"OSREL\0" as *const u8 as *const i8,
                    5,
                ) == 0)
                {
                    continue;
                }
                var_val = (*s).OSREL;
                curr_src = (curr_src as u64).wrapping_add(5) as u64;
            }
            if curly != 0 {
                if *((*st).arr).offset(curr_src as isize) as i32 != '}' as i32 {
                    continue;
                }
                curr_src = curr_src.wrapping_add(1);
            }
            let mut var_len = strlen(var_val);
            string_table_maybe_grow(st, bytes_to_dollar.wrapping_add(var_len));
            memcpy(
                &mut *((*st).arr).offset((*s).string_table.n as isize) as *mut i8
                    as *mut libc::c_void,
                &mut *((*st).arr).offset(prev_src as isize) as *mut i8 as *const libc::c_void,
                bytes_to_dollar,
            );
            let ref mut fresh19 = (*s).string_table.n;
            *fresh19 = (*fresh19 as u64).wrapping_add(bytes_to_dollar) as u64;
            prev_src = curr_src;
            memcpy(
                &mut *((*st).arr).offset((*s).string_table.n as isize) as *mut i8
                    as *mut libc::c_void,
                var_val as *const libc::c_void,
                var_len,
            );
            let ref mut fresh20 = (*s).string_table.n;
            *fresh20 = (*fresh20 as u64).wrapping_add(var_len) as u64;
        }
        if prev_src != src {
            let mut n = (strlen(((*st).arr).offset(prev_src as isize))).wrapping_add(1);
            string_table_maybe_grow(st, n);
            memcpy(
                ((*st).arr).offset((*st).n as isize) as *mut libc::c_void,
                ((*st).arr).offset(prev_src as isize) as *const libc::c_void,
                n,
            );
            let ref mut fresh21 = (*st).n;
            *fresh21 = (*fresh21 as u64).wrapping_add(n) as u64;
            return 1;
        }
        return 0;
    }
}

extern "C" fn print_colon_delimited_paths(mut start: *const i8, mut indent: *const i8) {
    unsafe {
        while !(*start as i32 == '\0' as i32) {
            let mut next = strchr(start, ':' as i32);
            if start == next {
                start = start.offset(1);
            } else {
                fputs(indent, stdout);
                fputs(b"    \0" as *const u8 as *const i8, stdout);
                if next.is_null() {
                    puts(start);
                } else {
                    fwrite(
                        start as *const libc::c_void,
                        1,
                        next.offset_from(start) as u64,
                        stdout,
                    );
                    print!("{}", '\n' as i32);
                }
                if next.is_null() {
                    break;
                }
                start = next.offset(1 as isize);
            }
        }
    }
}

extern "C" fn print_line(
    mut depth: u64,
    mut name: *mut i8,
    mut color_bold: *mut i8,
    mut color_regular: *mut i8,
    mut highlight: i32,
    mut reason: found_t,
    mut s: *mut libtree_state_t,
) {
    unsafe {
        tree_preamble(s, depth);
        let mut slash = 0 as *mut i8;
        if (*s).color != 0 && highlight != 0 && {
            slash = strrchr(name, '/' as i32);
            !slash.is_null()
        } {
            fputs(color_regular, stdout);
            fwrite(
                name as *const libc::c_void,
                1,
                slash.offset(1 as isize).offset_from(name) as u64,
                stdout,
            );
            fputs(color_bold, stdout);
            fputs(slash.offset(1 as isize), stdout);
        } else {
            if (*s).color != 0 {
                fputs(color_bold, stdout);
            }
            fputs(name, stdout);
        }
        if (*s).color != 0 && highlight != 0 {
            fputs(b"\x1B[0m \x1B[33m\0" as *const u8 as *const i8, stdout);
        } else {
            print!("{}", ' ' as i32);
        }
        let mut conf_name: *mut i8 = 0 as *mut i8;
        match reason.how as u32 {
            2 => {
                if (reason.depth).wrapping_add(1) >= depth {
                    fputs(b"[rpath]\0" as *const u8 as *const i8, stdout);
                } else {
                    let mut num: [i8; 8] = [0; 8];
                    utoa(num.as_mut_ptr(), (reason.depth).wrapping_add(1));
                    fputs(b"[rpath of \0" as *const u8 as *const i8, stdout);
                    fputs(num.as_mut_ptr(), stdout);
                    print!("{}", ']' as i32);
                }
            }
            3 => {
                fputs(b"[LD_LIBRARY_PATH]\0" as *const u8 as *const i8, stdout);
            }
            4 => {
                fputs(b"[runpath]\0" as *const u8 as *const i8, stdout);
            }
            5 => {
                print!("{}", '[' as i32);
                conf_name = strrchr((*s).ld_conf_file, '/' as i32);
                conf_name = if conf_name.is_null() {
                    (*s).ld_conf_file
                } else {
                    conf_name.offset(1 as isize)
                };
                fputs(conf_name, stdout);
                print!("{}", ']' as i32);
            }
            1 => {
                fputs(b"[direct]\0" as *const u8 as *const i8, stdout);
            }
            6 => {
                fputs(b"[default path]\0" as *const u8 as *const i8, stdout);
            }
            _ => {}
        }
        if (*s).color != 0 {
            fputs(b"\x1B[0m\n\0" as *const u8 as *const i8, stdout);
        } else {
            print!("{}", '\n' as i32);
        };
    }
}

extern "C" fn print_error(
    mut depth: u64,
    mut needed_not_found: u64,
    mut needed_buf_offsets: *mut small_vec_u64_t,
    mut runpath: *mut i8,
    mut s: *mut libtree_state_t,
    mut no_def_lib: i32,
) {
    unsafe {
        let mut i = 0;
        while i < needed_not_found {
            (*s).found_all_needed[depth as usize] = (i.wrapping_add(1) >= needed_not_found) as i8;
            tree_preamble(s, depth.wrapping_add(1));
            if (*s).color != 0 {
                fputs(b"\x1B[1;31m\0" as *const u8 as *const i8, stdout);
            }
            fputs(
                ((*s).string_table.arr)
                    .offset(*((*needed_buf_offsets).p).offset(i as isize) as isize),
                stdout,
            );
            fputs(b" not found\n\0" as *const u8 as *const i8, stdout);
            if (*s).color != 0 {
                fputs(b"\x1B[0m\0" as *const u8 as *const i8, stdout);
            }
            i = i.wrapping_add(1);
        }
        let mut box_vertical = (if (*s).color != 0 {
            b"    \x1B[0;31m\xE2\x94\x8A\x1B[0m\0" as *const u8 as *const i8
        } else {
            b"    \xE2\x94\x8A\0" as *const u8 as *const i8
        }) as *mut i8;
        let mut indent = malloc(
            (::std::mem::size_of::<[i8; 7]>() as u64)
                .wrapping_mul(depth)
                .wrapping_add(strlen(box_vertical))
                .wrapping_add(1),
        ) as *mut i8;
        let mut p = indent;
        let mut i_0 = 0;
        while i_0 < depth {
            if (*s).found_all_needed[i_0 as usize] != 0 {
                let mut len = (::std::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1) as i32;
                memcpy(
                    p as *mut libc::c_void,
                    b"    \0" as *const u8 as *const i8 as *const libc::c_void,
                    len as u64,
                );
                p = p.offset(len as isize);
            } else {
                let mut len_0 = (::std::mem::size_of::<[i8; 7]>() as u64).wrapping_sub(1) as i32;
                memcpy(
                    p as *mut libc::c_void,
                    b"\xE2\x94\x82   \0" as *const u8 as *const i8 as *const libc::c_void,
                    len_0 as u64,
                );
                p = p.offset(len_0 as isize);
            }
            i_0 = i_0.wrapping_add(1);
        }
        strcpy(p, box_vertical);
        fputs(indent, stdout);
        if (*s).color != 0 {
            fputs(b"\x1B[0;90m\0" as *const u8 as *const i8, stdout);
        }
        fputs(
            b" Paths considered in this order:\n\0" as *const u8 as *const i8,
            stdout,
        );
        if (*s).color != 0 {
            fputs(b"\x1B[0m\0" as *const u8 as *const i8, stdout);
        }
        fputs(indent, stdout);
        if !runpath.is_null() {
            if (*s).color != 0 {
                fputs(b"\x1B[0;90m\0" as *const u8 as *const i8, stdout);
            }
            fputs(
                b" 1. rpath is skipped because runpath was set\n\0" as *const u8 as *const i8,
                stdout,
            );
            if (*s).color != 0 {
                fputs(b"\x1B[0m\0" as *const u8 as *const i8, stdout);
            }
        } else {
            if (*s).color != 0 {
                fputs(b"\x1B[0;90m\0" as *const u8 as *const i8, stdout);
            }
            fputs(b" 1. rpath:\n\0" as *const u8 as *const i8, stdout);
            if (*s).color != 0 {
                fputs(b"\x1B[0m\0" as *const u8 as *const i8, stdout);
            }
            let mut j = depth as i32;
            while j >= 0 {
                if (*s).rpath_offsets[j as usize] != 18446744073709551615 {
                    let mut num: [i8; 8] = [0; 8];
                    utoa(num.as_mut_ptr(), (j + 1i32) as u64);
                    fputs(indent, stdout);
                    if (*s).color != 0 {
                        fputs(b"\x1B[0;90m\0" as *const u8 as *const i8, stdout);
                    }
                    fputs(b"    depth \0" as *const u8 as *const i8, stdout);
                    fputs(num.as_mut_ptr(), stdout);
                    if (*s).color != 0 {
                        fputs(b"\x1B[0m\0" as *const u8 as *const i8, stdout);
                    }
                    print!("{}", '\n' as i32);
                    print_colon_delimited_paths(
                        ((*s).string_table.arr).offset((*s).rpath_offsets[j as usize] as isize),
                        indent,
                    );
                }
                j -= 1;
            }
        }
        fputs(indent, stdout);
        if (*s).color != 0 {
            fputs(b"\x1B[0;90m\0" as *const u8 as *const i8, stdout);
        }
        fputs(
            if (*s).ld_library_path_offset == 18446744073709551615 {
                b" 2. LD_LIBRARY_PATH was not set\n\0" as *const u8 as *const i8
            } else {
                b" 2. LD_LIBRARY_PATH:\n\0" as *const u8 as *const i8
            },
            stdout,
        );
        if (*s).color != 0 {
            fputs(b"\x1B[0m\0" as *const u8 as *const i8, stdout);
        }
        if (*s).ld_library_path_offset != 18446744073709551615 {
            print_colon_delimited_paths(
                ((*s).string_table.arr).offset((*s).ld_library_path_offset as isize),
                indent,
            );
        }
        fputs(indent, stdout);
        if (*s).color != 0 {
            fputs(b"\x1B[0;90m\0" as *const u8 as *const i8, stdout);
        }
        fputs(
            if runpath.is_null() {
                b" 3. runpath was not set\n\0" as *const u8 as *const i8
            } else {
                b" 3. runpath:\n\0" as *const u8 as *const i8
            },
            stdout,
        );
        if (*s).color != 0 {
            fputs(b"\x1B[0m\0" as *const u8 as *const i8, stdout);
        }
        if !runpath.is_null() {
            print_colon_delimited_paths(runpath, indent);
        }
        fputs(indent, stdout);
        if (*s).color != 0 {
            fputs(b"\x1B[0;90m\0" as *const u8 as *const i8, stdout);
        }
        fputs(
            if no_def_lib != 0 {
                b" 4. ld config files not considered due to NODEFLIB flag\n\0" as *const u8
                    as *const i8
            } else {
                b" 4. ld config files:\n\0" as *const u8 as *const i8
            },
            stdout,
        );
        if (*s).color != 0 {
            fputs(b"\x1B[0m\0" as *const u8 as *const i8, stdout);
        }
        print_colon_delimited_paths(
            ((*s).string_table.arr).offset((*s).ld_so_conf_offset as isize),
            indent,
        );
        fputs(indent, stdout);
        if (*s).color != 0 {
            fputs(b"\x1B[0;90m\0" as *const u8 as *const i8, stdout);
        }
        fputs(
            if no_def_lib != 0 {
                b" 5. Standard paths not considered due to NODEFLIB flag\n\0" as *const u8
                    as *const i8
            } else {
                b" 5. Standard paths:\n\0" as *const u8 as *const i8
            },
            stdout,
        );
        if (*s).color != 0 {
            fputs(b"\x1B[0m\0" as *const u8 as *const i8, stdout);
        }
        print_colon_delimited_paths(
            ((*s).string_table.arr).offset((*s).default_paths_offset as isize),
            indent,
        );
        free(indent as *mut libc::c_void);
    }
}

extern "C" fn visited_files_contains(
    mut files: *mut visited_file_array_t,
    mut needle: *mut stat,
) -> i32 {
    unsafe {
        let mut i = 0;
        while i < (*files).n {
            let mut f: *mut visited_file_t =
                &mut *((*files).arr).offset(i as isize) as *mut visited_file_t;
            if (*f).st_dev == (*needle).st_dev && (*f).st_ino == (*needle).st_ino {
                return 1;
            }
            i = i.wrapping_add(1);
        }
        return 0;
    }
}

extern "C" fn visited_files_append(mut files: *mut visited_file_array_t, mut new: *mut stat) {
    unsafe {
        if (*files).n == (*files).capacity {
            let ref mut fresh22 = (*files).capacity;
            *fresh22 = (*fresh22 as u64).wrapping_mul(2) as u64;
            let ref mut fresh23 = (*files).arr;
            *fresh23 = realloc(
                (*files).arr as *mut libc::c_void,
                ((*files).capacity).wrapping_mul(::std::mem::size_of::<visited_file_t>() as u64),
            ) as *mut visited_file_t;
            if ((*files).arr).is_null() {
                exit(1);
            }
        };
        (*((*files).arr).offset((*files).n as isize)).st_dev = (*new).st_dev;
        (*((*files).arr).offset((*files).n as isize)).st_ino = (*new).st_ino;
        let ref mut fresh24 = (*files).n;
        *fresh24 = (*fresh24).wrapping_add(1);
    }
}

extern "C" fn recurse(
    mut current_file: *mut i8,
    mut depth: u64,
    mut s: *mut libtree_state_t,
    mut compat: compat_t,
    mut reason: found_t,
) -> i32 {
    unsafe {
        let mut fptr = fopen(current_file, b"rb\0" as *const u8 as *const i8);
        if fptr.is_null() {
            return 31;
        }
        let mut old_buf_size = (*s).string_table.n;
        let mut e_ident: [i8; 16] = [0; 16];
        if fread(
            &mut e_ident as *mut [i8; 16] as *mut libc::c_void,
            16,
            1,
            fptr,
        ) != 1
        {
            fclose(fptr);
            return 11;
        }
        if e_ident[0 as usize] as i32 != 0x7f
            || e_ident[1 as usize] as i32 != 'E' as i32
            || e_ident[2 as usize] as i32 != 'L' as i32
            || e_ident[3 as usize] as i32 != 'F' as i32
        {
            fclose(fptr);
            return 11;
        }
        if e_ident[4 as usize] as i32 != 1 && e_ident[4 as usize] as i32 != 2 {
            fclose(fptr);
            return 12;
        }
        if e_ident[5 as usize] as i32 != '\u{1}' as i32
            && e_ident[5 as usize] as i32 != '\u{2}' as i32
        {
            fclose(fptr);
            return 13;
        }
        let mut curr_type = {
            let mut init = compat_t {
                any: 0,
                class: e_ident[4 as usize] as u8,
                machine: 0,
            };
            init
        };
        let mut is_little_endian = (e_ident[5 as usize] as i32 == '\u{1}' as i32) as i32;
        if compat.any == 0 && compat.class as i32 != curr_type.class as i32 {
            fclose(fptr);
            return 15;
        }
        if is_little_endian ^ host_is_little_endian() != 0 {
            fclose(fptr);
            return 16;
        }
        let mut header = C2RustUnnamed_1 {
            h64: header_64_t {
                e_type: 0,
                e_machine: 0,
                e_version: 0,
                e_entry: 0,
                e_phoff: 0,
                e_shoff: 0,
                e_flags: 0,
                e_ehsize: 0,
                e_phentsize: 0,
                e_phnum: 0,
                e_shentsize: 0,
                e_shnum: 0,
                e_shstrndx: 0,
            },
        };
        if curr_type.class as i32 == 2 {
            if fread(
                &mut header.h64 as *mut header_64_t as *mut libc::c_void,
                ::std::mem::size_of::<header_64_t>() as u64,
                1,
                fptr,
            ) != 1
            {
                fclose(fptr);
                return 14;
            }
            if header.h64.e_type as i32 != 2 && header.h64.e_type as i32 != 3 {
                fclose(fptr);
                return 17;
            }
            curr_type.machine = header.h64.e_machine;
            if compat.any == 0 && compat.machine as i32 != curr_type.machine as i32 {
                fclose(fptr);
                return 32;
            }
            if fseek(fptr, header.h64.e_phoff as i64, 0) != 0 {
                fclose(fptr);
                return 18;
            }
        } else {
            if fread(
                &mut header.h32 as *mut header_32_t as *mut libc::c_void,
                ::std::mem::size_of::<header_32_t>() as u64,
                1,
                fptr,
            ) != 1
            {
                fclose(fptr);
                return 14;
            }
            if header.h32.e_type as i32 != 2 && header.h32.e_type as i32 != 3 {
                fclose(fptr);
                return 17;
            }
            curr_type.machine = header.h32.e_machine;
            if compat.any == 0 && compat.machine as i32 != curr_type.machine as i32 {
                fclose(fptr);
                return 32;
            }
            if fseek(fptr, header.h32.e_phoff as i64, 0) != 0 {
                fclose(fptr);
                return 18;
            }
        }
        let mut prog = C2RustUnnamed_0 {
            p64: prog_64_t {
                p_type: 0,
                p_flags: 0,
                p_offset: 0,
                p_vaddr: 0,
                p_paddr: 0,
                p_filesz: 0,
                p_memsz: 0,
                p_align: 0,
            },
        };
        let mut pt_load_offset = small_vec_u64_t {
            buf: [0; 16],
            p: 0 as *mut u64,
            n: 0,
            capacity: 0,
        };
        let mut pt_load_vaddr = small_vec_u64_t {
            buf: [0; 16],
            p: 0 as *mut u64,
            n: 0,
            capacity: 0,
        };
        small_vec_u64_init(&mut pt_load_offset);
        small_vec_u64_init(&mut pt_load_vaddr);
        let mut p_offset = 0xffffffffffffffff;
        if curr_type.class as i32 == 2 {
            let mut i = 0;
            while i < header.h64.e_phnum as u64 {
                if fread(
                    &mut prog.p64 as *mut prog_64_t as *mut libc::c_void,
                    ::std::mem::size_of::<prog_64_t>() as u64,
                    1,
                    fptr,
                ) != 1
                {
                    fclose(fptr);
                    small_vec_u64_free(&mut pt_load_offset);
                    small_vec_u64_free(&mut pt_load_vaddr);
                    return 19;
                }
                if prog.p64.p_type == 1 {
                    small_vec_u64_append(&mut pt_load_offset, prog.p64.p_offset);
                    small_vec_u64_append(&mut pt_load_vaddr, prog.p64.p_vaddr);
                } else if prog.p64.p_type == 2 {
                    p_offset = prog.p64.p_offset;
                }
                i = i.wrapping_add(1);
            }
        } else {
            let mut i_0 = 0;
            while i_0 < header.h32.e_phnum as u32 {
                if fread(
                    &mut prog.p32 as *mut prog_32_t as *mut libc::c_void,
                    ::std::mem::size_of::<prog_32_t>() as u64,
                    1,
                    fptr,
                ) != 1
                {
                    fclose(fptr);
                    small_vec_u64_free(&mut pt_load_offset);
                    small_vec_u64_free(&mut pt_load_vaddr);
                    return 19;
                }
                if prog.p32.p_type == 1 {
                    small_vec_u64_append(&mut pt_load_offset, prog.p32.p_offset as u64);
                    small_vec_u64_append(&mut pt_load_vaddr, prog.p32.p_vaddr as u64);
                } else if prog.p32.p_type == 2 {
                    p_offset = prog.p32.p_offset as u64;
                }
                i_0 = i_0.wrapping_add(1);
            }
        }
        let mut finfo = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atime: 0,
            st_atimensec: 0,
            st_mtime: 0,
            st_mtimensec: 0,
            st_ctime: 0,
            st_ctimensec: 0,
            __glibc_reserved: [0; 3],
        };
        if stat(current_file, &mut finfo) != 0 {
            fclose(fptr);
            small_vec_u64_free(&mut pt_load_offset);
            small_vec_u64_free(&mut pt_load_vaddr);
            return 20;
        }
        let mut seen_before = visited_files_contains(&mut (*s).visited, &mut finfo);
        if seen_before == 0 {
            visited_files_append(&mut (*s).visited, &mut finfo);
        }
        if p_offset == 0xffffffffffffffff {
            print_line(
                depth,
                current_file,
                b"\x1B[1;36m\0" as *const u8 as *const i8 as *mut i8,
                b"\x1B[0;36m\0" as *const u8 as *const i8 as *mut i8,
                1,
                reason,
                s,
            );
            fclose(fptr);
            small_vec_u64_free(&mut pt_load_offset);
            small_vec_u64_free(&mut pt_load_vaddr);
            return 0;
        }
        if pt_load_offset.n == 0 {
            fclose(fptr);
            small_vec_u64_free(&mut pt_load_offset);
            small_vec_u64_free(&mut pt_load_vaddr);
            return 29;
        }
        if fseek(fptr, p_offset as i64, 0) != 0 {
            fclose(fptr);
            small_vec_u64_free(&mut pt_load_offset);
            small_vec_u64_free(&mut pt_load_vaddr);
            return 21;
        }
        let mut no_def_lib = 0;
        let mut strtab = 0xffffffffffffffff;
        let mut rpath = 0xffffffffffffffff;
        let mut runpath = 0xffffffffffffffff;
        let mut soname = 0xffffffffffffffff;
        let mut needed = small_vec_u64_t {
            buf: [0; 16],
            p: 0 as *mut u64,
            n: 0,
            capacity: 0,
        };
        small_vec_u64_init(&mut needed);
        let mut cont = 1;
        while cont != 0 {
            let mut d_tag: u64 = 0;
            let mut d_val: u64 = 0;
            if curr_type.class as i32 == 2 {
                let mut dyn_0 = dyn_64_t { d_tag: 0, d_val: 0 };
                if fread(
                    &mut dyn_0 as *mut dyn_64_t as *mut libc::c_void,
                    ::std::mem::size_of::<dyn_64_t>() as u64,
                    1,
                    fptr,
                ) != 1
                {
                    fclose(fptr);
                    small_vec_u64_free(&mut pt_load_offset);
                    small_vec_u64_free(&mut pt_load_vaddr);
                    small_vec_u64_free(&mut needed);
                    return 22;
                }
                d_tag = dyn_0.d_tag as u64;
                d_val = dyn_0.d_val;
            } else {
                let mut dyn_1 = dyn_32_t { d_tag: 0, d_val: 0 };
                if fread(
                    &mut dyn_1 as *mut dyn_32_t as *mut libc::c_void,
                    ::std::mem::size_of::<dyn_32_t>() as u64,
                    1,
                    fptr,
                ) != 1
                {
                    fclose(fptr);
                    small_vec_u64_free(&mut pt_load_offset);
                    small_vec_u64_free(&mut pt_load_vaddr);
                    small_vec_u64_free(&mut needed);
                    return 22;
                }
                d_tag = dyn_1.d_tag as u64;
                d_val = dyn_1.d_val as u64;
            }
            match d_tag {
                0 => {
                    cont = 0;
                }
                5 => {
                    strtab = d_val;
                }
                15 => {
                    rpath = d_val;
                }
                29 => {
                    runpath = d_val;
                }
                1 => {
                    small_vec_u64_append(&mut needed, d_val);
                }
                14 => {
                    soname = d_val;
                }
                1879048187 => {
                    no_def_lib |= (0x800 & d_val == 0x800u64) as i32;
                }
                _ => {}
            }
        }
        if strtab == 0xffffffffffffffff {
            fclose(fptr);
            small_vec_u64_free(&mut pt_load_offset);
            small_vec_u64_free(&mut pt_load_vaddr);
            small_vec_u64_free(&mut needed);
            return 23;
        }
        if is_ascending_order(pt_load_vaddr.p, pt_load_vaddr.n) == 0 {
            fclose(fptr);
            small_vec_u64_free(&mut pt_load_vaddr);
            small_vec_u64_free(&mut pt_load_offset);
            small_vec_u64_free(&mut needed);
            return 30;
        }
        let mut vaddr_idx = 0;
        while vaddr_idx.wrapping_add(1) != pt_load_vaddr.n
            && strtab >= *(pt_load_vaddr.p).offset(vaddr_idx.wrapping_add(1) as isize)
        {
            vaddr_idx = vaddr_idx.wrapping_add(1);
        }
        let mut strtab_offset = (*(pt_load_offset.p).offset(vaddr_idx as isize))
            .wrapping_add(strtab)
            .wrapping_sub(*(pt_load_vaddr.p).offset(vaddr_idx as isize));
        small_vec_u64_free(&mut pt_load_vaddr);
        small_vec_u64_free(&mut pt_load_offset);
        let mut soname_buf_offset = (*s).string_table.n;
        if soname != 0xffffffffffffffff {
            if fseek(fptr, strtab_offset.wrapping_add(soname) as i64, 0) != 0 {
                (*s).string_table.n = old_buf_size;
                fclose(fptr);
                small_vec_u64_free(&mut needed);
                return 24;
            }
            string_table_copy_from_file(&mut (*s).string_table, fptr);
        }
        let mut in_exclude_list = (soname != 0xffffffffffffffff
            && is_in_exclude_list(((*s).string_table.arr).offset(soname_buf_offset as isize)) != 0)
            as i32;
        let mut should_recurse = (depth < (*s).max_depth
            && (seen_before == 0 && in_exclude_list == 0
                || seen_before == 0 && in_exclude_list != 0 && (*s).verbosity >= 2
                || (*s).verbosity >= 3)) as i32;
        if should_recurse == 0 {
            let mut print_name = if soname == 0xffffffffffffffff || (*s).path != 0 {
                current_file
            } else {
                ((*s).string_table.arr).offset(soname_buf_offset as isize)
            };
            let mut bold_color = (if in_exclude_list != 0 {
                b"\x1B[0;35m\0" as *const u8 as *const i8
            } else if seen_before != 0 {
                b"\x1B[0;34m\0" as *const u8 as *const i8
            } else {
                b"\x1B[1;36m\0" as *const u8 as *const i8
            }) as *mut i8;
            let mut regular_color = (if in_exclude_list != 0 {
                b"\x1B[0;35m\0" as *const u8 as *const i8
            } else if seen_before != 0 {
                b"\x1B[0;34m\0" as *const u8 as *const i8
            } else {
                b"\x1B[0;36m\0" as *const u8 as *const i8
            }) as *mut i8;
            let mut highlight = (seen_before == 0 && in_exclude_list == 0) as i32;
            print_line(
                depth,
                print_name,
                bold_color,
                regular_color,
                highlight,
                reason,
                s,
            );
            (*s).string_table.n = old_buf_size;
            fclose(fptr);
            small_vec_u64_free(&mut needed);
            return 0;
        }
        let mut origin: [i8; 4096] = [0; 4096];
        let mut last_slash = strrchr(current_file, '/' as i32);
        if !last_slash.is_null() {
            let mut bytes = last_slash.offset_from(current_file) as u64;
            memcpy(
                origin.as_mut_ptr() as *mut libc::c_void,
                current_file as *const libc::c_void,
                bytes,
            );
            origin[bytes as usize] = '\0' as i8;
        } else {
            memcpy(
                origin.as_mut_ptr() as *mut libc::c_void,
                b"./\0" as *const u8 as *const i8 as *const libc::c_void,
                3,
            );
        }
        if rpath == 0xffffffffffffffff {
            (*s).rpath_offsets[depth as usize] = 18446744073709551615;
        } else {
            (*s).rpath_offsets[depth as usize] = (*s).string_table.n;
            if fseek(fptr, strtab_offset.wrapping_add(rpath) as i64, 0) != 0 {
                (*s).string_table.n = old_buf_size;
                fclose(fptr);
                small_vec_u64_free(&mut needed);
                return 25;
            }
            string_table_copy_from_file(&mut (*s).string_table, fptr);
            let mut curr_buf_size = (*s).string_table.n;
            if interpolate_variables(s, (*s).rpath_offsets[depth as usize], origin.as_mut_ptr())
                != 0
            {
                (*s).rpath_offsets[depth as usize] = curr_buf_size;
            }
        }
        let mut runpath_buf_offset = (*s).string_table.n;
        if runpath != 0xffffffffffffffff {
            if fseek(fptr, strtab_offset.wrapping_add(runpath) as i64, 0) != 0 {
                (*s).string_table.n = old_buf_size;
                fclose(fptr);
                small_vec_u64_free(&mut needed);
                return 26;
            }
            string_table_copy_from_file(&mut (*s).string_table, fptr);
            let mut curr_buf_size_0 = (*s).string_table.n;
            if interpolate_variables(s, runpath_buf_offset, origin.as_mut_ptr()) != 0 {
                runpath_buf_offset = curr_buf_size_0;
            }
        }
        let mut needed_buf_offsets = small_vec_u64_t {
            buf: [0; 16],
            p: 0 as *mut u64,
            n: 0,
            capacity: 0,
        };
        small_vec_u64_init(&mut needed_buf_offsets);
        let mut i_1 = 0;
        while i_1 < needed.n {
            small_vec_u64_append(&mut needed_buf_offsets, (*s).string_table.n);
            if fseek(
                fptr,
                strtab_offset.wrapping_add(*(needed.p).offset(i_1 as isize)) as i64,
                0,
            ) != 0
            {
                (*s).string_table.n = old_buf_size;
                fclose(fptr);
                small_vec_u64_free(&mut needed_buf_offsets);
                small_vec_u64_free(&mut needed);
                return 27;
            }
            string_table_copy_from_file(&mut (*s).string_table, fptr);
            i_1 = i_1.wrapping_add(1);
        }
        fclose(fptr);
        let mut print_name_0 = if soname == 0xffffffffffffffff || (*s).path != 0 {
            current_file
        } else {
            ((*s).string_table.arr).offset(soname_buf_offset as isize)
        };
        let mut bold_color_0 = (if in_exclude_list != 0 {
            b"\x1B[0;35m\0" as *const u8 as *const i8
        } else if seen_before != 0 {
            b"\x1B[0;34m\0" as *const u8 as *const i8
        } else {
            b"\x1B[1;36m\0" as *const u8 as *const i8
        }) as *mut i8;
        let mut regular_color_0 = (if in_exclude_list != 0 {
            b"\x1B[0;35m\0" as *const u8 as *const i8
        } else if seen_before != 0 {
            b"\x1B[0;34m\0" as *const u8 as *const i8
        } else {
            b"\x1B[0;36m\0" as *const u8 as *const i8
        }) as *mut i8;
        let mut highlight_0 = (seen_before == 0 && in_exclude_list == 0) as i32;
        print_line(
            depth,
            print_name_0,
            bold_color_0,
            regular_color_0,
            highlight_0,
            reason,
            s,
        );
        let mut exit_code = 0;
        let mut needed_not_found = needed_buf_offsets.n;
        if needed_not_found != 0 && (*s).verbosity == 0 {
            apply_exclude_list(&mut needed_not_found, &mut needed_buf_offsets, s);
        }
        if needed_not_found != 0 {
            exit_code |= check_absolute_paths(
                &mut needed_not_found,
                &mut needed_buf_offsets,
                depth,
                s,
                curr_type,
            );
        }
        if runpath == 0xffffffffffffffff {
            let mut j = depth as i32;
            while j >= 0 && needed_not_found != 0 {
                if !((*s).rpath_offsets[j as usize] == 18446744073709551615) {
                    exit_code |= check_search_paths(
                        {
                            let mut init = found_t {
                                how: RPATH,
                                depth: j as u64,
                            };
                            init
                        },
                        (*s).rpath_offsets[j as usize],
                        &mut needed_not_found,
                        &mut needed_buf_offsets,
                        depth,
                        s,
                        curr_type,
                    );
                }
                j -= 1;
            }
        }
        if needed_not_found != 0 && (*s).ld_library_path_offset != 18446744073709551615 {
            exit_code |= check_search_paths(
                {
                    let mut init = found_t {
                        how: LD_LIBRARY_PATH,
                        depth: 0,
                    };
                    init
                },
                (*s).ld_library_path_offset,
                &mut needed_not_found,
                &mut needed_buf_offsets,
                depth,
                s,
                curr_type,
            );
        }
        if needed_not_found != 0 && runpath != 0xffffffffffffffff {
            exit_code |= check_search_paths(
                {
                    let mut init = found_t {
                        how: RUNPATH,
                        depth: 0,
                    };
                    init
                },
                runpath_buf_offset,
                &mut needed_not_found,
                &mut needed_buf_offsets,
                depth,
                s,
                curr_type,
            );
        }
        if needed_not_found != 0 && no_def_lib == 0 {
            exit_code |= check_search_paths(
                {
                    let mut init = found_t {
                        how: LD_SO_CONF,
                        depth: 0,
                    };
                    init
                },
                (*s).ld_so_conf_offset,
                &mut needed_not_found,
                &mut needed_buf_offsets,
                depth,
                s,
                curr_type,
            );
        }
        if needed_not_found != 0 && no_def_lib == 0 {
            exit_code |= check_search_paths(
                {
                    let mut init = found_t {
                        how: DEFAULT,
                        depth: 0,
                    };
                    init
                },
                (*s).default_paths_offset,
                &mut needed_not_found,
                &mut needed_buf_offsets,
                depth,
                s,
                curr_type,
            );
        }
        if needed_not_found != 0 {
            print_error(
                depth,
                needed_not_found,
                &mut needed_buf_offsets,
                if runpath == 0xffffffffffffffff {
                    0 as *mut i8
                } else {
                    ((*s).string_table.arr).offset(runpath_buf_offset as isize)
                },
                s,
                no_def_lib,
            );
            (*s).string_table.n = old_buf_size;
            small_vec_u64_free(&mut needed_buf_offsets);
            small_vec_u64_free(&mut needed);
            return 28;
        };
        (*s).string_table.n = old_buf_size;
        small_vec_u64_free(&mut needed_buf_offsets);
        small_vec_u64_free(&mut needed);
        return exit_code;
    }
}

extern "C" fn ld_conf_globbing(mut st: *mut string_table_t, mut pattern: *mut i8) -> i32 {
    unsafe {
        let mut result = glob_t {
            gl_pathc: 0,
            gl_pathv: 0 as *mut *mut i8,
            gl_offs: 0,
            gl_flags: 0,
            gl_closedir: None,
            gl_readdir: None,
            gl_opendir: None,
            gl_lstat: None,
            gl_stat: None,
        };
        memset(
            &mut result as *mut glob_t as *mut libc::c_void,
            0,
            ::std::mem::size_of::<glob_t>() as u64,
        );
        let mut status = glob(pattern, 0, None, &mut result);
        match status {
            1 | 2 => {
                globfree(&mut result);
                return 1;
            }
            3 => {
                globfree(&mut result);
                return 0;
            }
            _ => {}
        }
        let mut code = 0;
        let mut i = 0;
        while i < result.gl_pathc {
            code |= parse_ld_config_file(st, *(result.gl_pathv).offset(i as isize));
            i = i.wrapping_add(1);
        }
        globfree(&mut result);
        return code;
    }
}

extern "C" fn parse_ld_config_file(mut st: *mut string_table_t, mut path: *mut i8) -> i32 {
    unsafe {
        let mut fptr = fopen(path, b"r\0" as *const u8 as *const i8);
        if fptr.is_null() {
            return 1;
        }
        let mut c = 0;
        let mut line: [i8; 4096] = [0; 4096];
        let mut tmp: [i8; 4096] = [0; 4096];
        while c != -1 {
            let mut line_len = 0;
            loop {
                c = _IO_getc(fptr);
                if !(c != '\n' as i32 && c != -1) {
                    break;
                }
                if line_len < (4096 - 1i32) as u64 {
                    let fresh25 = line_len;
                    line_len = line_len.wrapping_add(1);
                    line[fresh25 as usize] = c as i8;
                }
            }
            line[line_len as usize] = '\0' as i8;
            let mut begin = line.as_mut_ptr();
            let mut end = line.as_mut_ptr().offset(line_len as isize);
            while *(*__ctype_b_loc()).offset(*begin as i32 as isize) as i32 & _ISspace as i32 != 0 {
                begin = begin.offset(1);
            }
            let mut comment = strchr(begin, '#' as i32);
            if !comment.is_null() {
                *comment = '\0' as i8;
            }
            while end != begin {
                end = end.offset(-1);
                if *(*__ctype_b_loc()).offset(*end as i32 as isize) as i32 & _ISspace as i32 == 0 {
                    break;
                }
            }
            if begin == end {
                continue;
            };
            *end.offset(1 as isize) = '\0' as i8;
            if strncmp(begin, b"include\0" as *const u8 as *const i8, 7) == 0
                && *(*__ctype_b_loc()).offset(*begin.offset(7 as isize) as i32 as isize) as i32
                    & _ISspace as i32
                    != 0
            {
                begin = begin.offset(8 as isize);
                while *(*__ctype_b_loc()).offset(*begin as i32 as isize) as i32 & _ISspace as i32
                    != 0
                {
                    begin = begin.offset(1);
                }
                if *begin as i32 != '/' as i32 {
                    let mut wd = strrchr(path, '/' as i32);
                    wd = if wd.is_null() {
                        strrchr(path, '\0' as i32)
                    } else {
                        wd
                    };
                    let mut wd_len = wd.offset_from(path) as u64;
                    let mut include_len = (end.offset_from(begin) as i64 + 1i64) as u64;
                    if wd_len.wrapping_add(1).wrapping_add(include_len) >= 4096 {
                        continue;
                    }
                    memcpy(
                        tmp.as_mut_ptr() as *mut libc::c_void,
                        path as *const libc::c_void,
                        wd_len,
                    );
                    tmp[wd_len as usize] = '/' as i8;
                    memcpy(
                        tmp.as_mut_ptr().offset(wd_len as isize).offset(1 as isize)
                            as *mut libc::c_void,
                        begin as *const libc::c_void,
                        include_len,
                    );
                    tmp[wd_len.wrapping_add(1).wrapping_add(include_len) as usize] = '\0' as i8;
                    begin = tmp.as_mut_ptr();
                }
                ld_conf_globbing(st, begin);
            } else {
                string_table_store(st, begin);
                *((*st).arr).offset(((*st).n).wrapping_sub(1) as isize) = ':' as i8;
            }
        }
        fclose(fptr);
        return 0;
    }
}

extern "C" fn parse_ld_so_conf(mut s: *mut libtree_state_t) {
    unsafe {
        let mut st: *mut string_table_t = &mut (*s).string_table;
        (*s).ld_so_conf_offset = (*st).n;
        parse_ld_config_file(st, (*s).ld_conf_file);
        if (*st).n > (*s).ld_so_conf_offset {
            *((*st).arr).offset(((*st).n).wrapping_sub(1) as isize) = '\0' as i8;
        } else {
            string_table_store(st, b"\0" as *const u8 as *const i8);
        };
    }
}

extern "C" fn parse_ld_library_path(mut s: *mut libtree_state_t) {
    unsafe {
        (*s).ld_library_path_offset = 18446744073709551615;
        let mut val = getenv(b"LD_LIBRARY_PATH\0" as *const u8 as *const i8);
        if val.is_null() {
            return;
        };
        (*s).ld_library_path_offset = (*s).string_table.n;
        string_table_store(&mut (*s).string_table, val);
        let mut search = ((*s).string_table.arr).offset((*s).ld_library_path_offset as isize);
        loop {
            search = strchr(search, ';' as i32);
            if search.is_null() {
                break;
            }
            let fresh26 = search;
            search = search.offset(1);
            *fresh26 = ':' as i8;
        }
    }
}

extern "C" fn set_default_paths(mut s: *mut libtree_state_t) {
    unsafe {
        (*s).default_paths_offset = (*s).string_table.n;
        string_table_store(
            &mut (*s).string_table,
            b"/lib:/lib64:/usr/lib:/usr/lib64\0" as *const u8 as *const i8,
        );
    }
}

extern "C" fn libtree_state_init(mut s: *mut libtree_state_t) {
    unsafe {
        (*s).string_table.n = 0;
        (*s).string_table.capacity = 1024;
        let ref mut fresh27 = (*s).string_table.arr;
        *fresh27 =
            malloc(((*s).string_table.capacity).wrapping_mul(::std::mem::size_of::<i8>() as u64))
                as *mut i8;
        (*s).visited.n = 0;
        (*s).visited.capacity = 256;
        let ref mut fresh28 = (*s).visited.arr;
        *fresh28 = malloc(
            ((*s).visited.capacity).wrapping_mul(::std::mem::size_of::<visited_file_t>() as u64),
        ) as *mut visited_file_t;
    }
}

extern "C" fn libtree_state_free(mut s: *mut libtree_state_t) {
    unsafe {
        free((*s).string_table.arr as *mut libc::c_void);
        free((*s).visited.arr as *mut libc::c_void);
    }
}

extern "C" fn print_tree(
    mut pathc: i32,
    mut pathv: *mut *mut i8,
    mut s: *mut libtree_state_t,
) -> i32 {
    unsafe {
        libtree_state_init(s);
        parse_ld_so_conf(s);
        parse_ld_library_path(s);
        set_default_paths(s);
        let mut exit_code = 0;
        let mut i = 0;
        while i < pathc {
            let mut code = recurse(
                *pathv.offset(i as isize),
                0,
                s,
                {
                    let mut init = compat_t {
                        any: 1,
                        class: 0,
                        machine: 0,
                    };
                    init
                },
                {
                    let mut init = found_t {
                        how: INPUT,
                        depth: 0,
                    };
                    init
                },
            );
            fflush(stdout);
            if code != 0 {
                exit_code = code;
                fputs(b"Error [\0" as *const u8 as *const i8, stderr);
                fputs(*pathv.offset(i as isize), stderr);
                fputs(b"]: \0" as *const u8 as *const i8, stderr);
            }
            let mut msg = 0 as *mut i8;
            match code {
                11 => {
                    msg = b"Invalid ELF magic bytes\n\0" as *const u8 as *const i8 as *mut i8;
                }
                12 => {
                    msg = b"Invalid ELF class\n\0" as *const u8 as *const i8 as *mut i8;
                }
                13 => {
                    msg = b"Invalid ELF data\n\0" as *const u8 as *const i8 as *mut i8;
                }
                14 => {
                    msg = b"Invalid ELF header\n\0" as *const u8 as *const i8 as *mut i8;
                }
                15 => {
                    msg = b"Invalid bits\n\0" as *const u8 as *const i8 as *mut i8;
                }
                16 => {
                    msg = b"Invalid endianness\n\0" as *const u8 as *const i8 as *mut i8;
                }
                17 => {
                    msg = b"Not an ET_EXEC or ET_DYN ELF file\n\0" as *const u8 as *const i8
                        as *mut i8;
                }
                18 => {
                    msg = b"Invalid ELF program header offset\n\0" as *const u8 as *const i8
                        as *mut i8;
                }
                19 => {
                    msg = b"Invalid ELF program header\n\0" as *const u8 as *const i8 as *mut i8;
                }
                20 => {
                    msg = b"Can't stat file\n\0" as *const u8 as *const i8 as *mut i8;
                }
                21 => {
                    msg = b"Invalid ELF dynamic section\n\0" as *const u8 as *const i8 as *mut i8;
                }
                22 => {
                    msg =
                        b"Invalid ELF dynamic array entry\n\0" as *const u8 as *const i8 as *mut i8;
                }
                23 => {
                    msg = b"No ELF string table found\n\0" as *const u8 as *const i8 as *mut i8;
                }
                24 => {
                    msg = b"Can't read DT_SONAME\n\0" as *const u8 as *const i8 as *mut i8;
                }
                25 => {
                    msg = b"Can't read DT_RPATH\n\0" as *const u8 as *const i8 as *mut i8;
                }
                26 => {
                    msg = b"Can't read DT_RUNPATH\n\0" as *const u8 as *const i8 as *mut i8;
                }
                27 => {
                    msg = b"Can't read DT_NEEDED\n\0" as *const u8 as *const i8 as *mut i8;
                }
                28 => {
                    msg =
                        b"Not all dependencies were found\n\0" as *const u8 as *const i8 as *mut i8;
                }
                29 => {
                    msg = b"No PT_LOAD found in ELF file\n\0" as *const u8 as *const i8 as *mut i8;
                }
                30 => {
                    msg = b"Virtual addresses are not ordered\n\0" as *const u8 as *const i8
                        as *mut i8;
                }
                31 => {
                    msg = b"Could not open file\n\0" as *const u8 as *const i8 as *mut i8;
                }
                32 => {
                    msg = b"Incompatible ISA\n\0" as *const u8 as *const i8 as *mut i8;
                }
                _ => {}
            }
            if !msg.is_null() {
                fputs(msg, stderr);
            }
            fflush(stderr);
            i += 1;
        }
        libtree_state_free(s);
        return exit_code;
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut s = libtree_state_t {
            verbosity: 0,
            path: 0,
            color: 0,
            ld_conf_file: 0 as *mut i8,
            max_depth: 0,
            string_table: string_table_t {
                arr: 0 as *mut i8,
                n: 0,
                capacity: 0,
            },
            visited: visited_file_array_t {
                arr: 0 as *mut visited_file_t,
                n: 0,
                capacity: 0,
            },
            PLATFORM: 0 as *mut i8,
            LIB: 0 as *mut i8,
            OSNAME: 0 as *mut i8,
            OSREL: 0 as *mut i8,
            rpath_offsets: [0; 32],
            ld_library_path_offset: 0,
            default_paths_offset: 0,
            ld_so_conf_offset: 0,
            found_all_needed: [0; 32],
        };
        s.color =
            ((getenv(b"NO_COLOR\0" as *const u8 as *const i8)).is_null() && isatty(1) != 0) as i32;
        s.verbosity = 0;
        s.path = 0;
        s.max_depth = 32;
        let mut positional = 1;
        let mut uname_val = utsname {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            __domainname: [0; 65],
        };
        if uname(&mut uname_val) != 0 {
            return 1;
        }
        s.PLATFORM = (uname_val.machine).as_mut_ptr();
        s.OSNAME = (uname_val.sysname).as_mut_ptr();
        s.OSREL = (uname_val.release).as_mut_ptr();
        s.ld_conf_file = b"/etc/ld.so.conf\0" as *const u8 as *const i8 as *mut i8;
        if strcmp(
            (uname_val.sysname).as_mut_ptr(),
            b"FreeBSD\0" as *const u8 as *const i8,
        ) == 0
        {
            s.ld_conf_file = b"/etc/ld-elf.so.conf\0" as *const u8 as *const i8 as *mut i8;
        }
        s.LIB = b"lib\0" as *const u8 as *const i8 as *mut i8;
        let mut opt_help = 0;
        let mut opt_version = 0;
        let mut opt_raw = 0;
        let mut i = 1;
        while i < argc {
            let mut arg = *argv.offset(i as isize);
            if opt_raw != 0
                || *arg as i32 != '-' as i32
                || *arg.offset(1 as isize) as i32 == '\0' as i32
            {
                let fresh29 = positional;
                positional = positional + 1;
                let ref mut fresh30 = *argv.offset(fresh29 as isize);
                *fresh30 = arg;
            } else {
                arg = arg.offset(1);
                if *arg as i32 == '-' as i32 {
                    arg = arg.offset(1);
                    if *arg as i32 == '\0' as i32 {
                        opt_raw = 1;
                    } else if strcmp(arg, b"version\0" as *const u8 as *const i8) == 0 {
                        opt_version = 1;
                    } else if strcmp(arg, b"path\0" as *const u8 as *const i8) == 0 {
                        s.path = 1;
                    } else if strcmp(arg, b"verbose\0" as *const u8 as *const i8) == 0 {
                        s.verbosity += 1;
                    } else if strcmp(arg, b"help\0" as *const u8 as *const i8) == 0 {
                        opt_help = 1;
                    } else if strcmp(arg, b"ldconf\0" as *const u8 as *const i8) == 0 {
                        if i + 1 == argc {
                            fputs(
                                b"Expected value after `--ldconf`\n\0" as *const u8 as *const i8,
                                stderr,
                            );
                            return 1;
                        }
                        i += 1;
                        s.ld_conf_file = *argv.offset(i as isize);
                    } else if strcmp(arg, b"max-depth\0" as *const u8 as *const i8) == 0 {
                        if i + 1 == argc {
                            fputs(
                                b"Expected value after `--max-depth`\n\0" as *const u8 as *const i8,
                                stderr,
                            );
                            return 1;
                        }
                        let mut ptr = 0 as *mut i8;
                        i += 1;
                        s.max_depth = strtoul(*argv.offset(i as isize), &mut ptr, 10);
                        if s.max_depth > 32 {
                            s.max_depth = 32;
                        }
                    } else {
                        fputs(b"Unrecognized flag `--\0" as *const u8 as *const i8, stderr);
                        fputs(arg, stderr);
                        fputs(b"`\n\0" as *const u8 as *const i8, stderr);
                        return 1;
                    }
                } else {
                    while *arg as i32 != '\0' as i32 {
                        match *arg as i32 {
                            104 => {
                                opt_help = 1;
                            }
                            112 => {
                                s.path = 1;
                            }
                            118 => {
                                s.verbosity += 1;
                            }
                            _ => {
                                fputs(b"Unrecognized flag `-\0" as *const u8 as *const i8, stderr);
                                fputs(arg, stderr);
                                fputs(b"`\n\0" as *const u8 as *const i8, stderr);
                                return 1;
                            }
                        }
                        arg = arg.offset(1);
                    }
                }
            }
            i += 1;
        }
        argv = argv.offset(1);
        positional -= 1;
        if opt_help != 0 || opt_version == 0 && positional == 0 {
            fputs (
b"Show the dynamic dependency tree of ELF files\nUsage: libtree [OPTION]... [--] FILE [FILES]...\n\n  -h, --help     Print help info\n      --version  Print version info\n\nFile names starting with '-', for example '-.so', can be specified as follows:\n  libtree -- -.so\n\nLocating libs options:\n  -p, --path       Show the path of libraries instead of the soname\n  -v               Show libraries skipped by default*\n  -vv              Show dependencies of libraries skipped by default*\n  -vvv             Show dependencies of already encountered libraries\n  --ldconf <path>  Config file for extra search paths [\0"
            as * const u8 as * const i8, stdout,);
            fputs(s.ld_conf_file, stdout);
            fputs (b"]\n  --max-depth <n>  Limit library traversal to at most n levels of depth\n\n* For brevity, the following libraries are not shown by default:\n  \0" as * const u8 as * const i8, stdout,);
            let mut num_excluded = (::std::mem::size_of::<[*const i8; 14]>() as u64)
                .wrapping_div(::std::mem::size_of::<*mut i8>() as u64);
            let mut cursor_x = 3;
            let mut j = 0;
            while j < num_excluded {
                cursor_x = (cursor_x as u64).wrapping_add(strlen(exclude_list[j as usize])) as u64;
                if cursor_x > 60 {
                    cursor_x = 3;
                    fputs(b"\n  \0" as *const u8 as *const i8, stdout);
                }
                fputs(exclude_list[j as usize], stdout);
                if j.wrapping_add(1) != num_excluded {
                    fputs(b", \0" as *const u8 as *const i8, stdout);
                }
                j = j.wrapping_add(1);
            }
            fputs(
                b".\n\nThe following rpath/runpath substitutions are used:\n\0" as *const u8
                    as *const i8,
                stdout,
            );
            fputs(b"  PLATFORM       \0" as *const u8 as *const i8, stdout);
            fputs(s.PLATFORM, stdout);
            fputs(b"\n  LIB            \0" as *const u8 as *const i8, stdout);
            fputs(s.LIB, stdout);
            fputs(b"\n  OSNAME         \0" as *const u8 as *const i8, stdout);
            fputs(s.OSNAME, stdout);
            fputs(b"\n  OSREL          \0" as *const u8 as *const i8, stdout);
            fputs(s.OSREL, stdout);
            print!("{}", '\n' as i32);
            return (opt_help == 0) as i32;
        }
        if opt_version != 0 {
            puts(b"3.1.1\0" as *const u8 as *const i8);
            return 0;
        }
        return print_tree(positional, argv, &mut s);
    }
}
