use num_traits::Float;

use crate::complex_number::ComplexNumber;

#[derive(Debug, Clone, Copy)]
pub struct Viewport<F>
    where F: Float + Send + Sync + 'static,
{
    /// The top left coordinate for the grid that is to be plotted
    pub top_left: ComplexNumber<F>,
    pub bottom_right: ComplexNumber<F>,
    /// Width of the grid
    pub width: F,
    /// Height of the grid
    pub height: F,
}

impl<T: Float + Send + Sync> Default for Viewport<T> {
    fn default() -> Self {
        let a = ComplexNumber::new(T::from(-2.).unwrap(), T::from(1.15).unwrap());
        let b = ComplexNumber::new(T::from(0.5).unwrap(), T::from(-1.15).unwrap());

        Self {
            top_left: a,
            bottom_right: b,
            width: T::from(1000.).unwrap(),
            height: T::from(1000.).unwrap(),
        }
    }
}

impl<T: Float + Send + Sync> Viewport<T> {
    pub fn with_size(mut self, width: T, height: T) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_top_left(mut self, top_left: ComplexNumber<T>) -> Self {
        self.top_left = top_left;
        self
    }

    pub fn with_bottom_right(mut self, bottom_right: ComplexNumber<T>) -> Self {
        self.bottom_right = bottom_right;
        self
    }
}
