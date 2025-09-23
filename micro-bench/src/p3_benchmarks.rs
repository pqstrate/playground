use p3_blake3::Blake3;
use p3_commit::Mmcs;
use p3_dft::{Radix2DitParallel, TwoAdicSubgroupDft};
use p3_field::PrimeCharacteristicRing;
use p3_goldilocks::Goldilocks;
use p3_matrix::dense::RowMajorMatrix;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Instant;

type F = Goldilocks;
pub type Blake3FieldHash = SerializingHasher<Blake3>;
pub type Blake3Compress = CompressionFunctionFromHasher<Blake3, 2, 32>;
pub type Blake3ValMmcs = MerkleTreeMmcs<F, u8, Blake3FieldHash, Blake3Compress, 32>;

const POLY_SIZE: usize = 1 << 19; // 2^19

pub fn run_lde_bench() {
    println!("P3 LDE Benchmark - Polynomial size: {}", POLY_SIZE);

    // Generate random polynomial
    let mut rng = StdRng::seed_from_u64(42);
    let poly: Vec<F> = (0..POLY_SIZE)
        .map(|_| F::from_u64(rng.random::<u64>() & 0x7FFFFFFF)) // Keep positive for safety
        .collect();

    // Create DFT instance
    let dft = Radix2DitParallel::<F>::default();

    // Benchmark LDE (Low Degree Extension)
    let start = Instant::now();

    // Convert to matrix for LDE
    let poly_matrix = RowMajorMatrix::new(poly.clone(), 1);
    let _evaluated = dft.dft_batch(poly_matrix);

    let lde_time = start.elapsed();
    println!("P3 LDE time: {:?}", lde_time);

    // Simple IDFT benchmark
    let start = Instant::now();
    let poly_matrix2 = RowMajorMatrix::new(poly, 1);
    let _evaluated2 = dft.dft_batch(poly_matrix2);
    let idft_time = start.elapsed();
    println!("P3 DFT (simulated IDFT) time: {:?}", idft_time);
}

pub fn run_merkle_bench() {
    println!("P3 Merkle Tree Benchmark - {} leaves", POLY_SIZE);

    // Generate random data for Merkle tree
    let mut rng = StdRng::seed_from_u64(42);
    let leaves_bases: Vec<F> = (0..POLY_SIZE)
        .map(|_| F::from_u64(rng.random::<u64>() & 0x7FFFFFFF)) // Keep positive for safety
        .collect();
    let leave_matrix = RowMajorMatrix::new(leaves_bases.clone(), 1);

    // Benchmark Blake3 Merkle tree with simplified type

    let blake3_hash = Blake3 {};
    let compress = Blake3Compress::new(blake3_hash);

    let field_hash = Blake3FieldHash::new(blake3_hash);
    let val_mmcs = Blake3ValMmcs::new(field_hash, compress);

    let start = Instant::now();
    let (_commitment, _prover_data) = val_mmcs.commit(vec![leave_matrix]);
    let blake3_commit_time = start.elapsed();
    println!("P3 Blake3 Merkle commit time: {:?}", blake3_commit_time);
}
