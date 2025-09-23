use winterfell::{
    crypto::{hashers::Blake3_256, MerkleTree},
    math::{fft, f64::BaseElement},
};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::time::Instant;

const POLY_SIZE: usize = 1 << 19; // 2^19

pub fn run_lde_bench() {
    println!("WF LDE Benchmark - Polynomial size: {}", POLY_SIZE);
    
    // Generate random polynomial using Winterfell BaseElement
    let mut rng = StdRng::seed_from_u64(42);
    let mut poly: Vec<BaseElement> = (0..POLY_SIZE)
        .map(|_| BaseElement::new(rng.random::<u64>()))
        .collect();
    
    // Benchmark LDE (Low Degree Extension)
    let start = Instant::now();
    
    // Perform FFT for LDE
    let twiddles = fft::get_twiddles::<BaseElement>(poly.len().trailing_zeros() as usize);
    fft::evaluate_poly(&mut poly, &twiddles);
    
    let lde_time = start.elapsed();
    println!("WF LDE time: {:?}", lde_time);
    
    // Benchmark inverse FFT
    let start = Instant::now();
    let inv_twiddles = fft::get_inv_twiddles::<BaseElement>(poly.len().trailing_zeros() as usize);
    fft::interpolate_poly(&mut poly, &inv_twiddles);
    let ifft_time = start.elapsed();
    println!("WF IFFT time: {:?}", ifft_time);
}

pub fn run_merkle_bench() {
    println!("WF Merkle Tree Benchmark - {} leaves", POLY_SIZE);
    
    // Generate random data for Merkle tree using Winterfell BaseElement
    let mut rng = StdRng::seed_from_u64(42);
    let leaves: Vec<BaseElement> = (0..POLY_SIZE)
        .map(|_| BaseElement::new(rng.random::<u64>()))
        .collect();
    
    // Benchmark Blake3_256 Merkle tree
    {
        let start = Instant::now();
        let tree = MerkleTree::<Blake3_256<BaseElement>>::new(leaves.clone()).unwrap();
        let blake3_commit_time = start.elapsed();
        println!("WF Blake3_256 Merkle commit time: {:?}", blake3_commit_time);
        
        // Benchmark proof generation
        let start = Instant::now();
        let indices = vec![0, POLY_SIZE / 2, POLY_SIZE - 1];
        let _proof = tree.prove_batch(&indices).unwrap();
        let blake3_proof_time = start.elapsed();
        println!("WF Blake3_256 Merkle proof time: {:?}", blake3_proof_time);
    }
    
    println!("WF RPO Merkle benchmark skipped due to version compatibility issues");
}