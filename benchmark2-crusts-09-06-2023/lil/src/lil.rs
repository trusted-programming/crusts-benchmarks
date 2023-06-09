use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn atof(__nptr: *const i8) -> f64;
    fn atoll(__nptr: *const i8) -> i64;
    fn rand() -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fread(_: *mut libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn fwrite(_: *const libc::c_void, _: u64, _: u64, _: *mut FILE) -> u64;
    fn fseek(__stream: *mut FILE, __off: i64, __whence: i32) -> i32;
    fn ftell(__stream: *mut FILE) -> i64;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn __ctype_b_loc() -> *mut *const u16;
    fn fmod(_: f64, _: f64) -> f64;
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
pub struct _lil_value_t {
    pub l: u64,
    pub d: *mut i8,
}
pub type lil_value_t = *mut _lil_value_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_func_t {
    pub name: *mut i8,
    pub code: lil_value_t,
    pub argnames: lil_list_t,
    pub proc_0: lil_func_proc_t,
}
pub type lil_func_proc_t =
    Option<unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t>;
pub type lil_t = *mut _lil_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_t {
    pub code: *const i8,
    pub rootcode: *const i8,
    pub clen: u64,
    pub head: u64,
    pub ignoreeol: i32,
    pub cmd: *mut lil_func_t,
    pub cmds: u64,
    pub syscmds: u64,
    pub catcher: *mut i8,
    pub in_catcher: i32,
    pub dollarprefix: *mut i8,
    pub env: lil_env_t,
    pub rootenv: lil_env_t,
    pub downenv: lil_env_t,
    pub empty: lil_value_t,
    pub error: i32,
    pub err_head: u64,
    pub err_msg: *mut i8,
    pub callback: [lil_callback_proc_t; 8],
    pub parse_depth: u64,
    pub data: *mut libc::c_void,
}
pub type lil_callback_proc_t = Option<unsafe extern "C" fn() -> ()>;
pub type lil_env_t = *mut _lil_env_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_env_t {
    pub parent: *mut _lil_env_t,
    pub func: lil_func_t,
    pub catcher_for: lil_value_t,
    pub var: *mut lil_var_t,
    pub vars: u64,
    pub retval: lil_value_t,
    pub retval_set: i32,
    pub breakrun: i32,
}
pub type lil_var_t = *mut _lil_var_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_var_t {
    pub n: *mut i8,
    pub env: *mut _lil_env_t,
    pub v: lil_value_t,
}
pub type lil_func_t = *mut _lil_func_t;
pub type lil_list_t = *mut _lil_list_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _lil_list_t {
    pub v: *mut lil_value_t,
    pub c: u64,
}
pub type lil_exit_callback_proc_t = Option<unsafe extern "C" fn(lil_t, lil_value_t) -> ()>;
pub type lil_write_callback_proc_t = Option<unsafe extern "C" fn(lil_t, *const i8) -> ()>;
pub type lil_read_callback_proc_t = Option<unsafe extern "C" fn(lil_t, *const i8) -> *mut i8>;
pub type lil_source_callback_proc_t = Option<unsafe extern "C" fn(lil_t, *const i8) -> *mut i8>;
pub type lil_store_callback_proc_t =
    Option<unsafe extern "C" fn(lil_t, *const i8, *const i8) -> ()>;
pub type lil_error_callback_proc_t = Option<unsafe extern "C" fn(lil_t, u64, *const i8) -> ()>;
pub type lil_setvar_callback_proc_t =
    Option<unsafe extern "C" fn(lil_t, *const i8, *mut lil_value_t) -> i32>;
pub type lil_getvar_callback_proc_t =
    Option<unsafe extern "C" fn(lil_t, *const i8, *mut lil_value_t) -> i32>;
pub type expreval_t = _expreval_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _expreval_t {
    pub code: *const i8,
    pub len: u64,
    pub head: u64,
    pub ival: i64,
    pub dval: f64,
    pub type_0: i32,
    pub error: i32,
}
extern "C" fn strclone(mut s: *const i8) -> *mut i8 {
    unsafe {
        let mut len = (strlen(s)).wrapping_add(1);
        let mut ns = malloc(len) as *mut i8;
        if ns.is_null() {
            return 0 as *mut i8;
        }
        memcpy(ns as *mut libc::c_void, s as *const libc::c_void, len);
        return ns;
    }
}

extern "C" fn alloc_value(mut str: *const i8) -> lil_value_t {
    unsafe {
        let mut val = calloc(1, ::std::mem::size_of::<_lil_value_t>() as u64) as lil_value_t;
        if val.is_null() {
            return 0 as lil_value_t;
        }
        if !str.is_null() {
            (*val).l = strlen(str);
            let ref mut fresh0 = (*val).d;
            *fresh0 = malloc(((*val).l).wrapping_add(1)) as *mut i8;
            if ((*val).d).is_null() {
                free(val as *mut libc::c_void);
                return 0 as lil_value_t;
            }
            memcpy(
                (*val).d as *mut libc::c_void,
                str as *const libc::c_void,
                ((*val).l).wrapping_add(1),
            );
        } else {
            (*val).l = 0;
            let ref mut fresh1 = (*val).d;
            *fresh1 = 0 as *mut i8;
        }
        return val;
    }
}

#[no_mangle]
pub extern "C" fn lil_clone_value(mut src: lil_value_t) -> lil_value_t {
    let mut val = 0 as *mut _lil_value_t;
    if src.is_null() {
        return 0 as lil_value_t;
    }
    unsafe {
        val = calloc(1, ::std::mem::size_of::<_lil_value_t>() as u64) as lil_value_t;
    }
    if val.is_null() {
        return 0 as lil_value_t;
    };
    (*val).l = (*src).l;
    unsafe {
        if (*src).l != 0 {
            let ref mut fresh2 = (*val).d;
            *fresh2 = malloc(((*val).l).wrapping_add(1)) as *mut i8;
            if ((*val).d).is_null() {
                free(val as *mut libc::c_void);
                return 0 as lil_value_t;
            }
            memcpy(
                (*val).d as *mut libc::c_void,
                (*src).d as *const libc::c_void,
                ((*val).l).wrapping_add(1),
            );
        } else {
            let ref mut fresh3 = (*val).d;
            *fresh3 = 0 as *mut i8;
        }
    }
    return val;
}

#[no_mangle]
pub extern "C" fn lil_append_char(mut val: lil_value_t, mut ch: i8) -> i32 {
    unsafe {
        let mut new = realloc((*val).d as *mut libc::c_void, ((*val).l).wrapping_add(2)) as *mut i8;
        if new.is_null() {
            return 0;
        }
        let ref mut fresh4 = (*val).l;
        let fresh5 = *fresh4;
        *fresh4 = (*fresh4).wrapping_add(1);
        *new.offset(fresh5 as isize) = ch;
        *new.offset((*val).l as isize) = 0;
        let ref mut fresh6 = (*val).d;
        *fresh6 = new;
    }
    return 1;
}

#[no_mangle]
pub extern "C" fn lil_append_string(mut val: lil_value_t, mut s: *const i8) -> i32 {
    unsafe {
        let mut new = 0 as *mut i8;
        let mut len: u64 = 0;
        if s.is_null() || *s.offset(0 as isize) == 0 {
            return 1;
        }
        len = strlen(s);
        new = realloc(
            (*val).d as *mut libc::c_void,
            ((*val).l).wrapping_add(len).wrapping_add(1),
        ) as *mut i8;
        if new.is_null() {
            return 0;
        }
        memcpy(
            new.offset((*val).l as isize) as *mut libc::c_void,
            s as *const libc::c_void,
            len.wrapping_add(1),
        );
        let ref mut fresh7 = (*val).l;
        *fresh7 = (*fresh7 as u64).wrapping_add(len) as u64;
        let ref mut fresh8 = (*val).d;
        *fresh8 = new;
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn lil_append_val(mut val: lil_value_t, mut v: lil_value_t) -> i32 {
    let mut new = 0 as *mut i8;
    if v.is_null() || (*v).l == 0 {
        return 1;
    }
    unsafe {
        new = realloc(
            (*val).d as *mut libc::c_void,
            ((*val).l).wrapping_add((*v).l).wrapping_add(1),
        ) as *mut i8;
    }
    if new.is_null() {
        return 0;
    }
    unsafe {
        memcpy(
            new.offset((*val).l as isize) as *mut libc::c_void,
            (*v).d as *const libc::c_void,
            ((*v).l).wrapping_add(1),
        );
    }
    let ref mut fresh9 = (*val).l;
    *fresh9 = (*fresh9 as u64).wrapping_add((*v).l) as u64;
    let ref mut fresh10 = (*val).d;
    *fresh10 = new;
    return 1;
}

#[no_mangle]
pub extern "C" fn lil_free_value(mut val: lil_value_t) {
    if val.is_null() {
        return;
    }
    unsafe {
        free((*val).d as *mut libc::c_void);
        free(val as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn lil_alloc_list() -> lil_list_t {
    unsafe {
        let mut list = calloc(1, ::std::mem::size_of::<_lil_list_t>() as u64) as lil_list_t;
        let ref mut fresh11 = (*list).v;
        *fresh11 = 0 as *mut lil_value_t;
        (*list).c = 0;
        return list;
    }
}

#[no_mangle]
pub extern "C" fn lil_free_list(mut list: lil_list_t) {
    let mut i: u64 = 0;
    if list.is_null() {
        return;
    }
    i = 0;
    unsafe {
        while i < (*list).c {
            lil_free_value(*((*list).v).offset(i as isize));
            i = i.wrapping_add(1);
        }
        free((*list).v as *mut libc::c_void);
        free(list as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn lil_list_append(mut list: lil_list_t, mut val: lil_value_t) {
    unsafe {
        let mut nv = realloc(
            (*list).v as *mut libc::c_void,
            (::std::mem::size_of::<lil_value_t>() as u64).wrapping_mul(((*list).c).wrapping_add(1)),
        ) as *mut lil_value_t;
        if nv.is_null() {
            return;
        }
        let ref mut fresh12 = (*list).v;
        *fresh12 = nv;
        let ref mut fresh13 = (*list).c;
        let fresh14 = *fresh13;
        *fresh13 = (*fresh13).wrapping_add(1);
        let ref mut fresh15 = *nv.offset(fresh14 as isize);
        *fresh15 = val;
    }
}

#[no_mangle]
pub extern "C" fn lil_list_size(mut list: lil_list_t) -> u64 {
    return (*list).c;
}

#[no_mangle]
pub extern "C" fn lil_list_get(mut list: lil_list_t, mut index: u64) -> lil_value_t {
    unsafe {
        return if index >= (*list).c {
            0 as lil_value_t
        } else {
            *((*list).v).offset(index as isize)
        };
    }
}

extern "C" fn needs_escape(mut str: *const i8) -> i32 {
    unsafe {
        let mut i: u64 = 0;
        if str.is_null() || *str.offset(0 as isize) == 0 {
            return 1;
        }
        i = 0;
        while *str.offset(i as isize) != 0 {
            if *(*__ctype_b_loc()).offset(*str.offset(i as isize) as i32 as isize) as i32
                & _ISpunct as i32
                != 0
                || *(*__ctype_b_loc()).offset(*str.offset(i as isize) as i32 as isize) as i32
                    & _ISspace as i32
                    != 0
            {
                return 1;
            }
            i = i.wrapping_add(1);
        }
        return 0;
    }
}

#[no_mangle]
pub extern "C" fn lil_list_to_value(mut list: lil_list_t, mut do_escape: i32) -> lil_value_t {
    let mut val = alloc_value(0 as *const i8);
    let mut i: u64 = 0;
    i = 0;
    unsafe {
        while i < (*list).c {
            let mut escape = if do_escape != 0 {
                needs_escape(lil_to_string(*((*list).v).offset(i as isize)))
            } else {
                0
            };
            if i != 0 {
                lil_append_char(val, ' ' as i8);
            }
            if escape != 0 {
                lil_append_char(val, '{' as i8);
            }
            lil_append_val(val, *((*list).v).offset(i as isize));
            if escape != 0 {
                lil_append_char(val, '}' as i8);
            }
            i = i.wrapping_add(1);
        }
    }
    return val;
}

#[no_mangle]
pub extern "C" fn lil_alloc_env(mut parent: lil_env_t) -> lil_env_t {
    unsafe {
        let mut env = calloc(1, ::std::mem::size_of::<_lil_env_t>() as u64) as lil_env_t;
        let ref mut fresh16 = (*env).parent;
        *fresh16 = parent;
        return env;
    }
}

#[no_mangle]
pub extern "C" fn lil_free_env(mut env: lil_env_t) {
    let mut i: u64 = 0;
    if env.is_null() {
        return;
    }
    lil_free_value((*env).retval);
    i = 0;
    unsafe {
        while i < (*env).vars {
            free((**((*env).var).offset(i as isize)).n as *mut libc::c_void);
            lil_free_value((**((*env).var).offset(i as isize)).v);
            free(*((*env).var).offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1);
        }
        free((*env).var as *mut libc::c_void);
        free(env as *mut libc::c_void);
    }
}

extern "C" fn lil_find_local_var(
    mut lil: lil_t,
    mut env: lil_env_t,
    mut name: *const i8,
) -> lil_var_t {
    unsafe {
        if (*env).vars > 0 {
            let mut i = ((*env).vars).wrapping_sub(1);
            loop {
                if strcmp((**((*env).var).offset(i as isize)).n, name) == 0 {
                    return *((*env).var).offset(i as isize);
                }
                if i == 0 {
                    break;
                }
                i = i.wrapping_sub(1);
            }
        }
        return 0 as lil_var_t;
    }
}

extern "C" fn lil_find_var(mut lil: lil_t, mut env: lil_env_t, mut name: *const i8) -> lil_var_t {
    unsafe {
        let mut r = lil_find_local_var(lil, env, name);
        return if !r.is_null() {
            r
        } else if env == (*lil).rootenv {
            0 as lil_var_t
        } else {
            lil_find_var(lil, (*lil).rootenv, name)
        };
    }
}

extern "C" fn find_cmd(mut lil: lil_t, mut name: *const i8) -> lil_func_t {
    unsafe {
        if (*lil).cmds > 0 {
            let mut i = ((*lil).cmds).wrapping_sub(1);
            loop {
                if strcmp((**((*lil).cmd).offset(i as isize)).name, name) == 0 {
                    return *((*lil).cmd).offset(i as isize);
                }
                if i == 0 {
                    break;
                }
                i = i.wrapping_sub(1);
            }
        }
        return 0 as lil_func_t;
    }
}

extern "C" fn add_func(mut lil: lil_t, mut name: *const i8) -> lil_func_t {
    unsafe {
        let mut cmd = 0 as *mut _lil_func_t;
        let mut ncmd = 0 as *mut lil_func_t;
        cmd = find_cmd(lil, name);
        if !cmd.is_null() {
            return cmd;
        }
        cmd = calloc(1, ::std::mem::size_of::<_lil_func_t>() as u64) as lil_func_t;
        let ref mut fresh17 = (*cmd).name;
        *fresh17 = strclone(name);
        ncmd = realloc(
            (*lil).cmd as *mut libc::c_void,
            (::std::mem::size_of::<lil_func_t>() as u64)
                .wrapping_mul(((*lil).cmds).wrapping_add(1)),
        ) as *mut lil_func_t;
        if ncmd.is_null() {
            free(cmd as *mut libc::c_void);
            return 0 as lil_func_t;
        }
        let ref mut fresh18 = (*lil).cmd;
        *fresh18 = ncmd;
        let ref mut fresh19 = (*lil).cmds;
        let fresh20 = *fresh19;
        *fresh19 = (*fresh19).wrapping_add(1);
        let ref mut fresh21 = *ncmd.offset(fresh20 as isize);
        *fresh21 = cmd;
        return cmd;
    }
}

#[no_mangle]
pub extern "C" fn lil_register(
    mut lil: lil_t,
    mut name: *const i8,
    mut proc_0: lil_func_proc_t,
) -> i32 {
    unsafe {
        let mut cmd = add_func(lil, name);
        if cmd.is_null() {
            return 0;
        }
        let ref mut fresh22 = (*cmd).proc_0;
        *fresh22 = proc_0;
        return 1;
    }
}

#[no_mangle]
pub extern "C" fn lil_set_var(
    mut lil: lil_t,
    mut name: *const i8,
    mut val: lil_value_t,
    mut local: i32,
) -> lil_var_t {
    unsafe {
        let mut nvar = 0 as *mut lil_var_t;
        let mut env = if local == 0 {
            (*lil).rootenv
        } else {
            (*lil).env
        };
        let mut freeval = 0;
        if *name.offset(0 as isize) == 0 {
            return 0 as lil_var_t;
        }
        if local != 2 {
            let mut var = lil_find_var(lil, env, name);
            if local == 3 && !var.is_null() && (*var).env == (*lil).rootenv && (*var).env != env {
                var = 0 as lil_var_t;
            }
            if (var.is_null() && env == (*lil).rootenv
                || !var.is_null() && (*var).env == (*lil).rootenv)
                && ((*lil).callback[6 as usize]).is_some()
            {
                let mut proc_0: lil_setvar_callback_proc_t =
                    ::std::mem::transmute::<lil_callback_proc_t, lil_setvar_callback_proc_t>(
                        (*lil).callback[6 as usize],
                    );
                let mut newval = val;
                let mut r = proc_0.expect("non-null function pointer")(lil, name, &mut newval);
                if r < 0 {
                    return 0 as lil_var_t;
                }
                if r != 0 {
                    val = newval;
                    freeval = 1;
                }
            }
            if !var.is_null() {
                lil_free_value((*var).v);
                let ref mut fresh23 = (*var).v;
                *fresh23 = if freeval != 0 {
                    val
                } else {
                    lil_clone_value(val)
                };
                return var;
            }
        }
        nvar = realloc(
            (*env).var as *mut libc::c_void,
            (::std::mem::size_of::<lil_var_t>() as u64).wrapping_mul(((*env).vars).wrapping_add(1)),
        ) as *mut lil_var_t;
        if nvar.is_null() {
            return 0 as lil_var_t;
        }
        let ref mut fresh24 = (*env).var;
        *fresh24 = nvar;
        let ref mut fresh25 = *nvar.offset((*env).vars as isize);
        *fresh25 = calloc(1, ::std::mem::size_of::<_lil_var_t>() as u64) as lil_var_t;
        let ref mut fresh26 = (**nvar.offset((*env).vars as isize)).n;
        *fresh26 = strclone(name);
        let ref mut fresh27 = (**nvar.offset((*env).vars as isize)).env;
        *fresh27 = env;
        let ref mut fresh28 = (**nvar.offset((*env).vars as isize)).v;
        *fresh28 = if freeval != 0 {
            val
        } else {
            lil_clone_value(val)
        };
        let ref mut fresh29 = (*env).vars;
        let fresh30 = *fresh29;
        *fresh29 = (*fresh29).wrapping_add(1);
        return *nvar.offset(fresh30 as isize);
    }
}

#[no_mangle]
pub extern "C" fn lil_get_var(mut lil: lil_t, mut name: *const i8) -> lil_value_t {
    unsafe {
        return lil_get_var_or(lil, name, (*lil).empty);
    }
}

#[no_mangle]
pub extern "C" fn lil_get_var_or(
    mut lil: lil_t,
    mut name: *const i8,
    mut defvalue: lil_value_t,
) -> lil_value_t {
    unsafe {
        let mut var = lil_find_var(lil, (*lil).env, name);
        let mut retval = if !var.is_null() { (*var).v } else { defvalue };
        if ((*lil).callback[7 as usize]).is_some()
            && (var.is_null() || (*var).env == (*lil).rootenv)
        {
            let mut proc_0: lil_getvar_callback_proc_t = ::std::mem::transmute::<
                lil_callback_proc_t,
                lil_getvar_callback_proc_t,
            >((*lil).callback[7 as usize]);
            let mut newretval = retval;
            if proc_0.expect("non-null function pointer")(lil, name, &mut newretval) != 0 {
                retval = newretval;
            }
        }
        return retval;
    }
}

#[no_mangle]
pub extern "C" fn lil_push_env(mut lil: lil_t) -> lil_env_t {
    let mut env = lil_alloc_env((*lil).env);
    let ref mut fresh31 = (*lil).env;
    *fresh31 = env;
    return env;
}

#[no_mangle]
pub extern "C" fn lil_pop_env(mut lil: lil_t) {
    if !((*(*lil).env).parent).is_null() {
        let mut next = (*(*lil).env).parent;
        lil_free_env((*lil).env);
        let ref mut fresh32 = (*lil).env;
        *fresh32 = next;
    }
}

#[no_mangle]
pub extern "C" fn lil_new() -> lil_t {
    unsafe {
        let mut lil = calloc(1, ::std::mem::size_of::<_lil_t>() as u64) as lil_t;
        let ref mut fresh33 = (*lil).env;
        *fresh33 = lil_alloc_env(0 as lil_env_t);
        let ref mut fresh34 = (*lil).rootenv;
        *fresh34 = *fresh33;
        let ref mut fresh35 = (*lil).empty;
        *fresh35 = alloc_value(0 as *const i8);
        let ref mut fresh36 = (*lil).dollarprefix;
        *fresh36 = strclone(b"set \0" as *const u8 as *const i8);
        register_stdcmds(lil);
        return lil;
    }
}

extern "C" fn islilspecial(mut ch: i8) -> i32 {
    return (ch as i32 == ';' as i32
        || ch as i32 == '$' as i32
        || ch as i32 == '[' as i32
        || ch as i32 == ']' as i32
        || ch as i32 == '{' as i32
        || ch as i32 == '}' as i32
        || ch as i32 == '"' as i32
        || ch as i32 == '\'' as i32) as i32;
}

extern "C" fn ateol(mut lil: lil_t) -> i32 {
    unsafe {
        return ((*lil).ignoreeol == 0
            && (*((*lil).code).offset((*lil).head as isize) as i32 == '\n' as i32
                || *((*lil).code).offset((*lil).head as isize) as i32 == '\r' as i32
                || *((*lil).code).offset((*lil).head as isize) as i32 == ';' as i32))
            as i32;
    }
}

extern "C" fn skip_spaces(mut lil: lil_t) {
    unsafe {
        while (*lil).head < (*lil).clen
            && (*((*lil).code).offset((*lil).head as isize) as i32 == '\\' as i32
                || *((*lil).code).offset((*lil).head as isize) as i32 == '#' as i32
                || *(*__ctype_b_loc())
                    .offset(*((*lil).code).offset((*lil).head as isize) as i32 as isize)
                    as i32
                    & _ISspace as i32
                    != 0
                    && ((*lil).ignoreeol != 0
                        || !(*((*lil).code).offset((*lil).head as isize) as i32 == '\r' as i32
                            || *((*lil).code).offset((*lil).head as isize) as i32 == '\n' as i32)))
        {
            if *((*lil).code).offset((*lil).head as isize) as i32 == '#' as i32 {
                while (*lil).head < (*lil).clen && ateol(lil) == 0 {
                    let ref mut fresh37 = (*lil).head;
                    *fresh37 = (*fresh37).wrapping_add(1);
                }
            } else if *((*lil).code).offset((*lil).head as isize) as i32 == '\\' as i32
                && (*((*lil).code).offset(((*lil).head).wrapping_add(1) as isize) as i32
                    == '\r' as i32
                    || *((*lil).code).offset(((*lil).head).wrapping_add(1) as isize) as i32
                        == '\n' as i32)
            {
                let ref mut fresh38 = (*lil).head;
                *fresh38 = (*fresh38).wrapping_add(1);
                while (*lil).head < (*lil).clen && ateol(lil) != 0 {
                    let ref mut fresh39 = (*lil).head;
                    *fresh39 = (*fresh39).wrapping_add(1);
                }
            } else {
                let ref mut fresh40 = (*lil).head;
                *fresh40 = (*fresh40).wrapping_add(1);
            }
        }
    }
}

extern "C" fn get_bracketpart(mut lil: lil_t) -> lil_value_t {
    let mut cnt = 1;
    let mut val = 0 as *mut _lil_value_t;
    let mut cmd = alloc_value(0 as *const i8);
    let ref mut fresh41 = (*lil).head;
    *fresh41 = (*fresh41).wrapping_add(1);
    unsafe {
        while (*lil).head < (*lil).clen {
            if *((*lil).code).offset((*lil).head as isize) as i32 == '[' as i32 {
                let ref mut fresh42 = (*lil).head;
                *fresh42 = (*fresh42).wrapping_add(1);
                cnt = cnt.wrapping_add(1);
                lil_append_char(cmd, '[' as i8);
            } else if *((*lil).code).offset((*lil).head as isize) as i32 == ']' as i32 {
                let ref mut fresh43 = (*lil).head;
                *fresh43 = (*fresh43).wrapping_add(1);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
                lil_append_char(cmd, ']' as i8);
            } else {
                let ref mut fresh44 = (*lil).head;
                let fresh45 = *fresh44;
                *fresh44 = (*fresh44).wrapping_add(1);
                lil_append_char(cmd, *((*lil).code).offset(fresh45 as isize));
            }
        }
    }
    val = lil_parse_value(lil, cmd, 0);
    lil_free_value(cmd);
    return val;
}

extern "C" fn get_dollarpart(mut lil: lil_t) -> lil_value_t {
    let mut val = 0 as *mut _lil_value_t;
    let mut name = 0 as *mut _lil_value_t;
    let mut tmp = 0 as *mut _lil_value_t;
    let ref mut fresh46 = (*lil).head;
    *fresh46 = (*fresh46).wrapping_add(1);
    name = next_word(lil);
    tmp = alloc_value((*lil).dollarprefix);
    lil_append_val(tmp, name);
    lil_free_value(name);
    val = lil_parse_value(lil, tmp, 0);
    lil_free_value(tmp);
    return val;
}

extern "C" fn next_word(mut lil: lil_t) -> lil_value_t {
    let mut val = 0 as *mut _lil_value_t;
    skip_spaces(lil);
    unsafe {
        if *((*lil).code).offset((*lil).head as isize) as i32 == '$' as i32 {
            val = get_dollarpart(lil);
        } else if *((*lil).code).offset((*lil).head as isize) as i32 == '{' as i32 {
            let mut cnt = 1;
            let ref mut fresh47 = (*lil).head;
            *fresh47 = (*fresh47).wrapping_add(1);
            val = alloc_value(0 as *const i8);
            while (*lil).head < (*lil).clen {
                if *((*lil).code).offset((*lil).head as isize) as i32 == '{' as i32 {
                    let ref mut fresh48 = (*lil).head;
                    *fresh48 = (*fresh48).wrapping_add(1);
                    cnt = cnt.wrapping_add(1);
                    lil_append_char(val, '{' as i8);
                } else if *((*lil).code).offset((*lil).head as isize) as i32 == '}' as i32 {
                    let ref mut fresh49 = (*lil).head;
                    *fresh49 = (*fresh49).wrapping_add(1);
                    cnt = cnt.wrapping_sub(1);
                    if cnt == 0 {
                        break;
                    }
                    lil_append_char(val, '}' as i8);
                } else {
                    let ref mut fresh50 = (*lil).head;
                    let fresh51 = *fresh50;
                    *fresh50 = (*fresh50).wrapping_add(1);
                    lil_append_char(val, *((*lil).code).offset(fresh51 as isize));
                }
            }
        } else if *((*lil).code).offset((*lil).head as isize) as i32 == '[' as i32 {
            val = get_bracketpart(lil);
        } else if *((*lil).code).offset((*lil).head as isize) as i32 == '"' as i32
            || *((*lil).code).offset((*lil).head as isize) as i32 == '\'' as i32
        {
            let ref mut fresh52 = (*lil).head;
            let fresh53 = *fresh52;
            *fresh52 = (*fresh52).wrapping_add(1);
            let mut sc = *((*lil).code).offset(fresh53 as isize);
            val = alloc_value(0 as *const i8);
            while (*lil).head < (*lil).clen {
                if *((*lil).code).offset((*lil).head as isize) as i32 == '[' as i32
                    || *((*lil).code).offset((*lil).head as isize) as i32 == '$' as i32
                {
                    let mut tmp =
                        if *((*lil).code).offset((*lil).head as isize) as i32 == '$' as i32 {
                            get_dollarpart(lil)
                        } else {
                            get_bracketpart(lil)
                        };
                    lil_append_val(val, tmp);
                    lil_free_value(tmp);
                    let ref mut fresh54 = (*lil).head;
                    *fresh54 = (*fresh54).wrapping_sub(1);
                } else if *((*lil).code).offset((*lil).head as isize) as i32 == '\\' as i32 {
                    let ref mut fresh55 = (*lil).head;
                    *fresh55 = (*fresh55).wrapping_add(1);
                    match *((*lil).code).offset((*lil).head as isize) as i32 {
                        98 => {
                            lil_append_char(val, '\u{8}' as i8);
                        }
                        116 => {
                            lil_append_char(val, '\t' as i8);
                        }
                        110 => {
                            lil_append_char(val, '\n' as i8);
                        }
                        118 => {
                            lil_append_char(val, '\u{b}' as i8);
                        }
                        102 => {
                            lil_append_char(val, '\u{c}' as i8);
                        }
                        114 => {
                            lil_append_char(val, '\r' as i8);
                        }
                        48 => {
                            lil_append_char(val, 0);
                        }
                        97 => {
                            lil_append_char(val, '\u{7}' as i8);
                        }
                        99 => {
                            lil_append_char(val, '}' as i8);
                        }
                        111 => {
                            lil_append_char(val, '{' as i8);
                        }
                        _ => {
                            lil_append_char(val, *((*lil).code).offset((*lil).head as isize));
                        }
                    }
                } else if *((*lil).code).offset((*lil).head as isize) as i32 == sc as i32 {
                    let ref mut fresh56 = (*lil).head;
                    *fresh56 = (*fresh56).wrapping_add(1);
                    break;
                } else {
                    lil_append_char(val, *((*lil).code).offset((*lil).head as isize));
                }
                let ref mut fresh57 = (*lil).head;
                *fresh57 = (*fresh57).wrapping_add(1);
            }
        } else {
            val = alloc_value(0 as *const i8);
            while (*lil).head < (*lil).clen
                && *(*__ctype_b_loc())
                    .offset(*((*lil).code).offset((*lil).head as isize) as i32 as isize)
                    as i32
                    & _ISspace as i32
                    == 0
                && islilspecial(*((*lil).code).offset((*lil).head as isize)) == 0
            {
                let ref mut fresh58 = (*lil).head;
                let fresh59 = *fresh58;
                *fresh58 = (*fresh58).wrapping_add(1);
                lil_append_char(val, *((*lil).code).offset(fresh59 as isize));
            }
        }
    }
    return if !val.is_null() {
        val
    } else {
        alloc_value(0 as *const i8)
    };
}

extern "C" fn substitute(mut lil: lil_t) -> lil_list_t {
    let mut words = lil_alloc_list();
    skip_spaces(lil);
    unsafe {
        while (*lil).head < (*lil).clen && ateol(lil) == 0 && (*lil).error == 0 {
            let mut w = alloc_value(0 as *const i8);
            loop {
                let mut head = (*lil).head;
                let mut wp = next_word(lil);
                if head == (*lil).head {
                    lil_free_value(w);
                    lil_free_value(wp);
                    lil_free_list(words);
                    return 0 as lil_list_t;
                }
                lil_append_val(w, wp);
                lil_free_value(wp);
                if !((*lil).head < (*lil).clen
                    && ateol(lil) == 0
                    && *(*__ctype_b_loc())
                        .offset(*((*lil).code).offset((*lil).head as isize) as i32 as isize)
                        as i32
                        & _ISspace as i32
                        == 0
                    && (*lil).error == 0)
                {
                    break;
                }
            }
            skip_spaces(lil);
            lil_list_append(words, w);
        }
    }
    return words;
}

#[no_mangle]
pub extern "C" fn lil_subst_to_list(mut lil: lil_t, mut code: lil_value_t) -> lil_list_t {
    let mut save_code = (*lil).code;
    let mut save_clen = (*lil).clen;
    let mut save_head = (*lil).head;
    let mut save_igeol = (*lil).ignoreeol;
    let mut words = 0 as *mut _lil_list_t;
    let ref mut fresh60 = (*lil).code;
    *fresh60 = lil_to_string(code);
    (*lil).clen = (*code).l;
    (*lil).head = 0;
    (*lil).ignoreeol = 1;
    words = substitute(lil);
    let ref mut fresh61 = (*lil).code;
    *fresh61 = save_code;
    (*lil).clen = save_clen;
    (*lil).head = save_head;
    (*lil).ignoreeol = save_igeol;
    return words;
}

#[no_mangle]
pub extern "C" fn lil_subst_to_value(mut lil: lil_t, mut code: lil_value_t) -> lil_value_t {
    let mut words = lil_subst_to_list(lil, code);
    let mut val = 0 as *mut _lil_value_t;
    if words.is_null() {
        return lil_clone_value(code);
    }
    val = lil_list_to_value(words, 0);
    lil_free_list(words);
    return val;
}

#[no_mangle]
pub extern "C" fn lil_parse(
    mut lil: lil_t,
    mut code: *const i8,
    mut codelen: u64,
    mut funclevel: i32,
) -> lil_value_t {
    unsafe {
        let mut save_code = (*lil).code;
        let mut save_clen = (*lil).clen;
        let mut save_head = (*lil).head;
        let mut val = 0 as lil_value_t;
        let mut words = 0 as lil_list_t;
        if save_code.is_null() {
            let ref mut fresh62 = (*lil).rootcode;
            *fresh62 = code;
        }
        let ref mut fresh63 = (*lil).code;
        *fresh63 = code;
        (*lil).clen = if codelen != 0 { codelen } else { strlen(code) };
        (*lil).head = 0;
        skip_spaces(lil);
        let ref mut fresh64 = (*lil).parse_depth;
        *fresh64 = (*fresh64).wrapping_add(1);
        if (*lil).parse_depth == 1 {
            (*lil).error = 0;
        }
        if funclevel != 0 {
            (*(*lil).env).breakrun = 0;
        }
        while (*lil).head < (*lil).clen && (*lil).error == 0 {
            if !words.is_null() {
                lil_free_list(words);
            }
            if !val.is_null() {
                lil_free_value(val);
            }
            val = 0 as lil_value_t;
            words = substitute(lil);
            if words.is_null() || (*lil).error != 0 {
                break;
            }
            if (*words).c != 0 {
                let mut cmd = find_cmd(lil, lil_to_string(*((*words).v).offset(0 as isize)));
                if cmd.is_null() {
                    if (**((*words).v).offset(0 as isize)).l != 0 {
                        if !((*lil).catcher).is_null() {
                            if (*lil).in_catcher < 16384 {
                                let mut args = 0 as *mut _lil_value_t;
                                let ref mut fresh65 = (*lil).in_catcher;
                                *fresh65 += 1;
                                lil_push_env(lil);
                                let ref mut fresh66 = (*(*lil).env).catcher_for;
                                *fresh66 = *((*words).v).offset(0 as isize);
                                args = lil_list_to_value(words, 1);
                                lil_set_var(lil, b"args\0" as *const u8 as *const i8, args, 2);
                                lil_free_value(args);
                                val = lil_parse(lil, (*lil).catcher, 0, 1);
                                lil_pop_env(lil);
                                let ref mut fresh67 = (*lil).in_catcher;
                                *fresh67 -= 1;
                            } else {
                                let mut msg = malloc(
                                    ((**((*words).v).offset(0 as isize)).l).wrapping_add(64),
                                ) as *mut i8;
                                sprintf (msg, b"catcher limit reached while trying to call unknown function %s\0" as * const u8 as * const i8, (** ((* words).v).offset (0 as isize)).d,);
                                lil_set_error_at(lil, (*lil).head, msg);
                                free(msg as *mut libc::c_void);
                                break;
                            }
                        } else {
                            let mut msg_0 =
                                malloc(((**((*words).v).offset(0 as isize)).l).wrapping_add(32))
                                    as *mut i8;
                            sprintf(
                                msg_0,
                                b"unknown function %s\0" as *const u8 as *const i8,
                                (**((*words).v).offset(0 as isize)).d,
                            );
                            lil_set_error_at(lil, (*lil).head, msg_0);
                            free(msg_0 as *mut libc::c_void);
                            break;
                        }
                    }
                }
                if !cmd.is_null() {
                    if ((*cmd).proc_0).is_some() {
                        let mut shead = (*lil).head;
                        val = ((*cmd).proc_0).expect("non-null function pointer")(
                            lil,
                            ((*words).c).wrapping_sub(1),
                            ((*words).v).offset(1 as isize),
                        );
                        if (*lil).error == 2 {
                            (*lil).error = 1;
                            (*lil).err_head = shead;
                        }
                    } else {
                        lil_push_env(lil);
                        let ref mut fresh68 = (*(*lil).env).func;
                        *fresh68 = cmd;
                        if (*(*cmd).argnames).c == 1
                            && strcmp(
                                lil_to_string(*((*(*cmd).argnames).v).offset(0 as isize)),
                                b"args\0" as *const u8 as *const i8,
                            ) == 0
                        {
                            let mut args_0 = lil_list_to_value(words, 1);
                            lil_set_var(lil, b"args\0" as *const u8 as *const i8, args_0, 2);
                            lil_free_value(args_0);
                        } else {
                            let mut i: u64 = 0;
                            i = 0;
                            while i < (*(*cmd).argnames).c {
                                lil_set_var(
                                    lil,
                                    lil_to_string(*((*(*cmd).argnames).v).offset(i as isize)),
                                    if i < ((*words).c).wrapping_sub(1) {
                                        *((*words).v).offset(i.wrapping_add(1) as isize)
                                    } else {
                                        (*lil).empty
                                    },
                                    2,
                                );
                                i = i.wrapping_add(1);
                            }
                        }
                        val = lil_parse_value(lil, (*cmd).code, 1);
                        lil_pop_env(lil);
                    }
                }
            }
            if (*(*lil).env).breakrun != 0 {
                break;
            }
            skip_spaces(lil);
            while ateol(lil) != 0 {
                let ref mut fresh69 = (*lil).head;
                *fresh69 = (*fresh69).wrapping_add(1);
            }
            skip_spaces(lil);
        }
        if (*lil).error != 0 && ((*lil).callback[5 as usize]).is_some() && (*lil).parse_depth == 1 {
            let mut proc_0: lil_error_callback_proc_t = ::std::mem::transmute::<
                lil_callback_proc_t,
                lil_error_callback_proc_t,
            >((*lil).callback[5 as usize]);
            proc_0.expect("non-null function pointer")(lil, (*lil).err_head, (*lil).err_msg);
        }
        if !words.is_null() {
            lil_free_list(words);
        }
        let ref mut fresh70 = (*lil).code;
        *fresh70 = save_code;
        (*lil).clen = save_clen;
        (*lil).head = save_head;
        if funclevel != 0 && (*(*lil).env).retval_set != 0 {
            if !val.is_null() {
                lil_free_value(val);
            }
            val = (*(*lil).env).retval;
            let ref mut fresh71 = (*(*lil).env).retval;
            *fresh71 = 0 as lil_value_t;
            (*(*lil).env).retval_set = 0;
            (*(*lil).env).breakrun = 0;
        }
        let ref mut fresh72 = (*lil).parse_depth;
        *fresh72 = (*fresh72).wrapping_sub(1);
        return if !val.is_null() {
            val
        } else {
            alloc_value(0 as *const i8)
        };
    }
}

#[no_mangle]
pub extern "C" fn lil_parse_value(
    mut lil: lil_t,
    mut val: lil_value_t,
    mut funclevel: i32,
) -> lil_value_t {
    if val.is_null() || ((*val).d).is_null() || (*val).l == 0 {
        return alloc_value(0 as *const i8);
    }
    return lil_parse(lil, (*val).d, (*val).l, funclevel);
}

#[no_mangle]
pub extern "C" fn lil_callback(mut lil: lil_t, mut cb: i32, mut proc_0: lil_callback_proc_t) {
    if cb < 0 || cb > 8 {
        return;
    }
    let ref mut fresh73 = (*lil).callback[cb as usize];
    unsafe {
        *fresh73 = proc_0;
    }
}

#[no_mangle]
pub extern "C" fn lil_set_error(mut lil: lil_t, mut msg: *const i8) {
    unsafe {
        if (*lil).error != 0 {
            return;
        }
        free((*lil).err_msg as *mut libc::c_void);
        (*lil).error = 2;
        (*lil).err_head = 0;
        let ref mut fresh74 = (*lil).err_msg;
        *fresh74 = strclone(if !msg.is_null() {
            msg
        } else {
            b"\0" as *const u8 as *const i8
        });
    }
}

#[no_mangle]
pub extern "C" fn lil_set_error_at(mut lil: lil_t, mut pos: u64, mut msg: *const i8) {
    unsafe {
        if (*lil).error != 0 {
            return;
        }
        free((*lil).err_msg as *mut libc::c_void);
        (*lil).error = 1;
        (*lil).err_head = pos;
        let ref mut fresh75 = (*lil).err_msg;
        *fresh75 = strclone(if !msg.is_null() {
            msg
        } else {
            b"\0" as *const u8 as *const i8
        });
    }
}

#[no_mangle]
pub extern "C" fn lil_error(mut lil: lil_t, mut msg: *mut *const i8, mut pos: *mut u64) -> i32 {
    unsafe {
        if (*lil).error == 0 {
            return 0;
        }
        *msg = (*lil).err_msg;
        *pos = (*lil).err_head;
        (*lil).error = 0;
        return 1;
    }
}

extern "C" fn ee_skip_spaces(mut ee: *mut expreval_t) {
    unsafe {
        while (*ee).head < (*ee).len
            && *(*__ctype_b_loc()).offset(*((*ee).code).offset((*ee).head as isize) as i32 as isize)
                as i32
                & _ISspace as i32
                != 0
        {
            let ref mut fresh76 = (*ee).head;
            *fresh76 = (*fresh76).wrapping_add(1);
        }
    }
}

extern "C" fn ee_numeric_element(mut ee: *mut expreval_t) {
    unsafe {
        let mut fpart = 0;
        let mut fpartlen = 1;
        (*ee).type_0 = 0;
        ee_skip_spaces(ee);
        (*ee).ival = 0;
        (*ee).dval = 0 as f64;
        while (*ee).head < (*ee).len {
            if *((*ee).code).offset((*ee).head as isize) as i32 == '.' as i32 {
                if (*ee).type_0 == 1 {
                    break;
                };
                (*ee).type_0 = 1;
                let ref mut fresh77 = (*ee).head;
                *fresh77 = (*fresh77).wrapping_add(1);
            } else if *(*__ctype_b_loc())
                .offset(*((*ee).code).offset((*ee).head as isize) as i32 as isize)
                as i32
                & _ISdigit as i32
                == 0
            {
                break;
            }
            if (*ee).type_0 == 0 {
                (*ee).ival = (*ee).ival * 10
                    + (*((*ee).code).offset((*ee).head as isize) as i32 - '0' as i32) as i64;
            } else {
                fpart = fpart * 10
                    + (*((*ee).code).offset((*ee).head as isize) as i32 - '0' as i32) as i64;
                fpartlen *= 10;
            }
            let ref mut fresh78 = (*ee).head;
            *fresh78 = (*fresh78).wrapping_add(1);
        }
        if (*ee).type_0 == 1 {
            (*ee).dval = (*ee).ival as f64 + fpart as f64 / fpartlen as f64;
        }
    }
}

extern "C" fn ee_element(mut ee: *mut expreval_t) {
    unsafe {
        if *(*__ctype_b_loc()).offset(*((*ee).code).offset((*ee).head as isize) as i32 as isize)
            as i32
            & _ISdigit as i32
            != 0
        {
            ee_numeric_element(ee);
            return;
        };
        (*ee).type_0 = 0;
        (*ee).ival = 1;
        (*ee).error = 4;
    }
}

extern "C" fn ee_paren(mut ee: *mut expreval_t) {
    unsafe {
        ee_skip_spaces(ee);
        if *((*ee).code).offset((*ee).head as isize) as i32 == '(' as i32 {
            let ref mut fresh79 = (*ee).head;
            *fresh79 = (*fresh79).wrapping_add(1);
            ee_expr(ee);
            ee_skip_spaces(ee);
            if *((*ee).code).offset((*ee).head as isize) as i32 == ')' as i32 {
                let ref mut fresh80 = (*ee).head;
                *fresh80 = (*fresh80).wrapping_add(1);
            } else {
                (*ee).error = 1;
            }
        } else {
            ee_element(ee);
        };
    }
}

extern "C" fn ee_unary(mut ee: *mut expreval_t) {
    unsafe {
        ee_skip_spaces(ee);
        if (*ee).head < (*ee).len
            && (*ee).error == 0
            && (*((*ee).code).offset((*ee).head as isize) as i32 == '-' as i32
                || *((*ee).code).offset((*ee).head as isize) as i32 == '+' as i32
                || *((*ee).code).offset((*ee).head as isize) as i32 == '~' as i32
                || *((*ee).code).offset((*ee).head as isize) as i32 == '!' as i32)
        {
            let ref mut fresh81 = (*ee).head;
            let fresh82 = *fresh81;
            *fresh81 = (*fresh81).wrapping_add(1);
            let mut op = *((*ee).code).offset(fresh82 as isize);
            ee_unary(ee);
            if (*ee).error != 0 {
                return;
            }
            match op as i32 {
                45 => match (*ee).type_0 {
                    1 => {
                        (*ee).dval = -(*ee).dval;
                    }
                    0 => {
                        (*ee).ival = -(*ee).ival;
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                126 => match (*ee).type_0 {
                    1 => {
                        (*ee).ival = !((*ee).dval as i64);
                        (*ee).type_0 = 0;
                    }
                    0 => {
                        (*ee).ival = !(*ee).ival;
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                33 => match (*ee).type_0 {
                    1 => {
                        (*ee).dval = ((*ee).dval == 0.) as i32 as f64;
                    }
                    0 => {
                        (*ee).ival = ((*ee).ival == 0) as i64;
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                43 | _ => {}
            }
        } else {
            ee_paren(ee);
        };
    }
}

extern "C" fn ee_muldiv(mut ee: *mut expreval_t) {
    unsafe {
        ee_unary(ee);
        if (*ee).error != 0 {
            return;
        }
        ee_skip_spaces(ee);
        while (*ee).head < (*ee).len
            && (*ee).error == 0
            && *(*__ctype_b_loc())
                .offset(*((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32 as isize)
                as i32
                & _ISpunct as i32
                == 0
            && (*((*ee).code).offset((*ee).head as isize) as i32 == '*' as i32
                || *((*ee).code).offset((*ee).head as isize) as i32 == '/' as i32
                || *((*ee).code).offset((*ee).head as isize) as i32 == '\\' as i32
                || *((*ee).code).offset((*ee).head as isize) as i32 == '%' as i32)
        {
            let mut odval = (*ee).dval;
            let mut oival = (*ee).ival;
            match *((*ee).code).offset((*ee).head as isize) as i32 {
                42 => match (*ee).type_0 {
                    1 => {
                        let ref mut fresh83 = (*ee).head;
                        *fresh83 = (*fresh83).wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).dval = (*ee).dval * odval;
                            }
                            0 => {
                                (*ee).dval = (*ee).ival as f64 * odval;
                                (*ee).type_0 = 1;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        let ref mut fresh84 = (*ee).head;
                        *fresh84 = (*fresh84).wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).dval = (*ee).dval * oival as f64;
                                (*ee).type_0 = 1;
                            }
                            0 => {
                                (*ee).ival = (*ee).ival * oival;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                37 => match (*ee).type_0 {
                    1 => {
                        let ref mut fresh85 = (*ee).head;
                        *fresh85 = (*fresh85).wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = 3;
                                } else {
                                    (*ee).dval = fmod(odval, (*ee).dval);
                                }
                            }
                            0 => {
                                if (*ee).ival == 0 {
                                    (*ee).error = 3;
                                } else {
                                    (*ee).dval = fmod(odval, (*ee).ival as f64);
                                };
                                (*ee).type_0 = 1;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        let ref mut fresh86 = (*ee).head;
                        *fresh86 = (*fresh86).wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = 3;
                                } else {
                                    (*ee).dval = fmod(oival as f64, (*ee).dval);
                                }
                            }
                            0 => {
                                if (*ee).ival == 0 {
                                    (*ee).error = 3;
                                } else {
                                    (*ee).ival = oival % (*ee).ival;
                                }
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {}
                },
                47 => match (*ee).type_0 {
                    1 => {
                        let ref mut fresh87 = (*ee).head;
                        *fresh87 = (*fresh87).wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = 3;
                                } else {
                                    (*ee).dval = odval / (*ee).dval;
                                }
                            }
                            0 => {
                                if (*ee).ival == 0 {
                                    (*ee).error = 3;
                                } else {
                                    (*ee).dval = odval / (*ee).ival as f64;
                                };
                                (*ee).type_0 = 1;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        let ref mut fresh88 = (*ee).head;
                        *fresh88 = (*fresh88).wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = 3;
                                } else {
                                    (*ee).dval = oival as f64 / (*ee).dval;
                                }
                            }
                            0 => {
                                if (*ee).ival == 0 {
                                    (*ee).error = 3;
                                } else {
                                    (*ee).dval = oival as f64 / (*ee).ival as f64;
                                };
                                (*ee).type_0 = 1;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {}
                },
                92 => match (*ee).type_0 {
                    1 => {
                        let ref mut fresh89 = (*ee).head;
                        *fresh89 = (*fresh89).wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = 3;
                                } else {
                                    (*ee).ival = (odval / (*ee).dval) as i64;
                                };
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                if (*ee).ival == 0 {
                                    (*ee).error = 3;
                                } else {
                                    (*ee).ival = (odval / (*ee).ival as f64) as i64;
                                }
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        let ref mut fresh90 = (*ee).head;
                        *fresh90 = (*fresh90).wrapping_add(1);
                        ee_unary(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                if (*ee).dval == 0.0f64 {
                                    (*ee).error = 3;
                                } else {
                                    (*ee).ival = (oival as f64 / (*ee).dval) as i64;
                                };
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                if (*ee).ival == 0 {
                                    (*ee).error = 3;
                                } else {
                                    (*ee).ival = oival / (*ee).ival;
                                }
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                _ => {}
            }
            ee_skip_spaces(ee);
        }
    }
}

extern "C" fn ee_addsub(mut ee: *mut expreval_t) {
    unsafe {
        ee_muldiv(ee);
        ee_skip_spaces(ee);
        while (*ee).head < (*ee).len
            && (*ee).error == 0
            && *(*__ctype_b_loc())
                .offset(*((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32 as isize)
                as i32
                & _ISpunct as i32
                == 0
            && (*((*ee).code).offset((*ee).head as isize) as i32 == '+' as i32
                || *((*ee).code).offset((*ee).head as isize) as i32 == '-' as i32)
        {
            let mut odval = (*ee).dval;
            let mut oival = (*ee).ival;
            match *((*ee).code).offset((*ee).head as isize) as i32 {
                43 => match (*ee).type_0 {
                    1 => {
                        let ref mut fresh91 = (*ee).head;
                        *fresh91 = (*fresh91).wrapping_add(1);
                        ee_muldiv(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).dval = (*ee).dval + odval;
                            }
                            0 => {
                                (*ee).dval = (*ee).ival as f64 + odval;
                                (*ee).type_0 = 1;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        let ref mut fresh92 = (*ee).head;
                        *fresh92 = (*fresh92).wrapping_add(1);
                        ee_muldiv(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).dval = (*ee).dval + oival as f64;
                                (*ee).type_0 = 1;
                            }
                            0 => {
                                (*ee).ival = (*ee).ival + oival;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                45 => match (*ee).type_0 {
                    1 => {
                        let ref mut fresh93 = (*ee).head;
                        *fresh93 = (*fresh93).wrapping_add(1);
                        ee_muldiv(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).dval = odval - (*ee).dval;
                            }
                            0 => {
                                (*ee).dval = odval - (*ee).ival as f64;
                                (*ee).type_0 = 1;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        let ref mut fresh94 = (*ee).head;
                        *fresh94 = (*fresh94).wrapping_add(1);
                        ee_muldiv(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).dval = oival as f64 - (*ee).dval;
                                (*ee).type_0 = 1;
                            }
                            0 => {
                                (*ee).ival = oival - (*ee).ival;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                _ => {}
            }
            ee_skip_spaces(ee);
        }
    }
}

extern "C" fn ee_shift(mut ee: *mut expreval_t) {
    unsafe {
        ee_addsub(ee);
        ee_skip_spaces(ee);
        while (*ee).head < (*ee).len
            && (*ee).error == 0
            && (*((*ee).code).offset((*ee).head as isize) as i32 == '<' as i32
                && *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32 == '<' as i32
                || *((*ee).code).offset((*ee).head as isize) as i32 == '>' as i32
                    && *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32
                        == '>' as i32)
        {
            let mut odval = (*ee).dval;
            let mut oival = (*ee).ival;
            let ref mut fresh95 = (*ee).head;
            *fresh95 = (*fresh95).wrapping_add(1);
            match *((*ee).code).offset((*ee).head as isize) as i32 {
                60 => match (*ee).type_0 {
                    1 => {
                        let ref mut fresh96 = (*ee).head;
                        *fresh96 = (*fresh96).wrapping_add(1);
                        ee_addsub(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival = (odval as i64) << (*ee).dval as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival = (odval as i64) << (*ee).ival;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        let ref mut fresh97 = (*ee).head;
                        *fresh97 = (*fresh97).wrapping_add(1);
                        ee_addsub(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival = oival << (*ee).dval as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival = oival << (*ee).ival;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                62 => match (*ee).type_0 {
                    1 => {
                        let ref mut fresh98 = (*ee).head;
                        *fresh98 = (*fresh98).wrapping_add(1);
                        ee_addsub(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival = odval as i64 >> (*ee).dval as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival = odval as i64 >> (*ee).ival;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        let ref mut fresh99 = (*ee).head;
                        *fresh99 = (*fresh99).wrapping_add(1);
                        ee_addsub(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival = oival >> (*ee).dval as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival = oival >> (*ee).ival;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                _ => {}
            }
            ee_skip_spaces(ee);
        }
    }
}

extern "C" fn ee_compare(mut ee: *mut expreval_t) {
    unsafe {
        ee_shift(ee);
        ee_skip_spaces(ee);
        while (*ee).head < (*ee).len
            && (*ee).error == 0
            && (*((*ee).code).offset((*ee).head as isize) as i32 == '<' as i32
                && *(*__ctype_b_loc()).offset(
                    *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32 as isize,
                ) as i32
                    & _ISpunct as i32
                    == 0
                || *((*ee).code).offset((*ee).head as isize) as i32 == '>' as i32
                    && *(*__ctype_b_loc()).offset(
                        *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32 as isize,
                    ) as i32
                        & _ISpunct as i32
                        == 0
                || *((*ee).code).offset((*ee).head as isize) as i32 == '<' as i32
                    && *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32
                        == '=' as i32
                || *((*ee).code).offset((*ee).head as isize) as i32 == '>' as i32
                    && *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32
                        == '=' as i32)
        {
            let mut odval = (*ee).dval;
            let mut oival = (*ee).ival;
            let mut op = 4;
            if *((*ee).code).offset((*ee).head as isize) as i32 == '<' as i32
                && *(*__ctype_b_loc()).offset(
                    *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32 as isize,
                ) as i32
                    & _ISpunct as i32
                    == 0
            {
                op = 1;
            } else if *((*ee).code).offset((*ee).head as isize) as i32 == '>' as i32
                && *(*__ctype_b_loc()).offset(
                    *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32 as isize,
                ) as i32
                    & _ISpunct as i32
                    == 0
            {
                op = 2;
            } else if *((*ee).code).offset((*ee).head as isize) as i32 == '<' as i32
                && *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32 == '=' as i32
            {
                op = 3;
            }
            let ref mut fresh100 = (*ee).head;
            *fresh100 = (*fresh100 as u64).wrapping_add((if op > 2 { 2 } else { 1 }) as u64) as u64;
            match op {
                1 => match (*ee).type_0 {
                    1 => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival = (if odval < (*ee).dval { 1 } else { 0 }) as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival = (if odval < (*ee).ival as f64 { 1 } else { 0 }) as i64;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    (if (oival as f64) < (*ee).dval { 1 } else { 0 }) as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival = (if oival < (*ee).ival { 1 } else { 0 }) as i64;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                2 => match (*ee).type_0 {
                    1 => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival = (if odval > (*ee).dval { 1 } else { 0 }) as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival = (if odval > (*ee).ival as f64 { 1 } else { 0 }) as i64;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival = (if oival as f64 > (*ee).dval { 1 } else { 0 }) as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival = (if oival > (*ee).ival { 1 } else { 0 }) as i64;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                3 => match (*ee).type_0 {
                    1 => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival = (if odval <= (*ee).dval { 1 } else { 0 }) as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival =
                                    (if odval <= (*ee).ival as f64 { 1 } else { 0 }) as i64;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    (if oival as f64 <= (*ee).dval { 1 } else { 0 }) as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival = (if oival <= (*ee).ival { 1 } else { 0 }) as i64;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                4 => match (*ee).type_0 {
                    1 => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival = (if odval >= (*ee).dval { 1 } else { 0 }) as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival =
                                    (if odval >= (*ee).ival as f64 { 1 } else { 0 }) as i64;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        ee_shift(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    (if oival as f64 >= (*ee).dval { 1 } else { 0 }) as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival = (if oival >= (*ee).ival { 1 } else { 0 }) as i64;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                _ => {}
            }
            ee_skip_spaces(ee);
        }
    }
}

extern "C" fn ee_equals(mut ee: *mut expreval_t) {
    unsafe {
        ee_compare(ee);
        ee_skip_spaces(ee);
        while (*ee).head < (*ee).len
            && (*ee).error == 0
            && (*((*ee).code).offset((*ee).head as isize) as i32 == '=' as i32
                && *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32 == '=' as i32
                || *((*ee).code).offset((*ee).head as isize) as i32 == '!' as i32
                    && *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32
                        == '=' as i32)
        {
            let mut odval = (*ee).dval;
            let mut oival = (*ee).ival;
            let mut op = if *((*ee).code).offset((*ee).head as isize) as i32 == '=' as i32 {
                1
            } else {
                2
            };
            let ref mut fresh101 = (*ee).head;
            *fresh101 = (*fresh101 as u64).wrapping_add(2) as u64;
            match op {
                1 => match (*ee).type_0 {
                    1 => {
                        ee_compare(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival = (if odval == (*ee).dval { 1 } else { 0 }) as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival =
                                    (if odval == (*ee).ival as f64 { 1 } else { 0 }) as i64;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        ee_compare(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    (if oival as f64 == (*ee).dval { 1 } else { 0 }) as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival = (if oival == (*ee).ival { 1 } else { 0 }) as i64;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                2 => match (*ee).type_0 {
                    1 => {
                        ee_compare(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival = (if odval != (*ee).dval { 1 } else { 0 }) as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival =
                                    (if odval != (*ee).ival as f64 { 1 } else { 0 }) as i64;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    0 => {
                        ee_compare(ee);
                        if (*ee).error != 0 {
                            return;
                        }
                        match (*ee).type_0 {
                            1 => {
                                (*ee).ival =
                                    (if oival as f64 != (*ee).dval { 1 } else { 0 }) as i64;
                                (*ee).type_0 = 0;
                            }
                            0 => {
                                (*ee).ival = (if oival != (*ee).ival { 1 } else { 0 }) as i64;
                            }
                            _ => {
                                (*ee).error = 2;
                            }
                        }
                    }
                    _ => {
                        (*ee).error = 2;
                    }
                },
                _ => {}
            }
            ee_skip_spaces(ee);
        }
    }
}

extern "C" fn ee_bitand(mut ee: *mut expreval_t) {
    unsafe {
        ee_equals(ee);
        ee_skip_spaces(ee);
        while (*ee).head < (*ee).len
            && (*ee).error == 0
            && (*((*ee).code).offset((*ee).head as isize) as i32 == '&' as i32
                && *(*__ctype_b_loc()).offset(
                    *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32 as isize,
                ) as i32
                    & _ISpunct as i32
                    == 0)
        {
            let mut odval = (*ee).dval;
            let mut oival = (*ee).ival;
            let ref mut fresh102 = (*ee).head;
            *fresh102 = (*fresh102).wrapping_add(1);
            match (*ee).type_0 {
                1 => {
                    ee_equals(ee);
                    if (*ee).error != 0 {
                        return;
                    }
                    match (*ee).type_0 {
                        1 => {
                            (*ee).ival = odval as i64 & (*ee).dval as i64;
                            (*ee).type_0 = 0;
                        }
                        0 => {
                            (*ee).ival = odval as i64 & (*ee).ival;
                        }
                        _ => {
                            (*ee).error = 2;
                        }
                    }
                }
                0 => {
                    ee_equals(ee);
                    if (*ee).error != 0 {
                        return;
                    }
                    match (*ee).type_0 {
                        1 => {
                            (*ee).ival = oival & (*ee).dval as i64;
                            (*ee).type_0 = 0;
                        }
                        0 => {
                            (*ee).ival = oival & (*ee).ival;
                        }
                        _ => {
                            (*ee).error = 2;
                        }
                    }
                }
                _ => {
                    (*ee).error = 2;
                }
            }
            ee_skip_spaces(ee);
        }
    }
}

extern "C" fn ee_bitor(mut ee: *mut expreval_t) {
    unsafe {
        ee_bitand(ee);
        ee_skip_spaces(ee);
        while (*ee).head < (*ee).len
            && (*ee).error == 0
            && (*((*ee).code).offset((*ee).head as isize) as i32 == '|' as i32
                && *(*__ctype_b_loc()).offset(
                    *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32 as isize,
                ) as i32
                    & _ISpunct as i32
                    == 0)
        {
            let mut odval = (*ee).dval;
            let mut oival = (*ee).ival;
            let ref mut fresh103 = (*ee).head;
            *fresh103 = (*fresh103).wrapping_add(1);
            match (*ee).type_0 {
                1 => {
                    ee_bitand(ee);
                    if (*ee).error != 0 {
                        return;
                    }
                    match (*ee).type_0 {
                        1 => {
                            (*ee).ival = odval as i64 | (*ee).dval as i64;
                            (*ee).type_0 = 0;
                        }
                        0 => {
                            (*ee).ival = odval as i64 | (*ee).ival;
                        }
                        _ => {
                            (*ee).error = 2;
                        }
                    }
                }
                0 => {
                    ee_bitand(ee);
                    if (*ee).error != 0 {
                        return;
                    }
                    match (*ee).type_0 {
                        1 => {
                            (*ee).ival = oival | (*ee).dval as i64;
                            (*ee).type_0 = 0;
                        }
                        0 => {
                            (*ee).ival = oival | (*ee).ival;
                        }
                        _ => {
                            (*ee).error = 2;
                        }
                    }
                }
                _ => {
                    (*ee).error = 2;
                }
            }
            ee_skip_spaces(ee);
        }
    }
}

extern "C" fn ee_logand(mut ee: *mut expreval_t) {
    unsafe {
        ee_bitor(ee);
        ee_skip_spaces(ee);
        while (*ee).head < (*ee).len
            && (*ee).error == 0
            && (*((*ee).code).offset((*ee).head as isize) as i32 == '&' as i32
                && *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32 == '&' as i32)
        {
            let mut odval = (*ee).dval;
            let mut oival = (*ee).ival;
            let ref mut fresh104 = (*ee).head;
            *fresh104 = (*fresh104 as u64).wrapping_add(2) as u64;
            match (*ee).type_0 {
                1 => {
                    ee_bitor(ee);
                    if (*ee).error != 0 {
                        return;
                    }
                    match (*ee).type_0 {
                        1 => {
                            (*ee).ival = (if odval != 0. && (*ee).dval != 0. {
                                1
                            } else {
                                0
                            }) as i64;
                            (*ee).type_0 = 0;
                        }
                        0 => {
                            (*ee).ival =
                                (if odval != 0. && (*ee).ival != 0 { 1 } else { 0 }) as i64;
                        }
                        _ => {
                            (*ee).error = 2;
                        }
                    }
                }
                0 => {
                    ee_bitor(ee);
                    if (*ee).error != 0 {
                        return;
                    }
                    match (*ee).type_0 {
                        1 => {
                            (*ee).ival =
                                (if oival != 0 && (*ee).dval != 0. { 1 } else { 0 }) as i64;
                            (*ee).type_0 = 0;
                        }
                        0 => {
                            (*ee).ival = (if oival != 0 && (*ee).ival != 0 { 1 } else { 0 }) as i64;
                        }
                        _ => {
                            (*ee).error = 2;
                        }
                    }
                }
                _ => {
                    (*ee).error = 2;
                }
            }
            ee_skip_spaces(ee);
        }
    }
}

extern "C" fn ee_logor(mut ee: *mut expreval_t) {
    unsafe {
        ee_logand(ee);
        ee_skip_spaces(ee);
        while (*ee).head < (*ee).len
            && (*ee).error == 0
            && (*((*ee).code).offset((*ee).head as isize) as i32 == '|' as i32
                && *((*ee).code).offset(((*ee).head).wrapping_add(1) as isize) as i32 == '|' as i32)
        {
            let mut odval = (*ee).dval;
            let mut oival = (*ee).ival;
            let ref mut fresh105 = (*ee).head;
            *fresh105 = (*fresh105 as u64).wrapping_add(2) as u64;
            match (*ee).type_0 {
                1 => {
                    ee_logand(ee);
                    if (*ee).error != 0 {
                        return;
                    }
                    match (*ee).type_0 {
                        1 => {
                            (*ee).ival = (if odval != 0. || (*ee).dval != 0. {
                                1
                            } else {
                                0
                            }) as i64;
                            (*ee).type_0 = 0;
                        }
                        0 => {
                            (*ee).ival =
                                (if odval != 0. || (*ee).ival != 0 { 1 } else { 0 }) as i64;
                        }
                        _ => {
                            (*ee).error = 2;
                        }
                    }
                }
                0 => {
                    ee_logand(ee);
                    if (*ee).error != 0 {
                        return;
                    }
                    match (*ee).type_0 {
                        1 => {
                            (*ee).ival =
                                (if oival != 0 || (*ee).dval != 0. { 1 } else { 0 }) as i64;
                            (*ee).type_0 = 0;
                        }
                        0 => {
                            (*ee).ival = (if oival != 0 || (*ee).ival != 0 { 1 } else { 0 }) as i64;
                        }
                        _ => {
                            (*ee).error = 2;
                        }
                    }
                }
                _ => {
                    (*ee).error = 2;
                }
            }
            ee_skip_spaces(ee);
        }
    }
}

extern "C" fn ee_expr(mut ee: *mut expreval_t) {
    unsafe {
        ee_logor(ee);
        if (*ee).error == 4 {
            (*ee).error = 0;
            (*ee).ival = 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn lil_eval_expr(mut lil: lil_t, mut code: lil_value_t) -> lil_value_t {
    let mut ee = expreval_t {
        code: 0 as *const i8,
        len: 0,
        head: 0,
        ival: 0,
        dval: 0.,
        type_0: 0,
        error: 0,
    };
    code = lil_subst_to_value(lil, code);
    if (*lil).error != 0 {
        return 0 as lil_value_t;
    }
    ee.code = lil_to_string(code);
    unsafe {
        if *(ee.code).offset(0 as isize) == 0 {
            lil_free_value(code);
            return lil_alloc_integer(0);
        }
    }
    ee.head = 0;
    ee.len = (*code).l;
    ee.ival = 0;
    ee.dval = 0 as f64;
    ee.type_0 = 0;
    ee.error = 0;
    ee_expr(&mut ee);
    lil_free_value(code);
    if ee.error != 0 {
        match ee.error {
            3 => {
                lil_set_error(
                    lil,
                    b"division by zero in expression\0" as *const u8 as *const i8,
                );
            }
            2 => {
                lil_set_error(
                    lil,
                    b"mixing invalid types in expression\0" as *const u8 as *const i8,
                );
            }
            1 => {
                lil_set_error(lil, b"expression syntax error\0" as *const u8 as *const i8);
            }
            _ => {}
        }
        return 0 as lil_value_t;
    }
    if ee.type_0 == 0 {
        return lil_alloc_integer(ee.ival);
    } else {
        return lil_alloc_double(ee.dval);
    };
}

#[no_mangle]
pub extern "C" fn lil_unused_name(mut lil: lil_t, mut part: *const i8) -> lil_value_t {
    unsafe {
        let mut name = malloc((strlen(part)).wrapping_add(64)) as *mut i8;
        let mut val = 0 as *mut _lil_value_t;
        let mut i: u64 = 0;
        i = 0;
        while i < (-1) as i64 as u64 {
            sprintf(
                name,
                b"!!un!%s!%09u!nu!!\0" as *const u8 as *const i8,
                part,
                i as u32,
            );
            if (find_cmd(lil, name)).is_null() {
                if (lil_find_var(lil, (*lil).env, name)).is_null() {
                    val = lil_alloc_string(name);
                    free(name as *mut libc::c_void);
                    return val;
                }
            }
            i = i.wrapping_add(1);
        }
        return 0 as lil_value_t;
    }
}

#[no_mangle]
pub extern "C" fn lil_arg(mut argv: *mut lil_value_t, mut index: u64) -> lil_value_t {
    unsafe {
        return if !argv.is_null() {
            *argv.offset(index as isize)
        } else {
            0 as lil_value_t
        };
    }
}

#[no_mangle]
pub extern "C" fn lil_to_string(mut val: lil_value_t) -> *const i8 {
    return if !val.is_null() && !((*val).d).is_null() {
        (*val).d as *const i8
    } else {
        b"\0" as *const u8 as *const i8
    };
}

#[no_mangle]
pub extern "C" fn lil_to_double(mut val: lil_value_t) -> f64 {
    unsafe {
        return atof(lil_to_string(val));
    }
}

#[no_mangle]
pub extern "C" fn lil_to_integer(mut val: lil_value_t) -> i64 {
    unsafe {
        return atoll(lil_to_string(val)) as i64;
    }
}

#[no_mangle]
pub extern "C" fn lil_to_boolean(mut val: lil_value_t) -> i32 {
    let mut s = lil_to_string(val);
    let mut i: u64 = 0;
    let mut dots = 0;
    unsafe {
        if *s.offset(0 as isize) == 0 {
            return 0;
        }
    }
    i = 0;
    unsafe {
        while *s.offset(i as isize) != 0 {
            if *s.offset(i as isize) as i32 != '0' as i32
                && *s.offset(i as isize) as i32 != '.' as i32
            {
                return 1;
            }
            if *s.offset(i as isize) as i32 == '.' as i32 {
                if dots != 0 {
                    return 1;
                }
                dots = 1;
            }
            i = i.wrapping_add(1);
        }
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn lil_alloc_string(mut str: *const i8) -> lil_value_t {
    unsafe {
        return alloc_value(str);
    }
}

#[no_mangle]
pub extern "C" fn lil_alloc_double(mut num: f64) -> lil_value_t {
    let mut buff: [i8; 128] = [0; 128];
    unsafe {
        sprintf(buff.as_mut_ptr(), b"%f\0" as *const u8 as *const i8, num);
    }
    return alloc_value(buff.as_mut_ptr());
}

#[no_mangle]
pub extern "C" fn lil_alloc_integer(mut num: i64) -> lil_value_t {
    let mut buff: [i8; 128] = [0; 128];
    unsafe {
        sprintf(buff.as_mut_ptr(), b"%lli\0" as *const u8 as *const i8, num);
    }
    return alloc_value(buff.as_mut_ptr());
}

#[no_mangle]
pub extern "C" fn lil_free(mut lil: lil_t) {
    let mut i: u64 = 0;
    if lil.is_null() {
        return;
    }
    unsafe {
        free((*lil).err_msg as *mut libc::c_void);
    }
    lil_free_value((*lil).empty);
    while !((*lil).env).is_null() {
        let mut next = (*(*lil).env).parent;
        lil_free_env((*lil).env);
        let ref mut fresh106 = (*lil).env;
        *fresh106 = next;
    }
    i = 0;
    unsafe {
        while i < (*lil).cmds {
            if !((**((*lil).cmd).offset(i as isize)).argnames).is_null() {
                lil_free_list((**((*lil).cmd).offset(i as isize)).argnames);
            }
            lil_free_value((**((*lil).cmd).offset(i as isize)).code);
            free((**((*lil).cmd).offset(i as isize)).name as *mut libc::c_void);
            free(*((*lil).cmd).offset(i as isize) as *mut libc::c_void);
            i = i.wrapping_add(1);
        }
        free((*lil).cmd as *mut libc::c_void);
        free((*lil).dollarprefix as *mut libc::c_void);
        free((*lil).catcher as *mut libc::c_void);
        free(lil as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn lil_set_data(mut lil: lil_t, mut data: *mut libc::c_void) {
    unsafe {
        let ref mut fresh107 = (*lil).data;
        *fresh107 = data;
    }
}

#[no_mangle]
pub extern "C" fn lil_get_data(mut lil: lil_t) -> *mut libc::c_void {
    return (*lil).data;
}

extern "C" fn fnc_reflect(
    mut lil: lil_t,
    mut argc: u64,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    unsafe {
        let mut func = 0 as *mut _lil_func_t;
        let mut type_0 = 0 as *const i8;
        let mut i: u64 = 0;
        let mut r = 0 as *mut _lil_value_t;
        if argc == 0 {
            return 0 as lil_value_t;
        }
        type_0 = lil_to_string(*argv.offset(0 as isize));
        if strcmp(type_0, b"version\0" as *const u8 as *const i8) == 0 {
            return lil_alloc_string(b"0.1\0" as *const u8 as *const i8);
        }
        if strcmp(type_0, b"args\0" as *const u8 as *const i8) == 0 {
            if argc < 2 {
                return 0 as lil_value_t;
            }
            func = find_cmd(lil, lil_to_string(*argv.offset(1 as isize)));
            if func.is_null() || ((*func).argnames).is_null() {
                return 0 as lil_value_t;
            }
            return lil_list_to_value((*func).argnames, 1);
        }
        if strcmp(type_0, b"body\0" as *const u8 as *const i8) == 0 {
            if argc < 2 {
                return 0 as lil_value_t;
            }
            func = find_cmd(lil, lil_to_string(*argv.offset(1 as isize)));
            if func.is_null() || ((*func).proc_0).is_some() {
                return 0 as lil_value_t;
            }
            return lil_clone_value((*func).code);
        }
        if strcmp(type_0, b"func-count\0" as *const u8 as *const i8) == 0 {
            return lil_alloc_integer((*lil).cmds as i64);
        }
        if strcmp(type_0, b"funcs\0" as *const u8 as *const i8) == 0 {
            let mut funcs = lil_alloc_list();
            i = 0;
            while i < (*lil).cmds {
                lil_list_append(
                    funcs,
                    lil_alloc_string((**((*lil).cmd).offset(i as isize)).name),
                );
                i = i.wrapping_add(1);
            }
            r = lil_list_to_value(funcs, 1);
            lil_free_list(funcs);
            return r;
        }
        if strcmp(type_0, b"vars\0" as *const u8 as *const i8) == 0 {
            let mut vars = lil_alloc_list();
            let mut env = (*lil).env;
            while !env.is_null() {
                i = 0;
                while i < (*env).vars {
                    lil_list_append(
                        vars,
                        lil_alloc_string((**((*env).var).offset(i as isize)).n),
                    );
                    i = i.wrapping_add(1);
                }
                env = (*env).parent;
            }
            r = lil_list_to_value(vars, 1);
            lil_free_list(vars);
            return r;
        }
        if strcmp(type_0, b"globals\0" as *const u8 as *const i8) == 0 {
            let mut vars_0 = lil_alloc_list();
            i = 0;
            while i < (*(*lil).rootenv).vars {
                lil_list_append(
                    vars_0,
                    lil_alloc_string((**((*(*lil).rootenv).var).offset(i as isize)).n),
                );
                i = i.wrapping_add(1);
            }
            r = lil_list_to_value(vars_0, 1);
            lil_free_list(vars_0);
            return r;
        }
        if strcmp(type_0, b"has-func\0" as *const u8 as *const i8) == 0 {
            let mut target = 0 as *const i8;
            if argc == 1 {
                return 0 as lil_value_t;
            }
            target = lil_to_string(*argv.offset(1 as isize));
            i = 0;
            while i < (*lil).cmds {
                if strcmp(target, (**((*lil).cmd).offset(i as isize)).name) == 0 {
                    return lil_alloc_string(b"1\0" as *const u8 as *const i8);
                }
                i = i.wrapping_add(1);
            }
            return 0 as lil_value_t;
        }
        if strcmp(type_0, b"has-var\0" as *const u8 as *const i8) == 0 {
            let mut target_0 = 0 as *const i8;
            let mut env_0 = (*lil).env;
            if argc == 1 {
                return 0 as lil_value_t;
            }
            target_0 = lil_to_string(*argv.offset(1 as isize));
            while !env_0.is_null() {
                i = 0;
                while i < (*env_0).vars {
                    if strcmp(target_0, (**((*env_0).var).offset(i as isize)).n) == 0 {
                        return lil_alloc_string(b"1\0" as *const u8 as *const i8);
                    }
                    i = i.wrapping_add(1);
                }
                env_0 = (*env_0).parent;
            }
            return 0 as lil_value_t;
        }
        if strcmp(type_0, b"has-global\0" as *const u8 as *const i8) == 0 {
            let mut target_1 = 0 as *const i8;
            if argc == 1 {
                return 0 as lil_value_t;
            }
            target_1 = lil_to_string(*argv.offset(1 as isize));
            i = 0;
            while i < (*(*lil).rootenv).vars {
                if strcmp(target_1, (**((*(*lil).rootenv).var).offset(i as isize)).n) == 0 {
                    return lil_alloc_string(b"1\0" as *const u8 as *const i8);
                }
                i = i.wrapping_add(1);
            }
            return 0 as lil_value_t;
        }
        if strcmp(type_0, b"error\0" as *const u8 as *const i8) == 0 {
            return if !((*lil).err_msg).is_null() {
                lil_alloc_string((*lil).err_msg)
            } else {
                0 as lil_value_t
            };
        }
        if strcmp(type_0, b"dollar-prefix\0" as *const u8 as *const i8) == 0 {
            let mut r_0 = 0 as *mut _lil_value_t;
            if argc == 1 {
                return lil_alloc_string((*lil).dollarprefix);
            }
            r_0 = lil_alloc_string((*lil).dollarprefix);
            free((*lil).dollarprefix as *mut libc::c_void);
            let ref mut fresh108 = (*lil).dollarprefix;
            *fresh108 = strclone(lil_to_string(*argv.offset(1 as isize)));
            return r_0;
        }
        if strcmp(type_0, b"this\0" as *const u8 as *const i8) == 0 {
            let mut env_1 = (*lil).env;
            while env_1 != (*lil).rootenv
                && ((*env_1).catcher_for).is_null()
                && ((*env_1).func).is_null()
            {
                env_1 = (*env_1).parent;
            }
            if !((*env_1).catcher_for).is_null() {
                return lil_alloc_string((*lil).catcher);
            }
            if env_1 == (*lil).rootenv {
                return lil_alloc_string((*lil).rootcode);
            }
            return if !((*env_1).func).is_null() {
                (*(*env_1).func).code
            } else {
                0 as lil_value_t
            };
        }
        if strcmp(type_0, b"name\0" as *const u8 as *const i8) == 0 {
            let mut env_2 = (*lil).env;
            while env_2 != (*lil).rootenv
                && ((*env_2).catcher_for).is_null()
                && ((*env_2).func).is_null()
            {
                env_2 = (*env_2).parent;
            }
            if !((*env_2).catcher_for).is_null() {
                return (*env_2).catcher_for;
            }
            if env_2 == (*lil).rootenv {
                return 0 as lil_value_t;
            }
            return if !((*env_2).func).is_null() {
                lil_alloc_string((*(*env_2).func).name)
            } else {
                0 as lil_value_t
            };
        }
        return 0 as lil_value_t;
    }
}

extern "C" fn fnc_func(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut name = 0 as *mut _lil_value_t;
        let mut cmd = 0 as *mut _lil_func_t;
        if argc < 1 {
            return 0 as lil_value_t;
        }
        if argc == 3 {
            name = lil_clone_value(*argv.offset(0 as isize));
            cmd = add_func(lil, lil_to_string(*argv.offset(0 as isize)));
            let ref mut fresh109 = (*cmd).argnames;
            *fresh109 = lil_subst_to_list(lil, *argv.offset(1 as isize));
            let ref mut fresh110 = (*cmd).code;
            *fresh110 = lil_clone_value(*argv.offset(2 as isize));
        } else {
            name = lil_unused_name(lil, b"anonymous-function\0" as *const u8 as *const i8);
            cmd = add_func(lil, lil_to_string(name));
            if argc < 2 {
                let mut tmp = lil_alloc_string(b"args\0" as *const u8 as *const i8);
                let ref mut fresh111 = (*cmd).argnames;
                *fresh111 = lil_subst_to_list(lil, tmp);
                lil_free_value(tmp);
                let ref mut fresh112 = (*cmd).code;
                *fresh112 = lil_clone_value(*argv.offset(0 as isize));
            } else {
                let ref mut fresh113 = (*cmd).argnames;
                *fresh113 = lil_subst_to_list(lil, *argv.offset(0 as isize));
                let ref mut fresh114 = (*cmd).code;
                *fresh114 = lil_clone_value(*argv.offset(1 as isize));
            }
        }
        return name;
    }
}

extern "C" fn fnc_rename(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut r = 0 as *mut _lil_value_t;
        let mut func = 0 as *mut _lil_func_t;
        let mut oldname = 0 as *const i8;
        let mut newname = 0 as *const i8;
        if argc < 2 {
            return 0 as lil_value_t;
        }
        oldname = lil_to_string(*argv.offset(0 as isize));
        newname = lil_to_string(*argv.offset(1 as isize));
        func = find_cmd(lil, oldname);
        if func.is_null() {
            let mut msg = malloc(24u64.wrapping_add(strlen(oldname))) as *mut i8;
            sprintf(
                msg,
                b"unknown function '%s'\0" as *const u8 as *const i8,
                oldname,
            );
            lil_set_error_at(lil, (*lil).head, msg);
            free(msg as *mut libc::c_void);
            return 0 as lil_value_t;
        }
        r = lil_alloc_string((*func).name);
        free((*func).name as *mut libc::c_void);
        let ref mut fresh115 = (*func).name;
        *fresh115 = strclone(newname);
        return r;
    }
}

extern "C" fn fnc_unusedname(
    mut lil: lil_t,
    mut argc: u64,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    unsafe {
        return lil_unused_name(
            lil,
            if argc > 0 {
                lil_to_string(*argv.offset(0 as isize))
            } else {
                b"unusedname\0" as *const u8 as *const i8
            },
        );
    }
}

extern "C" fn fnc_quote(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut r = 0 as *mut _lil_value_t;
        let mut i: u64 = 0;
        if argc < 1 {
            return 0 as lil_value_t;
        }
        r = alloc_value(0 as *const i8);
        i = 0;
        while i < argc {
            if i != 0 {
                lil_append_char(r, ' ' as i8);
            }
            lil_append_val(r, *argv.offset(i as isize));
            i = i.wrapping_add(1);
        }
        return r;
    }
}

extern "C" fn fnc_set(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut i = 0;
        let mut var = 0 as lil_var_t;
        let mut access = 1;
        if argc == 0 {
            return 0 as lil_value_t;
        }
        if strcmp(
            lil_to_string(*argv.offset(0 as isize)),
            b"global\0" as *const u8 as *const i8,
        ) == 0
        {
            i = 1;
            access = 0;
        }
        while i < argc {
            if argc == i.wrapping_add(1) {
                return lil_clone_value(lil_get_var(lil, lil_to_string(*argv.offset(i as isize))));
            }
            var = lil_set_var(
                lil,
                lil_to_string(*argv.offset(i as isize)),
                *argv.offset(i.wrapping_add(1) as isize),
                access,
            );
            i = (i as u64).wrapping_add(2) as u64;
        }
        return if !var.is_null() {
            lil_clone_value((*var).v)
        } else {
            0 as lil_value_t
        };
    }
}

extern "C" fn fnc_local(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut i: u64 = 0;
        i = 0;
        while i < argc {
            let mut varname = lil_to_string(*argv.offset(i as isize));
            if (lil_find_local_var(lil, (*lil).env, varname)).is_null() {
                lil_set_var(lil, varname, (*lil).empty, 2);
            }
            i = i.wrapping_add(1);
        }
        return 0 as lil_value_t;
    }
}

extern "C" fn fnc_write(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut i: u64 = 0;
        let mut msg = lil_alloc_string(0 as *const i8);
        i = 0;
        while i < argc {
            if i != 0 {
                lil_append_char(msg, ' ' as i8);
            }
            lil_append_val(msg, *argv.offset(i as isize));
            i = i.wrapping_add(1);
        }
        if ((*lil).callback[1 as usize]).is_some() {
            let mut proc_0: lil_write_callback_proc_t = ::std::mem::transmute::<
                lil_callback_proc_t,
                lil_write_callback_proc_t,
            >((*lil).callback[1 as usize]);
            proc_0.expect("non-null function pointer")(lil, lil_to_string(msg));
        } else {
            print!("{},9998");
        }
        lil_free_value(msg);
        return 0 as lil_value_t;
    }
}

extern "C" fn fnc_print(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        fnc_write(lil, argc, argv);
        if ((*lil).callback[1 as usize]).is_some() {
            let mut proc_0: lil_write_callback_proc_t = ::std::mem::transmute::<
                lil_callback_proc_t,
                lil_write_callback_proc_t,
            >((*lil).callback[1 as usize]);
            proc_0.expect("non-null function pointer")(lil, b"\n\0" as *const u8 as *const i8);
        } else {
            print!("\n");
        }
        return 0 as lil_value_t;
    }
}

extern "C" fn fnc_eval(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        if argc == 1 {
            return lil_parse_value(lil, *argv.offset(0 as isize), 0);
        }
        if argc > 1 {
            let mut val = alloc_value(0 as *const i8);
            let mut r = 0 as *mut _lil_value_t;
            let mut i: u64 = 0;
            i = 0;
            while i < argc {
                if i != 0 {
                    lil_append_char(val, ' ' as i8);
                }
                lil_append_val(val, *argv.offset(i as isize));
                i = i.wrapping_add(1);
            }
            r = lil_parse_value(lil, val, 0);
            lil_free_value(val);
            return r;
        }
        return 0 as lil_value_t;
    }
}

extern "C" fn fnc_topeval(
    mut lil: lil_t,
    mut argc: u64,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    unsafe {
        let mut thisenv = (*lil).env;
        let mut thisdownenv = (*lil).downenv;
        let mut r = 0 as *mut _lil_value_t;
        let ref mut fresh116 = (*lil).env;
        *fresh116 = (*lil).rootenv;
        let ref mut fresh117 = (*lil).downenv;
        *fresh117 = thisenv;
        r = fnc_eval(lil, argc, argv);
        let ref mut fresh118 = (*lil).downenv;
        *fresh118 = thisdownenv;
        let ref mut fresh119 = (*lil).env;
        *fresh119 = thisenv;
        return r;
    }
}

extern "C" fn fnc_upeval(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut thisenv = (*lil).env;
        let mut thisdownenv = (*lil).downenv;
        let mut r = 0 as *mut _lil_value_t;
        if (*lil).rootenv == thisenv {
            return fnc_eval(lil, argc, argv);
        }
        let ref mut fresh120 = (*lil).env;
        *fresh120 = (*thisenv).parent;
        let ref mut fresh121 = (*lil).downenv;
        *fresh121 = thisenv;
        r = fnc_eval(lil, argc, argv);
        let ref mut fresh122 = (*lil).env;
        *fresh122 = thisenv;
        let ref mut fresh123 = (*lil).downenv;
        *fresh123 = thisdownenv;
        return r;
    }
}

extern "C" fn fnc_downeval(
    mut lil: lil_t,
    mut argc: u64,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    unsafe {
        let mut r = 0 as *mut _lil_value_t;
        let mut upenv = (*lil).env;
        let mut downenv = (*lil).downenv;
        if downenv.is_null() {
            return fnc_eval(lil, argc, argv);
        }
        let ref mut fresh124 = (*lil).downenv;
        *fresh124 = 0 as lil_env_t;
        let ref mut fresh125 = (*lil).env;
        *fresh125 = downenv;
        r = fnc_eval(lil, argc, argv);
        let ref mut fresh126 = (*lil).downenv;
        *fresh126 = downenv;
        let ref mut fresh127 = (*lil).env;
        *fresh127 = upenv;
        return r;
    }
}

extern "C" fn fnc_enveval(
    mut lil: lil_t,
    mut argc: u64,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    unsafe {
        let mut r = 0 as *mut _lil_value_t;
        let mut invars = 0 as lil_list_t;
        let mut outvars = 0 as lil_list_t;
        let mut varvalues = 0 as *mut lil_value_t;
        let mut codeindex: i32 = 0;
        let mut i: u64 = 0;
        if argc < 1 {
            return 0 as lil_value_t;
        }
        if argc == 1 {
            codeindex = 0;
        } else if argc >= 2 {
            invars = lil_subst_to_list(lil, *argv.offset(0 as isize));
            varvalues = malloc(
                (::std::mem::size_of::<lil_value_t>() as u64).wrapping_mul(lil_list_size(invars)),
            ) as *mut lil_value_t;
            i = 0;
            while i < lil_list_size(invars) {
                let ref mut fresh128 = *varvalues.offset(i as isize);
                *fresh128 =
                    lil_clone_value(lil_get_var(lil, lil_to_string(lil_list_get(invars, i))));
                i = i.wrapping_add(1);
            }
            if argc > 2 {
                codeindex = 2;
                outvars = lil_subst_to_list(lil, *argv.offset(1 as isize));
            } else {
                codeindex = 1;
            }
        }
        lil_push_env(lil);
        if !invars.is_null() {
            i = 0;
            while i < lil_list_size(invars) {
                lil_set_var(
                    lil,
                    lil_to_string(lil_list_get(invars, i)),
                    *varvalues.offset(i as isize),
                    2,
                );
                lil_free_value(*varvalues.offset(i as isize));
                i = i.wrapping_add(1);
            }
        }
        r = lil_parse_value(lil, *argv.offset(codeindex as isize), 0);
        if !invars.is_null() || !outvars.is_null() {
            if !outvars.is_null() {
                varvalues = realloc(
                    varvalues as *mut libc::c_void,
                    (::std::mem::size_of::<lil_value_t>() as u64)
                        .wrapping_mul(lil_list_size(outvars)),
                ) as *mut lil_value_t;
                i = 0;
                while i < lil_list_size(outvars) {
                    let ref mut fresh129 = *varvalues.offset(i as isize);
                    *fresh129 =
                        lil_clone_value(lil_get_var(lil, lil_to_string(lil_list_get(outvars, i))));
                    i = i.wrapping_add(1);
                }
            } else {
                i = 0;
                while i < lil_list_size(invars) {
                    let ref mut fresh130 = *varvalues.offset(i as isize);
                    *fresh130 =
                        lil_clone_value(lil_get_var(lil, lil_to_string(lil_list_get(invars, i))));
                    i = i.wrapping_add(1);
                }
            }
        }
        lil_pop_env(lil);
        if !invars.is_null() {
            if !outvars.is_null() {
                i = 0;
                while i < lil_list_size(outvars) {
                    lil_set_var(
                        lil,
                        lil_to_string(lil_list_get(outvars, i)),
                        *varvalues.offset(i as isize),
                        1,
                    );
                    lil_free_value(*varvalues.offset(i as isize));
                    i = i.wrapping_add(1);
                }
            } else {
                i = 0;
                while i < lil_list_size(invars) {
                    lil_set_var(
                        lil,
                        lil_to_string(lil_list_get(invars, i)),
                        *varvalues.offset(i as isize),
                        1,
                    );
                    lil_free_value(*varvalues.offset(i as isize));
                    i = i.wrapping_add(1);
                }
            }
            lil_free_list(invars);
            if !outvars.is_null() {
                lil_free_list(outvars);
            }
            free(varvalues as *mut libc::c_void);
        }
        return r;
    }
}

extern "C" fn fnc_jaileval(
    mut lil: lil_t,
    mut argc: u64,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    unsafe {
        let mut i: u64 = 0;
        let mut sublil = 0 as *mut _lil_t;
        let mut r = 0 as *mut _lil_value_t;
        let mut base = 0;
        if argc == 0 {
            return 0 as lil_value_t;
        }
        if strcmp(
            lil_to_string(*argv.offset(0 as isize)),
            b"clean\0" as *const u8 as *const i8,
        ) == 0
        {
            base = 1;
            if argc == 1 {
                return 0 as lil_value_t;
            }
        }
        sublil = lil_new();
        if base != 1 {
            i = (*lil).syscmds;
            while i < (*lil).cmds {
                let mut fnc = *((*lil).cmd).offset(i as isize);
                if !((*fnc).proc_0).is_none() {
                    lil_register(sublil, (*fnc).name, (*fnc).proc_0);
                }
                i = i.wrapping_add(1);
            }
        }
        r = lil_parse_value(sublil, *argv.offset(base as isize), 1);
        lil_free(sublil);
        return r;
    }
}

extern "C" fn fnc_count(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut list = 0 as *mut _lil_list_t;
        let mut buff: [i8; 64] = [0; 64];
        if argc == 0 {
            return alloc_value(b"0\0" as *const u8 as *const i8);
        }
        list = lil_subst_to_list(lil, *argv.offset(0 as isize));
        sprintf(
            buff.as_mut_ptr(),
            b"%u\0" as *const u8 as *const i8,
            (*list).c as u32,
        );
        lil_free_list(list);
        return alloc_value(buff.as_mut_ptr());
    }
}

extern "C" fn fnc_index(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut list = 0 as *mut _lil_list_t;
        let mut index: u64 = 0;
        let mut r = 0 as *mut _lil_value_t;
        if argc < 2 {
            return 0 as lil_value_t;
        }
        list = lil_subst_to_list(lil, *argv.offset(0 as isize));
        index = lil_to_integer(*argv.offset(1 as isize)) as u64;
        if index >= (*list).c {
            r = 0 as lil_value_t;
        } else {
            r = lil_clone_value(*((*list).v).offset(index as isize));
        }
        lil_free_list(list);
        return r;
    }
}

extern "C" fn fnc_indexof(
    mut lil: lil_t,
    mut argc: u64,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    unsafe {
        let mut list = 0 as *mut _lil_list_t;
        let mut index: u64 = 0;
        let mut r = 0 as lil_value_t;
        if argc < 2 {
            return 0 as lil_value_t;
        }
        list = lil_subst_to_list(lil, *argv.offset(0 as isize));
        index = 0;
        while index < (*list).c {
            if strcmp(
                lil_to_string(*((*list).v).offset(index as isize)),
                lil_to_string(*argv.offset(1 as isize)),
            ) == 0
            {
                r = lil_alloc_integer(index as i64);
                break;
            } else {
                index = index.wrapping_add(1);
            }
        }
        lil_free_list(list);
        return r;
    }
}

extern "C" fn fnc_append(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut list = 0 as *mut _lil_list_t;
        let mut r = 0 as *mut _lil_value_t;
        let mut i: u64 = 0;
        let mut base = 1;
        let mut access = 1;
        let mut varname = 0 as *const i8;
        if argc < 2 {
            return 0 as lil_value_t;
        }
        varname = lil_to_string(*argv.offset(0 as isize));
        if strcmp(varname, b"global\0" as *const u8 as *const i8) == 0 {
            if argc < 3 {
                return 0 as lil_value_t;
            }
            varname = lil_to_string(*argv.offset(1 as isize));
            base = 2;
            access = 0;
        }
        list = lil_subst_to_list(lil, lil_get_var(lil, varname));
        i = base;
        while i < argc {
            lil_list_append(list, lil_clone_value(*argv.offset(i as isize)));
            i = i.wrapping_add(1);
        }
        r = lil_list_to_value(list, 1);
        lil_free_list(list);
        lil_set_var(lil, varname, r, access);
        return r;
    }
}

extern "C" fn fnc_slice(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut list = 0 as *mut _lil_list_t;
        let mut slice = 0 as *mut _lil_list_t;
        let mut i: u64 = 0;
        let mut from: i64 = 0;
        let mut to: i64 = 0;
        let mut r = 0 as *mut _lil_value_t;
        if argc < 1 {
            return 0 as lil_value_t;
        }
        if argc < 2 {
            return lil_clone_value(*argv.offset(0 as isize));
        }
        from = lil_to_integer(*argv.offset(1 as isize));
        if from < 0 {
            from = 0;
        }
        list = lil_subst_to_list(lil, *argv.offset(0 as isize));
        to = if argc > 2 {
            lil_to_integer(*argv.offset(2 as isize))
        } else {
            (*list).c as i64
        };
        if to > (*list).c as i64 {
            to = (*list).c as i64;
        }
        if to < from {
            to = from;
        }
        slice = lil_alloc_list();
        i = from as u64;
        while i < to as u64 {
            lil_list_append(slice, lil_clone_value(*((*list).v).offset(i as isize)));
            i = i.wrapping_add(1);
        }
        lil_free_list(list);
        r = lil_list_to_value(slice, 1);
        lil_free_list(slice);
        return r;
    }
}

extern "C" fn fnc_filter(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut list = 0 as *mut _lil_list_t;
        let mut filtered = 0 as *mut _lil_list_t;
        let mut i: u64 = 0;
        let mut r = 0 as *mut _lil_value_t;
        let mut varname = b"x\0" as *const u8 as *const i8;
        let mut base = 0;
        if argc < 1 {
            return 0 as lil_value_t;
        }
        if argc < 2 {
            return lil_clone_value(*argv.offset(0 as isize));
        }
        if argc > 2 {
            base = 1;
            varname = lil_to_string(*argv.offset(0 as isize));
        }
        list = lil_subst_to_list(lil, *argv.offset(base as isize));
        filtered = lil_alloc_list();
        i = 0;
        while i < (*list).c && (*(*lil).env).breakrun == 0 {
            lil_set_var(lil, varname, *((*list).v).offset(i as isize), 3);
            r = lil_eval_expr(lil, *argv.offset((base + 1i32) as isize));
            if lil_to_boolean(r) != 0 {
                lil_list_append(filtered, lil_clone_value(*((*list).v).offset(i as isize)));
            }
            lil_free_value(r);
            i = i.wrapping_add(1);
        }
        lil_free_list(list);
        r = lil_list_to_value(filtered, 1);
        lil_free_list(filtered);
        return r;
    }
}

extern "C" fn fnc_list(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut list = lil_alloc_list();
        let mut r = 0 as *mut _lil_value_t;
        let mut i: u64 = 0;
        i = 0;
        while i < argc {
            lil_list_append(list, lil_clone_value(*argv.offset(i as isize)));
            i = i.wrapping_add(1);
        }
        r = lil_list_to_value(list, 1);
        lil_free_list(list);
        return r;
    }
}

extern "C" fn fnc_subst(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        if argc < 1 {
            return 0 as lil_value_t;
        }
        return lil_subst_to_value(lil, *argv.offset(0 as isize));
    }
}

extern "C" fn fnc_concat(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut list = 0 as *mut _lil_list_t;
        let mut r = 0 as *mut _lil_value_t;
        let mut tmp = 0 as *mut _lil_value_t;
        let mut i: u64 = 0;
        if argc < 1 {
            return 0 as lil_value_t;
        }
        r = lil_alloc_string(b"\0" as *const u8 as *const i8);
        i = 0;
        while i < argc {
            list = lil_subst_to_list(lil, *argv.offset(i as isize));
            tmp = lil_list_to_value(list, 1);
            lil_free_list(list);
            lil_append_val(r, tmp);
            lil_free_value(tmp);
            i = i.wrapping_add(1);
        }
        return r;
    }
}

extern "C" fn fnc_foreach(
    mut lil: lil_t,
    mut argc: u64,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    unsafe {
        let mut list = 0 as *mut _lil_list_t;
        let mut rlist = 0 as *mut _lil_list_t;
        let mut r = 0 as *mut _lil_value_t;
        let mut i: u64 = 0;
        let mut listidx = 0;
        let mut codeidx = 1;
        let mut varname = b"i\0" as *const u8 as *const i8;
        if argc < 2 {
            return 0 as lil_value_t;
        }
        if argc >= 3 {
            varname = lil_to_string(*argv.offset(0 as isize));
            listidx = 1;
            codeidx = 2;
        }
        rlist = lil_alloc_list();
        list = lil_subst_to_list(lil, *argv.offset(listidx as isize));
        i = 0;
        while i < (*list).c {
            let mut rv = 0 as *mut _lil_value_t;
            lil_set_var(lil, varname, *((*list).v).offset(i as isize), 3);
            rv = lil_parse_value(lil, *argv.offset(codeidx as isize), 0);
            if (*rv).l != 0 {
                lil_list_append(rlist, rv);
            } else {
                lil_free_value(rv);
            }
            if (*(*lil).env).breakrun != 0 || (*lil).error != 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        r = lil_list_to_value(rlist, 1);
        lil_free_list(list);
        lil_free_list(rlist);
        return r;
    }
}

extern "C" fn fnc_return(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        (*(*lil).env).breakrun = 1;
        lil_free_value((*(*lil).env).retval);
        let ref mut fresh131 = (*(*lil).env).retval;
        *fresh131 = if argc < 1 {
            0 as lil_value_t
        } else {
            lil_clone_value(*argv.offset(0 as isize))
        };
        (*(*lil).env).retval_set = 1;
        return if argc < 1 {
            0 as lil_value_t
        } else {
            lil_clone_value(*argv.offset(0 as isize))
        };
    }
}

extern "C" fn fnc_result(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        if argc > 0 {
            lil_free_value((*(*lil).env).retval);
            let ref mut fresh132 = (*(*lil).env).retval;
            *fresh132 = lil_clone_value(*argv.offset(0 as isize));
            (*(*lil).env).retval_set = 1;
        }
        return if (*(*lil).env).retval_set != 0 {
            lil_clone_value((*(*lil).env).retval)
        } else {
            0 as lil_value_t
        };
    }
}

extern "C" fn fnc_expr(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        if argc == 1 {
            return lil_eval_expr(lil, *argv.offset(0 as isize));
        }
        if argc > 1 {
            let mut val = alloc_value(0 as *const i8);
            let mut r = 0 as *mut _lil_value_t;
            let mut i: u64 = 0;
            i = 0;
            while i < argc {
                if i != 0 {
                    lil_append_char(val, ' ' as i8);
                }
                lil_append_val(val, *argv.offset(i as isize));
                i = i.wrapping_add(1);
            }
            r = lil_eval_expr(lil, val);
            lil_free_value(val);
            return r;
        }
        return 0 as lil_value_t;
    }
}

extern "C" fn real_inc(
    mut lil: lil_t,
    mut varname: *const i8,
    mut v: libc::c_float,
) -> lil_value_t {
    unsafe {
        let mut pv = lil_get_var(lil, varname);
        let mut dv = lil_to_double(pv) + v as f64;
        if fmod(dv, 1 as f64) != 0. {
            pv = lil_alloc_double(dv);
        } else {
            pv = lil_alloc_integer((lil_to_integer(pv) as libc::c_float + v) as i64);
        }
        lil_set_var(lil, varname, pv, 1);
        return pv;
    }
}

extern "C" fn fnc_inc(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        if argc < 1 {
            return 0 as lil_value_t;
        }
        return real_inc(
            lil,
            lil_to_string(*argv.offset(0 as isize)),
            (if argc > 1u64 {
                lil_to_double(*argv.offset(1 as isize))
            } else {
                1 as f64
            }) as libc::c_float,
        );
    }
}

extern "C" fn fnc_dec(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        if argc < 1 {
            return 0 as lil_value_t;
        }
        return real_inc(
            lil,
            lil_to_string(*argv.offset(0 as isize)),
            -if argc > 1 {
                lil_to_double(*argv.offset(1 as isize))
            } else {
                1 as f64
            } as libc::c_float,
        );
    }
}

extern "C" fn fnc_read(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut f = 0 as *mut FILE;
        let mut size: u64 = 0;
        let mut buffer = 0 as *mut i8;
        let mut r = 0 as *mut _lil_value_t;
        if argc < 1 {
            return 0 as lil_value_t;
        }
        if ((*lil).callback[2 as usize]).is_some() {
            let mut proc_0: lil_read_callback_proc_t = ::std::mem::transmute::<
                lil_callback_proc_t,
                lil_read_callback_proc_t,
            >((*lil).callback[2 as usize]);
            buffer = proc_0.expect("non-null function pointer")(
                lil,
                lil_to_string(*argv.offset(0 as isize)),
            );
        } else {
            f = fopen(
                lil_to_string(*argv.offset(0 as isize)),
                b"rb\0" as *const u8 as *const i8,
            );
            if f.is_null() {
                return 0 as lil_value_t;
            }
            fseek(f, 0, 2);
            size = ftell(f) as u64;
            fseek(f, 0, 0);
            buffer = malloc(size.wrapping_add(1)) as *mut i8;
            fread(buffer as *mut libc::c_void, 1, size, f);
            *buffer.offset(size as isize) = 0;
            fclose(f);
        }
        r = lil_alloc_string(buffer);
        free(buffer as *mut libc::c_void);
        return r;
    }
}

extern "C" fn fnc_store(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut f = 0 as *mut FILE;
        let mut buffer = 0 as *const i8;
        if argc < 2 {
            return 0 as lil_value_t;
        }
        if ((*lil).callback[3 as usize]).is_some() {
            let mut proc_0: lil_store_callback_proc_t = ::std::mem::transmute::<
                lil_callback_proc_t,
                lil_store_callback_proc_t,
            >((*lil).callback[3 as usize]);
            proc_0.expect("non-null function pointer")(
                lil,
                lil_to_string(*argv.offset(0 as isize)),
                lil_to_string(*argv.offset(1 as isize)),
            );
        } else {
            f = fopen(
                lil_to_string(*argv.offset(0 as isize)),
                b"wb\0" as *const u8 as *const i8,
            );
            if f.is_null() {
                return 0 as lil_value_t;
            }
            buffer = lil_to_string(*argv.offset(1 as isize));
            fwrite(buffer as *const libc::c_void, 1, strlen(buffer), f);
            fclose(f);
        }
        return lil_clone_value(*argv.offset(1 as isize));
    }
}

extern "C" fn fnc_if(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut val = 0 as *mut _lil_value_t;
        let mut r = 0 as lil_value_t;
        let mut base = 0;
        let mut not = 0;
        let mut v: i32 = 0;
        if argc < 1 {
            return 0 as lil_value_t;
        }
        if strcmp(
            lil_to_string(*argv.offset(0 as isize)),
            b"not\0" as *const u8 as *const i8,
        ) == 0
        {
            not = 1;
            base = not;
        }
        if argc < (base as u64).wrapping_add(2) {
            return 0 as lil_value_t;
        }
        val = lil_eval_expr(lil, *argv.offset(base as isize));
        if val.is_null() || (*lil).error != 0 {
            return 0 as lil_value_t;
        }
        v = lil_to_boolean(val);
        if not != 0 {
            v = (v == 0) as i32;
        }
        if v != 0 {
            r = lil_parse_value(lil, *argv.offset((base + 1i32) as isize), 0);
        } else if argc > (base as u64).wrapping_add(2) {
            r = lil_parse_value(lil, *argv.offset((base + 2i32) as isize), 0);
        }
        lil_free_value(val);
        return r;
    }
}

extern "C" fn fnc_while(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut val = 0 as *mut _lil_value_t;
        let mut r = 0 as lil_value_t;
        let mut base = 0;
        let mut not = 0;
        let mut v: i32 = 0;
        if argc < 1 {
            return 0 as lil_value_t;
        }
        if strcmp(
            lil_to_string(*argv.offset(0 as isize)),
            b"not\0" as *const u8 as *const i8,
        ) == 0
        {
            not = 1;
            base = not;
        }
        if argc < (base as u64).wrapping_add(2) {
            return 0 as lil_value_t;
        }
        while (*lil).error == 0 && (*(*lil).env).breakrun == 0 {
            val = lil_eval_expr(lil, *argv.offset(base as isize));
            if val.is_null() || (*lil).error != 0 {
                return 0 as lil_value_t;
            }
            v = lil_to_boolean(val);
            if not != 0 {
                v = (v == 0) as i32;
            }
            if v == 0 {
                lil_free_value(val);
                break;
            } else {
                if !r.is_null() {
                    lil_free_value(r);
                }
                r = lil_parse_value(lil, *argv.offset((base + 1i32) as isize), 0);
                lil_free_value(val);
            }
        }
        return r;
    }
}

extern "C" fn fnc_for(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut val = 0 as *mut _lil_value_t;
        let mut r = 0 as lil_value_t;
        if argc < 4 {
            return 0 as lil_value_t;
        }
        lil_free_value(lil_parse_value(lil, *argv.offset(0 as isize), 0));
        while (*lil).error == 0 && (*(*lil).env).breakrun == 0 {
            val = lil_eval_expr(lil, *argv.offset(1 as isize));
            if val.is_null() || (*lil).error != 0 {
                return 0 as lil_value_t;
            }
            if lil_to_boolean(val) == 0 {
                lil_free_value(val);
                break;
            } else {
                if !r.is_null() {
                    lil_free_value(r);
                }
                r = lil_parse_value(lil, *argv.offset(3 as isize), 0);
                lil_free_value(val);
                lil_free_value(lil_parse_value(lil, *argv.offset(2 as isize), 0));
            }
        }
        return r;
    }
}

extern "C" fn fnc_char(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut s: [i8; 2] = [0; 2];
        if argc == 0 {
            return 0 as lil_value_t;
        }
        s[0 as usize] = lil_to_integer(*argv.offset(0 as isize)) as i8;
        s[1 as usize] = 0;
        return lil_alloc_string(s.as_mut_ptr());
    }
}

extern "C" fn fnc_charat(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut index: u64 = 0;
        let mut chstr: [i8; 2] = [0; 2];
        let mut str = 0 as *const i8;
        if argc < 2 {
            return 0 as lil_value_t;
        }
        str = lil_to_string(*argv.offset(0 as isize));
        index = lil_to_integer(*argv.offset(1 as isize)) as u64;
        if index >= strlen(str) {
            return 0 as lil_value_t;
        }
        chstr[0 as usize] = *str.offset(index as isize);
        chstr[1 as usize] = 0;
        return lil_alloc_string(chstr.as_mut_ptr());
    }
}

extern "C" fn fnc_codeat(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut index: u64 = 0;
        let mut str = 0 as *const i8;
        if argc < 2 {
            return 0 as lil_value_t;
        }
        str = lil_to_string(*argv.offset(0 as isize));
        index = lil_to_integer(*argv.offset(1 as isize)) as u64;
        if index >= strlen(str) {
            return 0 as lil_value_t;
        }
        return lil_alloc_integer(*str.offset(index as isize) as i64);
    }
}

extern "C" fn fnc_substr(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut str = 0 as *const i8;
        let mut r = 0 as *mut _lil_value_t;
        let mut start: u64 = 0;
        let mut end: u64 = 0;
        let mut i: u64 = 0;
        let mut slen: u64 = 0;
        if argc < 2 {
            return 0 as lil_value_t;
        }
        str = lil_to_string(*argv.offset(0 as isize));
        if *str.offset(0 as isize) == 0 {
            return 0 as lil_value_t;
        }
        slen = strlen(str);
        start = atoll(lil_to_string(*argv.offset(1 as isize))) as u64;
        end = if argc > 2 {
            atoll(lil_to_string(*argv.offset(2 as isize))) as u64
        } else {
            slen
        };
        if end > slen {
            end = slen;
        }
        if start >= end {
            return 0 as lil_value_t;
        }
        r = lil_alloc_string(b"\0" as *const u8 as *const i8);
        i = start;
        while i < end {
            lil_append_char(r, *str.offset(i as isize));
            i = i.wrapping_add(1);
        }
        return r;
    }
}

extern "C" fn fnc_strpos(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut hay = 0 as *const i8;
        let mut str = 0 as *const i8;
        let mut min = 0;
        if argc < 2 {
            return lil_alloc_integer(-1 as i64);
        }
        hay = lil_to_string(*argv.offset(0 as isize));
        if argc > 2 {
            min = atoll(lil_to_string(*argv.offset(2 as isize))) as u64;
            if min >= strlen(hay) {
                return lil_alloc_integer(-1 as i64);
            }
        }
        str = strstr(
            hay.offset(min as isize),
            lil_to_string(*argv.offset(1 as isize)),
        );
        if str.is_null() {
            return lil_alloc_integer(-1 as i64);
        }
        return lil_alloc_integer(str.offset_from(hay) as i64);
    }
}

extern "C" fn fnc_length(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut i: u64 = 0;
        let mut total = 0;
        i = 0;
        while i < argc {
            if i != 0 {
                total = total.wrapping_add(1);
            }
            total =
                (total as u64).wrapping_add(strlen(lil_to_string(*argv.offset(i as isize)))) as u64;
            i = i.wrapping_add(1);
        }
        return lil_alloc_integer(total as i64);
    }
}

extern "C" fn real_trim(
    mut str: *const i8,
    mut chars: *const i8,
    mut left: i32,
    mut right: i32,
) -> lil_value_t {
    unsafe {
        let mut base = 0;
        let mut r = 0 as lil_value_t;
        if left != 0 {
            while *str.offset(base as isize) as i32 != 0
                && !(strchr(chars, *str.offset(base as isize) as i32)).is_null()
            {
                base += 1;
            }
            if right == 0 {
                r = lil_alloc_string(if *str.offset(base as isize) as i32 != 0 {
                    str.offset(base as isize)
                } else {
                    0 as *const i8
                });
            }
        }
        if right != 0 {
            let mut len: u64 = 0;
            let mut s = 0 as *mut i8;
            s = strclone(str.offset(base as isize));
            len = strlen(s);
            while len != 0
                && !(strchr(chars, *s.offset(len.wrapping_sub(1) as isize) as i32)).is_null()
            {
                len = len.wrapping_sub(1);
            }
            *s.offset(len as isize) = 0;
            r = lil_alloc_string(s);
            free(s as *mut libc::c_void);
        }
        return r;
    }
}

extern "C" fn fnc_trim(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        if argc == 0 {
            return 0 as lil_value_t;
        }
        return real_trim(
            lil_to_string(*argv.offset(0 as isize)),
            if argc < 2 {
                b" \x0C\n\r\t\x0B\0" as *const u8 as *const i8
            } else {
                lil_to_string(*argv.offset(1 as isize))
            },
            1,
            1,
        );
    }
}

extern "C" fn fnc_ltrim(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        if argc == 0 {
            return 0 as lil_value_t;
        }
        return real_trim(
            lil_to_string(*argv.offset(0 as isize)),
            if argc < 2 {
                b" \x0C\n\r\t\x0B\0" as *const u8 as *const i8
            } else {
                lil_to_string(*argv.offset(1 as isize))
            },
            1,
            0,
        );
    }
}

extern "C" fn fnc_rtrim(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        if argc == 0 {
            return 0 as lil_value_t;
        }
        return real_trim(
            lil_to_string(*argv.offset(0 as isize)),
            if argc < 2 {
                b" \x0C\n\r\t\x0B\0" as *const u8 as *const i8
            } else {
                lil_to_string(*argv.offset(1 as isize))
            },
            0,
            1,
        );
    }
}

extern "C" fn fnc_strcmp(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        if argc < 2 {
            return 0 as lil_value_t;
        }
        return lil_alloc_integer(strcmp(
            lil_to_string(*argv.offset(0 as isize)),
            lil_to_string(*argv.offset(1 as isize)),
        ) as i64);
    }
}

extern "C" fn fnc_streq(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        if argc < 2 {
            return 0 as lil_value_t;
        }
        return lil_alloc_integer(
            (if strcmp(
                lil_to_string(*argv.offset(0 as isize)),
                lil_to_string(*argv.offset(1 as isize)),
            ) != 0
            {
                0
            } else {
                1
            }) as i64,
        );
    }
}

extern "C" fn fnc_repstr(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut from = 0 as *const i8;
        let mut to = 0 as *const i8;
        let mut src = 0 as *mut i8;
        let mut sub = 0 as *const i8;
        let mut idx: u64 = 0;
        let mut fromlen: u64 = 0;
        let mut tolen: u64 = 0;
        let mut srclen: u64 = 0;
        let mut r = 0 as *mut _lil_value_t;
        if argc < 1 {
            return 0 as lil_value_t;
        }
        if argc < 3 {
            return lil_clone_value(*argv.offset(0 as isize));
        }
        from = lil_to_string(*argv.offset(1 as isize));
        to = lil_to_string(*argv.offset(2 as isize));
        if *from.offset(0 as isize) == 0 {
            return 0 as lil_value_t;
        }
        src = strclone(lil_to_string(*argv.offset(0 as isize)));
        srclen = strlen(src);
        fromlen = strlen(from);
        tolen = strlen(to);
        loop {
            sub = strstr(src, from);
            if sub.is_null() {
                break;
            }
            let mut newsrc = malloc(
                srclen
                    .wrapping_sub(fromlen)
                    .wrapping_add(tolen)
                    .wrapping_add(1),
            ) as *mut i8;
            idx = sub.offset_from(src) as u64;
            if idx != 0 {
                memcpy(newsrc as *mut libc::c_void, src as *const libc::c_void, idx);
            }
            memcpy(
                newsrc.offset(idx as isize) as *mut libc::c_void,
                to as *const libc::c_void,
                tolen,
            );
            memcpy(
                newsrc.offset(idx as isize).offset(tolen as isize) as *mut libc::c_void,
                src.offset(idx as isize).offset(fromlen as isize) as *const libc::c_void,
                srclen.wrapping_sub(idx).wrapping_sub(fromlen),
            );
            srclen = srclen.wrapping_sub(fromlen).wrapping_add(tolen);
            free(src as *mut libc::c_void);
            src = newsrc;
            *src.offset(srclen as isize) = 0;
        }
        r = lil_alloc_string(src);
        free(src as *mut libc::c_void);
        return r;
    }
}

extern "C" fn fnc_split(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut list = 0 as *mut _lil_list_t;
        let mut sep = b" \0" as *const u8 as *const i8;
        let mut i: u64 = 0;
        let mut val = 0 as *mut _lil_value_t;
        let mut str = 0 as *const i8;
        if argc == 0 {
            return 0 as lil_value_t;
        }
        if argc > 1 {
            sep = lil_to_string(*argv.offset(1 as isize));
            if sep.is_null() || *sep.offset(0 as isize) == 0 {
                return lil_clone_value(*argv.offset(0 as isize));
            }
        }
        val = lil_alloc_string(b"\0" as *const u8 as *const i8);
        str = lil_to_string(*argv.offset(0 as isize));
        list = lil_alloc_list();
        i = 0;
        while *str.offset(i as isize) != 0 {
            if !(strchr(sep, *str.offset(i as isize) as i32)).is_null() {
                lil_list_append(list, val);
                val = lil_alloc_string(b"\0" as *const u8 as *const i8);
            } else {
                lil_append_char(val, *str.offset(i as isize));
            }
            i = i.wrapping_add(1);
        }
        lil_list_append(list, val);
        val = lil_list_to_value(list, 1);
        lil_free_list(list);
        return val;
    }
}

extern "C" fn fnc_try(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut r = 0 as *mut _lil_value_t;
        if argc < 1 {
            return 0 as lil_value_t;
        }
        if (*lil).error != 0 {
            return 0 as lil_value_t;
        }
        r = lil_parse_value(lil, *argv.offset(0 as isize), 0);
        if (*lil).error != 0 {
            (*lil).error = 0;
            lil_free_value(r);
            if argc > 1 {
                r = lil_parse_value(lil, *argv.offset(1 as isize), 0);
            } else {
                r = 0 as lil_value_t;
            }
        }
        return r;
    }
}

extern "C" fn fnc_error(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        lil_set_error(
            lil,
            if argc > 0 {
                lil_to_string(*argv.offset(0 as isize))
            } else {
                0 as *const i8
            },
        );
        return 0 as lil_value_t;
    }
}

extern "C" fn fnc_exit(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        if ((*lil).callback[0 as usize]).is_some() {
            let mut proc_0: lil_exit_callback_proc_t = ::std::mem::transmute::<
                lil_callback_proc_t,
                lil_exit_callback_proc_t,
            >((*lil).callback[0 as usize]);
            proc_0.expect("non-null function pointer")(
                lil,
                if argc > 0 {
                    *argv.offset(0 as isize)
                } else {
                    0 as lil_value_t
                },
            );
        }
        return 0 as lil_value_t;
    }
}

extern "C" fn fnc_source(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut f = 0 as *mut FILE;
        let mut size: u64 = 0;
        let mut buffer = 0 as *mut i8;
        let mut r = 0 as *mut _lil_value_t;
        if argc < 1 {
            return 0 as lil_value_t;
        }
        if ((*lil).callback[4 as usize]).is_some() {
            let mut proc_0: lil_source_callback_proc_t = ::std::mem::transmute::<
                lil_callback_proc_t,
                lil_source_callback_proc_t,
            >((*lil).callback[4 as usize]);
            buffer = proc_0.expect("non-null function pointer")(
                lil,
                lil_to_string(*argv.offset(0 as isize)),
            );
        } else if ((*lil).callback[2 as usize]).is_some() {
            let mut proc_1: lil_read_callback_proc_t = ::std::mem::transmute::<
                lil_callback_proc_t,
                lil_read_callback_proc_t,
            >((*lil).callback[2 as usize]);
            buffer = proc_1.expect("non-null function pointer")(
                lil,
                lil_to_string(*argv.offset(0 as isize)),
            );
        } else {
            f = fopen(
                lil_to_string(*argv.offset(0 as isize)),
                b"rb\0" as *const u8 as *const i8,
            );
            if f.is_null() {
                return 0 as lil_value_t;
            }
            fseek(f, 0, 2);
            size = ftell(f) as u64;
            fseek(f, 0, 0);
            buffer = malloc(size.wrapping_add(1)) as *mut i8;
            fread(buffer as *mut libc::c_void, 1, size, f);
            *buffer.offset(size as isize) = 0;
            fclose(f);
        }
        r = lil_parse(lil, buffer, 0, 0);
        free(buffer as *mut libc::c_void);
        return r;
    }
}

extern "C" fn fnc_lmap(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        let mut list = 0 as *mut _lil_list_t;
        let mut i: u64 = 0;
        if argc < 2 {
            return 0 as lil_value_t;
        }
        list = lil_subst_to_list(lil, *argv.offset(0 as isize));
        i = 1;
        while i < argc {
            lil_set_var(
                lil,
                lil_to_string(*argv.offset(i as isize)),
                lil_list_get(list, i.wrapping_sub(1)),
                1,
            );
            i = i.wrapping_add(1);
        }
        lil_free_list(list);
        return 0 as lil_value_t;
    }
}

extern "C" fn fnc_rand(mut lil: lil_t, mut argc: u64, mut argv: *mut lil_value_t) -> lil_value_t {
    unsafe {
        return lil_alloc_double(rand() as f64 / 2147483647 as f64);
    }
}

extern "C" fn fnc_catcher(
    mut lil: lil_t,
    mut argc: u64,
    mut argv: *mut lil_value_t,
) -> lil_value_t {
    unsafe {
        if argc == 0 {
            return lil_alloc_string((*lil).catcher);
        } else {
            let mut catcher = lil_to_string(*argv.offset(0 as isize));
            free((*lil).catcher as *mut libc::c_void);
            let ref mut fresh133 = (*lil).catcher;
            *fresh133 = if *catcher.offset(0 as isize) as i32 != 0 {
                strclone(catcher)
            } else {
                0 as *mut i8
            };
        }
        return 0 as lil_value_t;
    }
}

extern "C" fn register_stdcmds(mut lil: lil_t) {
    lil_register(
        lil,
        b"reflect\0" as *const u8 as *const i8,
        Some(fnc_reflect as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"func\0" as *const u8 as *const i8,
        Some(fnc_func as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"rename\0" as *const u8 as *const i8,
        Some(fnc_rename as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"unusedname\0" as *const u8 as *const i8,
        Some(fnc_unusedname as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"quote\0" as *const u8 as *const i8,
        Some(fnc_quote as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"set\0" as *const u8 as *const i8,
        Some(fnc_set as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"local\0" as *const u8 as *const i8,
        Some(fnc_local as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"write\0" as *const u8 as *const i8,
        Some(fnc_write as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"print\0" as *const u8 as *const i8,
        Some(fnc_print as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"eval\0" as *const u8 as *const i8,
        Some(fnc_eval as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"topeval\0" as *const u8 as *const i8,
        Some(fnc_topeval as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"upeval\0" as *const u8 as *const i8,
        Some(fnc_upeval as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"downeval\0" as *const u8 as *const i8,
        Some(fnc_downeval as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"enveval\0" as *const u8 as *const i8,
        Some(fnc_enveval as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"jaileval\0" as *const u8 as *const i8,
        Some(fnc_jaileval as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"count\0" as *const u8 as *const i8,
        Some(fnc_count as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"index\0" as *const u8 as *const i8,
        Some(fnc_index as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"indexof\0" as *const u8 as *const i8,
        Some(fnc_indexof as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"filter\0" as *const u8 as *const i8,
        Some(fnc_filter as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"list\0" as *const u8 as *const i8,
        Some(fnc_list as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"append\0" as *const u8 as *const i8,
        Some(fnc_append as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"slice\0" as *const u8 as *const i8,
        Some(fnc_slice as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"subst\0" as *const u8 as *const i8,
        Some(fnc_subst as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"concat\0" as *const u8 as *const i8,
        Some(fnc_concat as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"foreach\0" as *const u8 as *const i8,
        Some(fnc_foreach as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"return\0" as *const u8 as *const i8,
        Some(fnc_return as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"result\0" as *const u8 as *const i8,
        Some(fnc_result as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"expr\0" as *const u8 as *const i8,
        Some(fnc_expr as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"inc\0" as *const u8 as *const i8,
        Some(fnc_inc as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"dec\0" as *const u8 as *const i8,
        Some(fnc_dec as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"read\0" as *const u8 as *const i8,
        Some(fnc_read as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"store\0" as *const u8 as *const i8,
        Some(fnc_store as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"if\0" as *const u8 as *const i8,
        Some(fnc_if as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"while\0" as *const u8 as *const i8,
        Some(fnc_while as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"for\0" as *const u8 as *const i8,
        Some(fnc_for as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"char\0" as *const u8 as *const i8,
        Some(fnc_char as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"charat\0" as *const u8 as *const i8,
        Some(fnc_charat as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"codeat\0" as *const u8 as *const i8,
        Some(fnc_codeat as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"substr\0" as *const u8 as *const i8,
        Some(fnc_substr as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"strpos\0" as *const u8 as *const i8,
        Some(fnc_strpos as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"length\0" as *const u8 as *const i8,
        Some(fnc_length as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"trim\0" as *const u8 as *const i8,
        Some(fnc_trim as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"ltrim\0" as *const u8 as *const i8,
        Some(fnc_ltrim as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"rtrim\0" as *const u8 as *const i8,
        Some(fnc_rtrim as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"strcmp\0" as *const u8 as *const i8,
        Some(fnc_strcmp as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"streq\0" as *const u8 as *const i8,
        Some(fnc_streq as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"repstr\0" as *const u8 as *const i8,
        Some(fnc_repstr as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"split\0" as *const u8 as *const i8,
        Some(fnc_split as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"try\0" as *const u8 as *const i8,
        Some(fnc_try as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"error\0" as *const u8 as *const i8,
        Some(fnc_error as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"exit\0" as *const u8 as *const i8,
        Some(fnc_exit as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"source\0" as *const u8 as *const i8,
        Some(fnc_source as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"lmap\0" as *const u8 as *const i8,
        Some(fnc_lmap as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"rand\0" as *const u8 as *const i8,
        Some(fnc_rand as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    lil_register(
        lil,
        b"catcher\0" as *const u8 as *const i8,
        Some(fnc_catcher as unsafe extern "C" fn(lil_t, u64, *mut lil_value_t) -> lil_value_t),
    );
    (*lil).syscmds = (*lil).cmds;
}
