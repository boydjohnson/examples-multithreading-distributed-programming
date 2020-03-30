use examples_multithreading_distributed_programming::{main_inner, parse_args};
use rayon::prelude::*;

fn main() {
    let opts = parse_args(
        "rayon-parallel-mm",
        "Parallel, using rayon, matrix multiplication routine",
    );

    main_inner(opts, par_multiply_a_b);
}

/// Parallel routine where outer column loop is parallelized.
fn par_multiply_a_b(n: usize) {
    let a = vec![vec![1.0_f64; n]; n];
    let b = vec![vec![1.0_f64; n]; n];

    (0..n)
        .into_par_iter()
        .map(|j| {
            let mut c_row = vec![0.0_f64; n];
            (0..n).for_each(|i| {
                for (k, b_row) in b.iter().enumerate() {
                    c_row[j] += a[i][k] * b_row[j];
                }
            });
            c_row
        })
        .collect::<Vec<Vec<f64>>>();
}
