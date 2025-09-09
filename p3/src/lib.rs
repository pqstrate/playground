use ark_std::{end_timer, rand::RngCore, start_timer, test_rng};
use p3_air::{Air, AirBuilder, BaseAir};
use p3_challenger::{HashChallenger, SerializingChallenger64};
use p3_commit::ExtensionMmcs;
use p3_dft::Radix2DitParallel;
use p3_field::extension::BinomialExtensionField;
use p3_field::PrimeCharacteristicRing;
use p3_fri::{FriParameters, TwoAdicFriPcs};
use p3_goldilocks::Goldilocks;
use p3_keccak::{Keccak256Hash, KeccakF};
use p3_matrix::{dense::RowMajorMatrix, Matrix};
use p3_merkle_tree::MerkleTreeMmcs;
use p3_symmetric::{CompressionFunctionFromHasher, PaddingFreeSponge, SerializingHasher};
use p3_uni_stark::{prove, verify, StarkConfig};
use tracing::instrument;

// TRACE_WIDTH is now dynamic based on num_col

type Val = Goldilocks;
type Challenge = BinomialExtensionField<Val, 2>;

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
    let mut rng = test_rng();
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
    println!(
        "Trace generated with {} rows, {} cols",
        trace.height(),
        trace.width()
    );

    (trace, final_result)
}

#[instrument]
pub fn run_example(num_steps: usize, num_col: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Generating proof for sum constraint (x1^8 + x2 + ... + x{} = x{}) with {} steps",
        num_col - 1,
        num_col,
        num_steps
    );

    let (trace, final_result) = generate_trace(num_steps, num_col);
    // println!("Final result: {}", final_result);
    println!("Trace size: {}x{}", trace.height(), trace.width());

    // Set up cryptography like in fib_air test
    let byte_hash = ByteHash {};
    let u64_hash = U64Hash::new(KeccakF {});
    let compress = MyCompress::new(u64_hash);

    let field_hash = FieldHash::new(u64_hash);
    let val_mmcs = ValMmcs::new(field_hash, compress);
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());
    let dft = Dft::default();

    let fri_params = FriParameters {
        log_blowup: 3,
        log_final_poly_len: 1,
        num_queries: 100,
        proof_of_work_bits: 1,
        mmcs: challenge_mmcs,
    };
    // println!("FRI params: {:?}", fri_params);

    let pcs = Pcs::new(dft, val_mmcs, fri_params);
    let challenger = Challenger::from_hasher(vec![], byte_hash);

    let config = MyConfig::new(pcs, challenger);
    let air = FibLikeAir {
        final_result,
        num_col,
    };

    let timer = start_timer!(|| format!("proving for {} steps", num_steps));
    let proof = prove(&config, &air, trace, &vec![]);
    end_timer!(timer);
    println!("Proof generated successfully!");

    match verify(&config, &air, &proof, &vec![]) {
        Ok(()) => {
            println!("Proof verified successfully!");
            Ok(())
        }
        Err(e) => {
            println!("Proof verification failed: {:?}", e);
            Err(format!("Verification failed: {:?}", e).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power8_gate_small() {
        run_example(16, 3).expect("Small power8 gate test failed");
    }

    #[test]
    fn test_power8_gate_medium() {
        run_example(256, 4).expect("Medium power8 gate test failed");
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
