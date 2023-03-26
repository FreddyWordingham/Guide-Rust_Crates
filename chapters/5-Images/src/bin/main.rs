use std::{fs::create_dir, path::Path};

use mandy::{colour, sample};

/// Entry point function.
fn main() {
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        create_dir("output").unwrap();
    }

    let real = 0.42883258532;
    let imag = -0.23134912185;
    let scale = 1.0e-1;
    let max_iters = 1000;
    let res = [1920, 1080];
    let cmap = vec!["#000000", "#FFFFFF"];

    let data = sample::area(real, imag, scale, res, max_iters);
    let mut img = colour::image(data, cmap, max_iters);
    colour::encode(&mut img)
        .save(output_dir.join("mandy.png"))
        .unwrap();
}
