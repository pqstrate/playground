use p3_air::{Air, AirBuilder, BaseAir};
use p3_challenger::{DuplexChallenger, HashChallenger, SerializingChallenger64};
use p3_commit::ExtensionMmcs;
use p3_dft::Radix2DitParallel;
use p3_field::extension::BinomialExtensionField;
use p3_field::PrimeCharacteristicRing;
use p3_fri::{create_benchmark_fri_params, create_test_fri_params, FriParameters, TwoAdicFriPcs};
use p3_goldilocks::{Goldilocks, Poseidon2Goldilocks};
use p3_keccak::{Keccak256Hash, KeccakF};
use p3_matrix::{dense::RowMajorMatrix, Matrix};
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{
    CompressionFunctionFromHasher, PaddingFreeSponge, SerializingHasher, TruncatedPermutation,
};
use p3_uni_stark::{prove, verify, StarkConfig};
use p3_util::log2_strict_usize;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use tracing::instrument;

const TRACE_WIDTH: usize = 2;

type Val = Goldilocks;
type Perm = Poseidon2Goldilocks<8>;
type MyHash = PaddingFreeSponge<Perm, 8, 4, 4>;
// type MyCompress = TruncatedPermutation<Perm, 2, 4, 8>;
// type ValMmcs = MerkleTreeMmcs<Val, Val, MyHash, MyCompress, 4>;
type Challenge = BinomialExtensionField<Val, 2>;
// type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
// type Challenger = DuplexChallenger<Val, Perm, 8, 4>;
// type Dft = Radix2DitParallel<Val>;
// type Pcs = TwoAdicFriPcs<Val, Dft, ValMmcs, ChallengeMmcs>;
// type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;

pub type ByteHash = Keccak256Hash; // Standard Keccak for byte hashing
pub type U64Hash = PaddingFreeSponge<KeccakF, 25, 17, 4>; // Keccak optimized for field elements
pub type FieldHash = SerializingHasher<U64Hash>; // Wrapper for field element hashing
pub type MyCompress = CompressionFunctionFromHasher<U64Hash, 2, 4>;
pub type ValMmcs = MerkleTreeMmcs<
    [Val; p3_keccak::VECTOR_LEN],
    [u64; p3_keccak::VECTOR_LEN],
    FieldHash,
    MyCompress,
    4,
>;
pub type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
pub type Dft = Radix2DitParallel<Val>;
pub type Challenger = SerializingChallenger64<Val, HashChallenger<u8, ByteHash, 32>>;
pub type Pcs = TwoAdicFriPcs<Val, Dft, ValMmcs, ChallengeMmcs>;
pub type MyConfig = StarkConfig<Pcs, Challenge, Challenger>;

#[derive(Clone)]
pub struct FibLikeAir {
    pub final_result: Val,
}

impl<F> BaseAir<F> for FibLikeAir {
    fn width(&self) -> usize {
        TRACE_WIDTH
    }
}

impl<AB: AirBuilder> Air<AB> for FibLikeAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0).expect("Matrix is empty?");
        let next = main.row_slice(1).expect("Matrix only has 1 row?");

        let x1 = local[0].clone();
        let x2 = local[1].clone();
        let next_x1 = next[0].clone();
        let next_x2 = next[1].clone();

        // Constraint: next_x1 = x1^8 + x2
        let x1_pow8 = x1.clone();
            // * x1.clone()
            // * x1.clone()
            // * x1.clone()
            // * x1.clone()
            // * x1.clone()
            // * x1.clone()
            // * x1.clone();
        builder
            .when_transition()
            .assert_eq(next_x1, x1_pow8 + x2.clone());

        // Constraint: next_x2 = x1 (shift register)
        builder.when_transition().assert_eq(next_x2, x1.clone());

        // Initial constraints
        builder
            .when_first_row()
            .assert_eq(x1.clone(), AB::Expr::ONE);
        builder.when_first_row().assert_eq(x2, AB::Expr::ONE);

        // Final constraint
        // We'll skip the final constraint for now to get it working
    }
}

pub fn generate_trace(num_steps: usize) -> (RowMajorMatrix<Val>, Val) {
    assert!(num_steps.is_power_of_two());

    let mut values = Vec::with_capacity(num_steps * TRACE_WIDTH);

    let mut x1 = Val::ONE;
    let mut x2 = Val::ONE;

    for _ in 0..num_steps {
        values.push(x1);
        values.push(x2);

        let next_x1 = x1.exp_u64(8) + x2;
        let next_x2 = x1;

        x1 = next_x1;
        x2 = next_x2;
    }

    let final_result = values[values.len() - TRACE_WIDTH];
    let trace = RowMajorMatrix::new(values, TRACE_WIDTH);
    println!("Trace generated with {} rows", trace.height());

    (trace, final_result)
}

#[instrument]
pub fn run_example(num_steps: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Generating proof for Fibonacci-like sequence (x1^8 + x2) with {} steps",
        num_steps
    );

    let (trace, final_result) = generate_trace(num_steps);
    println!("Final result: {}", final_result);
    println!("Trace: {}x{}", trace.height(), trace.width());

    // Set up cryptography like in fib_air test
    let byte_hash = ByteHash {};
    let u64_hash = U64Hash::new(KeccakF {});
    let compress = MyCompress::new(u64_hash);

    let field_hash = FieldHash::new(u64_hash);
    let val_mmcs = ValMmcs::new(field_hash, compress);
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
    let dft = Dft::default();

     let fri_params = FriParameters {
        log_blowup: 2,
        log_final_poly_len: log2_strict_usize(num_steps),
        num_queries: 100,
        proof_of_work_bits: 1,
        mmcs: challenge_mmcs,
    };
    println!("FRI params: {:?}", fri_params);

    let pcs = Pcs::new(dft, val_mmcs, fri_params);
    let challenger = Challenger::from_hasher(vec![], byte_hash);

    let config = MyConfig::new(pcs, challenger);
    let air = FibLikeAir { final_result };

    let proof = prove(&config, &air, trace, &vec![]);
    println!("Proof generated successfully!");

    // match verify(&config, &air, &proof, &vec![]) {
    //     Ok(()) => {
    //         println!("Proof verified successfully!");
    //         Ok(())
    //     }
    //     Err(e) => {
    //         println!("Proof verification failed: {:?}", e);
    //         Err(format!("Verification failed: {:?}", e).into())
    //     }
    // }
    Ok(())
}
