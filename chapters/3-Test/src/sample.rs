//! Sample the Mandelbrot set.

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
