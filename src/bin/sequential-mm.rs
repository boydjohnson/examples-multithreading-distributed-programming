use examples_multithreading_distributed_programming::matrix_multiplication::simple_multiply_a_b;
use examples_multithreading_distributed_programming::{main_inner, parse_args};

fn main() {
    let opts = parse_args(
        "sequential-mm",
        "Sequential matrix multiplication routine for square matrices with 1.0_f64 elements",
    );

    main_inner(opts, simple_multiply_a_b);
}
