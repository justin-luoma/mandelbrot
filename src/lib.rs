use std::fmt::UpperHex;
use num_traits::{Bounded, Unsigned, Zero};
use crate::complex_number::ComplexNumber;
use crate::pixel::{IntoPixel, Pixel};

pub mod color_scale;
pub mod pixel;
pub mod complex_number;
pub mod mandelbrot;

pub fn flatten_array<T: Unsigned + Bounded + UpperHex + Zero + Copy>(
    grid: &[Vec<Pixel<T>>],
) -> Vec<T> {
    grid.iter()
        .flat_map(|col| col.iter().flat_map(|pixel| IntoPixel::<T>::new(pixel)))
        .collect()
}

pub fn julia(x: f64, y: f64, iterations: u32) -> (u32, ComplexNumber<f64>) {
    let mut z = ComplexNumber::new(x, y);
    let c = ComplexNumber::new(0.38, 0.28);
    let mut i = 0;
    while i < iterations && z.norm_sqr() < 32. {
        z = z * z + c;
        i += 1;
    }

    (i, z)
}
