use examples_multithreading_distributed_programming::{main_inner, parse_args};

fn main() {
    let opts = parse_args(
        "sequential-mm",
        "Sequential matrix multiplication routine for square matrices with 1.0_f64 elements",
    );

    main_inner(opts, multiply_a_b);
}

/// Sequentially multiply A and B square matrices.
///
/// NOTE:
///     - All elements are initialized to 1.0_f64.
fn multiply_a_b(n: usize) {
    let a = vec![vec![1.0_f64; n]; n];
    let b = vec![vec![1.0_f64; n]; n];

    let mut c = vec![vec![0.0_f64; n]; n];

    for j in 0..n {
        for i in 0..n {
            for (k, b_row) in b.iter().enumerate() {
                c[i][j] += a[i][k] * b_row[j];
            }
        }
    }
}
