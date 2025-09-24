use rand::{RngCore, SeedableRng, rngs::SmallRng};
use p3_blake3::Blake3;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_challenger::{HashChallenger, SerializingChallenger64, DuplexChallenger};
use p3_commit::ExtensionMmcs;
use p3_dft::Radix2DitParallel;
use p3_field::extension::BinomialExtensionField;
use p3_field::PrimeCharacteristicRing;
use p3_fri::{FriParameters, TwoAdicFriPcs};
use p3_goldilocks_monty::{Goldilocks, Poseidon2Goldilocks};
use p3_keccak::{Keccak256Hash, KeccakF};
use p3_matrix::{dense::RowMajorMatrix, Matrix};
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{CompressionFunctionFromHasher, PaddingFreeSponge, SerializingHasher, TruncatedPermutation};
use p3_uni_stark::{prove, verify, StarkConfig};
use tracing::{instrument, info, debug, info_span};

// TRACE_WIDTH is now dynamic based on num_col

type Val = Goldilocks;
type Challenge = BinomialExtensionField<Val, 2>;

// Keccak-based type definitions
pub type KeccakByteHash = Keccak256Hash;
pub type KeccakU64Hash = PaddingFreeSponge<KeccakF, 25, 17, 4>;
pub type KeccakFieldHash = SerializingHasher<KeccakU64Hash>;
pub type KeccakCompress = CompressionFunctionFromHasher<KeccakU64Hash, 2, 4>;
pub type KeccakValMmcs = MerkleTreeMmcs<
    [Val; p3_keccak::VECTOR_LEN],
    [u64; p3_keccak::VECTOR_LEN],
    KeccakFieldHash,
    KeccakCompress,
    4,
>;
pub type KeccakChallengeMmcs = ExtensionMmcs<Val, Challenge, KeccakValMmcs>;
pub type KeccakChallenger = SerializingChallenger64<Val, HashChallenger<u8, KeccakByteHash, 32>>;
pub type KeccakPcs = TwoAdicFriPcs<Val, Radix2DitParallel<Val>, KeccakValMmcs, KeccakChallengeMmcs>;
pub type KeccakConfig = StarkConfig<KeccakPcs, Challenge, KeccakChallenger>;

// Poseidon2-based type definitions  
pub type Poseidon2Perm = Poseidon2Goldilocks<16>;
pub type Poseidon2Hash = PaddingFreeSponge<Poseidon2Perm, 16, 8, 8>;
pub type Poseidon2Compress = TruncatedPermutation<Poseidon2Perm, 2, 8, 16>;
pub type Poseidon2ValMmcs = MerkleTreeMmcs<
    <Val as p3_field::Field>::Packing,
    <Val as p3_field::Field>::Packing,
    Poseidon2Hash,
    Poseidon2Compress,
    8,
>;
pub type Poseidon2ChallengeMmcs = ExtensionMmcs<Val, Challenge, Poseidon2ValMmcs>;
pub type Poseidon2Challenger = DuplexChallenger<Val, Poseidon2Perm, 16, 8>;
pub type Poseidon2Pcs = TwoAdicFriPcs<Val, Radix2DitParallel<Val>, Poseidon2ValMmcs, Poseidon2ChallengeMmcs>;
pub type Poseidon2Config = StarkConfig<Poseidon2Pcs, Challenge, Poseidon2Challenger>;

// Blake3-based type definitions (following merkle-tree benchmark pattern)
pub type Blake3ByteHash = Blake3;
pub type Blake3FieldHash = SerializingHasher<Blake3>;
pub type Blake3Compress = CompressionFunctionFromHasher<Blake3, 2, 32>;
pub type Blake3ValMmcs = MerkleTreeMmcs<Val, u8, Blake3FieldHash, Blake3Compress, 32>;
pub type Blake3ChallengeMmcs = ExtensionMmcs<Val, Challenge, Blake3ValMmcs>;
pub type Blake3Challenger = SerializingChallenger64<Val, HashChallenger<u8, Blake3ByteHash, 32>>;
pub type Blake3Pcs = TwoAdicFriPcs<Val, Radix2DitParallel<Val>, Blake3ValMmcs, Blake3ChallengeMmcs>;
pub type Blake3Config = StarkConfig<Blake3Pcs, Challenge, Blake3Challenger>;

