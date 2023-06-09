//! Data to colour mapping.

use image::RgbImage;
use ndarray::{arr1, s, Array2, Array3};
use palette::{Gradient, LinSrgb, Pixel};

/// Convert a hex colour string to an RGB tuple.
fn hex_to_rgb(hex: &str) -> (f32, f32, f32) {
    let hex = hex.trim_start_matches('#');

    let hex_val: u32 = (u32::from_str_radix(hex, 16).ok()).unwrap();

    let red = ((hex_val >> 16) & 0xFF) as f32 / 255.0;
    let green = ((hex_val >> 8) & 0xFF) as f32 / 255.0;
    let blue = (hex_val & 0xFF) as f32 / 255.0;

    (red, green, blue)
}

/// Convert a 2D array of integers to an RGB image array.
pub fn image(data: Array2<u16>, cols: &[&str], max_iter: u16) -> Array3<u8> {
    let cs: Vec<_> = cols
        .iter()
        .map(|col| {
            let (r, g, b) = hex_to_rgb(col);
            LinSrgb::new(r, g, b)
        })
        .collect();
    let cmap: Gradient<LinSrgb> = Gradient::new(cs);

    let mut cols = Array3::<u8>::zeros((data.shape()[0], data.shape()[1], 3));
    let max_inv = 1.0 / max_iter as f32;
    let (width, height) = data.dim();
    for yi in 0..height {
        for xi in 0..width {
            let col = cmap.get(data[(xi, yi)] as f32 * max_inv);
            let u8s: [u8; 3] = col.into_format().into_raw();
            cols.slice_mut(s![xi, yi, ..]).assign(&arr1(&u8s));
        }
    }

    cols
}

/// Encode an RGB image array as an image.
pub fn encode(arr: &Array3<u8>) -> RgbImage {
    let img = transpose(&arr);
    let (width, height, _) = img.dim();

    RgbImage::from_vec(
        height as u32,
        width as u32,
        img.view().as_slice().unwrap().to_vec(),
    )
    .expect("Container should have the right size for the image dimensions.")
}

/// Transpose an RGB image array, fipping it along the diagonal.
pub fn transpose(arr: &Array3<u8>) -> Array3<u8> {
    let (width, height, _) = arr.dim();
    let mut buffer = Array3::zeros((height, width, 3));

    for n in 0..(width * height) {
        let xi = n % width;
        let yi = n / width;
        buffer
            .slice_mut(s![yi, xi, ..])
            .assign(&arr.slice(s![xi, yi, ..]));
    }

    buffer
}
