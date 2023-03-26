use mandy::sample;

/// Entry point function.
fn main() {
    let real = -0.5;
    let imag = 0.0;
    let max_iters = 1000;

    let n = sample::point(real, imag, max_iters);

    println!("{} {} -> {}", real, imag, n);
}