#[derive(Clone)]
pub struct FibLikeAir {
    pub final_result: Val,
    pub num_col: usize,
}

impl<F> BaseAir<F> for FibLikeAir {
    fn width(&self) -> usize {
        self.num_col
    }
}

impl<AB: AirBuilder> Air<AB> for FibLikeAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0).expect("Matrix is empty?");
        let next = main.row_slice(1).expect("Matrix only has 1 row?");

        // Get all local variables
        let x1 = local[0].clone();

        // Constraint: x_1^8 + x_2 + ... + x_{num_col-1} = x_num_col
        let x1_pow8 = x1.clone()
            * x1.clone()
            * x1.clone()
            * x1.clone()
            * x1.clone()
            * x1.clone()
            * x1.clone()
            * x1.clone();

        let mut sum = x1_pow8;

        // Add x_2 through x_{num_col-1}
        for i in 1..self.num_col - 1 {
            sum = sum + local[i].clone();
        }

        // Assert sum equals x_num_col (last column)
        builder.assert_zero(sum - local[self.num_col - 1].clone());

        // Transition constraint: next_x1 = current x_num_col
        let next_x1 = next[0].clone();
        builder
            .when_transition()
            .assert_eq(next_x1, local[self.num_col - 1].clone());

        // No initial constraints needed - allowing random starting values
    }
}

pub fn generate_trace(num_steps: usize, num_col: usize) -> (RowMajorMatrix<Val>, Val) {
    debug!("Starting trace generation: {} steps, {} columns", num_steps, num_col);
    let mut rng = SmallRng::seed_from_u64(123);
    assert!(num_steps.is_power_of_two());
    assert!(num_col >= 2, "num_col must be at least 2");

    let mut values = Vec::with_capacity(num_steps * num_col);

    // Initialize first row: need to satisfy x_1^8 + x_2 + ... + x_{num_col-1} = x_num_col
    let mut current_row = (0..num_col)
        .map(|_| Val::from_u32(rng.next_u32()))
        .collect::<Vec<_>>();

    // Make the first row satisfy the constraint: x_1^8 + x_2 + ... + x_{num_col-1} = x_num_col
    let x1_pow8 = current_row[0].exp_u64(8); // 1^8 = 1
    let mut sum = x1_pow8;
    for i in 1..num_col - 1 {
        sum += current_row[i]; // Add x_2, x_3, ..., x_{num_col-1}
    }
    current_row[num_col - 1] = sum; // Set x_num_col = sum

    for step in 0..num_steps {
        // Add current row to trace
        values.extend_from_slice(&current_row);

        // Compute next row if not the last step
        if step < num_steps - 1 {
            let mut next_row = vec![Val::ZERO; num_col];

            // x_1 of next row = x_num_col of current row
            next_row[0] = current_row[num_col - 1];

            // For columns 1 to num_col-2: set to 1 for simplicity
            for i in 1..num_col - 1 {
                next_row[i] = Val::ONE;
            }

            // x_num_col = x_1^8 + x_2 + ... + x_{num_col-1}
            let x1_pow8 = next_row[0].exp_u64(8);
            let mut sum = x1_pow8;
            for i in 1..num_col - 1 {
                sum += next_row[i];
            }
            next_row[num_col - 1] = sum;

            current_row = next_row;
        }
    }

    let final_result = values[values.len() - num_col]; // First element of last row
    let trace = RowMajorMatrix::new(values, num_col);
    info!("Trace generated with {} rows, {} cols", trace.height(), trace.width());
    debug!("Final result: {}", final_result);

    (trace, final_result)
}

