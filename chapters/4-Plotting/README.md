# Plotting

## Dependencies

We're going to make use of multi-dimensional arrays from the `ndarray` crate:

```shell
cargo add ndarray
```

Adding it this way will add the latest compatible version to the `dependencies` section of the [`Cargo.toml`](./Cargo.lock) file.

## Area

Add the following function to the [`sample.rs`](./src/sample.rs) file:

```rust
use ndarray::Array2; // At the top of the file.

/// Sample a region of the Mandelbrot set.
pub fn area(real: f64, imag: f64, scale: f64, res: [usize; 2], max_iters: u16) -> Array2<u16> {
    let mut data = Array2::zeros((res[1], res[0]));

    let aspect_ratio = res[0] as f64 / res[1] as f64;
    let real_start = real - (scale * 0.5);
    let imag_start = imag - (scale / aspect_ratio * 0.5);

    let delta = scale / (res[0] - 1).max(1) as f64;

    for yi in 0..res[1] {
        let imag = imag_start + (delta * yi as f64);
        for xi in 0..res[0] {
            let real = real_start + (delta * xi as f64);
            data[(yi, xi)] = point(real, imag, max_iters);
        }
    }

    data
}
```

This function will be used to generate a 2D array of values that represent the number of iterations it takes to escape the Mandelbrot set for each point in the area.

## main

Update the [`main.rs`](./src/bin/main.rs) main function to use the `area` function:

```rust
/// Entry point function.
fn main() {
    let real = -0.5;
    let imag = 0.0;
    let scale = 3.0;
    let max_iters = 999;
    let res = [40, 30];

    let data = sample::area(real, imag, scale, res, max_iters);
    for y in 0..res[1] {
        for x in 0..res[0] {
            print!("{:4}", data[(x, y)]);
        }
        println!();
    }
}
```

## Try it

Run the `main` binary and check the output:

```shell
cargo run --bin main --release
```

You should see a 2D array of values that represent the number of iterations it takes to escape the Mandelbrot set for each point in the area:

