use image::{GrayImage, ImageBuffer, Luma};
use num::Complex;

pub fn mandelbrot() -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let bounds = (1000, 750);
    let upper_left = Complex::new(-1.2, 0.35);
    let lower_right = Complex::new(-1.0, 0.20);

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right)
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    let real = upper_left.re + pixel.0 as f64 * width / bounds.0 as f64;
    let image = upper_left.im - pixel.1 as f64 * height / bounds.1 as f64;

    Complex::new(real, image)
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    assert!(pixels.len() == bounds.0 * bounds.1);

    let mut image = GrayImage::new(bounds.0 as u32, bounds.1 as u32);

    image
        .enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Luma<u8>)>>()
        .iter_mut()
        .for_each(|(column, row, pixel)| {
            let point = pixel_to_point(
                bounds,
                (*column as usize, *row as usize),
                upper_left,
                lower_right,
            );

            let result_color = match escape_time(point, 255) {
                Some(count) => 255 - count as u8,
                None => 0,
            };

            pixel[0] = result_color;
        });

    image
}
