use wf::run_example;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Sum Constraint STARK Proof Demo (Winterfell)");
    println!("Constraint: x_1^8 + x_2 + ... + x_{{num_col-1}} = x_num_col");
    println!("Transition: next_x1 = current_x_num_col");
    println!();

    for log_num_steps in 10..=20 {
        let num_steps = 1 << log_num_steps;
        for num_col in [3, 5, 10, 40, 80].iter() {
            println!("Number of steps: {}, Columns: {}", num_steps, num_col);
            run_example(num_steps, *num_col)?;
        }
    }

    Ok(())
}
