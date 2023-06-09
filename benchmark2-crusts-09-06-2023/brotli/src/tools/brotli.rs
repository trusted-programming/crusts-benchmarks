use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type BrotliDecoderStateStruct;
    pub type BrotliEncoderStateStruct;
    fn __errno_location() -> *mut i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fgetc(__stream: *mut FILE) -> i32;
    fn fread(_: *mut libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn fwrite(_: *const libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn fseek(__stream: *mut FILE, __off: i64, __whence: i32) -> i32;
    fn ftell(__stream: *mut FILE) -> i64;
    fn feof(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: i32) -> !;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn chmod(__file: *const i8, __mode: u32) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn BrotliDecoderSetParameter(state: *mut BrotliDecoderState, param: u32, value: u32) -> i32;
    fn BrotliDecoderCreateInstance(
        alloc_func: brotli_alloc_func,
        free_func: brotli_free_func,
        opaque: *mut libc::c_void,
    ) -> *mut BrotliDecoderState;
    fn BrotliDecoderDestroyInstance(state: *mut BrotliDecoderState);
    fn BrotliDecoderDecompressStream(
        state: *mut BrotliDecoderState,
        available_in: *mut u64,
        next_in: *mut *const u8,
        available_out: *mut u64,
        next_out: *mut *mut u8,
        total_out: *mut u64,
    ) -> u32;
    fn BrotliEncoderSetParameter(state: *mut BrotliEncoderState, param: u32, value: u32) -> i32;
    fn BrotliEncoderCreateInstance(
        alloc_func: brotli_alloc_func,
        free_func: brotli_free_func,
        opaque: *mut libc::c_void,
    ) -> *mut BrotliEncoderState;
    fn BrotliEncoderDestroyInstance(state: *mut BrotliEncoderState);
    fn BrotliEncoderCompressStream(
        state: *mut BrotliEncoderState,
        op: u32,
        available_in: *mut u64,
        next_in: *mut *const u8,
        available_out: *mut u64,
        next_out: *mut *mut u8,
        total_out: *mut u64,
    ) -> i32;
    fn BrotliEncoderIsFinished(state: *mut BrotliEncoderState) -> i32;
    fn chown(__file: *const i8, __owner: u32, __group: u32) -> i32;
    fn isatty(__fd: i32) -> i32;
    fn unlink(__name: *const i8) -> i32;
    fn utime(__file: *const i8, __file_times: *const utimbuf) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
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
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [i64; 3],
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
pub type brotli_alloc_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>;
pub type brotli_free_func =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>;
pub type BrotliDecoderState = BrotliDecoderStateStruct;
pub const BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT: u32 = 3;
pub const BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT: u32 = 2;
pub const BROTLI_DECODER_RESULT_SUCCESS: u32 = 1;
pub const BROTLI_DECODER_RESULT_ERROR: u32 = 0;
pub const BROTLI_DECODER_PARAM_LARGE_WINDOW: u32 = 1;
pub const BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION: u32 = 0;
pub const BROTLI_OPERATION_EMIT_METADATA: u32 = 3;
pub const BROTLI_OPERATION_FINISH: u32 = 2;
pub const BROTLI_OPERATION_FLUSH: u32 = 1;
pub const BROTLI_OPERATION_PROCESS: u32 = 0;
pub const BROTLI_PARAM_STREAM_OFFSET: u32 = 9;
pub const BROTLI_PARAM_NDIRECT: u32 = 8;
pub const BROTLI_PARAM_NPOSTFIX: u32 = 7;
pub const BROTLI_PARAM_LARGE_WINDOW: u32 = 6;
pub const BROTLI_PARAM_SIZE_HINT: u32 = 5;
pub const BROTLI_PARAM_DISABLE_LITERAL_CONTEXT_MODELING: u32 = 4;
pub const BROTLI_PARAM_LGBLOCK: u32 = 3;
pub const BROTLI_PARAM_LGWIN: u32 = 2;
pub const BROTLI_PARAM_QUALITY: u32 = 1;
pub const BROTLI_PARAM_MODE: u32 = 0;
pub type BrotliEncoderState = BrotliEncoderStateStruct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: i64,
    pub modtime: i64,
}
pub const COMMAND_VERSION: u32 = 6;
pub const COMMAND_NOOP: u32 = 5;
pub const COMMAND_TEST_INTEGRITY: u32 = 4;
pub const COMMAND_INVALID: u32 = 3;
pub const COMMAND_HELP: u32 = 2;
pub const COMMAND_DECOMPRESS: u32 = 1;
pub const COMMAND_COMPRESS: u32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Context {
    pub quality: i32,
    pub lgwin: i32,
    pub verbosity: i32,
    pub force_overwrite: i32,
    pub junk_source: i32,
    pub copy_stat: i32,
    pub write_to_stdout: i32,
    pub test_integrity: i32,
    pub decompress: i32,
    pub large_window: i32,
    pub output_path: *const i8,
    pub suffix: *const i8,
    pub not_input_indices: [i32; 20],
    pub longest_path_len: u64,
    pub input_count: u64,
    pub argc: i32,
    pub argv: *mut *mut i8,
    pub modified_path: *mut i8,
    pub iterator: i32,
    pub ignore: i32,
    pub iterator_error: i32,
    pub buffer: *mut u8,
    pub input: *mut u8,
    pub output: *mut u8,
    pub current_input_path: *const i8,
    pub current_output_path: *const i8,
    pub input_file_length: i64,
    pub fin: *mut FILE,
    pub fout: *mut FILE,
    pub available_in: u64,
    pub next_in: *const u8,
    pub available_out: u64,
    pub next_out: *mut u8,
    pub total_in: u64,
    pub total_out: u64,
}
#[inline]
extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    unsafe {
        return __xstat(1, __path, __statbuf);
    }
}

extern "C" fn ParseInt(mut s: *const i8, mut low: i32, mut high: i32, mut result: *mut i32) -> i32 {
    unsafe {
        let mut value = 0;
        let mut i: i32 = 0;
        i = 0;
        while i < 5 {
            let mut c = *s.offset(i as isize);
            if c as i32 == 0 {
                break;
            }
            if (*s.offset(i as isize) as i32) < '0' as i32
                || *s.offset(i as isize) as i32 > '9' as i32
            {
                return 0;
            }
            value = 10 * value + (c as i32 - '0' as i32);
            i += 1;
        }
        if i == 0 {
            return 0;
        }
        if i > 1 && *s.offset(0 as isize) as i32 == '0' as i32 {
            return 0;
        }
        if *s.offset(i as isize) as i32 != 0 {
            return 0;
        }
        if value < low || value > high {
            return 0;
        }
        *result = value;
        return 1;
    }
}

