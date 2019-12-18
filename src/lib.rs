mod utils;

use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

//struct that will be sent back to the JavaScript layer with result of Rust sorting.
#[wasm_bindgen]
pub struct OutputWrapper {
    time_ms: f64,
    num_inputs: u32,
    alg_name: String
}

#[wasm_bindgen]
impl OutputWrapper {
    //run() is called by JavaScript layer. it creates the algorithm input, sets up the timer,
    //calls appropriate algorithm, and returns results to JavaScript layer in an OutputWrapper.
    pub fn run (name: String, num_inputs: u32) -> OutputWrapper {
        //create array of random inputs. 1 to num_inputs inclusive no dups
        let mut input_vec: Vec<u32> = (1..num_inputs + 1).collect();

        //randomize input vector
        let slice: &mut [u32] = &mut input_vec;
        let seed = [0; 32]; //have to seed the rng for shuffling because we don't have access to OS random bits
        let mut rng = ChaChaRng::from_seed(seed);
        slice.shuffle(&mut rng);
        assert_eq!(is_sorted(&input_vec), false); //check not sorted

        //setup timer
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");
        let mut start_time: f64 = 0 as f64;

        //start timer and call function
        let name: &str = &name;
        match name {
            "selection sort" => {
                start_time = performance.now(); //start timer in each arm to make more accurate for each function
                selection_sort(&mut input_vec);
            },
            _ => {start_time = performance.now();}
        }
        let end_time: f64 = performance.now(); //finish timing
        assert_eq!(is_sorted(&input_vec), true); //check sorted
        let time_ms = end_time - start_time;

        //output
        OutputWrapper {
            time_ms,
            num_inputs,
            alg_name: String::from(name)
        }
    }

    //get time function for JS layer
    pub fn time_ms (&self) -> f64 {
        self.time_ms
    }

    pub fn num_inputs (&self) -> u32{
        self.num_inputs
    }

    pub fn alg_name (&self) -> &str{
        &self.alg_name
    }
}

//used with assertions to test vectors
fn is_sorted (input_vec: &Vec<u32>) -> bool {
    for i in 1..input_vec.len() {
        if input_vec.get(i) < input_vec.get(i -1) {
            return false;
        }
    }
    true
}

fn selection_sort (input_vec: &mut Vec<u32>) {
    for i in 0..input_vec.len() {
        let mut small = i;

        for j in (i + 1)..input_vec.len() {
            if input_vec[j] < input_vec[small] {
                small = j;
            }
        }

        input_vec.swap(small, i);
    }
}