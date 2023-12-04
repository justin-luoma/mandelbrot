use std::fmt::UpperHex;
use num_traits::{Bounded, Float, Unsigned, Zero};
use crate::complex_number::ComplexNumber;
use crate::pixel::{PixelIter, Pixel};

pub mod color_scale;
pub mod pixel;
pub mod complex_number;
pub mod mandelbrot;

#[cfg(feature = "gui")]
mod gui;
mod color;
pub mod config;

pub fn flatten_array<T: Unsigned + Bounded + UpperHex + Zero + Copy + Send + Sync>(
    grid: Vec<Vec<Pixel<T>>>,
) -> Vec<T> {
    grid.iter()
        .flat_map(|col| col
            .iter()
            .flat_map(|pixel| PixelIter::<T>::new(pixel))
        )
        .collect()
}

pub fn slope((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
    (y2 - y1) / (x2 - x1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slope() {
        assert_eq!(3./1., slope((3., 2.), (4., 5.)))
    }
}