```text
   1   1   1   1   1   2   2   2   2   2   3   3   3   3   3   3   3   3   3   3   3   4   4   5   5   5   4   4   3   3   2   2   2   2   2   2   2   2   2   2
   1   1   1   1   2   2   2   2   3   3   3   3   3   3   3   3   3   3   3   4   4   4   5   6  13   6   5   4   4   3   3   3   2   2   2   2   2   2   2   2
   1   1   1   1   2   2   2   3   3   3   3   3   3   3   3   3   3   4   4   4   4   5   5   7   9  27   7   5   4   4   4   3   3   2   2   2   2   2   2   2
   1   1   1   2   2   2   3   3   3   3   3   3   3   3   3   3   4   4   4   4   5   5   6   9  27  17   8   5   5   4   4   3   3   3   2   2   2   2   2   2
   1   1   1   2   2   3   3   3   3   3   3   3   3   3   3   4   4   4   4   5   5   6   8  84 999 999  27   7   5   5   4   4   3   3   3   2   2   2   2   2
   1   1   2   2   3   3   3   3   3   3   3   3   3   4   4   4   4   4   5   6   6   7   9  31 999 999  19   8   6   6   5   4   4   3   3   3   2   2   2   2
   1   1   2   3   3   3   3   3   3   3   3   3   4   4   4   4   5   6  12  29   9  24  21  18 999 999  16  12  16   7   7   9   4   3   3   3   3   2   2   2
   1   1   3   3   3   3   3   3   3   3   3   4   4   5   5   5   6   6   9 109  28 999 999 999 999 999 999 999  96  14  19  14   5   4   3   3   3   3   2   2
   1   2   3   3   3   3   3   3   3   4   4   5   5   5   5   6   6   8  11  21 999 999 999 999 999 999 999 999 999 999 999   8   5   4   3   3   3   3   2   2
   1   2   3   3   3   3   3   4   4   6   6   6   6   6   6   7   7  13 999 999 999 999 999 999 999 999 999 999 999 999  61   8   6   4   3   3   3   3   2   2
   1   3   3   3   4   4   4   5   5   6  10   8   8  10   8   8   9  77 999 999 999 999 999 999 999 999 999 999 999 999 999 999  11   4   4   3   3   3   3   2
   1   3   4   4   4   4   5   5   6   7  10 191  29 999  23  10  11 999 999 999 999 999 999 999 999 999 999 999 999 999 999  23   7   4   4   3   3   3   3   2
   1   4   4   4   4   5   5   5   7   8  12  77 999 999 999 232  15 999 999 999 999 999 999 999 999 999 999 999 999 999 999 141   7   4   4   3   3   3   3   2
   1   4   4   4   5   6   6   7  11  11  29 999 999 999 999 999  25 999 999 999 999 999 999 999 999 999 999 999 999 999 999  14   6   4   4   3   3   3   3   2
   1   5   6  11   7   7   8  10  26 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999   7   5   4   4   3   3   3   3   2
   1   5   6  11   8   9   9  11  24 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999 999  29   7   5   4   4   3   3   3   3   2
   1   4   4   4   6   6   6   8  15  13 999 999 999 999 999 999  59 999 999 999 999 999 999 999 999 999 999 999 999 999 999  18   6   4   4   3   3   3   3   2
   1   4   4   4   4   5   5   5   7   9  20 999 999 999 999 999  16 999 999 999 999 999 999 999 999 999 999 999 999 999 999  55   6   4   4   3   3   3   3   2
   1   3   4   4   4   4   5   5   6   7  10 999  22 999  56  12  12 999 999 999 999 999 999 999 999 999 999 999 999 999 999  27   7   4   4   3   3   3   3   2
   1   3   3   3   4   4   4   5   5   7  12  18   9  11   9   8   9 121 999 999 999 999 999 999 999 999 999 999 999 999 999 999  10   4   4   3   3   3   3   2
   1   3   3   3   3   3   4   4   5   7   8   6   6   6   7   7   8  11 999 999 999 999 999 999 999 999 999 999 999 999 999  10   7   4   4   3   3   3   3   2
   1   2   3   3   3   3   3   3   3   4   5   5   5   5   5   6   6   9  12 999 999 999 999 999 999 999 999 999 999 999  37   8   5   4   3   3   3   3   2   2
   1   1   3   3   3   3   3   3   3   3   3   4   5   5   5   5   6   6   9 999 257 999 999 999 999 999 999 999 999  17 188  25   5   4   3   3   3   3   2   2
   1   1   2   3   3   3   3   3   3   3   3   3   4   4   4   5   5   6  10  18  11  26  29 999 999 999  29  31  27   8   9   8   5   3   3   3   3   2   2   2
   1   1   2   2   3   3   3   3   3   3   3   3   3   4   4   4   4   5   6   7   7   7   9  45 999 999  14   8   7   6   6   5   4   3   3   3   2   2   2   2
   1   1   1   2   3   3   3   3   3   3   3   3   3   3   4   4   4   4   4   5   6   6   8  55 999 999  19   7   5   5   5   4   3   3   3   3   2   2   2   2
   1   1   1   2   2   3   3   3   3   3   3   3   3   3   3   3   4   4   4   4   5   5   6  16  21  23   9   5   5   4   4   4   3   3   3   2   2   2   2   2
   1   1   1   1   2   2   3   3   3   3   3   3   3   3   3   3   3   4   4   4   4   5   5   7  10  12   7   5   4   4   4   3   3   3   2   2   2   2   2   2
   1   1   1   1   2   2   2   2   3   3   3   3   3   3   3   3   3   3   4   4   4   4   5   6  30   7   7   4   4   4   3   3   2   2   2   2   2   2   2   2
   1   1   1   1   1   2   2   2   2   3   3   3   3   3   3   3   3   3   3   3   4   4   4   7  10   5   4   4   3   3   3   2   2   2   2   2   2   2   2   2
```

## Return

[Return to the top-level README](./../../README.md)
