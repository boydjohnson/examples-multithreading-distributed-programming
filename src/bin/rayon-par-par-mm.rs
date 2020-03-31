use examples_multithreading_distributed_programming::main_inner;
use examples_multithreading_distributed_programming::matrix_multiplication::par_par_multiply_a_b_rayon;
use examples_multithreading_distributed_programming::parse_args;

fn main() {
    let opts = parse_args(
        "rayon-par-par-mm",
        "Both outer loops parallel matrix multiplication routine.",
    );

    main_inner(opts, par_par_multiply_a_b_rayon);
}
