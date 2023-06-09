use libc;
extern "C" {
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
        print!("GENANN example 1.\n");
        print!("Train a small ANN to the XOR function using backpropagation.\n");
        let input: [[f64; 2]; 4] = [
            [0 as f64, 0 as f64],
            [0 as f64, 1 as f64],
            [1 as f64, 0 as f64],
            [1 as f64, 1 as f64],
        ];
        let output: [f64; 4] = [0 as f64, 1 as f64, 1 as f64, 0 as f64];
        let mut i: i32 = 0;
        let mut ann = genann_init(2, 1, 2, 1);
        i = 0;
        while i < 300 {
            genann_train(
                ann,
                (input[0 as usize]).as_ptr(),
                output.as_ptr().offset(0 as isize),
                3 as f64,
            );
            genann_train(
                ann,
                (input[1 as usize]).as_ptr(),
                output.as_ptr().offset(1 as isize),
                3 as f64,
            );
            genann_train(
                ann,
                (input[2 as usize]).as_ptr(),
                output.as_ptr().offset(2 as isize),
                3 as f64,
            );
            genann_train(
                ann,
                (input[3 as usize]).as_ptr(),
                output.as_ptr().offset(3 as isize),
                3 as f64,
            );
            i += 1;
        }
        print!("");
        print!("");
        print!("");
        print!("");
        genann_free(ann);
        return 0;
    }
}
