use std::fmt::{Debug, UpperHex};
use std::ops::Deref;

use num_traits::{AsPrimitive, Bounded, Float, ToPrimitive, Unsigned, Zero};

use mandelbrot_gui::{Generator, RgbaData};

use crate::flatten_array;
use crate::mandelbrot::Mandelbrot;
use crate::pixel::{Pixel, PixelMath};

impl<T, D, P, F> Generator<T, D> for Mandelbrot<P, F>
    where T: ToPrimitive,
          P: 'static + Unsigned + Bounded + UpperHex + Copy + Zero + Send + Sync + Sync + Debug +
          Into<f64>,
          F: Float + Send + Sync + Debug + Into<f64> + From<f64>,
          f64: From<P> + From<F> + AsPrimitive<P>,
          D: From<Vec<P>>,
{
    fn data(&self) -> D where D: Deref<Target=[T]> {
        flatten_array(self.get_pixels()).into()
    }
}

impl<P> RgbaData for Pixel<P>
    where P: 'static + Unsigned + Bounded + Send + Sync + Into<f64> + UpperHex + Copy + Into<f32>,
{
    fn data(&self) -> [f32; 4] {
        [
            self.get_tuple().0.into(),
            self.get_tuple().1.into(),
            self.get_tuple().2.into(),
            self.get_tuple().3.into(),
        ]
    }
}