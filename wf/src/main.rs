use wf::run_example;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fibonacci-like Sequence STARK Proof Demo");
    println!("Sequence rule: x_{{n+1}} = x_n^8 + x_{{n-1}}");
    println!("Starting with x1 = 1, x2 = 1");
    println!();

    run_example(8)?;
    
    Ok(())
}