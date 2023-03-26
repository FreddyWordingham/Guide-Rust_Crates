//! Sample the Mandelbrot set.

use indicatif::ProgressBar;
use ndarray::Array2;

/// Determine the number of iterations required to escape a
/// circle of radius 2.0 from a given initial complex point.
pub fn point(real: f64, imag: f64, max_iters: u16) -> u16 {
    let mut z_re = 0.0;
    let mut z_im = 0.0;

    let mut i = 0;
    while (((z_re * z_re) + (z_im * z_im)) < 4.0) && (i < max_iters) {
        let new_re = (z_re * z_re) - (z_im * z_im) + real;
        let new_im = (2.0 * z_re * z_im) + imag;
        z_re = new_re;
        z_im = new_im;
        i += 1;
    }

    i as u16
}

/// Sample a region of the Mandelbrot set.
pub fn area(
    real: f64,
    imag: f64,
    scale: f64,
    res: [usize; 2],
    max_iters: u16,
    ss_power: u8,
) -> Array2<u16> {
    let mut data = Array2::zeros((res[0], res[1]));

    let aspect_ratio = res[0] as f64 / res[1] as f64;
    let real_start = real - (scale * 0.5);
    let imag_start = imag - (scale / aspect_ratio * 0.5);

    let delta = scale / (res[0] - 1).max(1) as f64;

    let total_pixels = (res[0] * res[1]) as usize;
    let pb = ProgressBar::new(total_pixels as u64);
    for n in 0..total_pixels {
        let xi = n % res[0];
        let yi = n / res[0];

        let imag = imag_start + (delta * yi as f64);
        let real = real_start + (delta * xi as f64);
        data[(xi, yi)] = super_sample(real, imag, max_iters, delta, ss_power);

        pb.inc(1);
    }

    data
}

/// Sample a small square region of the mandelbrot set and
/// return the average number of iterations required to escape a circle of radius 2.0.
fn super_sample(real: f64, imag: f64, max_iters: u16, delta: f64, ss_power: u8) -> u16 {
    let mut sum = 0;
    let epsilon = delta / ss_power as f64;

    let total_samples = ss_power * ss_power;
    for n in 0..total_samples {
        let i = n % ss_power;
        let j = n / ss_power;

        let re = real + (epsilon * i as f64);
        let im = imag + (epsilon * j as f64);
        sum += point(re, im, max_iters);
    }
    sum / total_samples as u16
}
