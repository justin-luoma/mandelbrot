use num_traits::{AsPrimitive, Bounded, Unsigned};
use crate::complex_number::ComplexNumber;
use crate::pixel::{Pixel, PixelMath};
use std::fmt::UpperHex;

pub trait ColorScale {
    fn pixel_color<T: 'static + Unsigned + Bounded + Copy + UpperHex + Into<f64>>(
        iters_to_escape: u32,
        ending_point: ComplexNumber<f64>,
        num_iterations: u32,
    ) -> Pixel<T>
    where
        f64: From<T> + AsPrimitive<T>;
}

pub struct ContinuousColorScale {}

impl ColorScale for ContinuousColorScale {
    fn pixel_color<T: 'static + Unsigned + Bounded + Copy + UpperHex + Into<f64>>(
        iters_to_escape: u32,
        ending_point: ComplexNumber<f64>,
        num_iterations: u32,
    ) -> Pixel<T>
    where
        f64: From<T> + AsPrimitive<T>,
    {
        ContinuousColorScale::pixel_color_gen(
            iters_to_escape,
            ending_point,
            num_iterations,
            200.95,
            0.8,
            1.0,
            10.0,
        )
    }
}

impl ContinuousColorScale {
    pub fn pixel_color_gen<T: 'static + Unsigned + Bounded + Copy + UpperHex + Into<f64>>(
        iters_to_escape: u32,
        ending_point: ComplexNumber<f64>,
        num_iterations: u32,
        hue: f64,
        sat: f64,
        val: f64,
        scale: f64,
    ) -> Pixel<T>
    where
        f64: From<T> + AsPrimitive<T>,
    {
        if iters_to_escape == num_iterations {
            return Pixel::new(T::zero(), T::zero(), T::zero());
        }

        let smooth: f64 = iters_to_escape.into();
        let smooth: f64 = smooth + 1.0 - ending_point.abs().log(10.0).log(2.0);

        Pixel::from_hsb(hue + scale * smooth, sat, val).unwrap()
    }

    pub fn get_color_fn<T: 'static + Unsigned + Bounded + Copy + UpperHex + Into<f64>>(
        hue: f64,
        sat: f64,
        val: f64,
    ) -> impl Fn(u32, ComplexNumber<f64>, u32) -> Pixel<T>
    where
        f64: From<T> + AsPrimitive<T>,
    {
        move |iters_to_escape: u32,
              ending_point: ComplexNumber<f64>,
              num_iterations: u32|
              -> Pixel<T> {
            ContinuousColorScale::pixel_color_gen(
                iters_to_escape,
                ending_point,
                num_iterations,
                hue,
                sat,
                val,
                10.0,
            )
        }
    }

    pub fn get_color_fn_boxed<T: 'static + Unsigned + Bounded + Copy + UpperHex + Into<f64>>(
        hue: f64,
        sat: f64,
        val: f64,
    ) -> Box<dyn Fn(u32, ComplexNumber<f64>, u32) -> Pixel<T>>
    where
        f64: From<T> + AsPrimitive<T>,
    {
        Box::new(
            move |iters_to_escape: u32,
                  ending_point: ComplexNumber<f64>,
                  num_iterations: u32|
                  -> Pixel<T> {
                ContinuousColorScale::pixel_color_gen(
                    iters_to_escape,
                    ending_point,
                    num_iterations,
                    hue,
                    sat,
                    val,
                    10.0,
                )
            },
        )
    }
}

pub struct DiscreteColorScale {}

impl ColorScale for DiscreteColorScale {
    fn pixel_color<T: 'static + Unsigned + Bounded + Copy + UpperHex + Into<f64>>(
        iters_to_escape: u32,
        _ending_point: ComplexNumber<f64>,
        max_iterations: u32,
    ) -> Pixel<T> {
        match f64::from(iters_to_escape) / f64::from(max_iterations) {
            p if p < 0.15 => Pixel::new(T::max_value(), T::min_value(), T::min_value()),
            p if p < 0.30 => Pixel::new(T::max_value(), T::max_value(), T::min_value()),
            p if p < 0.45 => Pixel::new(T::min_value(), T::max_value(), T::min_value()),
            p if p < 0.60 => Pixel::new(T::max_value(), T::max_value(), T::max_value()),
            p if p < 0.75 => Pixel::new(T::max_value(), T::min_value(), T::max_value()),
            p if p < 0.80 => Pixel::new(T::max_value(), T::min_value(), T::max_value()),
            p if p < 0.95 => Pixel::new(T::max_value(), T::max_value(), T::max_value()),
            _ => Pixel::new(T::min_value(), T::min_value(), T::min_value()),
        }
    }
}

pub struct SimpleColorScale {}

impl ColorScale for SimpleColorScale {
    fn pixel_color<T: 'static + Unsigned + Bounded + Copy + UpperHex + Into<f64>>(
        iters_to_escape: u32,
        _ending_point: ComplexNumber<f64>,
        max_iterations: u32,
    ) -> Pixel<T> {
        if iters_to_escape == max_iterations {
            Pixel::new(T::max_value(), T::min_value(), T::min_value())
        } else {
            Pixel::new(T::min_value(), T::min_value(), T::min_value())
        }
    }
}
