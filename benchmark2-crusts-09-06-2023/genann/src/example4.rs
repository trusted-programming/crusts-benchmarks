use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn fseek(__stream: *mut FILE, __off: i64, __whence: i32) -> i32;
    fn feof(__stream: *mut FILE) -> i32;
    fn perror(__s: *const i8);
    fn atof(__nptr: *const i8) -> f64;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn exit(_: i32) -> !;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strtok(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn genann_init(inputs: i32, hidden_layers: i32, hidden: i32, outputs: i32) -> *mut genann;
    fn genann_free(ann: *mut genann);
    fn genann_run(ann: *const genann, inputs: *const f64) -> *const f64;
    fn genann_train(
        ann: *const genann,
        inputs: *const f64,
        desired_outputs: *const f64,
        learning_rate: f64,
    );
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
pub static mut iris_data: *const i8 = b"example/iris.data\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut input: *mut f64 = 0 as *const f64 as *mut f64;
#[no_mangle]
pub static mut class: *mut f64 = 0 as *const f64 as *mut f64;
#[no_mangle]
pub static mut samples: i32 = 0;
#[no_mangle]
pub static mut class_names: [*const i8; 3] = [
    b"Iris-setosa\0" as *const u8 as *const i8,
    b"Iris-versicolor\0" as *const u8 as *const i8,
    b"Iris-virginica\0" as *const u8 as *const i8,
];
#[no_mangle]
pub extern "C" fn load_data() {
    unsafe {
        let mut in_0 = fopen(
            b"example/iris.data\0" as *const u8 as *const i8,
            b"r\0" as *const u8 as *const i8,
        );
        if in_0.is_null() {
            print!("Could not open file: {}\n,9998");
            exit(1);
        }
        let mut line: [i8; 1024] = [0; 1024];
        while feof(in_0) == 0 && !(fgets(line.as_mut_ptr(), 1024, in_0)).is_null() {
            samples += 1;
        }
        fseek(in_0, 0, 0);
        print!("Loading {} data points from {}\n,9999,9998");
        input = malloc(
            (::std::mem::size_of::<f64>() as u64)
                .wrapping_mul(samples as u64)
                .wrapping_mul(4),
        ) as *mut f64;
        class = malloc(
            (::std::mem::size_of::<f64>() as u64)
                .wrapping_mul(samples as u64)
                .wrapping_mul(3),
        ) as *mut f64;
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        i = 0;
        while i < samples {
            let mut p = input.offset((i * 4i32) as isize);
            let mut c = class.offset((i * 3i32) as isize);
            let ref mut fresh0 = *c.offset(2 as isize);
            *fresh0 = 0.0f64;
            let ref mut fresh1 = *c.offset(1 as isize);
            *fresh1 = *fresh0;
            *c.offset(0 as isize) = *fresh1;
            if (fgets(line.as_mut_ptr(), 1024, in_0)).is_null() {
                perror(b"fgets\0" as *const u8 as *const i8);
                exit(1);
            }
            let mut split = strtok(line.as_mut_ptr(), b",\0" as *const u8 as *const i8);
            j = 0;
            while j < 4 {
                *p.offset(j as isize) = atof(split);
                split = strtok(0 as *mut i8, b",\0" as *const u8 as *const i8);
                j += 1;
            }
            *split.offset((strlen(split)).wrapping_sub(1) as isize) = 0;
            if strcmp(split, class_names[0 as usize]) == 0 {
                *c.offset(0 as isize) = 1.0f64;
            } else if strcmp(split, class_names[1 as usize]) == 0 {
                *c.offset(1 as isize) = 1.0f64;
            } else if strcmp(split, class_names[2 as usize]) == 0 {
                *c.offset(2 as isize) = 1.0f64;
            } else {
                print!("Unknown class {}.\n,9998");
                exit(1);
            }
            i += 1;
        }
        fclose(in_0);
    }
}

fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        print!("GENANN example 4.\n");
        print!("Train an ANN on the IRIS dataset using backpropagation.\n");
        load_data();
        let mut ann = genann_init(4, 1, 4, 3);
        let mut i: i32 = 0;
        let mut j: i32 = 0;
        let mut loops = 5000;
        print!("Training for {} loops over data.\n,9999");
        i = 0;
        while i < loops {
            j = 0;
            while j < samples {
                genann_train(
                    ann,
                    input.offset((j * 4i32) as isize),
                    class.offset((j * 3i32) as isize),
                    0.01f64,
                );
                j += 1;
            }
            i += 1;
        }
        let mut correct = 0;
        j = 0;
        while j < samples {
            let mut guess = genann_run(ann, input.offset((j * 4i32) as isize));
            if *class.offset((j * 3 + 0i32) as isize) == 1.0f64 {
                if *guess.offset(0 as isize) > *guess.offset(1 as isize)
                    && *guess.offset(0 as isize) > *guess.offset(2 as isize)
                {
                    correct += 1;
                }
            } else if *class.offset((j * 3 + 1i32) as isize) == 1.0f64 {
                if *guess.offset(1 as isize) > *guess.offset(0 as isize)
                    && *guess.offset(1 as isize) > *guess.offset(2 as isize)
                {
                    correct += 1;
                }
            } else if *class.offset((j * 3 + 2i32) as isize) == 1.0f64 {
                if *guess.offset(2 as isize) > *guess.offset(0 as isize)
                    && *guess.offset(2 as isize) > *guess.offset(1 as isize)
                {
                    correct += 1;
                }
            } else {
                print!("Logic error.\n");
                exit(1);
            }
            j += 1;
        }
        print!("{}/{} correct ({:0.1}%).\n,9999,9999,9999");
        genann_free(ann);
        return 0;
    }
}