#[instrument(level = "info", fields(num_steps, num_col, hash_type = "keccak"))]
pub fn run_example_keccak(num_steps: usize, num_col: usize) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Generating proof for sum constraint (x1^8 + x2 + ... + x{} = x{}) with {} steps using Keccak (GoldilocksMonty simulation)",
        num_col - 1,
        num_col,
        num_steps
    );

    let (trace, final_result) = generate_trace(num_steps, num_col);
    println!("Trace size: {}x{}", trace.height(), trace.width());

    // Set up Keccak-based cryptography
    let byte_hash = KeccakByteHash {};
    let u64_hash = KeccakU64Hash::new(KeccakF {});
    let compress = KeccakCompress::new(u64_hash);

    let field_hash = KeccakFieldHash::new(u64_hash);
    let val_mmcs = KeccakValMmcs::new(field_hash, compress);
    let challenge_mmcs = KeccakChallengeMmcs::new(val_mmcs.clone());
    let dft = Radix2DitParallel::<Val>::default();

    let fri_params = FriParameters {
        log_blowup: 3,
        log_final_poly_len: 1,
        num_queries: 100,
        proof_of_work_bits: 1,
        mmcs: challenge_mmcs,
    };

    let pcs = KeccakPcs::new(dft, val_mmcs, fri_params);
    let challenger = KeccakChallenger::from_hasher(vec![], byte_hash);

    let config = KeccakConfig::new(pcs, challenger);
    let air = FibLikeAir {
        final_result,
        num_col,
    };

    info!("Starting proof generation");
    let proof = info_span!("prove", num_steps = num_steps)
        .in_scope(|| prove(&config, &air, trace, &vec![]));
    info!("Proof generated successfully!");

    info!("Starting proof verification");
    match verify(&config, &air, &proof, &vec![]) {
        Ok(()) => {
            info!("Proof verified successfully!");
            Ok(())
        }
        Err(e) => {
            info!("Proof verification failed: {:?}", e);
            Err(format!("Verification failed: {:?}", e).into())
        }
    }
}

#[instrument(level = "info", fields(num_steps, num_col, hash_type = "poseidon2"))]
pub fn run_example_poseidon2(num_steps: usize, num_col: usize) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Generating proof for sum constraint (x1^8 + x2 + ... + x{} = x{}) with {} steps using Poseidon2 (GoldilocksMonty simulation)",
        num_col - 1,
        num_col,
        num_steps
    );

    let (trace, final_result) = generate_trace(num_steps, num_col);
    println!("Trace size: {}x{}", trace.height(), trace.width());

    // Set up Poseidon2-based cryptography  
    let mut rng = SmallRng::seed_from_u64(42);
    let perm = Poseidon2Perm::new_from_rng_128(&mut rng);
    let poseidon2_hash = Poseidon2Hash::new(perm.clone());
    let compress = Poseidon2Compress::new(perm.clone());
    
    let val_mmcs = Poseidon2ValMmcs::new(poseidon2_hash, compress);
    let challenge_mmcs = Poseidon2ChallengeMmcs::new(val_mmcs.clone());
    let dft = Radix2DitParallel::<Val>::default();

    let fri_params = FriParameters {
        log_blowup: 3,
        log_final_poly_len: 1,
        num_queries: 100,
        proof_of_work_bits: 1,
        mmcs: challenge_mmcs,
    };

    let pcs = Poseidon2Pcs::new(dft, val_mmcs, fri_params);
    let challenger = Poseidon2Challenger::new(perm);

    let config = Poseidon2Config::new(pcs, challenger);
    let air = FibLikeAir {
        final_result,
        num_col,
    };

    info!("Starting proof generation");
    let proof = info_span!("prove", num_steps = num_steps)
        .in_scope(|| prove(&config, &air, trace, &vec![]));
    info!("Proof generated successfully!");

    info!("Starting proof verification");
    match verify(&config, &air, &proof, &vec![]) {
        Ok(()) => {
            info!("Proof verified successfully!");
            Ok(())
        }
        Err(e) => {
            info!("Proof verification failed: {:?}", e);
            Err(format!("Verification failed: {:?}", e).into())
        }
    }
}


