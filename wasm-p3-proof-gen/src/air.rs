use ark_std::string::ToString;
use ark_std::vec;
use ark_std::vec::Vec;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::PrimeCharacteristicRing;
use p3_matrix::{Matrix, dense::RowMajorMatrix};

use crate::{Val, console_log};

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
    console_log!(
        "Starting trace generation: {} steps, {} columns",
        num_steps,
        num_col
    );

    assert!(num_steps.is_power_of_two());
    assert!(num_col >= 2, "num_col must be at least 2");

    let mut values = Vec::with_capacity(num_steps * num_col);

    // Initialize first row: need to satisfy x_1^8 + x_2 + ... + x_{num_col-1} = x_num_col
    let mut current_row = (0..num_col)
        .map(|i| Val::from_u32((i + 1) as u32))
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
    console_log!(
        "Trace generated with {} rows, {} cols",
        trace.height(),
        trace.width()
    );
    console_log!("Final result: {}", final_result);

    (trace, final_result)
}
