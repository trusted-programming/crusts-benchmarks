use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _lil_value_t;
    pub type _lil_var_t;
    pub type _lil_list_t;
    pub type _lil_t;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fgetc(__stream: *mut FILE) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn fread(_: *mut libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn popen(__command: *const i8, __modes: *const i8) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn lil_new() -> lil_t;
    fn lil_free(lil: lil_t);
    fn lil_register(lil: lil_t, name: *const i8, proc_0: lil_func_proc_t) -> i32;
    fn lil_parse(lil: lil_t, code: *const i8, codelen: u64, funclevel: i32) -> lil_value_t;
    fn lil_callback(lil: lil_t, cb: i32, proc_0: lil_callback_proc_t);
    fn lil_error(lil: lil_t, msg: *mut *const i8, pos: *mut u64) -> i32;
    fn lil_to_string(val: lil_value_t) -> *const i8;
    fn lil_to_integer(val: lil_value_t) -> i64;
    fn lil_alloc_string(str: *const i8) -> lil_value_t;
    fn lil_free_value(val: lil_value_t);
    fn lil_alloc_list() -> lil_list_t;
    fn lil_free_list(list: lil_list_t);
    fn lil_list_append(list: lil_list_t, val: lil_value_t);
    fn lil_list_to_value(list: lil_list_t, do_escape: i32) -> lil_value_t;
    fn lil_set_var(lil: lil_t, name: *const i8, val: lil_value_t, local: i32) -> lil_var_t;
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
pub type lil_value_t = *mut _lil_value_t;
pub type lil_var_t = *mut _lil_var_t;
pub type lil_list_t = *mut _lil_list_t;
pub type lil_t = *mut _lil_t;
pub type lil_func_proc_t =
    Option<unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t>;
pub type lil_callback_proc_t = Option<unsafe extern "C" fn() -> ()>;
static mut running: i32 = 1;
static mut exit_code: i32 = 0;
extern "C" fn do_exit(mut lil: lil_t, mut val: lil_value_t) {
    unsafe {
        running = 0;
        exit_code = lil_to_integer(val) as i32;
    }
}

extern "C" fn do_system(mut argc: u64, mut argv: *mut *mut i8) -> *mut i8 {
    unsafe {
        let mut cmd = 0 as *mut i8;
        let mut cmdlen = 0;
        let mut i: u64 = 0;
        let mut p = 0 as *mut FILE;
        i = 0;
        while i < argc {
            let mut len = strlen(*argv.offset(i as isize));
            if i != 0 {
                cmd = realloc(cmd as *mut libc::c_void, (cmdlen + 1i32) as u64) as *mut i8;
                let fresh0 = cmdlen;
                cmdlen = cmdlen + 1;
                *cmd.offset(fresh0 as isize) = ' ' as i8;
            }
            cmd = realloc(cmd as *mut libc::c_void, (cmdlen as u64).wrapping_add(len)) as *mut i8;
            memcpy(
                cmd.offset(cmdlen as isize) as *mut libc::c_void,
                *argv.offset(i as isize) as *const libc::c_void,
                len,
            );
            cmdlen = (cmdlen as u64).wrapping_add(len) as i32;
            i = i.wrapping_add(1);
        }
        cmd = realloc(cmd as *mut libc::c_void, (cmdlen + 1i32) as u64) as *mut i8;
        *cmd.offset(cmdlen as isize) = 0;
        p = popen(cmd, b"r\0" as *const u8 as *const i8);
        free(cmd as *mut libc::c_void);
        if !p.is_null() {
            let mut retval = 0 as *mut i8;
            let mut size = 0;
            let mut buff: [i8; 1024] = [0; 1024];
            let mut bytes: i64 = 0;
            loop {
                bytes = fread(buff.as_mut_ptr() as *mut libc::c_void, 1, 1024, p) as i64;
                if !(bytes != 0) {
                    break;
                }
                retval = realloc(retval as *mut libc::c_void, size.wrapping_add(bytes as u64))
                    as *mut i8;
                memcpy(
                    retval.offset(size as isize) as *mut libc::c_void,
                    buff.as_mut_ptr() as *const libc::c_void,
                    bytes as u64,
                );
                size = (size as u64).wrapping_add(bytes as u64) as u64;
            }
            retval = realloc(retval as *mut libc::c_void, size.wrapping_add(1)) as *mut i8;
            *retval.offset(size as isize) = 0;
            pclose(p);
            return retval;
        } else {
            return 0 as *mut i8;
        };
    }
}

extern "C" fn fnc_writechar(
    mut lil: lil_t,
    mut argc: u64,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    unsafe {
        if argc == 0 {
            return 0 as lil_value_t;
        }
        print!("{},9999");
        return 0 as lil_value_t;
    }
}

extern "C" fn fnc_system(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut sargv =
            malloc((::std::mem::size_of::<*mut i8>() as u64).wrapping_mul(argc.wrapping_add(1)))
                as *mut *const i8;
        let mut r = 0 as lil_value_t;
        let mut rv = 0 as *mut i8;
        let mut i: u64 = 0;
        if argc == 0 {
            return 0 as lil_value_t;
        }
        i = 0;
        while i < argc {
            let ref mut fresh1 = *sargv.offset(i as isize);
            *fresh1 = lil_to_string(*argv.offset(i as isize));
            i = i.wrapping_add(1);
        }
        let ref mut fresh2 = *sargv.offset(argc as isize);
        *fresh2 = 0 as *const i8;
        rv = do_system(argc, sargv as *mut *mut i8);
        if !rv.is_null() {
            r = lil_alloc_string(rv);
            free(rv as *mut libc::c_void);
        }
        free(sargv as *mut libc::c_void);
        return r;
    }
}