extern "C" fn FileName(mut path: *const i8) -> *const i8 {
    unsafe {
        let mut separator_position: *const i8 = strrchr(path, '/' as i32);
        if !separator_position.is_null() {
            path = separator_position.offset(1 as isize);
        }
        separator_position = strrchr(path, '\\' as i32);
        if !separator_position.is_null() {
            path = separator_position.offset(1 as isize);
        }
        return path;
    }
}

extern "C" fn ParseAlias(mut name: *const i8) -> u32 {
    unsafe {
        let mut unbrotli = b"unbrotli\0" as *const u8 as *const i8;
        let mut unbrotli_len = strlen(unbrotli);
        name = FileName(name);
        if strncmp(name, unbrotli, unbrotli_len) == 0 {
            let mut terminator = *name.offset(unbrotli_len as isize);
            if terminator as i32 == 0 || terminator as i32 == '.' as i32 {
                return COMMAND_DECOMPRESS;
            }
        }
        return COMMAND_COMPRESS;
    }
}

extern "C" fn ParseParams(mut params: *mut Context) -> u32 {
    unsafe {
        let mut argc = (*params).argc;
        let mut argv = (*params).argv;
        let mut i: i32 = 0;
        let mut next_option_index = 0;
        let mut input_count = 0;
        let mut longest_path_len = 1;
        let mut command_set = 0;
        let mut quality_set = 0;
        let mut output_set = 0;
        let mut keep_set = 0;
        let mut lgwin_set = 0;
        let mut suffix_set = 0;
        let mut after_dash_dash = 0;
        let mut command = ParseAlias(*argv.offset(0 as isize));
        i = 1;
        while i < argc {
            let mut arg: *const i8 = *argv.offset(i as isize);
            let mut arg_len = if !arg.is_null() { strlen(arg) } else { 0 };
            if arg_len == 0 {
                let fresh0 = next_option_index;
                next_option_index = next_option_index + 1;
                (*params).not_input_indices[fresh0 as usize] = i;
            } else {
                if next_option_index > 20 - 2 {
                    fprintf(
                        stderr,
                        b"too many options passed\n\0" as *const u8 as *const i8,
                    );
                    return COMMAND_INVALID;
                }
                if after_dash_dash != 0
                    || *arg.offset(0 as isize) as i32 != '-' as i32
                    || arg_len == 1
                {
                    input_count = input_count.wrapping_add(1);
                    if longest_path_len < arg_len {
                        longest_path_len = arg_len;
                    }
                } else {
                    let fresh1 = next_option_index;
                    next_option_index = next_option_index + 1;
                    (*params).not_input_indices[fresh1 as usize] = i;
                    if arg_len == 2 && *arg.offset(1 as isize) as i32 == '-' as i32 {
                        after_dash_dash = 1;
                    } else if *arg.offset(1 as isize) as i32 != '-' as i32 {
                        let mut j: u64 = 0;
                        j = 1;
                        while j < arg_len {
                            let mut c = *arg.offset(j as isize);
                            if c as i32 >= '0' as i32 && c as i32 <= '9' as i32 {
                                if quality_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"quality already set\n\0" as *const u8 as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                }
                                quality_set = 1;
                                (*params).quality = c as i32 - '0' as i32;
                            } else if c as i32 == 'c' as i32 {
                                if output_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"write to standard output already set\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                }
                                output_set = 1;
                                (*params).write_to_stdout = 1;
                            } else if c as i32 == 'd' as i32 {
                                if command_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"command already set when parsing -d\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                }
                                command_set = 1;
                                command = COMMAND_DECOMPRESS;
                            } else if c as i32 == 'f' as i32 {
                                if (*params).force_overwrite != 0 {
                                    fprintf(
                                        stderr,
                                        b"force output overwrite already set\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                };
                                (*params).force_overwrite = 1;
                            } else if c as i32 == 'h' as i32 {
                                return COMMAND_HELP;
                            } else if c as i32 == 'j' as i32 || c as i32 == 'k' as i32 {
                                if keep_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"argument --rm / -j or --keep / -k already set\n\0"
                                            as *const u8
                                            as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                }
                                keep_set = 1;
                                (*params).junk_source = if c as i32 == 'j' as i32 { 1 } else { 0 };
                            } else if c as i32 == 'n' as i32 {
                                if (*params).copy_stat == 0 {
                                    fprintf(
                                        stderr,
                                        b"argument --no-copy-stat / -n already set\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                };
                                (*params).copy_stat = 0;
                            } else if c as i32 == 't' as i32 {
                                if command_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"command already set when parsing -t\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                }
                                command_set = 1;
                                command = COMMAND_TEST_INTEGRITY;
                            } else if c as i32 == 'v' as i32 {
                                if (*params).verbosity > 0 {
                                    fprintf(
                                        stderr,
                                        b"argument --verbose / -v already set\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                };
                                (*params).verbosity = 1;
                            } else if c as i32 == 'V' as i32 {
                                return COMMAND_VERSION;
                            } else if c as i32 == 'Z' as i32 {
                                if quality_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"quality already set\n\0" as *const u8 as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                }
                                quality_set = 1;
                                (*params).quality = 11;
                            } else {
                                if c as i32 != 'o' as i32
                                    && c as i32 != 'q' as i32
                                    && c as i32 != 'w' as i32
                                    && c as i32 != 'D' as i32
                                    && c as i32 != 'S' as i32
                                {
                                    fprintf(
                                        stderr,
                                        b"invalid argument -%c\n\0" as *const u8 as *const i8,
                                        c as i32,
                                    );
                                    return COMMAND_INVALID;
                                }
                                if j.wrapping_add(1) != arg_len {
                                    fprintf(
                                        stderr,
                                        b"expected parameter for argument -%c\n\0" as *const u8
                                            as *const i8,
                                        c as i32,
                                    );
                                    return COMMAND_INVALID;
                                }
                                i += 1;
                                if i == argc
                                    || (*argv.offset(i as isize)).is_null()
                                    || *(*argv.offset(i as isize)).offset(0 as isize) as i32 == 0
                                {
                                    fprintf(
                                        stderr,
                                        b"expected parameter for argument -%c\n\0" as *const u8
                                            as *const i8,
                                        c as i32,
                                    );
                                    return COMMAND_INVALID;
                                }
                                let fresh2 = next_option_index;
                                next_option_index = next_option_index + 1;
                                (*params).not_input_indices[fresh2 as usize] = i;
                                if c as i32 == 'o' as i32 {
                                    if output_set != 0 {
                                        fprintf(
                                            stderr,
                                            b"write to standard output already set (-o)\n\0"
                                                as *const u8
                                                as *const i8,
                                        );
                                        return COMMAND_INVALID;
                                    }
                                    let ref mut fresh3 = (*params).output_path;
                                    *fresh3 = *argv.offset(i as isize);
                                } else if c as i32 == 'q' as i32 {
                                    if quality_set != 0 {
                                        fprintf(
                                            stderr,
                                            b"quality already set\n\0" as *const u8 as *const i8,
                                        );
                                        return COMMAND_INVALID;
                                    }
                                    quality_set = ParseInt(
                                        *argv.offset(i as isize),
                                        0,
                                        11,
                                        &mut (*params).quality,
                                    );
                                    if quality_set == 0 {
                                        fprintf(
                                            stderr,
                                            b"error parsing quality value [%s]\n\0" as *const u8
                                                as *const i8,
                                            *argv.offset(i as isize),
                                        );
                                        return COMMAND_INVALID;
                                    }
                                } else if c as i32 == 'w' as i32 {
                                    if lgwin_set != 0 {
                                        fprintf(
                                            stderr,
                                            b"lgwin parameter already set\n\0" as *const u8
                                                as *const i8,
                                        );
                                        return COMMAND_INVALID;
                                    }
                                    lgwin_set = ParseInt(
                                        *argv.offset(i as isize),
                                        0,
                                        24,
                                        &mut (*params).lgwin,
                                    );
                                    if lgwin_set == 0 {
                                        fprintf(
                                            stderr,
                                            b"error parsing lgwin value [%s]\n\0" as *const u8
                                                as *const i8,
                                            *argv.offset(i as isize),
                                        );
                                        return COMMAND_INVALID;
                                    }
                                    if (*params).lgwin != 0 && (*params).lgwin < 10 {
                                        fprintf (stderr, b"lgwin parameter (%d) smaller than the minimum (%d)\n\0" as * const u8 as * const i8, (* params).lgwin, 10,);
                                        return COMMAND_INVALID;
                                    }
                                } else if c as i32 == 'S' as i32 {
                                    if suffix_set != 0 {
                                        fprintf(
                                            stderr,
                                            b"suffix already set\n\0" as *const u8 as *const i8,
                                        );
                                        return COMMAND_INVALID;
                                    }
                                    suffix_set = 1;
                                    let ref mut fresh4 = (*params).suffix;
                                    *fresh4 = *argv.offset(i as isize);
                                }
                            }
                            j = j.wrapping_add(1);
                        }
                    } else {
                        arg = &*arg.offset(2 as isize) as *const i8;
                        if strcmp(b"best\0" as *const u8 as *const i8, arg) == 0 {
                            if quality_set != 0 {
                                fprintf(
                                    stderr,
                                    b"quality already set\n\0" as *const u8 as *const i8,
                                );
                                return COMMAND_INVALID;
                            }
                            quality_set = 1;
                            (*params).quality = 11;
                        } else if strcmp(b"decompress\0" as *const u8 as *const i8, arg) == 0 {
                            if command_set != 0 {
                                fprintf(
                                    stderr,
                                    b"command already set when parsing --decompress\n\0"
                                        as *const u8
                                        as *const i8,
                                );
                                return COMMAND_INVALID;
                            }
                            command_set = 1;
                            command = COMMAND_DECOMPRESS;
                        } else if strcmp(b"force\0" as *const u8 as *const i8, arg) == 0 {
                            if (*params).force_overwrite != 0 {
                                fprintf(
                                    stderr,
                                    b"force output overwrite already set\n\0" as *const u8
                                        as *const i8,
                                );
                                return COMMAND_INVALID;
                            };
                            (*params).force_overwrite = 1;
                        } else if strcmp(b"help\0" as *const u8 as *const i8, arg) == 0 {
                            return COMMAND_HELP;
                        } else {
                            if strcmp(b"keep\0" as *const u8 as *const i8, arg) == 0 {
                                if keep_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"argument --rm / -j or --keep / -k already set\n\0"
                                            as *const u8
                                            as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                }
                                keep_set = 1;
                                (*params).junk_source = 0;
                            } else if strcmp(b"no-copy-stat\0" as *const u8 as *const i8, arg) == 0
                            {
                                if (*params).copy_stat == 0 {
                                    fprintf(
                                        stderr,
                                        b"argument --no-copy-stat / -n already set\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                };
                                (*params).copy_stat = 0;
                            } else if strcmp(b"rm\0" as *const u8 as *const i8, arg) == 0 {
                                if keep_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"argument --rm / -j or --keep / -k already set\n\0"
                                            as *const u8
                                            as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                }
                                keep_set = 1;
                                (*params).junk_source = 1;
                            } else if strcmp(b"stdout\0" as *const u8 as *const i8, arg) == 0 {
                                if output_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"write to standard output already set\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                }
                                output_set = 1;
                                (*params).write_to_stdout = 1;
                            } else if strcmp(b"test\0" as *const u8 as *const i8, arg) == 0 {
                                if command_set != 0 {
                                    fprintf(
                                        stderr,
                                        b"command already set when parsing --test\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                }
                                command_set = 1;
                                command = COMMAND_TEST_INTEGRITY;
                            } else if strcmp(b"verbose\0" as *const u8 as *const i8, arg) == 0 {
                                if (*params).verbosity > 0 {
                                    fprintf(
                                        stderr,
                                        b"argument --verbose / -v already set\n\0" as *const u8
                                            as *const i8,
                                    );
                                    return COMMAND_INVALID;
                                };
                                (*params).verbosity = 1;
                            } else if strcmp(b"version\0" as *const u8 as *const i8, arg) == 0 {
                                return COMMAND_VERSION;
                            } else {
                                let mut value: *const i8 = strrchr(arg, '=' as i32);
                                let mut key_len: u64 = 0;
                                if value.is_null() || *value.offset(1 as isize) as i32 == 0 {
                                    fprintf(
                                        stderr,
                                        b"must pass the parameter as --%s=value\n\0" as *const u8
                                            as *const i8,
                                        arg,
                                    );
                                    return COMMAND_INVALID;
                                }
                                key_len = value.offset_from(arg) as u64;
                                value = value.offset(1);
                                if strncmp(b"lgwin\0" as *const u8 as *const i8, arg, key_len) == 0
                                {
                                    if lgwin_set != 0 {
                                        fprintf(
                                            stderr,
                                            b"lgwin parameter already set\n\0" as *const u8
                                                as *const i8,
                                        );
                                        return COMMAND_INVALID;
                                    }
                                    lgwin_set = ParseInt(value, 0, 24, &mut (*params).lgwin);
                                    if lgwin_set == 0 {
                                        fprintf(
                                            stderr,
                                            b"error parsing lgwin value [%s]\n\0" as *const u8
                                                as *const i8,
                                            value,
                                        );
                                        return COMMAND_INVALID;
                                    }
                                    if (*params).lgwin != 0 && (*params).lgwin < 10 {
                                        fprintf (stderr, b"lgwin parameter (%d) smaller than the minimum (%d)\n\0" as * const u8 as * const i8, (* params).lgwin, 10,);
                                        return COMMAND_INVALID;
                                    }
                                } else if strncmp(
                                    b"large_window\0" as *const u8 as *const i8,
                                    arg,
                                    key_len,
                                ) == 0
                                {
                                    if lgwin_set != 0 {
                                        fprintf(
                                            stderr,
                                            b"lgwin parameter already set\n\0" as *const u8
                                                as *const i8,
                                        );
                                        return COMMAND_INVALID;
                                    }
                                    lgwin_set = ParseInt(value, 0, 30, &mut (*params).lgwin);
                                    if lgwin_set == 0 {
                                        fprintf(
                                            stderr,
                                            b"error parsing lgwin value [%s]\n\0" as *const u8
                                                as *const i8,
                                            value,
                                        );
                                        return COMMAND_INVALID;
                                    }
                                    if (*params).lgwin != 0 && (*params).lgwin < 10 {
                                        fprintf (stderr, b"lgwin parameter (%d) smaller than the minimum (%d)\n\0" as * const u8 as * const i8, (* params).lgwin, 10,);
                                        return COMMAND_INVALID;
                                    }
                                } else if strncmp(
                                    b"output\0" as *const u8 as *const i8,
                                    arg,
                                    key_len,
                                ) == 0
                                {
                                    if output_set != 0 {
                                        fprintf(
                                            stderr,
                                            b"write to standard output already set (--output)\n\0"
                                                as *const u8
                                                as *const i8,
                                        );
                                        return COMMAND_INVALID;
                                    }
                                    let ref mut fresh5 = (*params).output_path;
                                    *fresh5 = value;
                                } else if strncmp(
                                    b"quality\0" as *const u8 as *const i8,
                                    arg,
                                    key_len,
                                ) == 0
                                {
                                    if quality_set != 0 {
                                        fprintf(
                                            stderr,
                                            b"quality already set\n\0" as *const u8 as *const i8,
                                        );
                                        return COMMAND_INVALID;
                                    }
                                    quality_set = ParseInt(value, 0, 11, &mut (*params).quality);
                                    if quality_set == 0 {
                                        fprintf(
                                            stderr,
                                            b"error parsing quality value [%s]\n\0" as *const u8
                                                as *const i8,
                                            value,
                                        );
                                        return COMMAND_INVALID;
                                    }
                                } else if strncmp(
                                    b"suffix\0" as *const u8 as *const i8,
                                    arg,
                                    key_len,
                                ) == 0
                                {
                                    if suffix_set != 0 {
                                        fprintf(
                                            stderr,
                                            b"suffix already set\n\0" as *const u8 as *const i8,
                                        );
                                        return COMMAND_INVALID;
                                    }
                                    suffix_set = 1;
                                    let ref mut fresh6 = (*params).suffix;
                                    *fresh6 = value;
                                } else {
                                    fprintf(
                                        stderr,
                                        b"invalid parameter: [%s]\n\0" as *const u8 as *const i8,
                                        arg,
                                    );
                                    return COMMAND_INVALID;
                                }
                            }
                        }
                    }
                }
            }
            i += 1;
        }
        (*params).input_count = input_count;
        (*params).longest_path_len = longest_path_len;
        (*params).decompress = (command as u32 == COMMAND_DECOMPRESS as u32) as i32;
        (*params).test_integrity = (command as u32 == COMMAND_TEST_INTEGRITY as u32) as i32;
        if input_count > 1 && output_set != 0 {
            return COMMAND_INVALID;
        }
        if (*params).test_integrity != 0 {
            if !((*params).output_path).is_null() {
                return COMMAND_INVALID;
            }
            if (*params).write_to_stdout != 0 {
                return COMMAND_INVALID;
            }
        }
        if !(strchr((*params).suffix, '/' as i32)).is_null()
            || !(strchr((*params).suffix, '\\' as i32)).is_null()
        {
            return COMMAND_INVALID;
        }
        return command;
    }
}

extern "C" fn PrintVersion() {
    let mut major = 0x1000009 >> 24;
    let mut minor = 0x1000009 >> 12 & 0xfff;
    let mut patch = 0x1000009 & 0xfff;
    unsafe {
        fprintf(
            stdout,
            b"brotli %d.%d.%d\n\0" as *const u8 as *const i8,
            major,
            minor,
            patch,
        );
    }
}

extern "C" fn PrintHelp(mut name: *const i8, mut error: i32) {
    unsafe {
        let mut media = if error != 0 { stderr } else { stdout };
        fprintf(
            media,
            b"Usage: %s [OPTION]... [FILE]...\n\0" as *const u8 as *const i8,
            name,
        );
        fprintf (media,
b"Options:\n  -#                          compression level (0-9)\n  -c, --stdout                write on standard output\n  -d, --decompress            decompress\n  -f, --force                 force output file overwrite\n  -h, --help                  display this help and exit\n\0"
         as * const u8 as * const i8,);
        fprintf (media,
  b"  -j, --rm                    remove source file(s)\n  -k, --keep                  keep source file(s) (default)\n  -n, --no-copy-stat          do not copy source file(s) attributes\n  -o FILE, --output=FILE      output file (only if 1 input file)\n\0"
          as * const u8 as * const i8,);
        fprintf(
            media,
            b"  -q NUM, --quality=NUM       compression level (%d-%d)\n\0" as *const u8
                as *const i8,
            0,
            11,
        );
        fprintf (media, b"  -t, --test                  test compressed file integrity\n  -v, --verbose               verbose mode\n\0" as * const u8 as * const i8,);
        fprintf (media, b"  -w NUM, --lgwin=NUM         set LZ77 window size (0, %d-%d)\n                              window size = 2**NUM - 16\n                              0 lets compressor choose the optimal value\n\0" as * const u8 as * const i8, 10
          , 24,);
        fprintf (media,
b"  --large_window=NUM          use incompatible large-window brotli\n                              bitstream with window size (0, %d-%d)\n                              WARNING: this format is not compatible\n                              with brotli RFC 7932 and may not be\n                              decodable with regular brotli decoders\n\0"
         as * const u8 as * const i8, 10, 30,);
        fprintf(
            media,
            b"  -S SUF, --suffix=SUF        output file suffix (default:'%s')\n\0" as *const u8
                as *const i8,
            b".br\0" as *const u8 as *const i8,
        );
        fprintf (media,
b"  -V, --version               display version and exit\n  -Z, --best                  use best compression level (11) (default)\nSimple options could be coalesced, i.e. '-9kf' is equivalent to '-9 -k -f'.\nWith no FILE, or when FILE is -, read standard input.\nAll arguments after '--' are treated as files.\n\0"
         as * const u8 as * const i8,);
    }
}

extern "C" fn PrintablePath(mut path: *const i8) -> *const i8 {
    unsafe {
        return if !path.is_null() {
            path
        } else {
            b"con\0" as *const u8 as *const i8
        };
    }
}

extern "C" fn OpenInputFile(mut input_path: *const i8, mut f: *mut *mut FILE) -> i32 {
    unsafe {
        *f = 0 as *mut FILE;
        if input_path.is_null() {
            *f = fdopen(0, b"rb\0" as *const u8 as *const i8);
            return 1;
        }
        *f = fopen(input_path, b"rb\0" as *const u8 as *const i8);
        if (*f).is_null() {
            fprintf(
                stderr,
                b"failed to open input file [%s]: %s\n\0" as *const u8 as *const i8,
                PrintablePath(input_path),
                strerror(*__errno_location()),
            );
            return 0;
        }
        return 1;
    }
}

extern "C" fn OpenOutputFile(
    mut output_path: *const i8,
    mut f: *mut *mut FILE,
    mut force: i32,
) -> i32 {
    unsafe {
        let mut fd: i32 = 0;
        *f = 0 as *mut FILE;
        if output_path.is_null() {
            *f = fdopen(1, b"wb\0" as *const u8 as *const i8);
            return 1;
        }
        fd = open(
            output_path,
            0o100 | (if force != 0 { 0 } else { 0o200 }) | 0o1 | 0o1000,
            0o400 | 0o200,
        );
        if fd < 0 {
            fprintf(
                stderr,
                b"failed to open output file [%s]: %s\n\0" as *const u8 as *const i8,
                PrintablePath(output_path),
                strerror(*__errno_location()),
            );
            return 0;
        }
        *f = fdopen(fd, b"wb\0" as *const u8 as *const i8);
        if (*f).is_null() {
            fprintf(
                stderr,
                b"failed to open output file [%s]: %s\n\0" as *const u8 as *const i8,
                PrintablePath(output_path),
                strerror(*__errno_location()),
            );
            return 0;
        }
        return 1;
    }
}

extern "C" fn FileSize(mut path: *const i8) -> i64 {
    unsafe {
        let mut f = fopen(path, b"rb\0" as *const u8 as *const i8);
        let mut retval: i64 = 0;
        if f.is_null() {
            return -1 as i64;
        }
        if fseek(f, 0, 2) != 0 {
            fclose(f);
            return -1 as i64;
        }
        retval = ftell(f);
        if fclose(f) != 0 {
            return -1 as i64;
        }
        return retval;
    }
}

extern "C" fn CopyStat(mut input_path: *const i8, mut output_path: *const i8) {
    unsafe {
        let mut statbuf = stat {
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
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut times = utimbuf {
            actime: 0,
            modtime: 0,
        };
        let mut res: i32 = 0;
        if input_path.is_null() || output_path.is_null() {
            return;
        }
        if stat(input_path, &mut statbuf) != 0 {
            return;
        }
        times.actime = statbuf.st_atim.tv_sec;
        times.modtime = statbuf.st_mtim.tv_sec;
        utime(output_path, &mut times);
        res = chmod(
            output_path,
            statbuf.st_mode
                & (0o400
                    | 0o200
                    | 0o100
                    | (0o400 | 0o200 | 0o100i32) >> 3
                    | (0o400 | 0o200 | 0o100) >> 3 >> 3) as u32,
        );
        if res != 0 {
            fprintf(
                stderr,
                b"setting access bits failed for [%s]: %s\n\0" as *const u8 as *const i8,
                PrintablePath(output_path),
                strerror(*__errno_location()),
            );
        }
        res = chown(output_path, (-1) as i32 as u32, statbuf.st_gid);
        if res != 0 {
            fprintf(
                stderr,
                b"setting group failed for [%s]: %s\n\0" as *const u8 as *const i8,
                PrintablePath(output_path),
                strerror(*__errno_location()),
            );
        }
        res = chown(output_path, statbuf.st_uid, (-1) as i32 as u32);
        if res != 0 {
            fprintf(
                stderr,
                b"setting user failed for [%s]: %s\n\0" as *const u8 as *const i8,
                PrintablePath(output_path),
                strerror(*__errno_location()),
            );
        }
    }
}

extern "C" fn NextFile(mut context: *mut Context) -> i32 {
    unsafe {
        let mut arg = 0 as *const i8;
        let mut arg_len: u64 = 0;
        let ref mut fresh7 = (*context).iterator;
        *fresh7 += 1;
        (*context).input_file_length = -1 as i64;
        if (*context).input_count == 0 {
            if (*context).iterator > 1 {
                return 0;
            }
            let ref mut fresh8 = (*context).current_input_path;
            *fresh8 = 0 as *const i8;
            let ref mut fresh9 = (*context).current_output_path;
            *fresh9 = (*context).output_path;
            return 1;
        }
        while (*context).iterator == (*context).not_input_indices[(*context).ignore as usize] {
            let ref mut fresh10 = (*context).iterator;
            *fresh10 += 1;
            let ref mut fresh11 = (*context).ignore;
            *fresh11 += 1;
        }
        if (*context).iterator >= (*context).argc {
            return 0;
        }
        arg = *((*context).argv).offset((*context).iterator as isize);
        arg_len = strlen(arg);
        if arg_len == 1 && *arg.offset(0 as isize) as i32 == '-' as i32 {
            let ref mut fresh12 = (*context).current_input_path;
            *fresh12 = 0 as *const i8;
            let ref mut fresh13 = (*context).current_output_path;
            *fresh13 = (*context).output_path;
            return 1;
        }
        let ref mut fresh14 = (*context).current_input_path;
        *fresh14 = arg;
        (*context).input_file_length = FileSize(arg);
        let ref mut fresh15 = (*context).current_output_path;
        *fresh15 = (*context).output_path;
        if !((*context).output_path).is_null() {
            return 1;
        }
        if (*context).write_to_stdout != 0 {
            return 1;
        }
        strcpy((*context).modified_path, arg);
        let ref mut fresh16 = (*context).current_output_path;
        *fresh16 = (*context).modified_path;
        if (*context).decompress != 0 {
            let mut suffix_len = strlen((*context).suffix);
            let mut name = FileName((*context).modified_path) as *mut i8;
            let mut name_suffix = 0 as *mut i8;
            let mut name_len = strlen(name);
            if name_len < suffix_len.wrapping_add(1) {
                fprintf(
                    stderr,
                    b"empty output file name for [%s] input file\n\0" as *const u8 as *const i8,
                    PrintablePath(arg),
                );
                (*context).iterator_error = 1;
                return 0;
            }
            name_suffix = name
                .offset(name_len as isize)
                .offset(-(suffix_len as isize));
            if strcmp((*context).suffix, name_suffix) != 0 {
                fprintf(
                    stderr,
                    b"input file [%s] suffix mismatch\n\0" as *const u8 as *const i8,
                    PrintablePath(arg),
                );
                (*context).iterator_error = 1;
                return 0;
            };
            *name_suffix.offset(0 as isize) = 0;
            return 1;
        } else {
            strcpy(
                ((*context).modified_path).offset(arg_len as isize),
                (*context).suffix,
            );
            return 1;
        };
    }
}

extern "C" fn OpenFiles(mut context: *mut Context) -> i32 {
    unsafe {
        let mut is_ok = OpenInputFile((*context).current_input_path, &mut (*context).fin);
        if (*context).test_integrity == 0 && is_ok != 0 {
            is_ok = OpenOutputFile(
                (*context).current_output_path,
                &mut (*context).fout,
                (*context).force_overwrite,
            );
        }
        return is_ok;
    }
}

extern "C" fn CloseFiles(mut context: *mut Context, mut success: i32) -> i32 {
    unsafe {
        let mut is_ok = 1;
        if (*context).test_integrity == 0 && !((*context).fout).is_null() {
            if success == 0 && !((*context).current_output_path).is_null() {
                unlink((*context).current_output_path);
            }
            if fclose((*context).fout) != 0 {
                if success != 0 {
                    fprintf(
                        stderr,
                        b"fclose failed [%s]: %s\n\0" as *const u8 as *const i8,
                        PrintablePath((*context).current_output_path),
                        strerror(*__errno_location()),
                    );
                }
                is_ok = 0;
            }
            if success != 0 && is_ok != 0 && (*context).copy_stat != 0 {
                CopyStat(
                    (*context).current_input_path,
                    (*context).current_output_path,
                );
            }
        }
        if !((*context).fin).is_null() {
            if fclose((*context).fin) != 0 {
                if is_ok != 0 {
                    fprintf(
                        stderr,
                        b"fclose failed [%s]: %s\n\0" as *const u8 as *const i8,
                        PrintablePath((*context).current_input_path),
                        strerror(*__errno_location()),
                    );
                }
                is_ok = 0;
            }
        }
        if success != 0 && (*context).junk_source != 0 && !((*context).current_input_path).is_null()
        {
            unlink((*context).current_input_path);
        }
        let ref mut fresh17 = (*context).fin;
        *fresh17 = 0 as *mut FILE;
        let ref mut fresh18 = (*context).fout;
        *fresh18 = 0 as *mut FILE;
        return is_ok;
    }
}

static mut kFileBufferSize: u64 = (1i32 << 19) as u64;
extern "C" fn InitializeBuffers(mut context: *mut Context) {
    unsafe {
        (*context).available_in = 0;
        let ref mut fresh19 = (*context).next_in;
        *fresh19 = 0 as *const u8;
        (*context).available_out = kFileBufferSize;
        let ref mut fresh20 = (*context).next_out;
        *fresh20 = (*context).output;
        (*context).total_in = 0;
        (*context).total_out = 0;
    }
}

extern "C" fn HasMoreInput(mut context: *mut Context) -> i32 {
    unsafe {
        return if feof((*context).fin) != 0 { 0 } else { 1 };
    }
}

extern "C" fn ProvideInput(mut context: *mut Context) -> i32 {
    unsafe {
        (*context).available_in = fread(
            (*context).input as *mut libc::c_void,
            1,
            kFileBufferSize,
            (*context).fin,
        );
        let ref mut fresh21 = (*context).total_in;
        *fresh21 = (*fresh21 as u64).wrapping_add((*context).available_in) as u64;
        let ref mut fresh22 = (*context).next_in;
        *fresh22 = (*context).input;
        if ferror((*context).fin) != 0 {
            fprintf(
                stderr,
                b"failed to read input [%s]: %s\n\0" as *const u8 as *const i8,
                PrintablePath((*context).current_input_path),
                strerror(*__errno_location()),
            );
            return 0;
        }
        return 1;
    }
}

extern "C" fn WriteOutput(mut context: *mut Context) -> i32 {
    unsafe {
        let mut out_size = ((*context).next_out).offset_from((*context).output) as u64;
        let ref mut fresh23 = (*context).total_out;
        *fresh23 = (*fresh23 as u64).wrapping_add(out_size) as u64;
        if out_size == 0 {
            return 1;
        }
        if (*context).test_integrity != 0 {
            return 1;
        }
        fwrite(
            (*context).output as *const libc::c_void,
            1,
            out_size,
            (*context).fout,
        );
        if ferror((*context).fout) != 0 {
            fprintf(
                stderr,
                b"failed to write output [%s]: %s\n\0" as *const u8 as *const i8,
                PrintablePath((*context).current_output_path),
                strerror(*__errno_location()),
            );
            return 0;
        }
        return 1;
    }
}

extern "C" fn ProvideOutput(mut context: *mut Context) -> i32 {
    unsafe {
        if WriteOutput(context) == 0 {
            return 0;
        };
        (*context).available_out = kFileBufferSize;
        let ref mut fresh24 = (*context).next_out;
        *fresh24 = (*context).output;
        return 1;
    }
}

extern "C" fn FlushOutput(mut context: *mut Context) -> i32 {
    unsafe {
        if WriteOutput(context) == 0 {
            return 0;
        };
        (*context).available_out = 0;
        return 1;
    }
}

extern "C" fn PrintBytes(mut value: u64) {
    unsafe {
        if value < 1024 {
            fprintf(stderr, b"%d B\0" as *const u8 as *const i8, value as i32);
        } else if value < 1048576 {
            fprintf(
                stderr,
                b"%0.3f KiB\0" as *const u8 as *const i8,
                value as f64 / 1024.0f64,
            );
        } else if value < 1073741824 {
            fprintf(
                stderr,
                b"%0.3f MiB\0" as *const u8 as *const i8,
                value as f64 / 1048576.0f64,
            );
        } else {
            fprintf(
                stderr,
                b"%0.3f GiB\0" as *const u8 as *const i8,
                value as f64 / 1073741824.0f64,
            );
        };
    }
}

extern "C" fn PrintFileProcessingProgress(mut context: *mut Context) {
    unsafe {
        fprintf(
            stderr,
            b"[%s]: \0" as *const u8 as *const i8,
            PrintablePath((*context).current_input_path),
        );
        PrintBytes((*context).total_in);
        fprintf(stderr, b" -> \0" as *const u8 as *const i8);
        PrintBytes((*context).total_out);
    }
}

extern "C" fn DecompressFile(mut context: *mut Context, mut s: *mut BrotliDecoderState) -> i32 {
    unsafe {
        let mut result = BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT;
        InitializeBuffers(context);
        loop {
            if result as u32 == BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT as u32 {
                if HasMoreInput(context) == 0 {
                    fprintf(
                        stderr,
                        b"corrupt input [%s]\n\0" as *const u8 as *const i8,
                        PrintablePath((*context).current_input_path),
                    );
                    return 0;
                }
                if ProvideInput(context) == 0 {
                    return 0;
                }
            } else if result as u32 == BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT as u32 {
                if ProvideOutput(context) == 0 {
                    return 0;
                }
            } else if result as u32 == BROTLI_DECODER_RESULT_SUCCESS as u32 {
                if FlushOutput(context) == 0 {
                    return 0;
                }
                let mut has_more_input =
                    ((*context).available_in != 0 || fgetc((*context).fin) != -1) as i32;
                if has_more_input != 0 {
                    fprintf(
                        stderr,
                        b"corrupt input [%s]\n\0" as *const u8 as *const i8,
                        PrintablePath((*context).current_input_path),
                    );
                    return 0;
                }
                if (*context).verbosity > 0 {
                    fprintf(stderr, b"Decompressed \0" as *const u8 as *const i8);
                    PrintFileProcessingProgress(context);
                    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                }
                return 1;
            } else {
                fprintf(
                    stderr,
                    b"corrupt input [%s]\n\0" as *const u8 as *const i8,
                    PrintablePath((*context).current_input_path),
                );
                return 0;
            }
            result = BrotliDecoderDecompressStream(
                s,
                &mut (*context).available_in,
                &mut (*context).next_in,
                &mut (*context).available_out,
                &mut (*context).next_out,
                0 as *mut u64,
            );
        }
    }
}

extern "C" fn DecompressFiles(mut context: *mut Context) -> i32 {
    unsafe {
        while NextFile(context) != 0 {
            let mut is_ok = 1;
            let mut s = BrotliDecoderCreateInstance(None, None, 0 as *mut libc::c_void);
            if s.is_null() {
                fprintf(stderr, b"out of memory\n\0" as *const u8 as *const i8);
                return 0;
            }
            BrotliDecoderSetParameter(s, BROTLI_DECODER_PARAM_LARGE_WINDOW, 1);
            is_ok = OpenFiles(context);
            if is_ok != 0
                && ((*context).current_input_path).is_null()
                && (*context).force_overwrite == 0
                && isatty(0) != 0
            {
                fprintf(
                    stderr,
                    b"Use -h help. Use -f to force input from a terminal.\n\0" as *const u8
                        as *const i8,
                );
                is_ok = 0;
            }
            if is_ok != 0 {
                is_ok = DecompressFile(context, s);
            }
            BrotliDecoderDestroyInstance(s);
            if CloseFiles(context, is_ok) == 0 {
                is_ok = 0;
            }
            if is_ok == 0 {
                return 0;
            }
        }
        return 1;
    }
}

extern "C" fn CompressFile(mut context: *mut Context, mut s: *mut BrotliEncoderState) -> i32 {
    unsafe {
        let mut is_eof = 0;
        InitializeBuffers(context);
        loop {
            if (*context).available_in == 0 && is_eof == 0 {
                if ProvideInput(context) == 0 {
                    return 0;
                }
                is_eof = (HasMoreInput(context) == 0) as i32;
            }
            if BrotliEncoderCompressStream(
                s,
                (if is_eof != 0 {
                    BROTLI_OPERATION_FINISH as i32
                } else {
                    BROTLI_OPERATION_PROCESS as i32
                }) as u32,
                &mut (*context).available_in,
                &mut (*context).next_in,
                &mut (*context).available_out,
                &mut (*context).next_out,
                0 as *mut u64,
            ) == 0
            {
                fprintf(
                    stderr,
                    b"failed to compress data [%s]\n\0" as *const u8 as *const i8,
                    PrintablePath((*context).current_input_path),
                );
                return 0;
            }
            if (*context).available_out == 0 {
                if ProvideOutput(context) == 0 {
                    return 0;
                }
            }
            if BrotliEncoderIsFinished(s) != 0 {
                if FlushOutput(context) == 0 {
                    return 0;
                }
                if (*context).verbosity > 0 {
                    fprintf(stderr, b"Compressed \0" as *const u8 as *const i8);
                    PrintFileProcessingProgress(context);
                    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                }
                return 1;
            }
        }
    }
}

extern "C" fn CompressFiles(mut context: *mut Context) -> i32 {
    unsafe {
        while NextFile(context) != 0 {
            let mut is_ok = 1;
            let mut s = BrotliEncoderCreateInstance(None, None, 0 as *mut libc::c_void);
            if s.is_null() {
                fprintf(stderr, b"out of memory\n\0" as *const u8 as *const i8);
                return 0;
            }
            BrotliEncoderSetParameter(s, BROTLI_PARAM_QUALITY, (*context).quality as u32);
            if (*context).lgwin > 0 {
                if (*context).lgwin > 24 {
                    BrotliEncoderSetParameter(s, BROTLI_PARAM_LARGE_WINDOW, 1);
                }
                BrotliEncoderSetParameter(s, BROTLI_PARAM_LGWIN, (*context).lgwin as u32);
            } else {
                let mut lgwin = 24;
                if (*context).input_file_length >= 0 {
                    lgwin = 10;
                    while (1u64 << lgwin).wrapping_sub(16) < (*context).input_file_length as u64 {
                        lgwin = lgwin.wrapping_add(1);
                        if lgwin == 24 {
                            break;
                        }
                    }
                }
                BrotliEncoderSetParameter(s, BROTLI_PARAM_LGWIN, lgwin);
            }
            if (*context).input_file_length > 0 {
                let mut size_hint = if (*context).input_file_length < (1i32 << 30) as i64 {
                    (*context).input_file_length as u32
                } else {
                    1 << 30
                };
                BrotliEncoderSetParameter(s, BROTLI_PARAM_SIZE_HINT, size_hint);
            }
            is_ok = OpenFiles(context);
            if is_ok != 0
                && ((*context).current_output_path).is_null()
                && (*context).force_overwrite == 0
                && isatty(1) != 0
            {
                fprintf(
                    stderr,
                    b"Use -h help. Use -f to force output to a terminal.\n\0" as *const u8
                        as *const i8,
                );
                is_ok = 0;
            }
            if is_ok != 0 {
                is_ok = CompressFile(context, s);
            }
            BrotliEncoderDestroyInstance(s);
            if CloseFiles(context, is_ok) == 0 {
                is_ok = 0;
            }
            if is_ok == 0 {
                return 0;
            }
        }
        return 1;
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        let mut command = COMMAND_COMPRESS;
        let mut context = Context {
            quality: 0,
            lgwin: 0,
            verbosity: 0,
            force_overwrite: 0,
            junk_source: 0,
            copy_stat: 0,
            write_to_stdout: 0,
            test_integrity: 0,
            decompress: 0,
            large_window: 0,
            output_path: 0 as *const i8,
            suffix: 0 as *const i8,
            not_input_indices: [0; 20],
            longest_path_len: 0,
            input_count: 0,
            argc: 0,
            argv: 0 as *mut *mut i8,
            modified_path: 0 as *mut i8,
            iterator: 0,
            ignore: 0,
            iterator_error: 0,
            buffer: 0 as *mut u8,
            input: 0 as *mut u8,
            output: 0 as *mut u8,
            current_input_path: 0 as *const i8,
            current_output_path: 0 as *const i8,
            input_file_length: 0,
            fin: 0 as *mut FILE,
            fout: 0 as *mut FILE,
            available_in: 0,
            next_in: 0 as *const u8,
            available_out: 0,
            next_out: 0 as *mut u8,
            total_in: 0,
            total_out: 0,
        };
        let mut is_ok = 1;
        let mut i: i32 = 0;
        context.quality = 11;
        context.lgwin = -1;
        context.verbosity = 0;
        context.force_overwrite = 0;
        context.junk_source = 0;
        context.copy_stat = 1;
        context.test_integrity = 0;
        context.write_to_stdout = 0;
        context.decompress = 0;
        context.large_window = 0;
        context.output_path = 0 as *const i8;
        context.suffix = b".br\0" as *const u8 as *const i8;
        i = 0;
        while i < 20 {
            context.not_input_indices[i as usize] = 0;
            i += 1;
        }
        context.longest_path_len = 1;
        context.input_count = 0;
        context.argc = argc;
        context.argv = argv;
        context.modified_path = 0 as *mut i8;
        context.iterator = 0;
        context.ignore = 0;
        context.iterator_error = 0;
        context.buffer = 0 as *mut u8;
        context.current_input_path = 0 as *const i8;
        context.current_output_path = 0 as *const i8;
        context.fin = 0 as *mut FILE;
        context.fout = 0 as *mut FILE;
        command = ParseParams(&mut context);
        if command as u32 == COMMAND_COMPRESS as u32
            || command as u32 == COMMAND_DECOMPRESS as u32
            || command as u32 == COMMAND_TEST_INTEGRITY as u32
        {
            if is_ok != 0 {
                let mut modified_path_len = (context.longest_path_len)
                    .wrapping_add(strlen(context.suffix))
                    .wrapping_add(1);
                context.modified_path = malloc(modified_path_len) as *mut i8;
                context.buffer = malloc(kFileBufferSize.wrapping_mul(2)) as *mut u8;
                if (context.modified_path).is_null() || (context.buffer).is_null() {
                    fprintf(stderr, b"out of memory\n\0" as *const u8 as *const i8);
                    is_ok = 0;
                } else {
                    context.input = context.buffer;
                    context.output = (context.buffer).offset(kFileBufferSize as isize);
                }
            }
        }
        if is_ok == 0 {
            command = COMMAND_NOOP;
        }
        match command as u32 {
            5 => {}
            6 => {
                PrintVersion();
            }
            0 => {
                is_ok = CompressFiles(&mut context);
            }
            1 | 4 => {
                is_ok = DecompressFiles(&mut context);
            }
            2 | 3 | _ => {
                is_ok = (command as u32 == COMMAND_HELP as u32) as i32;
                PrintHelp(FileName(*argv.offset(0 as isize)), is_ok);
            }
        }
        if context.iterator_error != 0 {
            is_ok = 0;
        }
        free(context.modified_path as *mut libc::c_void);
        free(context.buffer as *mut libc::c_void);
        if is_ok == 0 {
            exit(1);
        }
        return 0;
    }
}
