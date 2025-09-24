fn main() {
    use std::env;

    // Get number of threads from environment or use default
    let num_threads = env::var("NUM_THREADS")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<usize>()
        .unwrap_or(8);
    println!("Using {} threads", num_threads);

    // Configure thread pool for Winterfell
    std::env::set_var("RAYON_NUM_THREADS", num_threads.to_string());

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
        // .compact()
        .init();

    println!("start p3 benches");
    micro_bench::p3_benchmarks::run_lde_bench();
    micro_bench::p3_benchmarks::run_merkle_bench();

    println!("\nstart wf benches");
    micro_bench::wf_benchmarks::run_lde_bench();
    micro_bench::wf_benchmarks::run_merkle_bench();
}
