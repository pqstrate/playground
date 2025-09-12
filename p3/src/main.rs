use p3::run_example;
use tracing_subscriber;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get number of threads from environment or use default
    let num_threads = env::var("NUM_THREADS")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<usize>()
        .unwrap_or(8);
    
    println!("Using {} threads", num_threads);
    
    // Configure rayon thread pool for parallelization
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    // Initialize tracing subscriber for logging/benchmarking with span traces
    tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .with_ansi(atty::is(atty::Stream::Stdout))
        .compact()
        .init();
    println!("Sum Constraint STARK Proof Demo (Plonky3)");
    println!("Constraint: x_1^8 + x_2 + ... + x_{{num_col-1}} = x_num_col");
    println!("Transition: next_x1 = current_x_num_col");
    println!();

    for &log_num_steps in [16, 20].iter() {
        let num_steps = 1 << log_num_steps;
        for &num_col in [40, 80].iter() {
            println!("Number of steps: {}, Columns: {}", num_steps, num_col);
            run_example(num_steps, num_col)?;
        }
    }

    Ok(())
}
