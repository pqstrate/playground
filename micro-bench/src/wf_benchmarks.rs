// use rand::rngs::StdRng;
// use rand::{Rng, SeedableRng};
use std::time::Instant;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[cfg(not(target_arch = "wasm32"))]
macro_rules! console_log {
    ($($t:tt)*) => (println!($($t)*))
}
use winterfell::{
    crypto::{hashers::Blake3_256, Hasher, MerkleTree},
    math::{fft, fields::f64::BaseElement},
};

#[cfg(target_arch = "wasm32")]
const POLY_SIZE: usize = 1 << 16; // 2^16 for WASM (smaller for browser performance)

#[cfg(not(target_arch = "wasm32"))]
const POLY_SIZE: usize = 1 << 19; // 2^19

pub fn run_lde_bench() {
    console_log!("WF LDE Benchmark - Polynomial size: {}", POLY_SIZE);

    // Generate polynomial data
    // #[cfg(target_arch = "wasm32")]
    let mut poly: Vec<BaseElement> = (0..POLY_SIZE)
        .map(|i| BaseElement::new((1u64 << 55) + (i as u64)))
        .collect();
    
    // #[cfg(not(target_arch = "wasm32"))]
    // let mut poly: Vec<BaseElement> = {
    //     let mut rng = StdRng::seed_from_u64(42);
    //     (0..POLY_SIZE)
    //         .map(|_| BaseElement::new(rng.random::<u64>()))
    //         .collect()
    // };

    // Benchmark LDE (Low Degree Extension)
    let start = Instant::now();

    let twiddles = fft::get_twiddles::<BaseElement>(POLY_SIZE);
    fft::evaluate_poly(&mut poly, &twiddles);

    let lde_time = start.elapsed();
    console_log!("WF LDE time: {:?}", lde_time);

    // Benchmark inverse FFT
    let start = Instant::now();
    let inv_twiddles = fft::get_inv_twiddles::<BaseElement>(POLY_SIZE);
    fft::interpolate_poly(&mut poly, &inv_twiddles);
    let ifft_time = start.elapsed();
    console_log!("WF IFFT time: {:?}", ifft_time);
}

pub fn run_merkle_bench() {
    console_log!("WF Merkle Tree Benchmark - {} leaves", POLY_SIZE);
    {
        // Generate data for Merkle tree
        // #[cfg(target_arch = "wasm32")]
        let leaves: Vec<_> = (0..POLY_SIZE)
            .map(|i| {
                let val = (1u64 << 55) + (i as u64);
                let bytes = val.to_le_bytes();
                let mut full_bytes = [0u8; 32];
                full_bytes[..8].copy_from_slice(&bytes);
                Blake3_256::<BaseElement>::hash(&full_bytes)
            })
            .collect();
        
        // #[cfg(not(target_arch = "wasm32"))]
        // let leaves: Vec<_> = {
        //     let mut rng = StdRng::seed_from_u64(42);
        //     (0..POLY_SIZE)
        //         .map(|_| {
        //             let bytes: [u8; 32] = rng.random();
        //             Blake3_256::<BaseElement>::hash(&bytes)
        //         })
        //         .collect()
        // };

        let start = Instant::now();
        let _tree = MerkleTree::<Blake3_256<BaseElement>>::new(leaves).unwrap();
        let blake3_commit_time = start.elapsed();
        console_log!("WF Blake3_256 Merkle commit time: {:?}", blake3_commit_time);
    }
}
