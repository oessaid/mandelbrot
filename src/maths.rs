use num::Complex;
/// Try to determine if `c` is in the Mandelbrot set, using at most `limit` iterations.
///
/// The Mandelbrot set is the set of values of `c` in the complex plane for which the orbit of the
/// critical point `z=0` remains bounded, under iteration of the quadratic map
/// z<sub>n+1</sub> = z<sub>n</sub><sup>2</sup> + c.
///
/// In other words, the Mandelbrot set is defined as the set of complex numbers `c` for which `z`
/// does not fly out to infinity.
///
/// The infinite loop takes a while to run, but we can leverage the following properties:
/// - a limited number of iterations still gives a close enough approximation of the set.
/// - the chosen number of iterations determines how precise the set's boundary is plotted.
/// - it's been shown that if `z` ever leaves the circle of radius `2` centered at the origin, it
/// will definitely fly out to inifinity eventually.
pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    // let mut iter = 0_usize;
    for i in 0..limit {
        // `norm_sqr` returns the square of `z`'s distance from the origin. To avoid computing a
        // square root, we compare this squared distance with the squared radius `4.0`, which is
        // faster.
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    // while iter < limit && z.norm_sqr() > 2.0 {
    //     z = z * z + c;
    //     iter += 1;
    // }
    None
}
