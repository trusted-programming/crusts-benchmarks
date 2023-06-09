use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn perror(__s: *const i8);
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __errno_location() -> *mut i32;
    fn exp(_: f64) -> f64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn rand() -> i32;
    fn free(_: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
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
pub type genann_actfun = Option<unsafe extern "C" fn(f64) -> f64>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct genann {
    pub inputs: i32,
    pub hidden_layers: i32,
    pub hidden: i32,
    pub outputs: i32,
    pub activation_hidden: genann_actfun,
    pub activation_output: genann_actfun,
    pub total_weights: i32,
    pub total_neurons: i32,
    pub weight: *mut f64,
    pub output: *mut f64,
    pub delta: *mut f64,
}
#[no_mangle]
pub extern "C" fn genann_act_sigmoid(mut a: f64) -> f64 {
    if a < -45.0f64 {
        return 0 as f64;
    }
    if a > 45.0f64 {
        return 1 as f64;
    }
    unsafe {
        return 1.0f64 / (1 as f64 + exp(-a));
    }
}

#[no_mangle]
pub extern "C" fn genann_act_sigmoid_cached(mut a: f64) -> f64 {
    let min = -15.0f64;
    let max = 15.0f64;
    unsafe {
        static mut interval: f64 = 0.;
        static mut initialized: i32 = 0;
        static mut lookup: [f64; 4096] = [0.; 4096];
        if initialized == 0 {
            interval = (max - min) / 4096 as f64;
            let mut i: i32 = 0;
            i = 0;
            while i < 4096 {
                lookup[i as usize] = genann_act_sigmoid(min + interval * i as f64);
                i += 1;
            }
            initialized = 1;
        }
        let mut i_0: i32 = 0;
        i_0 = ((a - min) / interval + 0.5f64) as i32;
        if i_0 <= 0 {
            return lookup[0 as usize];
        }
        if i_0 >= 4096 {
            return lookup[(4096 - 1i32) as usize];
        }
        return lookup[i_0 as usize];
    }
}

#[no_mangle]
pub extern "C" fn genann_act_threshold(mut a: f64) -> f64 {
    return (a > 0 as f64) as i32 as f64;
}

#[no_mangle]
pub extern "C" fn genann_act_linear(mut a: f64) -> f64 {
    return a;
}

#[no_mangle]
pub extern "C" fn genann_init(
    mut inputs: i32,
    mut hidden_layers: i32,
    mut hidden: i32,
    mut outputs: i32,
) -> *mut genann {
    if hidden_layers < 0 {
        return 0 as *mut genann;
    }
    if inputs < 1 {
        return 0 as *mut genann;
    }
    if outputs < 1 {
        return 0 as *mut genann;
    }
    if hidden_layers > 0 && hidden < 1 {
        return 0 as *mut genann;
    }
    let hidden_weights = if hidden_layers != 0 {
        (inputs + 1) * hidden + (hidden_layers - 1) * (hidden + 1) * hidden
    } else {
        0
    };
    let output_weights = (if hidden_layers != 0 {
        hidden + 1
    } else {
        inputs + 1
    }) * outputs;
    let total_weights = hidden_weights + output_weights;
    let total_neurons = inputs + hidden * hidden_layers + outputs;
    let size = (::std::mem::size_of::<genann>() as u64).wrapping_add(
        (::std::mem::size_of::<f64>() as u64)
            .wrapping_mul((total_weights + total_neurons + (total_neurons - inputs)) as u64),
    ) as i32;
    unsafe {
        let mut ret = malloc(size as u64) as *mut genann;
        if ret.is_null() {
            return 0 as *mut genann;
        };
        (*ret).inputs = inputs;
        (*ret).hidden_layers = hidden_layers;
        (*ret).hidden = hidden;
        (*ret).outputs = outputs;
        (*ret).total_weights = total_weights;
        (*ret).total_neurons = total_neurons;
        let ref mut fresh0 = (*ret).weight;
        *fresh0 =
            (ret as *mut i8).offset(::std::mem::size_of::<genann>() as u64 as isize) as *mut f64;
        let ref mut fresh1 = (*ret).output;
        *fresh1 = ((*ret).weight).offset((*ret).total_weights as isize);
        let ref mut fresh2 = (*ret).delta;
        *fresh2 = ((*ret).output).offset((*ret).total_neurons as isize);
        genann_randomize(ret);
        let ref mut fresh3 = (*ret).activation_hidden;
        *fresh3 = Some(genann_act_sigmoid_cached as unsafe extern "C" fn(f64) -> f64);
        let ref mut fresh4 = (*ret).activation_output;
        *fresh4 = Some(genann_act_sigmoid_cached as unsafe extern "C" fn(f64) -> f64);
        return ret;
    }
}

#[no_mangle]
pub extern "C" fn genann_read(mut in_0: *mut FILE) -> *mut genann {
    unsafe {
        let mut inputs: i32 = 0;
        let mut hidden_layers: i32 = 0;
        let mut hidden: i32 = 0;
        let mut outputs: i32 = 0;
        let mut rc: i32 = 0;
        *__errno_location() = 0;
        rc = fscanf(
            in_0,
            b"%d %d %d %d\0" as *const u8 as *const i8,
            &mut inputs as *mut i32,
            &mut hidden_layers as *mut i32,
            &mut hidden as *mut i32,
            &mut outputs as *mut i32,
        );
        if rc < 4 || *__errno_location() != 0 {
            perror(b"fscanf\0" as *const u8 as *const i8);
            return 0 as *mut genann;
        }
        let mut ann = genann_init(inputs, hidden_layers, hidden, outputs);
        let mut i: i32 = 0;
        i = 0;
        while i < (*ann).total_weights {
            *__errno_location() = 0;
            rc = fscanf(
                in_0,
                b" %le\0" as *const u8 as *const i8,
                ((*ann).weight).offset(i as isize),
            );
            if rc < 1 || *__errno_location() != 0 {
                perror(b"fscanf\0" as *const u8 as *const i8);
                genann_free(ann);
                return 0 as *mut genann;
            }
            i += 1;
        }
        return ann;
    }
}

#[no_mangle]
pub extern "C" fn genann_copy(mut ann: *const genann) -> *mut genann {
    unsafe {
        let size = (::std::mem::size_of::<genann>() as u64).wrapping_add(
            (::std::mem::size_of::<f64>() as u64).wrapping_mul(
                ((*ann).total_weights
                    + (*ann).total_neurons
                    + ((*ann).total_neurons - (*ann).inputs)) as u64,
            ),
        ) as i32;
        let mut ret = malloc(size as u64) as *mut genann;
        if ret.is_null() {
            return 0 as *mut genann;
        }
        memcpy(
            ret as *mut libc::c_void,
            ann as *const libc::c_void,
            size as u64,
        );
        let ref mut fresh5 = (*ret).weight;
        *fresh5 =
            (ret as *mut i8).offset(::std::mem::size_of::<genann>() as u64 as isize) as *mut f64;
        let ref mut fresh6 = (*ret).output;
        *fresh6 = ((*ret).weight).offset((*ret).total_weights as isize);
        let ref mut fresh7 = (*ret).delta;
        *fresh7 = ((*ret).output).offset((*ret).total_neurons as isize);
        return ret;
    }
}

#[no_mangle]
pub extern "C" fn genann_randomize(mut ann: *mut genann) {
    unsafe {
        let mut i: i32 = 0;
        i = 0;
        while i < (*ann).total_weights {
            let mut r = rand() as f64 / 2147483647 as f64;
            *((*ann).weight).offset(i as isize) = r - 0.5f64;
            i += 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn genann_free(mut ann: *mut genann) {
    unsafe {
        free(ann as *mut libc::c_void);
    }
}

#[no_mangle]
pub extern "C" fn genann_run(mut ann: *const genann, mut inputs: *const f64) -> *const f64 {
    unsafe {
        let mut w: *const f64 = (*ann).weight;
        let mut o = ((*ann).output).offset((*ann).inputs as isize);
        let mut i: *const f64 = (*ann).output;
        memcpy(
            (*ann).output as *mut libc::c_void,
            inputs as *const libc::c_void,
            (::std::mem::size_of::<f64>() as u64).wrapping_mul((*ann).inputs as u64),
        );
        let mut h: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let act: genann_actfun = (*ann).activation_hidden;
        let acto: genann_actfun = (*ann).activation_output;
        h = 0;
        while h < (*ann).hidden_layers {
            j = 0;
            while j < (*ann).hidden {
                let fresh8 = w;
                w = w.offset(1);
                let mut sum = *fresh8 * -1.0f64;
                k = 0;
                while k < (if h == 0 { (*ann).inputs } else { (*ann).hidden }) {
                    let fresh9 = w;
                    w = w.offset(1);
                    sum += *fresh9 * *i.offset(k as isize);
                    k += 1;
                }
                let fresh10 = o;
                o = o.offset(1);
                *fresh10 = act.expect("non-null function pointer")(sum);
                j += 1;
            }
            i = i.offset(
                (if h == 0i32 {
                    (*ann).inputs
                } else {
                    (*ann).hidden
                }) as isize,
            );
            h += 1;
        }
        let mut ret: *const f64 = o;
        j = 0;
        while j < (*ann).outputs {
            let fresh11 = w;
            w = w.offset(1);
            let mut sum_0 = *fresh11 * -1.0f64;
            k = 0;
            while k
                < (if (*ann).hidden_layers != 0 {
                    (*ann).hidden
                } else {
                    (*ann).inputs
                })
            {
                let fresh12 = w;
                w = w.offset(1);
                sum_0 += *fresh12 * *i.offset(k as isize);
                k += 1;
            }
            let fresh13 = o;
            o = o.offset(1);
            *fresh13 = acto.expect("non-null function pointer")(sum_0);
            j += 1;
        }
        if w.offset_from((*ann).weight) as i64 == (*ann).total_weights as i64 {
        } else {
            __assert_fail(
                b"w - ann->weight == ann->total_weights\0" as *const u8 as *const i8,
                b"genann.c\0" as *const u8 as *const i8,
                225,
                (*::std::mem::transmute::<&[u8; 57], &[i8; 57]>(
                    b"const double *genann_run(const genann *, const double *)\0",
                ))
                .as_ptr(),
            );
        }
        if o.offset_from((*ann).output) as i64 == (*ann).total_neurons as i64 {
        } else {
            __assert_fail(
                b"o - ann->output == ann->total_neurons\0" as *const u8 as *const i8,
                b"genann.c\0" as *const u8 as *const i8,
                226,
                (*::std::mem::transmute::<&[u8; 57], &[i8; 57]>(
                    b"const double *genann_run(const genann *, const double *)\0",
                ))
                .as_ptr(),
            );
        }
        return ret;
    }
}

#[no_mangle]
pub extern "C" fn genann_train(
    mut ann: *const genann,
    mut inputs: *const f64,
    mut desired_outputs: *const f64,
    mut learning_rate: f64,
) {
    unsafe {
        genann_run(ann, inputs);
        let mut h: i32 = 0;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut o: *const f64 = ((*ann).output)
            .offset((*ann).inputs as isize)
            .offset(((*ann).hidden * (*ann).hidden_layers) as isize);
        let mut d = ((*ann).delta).offset(((*ann).hidden * (*ann).hidden_layers) as isize);
        let mut t = desired_outputs;
        if (*ann).activation_output == Some(genann_act_linear as unsafe extern "C" fn(f64) -> f64) {
            j = 0;
            while j < (*ann).outputs {
                let fresh14 = t;
                t = t.offset(1);
                let fresh15 = o;
                o = o.offset(1);
                let fresh16 = d;
                d = d.offset(1);
                *fresh16 = *fresh14 - *fresh15;
                j += 1;
            }
        } else {
            j = 0;
            while j < (*ann).outputs {
                let fresh17 = d;
                d = d.offset(1);
                *fresh17 = (*t - *o) * *o * (1.0f64 - *o);
                o = o.offset(1);
                t = t.offset(1);
                j += 1;
            }
        }
        h = (*ann).hidden_layers - 1;
        while h >= 0 {
            let mut o_0: *const f64 = ((*ann).output)
                .offset((*ann).inputs as isize)
                .offset((h * (*ann).hidden) as isize);
            let mut d_0 = ((*ann).delta).offset((h * (*ann).hidden) as isize);
            let dd: *const f64 = ((*ann).delta).offset(((h + 1i32) * (*ann).hidden) as isize);
            let ww: *const f64 = ((*ann).weight)
                .offset((((*ann).inputs + 1i32) * (*ann).hidden) as isize)
                .offset((((*ann).hidden + 1i32) * (*ann).hidden * h) as isize);
            j = 0;
            while j < (*ann).hidden {
                let mut delta = 0 as f64;
                k = 0;
                while k
                    < (if h == (*ann).hidden_layers - 1 {
                        (*ann).outputs
                    } else {
                        (*ann).hidden
                    })
                {
                    let forward_delta = *dd.offset(k as isize);
                    let windex = k * ((*ann).hidden + 1) + (j + 1);
                    let forward_weight = *ww.offset(windex as isize);
                    delta += forward_delta * forward_weight;
                    k += 1;
                }
                *d_0 = *o_0 * (1.0f64 - *o_0) * delta;
                d_0 = d_0.offset(1);
                o_0 = o_0.offset(1);
                j += 1;
            }
            h -= 1;
        }
        let mut d_1: *const f64 =
            ((*ann).delta).offset(((*ann).hidden * (*ann).hidden_layers) as isize);
        let mut w = ((*ann).weight).offset(
            (if (*ann).hidden_layers != 0 {
                ((*ann).inputs + 1i32) * (*ann).hidden
                    + ((*ann).hidden + 1i32) * (*ann).hidden * ((*ann).hidden_layers - 1)
            } else {
                0
            }) as isize,
        );
        let i: *const f64 = ((*ann).output).offset(
            (if (*ann).hidden_layers != 0 {
                (*ann).inputs + (*ann).hidden * ((*ann).hidden_layers - 1i32)
            } else {
                0i32
            }) as isize,
        );
        j = 0;
        while j < (*ann).outputs {
            k = 0;
            while k
                < (if (*ann).hidden_layers != 0 {
                    (*ann).hidden
                } else {
                    (*ann).inputs
                }) + 1
            {
                if k == 0 {
                    let fresh18 = w;
                    w = w.offset(1);
                    *fresh18 += *d_1 * learning_rate * -1.0f64;
                } else {
                    let fresh19 = w;
                    w = w.offset(1);
                    *fresh19 += *d_1 * learning_rate * *i.offset((k - 1i32) as isize);
                }
                k += 1;
            }
            d_1 = d_1.offset(1);
            j += 1;
        }
        if w.offset_from((*ann).weight) as i64 == (*ann).total_weights as i64 {
        } else {
            __assert_fail(
                b"w - ann->weight == ann->total_weights\0" as *const u8 as *const i8,
                b"genann.c\0" as *const u8 as *const i8,
                318,
                (*::std::mem::transmute::<&[u8; 74], &[i8; 74]>(
                    b"void genann_train(const genann *, const double *, const double *, double)\0",
                ))
                .as_ptr(),
            );
        }
        h = (*ann).hidden_layers - 1;
        while h >= 0 {
            let mut d_2: *const f64 = ((*ann).delta).offset((h * (*ann).hidden) as isize);
            let mut i_0: *const f64 = ((*ann).output).offset(
                (if h != 0 {
                    (*ann).inputs + (*ann).hidden * (h - 1i32)
                } else {
                    0i32
                }) as isize,
            );
            let mut w_0 = ((*ann).weight).offset(
                (if h != 0 {
                    ((*ann).inputs + 1i32) * (*ann).hidden
                        + ((*ann).hidden + 1i32) * (*ann).hidden * (h - 1)
                } else {
                    0
                }) as isize,
            );
            j = 0;
            while j < (*ann).hidden {
                k = 0;
                while k < (if h == 0 { (*ann).inputs } else { (*ann).hidden }) + 1 {
                    if k == 0 {
                        let fresh20 = w_0;
                        w_0 = w_0.offset(1);
                        *fresh20 += *d_2 * learning_rate * -1.0f64;
                    } else {
                        let fresh21 = w_0;
                        w_0 = w_0.offset(1);
                        *fresh21 += *d_2 * learning_rate * *i_0.offset((k - 1i32) as isize);
                    }
                    k += 1;
                }
                d_2 = d_2.offset(1);
                j += 1;
            }
            h -= 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn genann_write(mut ann: *const genann, mut out: *mut FILE) {
    unsafe {
        fprintf(
            out,
            b"%d %d %d %d\0" as *const u8 as *const i8,
            (*ann).inputs,
            (*ann).hidden_layers,
            (*ann).hidden,
            (*ann).outputs,
        );
        let mut i: i32 = 0;
        i = 0;
        while i < (*ann).total_weights {
            fprintf(
                out,
                b" %.20e\0" as *const u8 as *const i8,
                *((*ann).weight).offset(i as isize),
            );
            i += 1;
        }
    }
}
