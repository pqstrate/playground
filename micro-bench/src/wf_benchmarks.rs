use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Instant;
use winterfell::{
    crypto::{hashers::Blake3_256, Hasher, MerkleTree},
    math::{fft, fields::f64::BaseElement},
};

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

    let twiddles = fft::get_twiddles::<BaseElement>(POLY_SIZE);
    fft::evaluate_poly(&mut poly, &twiddles);

    let lde_time = start.elapsed();
    println!("WF LDE time: {:?}", lde_time);

    // Benchmark inverse FFT
    let start = Instant::now();
    let inv_twiddles = fft::get_inv_twiddles::<BaseElement>(POLY_SIZE);
    fft::interpolate_poly(&mut poly, &inv_twiddles);
    let ifft_time = start.elapsed();
    println!("WF IFFT time: {:?}", ifft_time);
}

pub fn run_merkle_bench() {
    println!("WF Merkle Tree Benchmark - {} leaves", POLY_SIZE);
    {
        // Generate random byte data for Merkle tree
        let mut rng = StdRng::seed_from_u64(42);
        let leaves: Vec<_> = (0..POLY_SIZE)
            .map(|_| {
                let bytes: [u8; 32] = rng.random();
                Blake3_256::<BaseElement>::hash(&bytes)
            })
            .collect();

        let start = Instant::now();
        let _tree = MerkleTree::<Blake3_256<BaseElement>>::new(leaves).unwrap();
        let blake3_commit_time = start.elapsed();
        println!("WF Blake3_256 Merkle commit time: {:?}", blake3_commit_time);
    }
}