#[instrument(level = "info", fields(num_steps, num_col, hash_type = "blake3"))]
pub fn run_example_blake3(
    num_steps: usize,
    num_col: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Generating proof for sum constraint (x1^8 + x2 + ... + x{} = x{}) with {} steps using Blake3",
        num_col - 1,
        num_col,
        num_steps
    );

    let (trace, final_result) = generate_trace(num_steps, num_col);
    println!("Trace size: {}x{}", trace.height(), trace.width());

    // Set up Blake3-based cryptography
    let byte_hash = Blake3ByteHash {};
    let blake3_hash = Blake3 {};
    let compress = Blake3Compress::new(blake3_hash);

    let field_hash = Blake3FieldHash::new(blake3_hash);
    let val_mmcs = Blake3ValMmcs::new(field_hash, compress);
    let challenge_mmcs = Blake3ChallengeMmcs::new(val_mmcs.clone());
    let dft = Radix2DitParallel::<Val>::default();

    let fri_params = FriParameters {
        log_blowup: 3,
        log_final_poly_len: 1,
        num_queries: 100,
        proof_of_work_bits: 1,
        mmcs: challenge_mmcs,
    };

    let pcs = Blake3Pcs::new(dft, val_mmcs, fri_params);
    let challenger = Blake3Challenger::from_hasher(vec![], byte_hash);

    let config = Blake3Config::new(pcs, challenger);
    let air = FibLikeAir {
        final_result,
        num_col,
    };

    info!("Starting proof generation");
    let proof = info_span!("prove", num_steps = num_steps)
        .in_scope(|| prove(&config, &air, trace, &vec![]));
    info!("Proof generated successfully!");

    match verify(&config, &air, &proof, &vec![]) {
        Ok(()) => {
            info!("Proof verified successfully!");
            Ok(())
        }
        Err(e) => {
            info!("Proof verification failed: {:?}", e);
            Err(format!("Verification failed: {:?}", e).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power8_gate_small_keccak() {
        run_example_keccak(16, 3).expect("Small power8 gate test with Keccak failed");
    }

    #[test]
    fn test_power8_gate_medium_keccak() {
        run_example_keccak(256, 4).expect("Medium power8 gate test with Keccak failed");
    }

    #[test]
    fn test_power8_gate_small_poseidon2() {
        run_example_poseidon2(16, 3).expect("Small power8 gate test with Poseidon2 failed");
    }

    #[test]
    fn test_power8_gate_medium_poseidon2() {
        run_example_poseidon2(256, 4).expect("Medium power8 gate test with Poseidon2 failed");
    }

    #[test]
    fn test_trace_generation() {
        let (trace, final_result) = generate_trace(8, 3);
        assert_eq!(trace.height(), 8);
        assert_eq!(trace.width(), 3);

        // Verify constraint satisfaction for first row: x1^8 + x2 = x3
        let x1 = trace.get(0, 0).unwrap();
        let x2 = trace.get(0, 1).unwrap();
        let x3 = trace.get(0, 2).unwrap();
        let expected_x3 = x1.exp_u64(8) + x2;
        assert_eq!(x3, expected_x3);

        // Verify transition: x1[1] = x3[0]
        let x1_next = trace.get(1, 0).unwrap();
        assert_eq!(x1_next, x3);

        println!("Trace verification passed, final result: {}", final_result);
    }

    #[test]
    fn test_different_column_sizes() {
        // Test with 2 columns
        let (trace2, _) = generate_trace(4, 2);
        assert_eq!(trace2.width(), 2);

        // Test with 5 columns
        let (trace5, _) = generate_trace(4, 5);
        assert_eq!(trace5.width(), 5);

        println!("Different column size tests passed");
    }
}
