use libc;
extern "C" {
    fn rand() -> i32;
    fn pow(_: f64, _: f64) -> f64;
    fn genann_init(inputs: i32, hidden_layers: i32, hidden: i32, outputs: i32) -> *mut genann;
    fn genann_randomize(ann: *mut genann);
    fn genann_copy(ann: *const genann) -> *mut genann;
    fn genann_free(ann: *mut genann);
    fn genann_run(ann: *const genann, inputs: *const f64) -> *const f64;
}
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
fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    unsafe {
        print!("GENANN example 2.\n");
        print!("Train a small ANN to the XOR function using random search.\n");
        let input: [[f64; 2]; 4] = [
            [0 as f64, 0 as f64],
            [0 as f64, 1 as f64],
            [1 as f64, 0 as f64],
            [1 as f64, 1 as f64],
        ];
        let output: [f64; 4] = [0 as f64, 1 as f64, 1 as f64, 0 as f64];
        let mut i: i32 = 0;
        let mut ann = genann_init(2, 1, 2, 1);
        let mut err: f64 = 0.;
        let mut last_err = 1000 as f64;
        let mut count = 0;
        loop {
            count += 1;
            if count % 1000 == 0 {
                genann_randomize(ann);
            }
            let mut save = genann_copy(ann);
            i = 0;
            while i < (*ann).total_weights {
                *((*ann).weight).offset(i as isize) += rand() as f64 / 2147483647 as f64 - 0.5f64;
                i += 1;
            }
            err = 0 as f64;
            err += pow(
                *genann_run(ann, (input[0 as usize]).as_ptr()) - output[0 as usize],
                2.0f64,
            );
            err += pow(
                *genann_run(ann, (input[1 as usize]).as_ptr()) - output[1 as usize],
                2.0f64,
            );
            err += pow(
                *genann_run(ann, (input[2 as usize]).as_ptr()) - output[2 as usize],
                2.0f64,
            );
            err += pow(
                *genann_run(ann, (input[3 as usize]).as_ptr()) - output[3 as usize],
                2.0f64,
            );
            if err < last_err {
                genann_free(save);
                last_err = err;
            } else {
                genann_free(ann);
                ann = save;
            }
            if !(err > 0.01f64) {
                break;
            }
        }
        print!("Finished in {} loops.\n,9999");
        print!("");
        print!("");
        print!("");
        print!("");
        genann_free(ann);
        return 0;
    }
}
