use std::fmt::UpperHex;
use num_traits::{Bounded, Float, Unsigned, Zero};
use crate::complex_number::ComplexNumber;
use crate::pixel::{IntoPixel, Pixel};

pub mod color_scale;
pub mod pixel;
pub mod complex_number;
pub mod mandelbrot;

#[cfg(feature = "gui")]
mod gui;

pub fn flatten_array<T: Unsigned + Bounded + UpperHex + Zero + Copy + Send + Sync>(
    grid: Vec<Vec<Pixel<T>>>,
) -> Vec<T> {
    grid.iter()
        .flat_map(|col| col.iter().flat_map(|pixel| IntoPixel::<T>::new(pixel)))
        .collect()
}

pub fn julia<T: Float + Send + Sync>(x: f64, y: f64, iterations: u32) -> (u32, ComplexNumber<T>)
    where f64: Into<T> {
    let mut z = ComplexNumber::new(x.into(), y.into());
    let c = ComplexNumber::new((0.38).into(), (0.28).into());
    let mut i = 0;
    while i < iterations && z.norm_sqr() < (32.).into() {
        z = z * z + c;
        i += 1;
    }

    (i, z)
}
