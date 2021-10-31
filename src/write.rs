use crate::WriteImageError;
use image::{png::PngEncoder, ColorType};
use std::fs::File;
/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the file named
/// `filename`.
pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), WriteImageError> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    match encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8) {
        Ok(it) => it,
        Err(err) => return Err(WriteImageError::from(err)),
    };
    Ok(())
}
