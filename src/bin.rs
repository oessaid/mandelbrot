use mandelbrot_lib::{parse_complex, parse_pair, pixel_to_point, render, write_image};
use num::Complex;
use rayon::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT METHOD", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1.0,0.20 single",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let threads = 8;
    match args[5].as_str() {
        "single" => render(&mut pixels, bounds, upper_left, lower_right),
        "crossbeam" => render_with_crossbeam(&mut pixels, bounds, upper_left, lower_right, threads),
        "rayon" => render_with_rayon_bands(&mut pixels, bounds, upper_left, lower_right, threads),
        "fast" => render_with_rayon(&mut pixels, bounds, upper_left, lower_right),
        _ => println!("Choose a render method."),
    }

    write_image(&args[1], &pixels, bounds).expect("Error writing PNG file");
}

/// Render the image using `crossbeam`.
///
/// We divide the image into sections (one per processor) and let each processor color the pixels
/// assigned to it. For simplicity, we use horizontal bands. When all processors have finished, we
/// can write out the pixels to disk.
fn render_with_crossbeam(
    pixels: &mut Vec<u8>,
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    threads: usize,
) {
    {
        // How many rows of pixels each band should have.
        let rows_per_band = bounds.1 / threads + 1;
        // `chunks_mut` returns an iterator producing mutable, nonoverlapping slices of the buffer,
        // with `rows_per_band * bounds.0` pixels. In other words, `rows_per_band` complete rows of
        // pixels.
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .unwrap();
    }
}

/// Render the image using `rayon`.
///
/// Splitting the data into bands is not efficient, since the workload is not evenly. The lighter
/// parts of the image are pixels for which the Mandelbrot loop quickly exits, which is much faster
/// than the black parts, where the loop runs the full 255 iterations. Equal sized bands actually
/// create unequal workloads.
fn render_with_rayon_bands(
    pixels: &mut Vec<u8>,
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    threads: usize,
) {
    let rows_per_band = bounds.1 / threads + 1;
    pixels
        .par_chunks_mut(rows_per_band * bounds.0)
        .enumerate()
        .for_each(|(i, band)| {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
            render(band, band_bounds, band_upper_left, band_lower_right);
        });
}

/// Render the image using `rayon`, one task per pixel row.
///
/// We fire off a parallel task for each row of pixels in the output. This creates several hundred
/// tasks that Rayon can distribute across its threads. Rayon balances the work as it goes.
fn render_with_rayon(
    pixels: &mut Vec<u8>,
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    pixels
        .par_chunks_mut(bounds.0)
        .enumerate()
        .for_each(|(i, band)| {
            let top = i;
            let band_bounds = (bounds.0, 1);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + 1), upper_left, lower_right);
            render(band, band_bounds, band_upper_left, band_lower_right);
        });
}
