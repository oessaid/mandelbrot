// ░█▄█░█▀█░█▀█░█▀▄░█▀▀░█░░░█▀▄░█▀▄░█▀█░▀█▀
// ░█░█░█▀█░█░█░█░█░█▀▀░█░░░█▀▄░█▀▄░█░█░░█░
// ░▀░▀░▀░▀░▀░▀░▀▀░░▀▀▀░▀▀▀░▀▀░░▀░▀░▀▀▀░░▀░
//
//! Plots the Mandelbrot set, a fractal produced by iterating a simple function on complex numbers.
//! Plotting the Mandelbrot set is often referred to as an *embarrassingly parallel* algorithms,
//! because the pattern of communication between the threads is so simple.
//!
//! # [Computer drawings](https://en.wikipedia.org/wiki/Mandelbrot_set#Computer_drawings)
//!
//! N.B.: an overview of plotting algorithms for the Mandelbrot set can be found
//! [here](https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set).
//!
//! The simplest algorithm is the *escape time algorithm*. A representation is performed for each
//! `x,y` point in the plot area and based on the behaviour of that calculation, a color is chosen
//! for that pixel, representing how quickly the values reached the escape point. Usually values
//! that fail to escape before the iteration limit (and are therefore bound to the Mandelbrot set)
//! are represented in black. Gradually brighter colors are used for points that escape.
//!
//! Specifically, the region of the complex plane we are considering is subdivided into a certain
//! number of pixels. To color any such pixel, let `c` be the midpoint of the pixel. We iterate the
//! critical point `0` under f<sub>c</sub>, checking at each step whether the orbit point has a
//! radius larger than `2`. If it's the case, we know that `c` does not belong to the Mandelbrot
//! set and we color the pixel according to the number of iterations used to find out.
//!
mod errors;
mod maths;
mod parse;
mod render;
mod write;

pub use errors::WriteImageError;
pub use maths::escape_time;
pub use parse::{parse_complex, parse_pair};
pub use render::render;
pub use write::write_image;
