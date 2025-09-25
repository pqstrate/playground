use ark_std::string::ToString;
use ark_std::vec;
use p3_blake3::Blake3;
use p3_dft::Radix2DitParallel;
use p3_fri::FriParameters;
use p3_matrix::Matrix;
use p3_uni_stark::{prove, verify};

use crate::{
    Blake3ByteHash, Blake3ChallengeMmcs, Blake3Challenger, Blake3Compress, Blake3Config,
    Blake3FieldHash, Blake3Pcs, Blake3ValMmcs, FibLikeAir, Val, console_log, generate_trace,
};

pub fn run_example_blake3(num_steps: usize, num_col: usize) {
    console_log!(
        "Generating proof for sum constraint (x1^8 + x2 + ... + x{} = x{}) with {} steps using Blake3",
        num_col - 1,
        num_col,
        num_steps
    );

    let (trace, final_result) = generate_trace(num_steps, num_col);
    console_log!("Trace size: {}x{}", trace.height(), trace.width());

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

    console_log!("Starting proof generation");

    let proof = prove(&config, &air, trace, &vec![]);

    console_log!("Starting proof verification");
    match verify(&config, &air, &proof, &vec![]) {
        Ok(()) => {
            console_log!("Proof verified successfully!");
        }
        Err(e) => {
            console_log!("Proof verification failed: {:?}", e);
            return;
        }
    }
}
