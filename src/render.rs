use crate::escape_time;
use num::Complex;
/// Given the row and column of a pixel in the output image, return the corresponding point on the
/// complex plane.
///
/// - `bounds` is a pair giving the width and height of the image in pixels.
/// - `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// - `upper_left` and `lower_right` are points on the complex plane designating the area our image
/// covers.
///
/// Since a complex number `c` has both real and imaginary components, we'll treat `c.re` and
/// `c.im` as the `x` and `y` coordinate of a point on the Cartesian plane.
///
/// ```text
///              |- --- bounds.0 ---- -|
/// upper_left---*---------------------+ -|
///              |                     |  |
///              |                     |  | bounds.1
///              |                     |  |
///              |                     |  |
///              +---------------------* -|
///                                    |
///                               lower_right
/// ```
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

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`, which holds one
/// grayscale pixel per byte. The `upper_left` and `lower_right` arguments specify points on the
/// complex plane corresponding to the upper-left and lower-right corners of the pixel buffer.
pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_to_point() {
        assert_eq!(
            pixel_to_point(
                (100, 200),
                (25, 175),
                Complex { re: -1.0, im: 1.0 },
                Complex { re: 1.0, im: -1.0 }
            ),
            Complex {
                re: -0.5,
                im: -0.75
            }
        )
    }
}
