pub mod p3_benchmarks;
pub mod wf_benchmarks;

pub use p3_benchmarks::*;
pub use wf_benchmarks::*;

use wasm_bindgen::prelude::*;

// Import the `console.log` function from the `console` module
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to make `console.log` easier to use
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn bench_p3_lde() -> f64 {
    let start = js_sys::Date::now();
    p3_benchmarks::run_lde_bench();
    let end = js_sys::Date::now();
    end - start
}

#[wasm_bindgen]
pub fn bench_p3_merkle() -> f64 {
    let start = js_sys::Date::now();
    p3_benchmarks::run_merkle_bench();
    let end = js_sys::Date::now();
    end - start
}

#[wasm_bindgen]
pub fn bench_wf_lde() -> f64 {
    let start = js_sys::Date::now();
    wf_benchmarks::run_lde_bench();
    let end = js_sys::Date::now();
    end - start
}

#[wasm_bindgen]
pub fn bench_wf_merkle() -> f64 {
    let start = js_sys::Date::now();
    wf_benchmarks::run_merkle_bench();
    let end = js_sys::Date::now();
    end - start
}