extern "C" fn fnc_readline(
    mut lil: lil_t,
    mut argc: u64,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    unsafe {
        let mut len = 0;
        let mut size = 64;
        let mut buffer = malloc(size) as *mut i8;
        let mut ch: i8 = 0;
        let mut retval = 0 as *mut _lil_value_t;
        loop {
            ch = fgetc(stdin) as i8;
            if ch as i32 == -1 {
                break;
            }
            if ch as i32 == '\r' as i32 {
                continue;
            }
            if ch as i32 == '\n' as i32 {
                break;
            }
            if len < size {
                size = (size as u64).wrapping_add(64) as u64;
                buffer = realloc(buffer as *mut libc::c_void, size) as *mut i8;
            }
            let fresh3 = len;
            len = len.wrapping_add(1);
            *buffer.offset(fresh3 as isize) = ch;
        }
        buffer = realloc(buffer as *mut libc::c_void, len.wrapping_add(1)) as *mut i8;
        *buffer.offset(len as isize) = 0;
        retval = lil_alloc_string(buffer);
        free(buffer as *mut libc::c_void);
        return retval;
    }
}

extern "C" fn repl() -> i32 {
    let mut buffer: [i8; 16384] = [0; 16384];
    unsafe {
        let mut lil = lil_new();
        lil_register(
            lil,
            b"writechar\0" as *const u8 as *const i8,
            Some(
                fnc_writechar as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t,
            ),
        );
        lil_register(
            lil,
            b"system\0" as *const u8 as *const i8,
            Some(fnc_system as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
        );
        lil_register(
            lil,
            b"readline\0" as *const u8 as *const i8,
            Some(fnc_readline as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
        );
        print!("Little Interpreted Language Interactive Shell\n");
        lil_callback(
            lil,
            0,
            ::std::mem::transmute::<
                Option<unsafe extern "C" fn(lil_t, lil_value_t) -> ()>,
                lil_callback_proc_t,
            >(Some(
                do_exit as unsafe extern "C" fn(lil_t, lil_value_t) -> (),
            )),
        );
        while running != 0 {
            let mut result = 0 as *mut _lil_value_t;
            let mut strres = 0 as *const i8;
            let mut err_msg = 0 as *const i8;
            let mut pos: u64 = 0;
            buffer[0 as usize] = 0;
            print!("# ");
            if (fgets(buffer.as_mut_ptr(), 16384, stdin)).is_null() {
                break;
            }
            result = lil_parse(lil, buffer.as_mut_ptr(), 0, 0);
            strres = lil_to_string(result);
            if *strres.offset(0 as isize) != 0 {
                print!("{}\n,9998");
            }
            lil_free_value(result);
            if lil_error(lil, &mut err_msg, &mut pos) != 0 {
                print!("error at {}: {}\n,9999,9998");
            }
        }
        lil_free(lil);
        return exit_code;
    }
}

extern "C" fn nonint(mut argc: i32, mut argv: *mut *const i8) -> i32 {
    unsafe {
        let mut lil = lil_new();
        let mut filename = *argv.offset(1 as isize);
        let mut err_msg = 0 as *const i8;
        let mut pos: u64 = 0;
        let mut arglist = lil_alloc_list();
        let mut args = 0 as *mut _lil_value_t;
        let mut result = 0 as *mut _lil_value_t;
        let mut tmpcode = 0 as *mut i8;
        let mut i: i32 = 0;
        lil_register(
            lil,
            b"writechar\0" as *const u8 as *const i8,
            Some(
                fnc_writechar as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t,
            ),
        );
        lil_register(
            lil,
            b"system\0" as *const u8 as *const i8,
            Some(fnc_system as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
        );
        i = 2;
        while i < argc {
            lil_list_append(arglist, lil_alloc_string(*argv.offset(i as isize)));
            i += 1;
        }
        args = lil_list_to_value(arglist, 1);
        lil_free_list(arglist);
        lil_set_var(lil, b"argv\0" as *const u8 as *const i8, args, 0);
        lil_free_value(args);
        tmpcode = malloc((strlen(filename)).wrapping_add(256)) as *mut i8;
        sprintf (tmpcode, b"set __lilmain:code__ [read {%s}]\nif [streq $__lilmain:code__ ''] {print There is no code in the file or the file does not exist} {eval $__lilmain:code__}\n\0" as * const u8 as * const i8, filename,);
        result = lil_parse(lil, tmpcode, 0, 1);
        free(tmpcode as *mut libc::c_void);
        lil_free_value(result);
        if lil_error(lil, &mut err_msg, &mut pos) != 0 {
            fprintf(
                stderr,
                b"lil: error at %i: %s\n\0" as *const u8 as *const i8,
                pos as i32,
                err_msg,
            );
        }
        lil_free(lil);
        return exit_code;
    }
}

fn main_0(mut argc: i32, mut argv: *mut *const i8) -> i32 {
    unsafe {
        if argc < 2 {
            return repl();
        } else {
            return nonint(argc, argv);
        };
    }
}
