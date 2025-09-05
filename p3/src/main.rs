use p3::run_example;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fibonacci-like Sequence STARK Proof Demo (Plonky3)");
    println!("Sequence rule: x_{{n+1}} = x_n^8 + x_{{n-1}}");
    println!("Starting with x1 = 1, x2 = 1");
    println!();
    
    for log_num_steps in 4..=20 {
        let num_steps = 1 << log_num_steps;
        println!("Number of steps: {}", num_steps);
        run_example(num_steps)?;
    }

    Ok(())
}
