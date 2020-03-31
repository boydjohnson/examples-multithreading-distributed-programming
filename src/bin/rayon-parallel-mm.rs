use examples_multithreading_distributed_programming::main_inner;
use examples_multithreading_distributed_programming::matrix_multiplication::par_multiply_a_b_rayon;
use examples_multithreading_distributed_programming::parse_args;

fn main() {
    let opts = parse_args(
        "rayon-parallel-mm",
        "Parallel, using rayon, matrix multiplication routine",
    );

    main_inner(opts, par_multiply_a_b_rayon);
}
