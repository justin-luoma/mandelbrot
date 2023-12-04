use num_traits::{AsPrimitive, Bounded, Float, Unsigned, Zero};
use std::fmt::UpperHex;
use crate::color_scale::ContinuousColorScale;
use crate::complex_number::ComplexNumber;
use crate::config::viewport::Viewport;
use crate::pixel::Pixel;

pub mod viewport;

pub struct MandelbrotConfig<P: Unsigned + Bounded + UpperHex + Copy + Zero + Send + Sync, F:
Float + Send + Sync + 'static>
{
    /// The pixel dimensions of the area to generate values/pixels for
    pub dimensions: (u32, u32),
    /// The `Viewport` to cover
    pub viewport: Viewport<F>,
    /// The (Boxed) coloring function to be used
    pub color_fn: ColorFn<P, F>,
    pub exponent: u32,
}

impl<P: Unsigned + Bounded + UpperHex + Copy + Zero + Send + Sync, F:
Float + Send + Sync> MandelbrotConfig<P, F> {
    pub fn new(dimensions: (u32, u32), viewport: Viewport<F>, exponent: u32, color_fn: ColorFn<P,
        F>) -> Self {
        Self {
            dimensions,
            viewport,
            color_fn,
            exponent,
        }
    }

    pub fn with_dimensions(mut self, dimensions: (u32, u32)) -> Self {
        self.dimensions = dimensions;
        self
    }

    pub fn with_viewport(mut self, viewport: Viewport<F>) -> Self {
        self.viewport = viewport;
        self
    }

    pub fn with_color_fn(mut self, color_fn: ColorFn<P, F>) -> Self {
        self.color_fn = color_fn;
        self
    }

    pub fn with_exponent(mut self, exponent: u32) -> Self {
        self.exponent = exponent;
        self
    }
}

impl<P: 'static + Unsigned + Bounded + UpperHex + Copy + Zero + Send + Sync + Into<f64>, T:
Float + Send + Sync> Default
for
MandelbrotConfig<P, T>
    where
        f64: From<P> + AsPrimitive<P> + From<T>,
        T: Send + Sync + From<f64>,
{
    fn default() -> Self {
        Self::new(
            (500, 500),
            Viewport::default(),
            2,
            ContinuousColorScale::get_color_fn_boxed(200.0, 1.0, 1.0),
        )
    }
}

pub type ColorFn<P, T> = Box<dyn Fn(u32, ComplexNumber<T>, u32) -> Pixel<P> + Send + Sync>;
