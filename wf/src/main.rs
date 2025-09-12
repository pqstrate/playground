use wf::run_example;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get number of threads from environment or use default
    let num_threads = env::var("NUM_THREADS")
        .unwrap_or_else(|_| "8".to_string())
        .parse::<usize>()
        .unwrap_or(8);
    
    println!("Using {} threads", num_threads);
    
    // Configure thread pool for Winterfell
    std::env::set_var("RAYON_NUM_THREADS", num_threads.to_string());
    
    println!("Sum Constraint STARK Proof Demo (Winterfell)");
    println!("Constraint: x_1^8 + x_2 + ... + x_{{num_col-1}} = x_num_col");
    println!("Transition: next_x1 = current_x_num_col");
    println!();

    for &log_num_steps in [16, 20].iter() {
        let num_steps = 1 << log_num_steps;
        for num_col in [40, 80].iter() {
            println!("Number of steps: {}, Columns: {}", num_steps, num_col);
            run_example(num_steps, *num_col)?;
        }
    }

    Ok(())
}
