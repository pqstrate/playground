use p3_blake3::Blake3;
use p3_commit::Mmcs;
use p3_dft::{Radix2DitParallel, TwoAdicSubgroupDft};
use p3_field::PrimeCharacteristicRing;
use p3_goldilocks::Goldilocks;
use p3_matrix::dense::RowMajorMatrix;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher};

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

type F = Goldilocks;
pub type Blake3FieldHash = SerializingHasher<Blake3>;
pub type Blake3Compress = CompressionFunctionFromHasher<Blake3, 2, 32>;
pub type Blake3ValMmcs = MerkleTreeMmcs<F, u8, Blake3FieldHash, Blake3Compress, 32>;

const POLY_SIZE: usize = 1 << 19; // 2^19

pub fn run_lde_bench() {
    console_log!("P3 LDE Benchmark - Polynomial size: {}", POLY_SIZE);

    // Generate polynomial data
    let poly: Vec<F> = (0..POLY_SIZE)
        .map(|i| F::from_u64((1u64 << 55) + (i as u64)))
        .collect();


    // Create DFT instance
    let dft = Radix2DitParallel::<F>::default();

    let poly_matrix = RowMajorMatrix::new(poly.clone(), 1);
    // Benchmark LDE (Low Degree Extension)
    let start = Instant::now();

    // Convert to matrix for LDE
    let _evaluated = dft.dft_batch(poly_matrix);

    let lde_time = start.elapsed();
    console_log!("P3 LDE time: {:?}", lde_time);
}

pub fn run_merkle_bench() {
    console_log!("P3 Merkle Tree Benchmark - {} leaves", POLY_SIZE);

    // Generate data for Merkle tree
    let leaves_bases: Vec<F> = (0..POLY_SIZE)
        .map(|i| F::from_u64((1u64 << 55) + (i as u64)))
        .collect();

    {
        let leave_matrix = RowMajorMatrix::new(leaves_bases, 1);

        // Benchmark Blake3 Merkle tree
        let blake3_hash = Blake3 {};
        let compress = Blake3Compress::new(blake3_hash);

        let field_hash = Blake3FieldHash::new(blake3_hash);
        let val_mmcs = Blake3ValMmcs::new(field_hash, compress);

        let start = Instant::now();
        let (_commitment, _prover_data) = val_mmcs.commit(vec![leave_matrix]);
        let blake3_commit_time = start.elapsed();
        console_log!("P3 Blake3 Merkle commit time: {:?}", blake3_commit_time);
    }

    {
        // #[cfg(target_arch = "wasm32")]
        let leaves_bases: Vec<F> = (0..POLY_SIZE * 80)
            .map(|i| F::from_u64((1u64 << 55) + (i as u64)))
            .collect();

        let leave_matrix = RowMajorMatrix::new(leaves_bases, 80);

        // Benchmark Blake3 Merkle tree
        let blake3_hash = Blake3 {};
        let compress = Blake3Compress::new(blake3_hash);

        let field_hash = Blake3FieldHash::new(blake3_hash);
        let val_mmcs = Blake3ValMmcs::new(field_hash, compress);

        let start = Instant::now();
        let (_commitment, _prover_data) = val_mmcs.commit(vec![leave_matrix]);
        let blake3_commit_time = start.elapsed();
        console_log!("P3 Blake3 Merkle commit time: {:?}", blake3_commit_time);
    }
}
