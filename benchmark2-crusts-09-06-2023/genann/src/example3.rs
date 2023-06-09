use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn exit(_: i32) -> !;
    fn genann_read(in_0: *mut FILE) -> *mut genann;
    fn genann_free(ann: *mut genann);
    fn genann_run(ann: *const genann, inputs: *const f64) -> *const f64;
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
pub static mut save_name: *const i8 = b"example/xor.ann\0" as *const u8 as *const i8;
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        print!("GENANN example 3.\n");
        print!("Load a saved ANN to solve the XOR function.\n");
        let mut saved = fopen(save_name, b"r\0" as *const u8 as *const i8);
        if saved.is_null() {
            print!("mpty");
            exit(1);
        }
        let mut ann = genann_read(saved);
        fclose(saved);
        if ann.is_null() {
            print!("Error loading ANN from file: {}.,9998");
            exit(1);
        }
        let input: [[f64; 2]; 4] = [
            [0 as f64, 0 as f64],
            [0 as f64, 1 as f64],
            [1 as f64, 0 as f64],
            [1 as f64, 1 as f64],
        ];
        print!("");
        print!("");
        print!("");
        print!("");
        genann_free(ann);
        return 0;
    }
}
