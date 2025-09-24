use p3_monty::{run_example_blake3, run_example_poseidon2};
use std::env;
use tracing_subscriber;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get number of threads from environment or use default
    let num_threads = env::var("NUM_THREADS")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<usize>()
        .unwrap_or(8);

    // Get hash function type from environment or use default (both)
    let hash_type = env::var("HASH_TYPE")
        .unwrap_or_else(|_| "both".to_string())
        .to_lowercase();

    println!("Using {} threads", num_threads);

    // Configure rayon thread pool for parallelization
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    // Initialize tracing subscriber for logging/benchmarking with span traces
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_level(true)
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::NEW
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        )
        .with_ansi(atty::is(atty::Stream::Stdout))
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .init();
    println!("Sum Constraint STARK Proof Demo (Plonky3 with GoldilocksMonty simulation)");
    println!("Constraint: x_1^8 + x_2 + ... + x_{{num_col-1}} = x_num_col");
    println!("Transition: next_x1 = current_x_num_col");
    println!();

    for &log_num_steps in [19].iter() {
        let num_steps = 1 << log_num_steps;
        for &num_col in [80].iter() {
            println!("Number of steps: {}, Columns: {}", num_steps, num_col);

            match hash_type.as_str() {
                "blake3" => {
                    println!("Running with Blake3 hash function");
                    run_example_blake3(num_steps, num_col)?;
                }
                "poseidon2" => {
                    println!("Running with Poseidon2 hash function");
                    run_example_poseidon2(num_steps, num_col)?;
                }
                _ => {
                    println!("Running with Blake3 hash function");
                    run_example_blake3(num_steps, num_col)?;
                }
            }
        }
    }

    Ok(())
}
