use p3_field::PrimeCharacteristicRing;
use p3_matrix::Matrix;

use super::*;

#[test]
fn test_power8_gate_small_blake3() {
    run_example_blake3(16, 3);
}

#[test]
fn test_power8_gate_medium_blake3() {
    run_example_blake3(256, 4);
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

    console_log!("Trace verification passed, final result: {}", final_result);
}

#[test]
fn test_different_column_sizes() {
    // Test with 2 columns
    let (trace2, _) = generate_trace(4, 2);
    assert_eq!(trace2.width(), 2);

    // Test with 5 columns
    let (trace5, _) = generate_trace(4, 5);
    assert_eq!(trace5.width(), 5);

    console_log!("Different column size tests passed");
}
