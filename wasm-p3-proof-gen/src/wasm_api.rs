use ark_std::string::ToString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = ["performance"], js_name = now)]
    fn performance_now() -> f64;
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => ($crate::wasm_api::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn run_example_blake3_wasm(num_steps: usize, num_col: usize) {
    crate::proof::run_example_blake3(num_steps, num_col);
}
