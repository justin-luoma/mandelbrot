use crate::pixel::{Pixel, PixelMath};
use std::fmt::{Debug, UpperHex};

use num_traits::{sign::Unsigned, Bounded, Float, Zero};

use std::{cmp, convert::From};
use itertools_num::linspace;

use crate::complex_number::ComplexNumber;
use crate::julia;

#[derive(Debug, Clone, Copy)]
pub struct Viewport<T: Float> {
    /// The top left coordinate for the grid that is to be plotted
    pub top_left: ComplexNumber<T>,
    pub bottom_right: ComplexNumber<T>,
    /// Width of the grid
    pub width: T,
    /// Height of the grid
    pub height: T,
}

pub struct MandelbrotConfig<P: Unsigned + Bounded + UpperHex + Copy + Zero> {
    /// The pixel dimensions of the area to generate values/pixels for
    pub dimensions: (u32, u32),
    /// The `Viewport` to cover
    pub viewport: Viewport<f64>,
    /// The (Boxed) coloring function to be used
    pub color_fn: Box<dyn Fn(u32, ComplexNumber<f64>, u32) -> Pixel<P>>,
}

pub struct Mandelbrot<P: Unsigned + Bounded + UpperHex + Copy + Zero> {
    config: MandelbrotConfig<P>,
    pixels: Vec<Vec<Pixel<P>>>,
    coords: (Vec<f64>, Vec<f64>),
    values: Vec<Vec<(u32, ComplexNumber<f64>)>>,
    steps: (ComplexNumber<f64>, ComplexNumber<f64>),
    iterations: u32,
}

impl<P: 'static + Unsigned + Bounded + UpperHex + Copy + Zero + Into<f64>> Mandelbrot<P> {
    pub fn new(config: MandelbrotConfig<P>) -> Mandelbrot<P> {
        let (w, h) = config.dimensions;

        let top_left = config.viewport.top_left;
        let bottom_right = config.viewport.bottom_right;
        let w_c = ComplexNumber::new(bottom_right.r, 0.0);
        let h_c = ComplexNumber::new(0.0, bottom_right.i);

        let re_range = linspace(top_left.r, bottom_right.r, w as usize).collect();
        let im_range = linspace(top_left.i, bottom_right.i, h as usize).collect();

        // let top_left = config.viewport.top_left;
        //
        // let x_step = bottom_right.r - top_left.r / config.viewport.width;
        // let y_step = bottom_right.i - top_left.i / config.viewport.height;
        //
        // let x_step = ComplexNumber::new(x_step, 0.);
        // let y_step = ComplexNumber::new(0., y_step);

        Mandelbrot {
            config,
            pixels: vec![vec![Pixel::<P>::default(); w as usize]; h as usize],
            values: vec![vec![(0, ComplexNumber::new(0.0, 0.0)); w as usize]; h as usize],
            steps: (w_c / w, h_c / h),
            iterations: 0,
            coords: (re_range, im_range),
        }
    }

    /// Returns a reference to the current state of the Pixels in the
    /// Mandelbrot Set
    pub fn get_pixels(&self) -> &Vec<Vec<Pixel<P>>> {
        &self.pixels
    }

    pub fn julia_set(&mut self, iterations: u32) {
        for (y, im) in self.coords.1.iter().enumerate() {
            for (x, re) in self.coords.0.iter().enumerate() {
                let (iters, z) = julia(*re, *im, iterations);
                self.pixels[y][x] = (self.config.color_fn)(iters, z, iterations);
            }
        }
    }

    pub fn get_xy_complex(&self, x: usize, y: usize) -> Option<ComplexNumber<f64>> {
        // self.values.get(y).and_then(|row| row.get(x).map(|v| v.1))
        match (self.coords.0.get(x), self.coords.1.get(y)) {
            (Some(x), Some(y)) => Some(ComplexNumber::new(*x, *y)),
            _ => None
        }
    }

    pub fn run(&mut self, iters: u32) {
        // let (w, h) = self.config.dimensions;
        // let top_left = self.config.viewport.top_left;
        // let bottom_right = self.config.viewport.bottom_right;

        // let z = 3.;

        // let (re_min, im_min) = map_coordinates(top_left.r, top_left.i, z, w as usize);
        // let (re_max, im_max) = map_coordinates(bottom_right.r, bottom_right.i, z, h as usize);

        // let re_range = linspace(top_left.r, bottom_right.r, w as usize);
        // let im_range = linspace(top_left.i, bottom_right.i, h as usize);

        self.iterations += iters;

        let mut max_iterations = 0;

        let (d_w, d_h) = self.steps;

        for (y, im) in self.coords.1.iter().enumerate() {
            for (x, re) in self.coords.0.iter().enumerate() {
                self.values[y][x] = iterate_coordinate(
                    self.values[y][x],
                    // top_left + d_w * im + d_h * re,
                    ComplexNumber::new(*re, *im),
                    iters,
                );

                max_iterations = cmp::max(max_iterations, self.values[y][x].0);
            }
        }

        for (r, row) in self.values.iter().enumerate() {
            for (c, (iters, zn)) in row.iter().enumerate() {
                self.pixels[r][c] = (self.config.color_fn)(*iters, *zn, max_iterations);
            }
        }
    }

    pub fn update(&mut self, viewport: Viewport<f64>) {
        let (w, h) = self.config.dimensions;

        let top_left = viewport.top_left;
        let bottom_right = viewport.bottom_right;
        let w_c = ComplexNumber::new(bottom_right.r, 0.0);
        let h_c = ComplexNumber::new(0.0, bottom_right.i);
        let re_range = linspace(top_left.r, bottom_right.r, w as usize).collect();
        let im_range = linspace(top_left.i, bottom_right.i, h as usize).collect();

        self.config.viewport = viewport;
        self.steps = (w_c / w, h_c / h);
        self.pixels = vec![vec![Pixel::<P>::default(); w as usize]; h as usize];
        self.values = vec![vec![(0, ComplexNumber::new(0.0, 0.0)); w as usize]; h as usize];
        self.iterations = 0;
        self.coords = (re_range, im_range);
    }

    pub fn reset(&mut self) {
        self.pixels
            .iter_mut()
            .map(|col| col.iter_mut().map(|px| *px = Pixel::<P>::default()))
            .count();
        self.values
            .iter_mut()
            .map(|row| {
                row.iter_mut()
                    .map(|coor| *coor = (0, ComplexNumber::new(0.0, 0.0)))
            })
            .count();
        self.iterations = 0;
    }
}

/// A helper function that runs the number of iterations given on a single
/// coordinate
fn iterate_coordinate<T: Float + Debug>(
    current_coord: (u32, ComplexNumber<T>),
    c: ComplexNumber<T>,
    limit: u32,
) -> (u32, ComplexNumber<T>)
    where
        f64: From<T>,
{
    let mut count = 0;
    let (finished_iters, mut z) = current_coord;

    let mut z = c;

    while z.norm_sqr() <= T::from(4.).unwrap() && count < limit {
        z = c + (z * z);
        count += 1;
    }

    (count + finished_iters, z)
}
